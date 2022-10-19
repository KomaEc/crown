use common::data_structure::vec_array::VecArray;
use petgraph::{algo::TarjanScc, prelude::DiGraphMap};
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{TyCtxt, TyKind};
use rustc_type_ir::TyKind::Adt;

use crate::{
    ownership::Precision,
    ptr::{abstract_ty, Measurable, Measure},
};

impl Measurable for StructTopology {
    #[inline]
    fn measure(&self, ty: rustc_middle::ty::Ty, ptr_chased: u32) -> Measure {
        let max_ptr_depth = self.offset_of.len() as u32;

        let (ptr_depth, maybe_adt) = abstract_ty(ty);

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
            .unwrap_or_default()
    }

    #[inline]
    fn measure_field_offset(
        &self,
        adt_def: rustc_middle::ty::AdtDef,
        field: usize,
        ptr_chased: u32,
    ) -> Measure {
        assert!(adt_def.is_struct());
        let Some(field_offsets) = self.field_offsets(&adt_def.did(), ptr_chased) else { return 0 };
        field_offsets[field]
    }

    fn max_precision(&self) -> Precision {
        // self.max_ptr_depth()
        self.offset_of.len() as Precision
    }
}

pub struct StructTopology {
    /// Structs in post order of the dependency graph.
    /// Dependency graph encodes direct dependencies between user defined structs.
    /// For instance, in `struct S { f: T } struct T;`
    /// `S` is considered dependent on `T`;
    /// in `struct S { f: *mut T } struct T;`
    /// `S` is not considered dependent on `T`
    pub post_order: Vec<DefId>,
    did_idx: FxHashMap<DefId, usize>,
    /// (precision - 1) -> struct -> field -> aggregate offset start of this field
    offset_of: Vec<VecArray<u32>>,
    /// (precision - 1) -> struct -> Vec<((struct, field, derefs), offset)>
    leaf_nodes: Vec<VecArray<((DefId, usize, u32), u32)>>,
}

impl StructTopology {
    // TODO refactor using `StructDependency`
    pub fn new(tcx: TyCtxt, structs: &[DefId]) -> Self {
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

        let offset_of = vec![];

        let mut this = StructTopology {
            post_order,
            did_idx,
            offset_of,
            leaf_nodes: vec![],
        };

        this.increase_precision(tcx);

        this
    }

    #[inline]
    pub fn structs_in_post_order(&self) -> &[DefId] {
        &self.post_order[..]
    }

    #[inline]
    pub fn struct_size(&self, did: &DefId, ptr_chased: u32) -> Option<Measure> {
        let idx = self.did_idx.get(did).copied()?;
        if ptr_chased as usize >= self.offset_of.len() {
            return None;
        }
        let offset_of = &self.offset_of[self.offset_of.len() - 1 - ptr_chased as usize];
        Some(offset_of[idx].last().copied().unwrap())
    }

    #[inline]
    pub fn field_offsets(&self, did: &DefId, ptr_chased: u32) -> Option<&[Measure]> {
        let idx = self.did_idx.get(did).copied()?;
        if ptr_chased as usize >= self.offset_of.len() {
            return None;
        }
        let offset_of = &self.offset_of[self.offset_of.len() - 1 - ptr_chased as usize];
        Some(&offset_of[idx])
    }

    #[inline]
    pub fn leaf_nodes(
        &self,
        did: &DefId,
        ptr_chased: u32,
    ) -> Option<&[((DefId, usize, u32), u32)]> {
        let idx = self.did_idx.get(did).copied()?;
        if ptr_chased as usize >= self.leaf_nodes.len() {
            return None;
        }
        let leaf_nodes = &self.leaf_nodes[self.offset_of.len() - 1 - ptr_chased as usize];
        Some(&leaf_nodes[idx])
    }

    #[inline]
    pub fn increase_precision(&mut self, tcx: TyCtxt) {
        let max_ptr_depth = self.offset_of.len() as u32 + 1;

        let data_capacity = self
            .offset_of
            .last()
            .map(|offset_of| offset_of.everything().len())
            .unwrap_or_default();
        let mut offset_of = VecArray::with_capacity(self.post_order.len(), data_capacity);
        let mut leaf_nodes = VecArray::with_capacity(self.post_order.len(), data_capacity);
        for &did in &self.post_order {
            let Adt(adt_def, subst_ref) = tcx.type_of(did).kind() else { unreachable!("impossible") };
            assert!(adt_def.is_struct());

            let mut offset = 0;
            offset_of.add_item_to_array(offset);

            for (field, field_def) in adt_def.all_fields().enumerate() {
                let field_ty = field_def.ty(tcx, subst_ref);
                let (ptr_depth, maybe_adt) = abstract_ty(field_ty);
                if ptr_depth >= max_ptr_depth {
                    leaf_nodes.add_item_to_array((
                        (did, field, max_ptr_depth - 1),
                        offset + max_ptr_depth - 1,
                    ));
                    offset += max_ptr_depth;
                } else if let Some(&idx) = maybe_adt.and_then(|adt| self.did_idx.get(&adt.did())) {
                    if ptr_depth == 0 {
                        for (ctxt, inner_offset) in leaf_nodes
                            .get_constructed(idx)
                            .iter()
                            .copied()
                            .collect::<smallvec::SmallVec<[_; 4]>>()
                        {
                            leaf_nodes.add_item_to_array((ctxt, offset + inner_offset));
                        }

                        offset += offset_of.get_constructed(idx).last().unwrap();
                    } else {
                        let leaves =
                            &self.leaf_nodes[(max_ptr_depth - ptr_depth - 1) as usize][idx];
                        for &(ctxt, inner_offset) in leaves {
                            leaf_nodes.add_item_to_array((ctxt, offset + ptr_depth + inner_offset));
                        }

                        offset += ptr_depth
                            + self.offset_of[(max_ptr_depth - ptr_depth - 1) as usize][idx]
                                .last()
                                .unwrap();
                    }
                } else {
                    offset += ptr_depth
                }
                offset_of.add_item_to_array(offset);
            }
            offset_of.done_with_array();
            leaf_nodes.done_with_array();
        }

        let offset_of = offset_of.done();
        let leaf_nodes = leaf_nodes.done();

        self.leaf_nodes.push(leaf_nodes);

        self.offset_of.push(offset_of);
    }
}

