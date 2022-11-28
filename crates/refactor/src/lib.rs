#![feature(rustc_private)]
#![feature(array_windows)]
#![feature(box_patterns)]
#![feature(split_array)]
#![feature(step_trait)]
#![feature(trusted_step)]
#![feature(min_specialization)]

mod rewrite_fn;
mod rewrite_ty;

use alias::{AliasResult, TaintResult};
use analysis::{
    ownership::{whole_program::WholeProgramResults, Ownership, Param},
    ssa::{AnalysisResults, FnResults},
    type_qualifier::flow_insensitive::{fatness::FatnessResult, mutability::MutabilityResult},
};
use common::{
    data_structure::vec_vec::VecVec,
    rewrite::{Rewrite, RewriteMode},
    CrateData,
};
use either::Either::{Left, Right};
use rewrite_fn::rewrite_fns;
use rewrite_ty::rewrite_structs;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::Location;
use smallvec::SmallVec;

extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_passes;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;

extern crate either;

pub fn refactor<'tcx>(
    crate_data: &CrateData<'tcx>,
    analysis: &Analysis<'tcx>,
) -> anyhow::Result<()> {
    let struct_decision = StructFields::new(crate_data, analysis);
    // let fn_decision = FnParams::new(crate_data, analysis);
    let fn_decision = FnLocals::new(crate_data, analysis);
    let mut rewriter = vec![];
    rewrite_structs(
        &crate_data.structs,
        &struct_decision,
        &mut rewriter,
        crate_data.tcx,
    )?;

    rewrite_fns(&crate_data.fns, &fn_decision, &mut rewriter, crate_data.tcx);

    rewriter.write(RewriteMode::Diff);

    Ok(())
}

pub struct Analysis<'tcx> {
    alias_result: AliasResult,
    taint_result: TaintResult,
    ownership_schemes: WholeProgramResults<'tcx>,
    mutability_result: MutabilityResult,
    fatness_result: FatnessResult,
}

impl<'tcx> Analysis<'tcx> {
    pub fn new(
        alias_result: AliasResult,
        taint_result: TaintResult,
        ownership_schemes: WholeProgramResults<'tcx>,
        mutability_result: MutabilityResult,
        fatness_result: FatnessResult,
    ) -> Self {
        Analysis {
            alias_result,
            taint_result,
            ownership_schemes,
            mutability_result,
            fatness_result,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PointerKind {
    Move,
    Mut,
    Shr,
    Raw,
}

impl PointerKind {
    fn is_raw(&self) -> bool {
        *self == PointerKind::Raw
    }
}

// #[derive(Clone, Copy, PartialEq, Eq, Debug)]
// pub struct PointerData {
//     pointer_kind: PointerKind,
//     meta_data: MetaData,
// }

// impl PointerData {
//     fn raw() -> Self {
//         PointerData {
//             pointer_kind: PointerKind::Raw,
//             meta_data: MetaData {
//                 ownership: Ownership::Unknown,
//                 mutability: Mutability::Mut,
//                 fatness: Fatness::Ptr,
//             },
//         }
//     }
// }

// #[derive(Clone, Copy, PartialEq, Eq, Debug)]
// pub struct MetaData {
//     ownership: Ownership,
//     mutability: Mutability,
//     fatness: Fatness,
// }

pub struct Decision {
    did_idx: FxHashMap<DefId, usize>,
    data: VecVec<SmallVec<[PointerKind; 3]>>,
}

pub struct StructFields(Decision);

impl StructFields {
    pub fn field_data(&self, did: &DefId) -> &[SmallVec<[PointerKind; 3]>] {
        let idx = self.0.did_idx[did];
        &self.0.data[idx]
    }

    pub fn new<'tcx>(crate_data: &CrateData<'tcx>, analysis: &Analysis<'tcx>) -> Self {
        let mut did_idx = FxHashMap::default();
        did_idx.reserve(crate_data.structs.len());
        let mut struct_fields = VecVec::with_capacity(
            crate_data.structs.len(),
            crate_data.structs.iter().fold(0, |acc, did| {
                let adt_def = crate_data.tcx.adt_def(*did);
                acc + adt_def.all_fields().count()
            }),
        );

        for (idx, did) in crate_data.structs.iter().enumerate() {
            let fields_ownership = analysis.ownership_schemes.fields(did).collect::<Vec<_>>();
            let fields_mutability = analysis.mutability_result.struct_results(did);
            let fields_fatness = analysis.fatness_result.struct_results(did);
            let fields_aliases = analysis.taint_result.fields_aliases(did);

            for (ownership, mutability, fatness, aliases) in itertools::izip!(
                fields_ownership.iter().copied(),
                fields_mutability,
                fields_fatness,
                fields_aliases.iter()
            ) {
                assert_eq!(ownership.len(), mutability.len());
                assert_eq!(mutability.len(), fatness.len());

                let aliasing_nonowning_field = aliases.iter().any(|&idx| {
                    fields_ownership[idx]
                        .iter()
                        .all(|ownership| !ownership.is_owning())
                });
                let mut field = SmallVec::with_capacity(ownership.len());

                for (&ownership, _, &fatness) in itertools::izip!(ownership, mutability, fatness) {
                    let pointer_kind = if ownership.is_owning()
                        && !fatness.is_arr()
                        && !aliasing_nonowning_field
                    {
                        PointerKind::Move
                    } else {
                        PointerKind::Raw
                    };
                    field.push(pointer_kind);
                }
                struct_fields.push_inner(field);
            }

            struct_fields.push();

            did_idx.insert(*did, idx);
        }

        let struct_fields = struct_fields.done();
        StructFields(Decision {
            did_idx,
            data: struct_fields,
        })
    }
}

impl std::fmt::Debug for StructFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (did, &idx) in self.0.did_idx.iter() {
            let mut index = 0;
            writeln!(f, "@{:?}: {{", did)?;
            for field in self.0.data[idx].iter() {
                let field_str = field
                    .iter()
                    .map(|pointer_kind| format!("{:?}", pointer_kind))
                    .collect::<Vec<_>>()
                    .join(" ");

                writeln!(f, "   {index}: {field_str}")?;

                index += 1;
            }
            writeln!(f, "}}")?;
        }
        Ok(())
    }
}

