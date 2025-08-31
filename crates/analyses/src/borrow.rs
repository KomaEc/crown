//! Borrow inference

use rustc_hir::def_id::DefId;
use rustc_index::{
    IndexVec,
    bit_set::{DenseBitSet, SparseBitMatrix},
};
use rustc_middle::{
    mir::{
        Body, HasLocalDecls, Local, Location, Operand, Place, PlaceElem, Rvalue, Terminator,
        TerminatorKind, visit::Visitor,
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
    },
    type_qualifier::mutability_analysis,
};

const ALLOW_INTERPROCEDURAL: bool = true;

mod errors;
mod invalidates;
mod killed;
mod loan_liveness;
mod places_conflict;
mod provenance_liveness;
mod requires;

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
    assigned: Place<'tcx>,
}

impl std::fmt::Debug for BorrowData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?} @ {:?}", self.borrowed, self.location))
    }
}

pub struct BorrowSet<'tcx> {
    loans: IndexVec<Loan, BorrowData<'tcx>>,
    location_map: FxHashMap<Location, Loan>,
    local_map: SparseBitMatrix<Local, Loan>,
}

pub trait HasBorrowSet<'tcx> {
    fn borrow_set(&self, tcx: TyCtxt<'tcx>, provenance_set: &ProvenanceSet) -> BorrowSet<'tcx>;
}

impl<'tcx> HasBorrowSet<'tcx> for Body<'tcx> {
    fn borrow_set(&self, tcx: TyCtxt<'tcx>, provenance_set: &ProvenanceSet) -> BorrowSet<'tcx> {
        struct Vis<'tcx, 'this, D> {
            loans: IndexVec<Loan, BorrowData<'tcx>>,
            location_map: FxHashMap<Location, Loan>,
            local_decl: &'this D,
            tcx: TyCtxt<'tcx>,
            provenance_set: &'this ProvenanceSet,
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
                            assigned: *lhs,
                        });
                        self.location_map.insert(location, loan);
                    }
                    Rvalue::CopyForDeref(place)
                    | Rvalue::Use(Operand::Copy(place) | Operand::Move(place)) => {
                        let loan = self.loans.push(BorrowData {
                            location,
                            borrowed: place.project_deeper(&[PlaceElem::Deref], self.tcx),
                            assigned: *lhs,
                        });
                        self.location_map.insert(location, loan);
                    }
                    _ => {}
                }
            }

            fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, _location: Location) {
                if matches!(terminator.kind, TerminatorKind::Call { .. }) && !ALLOW_INTERPROCEDURAL
                {
                    unimplemented!("calculate borrow set for calls")
                }
            }
        }

        let mut vis = Vis {
            loans: IndexVec::new(),
            location_map: FxHashMap::default(),
            local_decl: self,
            tcx,
            provenance_set,
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
    pub fn new<'tcx>(
        body: &Body<'tcx>,
        borrow_set: &BorrowSet<'tcx>,
        provenance_set: &ProvenanceSet,
    ) -> Self {
        struct Vis<'this, 'tcx> {
            graph: &'this mut ProvenanceConstraintGraph,
            borrow_set: &'this BorrowSet<'tcx>,
            provenance_set: &'this ProvenanceSet,
        }

        impl<'tcx> Visitor<'tcx> for Vis<'_, 'tcx> {
            fn visit_assign(
                &mut self,
                place: &Place<'tcx>,
                rvalue: &Rvalue<'tcx>,
                location: Location,
            ) {
                let Some(&loan) = self.borrow_set.location_map.get(&location) else {
                    return self.super_assign(place, rvalue, location);
                };
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

            fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, _location: Location) {
                if matches!(terminator.kind, TerminatorKind::Call { .. }) && !ALLOW_INTERPROCEDURAL
                {
                    unimplemented!("infer provenance constraints for calls")
                }
            }
        }

        let mut graph = ProvenanceConstraintGraph {
            subset: vec![],
            membership: vec![],
        };

        Vis {
            graph: &mut graph,
            borrow_set,
            provenance_set,
        }
        .visit_body(body);

        graph
    }
}

pub struct BorrowInferenceResults<'tcx> {
    pub provenance_set: ProvenanceSet,
    pub borrow_set: BorrowSet<'tcx>,
    pub location_map: DenseLocationMap,
    pub provenance_liveness: ProvenanceLiveness,
    pub killed: Killed,
    pub requires: ProvenanceRequiresLoan,
    pub loan_liveness: LoanLiveness,
    pub invalidates: Invalidates,
    pub errors: Errors,
}

