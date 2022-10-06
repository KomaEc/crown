use rustc_data_structures::sso::SsoHashSet;
use rustc_middle::mir::{
    visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor},
    Body, Local,
};

use crate::{CrateCtxt, ptr::Measurable};

pub type OutputParams = SsoHashSet<Local>;

pub fn least_output_params(body: &Body, crate_ctxt: &CrateCtxt) -> SsoHashSet<Local> {
    let return_ty = body.return_ty();
    if crate_ctxt.measure(return_ty, 0) > 0 {
        return SsoHashSet::default()
    }

    let mut output_params = body
        .args_iter()
        .filter(|&local| {
            let ty = body.local_decls[local].ty;
            ty.is_unsafe_ptr() || ty.is_region_ptr()
        })
        .collect();

    struct Prune<'me>(&'me mut SsoHashSet<Local>);
    impl<'me, 'tcx> Visitor<'tcx> for Prune<'me> {
        fn visit_local(
            &mut self,
            local: Local,
            context: PlaceContext,
            _location: rustc_middle::mir::Location,
        ) {
            if !self.0.contains(&local) {
                return;
            }

            // This does not support transitive output params.
            if matches!(
                context,
                PlaceContext::MutatingUse(
                    MutatingUseContext::AddressOf
                        | MutatingUseContext::Borrow
                        | MutatingUseContext::Store
                ) | PlaceContext::NonMutatingUse(
                    NonMutatingUseContext::Copy | NonMutatingUseContext::Move
                )
            ) {
                self.0.remove(&local);
            }
        }
    }

    Prune(&mut output_params).visit_body(body);

    output_params
}
