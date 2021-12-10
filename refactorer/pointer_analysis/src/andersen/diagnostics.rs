//! Andersen's pointer analysis diagnostics.

use rustc_errors::Applicability;
use rustc_middle::mir::VarDebugInfoContents;

use crate::andersen::AndersenResult;

impl<'ar, 'tcx> AndersenResult<'ar, 'tcx> {
    pub fn report_ptr_alias(&self) {
        let all_user_vars = self
            .aa_ctxt
            .all_functions
            .iter()
            .flat_map(|body| {
                body.var_debug_info
                    .clone()
                    .into_iter()
                    .map(|var| (var, body.source.def_id()))
            })
            .collect::<Vec<_>>();

        let mut iter = all_user_vars.iter();
        loop {
            match iter.next() {
                Some(&(ref var_p, did_p)) => {
                    let p = match var_p.value {
                        VarDebugInfoContents::Place(place) => place,
                        VarDebugInfoContents::Const(_) => continue,
                    };
                    // log::trace!("Checking alias for variable {}", var_p.name);
                    for &(ref var_q, did_q) in iter.clone() {
                        let q = match var_q.value {
                            VarDebugInfoContents::Place(place) => place,
                            VarDebugInfoContents::Const(_) => continue,
                        };

                        let node_p = self
                            .aa_ctxt
                            .lookup_local(did_p.as_local().unwrap(), p.local)
                            .unwrap();
                        let node_q = self
                            .aa_ctxt
                            .lookup_local(did_q.as_local().unwrap(), q.local)
                            .unwrap();

                        if self.pts_graph.alias(node_p, node_q) {
                            // log::debug!("Found alias! {} and {} [ {:?} and {:?} ]", var_p.name, var_q.name, p, q);

                            let mut err = self
                                .aa_ctxt
                                .tcx()
                                .sess
                                .struct_span_err(var_p.source_info.span, "Alias found!");
                            err.span_label(var_p.source_info.span, "First pointer found here");
                            err.span_label(var_q.source_info.span, "Second pointer found here");
                            if let Ok(snippet) = self
                                .aa_ctxt
                                .tcx()
                                .sess
                                .source_map()
                                .span_to_snippet(var_p.source_info.span)
                            {
                                // Use the snippet to generate a suggested fix
                                err.span_suggestion(
                                    var_p.source_info.span,
                                    "try using a shared reference here",
                                    format!("{}: *const _", snippet),
                                    Applicability::MaybeIncorrect,
                                );
                            }

                            err.emit()
                        }
                    }
                }
                None => break,
            }
        }
    }
}
