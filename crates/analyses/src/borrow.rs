//! Borrow inference

use colored::Colorize;
use rustc_hir::def_id::DefId;
use rustc_index::{
    IndexVec,
    bit_set::{DenseBitSet, SparseBitMatrix},
};
use rustc_middle::{
    mir::{
        Body, HasLocalDecls, Local, Location, Operand, Place, PlaceElem, RETURN_PLACE, Rvalue,
        Terminator, visit::Visitor,
    },
    ty::TyCtxt,
};
use rustc_mir_dataflow::{fmt::DebugWithContext, points::DenseLocationMap};
use utils::{rustc::RustProgram, rustc_hash::FxHashMap};

use crate::{
    borrow::{
        errors::{Errors, compute_errors},
        invalidates::{Invalidates, compute_invalidates},
        killed::{Killed, compute_killed},
        loan_liveness::{LoanLiveness, compute_loan_liveness},
        provenance_liveness::{ProvenanceLiveness, compute_provenance_liveness},
        requires::{ProvenanceRequiresLoan, compute_requires},
        subset_closure::{SubSetClosure, compute_subset_closure},
    },
    mir::TerminatorExt,
    type_qualifier::mutability_analysis,
};

macro_rules! disallow_interprocedural {
    () => {
        // panic!()
    };
}

mod errors;
mod invalidates;
mod killed;
mod loan_liveness;
mod places_conflict;
mod provenance_liveness;
mod requires;
mod subset_closure;

rustc_index::newtype_index! {
    #[orderable]
    pub struct Provenance {
    }
}

pub enum ProvenanceData {
    PlaceHolder(Local),
    Local(Local),
}

impl ProvenanceData {
    pub fn local(&self) -> Local {
        match self {
            ProvenanceData::PlaceHolder(local) => *local,
            ProvenanceData::Local(local) => *local,
        }
    }
}

impl std::fmt::Debug for ProvenanceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let local = self.local();
        f.write_fmt(format_args!("'{local:?}"))
    }
}

/// This formulation is definitely wrong as we don't create [`Origin`]
/// for nested pointers. But I guess it could be fine?
pub struct ProvenanceSet {
    local_data: IndexVec<Local, Option<Provenance>>,
    provenance_data: IndexVec<Provenance, ProvenanceData>,
}

pub trait HasProvenanceSet {
    fn provenance_set<I>(&self, is_candidate: I) -> ProvenanceSet
    where
        I: Fn(Local) -> bool;
}

impl HasProvenanceSet for Body<'_> {
    fn provenance_set<I>(&self, is_candidate: I) -> ProvenanceSet
    where
        I: Fn(Local) -> bool,
    {
        let body = self;
        let mut local_data = IndexVec::from_elem_n(None, body.local_decls.len());
        let mut provenance_data = IndexVec::new();

        for (provenance, (local, local_decl)) in local_data
            .iter_mut()
            .zip(body.local_decls.iter_enumerated())
        {
            if local_decl.ty.is_any_ptr() && is_candidate(local) {
                let data = if local.index() <= body.arg_count {
                    ProvenanceData::PlaceHolder(local)
                } else {
                    ProvenanceData::Local(local)
                };
                *provenance = Some(provenance_data.push(data));
            }
        }

        ProvenanceSet {
            local_data,
            provenance_data,
        }
    }
}

pub struct GBorrowInferCtxt {
    pub provenances: FxHashMap<DefId, ProvenanceSet>,
}

impl GBorrowInferCtxt {
    pub fn new<I, J>(program: &RustProgram, is_candidate: I) -> Self
    where
        I: Fn(DefId) -> J,
        J: Fn(Local) -> bool,
    {
        let mut provenances = FxHashMap::default();
        for f in program.functions.iter().copied() {
            let body = program
                .tcx
                .mir_drops_elaborated_and_const_checked(f.expect_local())
                .borrow();
            let is_candidate = is_candidate(f);
            provenances.insert(f, body.provenance_set(|local| is_candidate(local)));
        }

        GBorrowInferCtxt { provenances }
    }

