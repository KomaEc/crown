//! For updated path, we enforce the preservation constraints

use rustc_middle::mir::{
    visit::{PlaceContext, Visitor},
    Local, Location, Place,
};

use super::{Base, Intraprocedural};
use crate::flow::{
    def_use::{Def, Update},
    ownership::{
        access_path::Path,
        constraint::{Constraint, Database, OwnershipToken, StorageMode},
    },
};

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

type EnsureUpdatedBase = Update<OwnershipToken>;

struct Wrapper<'p, 'analysis, 'tcx, Mode: StorageMode, DB>(
    &'p mut Intraprocedural<'analysis, 'tcx, Mode, DB>,
);

impl<'p, 'analysis, 'tcx, Mode: StorageMode, DB> Wrapper<'p, 'analysis, 'tcx, Mode, DB> {
    fn ensure_updated(&self, path: &Path<Base>) -> Option<Path<EnsureUpdatedBase>> {
        if let (local, Def(update)) = path.base {
            Some(path.map_base(|_| update.map(|ssa_idx| self.0.tokens[local][ssa_idx])))
        } else {
            None
        }
    }
}

impl<'p, 'analysis, 'tcx, Mode, DB> Visitor<'tcx> for Wrapper<'p, 'analysis, 'tcx, Mode, DB>
where
    Mode: StorageMode,
    DB: Database<Mode>,
{
    fn visit_local(&mut self, local: Local, context: PlaceContext, location: Location) {
        self.visit_place(&Place::from(local), context, location)
    }

    fn visit_place(&mut self, place: &Place<'tcx>, _: PlaceContext, location: Location) {
        if let Some(path) = self
            .0
            .path(place, location)
            .and_then(|path| self.ensure_updated(&path))
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
