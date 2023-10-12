use rustc_index::bit_set::BitSet;
use rustc_middle::mir::{
    visit::{MutatingUseContext, NonMutatingUseContext, NonUseContext, PlaceContext, Visitor},
    Body, Local, Location, Operand, Place, Rvalue, Terminator, TerminatorKind, RETURN_PLACE,
};
use smallvec::SmallVec;

use self::{access_path::AccessPaths, constraint::StorageMode};
use super::{
    def_use::{Def, DefUseChain, Inspect, LocationBuilder, Update, UseKind},
    SSAIdx,
};
use crate::call_graph::CallGraph;

pub mod access_path;
pub mod constraint;
mod inference;
// TODO re-export
#[cfg(test)]
mod tests;

/// Ownership inference context
pub struct Ctxt<const K_LIMIT: usize, Mode: StorageMode, DB> {
    pub database: DB,
    pub access_paths: AccessPaths<K_LIMIT>,
    pub call_graph: CallGraph,
    pub storage: Mode::Storage,
}

pub fn flow_chain<'tcx, const K_LIMIT: usize>(
    body: &Body<'tcx>,
    access_paths: &AccessPaths<K_LIMIT>,
) -> DefUseChain {
    let flow_builder = OwnershipFlowBuilder {
        body,
        access_paths,
        location_data: Default::default(),
        deref_copies: DerefCopiesCollector::collect(body),
    };
    DefUseChain::new(body, flow_builder)
}

struct DerefCopiesCollector(BitSet<Local>);

impl DerefCopiesCollector {
    fn collect(body: &Body) -> BitSet<Local> {
        let mut vis = Self(BitSet::new_empty(body.local_decls.len()));
        vis.visit_body(body);
        vis.0
    }
}

impl<'tcx> Visitor<'tcx> for DerefCopiesCollector {
    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, _: Location) {
        if let Rvalue::CopyForDeref(..) = rvalue {
            assert!(place.as_local().is_some());
            let local = place.local;
            self.0.insert(local);
        }
    }
}

pub struct OwnershipFlowBuilder<'build, 'tcx, const K_LIMIT: usize> {
    body: &'build Body<'tcx>,
    access_paths: &'build AccessPaths<K_LIMIT>,
    location_data: SmallVec<[(Local, UseKind<SSAIdx>); 2]>,
    deref_copies: BitSet<Local>,
}

impl<'build, 'tcx, const K_LIMIT: usize> OwnershipFlowBuilder<'build, 'tcx, K_LIMIT> {
    fn place_flow(&self, place: &Place<'tcx>, context: PlaceContext) -> Option<OwnershipFlow> {
        if self
            .access_paths
            .path(place, self.body)
            .num_pointers_reachable()
            == 0
        {
            return None;
        }
        OwnershipFlow::for_place(context).map(|flow| {
            if self.deref_copies.contains(place.local) {
                if place.as_local().is_some()
                    && matches!(
                        context,
                        PlaceContext::MutatingUse(MutatingUseContext::Store)
                    )
                {
                    OwnershipFlow::Flow
                } else {
                    OwnershipFlow::Inspect
                }
            } else {
                flow
            }
        })
    }
}

impl<'build, 'tcx, const K_LIMIT: usize> LocationBuilder<'tcx>
    for OwnershipFlowBuilder<'build, 'tcx, K_LIMIT>
{
    fn retrieve(&mut self) -> SmallVec<[(Local, UseKind<SSAIdx>); 2]> {
        std::mem::take(&mut self.location_data)
    }
}

