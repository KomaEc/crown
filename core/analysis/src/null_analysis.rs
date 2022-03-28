use std::collections::HashSet;

use instrumented_rustc_mir_dataflow::{
    fmt::DebugWithContext, lattice::FlatSet, Analysis, AnalysisDomain, CallReturnPlaces, Engine,
    Forward, JoinSemiLattice, ResultsCursor,
};
use rustc_hir::definitions::DefPathData;
use rustc_index::{
    bit_set::{BitMatrix, BitSet},
    vec::IndexVec,
};
use rustc_middle::{
    mir::{
        visit::Visitor, BasicBlock, Body, Constant, ConstantKind, Local, LocalDecls, Location,
        Rvalue, TerminatorKind,
    },
    ty::{TyCtxt, TyKind},
};
use smallvec::{smallvec, SmallVec};

use crate::{pointer_analysis::intra::IntraPts, ty_ext::TyExt};

pub struct NullTestDest<'tcx> {
    tcx: TyCtxt<'tcx>,
}

impl<'tcx> std::fmt::Debug for NullTestDest<'tcx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("is null test analysis")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NullSource {
    TestPos(Local),
    TestNeg(Local),
}

impl std::ops::Not for NullSource {
    type Output = NullSource;

    fn not(self) -> Self::Output {
        match self {
            NullSource::TestPos(local) => NullSource::TestNeg(local),
            NullSource::TestNeg(local) => NullSource::TestPos(local),
        }
    }
}

impl<'tcx> DebugWithContext<NullTestDest<'tcx>> for IndexVec<Local, FlatSet<NullSource>> {}

impl<'tcx> AnalysisDomain<'tcx> for NullTestDest<'tcx> {
    type Domain = IndexVec<Local, FlatSet<NullSource>>;

    type Direction = Forward;

    const NAME: &'static str = "is null test";

    fn bottom_value(&self, body: &rustc_middle::mir::Body<'tcx>) -> Self::Domain {
        IndexVec::from_elem(FlatSet::Bottom, &body.local_decls)
    }

    fn initialize_start_block(
        &self,
        _body: &rustc_middle::mir::Body<'tcx>,
        _state: &mut Self::Domain,
    ) {
    }
}

