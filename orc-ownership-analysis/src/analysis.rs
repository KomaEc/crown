use crate::CrateInfo;

use self::constraint::{CadicalDatabase, Database};

pub mod body_ext;
pub mod constants;
pub mod constraint;
pub mod def_sites;
pub mod join_points;
pub mod place_ext;
pub mod state;
pub mod ty_ext;
// pub mod sssa;

pub struct OwnershipAnalysisCtxt<'octxt, 'tcx, DB = CadicalDatabase>
where
    DB: Database,
{
    program: &'octxt CrateInfo<'tcx>,
    database: DB,
}

impl<'octxt, 'tcx> OwnershipAnalysisCtxt<'octxt, 'tcx> {
    pub fn new(program: &'octxt CrateInfo<'tcx>) -> Self {
        Self {
            program,
            database: CadicalDatabase::new(),
        }
    }
}
