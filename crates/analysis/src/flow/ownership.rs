use rustc_hash::FxHashMap;
use rustc_index::{bit_set::BitSet, IndexVec};
use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, NonMutatingUseContext, NonUseContext, PlaceContext, Visitor},
        Body, Local, Location, Place, Rvalue, Terminator, TerminatorKind,
    },
    ty::{Ty, TyCtxt},
};
use rustc_span::def_id::DefId;
use rustc_type_ir::TyKind;
use smallvec::SmallVec;
use utils::compiler_interface::Program;

use self::{
    access_path::AccessPaths,
    constraint::{Database, OwnershipToken, StorageMode, Z3Database},
    copies::Copies,
    inference::Intraprocedural,
    ty_summary::TySummary,
};
use super::{
    def_use::{
        builder::prune::dead_code_elimination, Def, DefUseChain, Inspect, LocationBuilder, Update,
        UseKind,
    },
    LocalMap, SSAIdx,
};
use crate::{
    call_graph::CallGraph,
    flow::ownership::{constraint::Constraint, ty_summary::unwrap_pointers},
    type_qualifier::output_params::OutputParams,
};

pub mod access_path;
pub mod constraint;
mod inference;
// TODO re-export
pub mod copies;
#[cfg(test)]
mod tests;
mod ty_summary;

