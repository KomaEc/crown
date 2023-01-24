use std::{collections::HashSet, env, ffi::OsStr, fs, path::PathBuf};

const C2RUST_DEPENDENCIES: &[(&str, &str)] = &[
    ("libc2rust_bitfields", "rlib"),
    (
        "libc2rust_bitfields_derive",
        if cfg!(target_arch = "aarch64") {
            "dylib"
        } else {
            "so"
        },
    ),
    ("liblibc", "rlib"),
    ("libf128_internal", "rlib"),
    ("libf128", "rlib"),
    ("libnum_traits", "rlib"),
];

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let extra_deps_dir = project_dir.join("extra_deps");
    if extra_deps_dir.exists() {
        fs::remove_dir_all(extra_deps_dir.as_path()).unwrap();
    }
    fs::create_dir(extra_deps_dir.as_path()).unwrap();

    let profile = std::env::var("PROFILE").unwrap();
    let deps_path = project_dir.join("target").join(profile).join("deps");

    let mut lib_prepared: HashSet<&str> = HashSet::new();

    for file in fs::read_dir(deps_path.as_path()).expect("dependency build error") {
        let absolute_path = file.unwrap().path();
        let path = absolute_path
            .as_path()
            .strip_prefix(deps_path.as_path())
            .unwrap();
        if let Some(lib_name) = path.file_stem().and_then(|stem| stem.to_str()) {
            let lib_name = lib_name.split("-").next().unwrap();
            for &(lib, lib_type) in C2RUST_DEPENDENCIES {
                if lib_name == lib
                    && path.extension() == Some(OsStr::new(lib_type))
                    && !lib_prepared.contains(&lib)
                {
                    lib_prepared.insert(lib);
                    fs::copy(absolute_path.as_path(), extra_deps_dir.join(path))
                        .expect("failed to prepare dependency");
                }
            }
        }
    }
}
