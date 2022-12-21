mod pattern;

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

    for file_name in user_files.iter() {
        let source = fs::read_to_string(file_name).unwrap();

        let mut replacement = source.clone();

        for pattern in pattern::PATTERNS {
            let re = regex::Regex::new(pattern.pattern).unwrap();
            replacement = re.replace_all(&replacement, pattern.replacer).to_string()
        }

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
