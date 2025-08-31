mod constraint;
pub mod location;
pub mod pp;
pub mod strategies;

pub use location::AbstractLocation;
use rustc_hir::def_id::DefId;

pub struct MemoryLocations(crate::encoding::Encoding<AbstractLocation>);

impl MemoryLocations {
    pub(crate) fn memory_location(&self, did: &DefId, index: usize) -> AbstractLocation {
        self.0.content(did, index).start
    }

    pub(crate) fn memory_locations(&self, did: &DefId) -> &[AbstractLocation] {
        let locations = &self.0.contents[self.0.did_idx[did]];
        &locations[..locations.len() - 1]
        // &self.0.contents[self.0.did_idx[did]]
    }
}

type FnLocals = MemoryLocations;
type StructFields = MemoryLocations;

pub use constraint::generation::*;
use rustc_index::IndexVec;
use rustc_middle::mir::visit::Visitor;
pub use strategies::*;
use utils::petgraph::unionfind::UnionFind;

use crate::{
    alias::steensgaard::constraint::watcher::WatcherLists,
    encoding::{encode_fns, encode_structs},
};

pub struct Steensgaard<F: FieldStrategy, D: DeallocArgStrategy, I: InterProceduralStrategy> {
    /// struct -> field -> [std::ops::Range<AbstractLocation>]
    pub(crate) struct_fields: F::StructFields,

    /// fn -> local -> [AbstractLocation]
    pub(crate) fn_locals: FnLocals,

    /// Argument of free
    pub(crate) dealloc_arg: D::Arg,
    pub(crate) pts_targets: UnionFind<AbstractLocation>,
    /// Steensgaard's analysis tracks for sinlge points-to relation for an
    /// abstract location, thus pts graph can be simplified as a vector.
    pub(crate) pts: IndexVec<AbstractLocation, AbstractLocation>,
    pub(crate) _interprocedural_strategy: std::marker::PhantomData<*const I>,
}

impl<I: InterProceduralStrategy> Steensgaard<FieldBased, MergeDeallocArg, I> {
    pub fn field_based(rust_program: &utils::rustc::RustProgram) -> Self {
        let n_struct_fields = rust_program.structs.iter().fold(0usize, |acc, did| {
            acc + rust_program.tcx.adt_def(*did).all_fields().count()
        });

        let mut pts = IndexVec::with_capacity(2 * n_struct_fields + 1);

        // null points to null
        assert_eq!(pts.push(AbstractLocation::NULL), AbstractLocation::NULL);

        // field pts targets should point to themselves
        for _ in 0..n_struct_fields {
            let this = pts.next_index();
            pts.push(this);
        }

        let next_location = pts.next_index();
        let (struct_fields, next_location) = encode_structs(
            next_location,
            &rust_program.structs,
            rust_program.tcx,
            |_| {
                let field = pts.next_index();
                let field_pt =
                    AbstractLocation::from_u32(field.as_u32() - (n_struct_fields as u32));
                pts.push(field_pt);
                1
            },
        );

        let (fn_locals, _) = encode_fns(
            next_location,
            &rust_program.functions,
            rust_program.tcx,
            |_| {
                let local: AbstractLocation = pts.next_index();
                assert_eq!(pts.push(AbstractLocation::NULL), local);
                1
            },
        );

        let arg_free = pts.push(AbstractLocation::NULL);

        let pts_targets = UnionFind::new(pts.len());

        let mut steensgaard = Steensgaard {
            struct_fields: MemoryLocations(struct_fields.0),
            fn_locals: MemoryLocations(fn_locals.0),
            dealloc_arg: arg_free,
            pts_targets,
            pts,
            _interprocedural_strategy: std::marker::PhantomData,
        };

        let mut constraints = Vec::new();
        let mut watchers = WatcherLists::new(steensgaard.node_count());
        let mut buffer = Vec::with_capacity(steensgaard.node_count());

        for &did in &rust_program.functions {
            let body = &*rust_program
                .tcx
                .mir_drops_elaborated_and_const_checked(did.expect_local())
                .borrow();
            let mut cg = ConstraintGeneration {
                steensgaard: &mut steensgaard,
                body,
                tcx: rust_program.tcx,
                constraints: &mut constraints,
                watchers: &mut watchers,
                buffer: &mut buffer,
            };
            cg.visit_body(body);
        }

        steensgaard
    }

