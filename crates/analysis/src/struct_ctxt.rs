use common::data_structure::vec_vec::VecVec;
use petgraph::{algo::TarjanScc, prelude::DiGraphMap};
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{Ty, TyCtxt, TyKind};
use rustc_type_ir::TyKind::Adt;

use crate::{
    ownership::Precision,
    ptr::{decompose_ty, Measurable, Measure},
};

impl<'tcx> Measurable<'tcx> for StructCtxt<'tcx> {
    #[inline]
    fn measure(&self, ty: rustc_middle::ty::Ty, ptr_chased: u32) -> Measure {
        // let max_ptr_depth = self.offset_of.len() as u32;
        let max_ptr_depth = self.max_ptr_chased() as u32;

        let (ptr_depth, maybe_adt) = decompose_ty(ty);

        if ptr_depth + ptr_chased >= max_ptr_depth {
            max_ptr_depth - ptr_chased
        } else if let Some(adt) = maybe_adt {
            ptr_depth + self.measure_adt(adt, ptr_depth + ptr_chased)
        } else {
            ptr_depth
        }
    }

    #[inline]
    fn measure_adt(&self, adt_def: rustc_middle::ty::AdtDef, ptr_chased: u32) -> Measure {
        self.struct_size(&adt_def.did(), ptr_chased)
    }

    #[inline]
    fn field_offset(
        &self,
        adt_def: rustc_middle::ty::AdtDef,
        field: usize,
        ptr_chased: u32,
    ) -> Measure {
        assert!(adt_def.is_struct());
        let Some(field_offsets) = self.field_offsets(&adt_def.did(), ptr_chased) else { return 0 };
        field_offsets[field]
    }

    fn max_ptr_chased(&self) -> Precision {
        // self.max_ptr_depth()
        (self.offset_of.len() - 1) as Precision
    }

    fn leaf_nodes(&self, adt_def: rustc_middle::ty::AdtDef, ptr_chased: u32) -> &[(Ty<'tcx>, u32)] {
        let idx = self.did_idx[&adt_def.did()];
        assert!(ptr_chased as usize <= self.offset_of.len());
        let leaf_nodes = &self.leaf_nodes[self.leaf_nodes.len() - 1 - ptr_chased as usize];
        &leaf_nodes[idx]
    }

    fn absolute_precision(&self, ty: Ty, measure: Measure) -> Precision {
        let max_precision = self.max_ptr_chased();
        assert!(max_precision > 0);
        let mut ptr_chased = max_precision;
        while self.measure(ty, ptr_chased as u32) < measure {
            ptr_chased = ptr_chased.checked_sub(1).unwrap()
        }

        assert_eq!(self.measure(ty, ptr_chased as u32), measure);

        max_precision - ptr_chased
    }
}

pub struct StructCtxt<'tcx> {
    /// Structs in post order of the dependency graph.
    /// Dependency graph encodes direct dependencies between user defined structs.
    /// For instance, in `struct S { f: T } struct T;`
    /// `S` is considered dependent on `T`;
    /// in `struct S { f: *mut T } struct T;`
    /// `S` is not considered dependent on `T`
    pub post_order: Vec<DefId>,
    did_idx: FxHashMap<DefId, usize>,
    /// precision -> struct -> field -> aggregate offset start of this field
    offset_of: Vec<VecVec<u32>>,
    /// precision -> struct -> Vec<(Ty, offset_should_be)>
    leaf_nodes: Vec<VecVec<(Ty<'tcx>, u32)>>,
}

impl<'tcx> StructCtxt<'tcx> {
    // TODO refactor using `StructDependency`
    pub fn new(tcx: TyCtxt<'tcx>, structs: &[DefId]) -> Self {
        let mut graph = DiGraphMap::with_capacity(structs.len(), structs.len());
        structs.iter().for_each(|did| {
            graph.add_node(*did);
        });
        for did in structs.iter() {
            let Adt(adt_def, subst_ref) = tcx.type_of(did).kind() else { unreachable!("impossible") };
            assert!(adt_def.is_struct());
            for field_def in adt_def.all_fields() {
                let mut ty = field_def.ty(tcx, subst_ref);
                while let TyKind::Array(inner_ty, _) = ty.kind() {
                    ty = *inner_ty;
                }
                if let TyKind::Adt(sub_adt_def, _) = ty.kind() {
                    if graph.contains_node(sub_adt_def.did()) {
                        graph.add_edge(*did, sub_adt_def.did(), ());
                    }
                }
            }
        }

        let mut post_order = Vec::with_capacity(structs.len());
        TarjanScc::new().run(&graph, |nodes| post_order.extend(nodes));

        let did_idx: FxHashMap<DefId, usize> = post_order
            .iter()
            .enumerate()
            .map(|(idx, &did)| (did, idx))
            .collect();

        let mut offset_of = VecVec::with_indices_capacity(post_order.len());
        let mut leaf_nodes = VecVec::with_indices_capacity(post_order.len());
        for &did in &post_order {
            let ty = tcx.type_of(did);
            let Adt(adt_def, _) = ty.kind() else { unreachable!("impossible") };
            assert!(adt_def.is_struct());

            offset_of.push_inner(0);
            for _ in adt_def.all_fields() {
                offset_of.push_inner(0);
            }
            offset_of.push();

            leaf_nodes.push_inner((ty, 0));
            leaf_nodes.push();
        }
        let offset_of = offset_of.done();
        let leaf_nodes = leaf_nodes.done();

        let mut this = StructCtxt {
            post_order,
            did_idx,
            offset_of: vec![offset_of],
            leaf_nodes: vec![leaf_nodes],
        };

        this.increase_precision(tcx);

        this
    }