    pub fn mutable_pointers_only(program: &RustProgram) -> Self {
        let mutability_results = mutability_analysis(program);

        GBorrowInferCtxt::new(program, |f| {
            let mutability_results = mutability_results
                .function_body_facts(&f)
                .collect::<IndexVec<Local, _>>();

            move |local| {
                mutability_results[local]
                    .first()
                    .is_some_and(|mutability| mutability.is_mutable())
            }
        })
    }
}

rustc_index::newtype_index! {
    #[orderable]
    #[debug_format = "L_({})"]
    pub struct Loan {
    }
}

impl<C> DebugWithContext<C> for Loan {}

pub struct BorrowData<'tcx> {
    location: Location,
    borrowed: Place<'tcx>,
    assigned: Borrower<'tcx>,
}

#[derive(Clone, Copy, Debug)]
pub enum Borrower<'tcx> {
    AssignStmt(Place<'tcx>),
    CallArg(DefId, usize),
}

impl std::fmt::Debug for BorrowData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?} @ {:?}", self.borrowed, self.location))
    }
}

pub struct BorrowSet<'tcx> {
    loans: IndexVec<Loan, BorrowData<'tcx>>,
    location_map: FxHashMap<Location, Vec<Loan>>, // one location can hold multiple loans
    local_map: SparseBitMatrix<Local, Loan>,
}

pub trait HasBorrowSet<'tcx> {
    fn borrow_set<'local, 'global: 'local>(
        &self,
        tcx: TyCtxt<'tcx>,
        provenance_set: &'local ProvenanceSet,
        global_borrow_ctxt: &'global GBorrowInferCtxt,
    ) -> BorrowSet<'tcx>;
}

