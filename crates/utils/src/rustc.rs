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

/// Extended version that also contains the AST
pub struct RustProgramWithMappings<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub functions: Vec<DefId>,
    pub structs: Vec<DefId>,
    pub ir_mappings: IrMappings,
}

impl<'tcx> Into<RustProgram<'tcx>> for RustProgramWithMappings<'tcx> {
    fn into(self) -> RustProgram<'tcx> {
        RustProgram {
            tcx: self.tcx,
            functions: self.functions,
            structs: self.structs,
        }
    }
}

use rustc_driver::Callbacks;
use rustc_interface::Config;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::ir_util::IrMappings;
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

pub struct WithRustProgramAndMappings<'a, F> {
    callback: F,
    dir: &'a Path,
    ir_mappings: Option<IrMappings>,
}

impl<'a, F> WithRustProgramAndMappings<'a, F> {
    pub fn new(f: F, dir: &'a Path) -> Self {
        Self {
            callback: f,
            dir,
            ir_mappings: None,
        }
    }
}

impl<F> rustc_driver::Callbacks for WithRustProgramAndMappings<'_, F>
where
    F: FnMut(RustProgramWithMappings),
{
    fn after_expansion<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        self.ir_mappings = Some(IrMappings::new(self.dir, tcx));

        rustc_driver::Compilation::Continue
    }

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

        let ir_mappings = self
            .ir_mappings
            .take()
            .expect("IR mappings should have been captured");

        (self.callback)(RustProgramWithMappings {
            tcx,
            functions,
            structs,
            ir_mappings,
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

pub trait OptLevel {
    const CMDLINE: &[&str];
}
pub enum Opt3 {}
impl OptLevel for Opt3 {
    const CMDLINE: &[&str] = &["-C", "opt-level=3"];
}
pub enum NoOpt {}
impl OptLevel for NoOpt {
    const CMDLINE: &[&str] = &[];
}

pub fn run_compiler_with_opt_level<P, F, O>(program: P, callbacks: F)
where
    P: Into<SourceCode>,
    F: FnMut(RustProgram) + Send,
    O: OptLevel,
{
    let mut callbacks = WithRustProgram::new(callbacks);
    let (path, callbacks): (_, &mut (dyn Callbacks + Send)) = match program.into() {
        SourceCode::Text(program) => (
            Path::new("lib.rs").to_path_buf(),
            &mut TextCompiler(&mut callbacks, program),
        ),
        SourceCode::AbsolutePath(path_buf) => (path_buf, &mut callbacks),
        SourceCode::Libtree => (
            Path::new("lib.rs").to_path_buf(),
            &mut LibtreeCompiler(&mut callbacks),
        ),
    };
    let args = compiler_args::<O>(&path);
    rustc_driver::run_compiler(&args, callbacks);
}

pub fn run_compiler_without_opt<P, F>(program: P, callbacks: F)
where
    P: Into<SourceCode>,
    F: FnMut(RustProgram) + Send,
{
    run_compiler_with_opt_level::<_, _, NoOpt>(program, callbacks);
}

pub fn run_compiler<P, F>(program: P, callbacks: F)
where
    P: Into<SourceCode>,
    F: FnMut(RustProgram) + Send,
{
    run_compiler_with_opt_level::<_, _, Opt3>(program, callbacks);
}

pub fn run_compiler_with_mappings<P, F>(program: P, callbacks: F)
where
    P: Into<SourceCode>,
    F: FnMut(RustProgramWithMappings) + Send,
{
    let program: SourceCode = program.into();
    let path = match &program {
        SourceCode::Text(_) => PathBuf::from("lib.rs"),
        SourceCode::AbsolutePath(p) => p.clone(),
        SourceCode::Libtree => PathBuf::from("lib.rs"),
    };
    let mut callbacks =
        WithRustProgramAndMappings::new(callbacks, path.parent().unwrap_or(Path::new(".")));
    let callbacks: &mut (dyn Callbacks + Send) = match program {
        SourceCode::Text(program) => &mut TextCompiler(&mut callbacks, program),
        SourceCode::AbsolutePath(_) => &mut callbacks,
        SourceCode::Libtree => &mut LibtreeCompiler(&mut callbacks),
    };
    let args = compiler_args::<Opt3>(&path.to_path_buf());
    rustc_driver::run_compiler(&args, callbacks);
}

/// Builds rustc argument list based on prebuilt dependencies.
fn compiler_args<O: OptLevel>(input_path: &Path) -> Vec<String> {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let extra_deps_dir = crate_dir
        .ancestors()
        .nth(2)
        .expect("crate_dir has no grandparent")
        .join("extra_deps");

    let mut args = vec![
        "rustc",
        input_path.to_str().unwrap(),
        "--crate-type=lib",
        "--cap-lints",
        "allow",
        "-Awarnings",
        "-L",
        extra_deps_dir.to_str().unwrap(),
    ];

    args.extend_from_slice(O::CMDLINE);

    let mut args: Vec<String> = args.into_iter().map(|s| s.to_string()).collect();

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