impl<'tcx> Analysis<'tcx> for NullTestDest<'tcx> {
    fn apply_statement_effect(
        &self,
        state: &mut Self::Domain,
        statement: &rustc_middle::mir::Statement<'tcx>,
        _location: rustc_middle::mir::Location,
    ) {
        match &statement.kind {
            rustc_middle::mir::StatementKind::Assign(box (
                place,
                Rvalue::Use(operand) | Rvalue::Cast(_, operand, _),
            )) => {
                if let Some(lhs) = place.as_local() {
                    if let Some(rhs) = operand.place() {
                        if let Some(rhs) = rhs.as_local() {
                            state[lhs] = state[rhs];
                        }
                    }
                }
            }
            rustc_middle::mir::StatementKind::Assign(box (
                place,
                Rvalue::UnaryOp(rustc_middle::mir::UnOp::Not, operand),
            )) => {
                if let Some(lhs) = place.as_local() {
                    if let Some(rhs) = operand.place() {
                        if let Some(rhs) = rhs.as_local() {
                            state[lhs] = match &state[rhs] {
                                FlatSet::Elem(elem) => FlatSet::Elem(!*elem),
                                s @ _ => s.clone(),
                            };
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
        _location: rustc_middle::mir::Location,
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
                            state[lhs] = FlatSet::Elem(NullSource::TestPos(ptr));
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
        _return_places: CallReturnPlaces<'_, 'tcx>,
    ) {
    }
}

pub struct NullnessApproximation<'me, 'tcx> {
    intra_block_nullness: ResultsCursor<'me, 'tcx, IntraBlockNullness>,
    intra_pts: ResultsCursor<'me, 'tcx, IntraPts>,
    cache: BitSet<Local>
}

impl<'me, 'tcx> NullnessApproximation<'me, 'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, body: &'me Body<'tcx>) -> Self {
        let intra_block_nullness = IntraBlockNullness
            .into_engine(tcx, body)
            .iterate_to_fixpoint()
            .into_results_cursor(body);
        let intra_pts = IntraPts::new()
            .into_engine(tcx, body)
            .iterate_to_fixpoint()
            .into_results_cursor(body);
        Self { intra_block_nullness, intra_pts, cache: BitSet::new_empty(body.local_decls.len()) }
    }

    pub fn query(&mut self, location: Location) -> &BitSet<Local> {
        self.intra_block_nullness.seek_before_primary_effect(location);
        self.intra_pts.seek_before_primary_effect(location);
        self.cache.clear();
        for nullified_ptr in self.intra_block_nullness.get().iter() {
            self.cache.insert(nullified_ptr);
            let nullified_rep = self.intra_pts.get()[nullified_ptr];
            // alias might exist
            if matches!(nullified_rep, FlatSet::Elem(..)) {
                for (ptr, rep) in self.intra_pts.get().iter_enumerated() {
                    // if ptr is an alias
                    if *rep == nullified_rep {
                        self.cache.insert(ptr);
                    }
                }
            }
        }
        &self.cache
    }
}



pub struct IntraBlockNullness;

impl<'tcx> AnalysisDomain<'tcx> for IntraBlockNullness {
    type Domain = BitSet<Local>;

    type Direction = Forward;

    const NAME: &'static str = "must null";

    fn bottom_value(&self, body: &rustc_middle::mir::Body<'tcx>) -> Self::Domain {
        BitSet::new_empty(body.local_decls.len())
    }

    fn initialize_start_block(
        &self,
        body: &rustc_middle::mir::Body<'tcx>,
        state: &mut Self::Domain,
    ) {
        for local in body.vars_and_temps_iter() {
            if body.local_decls[local].ty.is_ptr_but_not_fn_ptr() {
                state.insert(local);
            }
        }
    }
}

struct NullTestDestResults<'me, 'tcx> {
    cursor: ResultsCursor<'me, 'tcx, NullTestDest<'tcx>>,
    entry_sets: IndexVec<BasicBlock, BitSet<Local>>,
}

impl<'me, 'tcx> rustc_middle::mir::visit::Visitor<'tcx> for NullTestDestResults<'me, 'tcx> {
    fn visit_terminator(
        &mut self,
        terminator: &rustc_middle::mir::Terminator<'tcx>,
        location: rustc_middle::mir::Location,
    ) {
        if let TerminatorKind::SwitchInt {
            discr,
            switch_ty: _,
            targets,
        } = &terminator.kind
        {
            if let Some(discr) = discr.place().map(|p| p.as_local()).flatten() {
                self.cursor.seek_before_primary_effect(location);
                if let FlatSet::Elem(elem) = self.cursor.get()[discr] {
                    assert_eq!(targets.all_targets().len(), 2);
                    match elem {
                        NullSource::TestPos(local) => {
                            println!("setting {:?} to null at {:?}", local, targets.all_targets()[1]);
                            self.entry_sets[targets.all_targets()[1]].insert(local);
                            // self.entry_sets[targets.all_targets()[1]] =
                        }
                        NullSource::TestNeg(local) => {
                            println!("setting {:?} to null at {:?}", local, targets.all_targets()[0]);
                            self.entry_sets[targets.all_targets()[0]].insert(local);
                        }
                    }
                }
            }
        }
    }
}



impl<'tcx> Analysis<'tcx> for IntraBlockNullness {
    fn into_engine<'mir>(
        self,
        tcx: TyCtxt<'tcx>,
        body: &'mir rustc_middle::mir::Body<'tcx>,
    ) -> Engine<'mir, 'tcx, Self>
    where
        Self: Sized,
    {
        let is_null_test = NullTestDest::into_engine(NullTestDest { tcx }, tcx, body)
            .iterate_to_fixpoint()
            .into_results_cursor(body);
        let entry_sets = IndexVec::from_elem(self.bottom_value(body), &body.basic_blocks());

        let mut vis = NullTestDestResults {
            cursor: is_null_test,
            entry_sets,
        };

        vis.visit_body(body);

        Engine::new_with_entry_sets(tcx, body, self, vis.entry_sets)
    }

    fn apply_statement_effect(
        &self,
        state: &mut Self::Domain,
        statement: &rustc_middle::mir::Statement<'tcx>,
        _location: rustc_middle::mir::Location,
    ) {
        match &statement.kind {
            rustc_middle::mir::StatementKind::Assign(box (
                place,
                Rvalue::Use(operand) | Rvalue::Cast(_, operand, _),
            )) => {
                if let Some(lhs) = place.as_local() {
                    match operand {
                        rustc_middle::mir::Operand::Copy(rhs)
                        | rustc_middle::mir::Operand::Move(rhs) => {
                            if let Some(rhs) = rhs.as_local() {
                                // for lhs = rhs, if rhs is known to be null
                                // then lhs must be null
                                if state.contains(rhs) {
                                    state.insert(lhs);
                                } else {
                                    state.remove(lhs);
                                }
                            }
                        }
                        _ => {} /*
                                rustc_middle::mir::Operand::Constant(box c) => {
                                    if let Some(scalar_int) = c.literal.try_to_scalar_int() {
                                        if scalar_int.is_null() {
                                            state.insert(lhs);
                                        }
                                    }
                                }
                                */
                    }
                }
            }
            _ => {}
        }
    }

    fn apply_terminator_effect(
        &self,
        _state: &mut Self::Domain,
        _terminator: &rustc_middle::mir::Terminator<'tcx>,
        _location: rustc_middle::mir::Location,
    ) {
    }

    fn apply_call_return_effect(
        &self,
        _state: &mut Self::Domain,
        _block: rustc_middle::mir::BasicBlock,
        _return_places: CallReturnPlaces<'_, 'tcx>,
    ) {
    }
}



