use rustc_index::bit_set::MixedBitSet;
use rustc_middle::{
    mir::{Body, Local, Location},
    ty::TyCtxt,
};
use rustc_mir_dataflow::Analysis;
use utils::rustc_hash::FxHashMap;

use crate::reaching_definitions::{
    DefIndex, DefinitionSet, ReachingDefinitions, compute_definition_set,
};

pub enum UseDefLocation {
    /// Function Parameter (defined at entry)
    Argument,
    /// Single Definition at a specific location
    Single(Location),
    /// Multiple definitions (phi-node like situation)
    Multiple(Vec<Location>),
}

pub struct UseDef {
    definition_set: DefinitionSet,
    reaching_definitions: FxHashMap<Location, MixedBitSet<DefIndex>>,
}

impl UseDef {
    pub fn def_loc(&self, local: Local, location: Location) -> UseDefLocation {
        let iter = self._def_loc(local, location);
        match iter.collect::<Vec<_>>().as_slice() {
            [] => UseDefLocation::Argument,
            [single] => UseDefLocation::Single(*single),
            multiple => UseDefLocation::Multiple(multiple.to_vec()),
        }
    }

    fn _def_loc(&self, local: Local, location: Location) -> impl Iterator<Item = Location> {
        self.reaching_definitions
            .get(&location)
            .into_iter()
            .flat_map(|defs| defs.iter())
            .filter_map(move |def_index| {
                let definition = &self.definition_set.definitions[def_index];
                (definition.defined == local).then_some(definition.location)
            })
    }

    pub fn new<'tcx>(tcx: TyCtxt<'tcx>, body: &Body<'tcx>) -> Self {
        let definition_set = compute_definition_set(body);

        let mut analysis = ReachingDefinitions {
            definition_set: &definition_set,
        }
        .iterate_to_fixpoint(tcx, body, None)
        .into_results_cursor(body);

        let mut reaching_definitions = FxHashMap::default();

        for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
            analysis.seek_to_block_start(bb);

            let bb_len = bb_data.statements.len() + bb_data.terminator.is_some() as usize;
            for position in 0..bb_len {
                let location = Location {
                    block: bb,
                    statement_index: position,
                };

                analysis.seek_before_primary_effect(location);

                reaching_definitions.insert(location, analysis.get().clone());
            }
        }

        UseDef {
            definition_set,
            reaching_definitions,
        }
    }
}
