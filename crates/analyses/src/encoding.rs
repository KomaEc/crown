use std::ops::{Add, Range};

use rustc_hir::def_id::DefId;
use rustc_middle::ty::{Ty, TyCtxt, TyKind};
use utils::rustc_hash::FxHashMap;

use utils::dsa::fixed_shape::VecVec;

/// Mapping locals of a function/fields of a struct
/// to a set of constraint variables
/// [`DefId`] -> entity -> [`std::ops::Range<Idx>`]
pub struct Encoding<Idx> {
    pub did_idx: FxHashMap<DefId, usize>,
    pub contents: VecVec<Idx>,
}

pub fn encode_structs<Idx, F>(
    initial: Idx,
    structs: &[DefId],
    tcx: TyCtxt,
    mut count_vars: F,
) -> (StructFields<Idx>, Idx)
where
    Idx: Add<usize, Output = Idx> + Clone,
    F: FnMut(Ty) -> usize,
{
    let mut did_idx = FxHashMap::default();
    did_idx.reserve(structs.len());

    let mut vars = VecVec::new();

    let mut next: Idx = initial;

    for (idx, r#struct) in structs.iter().enumerate() {
        did_idx.insert(*r#struct, idx);
        let struct_ty = tcx.type_of(*r#struct).skip_binder();
        let TyKind::Adt(adt_def, substs) = struct_ty.kind() else {
            unreachable!()
        };
        assert!(adt_def.is_struct());
        for field_def in adt_def.all_fields() {
            let field_ty = field_def.ty(tcx, substs);
            let ptr_count = count_vars(field_ty);
            vars.push_element(next.clone());
            next = next + ptr_count;
        }
        vars.push_element(next.clone());
        vars.complete_cur_vec();
    }
    let vars = vars.complete();

    (
        StructFields(Encoding {
            did_idx,
            contents: vars,
        }),
        next,
    )
}

pub fn encode_fns<Idx, F>(
    initial: Idx,
    fns: &[DefId],
    tcx: TyCtxt,
    mut count_vars: F,
) -> (FnLocals<Idx>, Idx)
where
    Idx: Add<usize, Output = Idx> + Clone,
    F: FnMut(Ty) -> usize,
{
    let mut did_idx = FxHashMap::default();
    did_idx.reserve(fns.len());

    let mut vars = VecVec::new();

    let mut next: Idx = initial;

    for (idx, r#fn) in fns.iter().enumerate() {
        did_idx.insert(*r#fn, idx);
        // let body = tcx.optimized_mir(*r#fn);
        let body = &*tcx
            .mir_drops_elaborated_and_const_checked(r#fn.expect_local())
            .borrow();
        for local_decl in &body.local_decls {
            let ptr_count = count_vars(local_decl.ty);
            vars.push_element(next.clone());
            next = next + ptr_count;
        }
        vars.push_element(next.clone());
        vars.complete_cur_vec();
    }
    let vars = vars.complete();

    (
        FnLocals(Encoding {
            did_idx,
            contents: vars,
        }),
        next,
    )
}

impl<Idx: Copy> Encoding<Idx> {
    #[inline]
    pub fn contents_iter(&self, did: &DefId) -> impl Iterator<Item = Range<Idx>> + '_ {
        let idx = self.did_idx[did];
        self.contents[idx]
            .array_windows()
            .map(|&[start, end]| start..end)
    }

    #[inline]
    pub fn content(&self, did: &DefId, idx: usize) -> Range<Idx> {
        let outer_idx = self.did_idx[did];
        self.contents[outer_idx][idx]..self.contents[outer_idx][idx + 1]
    }
}

pub struct StructFields<Idx>(pub Encoding<Idx>);
pub struct FnLocals<Idx>(pub Encoding<Idx>);

impl<Idx: Copy> StructFields<Idx> {
    /// [`fields()`] returns a slice of [`Range<T>`] that is in lock-step with [`all_fields()`]
    #[inline]
    pub fn fields(&self, did: &DefId) -> impl Iterator<Item = Range<Idx>> + '_ {
        self.0.contents_iter(did)
    }

    #[inline]
    pub fn field(&self, did: &DefId, f: usize) -> Range<Idx> {
        self.0.content(did, f)
    }
}

impl<Idx: Copy> FnLocals<Idx> {
    /// [`locals()`] returns a slice of [`Range<Var>`] that is in lock-step with [`local_decls`]
    /// #[inline]
    pub fn locals(&self, did: &DefId) -> impl Iterator<Item = Range<Idx>> + '_ {
        self.0.contents_iter(did)
    }

    #[inline]
    pub fn local(&self, did: &DefId, local: usize) -> Range<Idx> {
        self.0.content(did, local)
    }
}
