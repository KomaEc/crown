#![feature(rustc_private)]
#![feature(array_windows)]
#![feature(box_patterns)]
#![feature(split_array)]
#![feature(step_trait)]
#![feature(trusted_step)]
#![feature(min_specialization)]

mod rewrite_fn;
mod rewrite_ty;

use alias::TaintResult;
use analysis::{
    ownership::{solidify::SolidifiedOwnershipSchemes, whole_program::WholeProgramResults},
    ssa::AnalysisResults,
    type_qualifier::flow_insensitive::{fatness::FatnessResult, mutability::MutabilityResult},
};
use common::{
    data_structure::vec_vec::VecVec,
    rewrite::{Rewrite, RewriteMode},
    CrateData,
};
use rewrite_fn::rewrite_fns;
use rewrite_ty::rewrite_structs;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
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
extern crate rustc_type_ir;

extern crate either;

pub struct RefactorOptions {
    /// rewrite struct definitions and function signatures only
    pub type_only: bool,
    /// show detailed rewrite trace
    pub verbose: bool,
}

pub fn refactor<'tcx>(
    crate_data: &CrateData<'tcx>,
    analysis: &Analysis,
    rewrite_mode: RewriteMode,
    options: RefactorOptions,
) -> anyhow::Result<()> {
    let struct_decision = StructFields::new(crate_data, analysis);
    let fn_decision = FnLocals::new(crate_data, analysis);

    if options.verbose {
        let mut rewriter = VerboseRewriter { rewriter: vec![] };
        rewrite(
            crate_data,
            &fn_decision,
            &struct_decision,
            options.type_only,
            &mut rewriter,
        )?;
        rewriter.write(rewrite_mode);
    } else {
        let mut rewriter = vec![];

        rewrite(
            crate_data,
            &fn_decision,
            &struct_decision,
            options.type_only,
            &mut rewriter,
        )?;

        rewriter.write(rewrite_mode);
    }

    Ok(())
}

fn rewrite(
    crate_data: &CrateData,
    fn_decision: &FnLocals,
    struct_decision: &StructFields,
    type_only: bool,
    rewriter: &mut impl Rewrite,
) -> anyhow::Result<()> {
    rewrite_structs(
        &crate_data.structs,
        &struct_decision,
        rewriter,
        crate_data.tcx,
    )?;

    rewrite_fns(
        &crate_data.fns,
        &fn_decision,
        &struct_decision,
        rewriter,
        crate_data.tcx,
        type_only,
    );

    Ok(())
}

pub struct Analysis<'tcx> {
    taint_result: TaintResult,
    ownership_schemes: WholeProgramResults<'tcx>,
    ownership_result: SolidifiedOwnershipSchemes,
    mutability_result: MutabilityResult,
    fatness_result: FatnessResult,
}

impl<'tcx> Analysis<'tcx> {
    pub fn new(
        taint_result: TaintResult,
        ownership_schemes: WholeProgramResults<'tcx>,
        ownership_result: SolidifiedOwnershipSchemes,
        mutability_result: MutabilityResult,
        fatness_result: FatnessResult,
    ) -> Self {
        Analysis {
            taint_result,
            ownership_schemes,
            ownership_result,
            mutability_result,
            fatness_result,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PointerKind {
    Move,
    Mut,
    Const,
    Raw(RawMeta),
}

impl PointerKind {
    fn is_raw(&self) -> bool {
        matches!(*self, PointerKind::Raw(..))
    }

    fn is_move(&self) -> bool {
        *self == PointerKind::Move
    }

    fn is_mut(&self) -> bool {
        *self == PointerKind::Mut
    }

    fn is_const(&self) -> bool {
        *self == PointerKind::Const
    }

    fn is_safe(&self) -> bool {
        !self.is_raw()
    }

    fn is_raw_move(&self) -> bool {
        matches!(*self, PointerKind::Raw(RawMeta::Move))
    }

    fn is_copy(&self) -> bool {
        matches!(
            *self,
            PointerKind::Raw(RawMeta::Const | RawMeta::Mut) | PointerKind::Const
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RawMeta {
    Move,
    Const,
    Mut,
}

/// TODO use [`common::discretization::Descretization`]
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

    pub fn new(crate_data: &CrateData, analysis: &Analysis) -> Self {
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
            let fields_ownership = analysis
                .ownership_result
                .struct_results(did)
                .collect::<Vec<_>>();
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

                for (&ownership, &mutability, &fatness) in
                    itertools::izip!(ownership, mutability, fatness)
                {
                    let pointer_kind = if ownership.is_owning() {
                        if fatness.is_arr() || aliasing_nonowning_field {
                            PointerKind::Raw(RawMeta::Move)
                        } else {
                            PointerKind::Move
                        }
                    } else if mutability.is_immutable() {
                        PointerKind::Raw(RawMeta::Const)
                    } else {
                        PointerKind::Raw(RawMeta::Mut)
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

    pub fn new(crate_data: &CrateData, analysis: &Analysis) -> Self {
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
            let ownership = analysis.ownership_result.fn_results(did).results();
            let mutability = analysis.mutability_result.fn_results(did).results();
            let fatness = analysis.fatness_result.fn_results(did).results();
            let local_kind = analysis
                .ownership_schemes
                .fn_sig(*did)
                .map(|param| matches!(param, Some(param) if param.is_output()))
                .chain(std::iter::repeat(false));

            for (is_output_param, ownership, mutability, fatness) in
                itertools::izip!(local_kind, ownership, mutability, fatness)
            {
                let mut local: SmallVec<[PointerKind; 3]> =
                    SmallVec::with_capacity(ownership.len());
                for (&ownership, &mutability, &fatness) in
                    itertools::izip!(ownership, mutability, fatness)
                {
                    let pointer_kind = if ownership.is_owning() {
                        if fatness.is_arr() {
                            PointerKind::Raw(RawMeta::Move)
                        } else {
                            PointerKind::Move
                        }
                    } else if mutability.is_immutable() {
                        // TODO correctness?
                        if fatness.is_arr() {
                            PointerKind::Raw(RawMeta::Const)
                        } else {
                            PointerKind::Const
                        }
                    } else {
                        PointerKind::Raw(RawMeta::Mut)
                    };
                    local.push(pointer_kind);
                }
                if is_output_param && local[0].is_move() {
                    local[0] = PointerKind::Mut
                }
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

pub struct VerboseRewriter<R> {
    rewriter: R,
}

impl<R> Rewrite for VerboseRewriter<R>
where
    R: Rewrite,
{
    fn replace_with_msg(
        &mut self,
        tcx: rustc_middle::ty::TyCtxt,
        span: rustc_span::Span,
        message: String,
        replacement: String,
    ) {
        let original = common::rewrite::get_snippet(tcx, span).text.1;
        println!(
            "replacing {} with {} @ {:?}",
            original,
            replacement.clone(),
            span
        );
        self.rewriter
            .replace_with_msg(tcx, span, message, replacement);
    }

    fn write(self, mode: RewriteMode) {
        self.rewriter.write(mode)
    }
}
