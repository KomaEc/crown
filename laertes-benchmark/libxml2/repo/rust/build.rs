use std::env;
use std::fs::create_dir;
use std::path::Path;

extern crate cc;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(manifest_dir.as_str());
    let lib_dir = cargo_dir.join("lib");
    let variadic_c = cargo_dir.join("../variadic.c");
    let xchecks_c = cargo_dir.join("../xchecks/xchecks.c");
    let libxml2_dir = cargo_dir.join("..");
    let libxml2_include_dir = cargo_dir.join("../include");
    let libvariadic_a = lib_dir.join("libvariadic.a");

    println!("cargo:rustc-link-lib=lzma");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    if !lib_dir.is_dir() {
        create_dir(&lib_dir).unwrap();
    }

    cc::Build::new()
        .flag("-c")
        .file(variadic_c)
        .file(xchecks_c)
        .flag("-DHAVE_CONFIG_H")
        .flag("-D_REENTRANT")
        .flag("-DPIC")
        .flag("-fPIC")
        .flag("-w") // Hide warnings; cc will pass them to cargo annoyingly
        .include(libxml2_dir)
        .include(libxml2_include_dir)
        .out_dir(&lib_dir)
        .compile("variadic");
}
