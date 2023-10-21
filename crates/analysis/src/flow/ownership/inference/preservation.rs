//! For updated path, we enforce the preservation constraints

use rustc_middle::mir::{
    visit::{PlaceContext, Visitor},
    Location, Place,
};

use super::Intraprocedural;
use crate::flow::ownership::constraint::{Constraint, Database, StorageMode};

impl<'analysis, 'tcx, Mode, DB> Intraprocedural<'analysis, 'tcx, Mode, DB>
where
    Mode: StorageMode,
    DB: Database<Mode>,
{
    pub fn enforce_preservation(&mut self) {
        // awkward LOL
        let body = self.tcx.optimized_mir(self.body.source.def_id());
        Wrapper(self).visit_body(body);
    }
}

struct Wrapper<'p, 'analysis, 'tcx, Mode: StorageMode, DB>(
    &'p mut Intraprocedural<'analysis, 'tcx, Mode, DB>,
);

impl<'p, 'analysis, 'tcx, Mode, DB> Visitor<'tcx> for Wrapper<'p, 'analysis, 'tcx, Mode, DB>
where
    Mode: StorageMode,
    DB: Database<Mode>,
{
    // Note that we don't visit local here. Indices cannot be paths, so we can safely ignore them.
    // In return locations, variables are inspected but not really used. So we just ignore them.
    fn visit_place(&mut self, place: &Place<'tcx>, _: PlaceContext, location: Location) {
        if let Some(path) = self
            .0
            .path(place, location)
            .map(|path| self.0.expand(&path))
        {
            let base_size = self
                .0
                .ctxt
                .access_paths
                .size_of(self.0.k_limit, self.0.body.local_decls[place.local].ty);
            let projection_offset = path.projection_offset();
            let size = path.num_pointers_reachable();
            // preservation constraints:
            // 1. [0..projection_offset),
            let use_range = path.base.r#use..path.base.r#use + projection_offset;
            let def_range = path.base.def..path.base.def + projection_offset;
            for (r#use, def) in use_range.zip(def_range) {
                self.0.ctxt.database.add(
                    Constraint::Equal { x: r#use, y: def },
                    &mut self.0.ctxt.storage,
                );
            }
            // 2. [projection_offset + size..base_size).
            assert!(projection_offset + size <= base_size);
            let use_range = path.base.r#use + projection_offset + size..path.base.r#use + base_size;
            let def_range = path.base.def + projection_offset + size..path.base.def + base_size;
            for (r#use, def) in use_range.zip(def_range) {
                self.0.ctxt.database.add(
                    Constraint::Equal { x: r#use, y: def },
                    &mut self.0.ctxt.storage,
                );
            }
        }
    }
}