impl<'tcx> HasBorrowSet<'tcx> for Body<'tcx> {
    fn borrow_set<'local, 'global: 'local>(
        &self,
        tcx: TyCtxt<'tcx>,
        provenance_set: &'local ProvenanceSet,
        global_borrow_ctxt: &'global GBorrowInferCtxt,
    ) -> BorrowSet<'tcx> {
        struct Vis<'tcx, 'this, D> {
            loans: IndexVec<Loan, BorrowData<'tcx>>,
            location_map: FxHashMap<Location, Vec<Loan>>,
            local_decl: &'this D,
            tcx: TyCtxt<'tcx>,
            provenance_set: &'this ProvenanceSet,
            global_borrow_ctxt: &'this GBorrowInferCtxt,
        }
        impl<'tcx, 'this, D: HasLocalDecls<'tcx>> Visitor<'tcx> for Vis<'tcx, 'this, D> {
            fn visit_assign(
                &mut self,
                lhs: &Place<'tcx>,
                rvalue: &Rvalue<'tcx>,
                location: Location,
            ) {
                if !matches!(lhs.as_local(), Some(lhs_local) if self.provenance_set.local_data[lhs_local].is_some())
                {
                    return self.super_assign(lhs, rvalue, location);
                }

                let rvalue_ty = rvalue.ty(self.local_decl, self.tcx);
                if !rvalue_ty.is_any_ptr() {
                    return self.super_assign(lhs, rvalue, location);
                }

                match rvalue {
                    Rvalue::Ref(_, _, place) | Rvalue::RawPtr(_, place) => {
                        let loan = self.loans.push(BorrowData {
                            location,
                            borrowed: *place,
                            assigned: Borrower::AssignStmt(*lhs),
                        });
                        self.location_map.insert(location, vec![loan]);
                        // self.location_map.entry(location).and_modify(|loans| loans.push(loan))
                        //     .or_default().push(loan);
                    }
                    Rvalue::CopyForDeref(place)
                    | Rvalue::Use(Operand::Copy(place) | Operand::Move(place)) => {
                        let loan = self.loans.push(BorrowData {
                            location,
                            borrowed: place.project_deeper(&[PlaceElem::Deref], self.tcx),
                            assigned: Borrower::AssignStmt(*lhs),
                        });
                        self.location_map.insert(location, vec![loan]);
                        // self.location_map.entry(location).and_modify(|loans| loans.push(loan))
                        //     .or_default().push(loan);
                    }
                    _ => {}
                }
            }

            fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
                let Some(mir_call) = terminator.as_call(self.tcx) else {
                    return self.super_terminator(terminator, location);
                };
                disallow_interprocedural!();

                if let Some(callee) = mir_call.func.did()
                    && let Some(callee_provenance_set) =
                        self.global_borrow_ctxt.provenances.get(&callee)
                {
                    for (arg_index, arg) in mir_call.args.iter().enumerate() {
                        let arg = &arg.node;
                        if let Some(arg) = arg.place() {
                            let callee_local = Local::from_usize(arg_index + 1);
                            if callee_provenance_set.local_data[callee_local].is_some() {
                                let loan = self.loans.push(BorrowData {
                                    location,
                                    borrowed: arg.project_deeper(&[PlaceElem::Deref], self.tcx),
                                    assigned: Borrower::CallArg(callee, arg_index),
                                });
                                // self.location_map.insert(location, loan);
                                self.location_map
                                    .entry(location)
                                    .and_modify(|loans| loans.push(loan))
                                    .or_default()
                                    .push(loan);
                            }
                        }
                    }
                };
                return self.super_terminator(terminator, location);
            }
        }

        let mut vis = Vis {
            loans: IndexVec::new(),
            location_map: FxHashMap::default(),
            local_decl: self,
            tcx,
            provenance_set,
            global_borrow_ctxt,
        };
        vis.visit_body(self);

        let Vis {
            loans,
            location_map,
            ..
        } = vis;

        let mut local_map = SparseBitMatrix::new(loans.len());

        for (loan, borrow_data) in loans.iter_enumerated() {
            local_map.insert(borrow_data.borrowed.local, loan);
        }

        BorrowSet {
            loans,
            location_map,
            local_map,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SubsetConstraint {
    sup: Provenance,
    sub: Provenance,
    _location: Location,
}

#[derive(Clone, Copy)]
pub struct MembershipConstraint {
    loan: Loan,
    provenance: Provenance,
}

pub struct ProvenanceConstraintGraph {
    subset: Vec<SubsetConstraint>,
    membership: Vec<MembershipConstraint>,
}

impl ProvenanceConstraintGraph {
    pub fn new<'tcx, 'local, 'global: 'local>(
        tcx: TyCtxt<'tcx>,
        body: &Body<'tcx>,
        borrow_set: &BorrowSet<'tcx>,
        provenance_set: &'local ProvenanceSet,
        global_borrow_ctxt: &'global GBorrowInferCtxt,
    ) -> Self {
        struct Vis<'this, 'tcx> {
            tcx: TyCtxt<'tcx>,
            graph: &'this mut ProvenanceConstraintGraph,
            borrow_set: &'this BorrowSet<'tcx>,
            provenance_set: &'this ProvenanceSet,
            global_borrow_ctxt: &'this GBorrowInferCtxt,
        }

        impl<'tcx> Visitor<'tcx> for Vis<'_, 'tcx> {
            fn visit_assign(
                &mut self,
                place: &Place<'tcx>,
                rvalue: &Rvalue<'tcx>,
                location: Location,
            ) {
                let Some(loans) = self.borrow_set.location_map.get(&location) else {
                    return self.super_assign(place, rvalue, location);
                };
                for &loan in loans {
                    let BorrowData {
                        location: _,
                        borrowed: rhs,
                        ..
                    } = &self.borrow_set.loans[loan];

                    let Some(lhs) = place.as_local() else {
                        return self.super_assign(place, rvalue, location);
                    };
                    let lhs_provenance = self.provenance_set.local_data[lhs].unwrap();

                    self.graph.membership.push(MembershipConstraint {
                        loan,
                        provenance: lhs_provenance,
                    });

                    if !rhs.projection.is_empty()
                        && rhs
                            .projection
                            .iter()
                            .all(|projection| matches!(projection, PlaceElem::Deref))
                    {
                        let rhs_provenance = self.provenance_set.local_data[rhs.local].unwrap();
                        self.graph.subset.push(SubsetConstraint {
                            sup: lhs_provenance,
                            sub: rhs_provenance,
                            _location: location,
                        });
                    }
                }
            }

            fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
                let Some(mir_call) = terminator.as_call(self.tcx) else {
                    return self.super_terminator(terminator, location);
                };
                disallow_interprocedural!();
                if let Some(callee) = mir_call.func.did()
                    && let Some(callee_provenance_set) =
                        self.global_borrow_ctxt.provenances.get(&callee)
                {
                    for (arg_index, arg) in mir_call.args.iter().enumerate() {
                        let arg = &arg.node;
                        if let Some(_arg) = arg.place() {
                            let callee_local = Local::from_usize(arg_index + 1);
                            if callee_provenance_set.local_data[callee_local].is_some() {
                                // TODO incorporating interprocedural constraints
                            }
                        }
                    }
                };
            }
        }

        let mut graph = ProvenanceConstraintGraph {
            subset: vec![],
            membership: vec![],
        };

        Vis {
            tcx,
            graph: &mut graph,
            borrow_set,
            provenance_set,
            global_borrow_ctxt,
        }
        .visit_body(body);

        graph
    }
}