impl<'build, 'tcx, const K_LIMIT: usize> Visitor<'tcx>
    for OwnershipFlowBuilder<'build, 'tcx, K_LIMIT>
{
    // for return terminator and indices
    fn visit_local(&mut self, local: Local, context: PlaceContext, location: Location) {
        self.visit_place(&Place::from(local), context, location)
    }

    fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, location: Location) {
        if let Some(flow) = self.place_flow(place, context) {
            if matches!(flow, OwnershipFlow::Flow) {
                self.location_data.push((place.local, Def(Update::new())));
            } else {
                self.location_data.push((place.local, Inspect(SSAIdx::MAX)));
            }
        }
        // call super_projection so that index operators are visited
        self.super_projection(place.as_ref(), context, location);
    }

    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        if let Rvalue::BinaryOp(_, box (operand1, operand2)) = rvalue {
            // special casing statements like _1 = BitAnd(_1, _3)
            // in this case, we do not generate usage for the right _1
            if let Some((lhs, operand1)) = place
                .as_local()
                .zip(operand1.place().and_then(|place| place.as_local()))
            {
                if lhs == operand1 {
                    self.visit_place(
                        place,
                        PlaceContext::MutatingUse(MutatingUseContext::Store),
                        location,
                    );
                    self.visit_operand(operand2, location);
                    return;
                }
            }
        } else if let Rvalue::CopyForDeref(rhs) = rvalue {
            // special casing deref copies
            assert!(place.as_local().is_some());
            self.visit_place(
                place,
                PlaceContext::MutatingUse(MutatingUseContext::Store),
                location,
            );
            self.visit_place(
                rhs,
                PlaceContext::NonMutatingUse(NonMutatingUseContext::Copy),
                location,
            );
            return;
        }
        self.super_assign(place, rvalue, location);
    }

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        if let TerminatorKind::Call {
            func,
            args,
            destination,
            ..
        } = &terminator.kind
        {
            self.visit_operand(func, location);
            for arg in args {
                match arg {
                    Operand::Move(place) => {
                        assert!(place.as_local().is_some());
                        self.visit_place(
                            place,
                            PlaceContext::NonMutatingUse(NonMutatingUseContext::Inspect),
                            location,
                        )
                    }
                    _ => self.visit_operand(arg, location),
                }
            }
            self.visit_place(
                destination,
                PlaceContext::MutatingUse(MutatingUseContext::Call),
                location,
            );
            return;
        } else if let TerminatorKind::Return = &terminator.kind {
            self.visit_local(
                RETURN_PLACE,
                PlaceContext::NonMutatingUse(NonMutatingUseContext::Move),
                location,
            );
            for local in self.body.local_decls.indices().skip(1) {
                self.visit_local(
                    local,
                    PlaceContext::NonMutatingUse(NonMutatingUseContext::Inspect),
                    location,
                )
            }
            return;
        }
        self.super_terminator(terminator, location);
    }
}

enum OwnershipFlow {
    Flow,
    Inspect,
}

impl OwnershipFlow {
    fn for_place(context: PlaceContext) -> Option<OwnershipFlow> {
        match context {
            PlaceContext::NonUse(NonUseContext::StorageDead)
            | PlaceContext::NonUse(NonUseContext::StorageLive) => {
                tracing::error!("StorageLive, StorageDead");
                None
            }

            PlaceContext::NonUse(_) => None,

            // Ownership flows for all mutating uses
            PlaceContext::MutatingUse(
                MutatingUseContext::Call
                | MutatingUseContext::Yield
                | MutatingUseContext::AsmOutput
                | MutatingUseContext::Store
                | MutatingUseContext::Deinit,
            ) => Some(OwnershipFlow::Flow),

            PlaceContext::MutatingUse(MutatingUseContext::SetDiscriminant) => {
                Some(OwnershipFlow::Inspect)
            }

            // Ownership flows for all kinds of borrows/address ofs
            //
            // Note that ownership flows for shared borrow as well, as it may be leaked to
            // a const address, which is not guaranteed read-only
            PlaceContext::MutatingUse(
                MutatingUseContext::AddressOf | MutatingUseContext::Borrow,
            )
            | PlaceContext::NonMutatingUse(
                NonMutatingUseContext::AddressOf
                | NonMutatingUseContext::ShallowBorrow
                | NonMutatingUseContext::SharedBorrow,
            ) => Some(OwnershipFlow::Flow),

            // Ownership flows for copy and move
            PlaceContext::NonMutatingUse(
                NonMutatingUseContext::Copy | NonMutatingUseContext::Move,
            ) => Some(OwnershipFlow::Flow),

            // deref copy, len, discriminant, etc.
            PlaceContext::NonMutatingUse(NonMutatingUseContext::Inspect) => {
                Some(OwnershipFlow::Inspect)
            }

            // TODO place mention?
            PlaceContext::NonMutatingUse(NonMutatingUseContext::PlaceMention) => todo!(),

            // All other contexts are uses...
            PlaceContext::MutatingUse(MutatingUseContext::Drop | MutatingUseContext::Retag) => {
                unreachable!()
            }

            PlaceContext::MutatingUse(MutatingUseContext::Projection)
            | PlaceContext::NonMutatingUse(NonMutatingUseContext::Projection) => {
                unreachable!("A projection could be a def or a use and must be handled separately")
            }
        }
    }
}
