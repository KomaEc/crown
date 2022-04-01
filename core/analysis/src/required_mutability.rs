use rustc_hir::def_id::LocalDefId;
use rustc_index::bit_set::BitSet;
use rustc_middle::{
    mir::{
        visit::{PlaceContext, Visitor},
        Body, Local, Location,
    },
    ty::TyCtxt,
};

use crate::ty_ext::TyExt;

/// compute for a mir body the locals of raw pointer type
/// that is required to have mutability
pub fn required_mutability(tcx: TyCtxt<'_>, did: LocalDefId) -> BitSet<Local> {
    let body = tcx.optimized_mir(did);
    let required_mutability = BitSet::new_empty(body.local_decls.len());

    struct RequiredMutabilityVis<'me, 'tcx> {
        body: &'me Body<'tcx>,
        required_mutability: BitSet<Local>,
    }

    impl<'me, 'tcx> Visitor<'tcx> for RequiredMutabilityVis<'me, 'tcx> {
        fn visit_local(
            &mut self,
            &local: &Local,
            place_context: PlaceContext,
            _location: Location,
        ) {
            if self.body.local_decls[local].ty.is_ptr_but_not_fn_ptr() {
                if let PlaceContext::MutatingUse(_) = place_context {
                    self.required_mutability.insert(local);
                }
            }
        }
    }

    let mut vis = RequiredMutabilityVis {
        body,
        required_mutability,
    };

    vis.visit_body(vis.body);

    vis.required_mutability
}
