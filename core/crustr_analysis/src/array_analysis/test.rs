use std::env;

use compiler_interface::run_compiler_with_file_with_single_func;
use rustc_middle::mir::visit::Visitor;

use crate::array_analysis::intra::IntraCtxt;

#[test]
fn test_print_mir() {
    let working_dir_path = env::current_dir().expect("current working directory value is invalid");
    run_compiler_with_file_with_single_func(
        working_dir_path.join("src/array_analysis/test/resource/simple/lib.rs"),
        |tcx, fn_did| {
            let body = tcx.optimized_mir(fn_did);

            let mut w = String::new();
            rustc_middle::mir::pretty::write_mir_fn(tcx, body, &mut |_, _| Ok(()), unsafe {
                &mut w.as_mut_vec()
            })
            .unwrap();
            println!("{}", w);

            // let mut intra = IntraContext::new(tcx, body);
            // intra.visit_body(body);
        },
    )
}
