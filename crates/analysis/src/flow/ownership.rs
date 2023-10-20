use rustc_hash::FxHashMap;
use rustc_index::bit_set::BitSet;
use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, NonMutatingUseContext, NonUseContext, PlaceContext, Visitor},
        Body, Local, Location, Place, Rvalue, Terminator, TerminatorKind,
    },
    ty::TyCtxt,
};
use rustc_span::def_id::DefId;
use smallvec::SmallVec;
use utils::compiler_interface::Program;

use self::{
    access_path::AccessPaths,
    constraint::{Database, OwnershipToken, StorageMode, Z3Database},
    copies::collect_copies,
    inference::Intraprocedural,
};
use super::{
    def_use::{Def, DefUseChain, Inspect, LocationBuilder, Update, UseKind},
    LocalMap, SSAIdx,
};
use crate::call_graph::CallGraph;

pub mod access_path;
pub mod constraint;
mod inference;
// TODO re-export
mod copies;
#[cfg(test)]
mod tests;

/// Ownership inference context
pub struct Interprocedural<Mode: StorageMode, DB> {
    database: DB,
    access_paths: AccessPaths,
    storage: Mode::Storage,
    call_graph: CallGraph,
    fn_sigs: FnMap<FnSig<OwnershipToken>>,
}

struct InterproceduralView<'intra, Mode: StorageMode, DB> {
    database: &'intra mut DB,
    storage: &'intra mut Mode::Storage,
    access_paths: &'intra AccessPaths,
    // call_graph: &'intra CallGraph,
    fn_sigs: &'intra FnMap<FnSig<OwnershipToken>>,
}

macro_rules! into_view {
    ($this: expr) => {
        InterproceduralView {
            database: &mut $this.database,
            storage: &mut $this.storage,
            access_paths: &$this.access_paths,
            // call_graph: &$this.call_graph,
            fn_sigs: &$this.fn_sigs,
        }
    };
}

impl<Mode: StorageMode> Interprocedural<Mode, Z3Database<'_>> {
    pub fn run(mut self, tcx: TyCtxt) -> (FnMap<FnSig<OwnershipToken>>, FnMap<BodySummary>) {
        let mut body_summaries = FnMap::default();
        for &def_id in self.call_graph.fns() {
            let body = tcx.optimized_mir(def_id);
            let mut k_limit = self.access_paths.max_k_limit();
            // Solving must succeed when k_limit == 0
            loop {
                tracing::debug!(
                    "solving {} with k_limit {k_limit}",
                    tcx.def_path_str(def_id)
                );
                self.database.solver.push();

                let mut inter_view: InterproceduralView<'_, Mode, _> = into_view!(self);
                let mut intra_inference = Intraprocedural::new(&mut inter_view, body, tcx, k_limit);
                intra_inference.visit_body(body);

                let body_summary = BodySummary {
                    flow_chain: intra_inference.flow_chain,
                    local_map: intra_inference.tokens,
                };

                match self.database.solver.check() {
                    z3::SatResult::Unsat => {
                        if k_limit == 0 {
                            unreachable!();
                        }
                        self.database.solver.pop(1);
                        k_limit -= 1;
                    }
                    z3::SatResult::Unknown => panic!("z3 timed out"),
                    z3::SatResult::Sat => {
                        body_summaries.insert(def_id, body_summary);
                        break;
                    }
                }
            }
        }
        (self.fn_sigs, body_summaries)
    }
}

