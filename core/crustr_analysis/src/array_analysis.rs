use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{HasLocalDecls, Local},
    ty::{subst::GenericArgKind, TyCtxt},
};
use rustc_target::abi::VariantIdx;

use crate::FieldDefIdx;

pub mod intra;
#[cfg(test)]
mod test;

fn gather_field_defs(
    tcx: TyCtxt,
    adt_defs: &[DefId],
) -> IndexVec<FieldDefIdx, (usize, VariantIdx, usize)> {
    let mut field_defs = IndexVec::new();
    for (adt_def_idx, &did) in adt_defs.iter().enumerate() {
        let adt_def = tcx.adt_def(did);
        for (variant_idx, variant) in adt_def.variants.iter_enumerated() {
            for (field_idx, _) in variant.fields.iter().enumerate() {
                field_defs.push((adt_def_idx, variant_idx, field_idx));
            }
        }
    }
    field_defs
}

/// This structure should hold info about all struct definitions
/// and local nested pointers in the crate
pub struct CrateSummary<'analysis, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub adt_defs: &'analysis [DefId],
    field_defs: IndexVec<FieldDefIdx, (usize, VariantIdx, usize)>,
    pub bodies: &'analysis [DefId],
    lambda_ctxt: CrateLambdaCtxt,
}

impl<'analysis, 'tcx> CrateSummary<'analysis, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        adt_defs: &'analysis [DefId],
        bodies: &'analysis [DefId],
    ) -> Self {
        let field_defs = gather_field_defs(tcx, adt_defs);
        let lambda_ctxt = CrateLambdaCtxt::initiate(tcx, &field_defs, adt_defs, bodies);
        CrateSummary {
            tcx,
            adt_defs,
            field_defs,
            bodies,
            lambda_ctxt,
        }
    }
}

/// A bidirectional map between constraint variables lambdas and the language constructs
/// we care about
pub struct CrateLambdaCtxt {
    pub lambda_map: IndexVec<Lambda, LambdaData>,
    pub field_def: IndexVec<FieldDefIdx, Vec<Lambda>>,
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
        field_defs: &IndexVec<FieldDefIdx, (usize, VariantIdx, usize)>,
        adt_defs: &[DefId],
        bodies: &[DefId],
    ) -> Self {
        let mut lambda_map = IndexVec::new();

        let field_def = field_defs
            .iter_enumerated()
            .map(|(idx, &(adt_def_idx, variant_idx, field_idx))| {
                let field =
                    &tcx.adt_def(adt_defs[adt_def_idx]).variants[variant_idx].fields[field_idx];
                let ty = tcx.type_of(field.did);
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
                    .map(|(level, _)| {
                        lambda_map.push(LambdaData::FieldDef {
                            field_def: idx,
                            nested_level: level,
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<IndexVec<_, _>>();

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
                            vec![],
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
            field_def,
            body_ctxt,
        }
    }
}

pub enum Constraint {
    /// λ1 = λ2
    Eq(Lambda, Lambda),
    /// λ1 ≤ λ2
    LE(Lambda, Lambda),
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
        field_def: FieldDefIdx,
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