#[cfg(test)]
mod tests {
    use common::CrateData;

    use super::StructTopology;
    use crate::CrateCtxt;

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
            let program = CrateCtxt::from(crate_data);
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
                program.struct_topology.field_offsets(&s, 0).unwrap(),
                [0, 2, 3, 4] //.map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_topology.field_offsets(&t, 0).unwrap(),
                [0, 1, 2] //.map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_topology.field_offsets(&u, 0).unwrap(),
                [0, 0, 1, 1] //.map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_topology.field_offsets(&v, 0).unwrap(),
                [0, 0] //.map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_topology.field_offsets(&w, 0).unwrap(),
                [0, 1] //.map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_topology.field_offsets(&x, 0).unwrap(),
                [0] //.map(|x| Offset::from_u32(x))
            );
        })
    }

    const TEXT2: &str =
        "struct Node { data: Data, left: *mut Node, right: *mut Node } struct Data;";

    #[test]
    fn test2() {
        common::test::run_compiler_with(TEXT2.into(), |tcx, _, structs| {
            let mut struct_topology = StructTopology::new(tcx, &structs);
            let &node = struct_topology
                .post_order
                .iter()
                .find(|&&did| {
                    let "Node" = tcx.def_path_str(did).as_str() else { return false };
                    true
                })
                .unwrap();

            struct_topology.increase_precision(tcx);
            struct_topology.increase_precision(tcx);
            assert_eq!(
                struct_topology
                    .leaf_nodes(&node, 1)
                    .unwrap()
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>(),
                vec![1, 2, 4, 5]
            );

            assert_eq!(
                struct_topology
                    .leaf_nodes(&node, 2)
                    .unwrap()
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>(),
                vec![0, 1]
            );
        })
    }

    const TEXT3: &str = "struct S { f: *mut *mut S, g: *mut *mut S }";
    #[test]
    fn test3() {
        common::test::run_compiler_with(TEXT3.into(), |tcx, _, structs| {
            let mut struct_topology = StructTopology::new(tcx, &structs);
            let &node = struct_topology
                .post_order
                .iter()
                .find(|&&did| {
                    let "S" = tcx.def_path_str(did).as_str() else { return false };
                    true
                })
                .unwrap();
            // println!("{:?}", struct_topology.leaf_nodes(&node, 0).unwrap());

            assert_eq!(
                struct_topology
                    .leaf_nodes(&node, 0)
                    .unwrap()
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>(),
                vec![0, 1]
            );
            struct_topology.increase_precision(tcx);
            assert_eq!(
                struct_topology
                    .leaf_nodes(&node, 0)
                    .unwrap()
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>(),
                vec![1, 3]
            );
            struct_topology.increase_precision(tcx);
            assert_eq!(
                struct_topology
                    .leaf_nodes(&node, 0)
                    .unwrap()
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>(),
                vec![2, 3, 6, 7]
            );
        })
    }

    const TEXT4: &str = "struct S { f: *mut S, g: *mut *mut S }";
    #[test]
    fn test4() {
        common::test::run_compiler_with(TEXT4.into(), |tcx, _, structs| {
            let mut struct_topology = StructTopology::new(tcx, &structs);
            let &node = struct_topology
                .post_order
                .iter()
                .find(|&&did| {
                    let "S" = tcx.def_path_str(did).as_str() else { return false };
                    true
                })
                .unwrap();
            // println!("{:?}", struct_topology.leaf_nodes(&node, 0).unwrap());
            assert_eq!(
                struct_topology
                    .leaf_nodes(&node, 0)
                    .unwrap()
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>(),
                vec![0, 1]
            );
            struct_topology.increase_precision(tcx);
            assert_eq!(
                struct_topology
                    .leaf_nodes(&node, 0)
                    .unwrap()
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>(),
                vec![1, 2, 4]
            );
            struct_topology.increase_precision(tcx);
            assert_eq!(
                struct_topology
                    .leaf_nodes(&node, 0)
                    .unwrap()
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>(),
                vec![2, 3, 5, 8, 9]
            );
        })
    }
}