pub struct BorrowInferenceResults<'tcx> {
    // pub provenance_set: ProvenanceSet,
    pub borrow_set: BorrowSet<'tcx>,
    pub constraint_graph: ProvenanceConstraintGraph,
    pub location_map: DenseLocationMap,
    pub provenance_liveness: ProvenanceLiveness,
    pub killed: Killed,
    pub subset_closure: SubSetClosure,
    pub requires: ProvenanceRequiresLoan,
    pub loan_liveness: LoanLiveness,
    pub invalidates: Invalidates,
    pub errors: Errors,
}

pub fn borrow_inference<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
    global_borrow_ctxt: &GBorrowInferCtxt,
) -> BorrowInferenceResults<'tcx> {
    let tcx = tcx;
    let f = def_id;

    let body = &*tcx
        .mir_drops_elaborated_and_const_checked(f.expect_local())
        .borrow();

    let provenance_set = global_borrow_ctxt.provenances.get(&f).unwrap();
    let borrow_set = body.borrow_set(tcx, &provenance_set, global_borrow_ctxt);
    let location_map = DenseLocationMap::new(body);
    let provenance_liveness =
        compute_provenance_liveness(&location_map, tcx, body, &provenance_set);
    let killed = compute_killed(body, tcx, &location_map, &borrow_set);
    let constraint_graph =
        ProvenanceConstraintGraph::new(tcx, body, &borrow_set, provenance_set, global_borrow_ctxt);
    let subset_closure = compute_subset_closure(provenance_set, &constraint_graph);
    let requires = compute_requires(&borrow_set, &provenance_set, &constraint_graph);
    let loan_liveness = compute_loan_liveness(
        tcx,
        body,
        &borrow_set,
        &location_map,
        &provenance_liveness,
        &requires,
        &killed,
    );
    let invalidates = compute_invalidates(tcx, body, &borrow_set, &location_map);
    let errors = compute_errors(&borrow_set, &loan_liveness, &invalidates);

    BorrowInferenceResults {
        borrow_set,
        location_map,
        provenance_liveness,
        killed,
        constraint_graph,
        subset_closure,
        requires,
        loan_liveness,
        invalidates,
        errors,
    }
}