#[cfg(test)]
mod test {
    use crate::null_analysis::NullnessApproximation;
    use rustc_middle::mir::{BasicBlock, Local, Location};

    #[test]
    fn test1() {
        const PROGRAM: &str = "
        unsafe fn f(p: *mut i32, q: *mut i32) -> i32 {
            if p.is_null() {
                if q.is_null() {
                    0
                } else {
                    1
                }
            } else {
                if q.is_null() {
                    2
                } else {
                    3
                }
            }
        }
        ";

        compiler_interface::run_compiler_with_single_func(PROGRAM.into(), |tcx, did| {
            let body = tcx.optimized_mir(did);
            rustc_middle::mir::pretty::write_mir_fn(
                tcx,
                body,
                &mut |_, _| Ok(()),
                &mut std::io::stdout(),
            )
            .unwrap();

            let mut nullness = NullnessApproximation::new(tcx, body);

            let mut location = Location {
                block: BasicBlock::from_u32(0),
                statement_index: 1,
            };

            let nullified_ptrs = nullness.query(Location { block: BasicBlock::from_u32(4), statement_index: 0});

            assert!(nullified_ptrs.contains(Local::from_u32(1)));
            assert!(nullified_ptrs.contains(Local::from_u32(2)));

            let nullified_ptrs = nullness.query(Location { block: BasicBlock::from_u32(5), statement_index: 0});
            assert!(nullified_ptrs.contains(Local::from_u32(1)));
            assert!(!nullified_ptrs.contains(Local::from_u32(2)));

            let nullified_ptrs = nullness.query(Location { block: BasicBlock::from_u32(8), statement_index: 0});
            assert!(!nullified_ptrs.contains(Local::from_u32(1)));
            assert!(nullified_ptrs.contains(Local::from_u32(2)));

            let nullified_ptrs = nullness.query(Location { block: BasicBlock::from_u32(9), statement_index: 0});
            assert!(!nullified_ptrs.contains(Local::from_u32(1)));
            assert!(!nullified_ptrs.contains(Local::from_u32(2)));
        })
    }
}