    pub fn is_struct_of_concerned(&self, did: &DefId) -> bool {
        self.did_idx.contains_key(did)
    }

    pub fn did_idx(&self, did: &DefId) -> usize {
        self.did_idx[did]
    }

    #[inline]
    pub fn structs_in_post_order(&self) -> &[DefId] {
        &self.post_order[..]
    }

    #[inline]
    pub fn struct_size(&self, did: &DefId, ptr_chased: u32) -> Measure {
        let Some(idx) = self.did_idx.get(did).copied() else { return 0 };
        // assert!(ptr_chased as usize <= self.offset_of.len());
        let offset_of = &self.offset_of[self.offset_of.len() - 1 - ptr_chased as usize];
        offset_of[idx].last().copied().unwrap()
    }

    #[inline]
    pub fn field_offsets(&self, did: &DefId, ptr_chased: u32) -> Option<&[Measure]> {
        let idx = self.did_idx.get(did).copied()?;
        // assert!(ptr_chased as usize <= self.offset_of.len());
        let offset_of = &self.offset_of[self.offset_of.len() - 1 - ptr_chased as usize];
        Some(&offset_of[idx])
    }

    #[inline]
    pub fn increase_precision(&mut self, tcx: TyCtxt<'tcx>) {
        let max_ptr_depth = self.max_ptr_chased() as u32 + 1;

        let data_capacity = self
            .offset_of
            .last()
            .map(|offset_of| offset_of.everything().len())
            .unwrap_or_default();
        let mut offset_of = VecVec::with_capacity(self.post_order.len(), data_capacity);
        let mut leaf_nodes = VecVec::with_capacity(self.post_order.len(), data_capacity);
        for &did in &self.post_order {
            let Adt(adt_def, subst_ref) = tcx.type_of(did).kind() else { unreachable!("impossible") };
            assert!(adt_def.is_struct());

            let mut offset = 0;
            offset_of.push_inner(offset);

            for field_def in adt_def.all_fields() {
                let field_ty = field_def.ty(tcx, subst_ref);
                let (ptr_depth, maybe_adt) = decompose_ty(field_ty);
                if ptr_depth >= max_ptr_depth {
                    let mut leaf_ext_ty = field_ty;
                    for _ in 0..max_ptr_depth {
                        while let Some(ty) = leaf_ext_ty.builtin_index() {
                            leaf_ext_ty = ty;
                        }
                        leaf_ext_ty = leaf_ext_ty.builtin_deref(true).unwrap().ty;
                    }
                    leaf_nodes.push_inner((leaf_ext_ty, offset + max_ptr_depth));
                    offset += max_ptr_depth;
                } else if let Some(&idx) = maybe_adt.and_then(|adt| self.did_idx.get(&adt.did())) {
                    if ptr_depth == 0 {
                        for (leaf_ext_ty, inner_offset) in leaf_nodes
                            .get_constructed(idx)
                            .iter()
                            .copied()
                            .collect::<smallvec::SmallVec<[_; 4]>>()
                        {
                            leaf_nodes.push_inner((leaf_ext_ty, offset + inner_offset));
                        }

                        offset += offset_of.get_constructed(idx).last().unwrap();
                    } else {
                        let leaves = &self.leaf_nodes[(max_ptr_depth - ptr_depth) as usize][idx];
                        for &(leaf_ext_ty, inner_offset) in leaves {
                            leaf_nodes.push_inner((leaf_ext_ty, offset + ptr_depth + inner_offset));
                        }

                        offset += ptr_depth
                            + self.offset_of[(max_ptr_depth - ptr_depth) as usize][idx]
                                .last()
                                .unwrap();
                    }
                } else {
                    offset += ptr_depth
                }
                offset_of.push_inner(offset);
            }
            offset_of.push();
            leaf_nodes.push();
        }

        let offset_of = offset_of.done();
        let leaf_nodes = leaf_nodes.done();

        self.leaf_nodes.push(leaf_nodes);

        self.offset_of.push(offset_of);
    }
}

