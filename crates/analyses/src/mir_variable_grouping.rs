//! Group MIR locals that correspond to the same source variable
//!
//! This module handles the mapping between MIR locals and source variables,
//! including temporaries that don't have debug info but are copies of source variables.

use rustc_hir::def_id::DefId;
use rustc_index::bit_set::DenseBitSet;
use rustc_middle::{
    mir::{Body, Local, Location, Operand, Place, Rvalue, VarDebugInfoContents, visit::Visitor},
    ty::TyCtxt,
};
use utils::{rustc::RustProgram, rustc_hash::FxHashMap};

/// Group MIR locals by their corresponding source variable names.
/// This includes both locals with debug info and temporaries that are copies.
pub struct SourceVarGroups {
    inner: FxHashMap<DefId, FxHashMap<Local, Vec<Local>>>,
}

impl SourceVarGroups {
    pub fn new(rust_program: &RustProgram) -> Self {
        let mut inner = FxHashMap::default();
        for f in rust_program.functions.iter().copied() {
            let body = &*rust_program
                .tcx
                .mir_drops_elaborated_and_const_checked(f.expect_local())
                .borrow();
            let groups = group_locals_by_source_variable(body, rust_program.tcx);
            // Store groups for function f
            inner.insert(f, groups);
        }
        Self { inner }
    }

    pub fn postprocess_promoted_mut_refs(
        &self,
        promoted_mut_refs: FxHashMap<DefId, DenseBitSet<Local>>,
    ) -> FxHashMap<DefId, DenseBitSet<Local>> {
        // a Local is promoted if all locals in its source variable group are promoted
        // otherwise its promotion is removed
        let mut result = FxHashMap::default();
        for (did, promoted) in promoted_mut_refs {
            let promoted = if let Some(groups) = self.inner.get(&did) {
                let mut new_promoted = DenseBitSet::new_empty(promoted.domain_size());
                for locals in groups.values() {
                    if locals.iter().all(|local| promoted.contains(*local)) {
                        for local in locals {
                            new_promoted.insert(*local);
                        }
                    }
                }
                new_promoted
            } else {
                promoted.clone()
            };
            result.insert(did, promoted);
        }
        result
    }
}

fn group_locals_by_source_variable<'tcx>(
    body: &Body<'tcx>,
    _tcx: TyCtxt<'tcx>,
) -> FxHashMap<Local, Vec<Local>> {
    // First, collect all locals that have direct debug info
    let mut src_local_to_locals: FxHashMap<Local, Vec<Local>> = FxHashMap::default();
    let mut local_to_src_local: FxHashMap<Local, Local> = FxHashMap::default();

    for debug_info in &body.var_debug_info {
        if let VarDebugInfoContents::Place(place) = &debug_info.value {
            if let Some(local) = place.as_local() {
                src_local_to_locals.entry(local).or_default().push(local);
                local_to_src_local.insert(local, local);
            }
        }
    }

    // Now find temporaries that are copies of source variables
    let copy_relationships = find_copy_relationships(body);

    // Propagate source variable names to temporaries
    // Caveat: the order of copy_relationships should be chronological
    for (dest, src) in copy_relationships {
        if let Some(src_local) = local_to_src_local.get(&src).cloned() {
            if !local_to_src_local.contains_key(&dest) {
                src_local_to_locals
                    .entry(src_local.clone())
                    .or_default()
                    .push(dest);
                local_to_src_local.insert(dest, src_local);
            }
        }
    }

    src_local_to_locals
}

/// Find copy relationships between locals (dest = copy src or dest = move src)
fn find_copy_relationships(body: &Body<'_>) -> Vec<(Local, Local)> {
    struct CopyVisitor {
        copies: Vec<(Local, Local)>,
    }

    impl<'tcx> Visitor<'tcx> for CopyVisitor {
        fn visit_assign(
            &mut self,
            place: &Place<'tcx>,
            rvalue: &Rvalue<'tcx>,
            _location: Location,
        ) {
            if let Some(dest_local) = place.as_local() {
                match rvalue {
                    Rvalue::Use(Operand::Copy(src_place) | Operand::Move(src_place)) => {
                        if let Some(src_local) = src_place.as_local() {
                            self.copies.push((dest_local, src_local));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let mut visitor = CopyVisitor { copies: Vec::new() };
    visitor.visit_body(body);
    visitor.copies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_variable_grouping() {
        const PROGRAM: &str = "
        pub struct object {
            pub name: usize,
        }
        unsafe fn json_parse_object() {
            let mut element = 0 as *mut object;
            let mut previous = 0 as *mut object;
            previous = element;
            (*element).name = 0;
            (*previous).name = 0;
        }";

        utils::rustc::run_compiler(PROGRAM, |program| {
            let tcx = program.tcx;
            let f = program.functions[0];
            let body = &*tcx
                .mir_drops_elaborated_and_const_checked(f.expect_local())
                .borrow();

            let groups = group_locals_by_source_variable(body, tcx);

            println!("Source variable groups:");
            for (src_local, locals) in &groups {
                println!("  {:?}: {:?}", src_local, locals);
            }
        });
    }
}
