use crate::{analysis::ty_ext::TyExt, struct_topology::StructTopology};
use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor},
        BasicBlock, Body, Local, LocalInfo, Location, Place,
    },
    ty::TyCtxt,
};

pub(crate) type DefSites = IndexVec<Local, BitSet<BasicBlock>>;

pub(crate) fn initial_def_sites<'tcx>(
    body: &Body<'tcx>,
    tcx: TyCtxt<'tcx>,
    struct_topology: &StructTopology,
) -> DefSites {
    let mut def_sites = IndexVec::from_elem(
        BitSet::new_empty(body.basic_blocks.len()),
        &body.local_decls,
    );

    struct Vis<'me, 'tcx> {
        def_sites: &'me mut DefSites,
        body: &'me Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        struct_topology: &'me StructTopology,
        basic_block: BasicBlock,
    }
    impl<'me, 'tcx> Visitor<'tcx> for Vis<'me, 'tcx> {
        fn visit_basic_block_data(
            &mut self,
            block: BasicBlock,
            data: &rustc_middle::mir::BasicBlockData<'tcx>,
        ) {
            self.basic_block = block;
            self.super_basic_block_data(block, data)
        }

        fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, _: Location) {
            if !matches!(
                context,
                PlaceContext::NonMutatingUse(
                    NonMutatingUseContext::Copy | NonMutatingUseContext::Move
                ) | PlaceContext::MutatingUse(MutatingUseContext::Call | MutatingUseContext::Store)
            ) {
                return;
            }

            let ty = place.ty(self.body, self.tcx).ty;
            let local_info = self.body.local_decls[place.local].local_info.as_deref();

            if ty.contains_ptr(self.struct_topology)
                && !place.is_indirect()
                && !matches!(local_info, Some(LocalInfo::DerefTemp))
            {
                self.def_sites[place.local].insert(self.basic_block);
            }
        }
    }

    Vis {
        def_sites: &mut def_sites,
        tcx,
        body,
        struct_topology,
        basic_block: BasicBlock::from_u32(0),
    }
    .visit_body(body);

    def_sites
}

// impl<'octxt, 'tcx> OwnershipAnalysisCtxt<'octxt, 'tcx> {
//     pub(crate) fn initial_def_sites(&self, body: &Body<'tcx>) -> DefSites {
//         let mut def_sites = IndexVec::from_elem(
//             BitSet::new_empty(body.basic_blocks.len()),
//             &body.local_decls,
//         );

//         struct Vis<'me, 'octxt, 'tcx>(
//             &'me mut DefSites,
//             &'me Body<'tcx>,
//             &'me OwnershipAnalysisCtxt<'octxt, 'tcx>,
//             BasicBlock,
//         );
//         impl<'me, 'octxt, 'tcx> Visitor<'tcx> for Vis<'me, 'octxt, 'tcx> {
//             fn visit_basic_block_data(
//                 &mut self,
//                 block: BasicBlock,
//                 data: &rustc_middle::mir::BasicBlockData<'tcx>,
//             ) {
//                 self.3 = block;
//                 self.super_basic_block_data(block, data)
//             }

//             fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, _: Location) {
//                 if !matches!(
//                     context,
//                     PlaceContext::NonMutatingUse(
//                         NonMutatingUseContext::Copy | NonMutatingUseContext::Move
//                     ) | PlaceContext::MutatingUse(
//                         MutatingUseContext::Call | MutatingUseContext::Store
//                     )
//                 ) {
//                     return;
//                 }

//                 let ty = place.ty(self.1, self.2.tcx()).ty;
//                 let local_info = self.1.local_decls[place.local].local_info.as_deref();

//                 if ty.contains_ptr(self.2.program.struct_topology())
//                     && !place.is_indirect()
//                     && !matches!(local_info, Some(LocalInfo::DerefTemp))
//                 {
//                     self.0[place.local].insert(self.3);
//                 }
//             }
//         }

//         Vis(&mut def_sites, body, self, BasicBlock::from_u32(0)).visit_body(body);

//         def_sites
//     }
// }
