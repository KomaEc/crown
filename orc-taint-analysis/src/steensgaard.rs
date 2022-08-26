pub mod constraint;
#[cfg(test)]
mod test;

use orc_common::{item_set::ItemSet, OrcInput};
use petgraph::unionfind::UnionFind;
use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::{
    mir::{visit::Visitor, Body, Local, Place, ProjectionElem, Rvalue, Terminator, TerminatorKind},
    ty::{Ty, TyCtxt, TyKind},
};
use rustc_type_ir::TyKind::FnDef;

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

fn peel_off_array(mut ty: Ty) -> Ty {
    while let TyKind::Array(inner_ty, _) = ty.kind() {
        ty = *inner_ty
    }
    ty
}

#[derive(Debug)]
pub struct Steensgaard {
    pub(crate) struct_fields: ItemSet<AbstractLocation>,
    pub(crate) function_locals: ItemSet<AbstractLocation>,
    pub(crate) pts_targets: UnionFind<AbstractLocation>,
    /// Steensgaard's analysis tracks for sinlge points-to relation for an
    /// abstract location, thus pts graph can be simplified as a vector.
    pub(crate) pts: IndexVec<AbstractLocation, AbstractLocation>,
}

impl Steensgaard {
    pub fn new<'tcx, Input: OrcInput<'tcx>>(input: Input) -> Self {
        let n_struct_fields = input.structs().iter().fold(0usize, |acc, did| {
            acc + input.tcx().adt_def(*did).all_fields().count()
        });

        let mut pts = IndexVec::with_capacity(2 * n_struct_fields + 1);

        // null points to null
        assert_eq!(pts.push(AbstractLocation::NULL), AbstractLocation::NULL);

        // field pts targets should point to themselves
        for _ in 0..n_struct_fields {
            let this = pts.next_index();
            pts.push(this);
        }

        let struct_fields = ItemSet::new(
            input.tcx(),
            input.structs(),
            pts.next_index(), //AbstractLocation::NULL + 1 + n_struct_fields_of_ptr_type as u32,
            |tcx, did| {
                let adt_def = tcx.adt_def(did);
                // println!("discretising {:?}", adt_def);
                assert!(adt_def.is_struct());
                adt_def.all_fields() //.inspect(|field_def| println!("field {:?}", field_def))
            },
            |field: AbstractLocation| {
                let field_pts = AbstractLocation::from_u32(
                    field.as_u32() - (n_struct_fields as u32), // (n_struct_fields_of_ptr_type as u32),
                );
                assert_eq!(pts.push(field_pts), field);
            },
            |_| |_, _| true, // peel_off_array(input.tcx().type_of(field_def.did)).is_unsafe_ptr(),
        );

        struct AddrTaken<'me>(&'me mut BitSet<Local>);
        impl<'me, 'tcx> Visitor<'tcx> for AddrTaken<'me> {
            fn visit_rvalue(&mut self, rvalue: &Rvalue<'tcx>, _: rustc_middle::mir::Location) {
                match rvalue {
                    Rvalue::AddressOf(_, place) | Rvalue::Ref(_, _, place) => {
                        let Some(local) = place.as_local() else { return };
                        self.0.insert(local);
                    }
                    _ => return,
                }
            }
        }

        let function_locals = ItemSet::new(
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
            |did| {
                let body = input.tcx().optimized_mir(did);
                let mut addr_taken = BitSet::<Local>::new_empty(body.local_decls.len());
                let mut vis = AddrTaken(&mut addr_taken);
                vis.visit_body(body);
                move |local_idx, local_decl| {
                    let ty = peel_off_array(local_decl.ty);
                    ty.is_unsafe_ptr()
                        || ty.is_region_ptr()
                        || addr_taken.contains(local_idx.into())
                }
            },
        );

        let pts_targets = UnionFind::new(pts.len());

        let mut steensgaard = Steensgaard {
            struct_fields,
            function_locals,
            pts_targets,
            pts,
        };

        let mut constraints = Vec::new();
        let mut watchers = WatcherLists::new(steensgaard.node_count());
        let mut buffer = Vec::with_capacity(steensgaard.node_count());

