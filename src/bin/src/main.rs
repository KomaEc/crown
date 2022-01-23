use clap::Parser;
use crustr_rustc_interface::{config_setup, run_compiler_with_config};
use std::path::PathBuf;
use std::{fs, process};

mod common_defs;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The path to the file or dir to read
    #[clap(parse(from_os_str))]
    path: PathBuf,

    /// Test on a single rust file
    #[clap(short, long)]
    single_file: bool,
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    if cli.single_file {
        let dir_path = cli.path;
        if !dir_path.is_dir() {
            eprintln!(
                "Expect a directory, given: {}",
                dir_path.as_path().to_str().unwrap()
            );
            process::exit(1);
        }

        let mut lib_entry = None;
        for entry in dir_path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                if let Some(extension) = entry.path().extension() {
                    if extension == "toml" {
                        assert!(entry.path().is_file());
                        if let Ok(cargo_toml) = fs::read_to_string(entry.path()) {
                            if let Ok(cargo_toml) = cargo_toml.parse::<toml::Value>() {
                                if let Some(path) = cargo_toml["lib"]["path"].as_str() {
                                    lib_entry = Some(PathBuf::from(path));
                                }
                            }
                        }
                    }
                }
            }
        }

        let lib_entry = match lib_entry {
            None => {
                eprintln!("Cannot find lib entry");
                process::exit(1);
            }
            Some(lib_entry) => lib_entry,
        };

        common_defs::add_defs_to_dir(dir_path.as_path());

        let config = config_setup(lib_entry.into());
        run_compiler_with_config::<crustr_rustc_interface::collect_structs::CollectStructInfo>(
            config,
        )
    } else {
        let config = config_setup(cli.path);
        run_compiler_with_config::<crustr_rustc_interface::collect_structs::CollectStructInfo>(
            config,
        )
    }
}
