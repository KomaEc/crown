use crustr_rustc_interface;
use std::env;

fn main() -> std::io::Result<()> {
    env_logger::init();

    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Expect appropriate arguments");
    }
    let input_file_name: &str = &args[1];
    crustr_rustc_interface::run(input_file_name.to_string());
    Ok(())
}
