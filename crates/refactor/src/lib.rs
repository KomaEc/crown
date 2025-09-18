#![feature(rustc_private)]
#![feature(array_windows)]
#![feature(box_patterns)]
#![feature(split_array)]
#![feature(step_trait)]
#![feature(trusted_step)]
#![feature(min_specialization)]

mod rewrite_fn;
mod rewrite_ty;

use analyses::borrow::PromotedMutRefs as PromotedMutRefResult;
use analyses::output_params::OutputParams as OutputParamResult;
use clap::{ArgGroup, Args};
use rewrite_fn::rewrite_fns;
use rustc_const_eval::interpret::Pointer;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;
use smallvec::SmallVec;
use utils::{
    dsa::fixed_shape::VecVec,
    rewrite::{Rewrite, RewriteMode},
    rustc::RustProgram,
};

extern crate rustc_abi;
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
#[derive(Clone, Debug)]
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
    rust_program: &RustProgram<'tcx>,
    analysis: &Analysis,
    rewrite_mode: RewriteMode,
    options: RefactorOptions,
) -> anyhow::Result<()> {
    let fn_decision = FnLocals::new(rust_program, analysis, &options);

    if options.verbose {
        let mut rewriter = VerboseRewriter { rewriter: vec![] };
        rewrite(
            rust_program,
            &fn_decision,
            &options,
            analysis,
            &mut rewriter,
        )?;
        rewriter.write(rewrite_mode);
    } else {
        let mut rewriter = vec![];

        rewrite(
            rust_program,
            &fn_decision,
            &options,
            analysis,
            &mut rewriter,
        )?;

        rewriter.write(rewrite_mode);
    }

    Ok(())
}

fn rewrite(
    rust_program: &RustProgram,
    fn_decision: &FnLocals,
    options: &RefactorOptions,
    analysis: &Analysis,
    rewriter: &mut impl Rewrite,
) -> anyhow::Result<()> {
    rewrite_fns(
        &rust_program.functions,
        &fn_decision,
        rewriter,
        rust_program.tcx,
        options,
        analysis,
    );

    Ok(())
}

pub struct Analysis {
    output_param_result: OutputParamResult,
    promoted_mut_ref_result: PromotedMutRefResult,
}

impl Analysis {
    pub fn new(
        output_param_result: OutputParamResult,
        promoted_mut_ref_result: PromotedMutRefResult,
    ) -> Self {
        Analysis {
            output_param_result,
            promoted_mut_ref_result,
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

    fn is_as_is(&self) -> bool {
        matches!(*self, PointerKind::Raw(RawMeta::AsIs))
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
    AsIs,
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
                    if let Some(adt_def) = field_ty.skip_binder().ty_adt_def() {
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

    pub fn new(
        _rust_program: &RustProgram,
        _analysis: &Analysis,
        _options: &RefactorOptions,
    ) -> Self {
        todo!();
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

    pub fn new(rust_program: &RustProgram, analysis: &Analysis, options: &RefactorOptions) -> Self {
        let mut did_idx = FxHashMap::default();
        did_idx.reserve(rust_program.functions.len());
        let mut fn_locals = VecVec::with_capacity(
            rust_program.functions.len(),
            rust_program.functions.iter().fold(0, |acc, did| {
                let r#fn = &*rust_program
                    .tcx
                    .mir_drops_elaborated_and_const_checked(did.expect_local())
                    .borrow();
                acc + r#fn.local_decls.len()
            }),
        );

        let no_attempt = options
            .no_attempt
            .as_deref()
            .map(|pattern| regex::Regex::new(pattern).expect("bad fn name patterns"));

        let output_param_total_num = analysis
            .output_param_result
            .values()
            .map(|params| params.iter().count())
            .sum::<usize>();
        let promoted_mut_ref_total_num = analysis
            .promoted_mut_ref_result
            .values()
            .map(|params| params.iter().count())
            .sum::<usize>();
        println!(
            "output params: {output_param_total_num}, promoted mut refs: {promoted_mut_ref_total_num}"
        );

        for (idx, did) in rust_program.functions.iter().enumerate() {
            let output_params = analysis.output_param_result.get(did).unwrap();
            let promoted_mut_refs = analysis.promoted_mut_ref_result.get(did).unwrap();

            let body = &*rust_program
                .tcx
                .mir_drops_elaborated_and_const_checked(did.expect_local())
                .borrow();

            let no_attempt = matches!(no_attempt.as_ref().map(|regex| regex.is_match(&rust_program.tcx.def_path_str(*did))), Some(matched) if matched);

            for (local, _local_decl) in body.local_decls.iter_enumerated() {
                let mut local_decision: SmallVec<[PointerKind; 3]> = SmallVec::with_capacity(1);
                let is_output_param = output_params.contains(local);
                let is_promoted_mut_ref = promoted_mut_refs.contains(local);

                // is_output_param -> is_promoted_mut_ref
                assert!(!is_output_param || is_promoted_mut_ref);

                if is_output_param || is_promoted_mut_ref {
                    // if is_output_param {
                    // if is_promoted_mut_ref {
                    local_decision.push(PointerKind::Mut);
                } else {
                    local_decision.push(PointerKind::Raw(RawMeta::AsIs));
                }
                local_decision.push(PointerKind::Raw(RawMeta::AsIs));
                local_decision.push(PointerKind::Raw(RawMeta::AsIs));

                if options.no_box || no_attempt {
                    for pointer_kind in &mut local_decision {
                        if pointer_kind.is_move() {
                            *pointer_kind = PointerKind::Raw(RawMeta::Move)
                        }
                    }
                }
                fn_locals.push_element(local_decision);
            }

            fn_locals.complete_cur_vec();

            did_idx.insert(*did, idx);
        }

        let fn_locals = fn_locals.complete();
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
        let original = utils::rewrite::get_snippet(tcx, span).text.1;
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
