use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, PlaceContext, Visitor},
        Body, Local, Location, Place, Rvalue,
    },
    ty::TyCtxt,
};
use smallvec::SmallVec;

use self::access_path::AccessPaths;
use super::{
    def_use::{Def, LocalPeek, LocationBuilder, Update, Use, UseKind},
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

pub struct OwnershipTransferBuilder<'build, 'tcx, const K_LIMIT: usize> {
    body: &'build Body<'tcx>,
    access_paths: &'build AccessPaths<K_LIMIT>,
    location_data: SmallVec<[(Local, UseKind<SSAIdx>); 2]>,
}

impl<'build, 'tcx, const K_LIMIT: usize> LocationBuilder<'tcx>
    for OwnershipTransferBuilder<'build, 'tcx, K_LIMIT>
{
    fn retrieve(&mut self) -> SmallVec<[(Local, UseKind<SSAIdx>); 2]> {
        std::mem::take(&mut self.location_data)
    }
}

impl<'build, 'tcx, const K_LIMIT: usize> Visitor<'tcx>
    for OwnershipTransferBuilder<'build, 'tcx, K_LIMIT>
{
    // for return terminator
    fn visit_local(&mut self, local: Local, context: PlaceContext, _: Location) {
        // if VanillaDefUse::for_place(Place::from(local), context).is_some() {
        //     let (_, num_pointers_reachable, _) =
        //         self.access_paths.path(&Place::from(local), self.body);
        //     if num_pointers_reachable > 0 {
        //         self.location_data.push((local, LocalPeek(SSAIdx::MAX)));
        //     } else {
        //         self.location_data.push((local, Use(SSAIdx::MAX)));
        //     }
        // }
    }

    fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, location: Location) {
        // TODO unions??
        // TODO peeks??
        // if VanillaDefUse::for_place(*place, context).is_some() {
        //     let (_, num_pointers_reachable, _) = self.access_paths.path(place, self.body);
        //     if num_pointers_reachable > 0 {
        //         self.location_data.push((place.local, Def(Update::new())));
        //     } else {
        //         self.location_data.push((place.local, Use(SSAIdx::MAX)));
        //     }
        // }
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
