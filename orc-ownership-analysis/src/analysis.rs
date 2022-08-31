use crate::analysis::body_ext::BodyExt;
use crate::CrateInfo;

use self::constraint::infer::{Pure, Renamer};
use self::def::initial_definitions;

pub(crate) mod body_ext;
pub(crate) mod constants;
pub mod constraint;
pub(crate) mod def;
pub(crate) mod join_points;
pub(crate) mod place_ext;
pub(crate) mod state;
pub(crate) mod ty_ext;
// pub(crate) mod sssa;

// pub(crate) struct OwnershipAnalysisCtxt<'octxt, 'tcx, DB = CadicalDatabase>
// where
//     DB: Database,
// {
//     program: &'octxt CrateInfo<'tcx>,
//     database: DB,
// }

// impl<'octxt, 'tcx> OwnershipAnalysisCtxt<'octxt, 'tcx> {
//     pub(crate) fn new(program: &'octxt CrateInfo<'tcx>) -> Self {
//         Self {
//             program,
//             database: CadicalDatabase::new(),
//         }
//     }
// }

impl<'tcx> CrateInfo<'tcx> {
    pub fn rename_all(&self) {
        for &did in self.functions() {
            println!("renaming {:?}", did);
            let body = self.tcx.optimized_mir(did);
            let dominance_frontier = body.compute_dominance_frontier();
            let definitions = initial_definitions(body, self.tcx, &self.struct_topology);
            let mut rn = Renamer::new(
                self.tcx,
                body,
                &self.struct_topology,
                &dominance_frontier,
                definitions,
            );
            rn.go::<Pure>();
            println!("completed");
        }
    }
}
