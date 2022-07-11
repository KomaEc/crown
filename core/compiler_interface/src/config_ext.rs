use rustc_errors::registry;
use rustc_interface::Config;
use rustc_session::config;
use std::path::PathBuf;
use std::process;
use std::str;

pub trait ConfigExt {
    fn with_input(input: config::Input) -> Config;
    fn with_input_file(input_path: PathBuf) -> Config;
    fn with_input_str(input: &'static str) -> Config;
}

impl ConfigExt for Config {
    fn with_input(input: config::Input) -> Config {
        let out = process::Command::new("rustc")
            .arg("--print=sysroot")
            .current_dir(".")
            .output()
            .unwrap();
        let sysroot = str::from_utf8(&out.stdout).unwrap().trim();
        Config {
            opts: config::Options {
                maybe_sysroot: Some(PathBuf::from(sysroot)),
                ..config::Options::default()
            },
            crate_cfg: rustc_hash::FxHashSet::default(),
            crate_check_cfg: Default::default(),
            input,
            input_path: None,
            output_dir: None,
            output_file: None,
            file_loader: None,
            diagnostic_output: rustc_session::DiagnosticOutput::Default,
            lint_caps: rustc_hash::FxHashMap::default(),
            parse_sess_created: None,
            register_lints: None,
            override_queries: None,
            make_codegen_backend: None,
            registry: registry::Registry::new(&rustc_error_codes::DIAGNOSTICS),
        }
    }

    fn with_input_file(input_path: PathBuf) -> Config {
        Config::with_input(config::Input::File(input_path))
    }

    fn with_input_str(input: &'static str) -> Config {
        Config::with_input(config::Input::Str {
            name: rustc_span::FileName::Custom("main.rs".to_string()),
            input: input.to_string(),
        })
    }
}
