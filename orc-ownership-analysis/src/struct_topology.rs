use orc_common::data_structure::vec_array::VecArray;
use petgraph::{algo::TarjanScc, prelude::DiGraphMap};
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{TyCtxt, TyKind};
use rustc_type_ir::TyKind::Adt;

pub(crate) type Offset = u32;

pub(crate) struct StructTopology {
    /// Structs in post order of the dependency graph.
    /// Dependency graph encodes direct dependencies between user defined structs.
    /// For instance, in `struct S { f: T } struct T;`
    /// `S` is considered dependent on `T`;
    /// in `struct S { f: *mut T } struct T;`
    /// `S` is not considered dependent on `T`
    pub(crate) post_order: Vec<DefId>,
    did_idx: FxHashMap<DefId, usize>,
    /// struct -> field -> aggregate offset start of this field
    /// an additional entry last() represents the sum
    /// TODO: will we need `Vec<VecArray<Offset>>`?
    /// First level index represents the level of pointer dereferenc
    /// we track
    offset_of: VecArray<Offset>,
}

impl StructTopology {
    pub(crate) fn new(tcx: TyCtxt, structs: Vec<DefId>) -> Self {
        let mut graph = DiGraphMap::with_capacity(structs.len(), structs.len());
        structs.iter().for_each(|did| {
            graph.add_node(*did);
        });
        for did in structs.iter() {
            let Adt(adt_def, subst_ref) = tcx.type_of(did).kind() else { unreachable!("impossible") };
            assert!(adt_def.is_struct());
            for field_def in adt_def.all_fields() {
                let ty = field_def.ty(tcx, subst_ref);
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

        let mut offset_of = VecArray::new(post_order.len());
        for &did in post_order.iter() {
            // println!("go {:?}", did);
            let Adt(adt_def, subst_ref) = tcx.type_of(did).kind() else { unreachable!("impossible") };
            assert!(adt_def.is_struct());

            let mut offset = 0; //Offset::from_u32(0);
            offset_of.add_item_to_array(offset);
            for field_def in adt_def.all_fields() {
                let mut ty = field_def.ty(tcx, subst_ref);
                // peel off arrays
                // Notice: this has to be in accordnace with TyExt
                while let TyKind::Array(inner_ty, _) = ty.kind() {
                    ty = *inner_ty;
                }
                offset = offset
                    + match ty.kind() {
                        TyKind::RawPtr(..) | TyKind::Ref(..) => 1,
                        TyKind::Adt(sub_adt_def, _) if sub_adt_def.is_box() => 1,
                        TyKind::Adt(sub_adt_def, _) => did_idx
                            .get(&sub_adt_def.did())
                            .and_then(|&field_did_idx| {
                                offset_of.get_constructed(field_did_idx).last().copied()
                                // .map(|&offset| offset.as_usize())
                            })
                            .unwrap_or(0),
                        _ => 0,
                    };
                offset_of.add_item_to_array(offset);
            }
            offset_of.done_with_array();
        }
        let offset_of = offset_of.done();

        StructTopology {
            post_order,
            did_idx,
            offset_of,
        }
    }

    #[inline]
    pub(crate) fn structs_in_post_order(&self) -> &[DefId] {
        &self.post_order[..]
    }

    // #[inline]
    // pub(crate) fn contains(&self, did: &DefId) -> bool {
    //     self.did_idx.contains_key(did)
    // }

    #[inline]
    pub(crate) fn struct_size(&self, did: &DefId) -> Option<Offset> {
        let idx = self.did_idx.get(did).copied()?;
        Some(self.offset_of[idx].last().copied().unwrap())
    }

    #[inline]
    pub(crate) fn field_offsets(&self, did: &DefId) -> Option<&[Offset]> {
        let idx = self.did_idx.get(did).copied()?;
        Some(&self.offset_of[idx])
    }
}

#[cfg(test)]
mod tests {
    use crate::CrateCtxt;

    const TEXT: &str = "
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
    fn test() {
        orc_common::test_infra::run_compiler_with(TEXT.into(), |tcx, functions, structs| {
            let program = CrateCtxt::new(tcx, functions, structs);
            macro_rules! define_structs {
                ($( $x: ident ),*) => {
                    $(
                        let $x = program
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
                program.struct_topology.field_offsets(&s).unwrap(),
                [0, 2, 3, 4] //.map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_topology.field_offsets(&t).unwrap(),
                [0, 1, 2] //.map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_topology.field_offsets(&u).unwrap(),
                [0, 0, 1, 1] //.map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_topology.field_offsets(&v).unwrap(),
                [0, 0] //.map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_topology.field_offsets(&w).unwrap(),
                [0, 1] //.map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_topology.field_offsets(&x).unwrap(),
                [0] //.map(|x| Offset::from_u32(x))
            )
        })
    }
}
