//! Borrow inference

use rustc_index::{IndexVec, bit_set::SparseBitMatrix};
use rustc_middle::{
    mir::{
        Body, HasLocalDecls, Local, Location, Operand, Place, PlaceElem, Rvalue, Terminator,
        TerminatorKind, visit::Visitor,
    },
    ty::TyCtxt,
};
use rustc_mir_dataflow::fmt::DebugWithContext;
use utils::rustc_hash::FxHashMap;

mod errors;
mod invalidates;
mod killed;
mod loan_liveness;
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
    fn provenance_set(&self) -> ProvenanceSet;
}

impl HasProvenanceSet for Body<'_> {
    fn provenance_set(&self) -> ProvenanceSet {
        let body = self;
        let mut local_data = IndexVec::from_elem_n(None, body.local_decls.len());
        let mut provenance_data = IndexVec::new();

        for (provenance, (local, local_decl)) in local_data
            .iter_mut()
            .zip(body.local_decls.iter_enumerated())
        {
            if local_decl.ty.is_any_ptr() {
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

pub struct BorrowData<P> {
    location: Location,
    path: P,
}

impl<P> std::fmt::Debug for BorrowData<P>
where
    P: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?} @ {:?}", self.path, self.location))
    }
}

pub struct BorrowSet<P> {
    loans: IndexVec<Loan, BorrowData<P>>,
    location_map: FxHashMap<Location, Loan>,
    local_map: SparseBitMatrix<Local, Loan>,
}

pub trait HasBorrowSet<'tcx, P> {
    fn borrow_set(&self, tcx: TyCtxt<'tcx>) -> BorrowSet<P>;
}

impl<'tcx> HasBorrowSet<'tcx, Place<'tcx>> for Body<'tcx> {
    fn borrow_set(&self, tcx: TyCtxt<'tcx>) -> BorrowSet<Place<'tcx>> {
        struct Vis<'tcx, 'this, D> {
            loans: IndexVec<Loan, BorrowData<Place<'tcx>>>,
            location_map: FxHashMap<Location, Loan>,
            local_decl: &'this D,
            tcx: TyCtxt<'tcx>,
        }
        impl<'tcx, 'this, D: HasLocalDecls<'tcx>> Visitor<'tcx> for Vis<'tcx, 'this, D> {
            fn visit_rvalue(&mut self, rvalue: &Rvalue<'tcx>, location: Location) {
                let rvalue_ty = rvalue.ty(self.local_decl, self.tcx);
                if !rvalue_ty.is_any_ptr() {
                    return self.super_rvalue(rvalue, location);
                }

                match rvalue {
                    Rvalue::Ref(_, _, place) | Rvalue::RawPtr(_, place) => {
                        let loan = self.loans.push(BorrowData {
                            location,
                            path: *place,
                        });
                        self.location_map.insert(location, loan);
                    }
                    Rvalue::CopyForDeref(place)
                    | Rvalue::Use(Operand::Copy(place) | Operand::Move(place)) => {
                        let loan = self.loans.push(BorrowData {
                            location,
                            path: place.project_deeper(&[PlaceElem::Deref], self.tcx),
                        });
                        self.location_map.insert(location, loan);
                    }
                    _ => {}
                }
            }

            fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
                if matches!(terminator.kind, TerminatorKind::Call { .. }) {
                    unimplemented!("calculate borrow set for calls")
                }
            }
        }

        let mut vis = Vis {
            loans: IndexVec::new(),
            location_map: FxHashMap::default(),
            local_decl: self,
            tcx,
        };
        vis.visit_body(self);

        let Vis {
            loans,
            location_map,
            ..
        } = vis;

        let mut local_map = SparseBitMatrix::new(loans.len());

        for (loan, borrow_data) in loans.iter_enumerated() {
            local_map.insert(borrow_data.path.local, loan);
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
    location: Location,
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
        borrow_set: &BorrowSet<Place<'tcx>>,
        provenance_set: &ProvenanceSet,
    ) -> Self {
        struct Vis<'this, 'tcx> {
            graph: &'this mut ProvenanceConstraintGraph,
            borrow_set: &'this BorrowSet<Place<'tcx>>,
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
                    path: rhs,
                } = &self.borrow_set.loans[loan];

                let Some(lhs) = place.as_local() else {
                    return self.super_assign(place, rvalue, location);
                };
                let lhs_provenance = self.provenance_set.local_data[lhs].unwrap();

                assert!(rhs.is_indirect_first_projection());

                self.graph.membership.push(MembershipConstraint {
                    loan,
                    provenance: lhs_provenance,
                });

                if rhs
                    .projection
                    .iter()
                    .all(|projection| matches!(projection, PlaceElem::Deref))
                {
                    let rhs_provenance = self.provenance_set.local_data[rhs.local].unwrap();
                    self.graph.subset.push(SubsetConstraint {
                        sup: lhs_provenance,
                        sub: rhs_provenance,
                        location,
                    });
                }
            }

            fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, _location: Location) {
                if matches!(terminator.kind, TerminatorKind::Call { .. }) {
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

#[cfg(test)]
mod test {
    use rustc_middle::mir::{PassWhere, pretty::PrettyPrintMirOptions};
    use rustc_mir_dataflow::points::DenseLocationMap;
    use utils::itertools::Itertools;

    use crate::borrow::{
        HasBorrowSet, HasProvenanceSet, ProvenanceConstraintGraph, errors::compute_errors,
        invalidates::compute_invalidates, killed::compute_killed,
        loan_liveness::compute_loan_liveness, provenance_liveness::compute_provenance_liveness,
        requires::compute_requires,
    };

    #[test]
    fn play() {
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

            let provenance_set = body.provenance_set();
            let borrow_set = body.borrow_set(program.tcx);

            let _ = ProvenanceConstraintGraph::new(body, &borrow_set, &provenance_set);
            let location_map = DenseLocationMap::new(body);
            let provenance_liveness =
                compute_provenance_liveness(&location_map, tcx, body, &provenance_set);
            let killed = compute_killed(body, tcx, &location_map, &borrow_set);
            let requires = compute_requires(body, &borrow_set, &provenance_set);
            let loan_liveness = compute_loan_liveness(
                tcx,
                body,
                &borrow_set,
                &provenance_set,
                &location_map,
                &provenance_liveness,
                &requires,
                &killed,
            );
            let invalidates = compute_invalidates(tcx, body, &borrow_set, &location_map);
            let errors = compute_errors(&borrow_set, &loan_liveness, &invalidates);

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
                            .map(|provenance| {
                                format!("{:?}", provenance_set.provenance_data[provenance])
                            })
                            .join(", ");

                        w.write_fmt(format_args!(
                            "\t// live provenances: [{live_provenances}]\n",
                        ))?;

                        Ok(())
                    }
                    _ => Ok(()),
                },
                &mut std::io::stdout(),
                PrettyPrintMirOptions {
                    include_extra_comments: false,
                },
            )
            .unwrap();

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

                println!(
                    "illegal accesses: [{illegal_accesses}] @ {:?}",
                    location_map.to_location(point_index)
                );
            }
        })
    }
}
