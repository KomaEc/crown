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

pub struct WithRustProgram<F>(pub F);

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
        SourceCode::Text(text) => compile_text(text, &mut WithRustProgram(callback)),
        SourceCode::AbsolutePath(path) => {
            compile_absolute_path(path, &mut WithRustProgram(callback))
        }
        SourceCode::Libtree => compile_libtree(&mut WithRustProgram(callback)),
    }
}

const RUSTC_OPTIONS: &str = "--crate-type=lib -Awarnings -C opt-level=3";
// const RUSTC_OPTIONS: &str = "--crate-type=lib -Awarnings";

pub fn compile_libtree(callbacks: &mut (dyn Callbacks + Send)) {
    let mut args = vec!["ignored", "lib.rs"];
    args.extend(RUSTC_OPTIONS.split(" "));
    let args = args
        .into_iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    rustc_driver::run_compiler(&args, &mut LibtreeCompiler(callbacks));
}

pub fn compile_text(program: String, callbacks: &mut (dyn Callbacks + Send)) {
    let mut args = vec!["ignored", "lib.rs"];
    args.extend(RUSTC_OPTIONS.split(" "));
    let args = args
        .into_iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    rustc_driver::run_compiler(&args, &mut TextCompiler(callbacks, program));
}

pub fn compile_absolute_path(program: PathBuf, callbacks: &mut (dyn Callbacks + Send)) {
    let mut args = vec!["ignored", program.to_str().unwrap()];
    args.extend(RUSTC_OPTIONS.split(" "));
    let args = args
        .into_iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    rustc_driver::run_compiler(&args, callbacks);
}
