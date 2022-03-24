use rustc_hash::FxHashMap;
use rustc_hir::definitions::DefPathData;
use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::{
    mir::{Constant, ConstantKind, Local, LocalDecl, LocalDecls, Rvalue, TerminatorKind},
    ty::{TyCtxt, TyKind},
};
use rustc_mir_dataflow::{lattice::FlatSet, Analysis, AnalysisDomain, JoinSemiLattice};

use crate::ty_ext::TyExt;

pub struct MustNullAnalysis<'me, 'tcx> {
    tcx: TyCtxt<'tcx>,
    local_decls: &'me LocalDecls<'tcx>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Product<L1: JoinSemiLattice, L2: JoinSemiLattice>(L1, L2);

impl<L1: JoinSemiLattice, L2: JoinSemiLattice> JoinSemiLattice for Product<L1, L2> {
    fn join(&mut self, other: &Self) -> bool {
        let left = self.0.join(&other.0);
        let right = self.1.join(&other.1);
        left || right
    }
}

impl<'me, 'tcx> AnalysisDomain<'tcx> for MustNullAnalysis<'me, 'tcx> {
    type Domain = Product<IndexVec<Local, FlatSet<Local>>, IndexVec<Local, bool>>;

    type Direction = rustc_mir_dataflow::Forward;

    const NAME: &'static str = "Must be Null Analysis";

    fn bottom_value(&self, body: &rustc_middle::mir::Body<'tcx>) -> Self::Domain {
        // all pointers are assumed to be non null
        // IndexVec::from_elem(false, &body.local_decls)
        let from_null_test = IndexVec::from_elem(FlatSet::Bottom, &body.local_decls);
        let must_be_null = IndexVec::from_elem(false, &body.local_decls);
        Product(from_null_test, must_be_null)
    }

    fn initialize_start_block(
        &self,
        _body: &rustc_middle::mir::Body<'tcx>,
        _state: &mut Self::Domain,
    ) {
    }
}

impl<'me, 'tcx> Analysis<'tcx> for MustNullAnalysis<'me, 'tcx> {
    fn apply_statement_effect(
        &self,
        state: &mut Self::Domain,
        statement: &rustc_middle::mir::Statement<'tcx>,
        location: rustc_middle::mir::Location,
    ) {
        match &statement.kind {
            rustc_middle::mir::StatementKind::Assign(box (
                place,
                Rvalue::Use(operand) | Rvalue::Cast(_, operand, _),
            )) => {
                if let Some(lhs) = place.as_local() {
                    if self.local_decls[lhs].ty.is_ptr_but_not_fn_ptr() {
                        match operand {
                            rustc_middle::mir::Operand::Copy(rhs)
                            | rustc_middle::mir::Operand::Move(rhs) => {
                                if let Some(rhs) = rhs.as_local() {
                                    state.1[lhs] = state.1[rhs];
                                }
                            }
                            rustc_middle::mir::Operand::Constant(box c) => {
                                if let Some(scalar_int) = c.literal.try_to_scalar_int() {
                                    if scalar_int.is_null() {
                                        state.1[lhs] = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn apply_terminator_effect(
        &self,
        state: &mut Self::Domain,
        terminator: &rustc_middle::mir::Terminator<'tcx>,
        location: rustc_middle::mir::Location,
    ) {
        if let TerminatorKind::Call {
            func,
            destination,
            args,
            ..
        } = &terminator.kind
        {
            let constant = match func.constant() {
                Some(Constant {
                    literal: ConstantKind::Ty(v),
                    ..
                }) => v,
                _ => return,
            };
            let def_id = match constant.ty.kind() {
                TyKind::FnDef(def_id, _) => def_id,
                _ => return,
            };
            let def_path = self.tcx.def_path(*def_id);
            // ::core ...
            let in_core = self.tcx.crate_name(def_path.krate).as_str() == "core";
            // ::ptr ...
            let in_ptr = def_path
                .data
                .get(0)
                .map(|d| match d.data {
                    DefPathData::TypeNs(s) if s.as_str() == "ptr" => true,
                    _ => false,
                })
                .unwrap_or(false);
            // ::{const_ptr, mut_ptr}::{impl} ...
            // ::is_null
            let is_is_null = def_path
                .data
                .get(3)
                .map(|d| match d.data {
                    DefPathData::ValueNs(s) if s.as_str() == "is_null" => true,
                    _ => false,
                })
                .unwrap_or(false);
            if in_core && in_ptr && is_is_null {
                let (destination, _) = destination.unwrap();

                if let Some(lhs) = destination.as_local() {
                    if let Some(ptr) = args[0].place() {
                        if let Some(ptr) = ptr.as_local() {
                            state.0[lhs] = FlatSet::Elem(ptr);
                        }
                    }
                }

                return;
            }
        }
    }

    fn apply_call_return_effect(
        &self,
        _state: &mut Self::Domain,
        _block: rustc_middle::mir::BasicBlock,
        _return_places: rustc_mir_dataflow::CallReturnPlaces<'_, 'tcx>,
    ) {
    }

    /*

    fn apply_switch_int_edge_effects(
        &self,
        _block: rustc_middle::mir::BasicBlock,
        _discr: &rustc_middle::mir::Operand<'tcx>,
        _apply_edge_effects: &mut impl rustc_mir_dataflow::framework::SwitchIntEdgeEffects<Self::Domain>,
    ) {
    }
    */
}
