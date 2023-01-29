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
use clap::{ArgGroup, Args};
use common::{
    data_structure::vec_vec::VecVec,
    rewrite::{Rewrite, RewriteMode},
    CrateData,
};
use rewrite_fn::rewrite_fns;
use rewrite_ty::rewrite_structs;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;
use smallvec::SmallVec;

extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_const_eval;
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

#[derive(Args)]
#[command(group(
    ArgGroup::new("box")
        .args(["no_box", "force_box"]),
))]
#[command(group(
    ArgGroup::new("mutability")
        .args(["const_reference", "raw_mutability"]),
))]
pub struct RefactorOptions {
    /// rewrite struct definitions and function signatures only
    #[clap(long)]
    pub type_only: bool,
    /// show detailed rewrite trace
    #[clap(long, short)]
    pub verbose: bool,
    /// attempt to rewrite const reference
    #[clap(long)]
    pub const_reference: bool,
    /// reconstruct every type it visits
    #[clap(long)]
    pub type_reconstruction: bool,
    /// not attempt to introduce box
    #[clap(long)]
    pub no_box: bool,
    /// force rewrite box even if no-box is deduced
    #[clap(long)]
    pub force_box: bool,
    /// rewrite raw pointer mutability with respect to mutability analysis
    #[clap(long)]
    pub raw_mutability: bool,
    /// fn name pattern that is required to be raw
    #[clap(long)]
    pub no_attempt: Option<String>,
}