pub fn dump_borrow_inference_mir<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    inference: &BorrowInferenceResults<'tcx>,
    global_borrow_ctxt: &GBorrowInferCtxt,
    w: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    let BorrowInferenceResults {
        borrow_set,
        location_map,
        provenance_liveness,
        killed: _killed,
        constraint_graph: _constraint_graph,
        subset_closure: _subset_closure,
        requires: _requires,
        loan_liveness,
        invalidates: _invalidates,
        errors,
    } = inference;
    let provenance_set = global_borrow_ctxt
        .provenances
        .get(&body.source.def_id())
        .unwrap();

    use rustc_middle::mir::{PassWhere, pretty::PrettyPrintMirOptions};
    use utils::itertools::Itertools as _;

    rustc_middle::mir::pretty::write_mir_fn(
        tcx,
        body,
        &mut |pass_where, w| match pass_where {
            PassWhere::BeforeLocation(location) => {
                let point_index = location_map.point_from_location(location);
                let live_loans = loan_liveness
                    .row(point_index)
                    .iter()
                    .flat_map(|loans| loans.iter())
                    .map(|loan| format!("{:?}", &borrow_set.loans[loan]))
                    .join(", ");

                w.write_fmt(format_args!("\t// live loans: [{live_loans}]\n",))?;

                Ok(())
            }
            PassWhere::AfterLocation(location) => {
                let point_index = location_map.point_from_location(location);
                let errors = errors
                    .row(point_index)
                    .iter()
                    .flat_map(|loans| loans.iter())
                    .map(|loan| format!("{:?}", &borrow_set.loans[loan]))
                    .join(", ");

                if !errors.is_empty() {
                    let error_notification = format!("errors: [{}]", errors);
                    w.write_fmt(format_args!("\t// {}\n", error_notification.red()))?;
                }

                let live_provenances = provenance_liveness
                    .row(point_index)
                    .iter()
                    .flat_map(|provenances| provenances.iter())
                    .map(|provenance| format!("{:?}", provenance_set.provenance_data[provenance]))
                    .join(", ");

                w.write_fmt(format_args!(
                    "\t// live provenances: [{live_provenances}]\n",
                ))?;

                Ok(())
            }
            _ => Ok(()),
        },
        w,
        PrettyPrintMirOptions {
            include_extra_comments: false,
        },
    )?;

    for point_index in errors.rows() {
        let illegal_accesses = errors
            .row(point_index)
            .iter()
            .flat_map(|loans| loans.iter())
            .map(|loan| format!("{:?}", &borrow_set.loans[loan]))
            .join(", ");

        if illegal_accesses == "" {
            continue;
        }

        writeln!(
            w,
            "illegal accesses: [{illegal_accesses}] @ {:?}",
            location_map.to_location(point_index)
        )?;
    }

    Ok(())
}

pub fn dump_coarse_inferred_bounds(program: &RustProgram, global_borrow_ctxt: &GBorrowInferCtxt) {
    let tcx = program.tcx;

    for f in program.functions.iter() {
        let body = &*program
            .tcx
            .mir_drops_elaborated_and_const_checked(f.expect_local())
            .borrow();

        let provenance_set = &global_borrow_ctxt.provenances[f];
        let return_place = RETURN_PLACE;
        let Some(return_provenance) = provenance_set.local_data[return_place] else {
            continue;
        };
        println!("{} inferred bounds:", program.tcx.def_path_str(f));
        let BorrowInferenceResults { subset_closure, .. } =
            borrow_inference(tcx, *f, &global_borrow_ctxt);

        for arg in body.args_iter() {
            if let Some(arg_provenance) = provenance_set.local_data[arg]
                && subset_closure.contains(arg_provenance, return_provenance)
            {
                for var_debug_info in body.var_debug_info.iter() {
                    if var_debug_info
                        .argument_index
                        .is_some_and(|arg_index| arg_index == arg.as_u32() as u16)
                    {
                        println!("'{}: 'return", var_debug_info.name);
                    }
                }
            }
        }
    }
}

pub fn demote_pointers(
    program: &RustProgram,
    global_borrow_ctxt: &GBorrowInferCtxt,
) -> FxHashMap<DefId, DenseBitSet<Local>> {
    let mut demoted = FxHashMap::default();

    let tcx = program.tcx;

    for f in program.functions.iter() {
        let body = &*program
            .tcx
            .mir_drops_elaborated_and_const_checked(f.expect_local())
            .borrow();

        let BorrowInferenceResults {
            borrow_set, errors, ..
        } = borrow_inference(tcx, *f, &global_borrow_ctxt);

        let mut invalid_loans = DenseBitSet::new_empty(borrow_set.loans.len());
        for row in errors.rows() {
            if let Some(loans) = errors.row(row) {
                invalid_loans.union(loans);
            }
        }

        let mut demoted_locals = DenseBitSet::new_empty(body.local_decls.len());

        for loan in invalid_loans.iter() {
            let borrow_data = &borrow_set.loans[loan];
            match borrow_data.assigned {
                Borrower::AssignStmt(assigned) => {
                    demoted_locals.insert(assigned.local);
                }
                Borrower::CallArg(..) => unimplemented!(),
            }
        }

        demoted.insert(*f, demoted_locals);
    }

    demoted
}