#[derive(Clone, Copy)]
pub struct RestrictedStructCtxt<'intra, 'tcx> {
    pub(crate) unrestricted: &'intra StructCtxt<'tcx>,
    allowed_ptr_depth: Precision,
}

impl<'tcx> StructCtxt<'tcx> {
    pub fn with_max_precision(&self, precision: Precision) -> RestrictedStructCtxt<'_, 'tcx> {
        RestrictedStructCtxt {
            unrestricted: self,
            allowed_ptr_depth: precision,
        }
    }
}

impl<'intra, 'tcx> Measurable<'tcx> for RestrictedStructCtxt<'intra, 'tcx> {
    fn measure(&self, ty: Ty, ptr_chased: u32) -> crate::ptr::Measure {
        let allowed = self.allowed_ptr_depth as u32;
        let maximum = self.unrestricted.max_ptr_chased() as u32;
        if allowed >= maximum {
            self.unrestricted.measure(ty, ptr_chased)
        } else {
            self.unrestricted
                .measure(ty, maximum - allowed + ptr_chased)
        }
    }

    fn measure_adt(
        &self,
        adt_def: rustc_middle::ty::AdtDef,
        ptr_chased: u32,
    ) -> crate::ptr::Measure {
        let allowed = self.allowed_ptr_depth as u32;
        let maximum = self.unrestricted.max_ptr_chased() as u32;
        if allowed >= maximum {
            self.unrestricted.measure_adt(adt_def, ptr_chased)
        } else {
            self.unrestricted
                .measure_adt(adt_def, maximum - allowed + ptr_chased)
        }
    }

    fn field_offset(
        &self,
        adt_def: rustc_middle::ty::AdtDef,
        field: usize,
        ptr_chased: u32,
    ) -> crate::ptr::Measure {
        let allowed = self.allowed_ptr_depth as u32;
        let maximum = self.unrestricted.max_ptr_chased() as u32;
        if allowed >= maximum {
            self.unrestricted.field_offset(adt_def, field, ptr_chased)
        } else {
            self.unrestricted
                .field_offset(adt_def, field, maximum - allowed + ptr_chased)
        }
    }

    fn max_ptr_chased(&self) -> Precision {
        std::cmp::min(self.allowed_ptr_depth, self.unrestricted.max_ptr_chased())
    }

    fn leaf_nodes(&self, adt_def: rustc_middle::ty::AdtDef, ptr_chased: u32) -> &[(Ty<'tcx>, u32)] {
        let allowed = self.allowed_ptr_depth as u32;
        let maximum = self.unrestricted.max_ptr_chased() as u32;
        if allowed >= maximum {
            self.unrestricted.leaf_nodes(adt_def, ptr_chased)
        } else {
            self.unrestricted
                .leaf_nodes(adt_def, maximum - allowed + ptr_chased)
        }
    }

    fn absolute_precision(&self, ty: Ty, measure: u32) -> Precision {
        self.unrestricted.absolute_precision(ty, measure)
    }
}

#[cfg(test)]
mod tests {
    use common::CrateData;

    use super::StructCtxt;
    use crate::{ptr::Measurable, CrateCtxt};

    const TEXT1: &str = "
    struct s {
        f: t,
        g: *mut i32,
        h: *mut s,
    }
    struct t {
        f: *mut i32,
        g: u
    }
    struct u {
        f: v,
        g: w,
        h: x
    }
    struct v {
        f: i32,
    }
    struct w {
        f: *mut i32
    }
    struct x;
    ";

    #[test]
    fn test1() {
        common::test::run_compiler_with(TEXT1.into(), |tcx, functions, structs| {
            let crate_data = CrateData::new(tcx, functions, structs);
            let program = CrateCtxt::new(&crate_data);
            macro_rules! define_structs {
                ($( $x: ident ),*) => {
                    $(
                        let &$x = program
                            .structs()
                            .iter()
                            .find(|&&did| {
                            let stringify!($x) = program.tcx.def_path_str(did).as_str() else { return false };
                            true
                        })
                        .unwrap();
                    )*
                };
            }
            define_structs!(s, t, u, v, w, x);
            assert_eq!(
                program.struct_ctxt.field_offsets(&s, 0).unwrap(),
                [0, 2, 3, 4] //.map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_ctxt.field_offsets(&t, 0).unwrap(),
                [0, 1, 2] //.map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_ctxt.field_offsets(&u, 0).unwrap(),
                [0, 0, 1, 1] //.map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_ctxt.field_offsets(&v, 0).unwrap(),
                [0, 0] //.map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_ctxt.field_offsets(&w, 0).unwrap(),
                [0, 1] //.map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_ctxt.field_offsets(&x, 0).unwrap(),
                [0] //.map(|x| Offset::from_u32(x))
            );
        })
    }

