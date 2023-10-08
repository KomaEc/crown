use rustc_middle::mir::{
    visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor},
    Body, Local, Location, Place, Rvalue,
};
use smallvec::SmallVec;

use self::access_path::AccessPaths;
use super::{
    def_use::{Def, Inspect, LocationBuilder, Update, UseKind},
    SSAIdx,
};
use crate::call_graph::CallGraph;

pub mod access_path;
pub mod constraint;

/// Ownership inference context
pub struct Ctxt<const K_LIMIT: usize, DB> {
    pub database: DB,
    pub access_path: AccessPaths<K_LIMIT>,
    pub call_graph: CallGraph,
}

pub struct OwnershipFlowBuilder<'build, 'tcx, const K_LIMIT: usize> {
    body: &'build Body<'tcx>,
    access_paths: &'build AccessPaths<K_LIMIT>,
    location_data: SmallVec<[(Local, UseKind<SSAIdx>); 2]>,
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
        if let Some(flow) = OwnershipFlow::for_place(context) {
            let (_, num_pointers_reachable, _) = self.access_paths.path(place, self.body);
            if num_pointers_reachable > 0 && matches!(flow, OwnershipFlow::Flow) {
                self.location_data.push((place.local, Def(Update::new())));
            } else {
                self.location_data.push((place.local, Inspect(SSAIdx::MAX)));
            }
        }
        // call super_projection so that index operators are visited
        self.super_projection(place.as_ref(), context, location);
    }

    // special casing statements like _1 = BitAnd(_1, _3)
    // in this case, we do not generate usage for the right _1
    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        if let Rvalue::BinaryOp(_, box (operand1, operand2)) = rvalue {
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
        }
        self.super_assign(place, rvalue, location);
    }
}

enum OwnershipFlow {
    Flow,
    Inspect,
}

impl OwnershipFlow {
    fn for_place(context: PlaceContext) -> Option<OwnershipFlow> {
        match context {
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

            // deref copy, len, etc.
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