impl<Mode, DB> Interprocedural<Mode, DB>
where
    Mode: StorageMode,
    DB: Database<Mode>,
{
    pub fn new(program: &Program, database: DB, storage: Mode::Storage) -> Self {
        let access_paths = AccessPaths::new(program);
        let call_graph = CallGraph::new(program.tcx, &program.fns);
        let mut ctxt = Self {
            database,
            access_paths,
            storage,
            call_graph,
            fn_sigs: FxHashMap::default(),
        };
        ctxt.initialise_fn_sigs(&program.fns, program.tcx);
        ctxt
    }

    fn initialise_fn_sigs(&mut self, fns: &[DefId], tcx: TyCtxt) {
        let max_k_limit = self.access_paths.max_k_limit();
        for def_id in fns {
            let fn_sig = self.new_fn_sig(max_k_limit, def_id, tcx);
            self.fn_sigs.insert(*def_id, fn_sig);
        }
    }

    fn new_fn_sig(&mut self, k_limit: usize, def_id: &DefId, tcx: TyCtxt) -> FnSig<OwnershipToken> {
        let fn_sig = tcx.fn_sig(def_id).skip_binder();
        let output = self
            .database
            .new_tokens(
                self.access_paths
                    .size_of(k_limit, fn_sig.output().skip_binder()),
            )
            .start;
        let inputs = fn_sig
            .inputs()
            .iter()
            .map(|ty| {
                let size = self.access_paths.size_of(k_limit, *ty.skip_binder());
                self.database.new_tokens(size).start
            })
            .collect();

        tracing::debug!("generating signature with output = {output:?}, inputs = {inputs:?}");

        FnSig {
            k_limit,
            output,
            inputs,
        }
    }

    #[cfg(test)]
    fn dry_run(&mut self, tcx: TyCtxt) {
        let max_k_limit = self.access_paths.max_k_limit();
        for &def_id in self.call_graph.fns() {
            let body = tcx.optimized_mir(def_id);
            let mut inter_view = into_view!(self);
            let mut intra_inference = Intraprocedural::new(&mut inter_view, body, tcx, max_k_limit);
            intra_inference.visit_body(body);
        }
    }
}

pub type FnMap<T> = FxHashMap<DefId, T>;

#[derive(Clone, Debug)]
pub struct FnSig<T> {
    k_limit: usize,
    output: T,
    inputs: SmallVec<[T; 3]>,
}

pub struct BodySummary {
    pub flow_chain: DefUseChain,
    pub local_map: LocalMap<OwnershipToken>,
}

fn flow_chain<'tcx>(body: &Body<'tcx>, access_paths: &AccessPaths, k_limit: usize) -> DefUseChain {
    let flow_builder = OwnershipFlowBuilder {
        body,
        access_paths,
        location_data: Default::default(),
        copies: &collect_copies(body),
        k_limit,
    };
    DefUseChain::new(body, flow_builder)
}

struct OwnershipFlowBuilder<'build, 'tcx> {
    body: &'build Body<'tcx>,
    access_paths: &'build AccessPaths,
    location_data: SmallVec<[(Local, UseKind<SSAIdx>); 2]>,
    copies: &'build BitSet<Local>,
    k_limit: usize,
}

impl<'build, 'tcx> OwnershipFlowBuilder<'build, 'tcx> {
    fn place_flow(&self, place: &Place<'tcx>, context: PlaceContext) -> Option<OwnershipFlow> {
        if self
            .access_paths
            .path(self.k_limit, place, self.body)
            .num_pointers_reachable()
            == 0
        {
            return None;
        }
        OwnershipFlow::for_place(context).map(|flow| {
            if self.copies.contains(place.local) {
                if place.as_local().is_some()
                    && matches!(
                        context,
                        PlaceContext::MutatingUse(MutatingUseContext::Store)
                    )
                {
                    OwnershipFlow::Flow
                } else {
                    OwnershipFlow::Inspect
                }
            } else {
                flow
            }
        })
    }
}

impl<'build, 'tcx> LocationBuilder<'tcx> for OwnershipFlowBuilder<'build, 'tcx> {
    fn retrieve(&mut self) -> SmallVec<[(Local, UseKind<SSAIdx>); 2]> {
        std::mem::take(&mut self.location_data)
    }
}

