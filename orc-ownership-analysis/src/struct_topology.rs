// use crate::sparse_bit_vector::{self, SparseBitVectorGraph};
// use rustc_data_structures::graph::iterate::post_order_from;
use petgraph::{algo::TarjanScc, prelude::DiGraphMap};
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{TyCtxt, TyKind};
use rustc_type_ir::TyKind::Adt;

use crate::analysis::place_ext::place_abs::Offset;

pub(crate) struct StructTopology {
    /// Structs in post order of the dependency graph.
    /// Dependency graph encodes direct dependencies between user defined structs.
    /// For instance, in `struct S { f: T } struct T;`
    /// `S` is considered dependent on `T`;
    /// in `struct S { f: *mut T } struct T;`
    /// `S` is not considered dependent on `T`
    pub(crate) post_order: Vec<DefId>,
    /// struct -> field -> aggregate offset start of this field
    /// an additional entry last() represents the sum
    offset_of: FxHashMap<DefId, Vec<Offset>>,
}

impl StructTopology {
    pub(crate) fn new(tcx: TyCtxt, structs: Vec<DefId>) -> Self {
        let mut graph = DiGraphMap::with_capacity(structs.len(), structs.len());
        structs.iter().for_each(|did| {
            graph.add_node(*did);
        });
        for did in structs.iter() {
            let adt_def = tcx.adt_def(did);
            assert!(adt_def.is_struct());
            for field_def in adt_def.all_fields() {
                let ty = tcx.type_of(field_def.did);
                if let TyKind::Adt(sub_adt_def, _) = ty.kind() {
                    graph.add_edge(*did, sub_adt_def.did(), ());
                }
            }
        }

        let mut post_order = Vec::with_capacity(structs.len());
        TarjanScc::new().run(&graph, |nodes| post_order.extend(nodes));

        let mut offset_of = FxHashMap::default();

        for &did in &post_order {
            let Adt(adt_def, subst_ref) = tcx.type_of(did).kind() else { unreachable!("impossible") };
            assert!(adt_def.is_struct());

            let mut offset = Offset::from_u32(0);
            let mut offsets = vec![offset];

            offsets.extend(adt_def.all_fields().map(|field_def| {
                offset = offset
                    + match field_def.ty(tcx, subst_ref).kind() {
                        TyKind::RawPtr(..) | TyKind::Ref(..) => 1,
                        TyKind::Adt(sub_adt_def, _) if sub_adt_def.is_box() => 1,
                        TyKind::Adt(sub_adt_def, _) => {
                            offset_of
                                .get(&sub_adt_def.did())
                                .and_then(|offsets: &Vec<Offset>| {
                                    assert!(!offsets.is_empty());
                                    offsets.last().map(|&offset| offset.as_usize())
                                })
                                // non-user defined structs are ignored
                                .unwrap_or(0)
                        }
                        _ => 0,
                    };
                offset
            }));

            offset_of.insert(did, offsets);
        }

        StructTopology {
            post_order,
            offset_of,
        }
    }

    #[inline]
    pub(crate) fn structs_in_post_order(&self) -> &[DefId] {
        &self.post_order[..]
    }

    /// Return the total offset of a struct definition, `None` if
    /// `did` is a library struct/enum/union
    #[inline]
    pub(crate) fn struct_offset(&self, did: &DefId) -> Option<Offset> {
        let last = self.offset_of.get(did)?.last();
        assert!(last.is_some());
        last.map(|&offset| offset)
    }

    #[inline]
    pub(crate) fn field_offsets(&self, did: &DefId) -> Option<&[Offset]> {
        self.offset_of.get(did).map(|vec| &vec[..])
    }
}

#[cfg(test)]
mod tests {
    use crate::{analysis::place_ext::place_abs::Offset, CrateInfo};

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
            let program = CrateInfo::new(tcx, functions, structs);
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
                program.struct_topology().field_offsets(&s).unwrap(),
                [0, 2, 3, 4].map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_topology().field_offsets(&t).unwrap(),
                [0, 1, 2].map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_topology().field_offsets(&u).unwrap(),
                [0, 0, 1, 1].map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_topology().field_offsets(&v).unwrap(),
                [0, 0].map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_topology().field_offsets(&w).unwrap(),
                [0, 1].map(|x| Offset::from_u32(x))
            );
            assert_eq!(
                program.struct_topology().field_offsets(&x).unwrap(),
                [0].map(|x| Offset::from_u32(x))
            )
        })
    }
}
