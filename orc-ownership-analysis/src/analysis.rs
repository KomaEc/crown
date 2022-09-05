use crate::analysis::body_ext::BodyExt;
use crate::CrateCtxt;
use crate::analysis::constraint::CadicalDatabase;
use crate::analysis::constraint::infer::{InferCtxt, WithCtxt};

use self::constraint::infer::{Pure, Renamer};
use self::def::initial_definitions;

pub(crate) mod body_ext;
pub(crate) mod constants;
pub(crate) mod constraint;
pub(crate) mod def;
pub(crate) mod dom;
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

impl<'tcx> CrateCtxt<'tcx> {
    pub fn crash_me_with_pure_rename(&self) {
        for &did in self.functions() {
            println!("renaming {:?}", did);
            let body = self.tcx.optimized_mir(did);
            let dominance_frontier = body.compute_dominance_frontier();
            let definitions = initial_definitions(body, self.tcx, self);
            let mut rn = Renamer::new(body, &dominance_frontier, definitions);
            rn.go::<Pure, ()>(());
            println!("completed");
        }
    }

    pub fn crash_me_with_inference(&self) {
        for &did in self.functions() {
            println!("inferring {:?}", did);
            let body = self.tcx.optimized_mir(did);
            let dominance_frontier = body.compute_dominance_frontier();
            let definitions = initial_definitions(body, self.tcx, self);
            let mut infer_cx = InferCtxt::new(self, body, &definitions, CadicalDatabase::new());
            let mut rn = Renamer::new(body, &dominance_frontier, definitions);
            rn.go::<WithCtxt, _>(&mut infer_cx);
            println!("completed");
        }
    }
}
