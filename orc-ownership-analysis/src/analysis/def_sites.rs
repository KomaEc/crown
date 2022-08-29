use crate::{analysis::ty_ext::TyExt, struct_topology::StructTopology};
use orc_common::data_structure::vec_array::{VecArray, VecArrayConstruction};
use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor},
        BasicBlock, BasicBlockData, Body, Local, LocalInfo, Location, Place,
    },
    ty::TyCtxt,
};
use smallvec::SmallVec;

pub(crate) struct Definitions {
    /// BasicBlock -> statement_index -> possible definitions
    /// 
    /// We've made an assumption that a local can only be used or defined
    /// once in a statement/terminator
    pub(crate) defs: VecArray<SmallVec<[Local; 2]>>,
    pub(crate) sites: IndexVec<Local, BitSet<BasicBlock>>,
}

impl Definitions {
    #[inline]
    pub(crate) fn of_block(&self, block: BasicBlock) -> &[SmallVec<[Local; 2]>] {
        &self.defs[block.index()]
    }

    #[inline]
    pub(crate) fn of_location(&self, location: Location) -> &SmallVec<[Local; 2]> {
        let Location {
            block,
            statement_index,
        } = location;
        &self.defs[block.index()][statement_index]
    }
}

pub(crate) fn initial_definitions<'tcx>(
    body: &Body<'tcx>,
    tcx: TyCtxt<'tcx>,
    struct_topology: &StructTopology,
) -> Definitions {
    let mut sites = IndexVec::from_elem(
        BitSet::new_empty(body.basic_blocks.len()),
        &body.local_decls,
    );

    let mut defs = VecArray::new(body.basic_blocks().len());

    struct Vis<'me, 'tcx> {
        sites: &'me mut IndexVec<Local, BitSet<BasicBlock>>,
        defs: &'me mut VecArrayConstruction<SmallVec<[Local; 2]>>,
        defs_in_cur_stmt: SmallVec<[Local; 2]>,
        body: &'me Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        struct_topology: &'me StructTopology,
        basic_block: Option<BasicBlock>,
    }
    impl<'me, 'tcx> Visitor<'tcx> for Vis<'me, 'tcx> {
        fn visit_basic_block_data(
            &mut self,
            block: BasicBlock,
            data: &rustc_middle::mir::BasicBlockData<'tcx>,
        ) {
            let old_block = std::mem::replace(&mut self.basic_block, Some(block));
            if old_block.is_some() {
                self.defs.done_with_array();
            }
            // // self.basic_block = Some(block);
            // self.super_basic_block_data(block, data)

            let BasicBlockData {
                statements,
                terminator,
                is_cleanup: _,
            } = data;

            let mut index = 0;
            for statement in statements {
                let location = Location {
                    block,
                    statement_index: index,
                };
                self.visit_statement(statement, location);
                let defs_in_cur_stmt = std::mem::take(&mut self.defs_in_cur_stmt);
                self.defs.add_item_to_array(defs_in_cur_stmt);
                index += 1;
            }

            if let Some(terminator) = terminator {
                let location = Location {
                    block,
                    statement_index: index,
                };
                self.visit_terminator(terminator, location);
                let defs_in_cur_stmt = std::mem::take(&mut self.defs_in_cur_stmt);
                self.defs.add_item_to_array(defs_in_cur_stmt);
            }

            self.defs.done_with_array();
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
                // self.defs.add_item_to_array(item);
                self.defs_in_cur_stmt.push(place.local);
                self.sites[place.local].insert(self.basic_block.unwrap());
            }
        }
    }

    Vis {
        sites: &mut sites,
        defs: &mut defs,
        defs_in_cur_stmt: SmallVec::default(),
        tcx,
        body,
        struct_topology,
        basic_block: None, // BasicBlock::from_u32(0),
    }
    .visit_body(body);

    Definitions {
        defs: defs.done(),
        sites,
    }
}
