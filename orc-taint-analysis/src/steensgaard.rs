pub mod constraint;

use orc_common::{whole_crate_discretization::WholeCrateDiscretization, OrcInput};
use petgraph::{
    graph::node_index, prelude::DiGraph, unionfind::UnionFind,
};
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{visit::Visitor, Place, Rvalue, Body},
    ty::TyCtxt,
};

orc_common::macros::orc_index!(AbstractLocation);

impl AbstractLocation {
    pub const NULL: Self = AbstractLocation::from_u32(0);
}

impl std::ops::Add<u32> for AbstractLocation {
    type Output = Self;

    #[inline]
    fn add(self, rhs: u32) -> Self::Output {
        self + (rhs as usize)
    }
}

impl std::ops::AddAssign<u32> for AbstractLocation {
    #[inline]
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + rhs
    }
}

impl Default for AbstractLocation {
    fn default() -> Self {
        Self::NULL
    }
}

#[derive(Debug)]
pub struct Steensgaard {
    structs: WholeCrateDiscretization<AbstractLocation>,
    functions: WholeCrateDiscretization<AbstractLocation>,
    pts_targets: UnionFind<AbstractLocation>,
    pts_graph: IndexVec<AbstractLocation, AbstractLocation>,
    // pts_graph: DiGraph<(), (), AbstractLocation>,
    // /// `DefId` of structs -> indices of `field_targets`
    // structs: FxHashMap<DefId, usize>,
    // /// Structs -> start `AbstractLocation`, the set of field targets
    // /// for struct `idx` is `field_targets[idx] ~ field_targets[idx+1]`.
    // /// There is an additional trailing entry that indicates the end of
    // /// the last struct
    // fields: Vec<AbstractLocation>,
    // field_indices_start: Vec<usize>,
    // field_indices: Vec<usize>,
    // /// `DefId` of functions -> indices of `formals`
    // functions: FxHashMap<DefId, usize>,
    // /// Similar as `field_targets`, but stores function locals.
    // /// Function formals are the starting member of `locals`
    // locals: Vec<AbstractLocation>,
    // /// Functionally similar as `locals`, but points into `local_indices`
    // local_indices_start: Vec<usize>,
    // /// Store actual indices for each locals. For example, if a function
    // /// containts three locals _0, _1, _2, if only _1 is of pointer type,
    // /// then the indices should all be 0, and only that of _1 is meaningful
    // local_indices: Vec<usize>,
}

