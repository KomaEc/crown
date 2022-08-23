use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::mir::{
    visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor},
    BasicBlock, Body, Local, LocalKind, Location,
};

use crate::analysis::ty_ext::TyExt;

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
            &'me IndexVec<Local, usize>,
            &'me Body<'tcx>,
            &'me OwnershipAnalysisCtxt<'octxt, 'tcx>,
            BasicBlock,
        );
        impl<'me, 'octxt, 'tcx> Visitor<'tcx> for Vis<'me, 'octxt, 'tcx> {
            fn visit_basic_block_data(&mut self, block: BasicBlock, data: &rustc_middle::mir::BasicBlockData<'tcx>) {
                self.4 = block;
                self.super_basic_block_data(block, data)
            }

            fn visit_local(&mut self, local: Local, context: PlaceContext, _: Location) {
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

                // for local variables (user defined or compiler generated temporaries),
                // we track only for those that are non-local, which means they are defined
                // at least twice
                if self.2.local_decls[local].ty.contains_ptr(self.3)
                    && (matches!(
                        self.2.local_kind(local),
                        LocalKind::Arg | LocalKind::ReturnPointer
                    ) || self.1[local] > 1)
                {
                    self.0[local].insert(self.4);
                }
            }
        }

        Vis(&mut def_sites, &def_counts, body, self, BasicBlock::from_u32(0)).visit_body(body);

        def_sites
    }
}

struct DefCounts<'me>(&'me mut IndexVec<Local, usize>);

impl<'me, 'tcx> Visitor<'tcx> for DefCounts<'me> {
    fn visit_local(&mut self, local: Local, context: PlaceContext, _location: Location) {
        if matches!(
            context,
            PlaceContext::MutatingUse(MutatingUseContext::Call | MutatingUseContext::Store)
        ) {
            self.0[local] += 1;
        }
    }
}