/// Analyse which raw pointer locals within a function can potentially be a mutable references.
/// Currently there is no safety guarantee, as we need to
/// 1. study what formal guarantee can we obtain from our demoting strategy;
/// 2. implement the necessary fixpoint iteration to compute inferred bounds.
pub fn mutable_references_no_guarantee(
    program: &RustProgram,
) -> FxHashMap<DefId, DenseBitSet<Local>> {
    let mut mutabla_references = FxHashMap::default();

    let global_borrow_ctxt = GBorrowInferCtxt::mutable_pointers_only(program);
    let demoted = demote_pointers(program, &global_borrow_ctxt);

    for (&f, demoted) in demoted.iter() {
        let provenance_set = &global_borrow_ctxt.provenances[&f];
        let mut promoted = DenseBitSet::new_empty(demoted.domain_size());
        for (local, local_data) in provenance_set.local_data.iter_enumerated() {
            if local_data.is_some() {
                promoted.insert(local);
            }
        }
        promoted.subtract(demoted);

        mutabla_references.insert(f, promoted);
    }

    mutabla_references
}

#[cfg(test)]
mod test {
    use super::*;
    use rustc_middle::mir::VarDebugInfoContents;

    fn filter_results_for_user_vars(
        tcx: TyCtxt,
        results: FxHashMap<DefId, DenseBitSet<Local>>,
    ) -> FxHashMap<DefId, Vec<String>> {
        results
            .into_iter()
            .map(|(f, results)| {
                let body = &*tcx
                    .mir_drops_elaborated_and_const_checked(f.expect_local())
                    .borrow();

                let mut results_for_user_vars = vec![];

                for var_debug_info in body.var_debug_info.iter() {
                    if let VarDebugInfoContents::Place(place) = &var_debug_info.value {
                        if let Some(local) = place.as_local()
                            && results.contains(local)
                        {
                            results_for_user_vars.push(var_debug_info.name.as_str().to_string());
                        }
                    }
                }
                (f, results_for_user_vars)
            })
            .collect()
    }

    #[test]
    fn test_proof_of_concept() {
        const PROGRAM: &str = "
        unsafe fn f(mut p: *mut i32) -> i32 {
            let mut r1 = p;
            let mut r2 = r1;
            let mut q = r1;
            *q = 1;
            *r1 = 2;
            *r2 = 3;
            *p = 4;
            *p
        }";

        utils::rustc::run_compiler(PROGRAM, |program| {
            let tcx = program.tcx;
            let f = program.functions[0];
            let body = &*tcx
                .mir_drops_elaborated_and_const_checked(f.expect_local())
                .borrow();

            let global_borrow_ctxt = GBorrowInferCtxt::mutable_pointers_only(&program);

            let inference = borrow_inference(tcx, f, &global_borrow_ctxt);

            dump_borrow_inference_mir(
                tcx,
                body,
                &inference,
                &global_borrow_ctxt,
                &mut std::io::stdout(),
            )
            .unwrap();
        })
    }

