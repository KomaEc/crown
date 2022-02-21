use std::fmt::Display;

use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{HasLocalDecls, Local},
    ty::{subst::GenericArgKind, TyCtxt},
};
use rustc_target::abi::VariantIdx;

pub mod intra;
#[cfg(test)]
mod test;

/// This structure should hold info about all struct definitions
/// and local nested pointers in the crate
pub struct CrateSummary<'analysis, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub bodies: &'analysis [DefId],
    lambda_ctxt: CrateLambdaCtxt,
}

impl<'analysis, 'tcx> CrateSummary<'analysis, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        adt_defs: &'analysis [DefId],
        bodies: &'analysis [DefId],
    ) -> Self {
        // let field_defs = gather_field_defs(tcx, adt_defs);
        let lambda_ctxt = CrateLambdaCtxt::initiate(tcx, adt_defs, bodies);
        CrateSummary {
            tcx,
            bodies,
            lambda_ctxt,
        }
    }

    pub fn debug(&self) {
        log::debug!("Initialising crate summary");
        const INDENT: &str = "   in f, ";
        for (&adt_did, x) in &self.lambda_ctxt.field_defs {
            for (variant_idx, y) in x.iter_enumerated() {
                for (field_idx, z) in y.iter().enumerate() {
                    let adt_def = self.tcx.adt_def(adt_did);
                    let field_def = &adt_def.variants[variant_idx].fields[field_idx];
                    let field_def_str = format!("{}.{}", self.tcx.type_of(adt_did), field_def.name);
                    log::debug!(
                        "for field {}: {}:",
                        field_def_str,
                        self.tcx.type_of(field_def.did)
                    );
                    for (idx, lambda) in z.iter().enumerate() {
                        log::debug!("{:*<1$}{2} ==> {3:?}", "", idx, field_def_str, lambda)
                    }
                }
            }
        }

        for (&body_did, ctxt) in std::iter::zip(self.bodies, &self.lambda_ctxt.body_ctxt) {
            let body = self.tcx.optimized_mir(body_did);
            log::debug!(
                "for function {}:",
                self.tcx.def_path_debug_str(body.source.def_id())
            );
            for (local, x) in ctxt.local_nested.iter_enumerated() {
                for (idx, lambda) in x.iter().enumerate() {
                    log::debug!(
                        "{}{:*<2$}{3:?} ==> {4:?}",
                        INDENT,
                        "",
                        idx + 1,
                        local,
                        lambda
                    )
                }
            }
            for (local, y) in ctxt.local.iter_enumerated() {
                if body.local_decls()[local].ty.is_any_ptr() {
                    assert_eq!(y.len(), 1);
                    log::debug!("{}{:?}^0 ==> {:?}", INDENT, local, y[0])
                } else {
                    assert!(y.is_empty())
                }
            }
        }
    }
}

/// A bidirectional map between constraint variables lambdas and the language constructs
/// we care about
pub struct CrateLambdaCtxt {
    pub lambda_map: IndexVec<Lambda, LambdaData>,
    /// did of adt_def -> variant_idx -> field_idx -> nested_level -> lambda
    pub field_defs: FxHashMap<DefId, IndexVec<VariantIdx, Vec<Vec<Lambda>>>>,
    pub body_ctxt: Vec<LambdaCtxt>,
}

pub struct LambdaCtxt {
    pub local: IndexVec<Local, Vec<Lambda>>,
    pub local_nested: IndexVec<Local, Vec<Lambda>>,
}

impl From<(IndexVec<Local, Vec<Lambda>>, IndexVec<Local, Vec<Lambda>>)> for LambdaCtxt {
    fn from(
        (local, local_nested): (IndexVec<Local, Vec<Lambda>>, IndexVec<Local, Vec<Lambda>>),
    ) -> Self {
        LambdaCtxt {
            local,
            local_nested,
        }
    }
}