pub fn analyse(program: &Program, output_params: &OutputParams) -> AnalysisResult {
    let config = z3::Config::new();
    let ctx = z3::Context::new(&config);

    use self::constraint::Debug;
    let infer_ctxt: Interprocedural<Debug, _> =
        Interprocedural::new(&program, output_params, Z3Database::new(&ctx), ());
    infer_ctxt.run(program.tcx)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Ownership {
    Owning,
    Transient,
}

pub struct AnalysisResult {
    pub model: IndexVec<OwnershipToken, Ownership>,
    access_paths: AccessPaths,
    pub fn_sigs: FnMap<FnSig<OwnershipToken>>,
    body_summaries: FnMap<BodySummary>,
    pub ty_summaries: StructMap<TySummary>,
}

impl AnalysisResult {
    pub fn fn_sig_str(&self, body: &Body) -> String {
        let fn_sig = &self.fn_sigs[&body.source.def_id()];
        let k_limit = fn_sig.k_limit;
        if k_limit == 0 {
            return "() ->".to_owned();
        }
        let inputs = fn_sig
            .inputs
            .iter()
            .copied()
            .zip(body.args_iter())
            .map(|(param, local)| {
                let ty = body.local_decls[local].ty;
                match param {
                    Param::Normal(token) => self.ownership_type_str(token, ty, k_limit),
                    Param::Output(Update { r#use, def }) => format!(
                        "&mut {} \u{2193} &mut {}",
                        self.ownership_type_str(
                            r#use + 1u32,
                            ty.builtin_deref(true).unwrap().ty,
                            k_limit - 1
                        ),
                        self.ownership_type_str(
                            def + 1u32,
                            ty.builtin_deref(true).unwrap().ty,
                            k_limit - 1
                        )
                    ),
                }
            })
            .collect::<Vec<_>>()
            .join(", ");

        let output = {
            let ty = body.bound_return_ty().skip_binder();
            self.ownership_type_str(fn_sig.output, ty, k_limit)
        };

        format!("({inputs}) -> {output}")
    }

    pub fn body_summary_str(&self, body: &Body) -> String {
        let fn_sig = &self.fn_sigs[&body.source.def_id()];
        let body_summary = &self.body_summaries[&body.source.def_id()];
        let k_limit = fn_sig.k_limit;

        use std::fmt::Write as _;
        let mut ret = String::new();
        for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
            writeln!(&mut ret, "{:?}:", bb).unwrap();
            let mut statement_index = 0;
            for statement in bb_data.statements.iter() {
                writeln!(&mut ret, "  {:?}", statement).unwrap();

                let location = Location {
                    block: bb,
                    statement_index,
                };
                let uses = body_summary.flow_chain.uses[location]
                    .iter()
                    .map(|&(local, use_kind)| {
                        let ty = body.local_decls[local].ty;
                        match use_kind {
                            Inspect(ssa_idx) => {
                                let start_token = body_summary.local_map[local][ssa_idx];
                                format!(
                                    "{local:?}: {}",
                                    self.ownership_type_str(start_token, ty, k_limit)
                                )
                            }
                            Def(update) => {
                                let update =
                                    update.map(|ssa_idx| body_summary.local_map[local][ssa_idx]);
                                format!(
                                    "{local:?}: {} \u{2193} {}",
                                    self.ownership_type_str(update.r#use, ty, k_limit),
                                    self.ownership_type_str(update.def, ty, k_limit)
                                )
                            }
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                writeln!(&mut ret, "  uses: {uses}").unwrap();

                statement_index += 1;
            }
            if let Some(terminator) = &bb_data.terminator {
                writeln!(&mut ret, "  {:?}", terminator.kind).unwrap();
                let location = Location {
                    block: bb,
                    statement_index,
                };
                let uses = body_summary.flow_chain.uses[location]
                    .iter()
                    .map(|&(local, use_kind)| {
                        let ty = body.local_decls[local].ty;
                        match use_kind {
                            Inspect(ssa_idx) => {
                                let start_token = body_summary.local_map[local][ssa_idx];
                                format!(
                                    "{local:?}: {}",
                                    self.ownership_type_str(start_token, ty, k_limit)
                                )
                            }
                            Def(update) => {
                                let update =
                                    update.map(|ssa_idx| body_summary.local_map[local][ssa_idx]);
                                format!(
                                    "{local:?}: {} \u{2193} {}",
                                    self.ownership_type_str(update.r#use, ty, k_limit),
                                    self.ownership_type_str(update.def, ty, k_limit)
                                )
                            }
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                writeln!(&mut ret, "  uses: {uses}").unwrap();
            }
        }

        ret
    }

    pub fn ty_summary_str(&self, def_id: DefId, tcx: TyCtxt) -> String {
        use std::fmt::Write as _;
        let mut ret = String::new();

        writeln!(&mut ret, "{} {{", tcx.def_path_str(def_id)).unwrap();
        let ty_summary = self.ty_summaries.get(&def_id).unwrap();
        let TyKind::Adt(adt_def, arg) = tcx.type_of(def_id).skip_binder().kind() else {
            unreachable!()
        };
        for (field_def, field_summary) in
            adt_def.all_fields().zip(ty_summary.fields.iter().copied())
        {
            let field_ty = field_def.ty(tcx, arg);
            let (num_wrapping_pointers, _) = unwrap_pointers(field_ty);
            writeln!(
                &mut ret,
                "    {}: {}",
                field_def.ident(tcx),
                self.ownership_type_str(field_summary, field_ty, num_wrapping_pointers)
            )
            .unwrap();
        }
        writeln!(&mut ret, "}}").unwrap();
        ret
    }

    pub fn ownership_type_str(
        &self,
        start_token: OwnershipToken,
        ty: Ty,
        k_limit: usize,
    ) -> String {
        let size = self.access_paths.size_of(k_limit, ty);
        let ownership_type = (start_token..start_token + size)
            // .map(|token| format!("{token} = {:?}", self.model[token]))
            .map(|token| {
                match self.model[token] {
                    Ownership::Owning => "&move",
                    Ownership::Transient => "&",
                }
                .to_owned()
            })
            .collect::<Vec<_>>()
            .join(" ");
        format!("{ownership_type}")
        // format!("{:?}", ownership_type)
    }
}

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

impl<'z3, Mode: StorageMode> Interprocedural<Mode, Z3Database<'z3>> {
    pub fn run(mut self, tcx: TyCtxt) -> AnalysisResult {
        let mut ty_summaries = StructMap::default();
        for &def_id in self.access_paths.post_order() {
            ty_summaries.insert(
                def_id,
                TySummary::new::<Mode, _>(def_id, &mut self.database, tcx),
            );
        }
        let mut inter_view: InterproceduralView<'_, Mode, _> = into_view!(self);
        inter_view.summarise_types(&ty_summaries, &self.call_graph, tcx);
        let mut body_summaries = FnMap::default();
        for &def_id in self.call_graph.fns() {
            let body = tcx.optimized_mir(def_id);
            let mut k_limit = self.access_paths.max_k_limit();
            // Solving must succeed when k_limit == 0
            loop {
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
                        tracing::debug!(
                            "\u{274C} {} failed with k_limit {k_limit}",
                            tcx.def_path_str(def_id)
                        );
                        k_limit -= 1;
                        // FIXME: generate new signature, and equate the previous with current!!
                        self.fn_sigs.get_mut(&def_id).unwrap().k_limit = k_limit;
                    }
                    z3::SatResult::Unknown => panic!("z3 timed out"),
                    z3::SatResult::Sat => {
                        tracing::debug!(
                            "\u{2705} solved {} with k_limit {k_limit}",
                            tcx.def_path_str(def_id)
                        );
                        body_summaries.insert(def_id, body_summary);
                        break;
                    }
                }
            }
        }
        AnalysisResult {
            model: self.extract_model(),
            access_paths: self.access_paths,
            fn_sigs: self.fn_sigs,
            body_summaries,
            ty_summaries,
        }
    }

    // TODO model completion?
    fn extract_model(&self) -> IndexVec<OwnershipToken, Ownership> {
        let z3_model = self.database.solver.get_model().unwrap();
        let mut model = IndexVec::with_capacity(self.database.gen.next().index());
        for ast_node in self.database.z3_ast.iter() {
            let value = z3_model.eval(ast_node, false).unwrap().as_bool();
            if matches!(value, Some(cond) if cond) {
                model.push(Ownership::Owning);
            } else {
                model.push(Ownership::Transient);
            }
        }
        model
    }
}

impl<Mode, DB> Interprocedural<Mode, DB>
where
    Mode: StorageMode,
    DB: Database<Mode>,
{
    pub fn new(
        program: &Program,
        output_params: &OutputParams,
        mut database: DB,
        mut storage: Mode::Storage,
    ) -> Self {
        let access_paths = AccessPaths::new(program);
        let call_graph = CallGraph::new(program.tcx, &program.fns);
        let fn_sigs = program
            .fns
            .iter()
            .map(|&def_id| {
                let body = program.tcx.optimized_mir(def_id);
                (
                    def_id,
                    FnSig::new(
                        body,
                        output_params.get(&body.source.def_id()).unwrap(),
                        &mut database,
                        &mut storage,
                        &access_paths,
                        access_paths.max_k_limit(),
                    ),
                )
            })
            .collect();
        Self {
            database,
            access_paths,
            storage,
            call_graph,
            fn_sigs,
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
    pub k_limit: usize,
    pub output: T,
    pub inputs: SmallVec<[Param<T>; 3]>,
}

impl FnSig<OwnershipToken> {
    fn new<Mode: StorageMode, DB: Database<Mode>>(
        body: &Body,
        output_params: &BitSet<Local>,
        database: &mut DB,
        storage: &mut Mode::Storage,
        access_paths: &AccessPaths,
        k_limit: usize,
    ) -> Self {
        let output = database
            .new_tokens(access_paths.size_of(k_limit, body.return_ty()))
            .start;
        let inputs = body
            .args_iter()
            .map(|local| {
                let ty = body.local_decls[local].ty;
                let size = access_paths.size_of(k_limit, ty);
                if output_params.contains(local) {
                    let r#use = database.new_tokens(size).start;
                    let def = database.new_tokens(size).start;
                    database.add(
                        Constraint::Assume {
                            x: r#use,
                            sign: true,
                        },
                        storage,
                    );
                    database.add(Constraint::Assume { x: def, sign: true }, storage);
                    Param::Output(Update { r#use, def })
                } else {
                    Param::Normal(database.new_tokens(size).start)
                }
            })
            .collect();

        tracing::debug!("generating signature with output = {output:?}, inputs = {inputs:?}");

        FnSig {
            k_limit,
            output,
            inputs,
        }
    }
}

pub struct BodySummary {
    pub flow_chain: DefUseChain,
    pub local_map: LocalMap<OwnershipToken>,
}

#[derive(Clone, Copy, Debug)]
pub enum Param<T> {
    Normal(T),
    Output(Update<T>),
}

impl<T> Param<T> {
    pub fn input(self) -> T {
        match self {
            Param::Normal(t) => t,
            Param::Output(update) => update.r#use,
        }
    }

    pub fn output(self) -> Option<T> {
        match self {
            Param::Normal(_) => None,
            Param::Output(update) => Some(update.def),
        }
    }
}

pub type StructMap<T> = FxHashMap<DefId, T>;

fn flow_chain<'tcx>(
    body: &Body<'tcx>,
    copies: &Copies,
    access_paths: &AccessPaths,
    k_limit: usize,
) -> DefUseChain {
    let flow_builder = OwnershipFlowBuilder {
        body,
        access_paths,
        location_data: Default::default(),
        copies,
        k_limit,
    };
    let flow_chain = DefUseChain::new(body, flow_builder);
    dead_code_elimination(body, flow_chain)
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
        OwnershipFlow::is_flow(context).then(|| {
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
                OwnershipFlow::Flow
            }
        })
    }

    fn use_place(&mut self, place: &Place<'tcx>, flow: Option<OwnershipFlow>) {
        if let Some(flow) = flow {
            if matches!(flow, OwnershipFlow::Flow) {
                self.location_data.push((place.local, Def(Update::new())));
            } else {
                self.location_data.push((place.local, Inspect(SSAIdx::MAX)));
            }
        }
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
        self.use_place(place, self.place_flow(place, context));
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
                if !self.copies.contains(local)
                    && self
                        .access_paths
                        .size_of(self.k_limit, self.body.local_decls[local].ty)
                        > 0
                {
                    self.use_place(&Place::from(local), Some(OwnershipFlow::Inspect))
                }
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
    fn is_flow(context: PlaceContext) -> bool {
        match context {
            PlaceContext::NonUse(NonUseContext::StorageDead)
            | PlaceContext::NonUse(NonUseContext::StorageLive) => {
                tracing::error!("StorageLive, StorageDead");
                false
            }

            PlaceContext::NonUse(_) => false,

            // Ownership flows for all mutating uses
            PlaceContext::MutatingUse(
                MutatingUseContext::Call
                | MutatingUseContext::Yield
                | MutatingUseContext::AsmOutput
                | MutatingUseContext::Store
                | MutatingUseContext::Deinit,
            ) => true,

            PlaceContext::MutatingUse(MutatingUseContext::SetDiscriminant) => false,

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
            ) => true,

            // Ownership flows for copy and move
            PlaceContext::NonMutatingUse(
                NonMutatingUseContext::Copy | NonMutatingUseContext::Move,
            ) => true,

            // deref copy, len, discriminant, etc.
            PlaceContext::NonMutatingUse(NonMutatingUseContext::Inspect) => false,

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