    pub fn print_results(&self) {
        for (&did, _) in self.struct_fields.0.did_idx.iter() {
            println!("results for {:?}:", did);
            let fields_result = self
                .struct_fields
                .memory_locations(&did)
                .split_last()
                .unwrap()
                .1
                .iter()
                .copied()
                .map(|loc| self.pts_targets.find(self.pts[loc]));
            // .collect::<Vec<_>>();
            for (idx, tgt) in fields_result.enumerate() {
                println!("{:?}.{idx} -> {:?}", did, tgt);
            }
        }

        for (&did, _) in self.fn_locals.0.did_idx.iter() {
            println!("results for {:?}:", did);
            let locals_result = self
                .fn_locals
                .memory_locations(&did)
                .iter()
                .copied()
                .map(|loc| self.pts_targets.find(self.pts[loc]));
            for (idx, tgt) in locals_result.enumerate() {
                println!("{:?}.{idx} -> {:?}", did, tgt);
            }
        }
    }
}

impl<I: InterProceduralStrategy> Steensgaard<FieldInsensitive, NopDeallocArg, I> {
    pub fn field_insensitive(rust_program: &utils::rustc::RustProgram) -> Self {
        let n_fn_locals = rust_program.functions.iter().fold(0usize, |acc, did| {
            acc + rust_program
                .tcx
                .mir_drops_elaborated_and_const_checked(did.expect_local())
                .borrow()
                .local_decls
                .len()
        });

        let mut pts = IndexVec::with_capacity(2 * n_fn_locals + 1);

        // null points to null
        assert_eq!(pts.push(AbstractLocation::NULL), AbstractLocation::NULL);

        // initial points-to target for each function local
        for _ in 0..n_fn_locals {
            pts.push(AbstractLocation::NULL);
        }

        let next_location = pts.next_index();
        let (fn_locals, _) = encode_fns(
            next_location,
            &rust_program.functions,
            rust_program.tcx,
            |_| {
                let local: AbstractLocation = pts.next_index();
                let local_pt = AbstractLocation::from_u32(local.as_u32() - n_fn_locals as u32);
                assert_eq!(pts.push(local_pt), local);
                1
            },
        );

        let pts_targets = UnionFind::new(pts.len());

        let mut steensgaard = Steensgaard {
            struct_fields: (),
            fn_locals: MemoryLocations(fn_locals.0),
            dealloc_arg: (),
            pts_targets,
            pts,
            _interprocedural_strategy: std::marker::PhantomData,
        };

        let mut constraints = Vec::new();
        let mut watchers = WatcherLists::new(steensgaard.node_count());
        let mut buffer = Vec::with_capacity(steensgaard.node_count());

        for &did in &rust_program.functions {
            let body = &*rust_program
                .tcx
                .mir_drops_elaborated_and_const_checked(did.expect_local())
                .borrow();
            let mut cg = ConstraintGeneration {
                steensgaard: &mut steensgaard,
                body,
                tcx: rust_program.tcx,
                constraints: &mut constraints,
                watchers: &mut watchers,
                buffer: &mut buffer,
            };
            cg.visit_body(body);
        }

        steensgaard
    }

    pub fn print_results(&self) {
        for (&did, _) in self.fn_locals.0.did_idx.iter() {
            println!("results for {:?}:", did);
            let locals_result = self
                .fn_locals
                .memory_locations(&did)
                .iter()
                .copied()
                .map(|loc| self.pts_targets.find(self.pts[loc]));
            for (idx, tgt) in locals_result.enumerate() {
                println!("{:?}.{idx} -> {:?}", did, tgt);
            }
        }
    }
}

impl<F: FieldStrategy, D: DeallocArgStrategy, I: InterProceduralStrategy> Steensgaard<F, D, I> {
    #[inline]
    pub fn node_count(&self) -> usize {
        self.pts.len()
    }

    pub fn local_locations(&self, body_id: &DefId) -> &[AbstractLocation] {
        &self.fn_locals.memory_locations(&body_id)
    }

    pub(crate) fn join(&mut self, p: AbstractLocation, q: AbstractLocation) {
        if self.pts_targets.find_mut(p) == self.pts_targets.find_mut(q) {
            return;
        }
        let p_pts = self.pts[p];
        let q_pts = self.pts[q];
        self.pts_targets.union(p, q);
        self.join(p_pts, q_pts);
    }

    #[inline]
    pub fn may_alias(&self, p: AbstractLocation, q: AbstractLocation) -> bool {
        if p.is_null() || q.is_null() {
            return false;
        }
        self.pts_targets.equiv(self.pts[p], self.pts[q])
    }

    #[inline]
    pub fn pts_rep(&self, p: AbstractLocation) -> AbstractLocation {
        self.pts_targets.find(self.pts[p])
    }
}
