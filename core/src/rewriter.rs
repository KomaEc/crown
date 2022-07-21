use clap::ArgEnum;
use rustc_middle::ty::TyCtxt;
use rustc_span::{FileName, Span};
pub use rustfix;
use rustfix::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum RewriteMode {
    InPlace,
    Print,
    Alongside,
}

#[derive(Default)]
pub struct Rewriter {
    /// source_file -> a set of edit suggestions
    suggestions: HashMap<PathBuf, Vec<Suggestion>>,
}

impl Rewriter {
    pub fn make_suggestion<'tcx>(
        &mut self,
        tcx: TyCtxt<'tcx>,
        span: Span,
        message: String,
        replacement: String,
    ) {
        let source_map = tcx.sess.source_map();
        let file_name = source_map.span_to_filename(span);
        if !matches!(file_name, FileName::Real(_)) {
            eprintln!(
                "rewriting in file {:?} at {:?} is not supported",
                file_name, span
            );
            std::process::exit(1)
        }

        let file = source_map
            .get_source_file(&file_name)
            .expect(&format!("failed to open source file {:?}", file_name));

        // get the char offsets within the file
        let lo_offset = file.original_relative_byte_pos(span.lo()).0;
        let hi_offset = file.original_relative_byte_pos(span.hi()).0;
        // get the line and the column numbers
        let lo = file.lookup_file_pos_with_col_display(span.lo());
        let hi = file.lookup_file_pos_with_col_display(span.hi());
        let line_range = LineRange {
            start: LinePosition {
                line: lo.0,
                column: lo.2,
            },
            end: LinePosition {
                line: hi.0,
                column: hi.2,
            },
        };
        let snippet = Snippet {
            file_name: file_name.prefer_local().to_string(),
            line_range,
            range: (lo_offset as usize)..(hi_offset as usize),
            text: (
                "".into(),
                source_map.span_to_snippet(span).unwrap(),
                "".into(),
            ),
        };
        let suggestion = Suggestion {
            message: snippet.file_name.clone(),
            snippets: vec![snippet.clone()],
            solutions: vec![Solution {
                message,
                replacements: vec![Replacement {
                    snippet,
                    replacement,
                }],
            }],
        };
        self.suggestions
            .entry(PathBuf::from(file_name.prefer_local().to_string()))
            .or_default()
            .push(suggestion);
    }

    pub fn write(self, mode: RewriteMode) {
        for (file, suggestions) in self.suggestions {
            use std::fs;
            use std::fs::File;
            use std::io::stdout;
            let code = fs::read_to_string(file.as_path()).expect("source code read failed");
            let rewrited =
                apply_suggestions(&code, &suggestions).expect("apply suggestions failed");
            let mut out: Box<dyn std::io::Write> = match mode {
                RewriteMode::InPlace => Box::new(File::create(file).expect("cannot open file")),
                RewriteMode::Print => Box::new(stdout()),
                RewriteMode::Alongside => {
                    todo!()
                }
            };
            write!(out, "{}", rewrited).expect("write failed");
            // fs::write(file, rewrited).expect("overwrite to file failed");
        }
    }
}
