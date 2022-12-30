use common::{
    data_structure::vec_vec::VecVec,
    discretization::{self, Discretization},
    CrateData,
};
use rustc_hash::FxHashMap;
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{
    visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor},
    Body, Local, Location, Place, Rvalue,
};

use super::{whole_program::WholeProgramResults, Ownership};
use crate::{
    ssa::{AnalysisResults, FnResults},
    type_qualifier::flow_insensitive::{TypeQualifiers, Var},
};

pub type SolidifiedOwnershipSchemes = TypeQualifiers<Ownership>;

impl<'tcx> WholeProgramResults<'tcx> {
    fn sanity_check(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn solidify(&self, crate_data: &CrateData) -> anyhow::Result<SolidifiedOwnershipSchemes> {
        self.sanity_check()?;

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

            let ownership_scheme = self.fn_results(*r#fn).unwrap();

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
                        for (solidified_ownership, ownership) in
                            solidified_ownership.iter_mut().zip(consume.def.iter())
                        {
                            if ownership.is_owning() {
                                *solidified_ownership = Ownership::Owning;
                            }
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

        Ok(TypeQualifiers::new(struct_fields, fn_locals, model))
    }
}

struct SanityCheck<'me, 'tcx> {
    ownership_schemes: &'me <WholeProgramResults<'tcx> as AnalysisResults<'me>>::FnResults,
    solidifed: &'me SolidifiedOwnershipSchemes,
    body: &'me Body<'tcx>,
}

impl<'me, 'tcx> Visitor<'tcx> for SanityCheck<'me, 'tcx> {
    fn visit_rvalue(&mut self, rvalue: &Rvalue<'tcx>, location: Location) {
        if let Rvalue::CopyForDeref(place) = rvalue {
            if self
                .solidifed
                .place_result(self.body, place)
                .first()
                .copied()
                .unwrap()
                .is_owning()
            {
                if let Some(flowing) = self.ownership_schemes.local_result(place.local, location) {
                    if flowing.r#use.len() >= 2
                        && !(flowing.r#use[1].is_owning() && flowing.def[1].is_owning())
                    {
                        // err
                    }
                }
            }
        }

        self.super_rvalue(rvalue, location)
    }

    fn visit_local(&mut self, local: Local, context: PlaceContext, location: Location) {
        if self
            .solidifed
            .place_result(self.body, &Place::from(local))
            .first()
            .copied()
            .unwrap()
            .is_owning()
            && matches!(
                context,
                PlaceContext::MutatingUse(MutatingUseContext::Projection)
                    | PlaceContext::NonMutatingUse(NonMutatingUseContext::Projection)
            )
        {
            if let Some(flowing) = self.ownership_schemes.local_result(local, location) {
                if !(flowing.r#use[0].is_owning() && flowing.def[0].is_owning()) {
                    // err
                }
            }
        }
    }
}
