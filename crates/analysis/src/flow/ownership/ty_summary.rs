use rustc_abi::FieldIdx;
use rustc_index::IndexVec;
use rustc_middle::{
    mir::Body,
    ty::{Ty, TyCtxt},
};
use rustc_span::def_id::DefId;
use rustc_type_ir::TyKind::{self, Adt};

use super::{
    constraint::{Database, OwnershipToken, StorageMode, Z3Database},
    FnSig, InterproceduralView, Param, StructMap,
};
use crate::{
    call_graph::{CallGraph, Monotonicity},
    flow::ownership::constraint::Constraint,
    lattice::FlatSet,
};

pub struct TySummary {
    pub fields: IndexVec<FieldIdx, OwnershipToken>,
}

impl TySummary {
    pub fn new<Mode: StorageMode, DB: Database<Mode>>(
        def_id: DefId,
        database: &mut DB,
        tcx: TyCtxt,
    ) -> Self {
        let ty = tcx.type_of(def_id).skip_binder();
        let Adt(adt_def, args) = ty.kind() else {
            unreachable!("impossible")
        };
        assert!(adt_def.is_struct());
        let mut fields = IndexVec::new();
        for field in adt_def.all_fields() {
            let field_ty = field.ty(tcx, args);
            let (num_wrapping_pointers, _) = unwrap_pointers(field_ty);
            let tokens = database.new_tokens(num_wrapping_pointers);
            fields.push(tokens.start);
        }

        Self { fields }
    }
}

impl<'analysis, 'z3, Mode: StorageMode> InterproceduralView<'analysis, Mode, Z3Database<'z3>> {
    pub(super) fn summarise_types(
        &mut self,
        ty_summaries: &StructMap<TySummary>,
        call_graph: &CallGraph,
        tcx: TyCtxt,
    ) {
        for (&def_id, fn_sig) in self.fn_sigs.iter() {
            let body = tcx.optimized_mir(def_id);
            let monotonicity = call_graph.monotonicity(def_id);
            self.summarise_fn_sig(body, fn_sig, monotonicity, ty_summaries, tcx);
        }
    }

    fn summarise_fn_sig<'tcx>(
        &mut self,
        body: &Body<'tcx>,
        fn_sig: &FnSig<OwnershipToken>,
        monotonicity: FlatSet<Monotonicity>,
        ty_summaries: &StructMap<TySummary>,
        tcx: TyCtxt<'tcx>,
    ) {
        let ret_ty = body.return_ty();
        let output = fn_sig.output;
        self.summarise(ret_ty, output, ty_summaries, tcx);
        for (ty, param) in body
            .args_iter()
            .map(|local| body.local_decls[local].ty)
            .zip(fn_sig.inputs.iter().copied())
        {
            match param {
                Param::Normal(input) => {
                    if matches!(monotonicity, FlatSet::Elem(Monotonicity::Dealloc)) {
                        self.summarise(ty, input, ty_summaries, tcx);
                    }
                }
                Param::Output(param) => {
                    if !matches!(monotonicity, FlatSet::Elem(Monotonicity::Alloc)) {
                        self.summarise(ty, param.r#use, ty_summaries, tcx);
                    }
                    if !matches!(monotonicity, FlatSet::Elem(Monotonicity::Dealloc)) {
                        self.summarise(ty, param.def, ty_summaries, tcx);
                    }
                }
            }
        }
    }

    fn summarise<'tcx>(
        &mut self,
        ty: Ty<'tcx>,
        start_token: OwnershipToken,
        ty_summaries: &StructMap<TySummary>,
        tcx: TyCtxt<'tcx>,
    ) {
        let k_limit = self.access_paths.max_k_limit();
        let size = self.access_paths.size_of(k_limit, ty);
        let (num_wrapping_pointers, ty) = unwrap_pointers(ty);
        if size <= num_wrapping_pointers {
            return;
        }
        let TyKind::Adt(adt_def, arg) = ty.kind() else {
            unreachable!()
        };
        assert!(adt_def.is_struct());
        assert!(adt_def.did().is_local());

        let k_limit = k_limit - num_wrapping_pointers;
        let start_token = start_token + num_wrapping_pointers;

        let ty_summary = ty_summaries.get(&adt_def.did()).unwrap();
        for (field_def, offset, field_summary) in itertools::izip!(
            adt_def.all_fields(),
            self.access_paths
                .offsets_of(k_limit, adt_def.did())
                .iter()
                .copied(),
            ty_summary.fields.iter().cloned()
        ) {
            let offset: usize = offset;
            let field_ty = field_def.ty(tcx, arg);
            let (num_wrapping_pointers, _) = unwrap_pointers(field_ty);
            let num_pointers_chased = std::cmp::min(k_limit, num_wrapping_pointers);
            let tokens = start_token + offset..start_token + offset + num_pointers_chased;
            let field_summary = field_summary..field_summary + num_pointers_chased;
            for (x, y) in tokens.zip(field_summary) {
                <Z3Database<'z3> as Database<Mode>>::add(
                    self.database,
                    Constraint::Equal { x, y },
                    self.storage,
                );
            }
        }
    }
}

/// Decompose a type into levels of outside pointers and a (possible) adt
pub fn unwrap_pointers(ty: Ty) -> (usize, Ty) {
    let mut ty = ty;

    let mut level_pointers = 0;
    loop {
        if let Some(inner_ty) = ty.builtin_index() {
            ty = inner_ty;
            continue;
        }

        if let Some(ty_mut) = ty.builtin_deref(true) {
            ty = ty_mut.ty;
            level_pointers += 1;
            continue;
        }

        break;
    }

    (level_pointers, ty)
}
