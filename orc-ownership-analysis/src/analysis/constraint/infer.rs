use rustc_data_structures::graph::WithSuccessors;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{visit::Visitor, BasicBlock, BasicBlockData, Body, Location, Statement, StatementKind, Place, Rvalue},
    ty::TyCtxt,
};

use crate::{
    analysis::{def_sites::Definitions, state::SSAState},
    CrateInfo,
};

use super::{CadicalDatabase, Database};

/// Should it hold 'tcx at all?
pub(crate) struct Ctxt<'me, 'tcx: 'me, DB = CadicalDatabase>
where
    DB: Database,
{
    crate_info: &'me CrateInfo<'tcx>,
    database: &'me mut DB,
    // TODO: signatures for the function that is analysed (and perhaps
    // those in the same connected component)
}

/// FIXME: is it the right way?
pub(crate) trait Mode {
    type Ctxt<'me, 'tcx, DB>
    where
        Self: 'me,
        'tcx: 'me,
        DB: Database + 'me;
}
#[derive(Debug)]
pub(crate) struct Pure;
#[derive(Debug)]
pub(crate) struct WithCtxt;
impl Mode for Pure {
    type Ctxt<'me, 'tcx: 'me, DB: Database + 'me> = ();
}
impl Mode for WithCtxt {
    type Ctxt<'me, 'tcx, DB> = Ctxt<'me, 'tcx, DB> where Self: 'me, 'tcx: 'me, DB: Database + 'me;
}

pub(crate) struct Renamer<'me, 'tcx: 'me> {
    body: &'me Body<'tcx>,
    tcx: TyCtxt<'tcx>,
    state: SSAState,
    definitions: Definitions,
}

impl<'me, 'tcx: 'me> Renamer<'me, 'tcx> {
    pub(crate) fn go<M: Mode>(&mut self) {
        let dominators = self.body.basic_blocks.dominators();
        let mut children = IndexVec::from_elem(vec![], self.body.basic_blocks());
        let mut root = BasicBlock::from_u32(0);
        self.body.basic_blocks().indices().for_each(|bb| {
            let dom = dominators.immediate_dominator(bb);
            if dom != bb {
                children[dom].push(bb)
            } else {
                root = bb;
            }
        });
        assert_eq!(root, BasicBlock::from_u32(0));

        enum State {
            ToVisit,
            ToPopNames,
        }

        let mut recursion = vec![(root, State::ToVisit)];

        while let Some((bb, state)) = recursion.pop() {
            match state {
                State::ToVisit => {
                    // self.state.name_state.enter_new_block();
                    self.go_basic_block::<M>(bb, &self.body.basic_blocks()[bb]);
                    recursion.push((bb, State::ToPopNames));
                    recursion.extend(children[bb].iter().rev().map(|&bb| (bb, State::ToVisit)));
                }
                State::ToPopNames => {
                    for &local in self
                        .definitions
                        .of_block(bb)
                        .iter()
                        .flatten()
                    {
                        self.state.name_state.pop(local);
                    }
                }
            }
        }
    }

    pub(crate) fn go_basic_block<M: Mode>(&mut self, bb: BasicBlock, data: &BasicBlockData) {
        tracing::debug!("Renaming {:?}", bb);

        let BasicBlockData {
            statements,
            terminator,
            is_cleanup: _,
        } = data;

        for (local, phi_node) in self.state.join_points[bb].iter_enumerated_mut() {
            let ssa_idx = self.state.name_state.generate_fresh_name(local);
            phi_node.lhs = ssa_idx;
            todo!()
        }

        let mut index = 0;
        for statement in statements {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            // go statement
            todo!();
            index += 1;
        }

        if let Some(terminator) = terminator {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            // go terminator
            todo!()
        }

        for succ in self.body.basic_blocks.successors(bb) {
            for (local, phi_node) in self.state.join_points[succ].iter_enumerated_mut() {
                let ssa_idx = self.state.name_state.get_name(local);
                phi_node.rhs.push(ssa_idx)
            }
        }

        todo!()
    }
}

impl<'me, 'tcx: 'me> Visitor<'tcx> for Renamer<'me, 'tcx> {
    fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location,) {
        match &statement.kind {
            StatementKind::Assign(box (place, rvalue)) => self.visit_assign(place, rvalue, location),
            StatementKind::SetDiscriminant { .. } => todo!(),
            StatementKind::AscribeUserType(_, _) => todo!(),
            StatementKind::StorageLive(_) |
            StatementKind::StorageDead(_) |
            StatementKind::Deinit(_) |
            StatementKind::Retag(_, _) |
            StatementKind::FakeRead(_) |
            StatementKind::Coverage(_) |
            StatementKind::CopyNonOverlapping(_) |
            StatementKind::Nop => unreachable!("statement {:?} is not assumed to appear", statement),
        }
    }

    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location,) {
        tracing::debug!("processing assignment {:?} = {:?}", place, rvalue);
        
    }
}
