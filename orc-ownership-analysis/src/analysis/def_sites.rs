use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::mir::{
    visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor},
    BasicBlock, Body, Local, LocalInfo, Location, Place,
};

use super::OwnershipAnalysisCtxt;

pub type DefSites = IndexVec<Local, BitSet<BasicBlock>>;

impl<'octxt, 'tcx> OwnershipAnalysisCtxt<'octxt, 'tcx> {
    pub fn initial_def_sites(&self, body: &Body<'tcx>) -> DefSites {
        let mut def_sites = IndexVec::from_elem(
            BitSet::new_empty(body.basic_blocks.len()),
            &body.local_decls,
        );

        let mut def_counts = IndexVec::from_elem(0, &body.local_decls);
        DefCounts(&mut def_counts).visit_body(body);

        struct Vis<'me, 'octxt, 'tcx>(
            &'me mut DefSites,
            &'me Body<'tcx>,
            &'me OwnershipAnalysisCtxt<'octxt, 'tcx>,
        );
        impl<'me, 'octxt, 'tcx> Visitor<'tcx> for Vis<'me, 'octxt, 'tcx> {
            fn visit_place(
                &mut self,
                place: &Place<'tcx>,
                context: PlaceContext,
                location: Location,
            ) {
                if !matches!(
                    context,
                    PlaceContext::NonMutatingUse(
                        NonMutatingUseContext::Copy | NonMutatingUseContext::Move
                    ) | PlaceContext::MutatingUse(
                        MutatingUseContext::Call | MutatingUseContext::Store
                    )
                ) {
                    return;
                }

                if matches!(
                    self.1.local_decls[place.local].local_info,
                    Some(box LocalInfo::DerefTemp)
                ) && matches!(
                    context,
                    PlaceContext::MutatingUse(MutatingUseContext::Store)
                ) {
                    // let Some(_) = place.as_local() else { unreachable!("deref temp is mutated through projections") };
                    assert!(
                        place.as_local().is_some(),
                        "deref temp is mutated through projections"
                    );
                    return;
                }
            }
        }

        Vis(&mut def_sites, body, self).visit_body(body);

        def_sites
    }
}

struct DefCounts<'me>(&'me mut IndexVec<Local, usize>);

impl<'me, 'tcx> Visitor<'tcx> for DefCounts<'me> {
    fn visit_local(
        &mut self,
        local: Local,
        context: PlaceContext,
        _location: Location,
    ) {
        if matches!(
            context,
            PlaceContext::MutatingUse(
                MutatingUseContext::Call | MutatingUseContext::Store
            )
        ) {
            self.0[local] += 1;
        }
    }
}