pub fn refactor<'tcx>(
    crate_data: &CrateData<'tcx>,
    analysis: &Analysis,
    rewrite_mode: RewriteMode,
    options: RefactorOptions,
) -> anyhow::Result<()> {
    let mut options = options;
    for did in &crate_data.fns {
        if analysis.ownership_schemes.precision(did) == 0 {
            options.no_box = true;
        }
    }
    if options.force_box {
        options.no_box = false;
    }
    let options = options;

    let struct_decision = StructFields::new(crate_data, analysis, &options);
    let fn_decision = FnLocals::new(crate_data, analysis, &options);

    if options.verbose {
        let mut rewriter = VerboseRewriter { rewriter: vec![] };
        rewrite(
            crate_data,
            &fn_decision,
            &struct_decision,
            &options,
            &mut rewriter,
        )?;
        rewriter.write(rewrite_mode);
    } else {
        let mut rewriter = vec![];

        rewrite(
            crate_data,
            &fn_decision,
            &struct_decision,
            &options,
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
    // type_only: bool,
    // type_reconstruction: bool,
    options: &RefactorOptions,
    rewriter: &mut impl Rewrite,
) -> anyhow::Result<()> {
    rewrite_structs(
        &crate_data.structs,
        &struct_decision,
        rewriter,
        crate_data.tcx,
        options,
    )?;

    rewrite_fns(
        &crate_data.fns,
        &fn_decision,
        &struct_decision,
        rewriter,
        crate_data.tcx,
        options,
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

    fn is_raw_mut(&self) -> bool {
        matches!(*self, PointerKind::Raw(RawMeta::Move))
    }

    fn is_raw_const(&self) -> bool {
        matches!(*self, PointerKind::Raw(RawMeta::Const))
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

    pub fn is_owning(&self, tcx: TyCtxt, did: &DefId) -> bool {
        let fields_data = self.field_data(did);
        let fields = tcx.adt_def(*did).all_fields();
        fields_data
            .iter()
            .zip(fields)
            .any(|(field_data, field_def)| {
                if field_data.is_empty() {
                    let field_ty = tcx.type_of(field_def.did);
                    if let Some(adt_def) = field_ty.ty_adt_def() {
                        if self.0.did_idx.contains_key(&adt_def.did()) {
                            self.is_owning(tcx, &adt_def.did())
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    field_data[0].is_move() || field_data[0].is_raw_move()
                }
            })
    }

    pub fn new(crate_data: &CrateData, analysis: &Analysis, options: &RefactorOptions) -> Self {
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

            let adt_def = crate_data.tcx.adt_def(*did);

            for (field, ownership, mutability, fatness, aliases) in itertools::izip!(
                adt_def.all_fields(),
                fields_ownership.iter().copied(),
                fields_mutability,
                fields_fatness,
                fields_aliases.iter()
            ) {
                assert_eq!(ownership.len(), mutability.len());
                assert_eq!(mutability.len(), fatness.len());

                let mut field_ty = crate_data.tcx.type_of(field.did);

                let aliasing_nonowning_field = aliases.iter().any(|&idx| {
                    fields_ownership[idx]
                        .iter()
                        .all(|ownership| !ownership.is_owning())
                });
                let mut field = SmallVec::with_capacity(ownership.len());

                for (&ownership, &mutability, &fatness) in
                    itertools::izip!(ownership, mutability, fatness)
                {
                    while let Some(inner_ty) = field_ty.builtin_index() {
                        field_ty = inner_ty;
                    }

                    let pointer_kind = if ownership.is_owning() {
                        if fatness.is_arr() || aliasing_nonowning_field || options.no_box {
                            PointerKind::Raw(RawMeta::Move)
                        } else {
                            PointerKind::Move
                        }
                    } else {
                        if options.raw_mutability {
                            if mutability.is_immutable() && !field_ty.is_mutable_ptr() {
                                PointerKind::Raw(RawMeta::Const)
                            } else {
                                PointerKind::Raw(RawMeta::Mut)
                            }
                        } else {
                            if !field_ty.is_mutable_ptr() {
                                PointerKind::Raw(RawMeta::Const)
                            } else {
                                PointerKind::Raw(RawMeta::Mut)
                            }
                        }
                    };
                    field.push(pointer_kind);

                    field_ty = field_ty.builtin_deref(true).unwrap().ty;
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

    pub fn new(crate_data: &CrateData, analysis: &Analysis, options: &RefactorOptions) -> Self {
        let mut did_idx = FxHashMap::default();
        did_idx.reserve(crate_data.fns.len());
        let mut fn_locals = VecVec::with_capacity(
            crate_data.fns.len(),
            crate_data.fns.iter().fold(0, |acc, did| {
                let r#fn = crate_data.tcx.optimized_mir(*did);
                acc + r#fn.local_decls.len()
            }),
        );

        let no_attempt = options
            .no_attempt
            .as_deref()
            .map(|pattern| regex::Regex::new(pattern).expect("bad fn name patterns"));

        for (idx, did) in crate_data.fns.iter().enumerate() {
            let ownership = analysis.ownership_result.fn_results(did).results();
            let mutability = analysis.mutability_result.fn_results(did).results();
            let fatness = analysis.fatness_result.fn_results(did).results();
            let local_kind = analysis
                .ownership_schemes
                .fn_sig(*did)
                .map(|param| matches!(param, Some(param) if param.is_output()))
                .chain(std::iter::repeat(false));

            let body = crate_data.tcx.optimized_mir(did);

            let no_attempt = matches!(no_attempt.as_ref().map(|regex| regex.is_match(&crate_data.tcx.def_path_str(*did))), Some(matched) if matched);

            for (local_decl, is_output_param, ownership, mutability, fatness) in itertools::izip!(
                body.local_decls.iter(),
                local_kind,
                ownership,
                mutability,
                fatness
            ) {
                let mut ty = local_decl.ty;
                let mut local: SmallVec<[PointerKind; 3]> =
                    SmallVec::with_capacity(ownership.len());
                for (&ownership, &mutability, &fatness) in
                    itertools::izip!(ownership, mutability, fatness)
                {
                    while let Some(inner_ty) = ty.builtin_index() {
                        ty = inner_ty;
                    }
                    let pointer_kind = if ownership.is_owning() {
                        if fatness.is_arr() {
                            PointerKind::Raw(RawMeta::Move)
                        } else {
                            PointerKind::Move
                        }
                    } else {
                        if options.raw_mutability {
                            if mutability.is_immutable() && !ty.is_mutable_ptr() {
                                PointerKind::Raw(RawMeta::Const)
                            } else {
                                PointerKind::Raw(RawMeta::Mut)
                            }
                        } else if options.const_reference {
                            if mutability.is_immutable() && !fatness.is_arr() {
                                PointerKind::Const
                            } else if !ty.is_mutable_ptr() {
                                PointerKind::Raw(RawMeta::Const)
                            } else {
                                PointerKind::Raw(RawMeta::Mut)
                            }
                        } else {
                            if !ty.is_mutable_ptr() {
                                PointerKind::Raw(RawMeta::Const)
                            } else {
                                PointerKind::Raw(RawMeta::Mut)
                            }
                        }
                    };
                    local.push(pointer_kind);

                    // update type
                    ty = ty.builtin_deref(true).unwrap().ty;
                }
                if is_output_param {
                    if local[0].is_move() && !no_attempt {
                        local[0] = PointerKind::Mut
                    } else if local[0].is_raw_move() || no_attempt {
                        ty = local_decl.ty;
                        local[0] = if !ty.is_mutable_ptr() {
                            PointerKind::Raw(RawMeta::Const)
                        } else {
                            PointerKind::Raw(RawMeta::Mut)
                        }
                    } else {
                        unreachable!()
                    }
                }
                if options.no_box || no_attempt {
                    for pointer_kind in &mut local {
                        if pointer_kind.is_move() {
                            *pointer_kind = PointerKind::Raw(RawMeta::Move)
                        }
                    }
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