impl Steensgaard {
    pub fn initiate<'tcx, Input: OrcInput<'tcx>>(input: Input) -> Self {
        let n_struct_fields_of_ptr_type = input.structs().iter().fold(0usize, |acc, did| {
            let adt_def = input.tcx().adt_def(*did);
            assert!(adt_def.is_struct());
            let n_fields = adt_def
                .all_fields()
                .filter(|field_def| {
                    let ty = input.tcx().type_of(field_def.did);
                    ty.is_unsafe_ptr()
                })
                .count();
            acc + n_fields
        });

        let mut pts_graph = DiGraph::with_capacity(
            2 * n_struct_fields_of_ptr_type + 1,
            n_struct_fields_of_ptr_type,
        );

        // Add the meaningless NULL node
        assert_eq!(pts_graph.add_node(()), AbstractLocation::NULL.into());

        // Adding the pts target of each struct field
        for _ in 0..n_struct_fields_of_ptr_type {
            pts_graph.add_node(());
        }

        let structs = WholeCrateDiscretization::new(
            input.tcx(),
            input.structs(),
            AbstractLocation::from_usize(pts_graph.node_count()), //AbstractLocation::NULL + 1 + n_struct_fields_of_ptr_type as u32,
            |tcx, did| {
                let adt_def = tcx.adt_def(did);
                assert!(adt_def.is_struct());
                adt_def.all_fields()
            },
            |field| {
                let field_node = pts_graph.add_node(());
                assert_eq!(field_node, field.into());
                pts_graph.add_edge(
                    field_node,
                    node_index(field_node.index() - n_struct_fields_of_ptr_type),
                    (),
                );
            },
            |tcx, field_def| tcx.type_of(field_def.did).is_unsafe_ptr(),
        );

        assert_eq!(pts_graph.node_count(), 1 + 2 * n_struct_fields_of_ptr_type);

        let functions = WholeCrateDiscretization::new(
            input.tcx(),
            input.functions(),
            AbstractLocation::from_usize(pts_graph.node_count()),
            |tcx, did| {
                let body = tcx.optimized_mir(did);
                body.local_decls.iter()
            },
            |_| {},
            |_, local_decl| local_decl.ty.is_unsafe_ptr(),
        );

        // let structs: FxHashMap<DefId, usize> = input
        //     .structs()
        //     .iter()
        //     .enumerate()
        //     .map(|(idx, did)| (*did, idx))
        //     .collect();
        // let mut fields: Vec<AbstractLocation> = Vec::with_capacity(structs.len() + 1); // vec![AbstractLocation::NULL; structs.len() + 1];
        // fields.push(AbstractLocation::NULL + 1 + n_struct_fields as u32);
        // let mut field_indices_start: Vec<usize> = Vec::with_capacity(structs.len() + 1); // vec![0; structs.len() + 1];
        // field_indices_start.push(0);
        // let mut field_indices: Vec<usize> = Vec::new();

        // for &did in input.structs() {
        //     let adt_def = input.tcx().adt_def(did);
        //     assert!(adt_def.is_struct());
        //     let mut field = unsafe { *fields.last().unwrap_unchecked() };
        //     let mut field_index = unsafe { *field_indices_start.last().unwrap_unchecked() };
        //     for field_def in adt_def.all_fields() {
        //         let new_node_index = pts_graph.add_node(());
        //         assert_eq!(new_node_index, field.into());
        //         pts_graph.add_edge(
        //             new_node_index,
        //             node_index(new_node_index.index() - n_struct_fields),
        //             (),
        //         );
        //         field_indices.push(field_index);
        //         if input.tcx().type_of(field_def.did).is_unsafe_ptr() {
        //             field += 1;
        //             field_index += 1;
        //         }
        //     }
        //     fields.push(field);
        //     field_indices_start.push(field_index);
        // }

        // let functions: FxHashMap<DefId, usize> = input
        //     .functions()
        //     .iter()
        //     .enumerate()
        //     .map(|(idx, did)| (*did, idx))
        //     .collect();
        // let mut locals: Vec<AbstractLocation> = Vec::with_capacity(functions.len() + 1); // vec![AbstractLocation::NULL; structs.len() + 1];
        // locals.push(AbstractLocation::NULL + 1 + 2 * n_struct_fields as u32);
        // let mut local_indices_start: Vec<usize> = Vec::with_capacity(functions.len() + 1); // vec![0; structs.len() + 1];
        // local_indices_start.push(0);
        // let mut local_indices: Vec<usize> = Vec::new();

        // for &did in input.functions() {
        //     let body = input.tcx().optimized_mir(did);
        //     let mut local = unsafe { *locals.last().unwrap_unchecked() };
        //     let mut local_index = unsafe { *local_indices_start.last().unwrap_unchecked() };
        //     for decl in &body.local_decls {
        //         // Do nothing with local
        //         local_indices.push(local_index);
        //         if decl.ty.is_unsafe_ptr() {
        //             local += 1;
        //             local_index += 1;
        //         }
        //     }
        //     locals.push(local);
        //     local_indices_start.push(local_index);
        // }

        let pts_targets = UnionFind::new(pts_graph.node_count());

        Steensgaard {
            structs,
            functions,
            pts_targets,
            pts_graph,
        }
    }
}

struct SteensgaardSolver<'me, 'tcx> {
    steensgaard: &'me mut Steensgaard,
    body: &'me Body<'tcx>,
    tcx: TyCtxt<'tcx>,
}

impl<'me, 'tcx> Visitor<'tcx> for SteensgaardSolver<'me, 'tcx> {
    fn visit_assign(
        &mut self,
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        _: rustc_middle::mir::Location,
    ) {
        if !place.ty(self.body, self.tcx).ty.is_unsafe_ptr() { return }

        match rvalue {
            Rvalue::Use(operand) | Rvalue::Cast(_, operand, _) => {
                assert!(
                    place.as_local().is_some()
                        || operand.place().and_then(|place| place.as_local()).is_some()
                        || operand.constant().is_some()
                );
            }
            Rvalue::CopyForDeref(rplace) => {
                assert!(place.as_local().is_some() || rplace.as_local().is_some());
            }
            Rvalue::Ref(_, _, _) | Rvalue::AddressOf(_, _) => {
                assert!(place.as_local().is_some())
            }
            _ => {}
        }
    }
}
