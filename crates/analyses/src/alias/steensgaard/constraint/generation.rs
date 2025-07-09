use rustc_middle::{
    mir::{Body, Place, Rvalue, Terminator, TerminatorKind, visit::Visitor},
    ty::TyCtxt,
};
use rustc_type_ir::TyKind::FnDef;

use crate::alias::steensgaard::{
    AbstractLocation, Steensgaard,
    constraint::{Constraint, ConstraintKind, watcher::WatcherLists},
    location::PlaceLocation,
    strategies::*,
};
use utils::tracing;

pub struct ConstraintGeneration<
    'me,
    'tcx,
    F: FieldStrategy,
    D: DeallocArgStrategy,
    I: InterProceduralStrategy,
> {
    pub steensgaard: &'me mut Steensgaard<F, D, I>,
    pub body: &'me Body<'tcx>,
    pub tcx: TyCtxt<'tcx>,
    pub constraints: &'me mut Vec<Constraint>,
    pub watchers: &'me mut WatcherLists,
    /// a buffer to hold a watcher list
    pub buffer: &'me mut Vec<usize>,
}

impl<'me, 'tcx, F: FieldStrategy, D: DeallocArgStrategy, I: InterProceduralStrategy>
    ConstraintGeneration<'me, 'tcx, F, D, I>
{
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
        let mut buffer = std::mem::take(self.buffer);
        self.notify(p, &mut buffer);
        let _ = std::mem::replace(self.buffer, buffer);
    }

    pub fn resolve_assign(
        &mut self,
        p: AbstractLocation,
        q: AbstractLocation,
        constraint_idx: usize,
    ) {
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
    pub fn resolve(&mut self, constraint @ Constraint(kind, mut p, mut q): Constraint) {
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
            .push(Constraint(ConstraintKind::Assign, p, q));
        // .push(Constraint::new(ConstraintKind::Assign, p, q));
        self.resolve_assign(p, q, constraint_idx)
    }

    #[inline]
    pub fn place_location(&self, place: Place<'tcx>) -> Option<PlaceLocation> {
        F::place_location(
            place,
            self.body,
            &self.steensgaard.struct_fields,
            &self.steensgaard.fn_locals,
            self.tcx,
        )
    }
}

impl<'me, 'tcx, F: FieldStrategy, D: DeallocArgStrategy, I: InterProceduralStrategy> Visitor<'tcx>
    for ConstraintGeneration<'me, 'tcx, F, D, I>
{
    fn visit_assign(
        &mut self,
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        _: rustc_middle::mir::Location,
    ) {
        let place_ty = place.ty(self.body, self.tcx).ty;
        // if !place_ty.is_unsafe_ptr() && !place_ty.is_ref() {
        //     return;
        // }
        if place_ty.is_primitive_ty() {
            return;
        }

        tracing::debug!("visiting assignment {:?}: {place_ty} = {:?}", place, rvalue);

        let (is_addr_of, rplace) = match rvalue {
            Rvalue::Use(operand) => {
                let Some(rplace) = operand.place() else {
                    return;
                };
                (false, rplace)
            }
            Rvalue::Cast(_, operand, _) => {
                let Some(rplace) = operand.place() else {
                    return;
                };
                (false, rplace)
            }
            Rvalue::CopyForDeref(rplace) => (false, *rplace),
            Rvalue::Ref(_, _, rplace) | Rvalue::RawPtr(_, rplace) => (true, *rplace),
            _ => return,
        };

        let Some(l_loc) = self.place_location(*place) else {
            return;
        };
        let Some(r_loc) = self.place_location(rplace) else {
            return;
        };

        let constraint = if is_addr_of {
            let PlaceLocation::Plain(p) = l_loc else {
                unreachable!()
            };
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
        } = &terminator.kind
        else {
            return;
        };

        let Some(func) = func.constant() else { return };
        let &FnDef(callee_did, _generic_args) = func.ty().kind() else {
            return;
        };

        if !self
            .steensgaard
            .fn_locals
            .0
            .did_idx
            .contains_key(&callee_did)
        {
            if let Some(local_did) = callee_did.as_local() {
                if let rustc_hir::Node::ForeignItem(foreign_item) =
                    self.tcx.hir_node_by_def_id(local_did)
                {
                    // special-casing free function
                    if foreign_item.ident.as_str() == "free" {
                        D::handle_dealloc_arg(self, &args.first().unwrap().node);
                        return;
                    }
                }
            }

            // handle unknown calls
            // self.handle_unknown_call(destination, args);
            I::handle_extern_call(self, destination, args);

            return;
        }

        I::handle_boundary(self, callee_did, destination, args);
    }
}
