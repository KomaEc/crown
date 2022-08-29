use crate::CrateInfo;

use self::constraint::{CadicalDatabase, Database};

pub(crate) mod body_ext;
pub(crate) mod constants;
pub mod constraint;
pub(crate) mod def;
pub(crate) mod join_points;
pub(crate) mod place_ext;
pub(crate) mod state;
pub(crate) mod ty_ext;
// pub(crate) mod sssa;

pub(crate) struct OwnershipAnalysisCtxt<'octxt, 'tcx, DB = CadicalDatabase>
where
    DB: Database,
{
    program: &'octxt CrateInfo<'tcx>,
    database: DB,
}

impl<'octxt, 'tcx> OwnershipAnalysisCtxt<'octxt, 'tcx> {
    pub(crate) fn new(program: &'octxt CrateInfo<'tcx>) -> Self {
        Self {
            program,
            database: CadicalDatabase::new(),
        }
    }
}
