use rustc_index::{IndexVec, bit_set::MixedBitSet};
use rustc_middle::mir::{
    Body, Local, Location, Place,
    visit::{PlaceContext, Visitor},
};
use rustc_mir_dataflow::{Analysis, Forward, fmt::DebugWithContext};
use utils::{rustc_hash::FxHashMap, smallvec::SmallVec};

use crate::liveness::DefUse;

rustc_index::newtype_index! {
    #[orderable]
    #[debug_format = "def_{}"]
    pub struct DefIndex {
    }
}

impl<C> DebugWithContext<C> for DefIndex {}

pub struct Definition {
    pub(crate) defined: Local,
    pub(crate) location: Location,
}

pub struct DefinitionSet {
    pub(crate) definitions: IndexVec<DefIndex, Definition>,
    location_map: FxHashMap<Location, DefIndex>,
    local_map: IndexVec<Local, MixedBitSet<DefIndex>>,
    uses: FxHashMap<Location, SmallVec<[Local; 2]>>,
}

pub fn compute_definition_set(body: &Body) -> DefinitionSet {
    struct Vis(DefinitionSet);

    impl<'tcx> Visitor<'tcx> for Vis {
        fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, location: Location) {
            match DefUse::for_place(*place, context) {
                Some(DefUse::Def) => {
                    let def_index = self.0.definitions.push(Definition {
                        defined: place.local,
                        location,
                    });
                    self.0.location_map.insert(location, def_index);
                }
                Some(DefUse::Use) => {
                    let uses = self.0.uses.entry(location).or_default();
                    if !uses.contains(&place.local) {
                        uses.push(place.local);
                    }
                }
                _ => {}
            }
        }

        fn visit_local(&mut self, local: Local, context: PlaceContext, location: Location) {
            self.visit_place(&Place::from(local), context, location);
        }
    }
    let mut vis = Vis(DefinitionSet {
        definitions: IndexVec::new(),
        location_map: FxHashMap::default(),
        local_map: IndexVec::new(),
        uses: FxHashMap::default(),
    });
    vis.visit_body(body);

    vis.0.local_map = IndexVec::from_elem(
        MixedBitSet::new_empty(vis.0.definitions.len()),
        &body.local_decls,
    );
    for (def_index, definition) in vis.0.definitions.iter_enumerated() {
        vis.0.local_map[definition.defined].insert(def_index);
    }
    vis.0
}

pub struct ReachingDefinitions<'r> {
    pub(crate) definition_set: &'r DefinitionSet,
}

impl ReachingDefinitions<'_> {
    fn apply_location_effect(&mut self, state: &mut MixedBitSet<DefIndex>, location: Location) {
        if let Some(&def_index) = self.definition_set.location_map.get(&location) {
            let defined = self.definition_set.definitions[def_index].defined;
            state.subtract(&self.definition_set.local_map[defined]);
            state.insert(def_index);
        }
    }
}

impl<'tcx> Analysis<'tcx> for ReachingDefinitions<'_> {
    type Domain = MixedBitSet<DefIndex>;
    type Direction = Forward;

    const NAME: &'static str = "reaching definitions";

    fn bottom_value(&self, _body: &Body<'tcx>) -> Self::Domain {
        MixedBitSet::new_empty(self.definition_set.definitions.len())
    }

    fn initialize_start_block(&self, _body: &Body<'tcx>, _state: &mut Self::Domain) {}

    fn apply_primary_statement_effect(
        &mut self,
        state: &mut Self::Domain,
        _statement: &rustc_middle::mir::Statement<'tcx>,
        location: Location,
    ) {
        self.apply_location_effect(state, location);
    }

    fn apply_primary_terminator_effect<'mir>(
        &mut self,
        state: &mut Self::Domain,
        terminator: &'mir rustc_middle::mir::Terminator<'tcx>,
        location: Location,
    ) -> rustc_middle::mir::TerminatorEdges<'mir, 'tcx> {
        self.apply_location_effect(state, location);
        terminator.edges()
    }
}
