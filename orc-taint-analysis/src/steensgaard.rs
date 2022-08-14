pub mod constraint;

use orc_common::{two_level_discretization::TwoLevelDiscretization, OrcInput};
use petgraph::unionfind::UnionFind;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{visit::Visitor, Body, Place, ProjectionElem, Rvalue},
    ty::{TyCtxt, TyKind},
};

use self::constraint::{Constraint, ConstraintKind};

orc_common::macros::orc_index!(AbstractLocation);

impl AbstractLocation {
    pub const NULL: Self = AbstractLocation::from_u32(0);

    pub fn is_null(&self) -> bool {
        *self == Self::NULL
    }
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
    structs: TwoLevelDiscretization<AbstractLocation>,
    functions: TwoLevelDiscretization<AbstractLocation>,
    pts_targets: UnionFind<AbstractLocation>,
    /// Steensgaard's analysis tracks for sinlge points-to relation for an
    /// abstract location, thus pts graph can be simplified as a vector.
    pts: IndexVec<AbstractLocation, AbstractLocation>,
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

        let mut pts = IndexVec::with_capacity(2 * n_struct_fields_of_ptr_type + 1);

        // null points to null
        assert_eq!(pts.push(AbstractLocation::NULL), AbstractLocation::NULL);

        // field pts targets should point to themselves
        for _ in 0..n_struct_fields_of_ptr_type {
            let this = pts.next_index();
            pts.push(this);
        }

        let structs = TwoLevelDiscretization::new(
            input.tcx(),
            input.structs(),
            pts.next_index(), //AbstractLocation::NULL + 1 + n_struct_fields_of_ptr_type as u32,
            |tcx, did| {
                let adt_def = tcx.adt_def(did);
                assert!(adt_def.is_struct());
                adt_def.all_fields()
            },
            |field: AbstractLocation| {
                let field_pts = AbstractLocation::from_u32(
                    field.as_u32() - (n_struct_fields_of_ptr_type as u32),
                );
                assert_eq!(pts.push(field_pts), field);
            },
            |tcx, field_def| tcx.type_of(field_def.did).is_unsafe_ptr(),
        );

        // assert_eq!(pts_graph.node_count(), 1 + 2 * n_struct_fields_of_ptr_type);

        let functions = TwoLevelDiscretization::new(
            input.tcx(),
            input.functions(),
            pts.next_index(),
            |tcx, did| {
                let body = tcx.optimized_mir(did);
                body.local_decls.iter()
            },
            |local| {
                assert_eq!(pts.push(AbstractLocation::NULL), local);
            },
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

        let pts_targets = UnionFind::new(pts.len());

        Steensgaard {
            structs,
            functions,
            pts_targets,
            pts,
        }
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
}

#[derive(Clone, Copy)]
struct Watch {
    next: u32,
    constraint: u32,
}

impl Watch {
    #[inline]
    fn new_first(constraint: usize) -> Self {
        Watch::new(0, constraint)
    }

    #[inline]
    fn new(next: usize, constraint: usize) -> Self {
        Watch {
            next: next as u32,
            constraint: constraint as u32,
        }
    }

    #[inline]
    fn next(&self) -> usize {
        self.next as usize
    }

    #[inline]
    fn constraint(&self) -> usize {
        self.constraint as usize
    }
}

struct WatchLists {
    nodes: Vec<Watch>,
    /// start index of an abstract location
    watches: IndexVec<AbstractLocation, usize>,
}

impl WatchLists {
    fn new(n_locs: usize) -> Self {
        WatchLists {
            nodes: vec![Watch::new_first(0)],
            watches: IndexVec::from_elem_n(0, n_locs),
        }
    }

    fn get_list(&self, loc: AbstractLocation) -> WatchList<'_> {
        WatchList {
            lists: self,
            this: loc,
        }
    }

    /// Add a new watch location for constraint
    #[inline]
    fn add_watch(&mut self, constraint_idx: usize, loc: AbstractLocation) {
        let next = std::mem::replace(&mut self.watches[loc], self.nodes.len());
        let watch = Watch::new(next, constraint_idx);
        self.nodes.push(watch);
    }
}

struct WatchList<'me> {
    lists: &'me WatchLists,
    this: AbstractLocation,
}

impl<'me> WatchList<'me> {
    fn iter(&self) -> WatchListIter<'_> {
        WatchListIter {
            watch_list: self.lists,
            node_idx: self.lists.watches[self.this],
        }
    }
}

struct WatchListIter<'me> {
    watch_list: &'me WatchLists,
    // loc: AbstractLocation,
    node_idx: usize,
}

impl<'me> Iterator for WatchListIter<'me> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.node_idx > 0 {
            let watch = self.watch_list.nodes[self.node_idx];
            self.node_idx = watch.next();
            Some(watch.constraint())
        } else {
            None
        }
    }
}

