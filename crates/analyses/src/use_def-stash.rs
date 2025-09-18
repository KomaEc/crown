//! Construct use-def chain without SSA

use rustc_index::IndexVec;
use rustc_middle::{
    mir::{
        BasicBlock, Body, Local, Location, Place, Rvalue,
        visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor},
    },
    ty::TyCtxt,
};

use crate::ssa::RichLocation;

/// Simple def-use chain without SSA
pub struct DefUseChain {
    /// For each local, maps from use locations to their defining location
    def_map: IndexVec<Local, Vec<(Location, RichLocation)>>,
    /// For each location, what locals are used there
    use_map: IndexVec<BasicBlock, Vec<Vec<(Local, bool)>>>, // [block][stmt][(local, is_def)]
}

impl DefUseChain {
    pub fn uses(&self, location: Location) -> impl Iterator<Item = Local> + '_ {
        self.use_map
            .get(location.block)
            .and_then(|block_stmts| block_stmts.get(location.statement_index))
            .into_iter()
            .flatten()
            .filter_map(|(local, is_def)| (!is_def).then_some(*local))
    }

    pub fn def_loc(&self, local: Local, location: Location) -> RichLocation {
        // Find the most recent definition of this local before the current location
        self.def_map[local]
            .iter()
            .rev()
            .find(|(use_loc, _)| self.dominates(*use_loc, location))
            .map(|(_, def_loc)| *def_loc)
            .unwrap_or(RichLocation::Entry)
    }

    pub fn phi_def_locs(
        &self,
        local: Local,
        block: BasicBlock,
    ) -> impl Iterator<Item = RichLocation> + '_ {
        // For SSA-free approach, we just return definitions that reach this block
        self.def_map[local]
            .iter()
            .filter(move |(use_loc, _)| use_loc.block != block)
            .map(|(_, def_loc)| *def_loc)
    }

    /// Simple dominance check - location1 dominates location2 if it comes before it
    fn dominates(&self, loc1: Location, loc2: Location) -> bool {
        loc1.block.index() < loc2.block.index()
            || (loc1.block == loc2.block && loc1.statement_index <= loc2.statement_index)
    }
}

pub fn def_use_chain<'tcx>(body: &Body<'tcx>, _tcx: TyCtxt<'tcx>) -> DefUseChain {
    let mut def_map = IndexVec::from_elem(Vec::new(), &body.local_decls);
    let mut use_map = IndexVec::from_elem(Vec::new(), &body.basic_blocks);

    for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
        let mut block_uses = Vec::new();
        let mut statement_index = 0;

        for statement in bb_data.statements.iter() {
            let location = Location {
                block: bb,
                statement_index,
            };
            let mut stmt_uses = Vec::new();

            // Collect uses and defs for this statement
            let mut collector = DefUseCollector {
                uses_and_defs: Vec::new(),
                current_location: location,
            };
            collector.visit_statement(statement, location);

            // Process the collected information
            for (local, is_def, _) in collector.uses_and_defs {
                stmt_uses.push((local, is_def));

                if !is_def {
                    // This is a use - find its definition
                    let def_loc = find_reaching_def(&def_map, local, location);
                    def_map[local].push((location, def_loc));
                }
            }

            block_uses.push(stmt_uses);
            statement_index += 1;
        }

        if let Some(terminator) = &bb_data.terminator {
            let location = Location {
                block: bb,
                statement_index,
            };
            let mut stmt_uses = Vec::new();

            let mut collector = DefUseCollector {
                uses_and_defs: Vec::new(),
                current_location: location,
            };
            collector.visit_terminator(terminator, location);

            for (local, is_def, _) in collector.uses_and_defs {
                stmt_uses.push((local, is_def));

                if !is_def {
                    let def_loc = find_reaching_def(&def_map, local, location);
                    def_map[local].push((location, def_loc));
                }
            }

            block_uses.push(stmt_uses);
        }

        use_map[bb] = block_uses;
    }

    DefUseChain { def_map, use_map }
}

fn find_reaching_def(
    def_map: &IndexVec<Local, Vec<(Location, RichLocation)>>,
    local: Local,
    use_location: Location,
) -> RichLocation {
    // Simple backward scan to find the most recent definition
    def_map[local]
        .iter()
        .rev()
        .find(|(def_loc, _)| def_loc < &use_location)
        .map(|(_, def_loc)| *def_loc)
        .unwrap_or(RichLocation::Entry)
}

