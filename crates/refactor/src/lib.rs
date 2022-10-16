#![feature(rustc_private)]
#![feature(array_windows)]
#![feature(box_patterns)]
#![feature(split_array)]
#![feature(let_else)]

// use analysis::{ownership::{OwnershipSchemes, self}, type_qualifier::flow_insensitive::{mutability::{MutabilityResult, self}, fatness::{self, FatnessResult}}};

extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_passes;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;


// pub struct PointerQualifier {
//     mutability: mutability::Mutability,
//     fatness: fatness::Fatness,
//     ownership: ownership::Ownership,
//     nullability: ()
// }

// pub struct RefactorCtxt<O: for<'analysis> OwnershipSchemes<'analysis>> {
//     ownership_schemes: O,
//     mutability_result: MutabilityResult,
//     fatness_result: FatnessResult,
//     nullability_result: (),
// }
