use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;

/// [`RustProgram`] contains constructs we care about in the
/// Rust program. Right now, we only care about user defined
/// struct type and free-standing functions.
pub struct RustProgram<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub functions: Vec<DefId>,
    pub structs: Vec<DefId>,
}

use rustc_driver::Callbacks;
use rustc_interface::Config;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::libtree::LibtreeCompiler;

struct Text(String);

impl rustc_span::source_map::FileLoader for Text {
    fn file_exists(&self, path: &Path) -> bool {
        path == Path::new("lib.rs")
    }

    fn read_file(&self, path: &Path) -> std::io::Result<String> {
        if path == Path::new("lib.rs") {
            Ok(self.0.to_string())
        } else {
            Err(io::Error::other("oops"))
        }
    }

    fn read_binary_file(&self, _path: &Path) -> std::io::Result<std::sync::Arc<[u8]>> {
        Err(io::Error::other("oops"))
    }
}

struct TextCompiler<'callbacks>(&'callbacks mut (dyn Callbacks + Send), String);

impl rustc_driver::Callbacks for TextCompiler<'_> {
    fn config(&mut self, config: &mut Config) {
        config.file_loader = Some(Box::new(Text(self.1.clone())));
    }

    fn after_crate_root_parsing(
        &mut self,
        compiler: &rustc_interface::interface::Compiler,
        krate: &mut rustc_ast::Crate,
    ) -> rustc_driver::Compilation {
        self.0.after_crate_root_parsing(compiler, krate)
    }

    fn after_analysis<'tcx>(
        &mut self,
        compiler: &rustc_interface::interface::Compiler,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        self.0.after_analysis(compiler, tcx)
    }
}

struct WithRustProgram<F>(F);

impl<F> WithRustProgram<F> {
    fn new(f: F) -> Self {
        Self(f)
    }
}

impl<F> rustc_driver::Callbacks for WithRustProgram<F>
where
    F: FnMut(RustProgram),
{
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        let mut functions = Vec::new();
        let mut structs = Vec::new();

        for id in tcx.hir_free_items() {
            let item = tcx.hir_item(id);

            // Use pattern-matching to find a specific node inside the main function.
            match item.kind {
                rustc_hir::ItemKind::Fn { .. } => {
                    let def_id = item.hir_id().owner.def_id.to_def_id();
                    functions.push(def_id);
                }
                rustc_hir::ItemKind::Struct(..) => {
                    let def_id = item.hir_id().owner.def_id.to_def_id();
                    structs.push(def_id);
                }
                _ => {}
            }
        }

        self.0(RustProgram {
            tcx,
            functions,
            structs,
        });

        rustc_driver::Compilation::Stop
    }
}

pub enum SourceCode {
    Text(String),
    AbsolutePath(PathBuf),
    Libtree,
}

impl From<PathBuf> for SourceCode {
    fn from(value: PathBuf) -> Self {
        SourceCode::AbsolutePath(value)
    }
}

impl From<String> for SourceCode {
    fn from(value: String) -> Self {
        SourceCode::Text(value)
    }
}

impl From<&str> for SourceCode {
    fn from(value: &str) -> Self {
        SourceCode::Text(value.to_string())
    }
}

pub fn run_compiler<P, F>(program: P, callback: F)
where
    P: Into<SourceCode>,
    F: FnMut(RustProgram) + Send,
{
    match program.into() {
        SourceCode::Text(text) => compile_text(text, &mut WithRustProgram::new(callback)),
        SourceCode::AbsolutePath(path) => {
            compile_absolute_path(path, &mut WithRustProgram::new(callback))
        }
        SourceCode::Libtree => compile_libtree(&mut WithRustProgram::new(callback)),
    }
}

// const RUSTC_OPTIONS: &str = "--crate-type=lib -Awarnings -C opt-level=3";
// const RUSTC_OPTIONS: &str = "--crate-type=lib -Awarnings";

/// Builds rustc argument list based on prebuilt dependencies.
fn compiler_args(input_path: &Path) -> Vec<String> {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let extra_deps_dir = crate_dir
        .ancestors()
        .nth(2)
        .expect("crate_dir has no grandparent")
        .join("extra_deps");

    let mut args = vec![
        "rustc".to_owned(),
        input_path.to_str().unwrap().to_owned(),
        "--crate-type=lib".to_owned(),
        "-C".to_owned(),
        "opt-level=3".to_owned(),
        "--cap-lints".to_owned(),
        "allow".to_owned(),
        "-Awarnings".to_owned(),
        "-L".to_owned(),
        extra_deps_dir.to_str().unwrap().to_owned(),
    ];

    for entry in fs::read_dir(&extra_deps_dir).expect("missing extra_deps dir") {
        let path = entry.unwrap().path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        let lib_prefix = filename.strip_prefix("lib").unwrap_or(filename);
        let crate_name = lib_prefix.split('-').next().unwrap();

        let extern_arg = match crate_name {
            "libc" => format!("libc={}", path.to_str().unwrap()),
            "c2rust_bitfields" => format!("c2rust_bitfields={}", path.to_str().unwrap()),
            "c2rust_bitfields_derive" => {
                format!("c2rust_bitfields_derive={}", path.to_str().unwrap())
            }
            #[cfg(target_arch = "x86_64")]
            "f128" => format!("f128={}", path.to_str().unwrap()),
            #[cfg(target_arch = "x86_64")]
            "f128_internal" => format!("f128_internal={}", path.to_str().unwrap()),
            "num_traits" => format!("num_traits={}", path.to_str().unwrap()),
            _ => continue,
        };

        args.push("--extern".to_owned());
        args.push(extern_arg);
    }

    args
}

pub fn compile_libtree(callbacks: &mut (dyn Callbacks + Send)) {
    let args = compiler_args(Path::new("lib.rs"));
    rustc_driver::run_compiler(&args, &mut LibtreeCompiler(callbacks));
}

pub fn compile_text(program: String, callbacks: &mut (dyn Callbacks + Send)) {
    let args = compiler_args(Path::new("lib.rs"));
    rustc_driver::run_compiler(&args, &mut TextCompiler(callbacks, program));
}

pub fn compile_absolute_path(program: PathBuf, callbacks: &mut (dyn Callbacks + Send)) {
    let args = compiler_args(&program);
    rustc_driver::run_compiler(&args, callbacks);
}
