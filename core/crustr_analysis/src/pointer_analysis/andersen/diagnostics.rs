//! Andersen's pointer analysis diagnostics.

use rustc_errors::Applicability;
use rustc_middle::mir::VarDebugInfoContents;

use crate::{pointer_analysis::andersen::AndersenResult, ty_ext::TyExt};

impl<'ar, 'tcx> AndersenResult<'ar, 'tcx> {
    pub fn report_ptr_alias(&self) {
        let all_user_vars = self
            .ptr_ctxt
            .bodies
            .iter()
            .flat_map(|body| {
                body.var_debug_info.clone().into_iter().filter_map(|var| {
                    if let VarDebugInfoContents::Place(place) = var.value {
                        let ty = place.ty(&body.local_decls, self.ptr_ctxt.tcx()).ty;
                        if ty.is_ptr_of_concerned() {
                            return Some((var, body.source.def_id()));
                        }
                    } else {
                        log::warn!("Andersen resultck: ignoring constant!");
                    }
                    None
                })
            })
            .collect::<Vec<_>>();

        let mut iter = all_user_vars.iter();
        loop {
            match iter.next() {
                Some(&(ref var_p, body_def_id_of_p)) => {
                    let p = match var_p.value {
                        VarDebugInfoContents::Place(place) => place,
                        VarDebugInfoContents::Const(_) => continue,
                    };
                    // log::trace!("Checking alias for variable {}", var_p.name);
                    for &(ref var_q, body_def_id_of_q) in iter.clone() {
                        let q = match var_q.value {
                            VarDebugInfoContents::Place(place) => place,
                            VarDebugInfoContents::Const(_) => continue,
                        };

                        let node_p = self
                            .ptr_ctxt
                            .lookup_local(body_def_id_of_p.as_local().unwrap(), p.local)
                            .expect(&format!(
                                "var {} at {:?} must have been generated for Andersen",
                                var_p.name, var_p.source_info
                            ));
                        let node_q = self
                            .ptr_ctxt
                            .lookup_local(body_def_id_of_q.as_local().unwrap(), q.local)
                            .expect(&format!(
                                "var {} at {:?} must have been generated for Andersen",
                                var_q.name, var_q.source_info
                            ));

                        if self.pts_graph.alias(node_p, node_q) {
                            // log::debug!("Found alias! {} and {} [ {:?} and {:?} ]", var_p.name, var_q.name, p, q);

                            let mut err = self
                                .ptr_ctxt
                                .tcx()
                                .sess
                                .struct_span_err(var_p.source_info.span, "Alias found!");
                            err.span_label(var_p.source_info.span, "First pointer found here");
                            err.span_label(var_q.source_info.span, "Second pointer found here");
                            if let Ok(snippet) = self
                                .ptr_ctxt
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