    const TEXT2: &str =
        "struct Node { data: Data, left: *mut Node, right: *mut Node } struct Data;";

    #[test]
    fn test2() {
        common::test::run_compiler_with(TEXT2.into(), |tcx, _, structs| {
            let mut struct_ctxt = StructCtxt::new(tcx, &structs);
            let &node = struct_ctxt
                .post_order
                .iter()
                .find(|&&did| {
                    let "Node" = tcx.def_path_str(did).as_str() else { return false };
                    true
                })
                .unwrap();
            let node = tcx.adt_def(node);

            struct_ctxt.increase_precision(tcx);
            struct_ctxt.increase_precision(tcx);
            assert_eq!(
                struct_ctxt
                    .leaf_nodes(node, 1)
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>(),
                vec![2, 3, 5, 6]
            );

            assert_eq!(
                struct_ctxt
                    .leaf_nodes(node, 2)
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>(),
                vec![1, 2]
            );
        })
    }

    const TEXT3: &str = "struct S { f: *mut *mut S, g: *mut *mut S }";
    #[test]
    fn test3() {
        common::test::run_compiler_with(TEXT3.into(), |tcx, _, structs| {
            let mut struct_ctxt = StructCtxt::new(tcx, &structs);
            let &node = struct_ctxt
                .post_order
                .iter()
                .find(|&&did| {
                    let "S" = tcx.def_path_str(did).as_str() else { return false };
                    true
                })
                .unwrap();
            let node = tcx.adt_def(node);
            // println!("{:?}", struct_ctxt.leaf_nodes(&node, 0).unwrap());

            assert_eq!(
                struct_ctxt
                    .leaf_nodes(node, 0)
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>(),
                vec![1, 2]
            );
            struct_ctxt.increase_precision(tcx);
            assert_eq!(
                struct_ctxt
                    .leaf_nodes(node, 0)
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>(),
                vec![2, 4]
            );
            struct_ctxt.increase_precision(tcx);
            assert_eq!(
                struct_ctxt
                    .leaf_nodes(node, 0)
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>(),
                vec![3, 4, 7, 8]
            );
        })
    }

    const TEXT4: &str = "struct S { f: *mut S, g: *mut *mut S }";
    #[test]
    fn test4() {
        common::test::run_compiler_with(TEXT4.into(), |tcx, _, structs| {
            let mut struct_ctxt = StructCtxt::new(tcx, &structs);
            let &node = struct_ctxt
                .post_order
                .iter()
                .find(|&&did| {
                    let "S" = tcx.def_path_str(did).as_str() else { return false };
                    true
                })
                .unwrap();
            let node = tcx.adt_def(node);
            // println!("{:?}", struct_ctxt.leaf_nodes(&node, 0).unwrap());
            assert_eq!(
                struct_ctxt
                    .leaf_nodes(node, 0)
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>(),
                vec![1, 2]
            );
            struct_ctxt.increase_precision(tcx);
            assert_eq!(
                struct_ctxt
                    .leaf_nodes(node, 0)
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>(),
                vec![2, 3, 5]
            );
            struct_ctxt.increase_precision(tcx);
            assert_eq!(
                struct_ctxt
                    .leaf_nodes(node, 0)
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>(),
                vec![3, 4, 6, 9, 10]
            );
        })
    }

    // const TEXT5: &str = "struct S { f: *mut Data, g: *mut S } struct Data { f: i32 }";
    // #[test]
    // fn test5() {
    //     common::test::run_compiler_with(TEXT5.into(), |tcx, _, structs| {
    //         let mut struct_ctxt = StructTopology::new(tcx, &structs);
    //         let &node = struct_ctxt
    //             .post_order
    //             .iter()
    //             .find(|&&did| {
    //                 let "S" = tcx.def_path_str(did).as_str() else { return false };
    //                 true
    //             })
    //             .unwrap();
    //         let node = tcx.adt_def(node);
    //         println!("{:?}", struct_ctxt.leaf_nodes(node, 0).unwrap());
    //         struct_ctxt.increase_precision(tcx);
    //         println!("{:?}", struct_ctxt.leaf_nodes(node, 0).unwrap());
    //         struct_ctxt.increase_precision(tcx);
    //         println!("{:?}", struct_ctxt.leaf_nodes(node, 0).unwrap());
    //         for &(leaf_ext_ty, _) in struct_ctxt.leaf_nodes(node, 0).unwrap() {
    //             let delta = 1;
    //             let leaf_ext_measure = struct_ctxt.measure(leaf_ext_ty, struct_ctxt.max_precision() as u32 - delta);
    //             println!("leaf_ext_ty ~ {leaf_ext_measure}")
    //         }
    //     })
    // }
}
