use clap::Parser;
use crustr_rustc_interface::{config_setup, run_compiler_with_config};
use std::path::PathBuf;
use std::{env, fs, process};
use toml_edit::{value, Document};

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

    if !cli.single_file {
        let dir_path = cli.path;
        if !dir_path.is_dir() {
            eprintln!(
                "Expect a directory, given: {}",
                dir_path.as_path().to_str().unwrap()
            );
            process::exit(1);
        }

        let working_dir_path =
            env::current_dir().expect("current working directory value is invalid");

        let mut relative_lib_entry = None;
        for entry in dir_path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                if let Some(extension) = entry.path().extension() {
                    if extension == "toml" {
                        assert!(entry.path().is_file());

                        let mut cargo_toml = fs::read_to_string(entry.path())
                            .expect("failed to read Cargo.toml")
                            .parse::<Document>()
                            .expect("invalid doc");

                        if let Some(path) = cargo_toml["lib"]["path"].as_str() {
                            relative_lib_entry = Some(PathBuf::from(path));
                        }

                        cargo_toml["dependencies"]["crustr_ptr"] = format!(
                            "{{ path = \"{}\" }}",
                            working_dir_path.join("crustr_ptr").as_path().display()
                        )
                        .parse::<toml_edit::Item>()
                        .unwrap();

                        fs::write(entry.path(), cargo_toml.to_string())
                            .expect("failed to write dependency into Cargo.toml");
                    }
                }
            }
        }

        let lib_entry = match relative_lib_entry {
            None => {
                eprintln!("Cannot find lib entry");
                process::exit(1);
            }
            Some(lib_entry) => dir_path.join(lib_entry),
        };

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