pub fn borrow_inference<'tcx, I>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
    is_candidate: I,
) -> BorrowInferenceResults<'tcx>
where
    I: Fn(Local) -> bool,
{
    let tcx = tcx;
    let f = def_id;

    let body = &*tcx
        .mir_drops_elaborated_and_const_checked(f.expect_local())
        .borrow();

    let provenance_set = body.provenance_set(is_candidate);
    let borrow_set = body.borrow_set(tcx, &provenance_set);
    let location_map = DenseLocationMap::new(body);
    let provenance_liveness =
        compute_provenance_liveness(&location_map, tcx, body, &provenance_set);
    let killed = compute_killed(body, tcx, &location_map, &borrow_set);
    let requires = compute_requires(body, &borrow_set, &provenance_set);
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
        provenance_set,
        borrow_set,
        location_map,
        provenance_liveness,
        killed,
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
    w: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    let BorrowInferenceResults {
        provenance_set,
        borrow_set,
        location_map,
        provenance_liveness,
        killed: _killed,
        requires: _requires,
        loan_liveness,
        invalidates: _invalidates,
        errors,
    } = inference;

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

                w.write_fmt(format_args!("\t// errors: [{errors}]\n",))?;

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

pub fn demote_pointers(program: &RustProgram) -> FxHashMap<DefId, DenseBitSet<Local>> {
    let mut demoted = FxHashMap::default();

    let tcx = program.tcx;

    let mutability_results = mutability_analysis(program);

    for f in program.functions.iter() {
        let body = &*program
            .tcx
            .mir_drops_elaborated_and_const_checked(f.expect_local())
            .borrow();

        let mutability_results = mutability_results
            .function_body_facts(f)
            .collect::<IndexVec<Local, _>>();

        let BorrowInferenceResults {
            borrow_set, errors, ..
        } = borrow_inference(tcx, *f, |local| {
            mutability_results[local]
                .first()
                .is_some_and(|mutability| mutability.is_mutable())
        });

        let mut invalid_loans = DenseBitSet::new_empty(borrow_set.loans.len());
        for row in errors.rows() {
            if let Some(loans) = errors.row(row) {
                invalid_loans.union(loans);
            }
        }

        let mut demoted_locals = DenseBitSet::new_empty(body.local_decls.len());

        for loan in invalid_loans.iter() {
            let borrow_data = &borrow_set.loans[loan];
            demoted_locals.insert(borrow_data.assigned.local);
        }

        demoted.insert(*f, demoted_locals);
    }

    demoted
}

#[cfg(test)]
mod test {
    use rustc_middle::mir::VarDebugInfoContents;

    use crate::borrow::{borrow_inference, demote_pointers, dump_borrow_inference_mir};

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

            let inference = borrow_inference(tcx, f, |_| true);

            dump_borrow_inference_mir(tcx, body, &inference, &mut std::io::stdout()).unwrap();
        })
    }

    #[test]
    fn smoke_test_libtree() {
        utils::rustc::run_compiler(utils::rustc::SourceCode::Libtree, |program| {
            let tcx = program.tcx;
            // for f in program.functions.iter().copied() {
            //     if tcx.def_path_str(f) == "src::libtree::parse_ld_so_conf" {
            //         let body = &*tcx
            //             .mir_drops_elaborated_and_const_checked(f.expect_local())
            //             .borrow();
            //         let inference = borrow_inference(tcx, f, |_| true);
            //         dump_borrow_inference_mir(tcx, body, &inference, &mut std::io::stdout())
            //             .unwrap();
            //     }
            // }
            let demoted = demote_pointers(&program);
            for f in program.functions.iter() {
                let body = &*tcx
                    .mir_drops_elaborated_and_const_checked(f.expect_local())
                    .borrow();

                let demoted_locals = &demoted[f];

                let mut demoted_user_vars = vec![];

                for var_debug_info in body.var_debug_info.iter() {
                    if let VarDebugInfoContents::Place(place) = &var_debug_info.value {
                        if let Some(local) = place.as_local()
                            && demoted_locals.contains(local)
                        {
                            demoted_user_vars.push(var_debug_info.name.as_str().to_string());
                        }
                    }
                }

                use utils::itertools::Itertools as _;
                println!(
                    "{}: [{}]",
                    tcx.def_path_str(f),
                    demoted_user_vars.into_iter().join(", ")
                )
            }
        });
    }
}