struct ConstraintGeneration<'me, 'tcx> {
    steensgaard: &'me mut Steensgaard,
    body: &'me Body<'tcx>,
    tcx: TyCtxt<'tcx>,
    constraints: &'me mut Vec<Constraint>,
    watches: &'me mut WatchLists,
}

enum PlaceLocation {
    Plain(AbstractLocation),
    Deref(AbstractLocation),
}

impl<'me, 'tcx> ConstraintGeneration<'me, 'tcx> {

    /// resolves the constraint (joins abstract locations), add constraint to
    /// constraint sets and watcher list if fails
    #[inline]
    pub(crate) fn resolve(&mut self, constraint@Constraint(kind, p, q): Constraint) {
        assert!(!p.is_null() && !q.is_null());

        let pts = &mut self.steensgaard.pts;
        match kind {
            ConstraintKind::Addr => {
                if pts[p].is_null() {
                    pts[p] = q;
                } else {
                    let pts_p = pts[p];
                    self.steensgaard.join(pts_p, q);
                }
            },
            ConstraintKind::Assign => todo!(),
            ConstraintKind::Store => todo!(),
            ConstraintKind::Load => todo!(),
        }

        // let (fst, snd) = match kind {
        //     ConstraintKind::Addr => (pts[p], q),
        //     ConstraintKind::Assign => (pts[p], pts[q]),
        //     ConstraintKind::Store => (pts[pts[p]], pts[q]),
        //     ConstraintKind::Load => (pts[p], pts[pts[q]]),
        // };
        // self.steensgaard.join(fst, snd);
    }

    fn place_location(&self, place: Place<'tcx>) -> PlaceLocation {
        let mut place = place.as_ref();

        // peel off all index projections
        for (place_base, proj_elem) in place.iter_projections().rev() {
            match proj_elem {
                ProjectionElem::Index(..)
                | ProjectionElem::ConstantIndex { .. }
                | ProjectionElem::Subslice { .. }
                | ProjectionElem::Downcast(..) => place = place_base,
                _ => break,
            }
        }

        if let Some((struct_place, ProjectionElem::Field(field, ty))) = place.last_projection() {
            let struct_ty = struct_place.ty(self.body, self.tcx).ty;
            let TyKind::Adt(adt_def, _) = struct_ty.kind() else { unreachable!() };
            let loc = self
                .steensgaard
                .structs
                .get_content(adt_def.did(), field.index());
            return PlaceLocation::Plain(loc);
        }

        assert!(place.local_or_deref_local().is_some());
        let loc = self
            .steensgaard
            .functions
            .get_content(self.body.source.def_id(), place.local.as_usize());
        if place.as_local().is_some() {
            return PlaceLocation::Plain(loc);
        } else {
            return PlaceLocation::Deref(loc);
        }
    }
}

impl<'me, 'tcx> Visitor<'tcx> for ConstraintGeneration<'me, 'tcx> {
    fn visit_assign(
        &mut self,
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        _: rustc_middle::mir::Location,
    ) {
        if !place.ty(self.body, self.tcx).ty.is_unsafe_ptr() {
            return;
        }

        let (is_addr_of, rplace) = match rvalue {
            Rvalue::Use(operand) | Rvalue::Cast(_, operand, _) => {
                assert!(
                    place.as_local().is_some()
                        || operand.place().and_then(|place| place.as_local()).is_some()
                        || operand.constant().is_some()
                );
                let Some(rplace) = operand.place() else { return };
                (false, rplace)
            }
            Rvalue::CopyForDeref(rplace) => {
                assert!(place.as_local().is_some() || rplace.as_local().is_some());
                (false, *rplace)
            }
            Rvalue::Ref(_, _, rplace) | Rvalue::AddressOf(_, rplace) => {
                assert!(place.as_local().is_some());
                (true, *rplace)
            }
            _ => {
                unreachable!("rvalue of pointer type can only be use/deref_copy/addr/ref")
            }
        };

        let l_loc = self.place_location(*place);
        let r_loc = self.place_location(rplace);

        let constraint = if is_addr_of {
            let PlaceLocation::Plain(p) = l_loc else { unreachable!() };
            match r_loc {
                PlaceLocation::Plain(q) => Constraint::new(ConstraintKind::Addr, p, q),
                PlaceLocation::Deref(q) => Constraint::new(ConstraintKind::Assign, p, q),
            }
        } else {
            match (l_loc, r_loc) {
                (PlaceLocation::Plain(p), PlaceLocation::Plain(q)) => {
                    Constraint::new(ConstraintKind::Assign, p, q)
                }
                (PlaceLocation::Plain(p), PlaceLocation::Deref(q)) => {
                    Constraint::new(ConstraintKind::Load, p, q)
                }
                (PlaceLocation::Deref(p), PlaceLocation::Plain(q)) => {
                    Constraint::new(ConstraintKind::Store, p, q)
                }
                _ => unreachable!(),
            }
        };

        self.resolve(constraint);
    }
}