impl CrateLambdaCtxt {
    pub fn initiate(
        tcx: TyCtxt,
        // field_defs: &IndexVec<FieldDefIdx, (usize, VariantIdx, usize)>,
        adt_defs: &[DefId],
        bodies: &[DefId],
    ) -> Self {
        let mut lambda_map = IndexVec::new();

        let field_defs = adt_defs
            .iter()
            .map(|&did| {
                (
                    did,
                    tcx.adt_def(did)
                        .variants
                        .iter_enumerated()
                        .map(|(variant_idx, variant_def)| {
                            variant_def
                                .fields
                                .iter()
                                .enumerate()
                                .map(|(field_idx, field_def)| {
                                    let ty = tcx.type_of(field_def.did);
                                    ty.walk()
                                        .filter_map(|generic_arg| {
                                            if let GenericArgKind::Type(ty) = generic_arg.unpack() {
                                                Some(ty)
                                            } else {
                                                None
                                            }
                                        })
                                        .take_while(|ty| ty.is_any_ptr() && !ty.is_fn_ptr())
                                        .enumerate()
                                        .map(|(nested_level, _)| {
                                            lambda_map.push(LambdaData::FieldDef {
                                                adt_def: did,
                                                variant_idx,
                                                field_idx,
                                                nested_level,
                                            })
                                        })
                                        .collect::<Vec<_>>()
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<IndexVec<_, _>>(),
                )
            })
            .collect::<FxHashMap<_, _>>();

        let body_ctxt = bodies
            .iter()
            .enumerate()
            .map(|(body_idx, &did)| {
                let body = tcx.optimized_mir(did);
                body.local_decls()
                    .iter_enumerated()
                    .map(|(local, local_decl)| {
                        let ty = local_decl.ty;
                        (
                            // vec![],
                            ty.is_any_ptr()
                                .then(|| {
                                    let lambda = lambda_map.push(LambdaData::Local {
                                        body: body_idx,
                                        base: local,
                                        ssa_idx: 0,
                                    });
                                    vec![lambda]
                                })
                                .unwrap_or_else(|| vec![]),
                            ty.walk()
                                .filter_map(|generic_arg| {
                                    if let GenericArgKind::Type(ty) = generic_arg.unpack() {
                                        Some(ty)
                                    } else {
                                        None
                                    }
                                })
                                .take_while(|ty| ty.is_any_ptr() && !ty.is_fn_ptr())
                                .skip(1)
                                .enumerate()
                                .map(|(level, _)| {
                                    lambda_map.push(LambdaData::LocalNested {
                                        body: body_idx,
                                        base: local,
                                        nested_level: level,
                                    })
                                })
                                .collect::<Vec<_>>(),
                        )
                    })
                    .unzip()
                    .into()
            })
            .collect::<Vec<_>>();

        CrateLambdaCtxt {
            lambda_map,
            field_defs,
            body_ctxt,
        }
    }
}

/// λ1 ≤ λ2
pub struct Constraint(Lambda, Lambda);

impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?} ≤ {:?}", self.0, self.1))
    }
}

/// The language constructs that a constraint variable λ corresponds to
pub enum LambdaData {
    /// A SSA scalar variable
    Local {
        body: usize,
        base: Local,
        ssa_idx: usize,
    },
    /// field definition
    FieldDef {
        adt_def: DefId,
        variant_idx: VariantIdx,
        field_idx: usize,
        nested_level: usize,
    },
    /// A local nested pointer type.
    /// For example, if a local `_1` has type `*mut *mut *mut i32`, then
    /// we should have entries for `*_1` and `**_1`
    LocalNested {
        body: usize,
        base: Local,
        nested_level: usize,
    },
}

rustc_index::newtype_index! {
    pub struct ConstraintIdx {
        DEBUG_FORMAT = "constraint ({})"
    }
}

rustc_index::newtype_index! {
    /// Constraint variables for array analysis
    pub struct Lambda {
        DEBUG_FORMAT = "λ_({})"
    }
}
