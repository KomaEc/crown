use std::{
    collections::HashSet,
    fs,
    io::{self, Write},
};

use common::rewrite::{get_snippet, RewriteMode};
use rustc_middle::ty::TyCtxt;

use crate::owner_items;

pub fn fold_let_ref_mut(tcx: TyCtxt, mode: RewriteMode) {
    // awkward but whatever
    let user_files = owner_items(tcx)
        .map(|item| {
            let span = item.span;
            let snippet = get_snippet(tcx, span);
            snippet.file_name
        })
        .collect::<HashSet<_>>();

    let pattern1 = concat!(
        r"let ref mut fresh(?P<version1>[0-9]+)[\s|\n]*=[\s|\n]*(?P<lhs>[^;]+);",
        r"[\s|\n]*\*fresh(?P<version2>[0-9]+)[\s|\n]*=[\s|\n]*(?P<rhs>[^;]+);"
    );

    let re1 = regex::Regex::new(pattern1).unwrap();

    let pattern2 = concat!(
        r"let ref mut fresh(?P<version1>[0-9]+)[\s|\n]*=[\s|\n]*(?P<lhs>[^;]+);[\s|\n]*",
        r"let fresh(?P<version2>[0-9]+)[\s|\n]*=[\s|\n]*\*fresh(?P<version3>[0-9]+)[^;]*;[\s|\n]*",
        r"(?P<incr>\*fresh(?P<version4>[0-9]+)[\s|\n]*=[\s|\n]*\(\*fresh(?P<version5>[0-9]+)\)\.wrapping_add\(1\)[\s|\n]*;)"
    );

    let re2 = regex::Regex::new(pattern2).unwrap();

    for file_name in user_files.iter() {
        let source = fs::read_to_string(file_name).unwrap();
        let replacement = re1
            .replace_all(&source, |caps: &regex::Captures| {
                let original = &caps[0];
                let version1 = &caps["version1"];
                let version2 = &caps["version2"];

                if version1 != version2 {
                    return original.to_owned();
                }

                let expr_fresh = r"\*fresh".to_owned() + version1;

                let lhs = &caps["lhs"];
                let rhs = &caps["rhs"];

                let rhs = regex::Regex::new(&expr_fresh)
                    .unwrap()
                    .replace_all(rhs, lhs);

                lhs.to_owned() + " = " + &rhs + ";"
            })
            .to_string();

        let replacement = re2
            .replace_all(&replacement, |caps: &regex::Captures| {
                let original = &caps[0];
                let version1 = &caps["version1"];
                let version2 = &caps["version2"];
                let version3 = &caps["version3"];
                let version4 = &caps["version4"];
                let version5 = &caps["version5"];

                if !(version1 == version3
                    && version1 == version4
                    && version1 == version5
                    && version1 != version2)
                {
                    return original.to_owned();
                }

                let expr_fresh = r"\*fresh".to_owned() + version1;

                let lhs = &caps["lhs"];
                let incr = &caps["incr"];

                let incr = regex::Regex::new(&expr_fresh)
                    .unwrap()
                    .replace_all(incr, lhs);

                r"let fresh".to_owned() + version2 + " = " + lhs + ";" + &incr
            })
            .to_string();

        match mode {
            RewriteMode::InPlace => fs::write(file_name, replacement).unwrap(),
            RewriteMode::Print => io::stdout().write_all(replacement.as_ref()).unwrap(),
            RewriteMode::Diff => {
                similar::TextDiff::from_lines(&source, &replacement)
                    .unified_diff()
                    .header("original", "rewritten")
                    .to_writer(io::stdout())
                    .unwrap();
            }
            RewriteMode::Noop => {}
        }
    }
}
