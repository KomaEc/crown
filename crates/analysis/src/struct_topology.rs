use common::data_structure::vec_array::VecArray;
use petgraph::{algo::TarjanScc, prelude::DiGraphMap};
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{TyCtxt, TyKind};
use rustc_type_ir::TyKind::Adt;

use crate::ptr::{abstract_ty, Measurable, Measure};

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
        let Some(field_offsets) = self.field_offsets(&adt_def.did(), ptr_chased) else { panic!() };
        field_offsets[field]
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
    /// derefs -> struct -> field -> aggregate offset start of this field
    offset_of: Vec<VecArray<Measure>>,
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
        };

        this.next_stage(tcx);

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
    pub fn max_ptr_depth(&self) -> u32 {
        self.offset_of.len() as u32
    }

    #[inline]
    pub fn next_stage(&mut self, tcx: TyCtxt) {
        let max_ptr_depth = self.offset_of.len() as u32 + 1;

        let data_capacity = self
            .offset_of
            .last()
            .map(|offset_of| offset_of.everything().len())
            .unwrap_or_default();
        let mut offset_of = VecArray::with_capacity(self.post_order.len(), data_capacity);
        for &did in &self.post_order {
            let Adt(adt_def, subst_ref) = tcx.type_of(did).kind() else { unreachable!("impossible") };
            assert!(adt_def.is_struct());

            let mut offset = 0;
            offset_of.add_item_to_array(offset);

            for field_def in adt_def.all_fields() {
                let field_ty = field_def.ty(tcx, subst_ref);
                let (ptr_depth, maybe_adt) = abstract_ty(field_ty);
                if ptr_depth >= max_ptr_depth {
                    offset += max_ptr_depth;
                } else if let Some(&idx) = maybe_adt.and_then(|adt| self.did_idx.get(&adt.did())) {
                    if ptr_depth == 0 {
                        offset += offset_of.get_constructed(idx).last().unwrap();
                    } else {
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
        }

        let offset_of = offset_of.done();

        self.offset_of.push(offset_of);
    }
}

#[cfg(test)]
mod tests {
    use common::CrateData;

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
        common::test::run_compiler_with(TEXT.into(), |tcx, functions, structs| {
            let crate_data = CrateData::new(tcx, functions, structs);
            let program = CrateCtxt::from(crate_data);
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
            )
        })
    }
}