    #[test]
    fn test_inferred_bounds() {
        const PROGRAM: &str = "
        unsafe fn f(mut p: *mut i32, mut q: *mut i32) -> *mut i32 {
            *p = 3;
            return &raw mut *q;
        }

        unsafe fn g() {
            let mut local1 = 0;
            let mut local2 = 1;
            let mut r = f(&raw mut local1, &raw mut local2);
            *r = 2;
            println!(\"{}\", *r);
        }
        ";

        utils::rustc::run_compiler(PROGRAM, |program| {
            let global_borrow_ctxt = GBorrowInferCtxt::mutable_pointers_only(&program);
            dump_coarse_inferred_bounds(&program, &global_borrow_ctxt);
        })
    }

    #[test]
    fn test_demote_strategy_no_ub() {
        const PROGRAM: &str = "
        unsafe fn f() {
            let mut local = 0i32;
            let x = &mut local as *mut _;
            let y = &mut local as *mut _;
            *x = 1;
            *y = 2;
        }
        ";

        utils::rustc::run_compiler(PROGRAM, |program| {
            let tcx = program.tcx;
            let f = program.functions[0];
            let body = &*tcx
                .mir_drops_elaborated_and_const_checked(f.expect_local())
                .borrow();

            let global_borrow_ctxt = GBorrowInferCtxt::mutable_pointers_only(&program);

            let inference = borrow_inference(tcx, f, &global_borrow_ctxt);

            dump_borrow_inference_mir(
                tcx,
                body,
                &inference,
                &global_borrow_ctxt,
                &mut std::io::stdout(),
            )
            .unwrap();
        })
    }

    #[test]
    fn test_demote_strategy_optimality() {
        const PROGRAM: &str = "
        unsafe fn f() {
            let mut local = 0i32;
            let x = &raw mut local;
            let y = &raw mut local;
            *y = 2;
            *x = 1;
        }
        ";

        utils::rustc::run_compiler(PROGRAM, |program| {
            let tcx = program.tcx;
            let f = program.functions[0];
            let body = &*tcx
                .mir_drops_elaborated_and_const_checked(f.expect_local())
                .borrow();

            let global_borrow_ctxt = GBorrowInferCtxt::mutable_pointers_only(&program);

            let inference = borrow_inference(tcx, f, &global_borrow_ctxt);

            dump_borrow_inference_mir(
                tcx,
                body,
                &inference,
                &global_borrow_ctxt,
                &mut std::io::stdout(),
            )
            .unwrap();
        })
    }

    #[test]
    fn test_callee_bad() {
        const PROGRAM: &str = r#"
#[no_mangle]
pub unsafe extern "C" fn bar(mut p: *mut i32) {}
#[no_mangle]
pub unsafe extern "C" fn foo() {
    let mut x: i32 = 0 as i32;
    let mut p: *mut i32 = &mut x;
    *p = 1 as i32;
    bar(p);
}
        "#;

        utils::rustc::run_compiler(PROGRAM, |program| {
            let tcx = program.tcx;
            let potential_mutable_references = mutable_references_no_guarantee(&program);
            let potential_mutable_references =
                filter_results_for_user_vars(tcx, potential_mutable_references);
            let global_borrow_ctxt = GBorrowInferCtxt::mutable_pointers_only(&program);
            let demoted_pointers = demote_pointers(&program, &global_borrow_ctxt);
            let demoted_pointers = filter_results_for_user_vars(tcx, demoted_pointers);

            for (f, ok_user_vars) in potential_mutable_references.into_iter() {
                let demoted = &demoted_pointers[&f];
                println!(
                    "{}: mut refs = [{}], demoted = [{}]",
                    tcx.def_path_str(f),
                    ok_user_vars.join(", "),
                    demoted.join(", ")
                );
            }
        });
    }

    #[test]
    fn test_is_null_bad() {
        const PROGRAM: &str = r#"
#[no_mangle]
pub unsafe extern "C" fn foo(mut p: *mut i32) {
    if !p.is_null() {
        *p = 1 as i32;
    }
}
        "#;

        utils::rustc::run_compiler(PROGRAM, |program| {
            let tcx = program.tcx;
            let potential_mutable_references = mutable_references_no_guarantee(&program);
            let potential_mutable_references =
                filter_results_for_user_vars(tcx, potential_mutable_references);
            let global_borrow_ctxt = GBorrowInferCtxt::mutable_pointers_only(&program);
            let demoted_pointers = demote_pointers(&program, &global_borrow_ctxt);
            let demoted_pointers = filter_results_for_user_vars(tcx, demoted_pointers);

            for (f, ok_user_vars) in potential_mutable_references.into_iter() {
                let demoted = &demoted_pointers[&f];
                println!(
                    "{}: mut refs = [{}], demoted = [{}]",
                    tcx.def_path_str(f),
                    ok_user_vars.join(", "),
                    demoted.join(", ")
                );
            }
        });
    }

    #[test]
    fn test_ownership_transfer_like() {
        const PROGRAM: &str = "
        unsafe fn f() {
            let mut local = 0i32;
            let mut p = &raw mut local;
            let q = &raw mut *p;
            *q = 1;
            p = &raw mut *q;
            *p = 2;
        }
        ";

        utils::rustc::run_compiler(PROGRAM, |program| {
            let tcx = program.tcx;
            let f = program.functions[0];
            let body = &*tcx
                .mir_drops_elaborated_and_const_checked(f.expect_local())
                .borrow();

            let global_borrow_ctxt = GBorrowInferCtxt::mutable_pointers_only(&program);

            let inference = borrow_inference(tcx, f, &global_borrow_ctxt);

            dump_borrow_inference_mir(
                tcx,
                body,
                &inference,
                &global_borrow_ctxt,
                &mut std::io::stdout(),
            )
            .unwrap();
        })
    }

    #[test]
    fn test_json_c() {
        const PROGRAM: &str = r#"
        unsafe fn json_parse_object() {
            let previous = 0 as *mut usize;

            {
                let element = previous;
                (*element) = 0;
            }

            previous.is_null();
        }"#;

        utils::rustc::run_compiler(PROGRAM, |program| {
            let tcx = program.tcx;
            let f = program.functions[0];
            let body = &*tcx
                .mir_drops_elaborated_and_const_checked(f.expect_local())
                .borrow();

            let global_borrow_ctxt = GBorrowInferCtxt::mutable_pointers_only(&program);

            let inference = borrow_inference(tcx, f, &global_borrow_ctxt);

            dump_borrow_inference_mir(
                tcx,
                body,
                &inference,
                &global_borrow_ctxt,
                &mut std::io::stdout(),
            )
            .unwrap();
        })
    }

    #[test]
    fn smoke_test_libtree() {
        utils::rustc::run_compiler(utils::rustc::SourceCode::Libtree, |program| {
            let tcx = program.tcx;
            let potential_mutable_references = mutable_references_no_guarantee(&program);
            let potential_mutable_references =
                filter_results_for_user_vars(tcx, potential_mutable_references);
            let global_borrow_ctxt = GBorrowInferCtxt::mutable_pointers_only(&program);
            let demoted_pointers = demote_pointers(&program, &global_borrow_ctxt);
            let demoted_pointers = filter_results_for_user_vars(tcx, demoted_pointers);

            for (f, ok_user_vars) in potential_mutable_references.into_iter() {
                let demoted = &demoted_pointers[&f];
                println!(
                    "{}: mut refs = [{}], demoted = [{}]",
                    tcx.def_path_str(f),
                    ok_user_vars.join(", "),
                    demoted.join(", ")
                );
            }
        });
    }

    #[test]
    fn smoke_test_buffer() {
        utils::rustc::run_compiler(utils::rustc::SourceCode::Buffer, |program| {
            let tcx = program.tcx;
            let potential_mutable_references = mutable_references_no_guarantee(&program);
            let potential_mutable_references =
                filter_results_for_user_vars(tcx, potential_mutable_references);
            let global_borrow_ctxt = GBorrowInferCtxt::mutable_pointers_only(&program);
            let demoted_pointers = demote_pointers(&program, &global_borrow_ctxt);
            let demoted_pointers = filter_results_for_user_vars(tcx, demoted_pointers);

            for (f, ok_user_vars) in potential_mutable_references.into_iter() {
                let demoted = &demoted_pointers[&f];
                println!(
                    "{}: mut refs = [{}], demoted = [{}]",
                    tcx.def_path_str(f),
                    ok_user_vars.join(", "),
                    demoted.join(", ")
                );
            }
        });
    }
}
