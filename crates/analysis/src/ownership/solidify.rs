use std::ops::Range;

use common::{
    data_structure::vec_vec::VecVec,
    discretization::{self, Discretization},
    CrateData,
};
use either::Either::Left;
use rustc_hash::{FxHashMap, FxHashSet};
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{
    visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor},
    Body, Local, LocalInfo, Location, Operand, Place, Rvalue, StatementKind, TerminatorKind,
};

use super::{whole_program::WholeProgramResults, Ownership};
use crate::{
    ssa::{AnalysisResults, FnResults},
    type_qualifier::flow_insensitive::{TypeQualifiers, Var},
};

pub type SolidifiedOwnershipSchemes = TypeQualifiers<Ownership>;

impl<'tcx> WholeProgramResults<'tcx> {
    pub fn solidify(&self, crate_data: &CrateData<'tcx>) -> SolidifiedOwnershipSchemes {
        let mut model = IndexVec::new();
        let mut next: Var = model.next_index();
        let mut did_idx = FxHashMap::default();
        did_idx.reserve(crate_data.structs.len());
        let mut vars =
            VecVec::with_capacity(crate_data.structs.len(), crate_data.structs.len() * 4);

        for (idx, r#struct) in crate_data.structs.iter().enumerate() {
            did_idx.insert(*r#struct, idx);

            let fields_ownership = self.fields(r#struct);
            for ownership in fields_ownership {
                model.extend(ownership.iter().copied());
                vars.push_inner(next);
                next = next + ownership.len();
                assert_eq!(model.next_index(), next);
            }
            vars.push_inner(next);
            vars.push();
        }
        let vars = vars.done();
        let struct_fields = discretization::StructFields(Discretization {
            did_idx,
            contents: vars,
        });

        let mut did_idx = FxHashMap::default();
        did_idx.reserve(crate_data.fns.len());
        let mut vars = VecVec::with_capacity(crate_data.fns.len(), crate_data.fns.len() * 15);
        for (idx, r#fn) in crate_data.fns.iter().enumerate() {
            did_idx.insert(*r#fn, idx);

            let body = crate_data.tcx.optimized_mir(*r#fn);

            let mut locals = Vec::with_capacity(body.local_decls.len());
            for local_decl in body.local_decls.iter() {
                fn mir_ty_ptr_count(mut ty: rustc_middle::ty::Ty) -> usize {
                    let mut cnt = 0;
                    loop {
                        if let Some(inner) = ty.builtin_index() {
                            ty = inner;
                            continue;
                        }
                        if let Some(inner_mut) = ty.builtin_deref(true) {
                            ty = inner_mut.ty;
                            cnt += 1;
                            continue;
                        }
                        break;
                    }
                    cnt
                }
                let ptr_depth = mir_ty_ptr_count(local_decl.ty);
                locals.push(smallvec::smallvec![Ownership::Transient; ptr_depth]);
            }

            for (param, ownership) in self.fn_sig(*r#fn).zip(&mut locals) {
                if matches!(param, Some(param) if param.is_output()) {
                    ownership[0] = Ownership::Owning
                }
            }

            let ownership_scheme = self.fn_results(*r#fn).unwrap();

            let mut proxy_temporaries = FxHashSet::default();
            for bb_data in body.basic_blocks.iter() {
                let Some(terminator) = &bb_data.terminator else { continue; };
                if let TerminatorKind::Call { args, .. } = &terminator.kind {
                    proxy_temporaries.extend(
                        args.iter()
                            .filter_map(|arg| arg.place().and_then(|arg| arg.as_local())),
                    )
                }
            }
            for (local, local_decl) in body.local_decls.iter_enumerated() {
                if matches!(local_decl.local_info, Some(box LocalInfo::DerefTemp)) {
                    proxy_temporaries.insert(local);
                }
            }

            for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
                for statement_index in
                    0usize..(bb_data.statements.len() + bb_data.terminator.is_some() as usize)
                {
                    let location = Location {
                        block: bb,
                        statement_index,
                    };

                    let location_results = ownership_scheme.location_results(location);
                    for (local, consume) in location_results {
                        let solidified_ownership: &mut smallvec::SmallVec<[Ownership; 3]> =
                            &mut locals[local.as_usize()];
                        for (solidified_ownership, r#use, def) in
                            itertools::izip!(solidified_ownership, consume.r#use, consume.def)
                        {
                            if r#use.is_owning() || def.is_owning() {
                                *solidified_ownership = Ownership::Owning;
                            }
                        }
                    }

                    let Left(stmt) = body.stmt_at(location) else { continue };
                    let StatementKind::Assign(box (place, rvalue)) = &stmt.kind else { continue };
                    if matches!(place.as_local(), Some(local) if proxy_temporaries.contains(&local))
                    {
                        let mut by_ref = false;
                        let rplace = match rvalue {
                            Rvalue::AddressOf(_, rplace) | Rvalue::Ref(_, _, rplace) => {
                                by_ref = true;
                                rplace
                            }
                            Rvalue::Cast(_, Operand::Copy(rplace) | Operand::Move(rplace), _) => {
                                rplace
                            }
                            Rvalue::Use(Operand::Copy(rplace) | Operand::Move(rplace)) => rplace,
                            Rvalue::CopyForDeref(rplace) => rplace,
                            // Rvalue::Cast(_, Operand::Constant(..), _) | Rvalue::Use(Operand::Constant(..)) => { continue }
                            // _ => unimplemented!("{:?}", rvalue),
                            _ => continue,
                        };

                        let mut ownership: &[Ownership] = &locals[rplace.local.index()][..];
                        if ownership.is_empty() {
                            continue;
                        }
                        let mut index = 0;
                        let mut ty = body.local_decls[rplace.local].ty;

                        for proj in rplace.projection {
                            match proj {
                                rustc_middle::mir::ProjectionElem::Deref => {
                                    index += 1;
                                    ty = ty.builtin_deref(true).unwrap().ty;
                                }
                                rustc_middle::mir::ProjectionElem::Field(f, field_ty) => {
                                    assert_eq!(index, ownership.len());
                                    let Some(adt_def) = ty.ty_adt_def() else {
                                        // tuple
                                        continue
                                    };
                                    if adt_def.is_union() {
                                        // FIXME
                                        ownership = &[];
                                        continue;
                                    }
                                    let Range { start, end } =
                                        struct_fields.field(&adt_def.did(), f.index());
                                    ownership = &model.raw[start.index()..end.index()];
                                    index = 0;
                                    ty = field_ty;
                                }
                                rustc_middle::mir::ProjectionElem::Index(_) => {
                                    ty = ty.builtin_index().unwrap();
                                }
                                _ => unreachable!(),
                            }
                        }

                        let ownership =
                            smallvec::SmallVec::<[_; 2]>::from_slice(&ownership[index..]);

                        let proxy_temporary = if by_ref {
                            &mut locals[place.local.index()][1..]
                        } else {
                            &mut locals[place.local.index()]
                        };

                        for (proxy, ownership) in proxy_temporary.iter_mut().zip(ownership) {
                            *proxy = ownership
                        }
                    }
                }
            }

            for local in locals {
                vars.push_inner(next);
                next = next + local.len();
                model.extend(local.into_iter());
                assert_eq!(model.next_index(), next);
            }
            vars.push_inner(next);
            vars.push();
        }
        let vars = vars.done();
        let fn_locals = discretization::FnLocals(Discretization {
            did_idx,
            contents: vars,
        });

        let solidified = TypeQualifiers::new(struct_fields, fn_locals, model);

        sanity_check(self, &solidified, crate_data);

        solidified
    }
}

fn sanity_check<'tcx>(
    ownership_schemes: &WholeProgramResults<'tcx>,
    solidifed_ownership_schemes: &SolidifiedOwnershipSchemes,
    crate_data: &CrateData<'tcx>,
) {
    for r#fn in &crate_data.fns {
        let body = crate_data.tcx.optimized_mir(*r#fn);

        let ownership_schemes = ownership_schemes.fn_results(*r#fn).unwrap();

        let mut sanity_checker = SanityCheck {
            ownership_schemes: &ownership_schemes,
            solidifed: solidifed_ownership_schemes,
            body,
            err_locations: Vec::new(),
        };

        sanity_checker.visit_body(body);

        let err_locations = sanity_checker.err_locations;

        for location in err_locations {
            let stmt = body.stmt_at(location);
            let span = match stmt {
                either::Either::Left(stmt) => stmt.source_info.span,
                either::Either::Right(term) => term.source_info.span,
            };

            let source_text = common::rewrite::get_snippet(crate_data.tcx, span).text.1;
            tracing::error!("semantics changed: {source_text} @ {:?}", span)
        }
    }
}

/// This does not work for more than two dereferences
struct SanityCheck<'me, 'tcx> {
    ownership_schemes: &'me <WholeProgramResults<'tcx> as AnalysisResults<'me>>::FnResults,
    solidifed: &'me SolidifiedOwnershipSchemes,
    body: &'me Body<'tcx>,
    err_locations: Vec<Location>,
}

impl<'me, 'tcx> Visitor<'tcx> for SanityCheck<'me, 'tcx> {
    fn visit_rvalue(&mut self, rvalue: &Rvalue<'tcx>, location: Location) {
        if let Rvalue::CopyForDeref(place) = rvalue {
            if matches!(self
                .solidifed
                .place_result(self.body, &Place::from(place.local))
                .chunks(2)
                .next(),
                Some(&[ownership1, ownership2]) if ownership1.is_owning() && ownership2.is_owning())
            {
                if let Some(flowing) = self.ownership_schemes.local_result(place.local, location) {
                    if flowing.r#use.len() >= 2
                        && !(flowing.r#use[1].is_owning() && flowing.def[1].is_owning())
                    {
                        self.err_locations.push(location);
                    }
                }
            }
        }

        self.super_rvalue(rvalue, location)
    }

    fn visit_local(&mut self, local: Local, context: PlaceContext, location: Location) {
        if matches!(self
            .solidifed
            .place_result(self.body, &Place::from(local))
            .first()
            .copied(), Some(ownership) if ownership.is_owning())
            && matches!(
                context,
                PlaceContext::MutatingUse(MutatingUseContext::Projection)
                    | PlaceContext::NonMutatingUse(NonMutatingUseContext::Projection)
            )
        {
            if let Some(flowing) = self.ownership_schemes.local_result(local, location) {
                if flowing.is_use() && !flowing.r#use[0].is_owning()
                    || !flowing.is_use()
                        && !(flowing.r#use[0].is_owning() && flowing.def[0].is_owning())
                {
                    self.err_locations.push(location);
                }
            }
        }
    }
}
