use rustc_middle::mir::{Body, Location};

pub trait BodyExt<'tcx> {
    fn locations(&self) -> impl Iterator<Item = Location> + '_;
}

impl BodyExt<'_> for Body<'_> {
    fn locations(&self) -> impl Iterator<Item = Location> + '_ {
        self.basic_blocks
            .iter_enumerated()
            .flat_map(|(bb, bb_data)| {
                (0..(bb_data.statements.len() + bb_data.terminator.iter().count())).map(
                    move |index| Location {
                        block: bb,
                        statement_index: index,
                    },
                )
            })
    }
}
