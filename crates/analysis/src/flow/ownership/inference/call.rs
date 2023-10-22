use rustc_middle::mir::{Location, Operand, Place};
use rustc_span::def_id::DefId;

use super::{ownership_tokens, Intraprocedural};
use crate::flow::ownership::{
    constraint::{Constraint, Database, StorageMode},
    Param,
};

impl<'analysis, 'tcx, Mode, DB> Intraprocedural<'analysis, 'tcx, Mode, DB>
where
    Mode: StorageMode,
    DB: Database<Mode>,
{
    pub fn call(
        &mut self,
        callee: DefId,
        args: &Vec<Operand<'tcx>>,
        destination: &Place<'tcx>,
        location: Location,
    ) {
        let callee_sig = &self.ctxt.fn_sigs[&callee];
        let min_k_limit = std::cmp::min(self.k_limit, callee_sig.k_limit);
        if let Some(dest) = self.path(destination, location) {
            let dest_ty = destination.ty(self.body, self.tcx).ty;
            let dest = self
                .expand(&dest)
                .transpose()
                .map(|path| ownership_tokens(&path, min_k_limit, self.ctxt.access_paths, dest_ty));
            for x in dest.r#use {
                self.ctxt.database.add(
                    Constraint::Assume { x, sign: false },
                    &mut self.ctxt.storage,
                );
            }
            let output = self
                .ctxt
                .access_paths
                .patch_up(
                    min_k_limit,
                    self.ctxt.access_paths.max_k_limit() - min_k_limit,
                    dest_ty,
                )
                .map(|offset| callee_sig.output + offset);
            for (dest, output) in dest.def.zip(output) {
                self.ctxt.database.add(
                    Constraint::Equal { x: dest, y: output },
                    &mut self.ctxt.storage,
                );
            }
        }

        for (arg, param) in args.iter().zip(callee_sig.inputs.iter().copied()) {
            let arg_ty = arg.ty(self.body, self.tcx);
            let Some(arg) = arg.place().and_then(|place| self.path(&place, location)) else {
                continue;
            };
            let arg = self
                .expand(&arg)
                .transpose()
                .map(|path| ownership_tokens(&path, min_k_limit, self.ctxt.access_paths, arg_ty));

            match param {
                Param::Normal(param) => {
                    let param = self
                        .ctxt
                        .access_paths
                        .patch_up(
                            min_k_limit,
                            self.ctxt.access_paths.max_k_limit() - min_k_limit,
                            arg_ty,
                        )
                        .map(|offset| param + offset);

                    for (arg_use, arg_def, param) in itertools::izip!(arg.r#use, arg.def, param) {
                        self.ctxt.database.add(
                            Constraint::Linear {
                                x: param,
                                y: arg_def,
                                z: arg_use,
                            },
                            &mut self.ctxt.storage,
                        );
                    }
                }
                Param::Output(param) => {
                    let param = param.map(|param| {
                        self.ctxt
                            .access_paths
                            .patch_up(
                                min_k_limit,
                                self.ctxt.access_paths.max_k_limit() - min_k_limit,
                                arg_ty,
                            )
                            .map(move |offset| param + offset)
                    });

                    for (arg_use, arg_def, param_use, param_def) in
                        itertools::izip!(arg.r#use, arg.def, param.r#use, param.def)
                    {
                        self.ctxt.database.add(
                            Constraint::Equal {
                                x: arg_use,
                                y: param_use,
                            },
                            &mut self.ctxt.storage,
                        );
                        self.ctxt.database.add(
                            Constraint::Equal {
                                x: arg_def,
                                y: param_def,
                            },
                            &mut self.ctxt.storage,
                        );
                    }
                }
            }
        }
    }
}