pub struct FnLocals(Decision);

impl FnLocals {
    pub fn local_data(&self, did: &DefId) -> &[SmallVec<[PointerKind; 3]>] {
        let idx = self.0.did_idx[did];
        &self.0.data[idx]
    }

    pub fn new<'tcx>(crate_data: &CrateData<'tcx>, analysis: &Analysis<'tcx>) -> Self {
        let mut did_idx = FxHashMap::default();
        did_idx.reserve(crate_data.fns.len());
        let mut fn_locals = VecVec::with_capacity(
            crate_data.fns.len(),
            crate_data.fns.iter().fold(0, |acc, did| {
                let r#fn = crate_data.tcx.optimized_mir(*did);
                acc + r#fn.local_decls.len()
            }),
        );

        for (idx, did) in crate_data.fns.iter().enumerate() {
            let r#fn = crate_data.tcx.optimized_mir(*did);

            // parameters

            let sig_size = r#fn.arg_count + 1;
            let ownership_sig = analysis.ownership_schemes.fn_sig(*did);
            let mut mutability = analysis.mutability_result.fn_results(did).results();
            // .take(sig_size);
            let mut fatness = analysis.fatness_result.fn_results(did).results();
            // .take(sig_size);

            for (ownership, mutability, fatness) in
                itertools::izip!(ownership_sig, mutability.by_ref(), fatness.by_ref())
            {
                let mut is_output_param = false;
                let ownership = match ownership {
                    Some(ownership) => Left(match ownership {
                        Param::Output(ownership) => {
                            is_output_param = true;
                            Left(
                                itertools::izip!(
                                    ownership.r#use.iter().copied(),
                                    ownership.def.iter().copied()
                                )
                                .map(|(r#in, out)| {
                                    if r#in.is_owning() || out.is_owning() {
                                        Ownership::Owning
                                    } else {
                                        r#in
                                    }
                                }),
                            )
                        }
                        Param::Normal(ownership) => Right(ownership.iter().copied()),
                    }),
                    None => Right(std::iter::empty()),
                }
                .chain(std::iter::repeat(Ownership::Unknown));

                let mut param: SmallVec<[PointerKind; 3]> =
                    itertools::izip!(ownership, mutability, fatness)
                        .map(|(ownership, &mutability, &fatness)| {
                            let pointer_kind = if fatness.is_arr() {
                                PointerKind::Raw
                            } else if ownership.is_owning() {
                                PointerKind::Move
                            } else if mutability.is_immutable() {
                                PointerKind::Shr
                            } else {
                                PointerKind::Raw
                            };

                            pointer_kind
                        })
                        .collect();

                if is_output_param {
                    param[0] = PointerKind::Mut;
                }

                fn_locals.push_inner(param);
            }

