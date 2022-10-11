use alias::AliasResult;

use crate::CrateCtxt;

pub mod flow_insensitive;
pub mod noalias;
pub mod output;

pub fn show_param_qualifiers(crate_ctxt: &CrateCtxt, alias_result: &AliasResult) {
    fn show_set<T: std::fmt::Debug>(set: impl Iterator<Item = T>) -> String {
        set.map(|x| format!("{:?}", x))
            .collect::<Vec<_>>()
            .join(", ")
    }

    for &did in crate_ctxt.fns() {
        let body = crate_ctxt.tcx.optimized_mir(did);
        let output_params = output::least_output_params(body, crate_ctxt);
        let output_params_str = show_set(output_params.iter());

        let noalias_params = noalias::conservative_noalias_params(body, alias_result);
        let noalias_params_str = show_set(noalias_params.iter());

        let unique_params = output_params.intersection(&noalias_params);
        let unique_params_str = show_set(unique_params);

        println!("@{}:", crate_ctxt.tcx.def_path_str(did));
        println!("  {:<20}: {output_params_str}", "output_params");
        println!("  {:<20}: {noalias_params_str}", "noalias_params");
        println!("  {:<20}: {unique_params_str}", "unique_params");
    }
}
