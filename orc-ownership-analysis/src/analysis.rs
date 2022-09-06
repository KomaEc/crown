use crate::analysis::body_ext::BodyExt;
use crate::analysis::constraint::infer::{InferCtxt, Pure, Renamer, WithCtxt};
use crate::analysis::constraint::CadicalDatabase;
use crate::analysis::def::initial_definitions;
use crate::CrateCtxt;

pub(crate) mod body_ext;
pub(crate) mod constants;
pub(crate) mod constraint;
pub(crate) mod def;
// pub(crate) mod dom;
pub(crate) mod join_points;
pub(crate) mod state;
pub(crate) mod ty_ext;

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

    pub fn crash_me_with_inference_and_solve(&self) -> anyhow::Result<()> {
        let mut databases = Vec::with_capacity(self.functions().len());
        for &did in self.functions() {
            println!("solving {:?}", did);
            let body = self.tcx.optimized_mir(did);
            let dominance_frontier = body.compute_dominance_frontier();
            let definitions = initial_definitions(body, self.tcx, self);
            let mut infer_cx = InferCtxt::new(self, body, &definitions, CadicalDatabase::new());
            let mut rn = Renamer::new(body, &dominance_frontier, definitions);
            rn.go::<WithCtxt, _>(&mut infer_cx);
            match infer_cx.database.solver.solve() {
                Some(true) => println!("succeeded"),
                Some(false) => println!("failed"), // anyhow::bail!("failed in solving ownership constraints"),
                None => anyhow::bail!("timeout"),
            }
            databases.push(infer_cx.database);
        }
        Ok(())
    }
}
