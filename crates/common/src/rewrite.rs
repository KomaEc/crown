use std::{collections::HashMap, fs, io, io::Write};

use clap::ValueEnum;
use rustc_middle::ty::TyCtxt;
use rustc_span::{FileName, Span};
use rustfix::{LinePosition, LineRange, Replacement, Snippet, Solution, Suggestion};
// use std::path::PathBuf;

pub trait Rewrite {
    #[inline]
    fn erase(&mut self, tcx: TyCtxt, span: Span) {
        self.replace_with_msg(tcx, span, format!("erase @ {:?}", span), "".to_owned())
    }

    #[inline]
    fn replace(&mut self, tcx: TyCtxt, span: Span, replacement: String) {
        self.replace_with_msg(
            tcx,
            span,
            format!("replace @ {:?} with '{replacement}'", span),
            replacement,
        )
    }

    fn replace_with_msg(&mut self, tcx: TyCtxt, span: Span, message: String, replacement: String);

    fn write(self, mode: RewriteMode);
}

// Does not work
// impl Rewrite for Option<Suggestion> {
//     fn make_suggestion(&mut self, tcx: TyCtxt, span: Span, message: String, replacement: String) {
//         let snippet = get_snippet(tcx, span);
//         let solution = Solution {
//             message,
//             replacements: vec![Replacement {
//                 snippet: snippet.clone(),
//                 replacement,
//             }],
//         };
//         if let Some(suggestion) = self {
//             suggestion.solutions.push(solution)
//         } else {
//             *self = Some(Suggestion {
//                 message: snippet.file_name,
//                 snippets: vec![],
//                 solutions: vec![solution],
//             });
//         }
//     }

//     fn write(self, mode: RewriteMode) {
//         let mut files = HashMap::new();
//         for suggestion in self {
//             let file = suggestion.solutions[0].replacements[0]
//                 .snippet
//                 .file_name
//                 .clone();
//             files.entry(file).or_insert_with(Vec::new).push(suggestion);
//         }

//         for (source_file, suggestions) in &files {
//             let source = fs::read_to_string(source_file).unwrap();
//             let mut fix = rustfix::CodeFix::new(&source);
//             for suggestion in suggestions.iter().rev() {
//                 if let Err(e) = fix.apply(suggestion) {
//                     eprintln!("Failed to apply suggestion to {}: {}", source_file, e);
//                 }
//             }
//             let fixes = fix.finish().unwrap();
//             match mode {
//                 RewriteMode::InPlace => fs::write(source_file, fixes).unwrap(),
//                 RewriteMode::Print => io::stdout().write_all(fixes.as_ref()).unwrap(),
//                 RewriteMode::Alongside => todo!(),
//             }
//         }
//     }
// }

impl Rewrite for Vec<Suggestion> {
    fn replace_with_msg(&mut self, tcx: TyCtxt, span: Span, message: String, replacement: String) {
        let suggestion = make_suggestion(tcx, span, message, replacement);
        self.push(suggestion);
    }

    fn write(self, mode: RewriteMode) {
        let mut files = HashMap::new();
        for suggestion in self {
            let file = suggestion.solutions[0].replacements[0]
                .snippet
                .file_name
                .clone();
            files.entry(file).or_insert_with(Vec::new).push(suggestion);
        }

        for (source_file, suggestions) in &files {
            let source = fs::read_to_string(source_file).unwrap();
            let mut fix = rustfix::CodeFix::new(&source);
            for suggestion in suggestions.iter().rev() {
                if let Err(e) = fix.apply(suggestion) {
                    eprintln!(
                        "Failed to apply {} to {}: {}",
                        suggestion.message, source_file, e
                    );
                }
            }
            let fixes = fix.finish().unwrap();
            match mode {
                RewriteMode::InPlace => fs::write(source_file, fixes).unwrap(),
                RewriteMode::Print => io::stdout().write_all(fixes.as_ref()).unwrap(),
                RewriteMode::Diff => {
                    similar::TextDiff::from_lines(&source, &fixes)
                        .unified_diff()
                        .header("original", "rewritten")
                        .to_writer(io::stdout())
                        .unwrap();
                }
                RewriteMode::Noop => {}
            }
        }
    }
}

pub fn get_snippet(tcx: TyCtxt, span: Span) -> Snippet {
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
        .unwrap_or_else(|| panic!("failed to open source file {:?}", file_name));

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
    Snippet {
        file_name: file_name.prefer_local().to_string(),
        line_range,
        range: (lo_offset as usize)..(hi_offset as usize),
        text: (
            "".into(),
            source_map.span_to_snippet(span).unwrap(),
            "".into(),
        ),
    }
}

pub fn make_suggestion(
    tcx: TyCtxt,
    span: Span,
    message: String,
    replacement: String,
) -> Suggestion {
    let snippet = get_snippet(tcx, span);
    Suggestion {
        message: message.clone(), //: snippet.file_name.clone(),
        snippets: vec![snippet.clone()],
        solutions: vec![Solution {
            message,
            replacements: vec![Replacement {
                snippet,
                replacement,
            }],
        }],
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum RewriteMode {
    InPlace,
    Print,
    Diff,
    Noop,
}

impl std::fmt::Display for RewriteMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            RewriteMode::InPlace => write!(f, "in-place"),
            RewriteMode::Print => write!(f, "print"),
            RewriteMode::Diff => write!(f, "diff"),
            RewriteMode::Noop => write!(f, "noop"),
        }
    }
}