        for did in input.functions() {
            let body = input.tcx().optimized_mir(did);
            let mut cg = ConstraintGeneration {
                steensgaard: &mut steensgaard,
                body,
                tcx: input.tcx(),
                constraints: &mut constraints,
                watchers: &mut watchers,
                buffer: &mut buffer,
            };
            cg.visit_body(body);
        }

        steensgaard
    }

    #[inline]
    pub fn node_count(&self) -> usize {
        self.pts.len()
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

    pub(crate) fn may_alias(&self, p: AbstractLocation, q: AbstractLocation) -> bool {
        if p.is_null() || q.is_null() {
            return false;
        }
        self.pts_targets.equiv(self.pts[p], self.pts[q])
    }

    // #[cfg(debug_assertions)]
    pub fn print_results(&self) {
        for &did in self.struct_fields.belongers() {
            println!("results for {:?}:", did);
            let fields_result = self
                .struct_fields
                .get_contents(did)
                .map(|loc| self.pts_targets.find(self.pts[loc]))
                .collect::<Vec<_>>();
            for (idx, tgt) in fields_result.into_iter().enumerate() {
                println!("{:?}.{idx} -> {:?}", did, tgt);
            }
        }

        for &did in self.function_locals.belongers() {
            println!("results for {:?}:", did);
            let locals_result = self
                .function_locals
                .get_contents(did)
                .map(|loc| self.pts_targets.find(self.pts[loc]))
                .collect::<Vec<_>>();
            for (idx, tgt) in locals_result.into_iter().enumerate() {
                println!("{:?}.{idx} -> {:?}", did, tgt);
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Watcher {
    next: u32,
    constraint: u32,
}

impl Watcher {
    #[inline]
    fn new_first(constraint: usize) -> Self {
        Watcher::new(0, constraint)
    }

    #[inline]
    fn new(next: usize, constraint: usize) -> Self {
        Watcher {
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

struct WatcherLists {
    nodes: Vec<Watcher>,
    /// start index of an abstract location
    head: IndexVec<AbstractLocation, usize>,
}

impl WatcherLists {
    fn new(n_locs: usize) -> Self {
        WatcherLists {
            nodes: vec![Watcher::new_first(0)],
            head: IndexVec::from_elem_n(0, n_locs),
        }
    }

    fn get_list(&self, loc: AbstractLocation) -> WatcherList<'_> {
        WatcherList {
            lists: self,
            this: loc,
        }
    }

    /// Add a new watch location for constraint
    #[inline]
    fn add_watch(&mut self, constraint_idx: usize, loc: AbstractLocation) {
        let next = std::mem::replace(&mut self.head[loc], self.nodes.len());
        let watch = Watcher::new(next, constraint_idx);
        self.nodes.push(watch);
    }
}

struct WatcherList<'me> {
    lists: &'me WatcherLists,
    this: AbstractLocation,
}

impl<'me> WatcherList<'me> {
    fn iter(&self) -> WatcherListIter<'_> {
        WatcherListIter {
            watcher_lists: self.lists,
            node_idx: self.lists.head[self.this],
        }
    }
}

struct WatcherListIter<'me> {
    watcher_lists: &'me WatcherLists,
    // loc: AbstractLocation,
    node_idx: usize,
}

impl<'me> Iterator for WatcherListIter<'me> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.node_idx > 0 {
            let cur_watcher = self.watcher_lists.nodes[self.node_idx];
            self.node_idx = cur_watcher.next();
            Some(cur_watcher.constraint())
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
    watchers: &'me mut WatcherLists,
    /// a buffer to hold a watcher list
    buffer: &'me mut Vec<usize>,
}

enum PlaceLocation {
    Plain(AbstractLocation),
    Deref(AbstractLocation),
}

impl<'me, 'tcx> ConstraintGeneration<'me, 'tcx> {
    pub(crate) fn notify(&mut self, p: AbstractLocation, buffer: &mut Vec<usize>) {
        buffer.clear();
        buffer.extend(self.watchers.get_list(p).iter());

        for &constraint_idx in &*buffer {
            match self.constraints[constraint_idx].0 {
                ConstraintKind::Assign => {
                    let q = AbstractLocation::from_u32(
                        self.constraints[constraint_idx].1.as_u32()
                            + self.constraints[constraint_idx].2.as_u32()
                            - p.as_u32(),
                    );
                    if self.steensgaard.pts[q].is_null() {
                        self.set_pts(q, self.steensgaard.pts[p]);
                    } else {
                        let pts_p = self.steensgaard.pts[p];
                        let pts_q = self.steensgaard.pts[q];
                        self.steensgaard.join(pts_p, pts_q)
                    }
                }
                ConstraintKind::Store => {
                    // *p = q
                    assert_eq!(self.constraints[constraint_idx].1, p);
                    let q = self.constraints[constraint_idx].2;
                    let pts_p = self.steensgaard.pts[p];
                    self.constraints[constraint_idx] =
                        Constraint::new(ConstraintKind::Assign, pts_p, q);
                    self.resolve_assign(pts_p, q, constraint_idx)
                }
                ConstraintKind::Load => {
                    // q = *p
                    assert_eq!(self.constraints[constraint_idx].2, p);
                    let q = self.constraints[constraint_idx].1;
                    let pts_p = self.steensgaard.pts[p];
                    self.constraints[constraint_idx] =
                        Constraint::new(ConstraintKind::Assign, q, pts_p);
                    self.resolve_assign(q, pts_p, constraint_idx)
                }
                _ => {
                    unreachable!("internal error: addr constraint is watching")
                }
            }
        }
    }

    /// update the pts graph such that `p` points to `q`
    pub(crate) fn set_pts(&mut self, p: AbstractLocation, q: AbstractLocation) {
        assert!(self.steensgaard.pts[p].is_null());
        assert!(!q.is_null());
        self.steensgaard.pts[p] = q;

        // notify all watchers of `p`
        let mut buffer = std::mem::replace(self.buffer, Vec::new());
        self.notify(p, &mut buffer);
        let _ = std::mem::replace(self.buffer, buffer);
    }

    fn resolve_assign(&mut self, p: AbstractLocation, q: AbstractLocation, constraint_idx: usize) {
        let pts = &self.steensgaard.pts;
        match (pts[p].is_null(), pts[q].is_null()) {
            (true, true) => {
                self.watchers.add_watch(constraint_idx, p);
                self.watchers.add_watch(constraint_idx, q);
            }
            (true, false) => {
                // pts[p] = pts[q];
                self.set_pts(p, pts[q]);
            }
            (false, true) => {
                // pts[q] = pts[p];
                self.set_pts(q, pts[p]);
            }
            (false, false) => {
                let pts_p = pts[p];
                let pts_q = pts[q];
                self.steensgaard.join(pts_p, pts_q);
            }
        };
    }

    /// resolves the constraint (joins abstract locations), add constraint to
    /// constraint sets and watcher list if fails
    pub(crate) fn resolve(&mut self, constraint @ Constraint(kind, mut p, mut q): Constraint) {
        assert!(!p.is_null() && !q.is_null());

        let pts = &mut self.steensgaard.pts;
        match kind {
            ConstraintKind::Addr => {
                if pts[p].is_null() {
                    // pts[p] = q;
                    self.set_pts(p, q)
                } else {
                    let pts_p = pts[p];
                    self.steensgaard.join(pts_p, q);
                }
                return;
            }
            ConstraintKind::Assign => {}
            ConstraintKind::Store => {
                if pts[p].is_null() {
                    let constraint_idx = self.constraints.len();
                    self.constraints.push(constraint);
                    self.watchers.add_watch(constraint_idx, p);
                    return;
                } else {
                    p = pts[p];
                }
            }
            ConstraintKind::Load => {
                if pts[q].is_null() {
                    let constraint_idx = self.constraints.len();
                    self.constraints.push(constraint);
                    self.watchers.add_watch(constraint_idx, q);
                    return;
                } else {
                    q = pts[q];
                }
            }
        }

        // process assign(p, q)
        let constraint_idx = self.constraints.len();
        self.constraints
            .push(Constraint::new(ConstraintKind::Assign, p, q));
        self.resolve_assign(p, q, constraint_idx)
    }

    #[inline]
    fn place_location(&self, place: Place<'tcx>) -> Option<PlaceLocation> {
        // println!("place: {:?}", place);
        // println!(
        //     "{:?}: {}",
        //     place.local, self.body.local_decls[place.local].ty
        // );

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

        if let Some((struct_place, ProjectionElem::Field(field, _))) = place.last_projection() {
            let struct_ty = struct_place.ty(self.body, self.tcx).ty;
            let TyKind::Adt(adt_def, _) = struct_ty.kind() else { unreachable!() };
            if !self.steensgaard.struct_fields.has_entry(adt_def.did()) {
                return None;
            }
            let loc = self
                .steensgaard
                .struct_fields
                .get_singleton_content(adt_def.did(), field.index());
            return Some(PlaceLocation::Plain(loc));
        }

        assert!(place.local_or_deref_local().is_some());
        let loc = self
            .steensgaard
            .function_locals
            .get_singleton_content(self.body.source.def_id(), place.local.as_usize());
        if place.as_local().is_some() {
            return Some(PlaceLocation::Plain(loc));
        } else {
            return Some(PlaceLocation::Deref(loc));
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
        let place_ty = place.ty(self.body, self.tcx).ty;
        if !place_ty.is_unsafe_ptr() && !place_ty.is_region_ptr() {
            return;
        }

        // println!("visiting assignment {:?}: {place_ty} = {:?}", place, rvalue);

        let (is_addr_of, rplace) = match rvalue {
            Rvalue::Use(operand) => {
                let Some(rplace) = operand.place() else { return };
                (false, rplace)
            }
            Rvalue::Cast(_, operand, _) => {
                let Some(rplace) = operand.place() else { return };
                // integer pointer cast is ignored
                let rplace_ty = rplace.ty(self.body, self.tcx).ty;
                if !rplace_ty.is_unsafe_ptr() && !rplace_ty.is_region_ptr() {
                    return;
                }
                (false, rplace)
            }
            Rvalue::CopyForDeref(rplace) => (false, *rplace),
            Rvalue::Ref(_, _, rplace) | Rvalue::AddressOf(_, rplace) => (true, *rplace),
            _ => {
                unreachable!("rvalue of pointer type can only be use/deref_copy/addr/ref")
            }
        };

        let Some(l_loc) = self.place_location(*place) else { return };
        let Some(r_loc) = self.place_location(rplace) else { return };

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

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, _: rustc_middle::mir::Location) {
        let TerminatorKind::Call {
            func,
            args,
            destination,
            ..
        } = &terminator.kind else { return };

        let Some(func) = func.constant() else { return };
        let &FnDef(callee_did, _generic_args) = func.ty().kind() else { return };

        if !self.steensgaard.function_locals.has_entry(callee_did) {
            return;
        }

        for (idx, arg) in args.iter().enumerate() {
            let Some(place) = arg.place() else { continue };
            let place_ty = place.ty(self.body, self.tcx).ty;
            if !place_ty.is_unsafe_ptr() && !place_ty.is_region_ptr() {
                continue;
            }
            let Some(arg_loc) = self.place_location(place) else { continue };
            let param_loc = self
                .steensgaard
                .function_locals
                .get_singleton_content(callee_did, idx + 1);

            let PlaceLocation::Plain(arg_loc) = arg_loc else { unreachable!("argument operand contains derefs") };
            let constraint_idx = self.constraints.len();
            self.constraints
                .push(Constraint::new(ConstraintKind::Assign, param_loc, arg_loc));
            self.resolve_assign(param_loc, arg_loc, constraint_idx)
        }

        let dest_ty = destination.ty(self.body, self.tcx).ty;
        if !dest_ty.is_unsafe_ptr() && !dest_ty.is_region_ptr() {
            return;
        }

        let Some(dest_loc) = self.place_location(*destination) else { return };
        let PlaceLocation::Plain(dest_loc) = dest_loc else { unreachable!("destination place contains derefs") };
        let ret_loc = self.steensgaard.function_locals.get_singleton_content(callee_did, 0);
        let constraint_idx = self.constraints.len();
        self.constraints
            .push(Constraint::new(ConstraintKind::Assign, dest_loc, ret_loc));
        self.resolve_assign(dest_loc, ret_loc, constraint_idx);
    }
}