struct DefUseCollector {
    uses_and_defs: Vec<(Local, bool, Location)>, // (local, is_def, location)
    current_location: Location,
}

impl<'tcx> Visitor<'tcx> for DefUseCollector {
    fn visit_local(&mut self, local: Local, context: PlaceContext, _location: Location) {
        match DefUse::for_place(Place::from(local), context) {
            Some(DefUse::Def) => {
                self.uses_and_defs
                    .push((local, true, self.current_location));
            }
            Some(DefUse::Use) => {
                self.uses_and_defs
                    .push((local, false, self.current_location));
            }
            None => {}
        }
    }

    fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, _location: Location) {
        match DefUse::for_place(*place, context) {
            Some(DefUse::Def) => {
                self.uses_and_defs
                    .push((place.local, true, self.current_location));
            }
            Some(DefUse::Use) => {
                self.uses_and_defs
                    .push((place.local, false, self.current_location));
            }
            None => {}
        }

        // Visit projections (for indexing operations)
        self.super_projection(place.as_ref(), context, self.current_location);
    }

    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        // Handle special case for self-assignment like _1 = BitAnd(_1, _3)
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

#[derive(Eq, PartialEq, Clone)]
pub enum DefUse {
    Def,
    Use,
}

impl DefUse {
    fn for_place<'tcx>(place: Place<'tcx>, context: PlaceContext) -> Option<DefUse> {
        match context {
            PlaceContext::NonUse(_) => None,

            PlaceContext::MutatingUse(
                MutatingUseContext::Call
                | MutatingUseContext::Yield
                | MutatingUseContext::AsmOutput
                | MutatingUseContext::Store
                | MutatingUseContext::Deinit,
            ) => {
                if place.is_indirect() {
                    // Treat derefs as a use of the base local. `*p = 4` is not a def of `p` but a
                    // use.
                    Some(DefUse::Use)
                } else if place.projection.is_empty() {
                    Some(DefUse::Def)
                } else {
                    None
                }
            }

            // Setting the discriminant is not a use because it does no reading, but it is also not
            // a def because it does not overwrite the whole place
            PlaceContext::MutatingUse(MutatingUseContext::SetDiscriminant) => {
                place.is_indirect().then_some(DefUse::Use)
            }

            // All other contexts are uses...
            PlaceContext::MutatingUse(
                MutatingUseContext::RawBorrow
                | MutatingUseContext::Borrow
                | MutatingUseContext::Drop
                | MutatingUseContext::Retag,
            )
            | PlaceContext::NonMutatingUse(
                NonMutatingUseContext::RawBorrow
                | NonMutatingUseContext::Copy
                | NonMutatingUseContext::Inspect
                | NonMutatingUseContext::Move
                | NonMutatingUseContext::FakeBorrow
                | NonMutatingUseContext::SharedBorrow
                | NonMutatingUseContext::PlaceMention,
            ) => Some(DefUse::Use),

            PlaceContext::MutatingUse(MutatingUseContext::Projection)
            | PlaceContext::NonMutatingUse(NonMutatingUseContext::Projection) => {
                unreachable!("A projection could be a def or a use and must be handled separately")
            }
        }
    }
}

pub fn show_def_use_chain(body: &Body, def_use_chain: &DefUseChain) {
    println!("@{:?}", body.source.def_id());
    for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
        println!("{:?}:", bb);
        let mut statement_index = 0;
        for statement in bb_data.statements.iter() {
            println!("  {:?}", statement);

            let location = Location {
                block: bb,
                statement_index,
            };
            let uses = def_use_chain
                .uses(location)
                .map(|local| (local, def_use_chain.def_loc(local, location)))
                .map(|(local, loc)| format!("{:?}@{:?}", local, loc))
                .collect::<Vec<_>>()
                .join(", ");
            println!("  using: {uses}");

            statement_index += 1;
        }
        if let Some(terminator) = &bb_data.terminator {
            println!("  {:?}", terminator.kind);
            let location = Location {
                block: bb,
                statement_index,
            };
            let uses = def_use_chain
                .uses(location)
                .map(|local| (local, def_use_chain.def_loc(local, location)))
                .map(|(local, loc)| format!("{:?}@{:?}", local, loc))
                .collect::<Vec<_>>()
                .join(", ");
            println!("  using: {uses}");
        }
    }
}