            // other locals
            let mut other_locals = Vec::with_capacity(r#fn.local_decls.len() - sig_size);
            for local_decl in r#fn.local_decls.iter().skip(sig_size) {
                let ptr_depth = mir_ty_ptr_count(local_decl.ty);
                // fn_locals.push_inner(smallvec::smallvec![PointerData::raw(); ptr_depth]);
                other_locals.push(smallvec::smallvec![PointerKind::Raw; ptr_depth]);
            }

            let ownership_scheme = analysis.ownership_schemes.fn_results(*did).unwrap();

            for (bb, bb_data) in r#fn.basic_blocks.iter_enumerated() {
                for statement_index in
                    0usize..(bb_data.statements.len() + bb_data.terminator.is_some() as usize)
                {
                    let location = Location {
                        block: bb,
                        statement_index,
                    };

                    let location_results = ownership_scheme.location_results(location);
                    for (local, consume) in location_results {
                        if local.as_usize() >= sig_size {
                            let ptr_data: &mut SmallVec<[PointerKind; 3]> =
                                &mut other_locals[local.as_usize() - sig_size];
                            for (ptr_kind, ownership) in ptr_data.iter_mut().zip(consume.def.iter())
                            {
                                if ownership.is_owning() {
                                    *ptr_kind = PointerKind::Move;
                                }
                            }
                        }
                    }
                }
            }

            for (local, mutability, fatness) in
                itertools::izip!(other_locals.iter_mut(), mutability, fatness)
            {
                for (ptr_kind, &mutability, &fatness) in
                    itertools::izip!(local, mutability, fatness)
                {
                    if fatness.is_arr() {
                        *ptr_kind = PointerKind::Raw;
                    } else if mutability.is_immutable() && (*ptr_kind == PointerKind::Move) {
                        *ptr_kind = PointerKind::Shr;
                    }
                }
            }

            for local in other_locals {
                fn_locals.push_inner(local);
            }

            fn_locals.push();

            did_idx.insert(*did, idx);
        }

        let fn_locals = fn_locals.done();
        FnLocals(Decision {
            did_idx,
            data: fn_locals,
        })
    }
}

struct HirPtrTypeWalker<'me, 'hir> {
    ty: &'me rustc_hir::Ty<'hir>,
}

impl<'me, 'hir> Iterator for HirPtrTypeWalker<'me, 'hir> {
    type Item = &'me rustc_hir::Ty<'hir>;

    fn next(&mut self) -> Option<Self::Item> {
        let ty = peel_arrs(self.ty);
        if let rustc_hir::TyKind::Ptr(inner) = &ty.kind {
            let ptr_ty = ty;
            self.ty = inner.ty;
            Some(ptr_ty)
        } else {
            None
        }
    }
}

pub fn peel_arrs<'a, 'hir>(ty: &'a rustc_hir::Ty<'hir>) -> &'a rustc_hir::Ty<'hir> {
    let mut final_ty = ty;
    while let rustc_hir::TyKind::Array(ty, _) | rustc_hir::TyKind::Slice(ty) = &final_ty.kind {
        final_ty = &ty;
    }
    final_ty
}

trait HirTyExt<'hir> {
    fn walk_ptr(&self) -> HirPtrTypeWalker;
}

impl<'hir> HirTyExt<'hir> for rustc_hir::Ty<'hir> {
    fn walk_ptr(&self) -> HirPtrTypeWalker {
        HirPtrTypeWalker { ty: self }
    }
}

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