impl<'build, 'tcx> Visitor<'tcx> for OwnershipFlowBuilder<'build, 'tcx> {
    // for return terminator and indices
    fn visit_local(&mut self, local: Local, context: PlaceContext, location: Location) {
        self.visit_place(&Place::from(local), context, location)
    }

    fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, location: Location) {
        if let Some(flow) = self.place_flow(place, context) {
            if matches!(flow, OwnershipFlow::Flow) {
                self.location_data.push((place.local, Def(Update::new())));
            } else {
                self.location_data.push((place.local, Inspect(SSAIdx::MAX)));
            }
        }
        // call super_projection so that index operators are visited
        self.super_projection(place.as_ref(), context, location);
    }

    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        if let Rvalue::BinaryOp(_, box (operand1, operand2)) = rvalue {
            // special casing statements like _1 = BitAnd(_1, _3)
            // in this case, we do not generate usage for the right _1
            if let Some((lhs, operand1)) = place
                .as_local()
                .zip(operand1.place().and_then(|place| place.as_local()))
            {
                if lhs == operand1 {
                    self.visit_place(
                        place,
                        PlaceContext::MutatingUse(MutatingUseContext::Store),
                        location,
                    );
                    self.visit_operand(operand2, location);
                    return;
                }
            }
        } else if let Rvalue::CopyForDeref(rhs) = rvalue {
            // special casing deref copies
            assert!(place.as_local().is_some());
            self.visit_place(
                place,
                PlaceContext::MutatingUse(MutatingUseContext::Store),
                location,
            );
            self.visit_place(
                rhs,
                PlaceContext::NonMutatingUse(NonMutatingUseContext::Copy),
                location,
            );
            return;
        }
        self.super_assign(place, rvalue, location);
    }

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        if let TerminatorKind::Return = &terminator.kind {
            for local in self.body.local_decls.indices() {
                self.visit_local(
                    local,
                    PlaceContext::NonMutatingUse(NonMutatingUseContext::Inspect),
                    location,
                )
            }
            return;
        }
        self.super_terminator(terminator, location);
    }
}

enum OwnershipFlow {
    Flow,
    Inspect,
}

impl OwnershipFlow {
    fn for_place(context: PlaceContext) -> Option<OwnershipFlow> {
        match context {
            PlaceContext::NonUse(NonUseContext::StorageDead)
            | PlaceContext::NonUse(NonUseContext::StorageLive) => {
                tracing::error!("StorageLive, StorageDead");
                None
            }

            PlaceContext::NonUse(_) => None,

            // Ownership flows for all mutating uses
            PlaceContext::MutatingUse(
                MutatingUseContext::Call
                | MutatingUseContext::Yield
                | MutatingUseContext::AsmOutput
                | MutatingUseContext::Store
                | MutatingUseContext::Deinit,
            ) => Some(OwnershipFlow::Flow),

            PlaceContext::MutatingUse(MutatingUseContext::SetDiscriminant) => {
                Some(OwnershipFlow::Inspect)
            }

            // Ownership flows for all kinds of borrows/address ofs
            //
            // Note that ownership flows for shared borrow as well, as it may be leaked to
            // a const address, which is not guaranteed read-only
            PlaceContext::MutatingUse(
                MutatingUseContext::AddressOf | MutatingUseContext::Borrow,
            )
            | PlaceContext::NonMutatingUse(
                NonMutatingUseContext::AddressOf
                | NonMutatingUseContext::ShallowBorrow
                | NonMutatingUseContext::SharedBorrow,
            ) => Some(OwnershipFlow::Flow),

            // Ownership flows for copy and move
            PlaceContext::NonMutatingUse(
                NonMutatingUseContext::Copy | NonMutatingUseContext::Move,
            ) => Some(OwnershipFlow::Flow),

            // deref copy, len, discriminant, etc.
            PlaceContext::NonMutatingUse(NonMutatingUseContext::Inspect) => {
                Some(OwnershipFlow::Inspect)
            }

            // TODO place mention?
            PlaceContext::NonMutatingUse(NonMutatingUseContext::PlaceMention) => todo!(),

            // All other contexts are uses...
            PlaceContext::MutatingUse(MutatingUseContext::Drop | MutatingUseContext::Retag) => {
                unreachable!()
            }

            PlaceContext::MutatingUse(MutatingUseContext::Projection)
            | PlaceContext::NonMutatingUse(NonMutatingUseContext::Projection) => {
                unreachable!("A projection could be a def or a use and must be handled separately")
            }
        }
    }
}
