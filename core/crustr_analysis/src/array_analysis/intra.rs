use std::marker::PhantomData;

use crate::{
    array_analysis::{Constraint, ConstraintIdx, CrateLambdaCtxt, Lambda, LambdaData},
    def_use::DefUseCategorisable,
    ssa::{
        body_ext::{BodyExt, PhiNodeInserted},
        rename::{HasSSANameHandler, HasSSARenameState, SSANameHandler, SSARename, SSARenameState},
    },
};
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_index::vec::{Idx, IndexVec};
use rustc_middle::{
    mir::{
        visit::{PlaceContext, Visitor},
        BasicBlock, Body, Local, Location, Operand, Place, PlaceElem, PlaceRef, ProjectionElem,
        Rvalue, Statement, Terminator, TerminatorKind,
    },
    ty::{subst::GenericArgKind, TyCtxt},
};
use rustc_target::abi::VariantIdx;
use smallvec::SmallVec;

use super::{CrateSummary, LambdaCtxt};

impl<'analysis, 'tcx> CrateSummary<'analysis, 'tcx> {
    pub fn infer<DefUse: DefUseCategorisable>(&mut self) {
        for (body_idx, &did) in self.bodies.iter().enumerate() {
            let body = self.tcx.optimized_mir(did);
            let insertion_points = body.compute_phi_node::<DefUse>(self.tcx);

            let mut infer: Infer<DefUse, ()> = Infer {
                ctxt: InferCtxt::new(
                    self.tcx,
                    body_idx,
                    body,
                    &mut self.lambda_ctxt,
                    &insertion_points,
                ),
                ssa_state: SSARenameState::new(&body.local_decls),
                extra_handlers: (),
                _marker: PhantomData,
            };

            infer.rename_body(body, &insertion_points);

            let body_ctxt = LambdaCtxt {
                local: infer.ctxt.lambda_ctxt.local,
                local_nested: infer.ctxt.lambda_ctxt.local_nested,
            };
            self.lambda_ctxt.body_ctxt.push(body_ctxt);
        }
    }
}

impl CrateLambdaCtxt {
    pub fn intra_view(&mut self, body: &Body, body_idx: usize) -> CrateLambdaCtxtIntraView<'_> {
        let (local, local_nested) = body
            .local_decls
            .iter_enumerated()
            .map(|(local, local_decl)| {
                let ty = local_decl.ty;
                (
                    // vec![],
                    ty.is_any_ptr()
                        .then(|| {
                            let lambda = self.lambda_map.push(LambdaData::Local {
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
                            self.lambda_map.push(LambdaData::LocalNested {
                                body: body_idx,
                                base: local,
                                nested_level: level,
                            })
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .unzip();

        CrateLambdaCtxtIntraView {
            body: body_idx,
            lambda_map: &mut self.lambda_map,
            field_defs: &self.field_defs,
            local,
            local_nested,
        }
    }
}

pub struct Infer<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
{
    pub ctxt: InferCtxt<'infercx, 'tcx>,
    pub ssa_state: SSARenameState<Local>,
    pub extra_handlers: Handler,
    pub _marker: PhantomData<*const DefUse>,
}

impl<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
    HasSSARenameState<Local> for Infer<'infercx, 'tcx, DefUse, Handler>
{
    #[inline]
    fn state(&mut self) -> &mut SSARenameState<Local> {
        &mut self.ssa_state
    }
}

impl<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
    SSANameHandler for Infer<'infercx, 'tcx, DefUse, Handler>
{
    type Output = Option<Lambda>;

    fn handle_def(&mut self, local: Local, idx: usize, location: Location) -> Self::Output {
        self.extra_handlers.handle_def(local, idx, location);
        self.ctxt.body.local_decls[local]
            .ty
            .is_any_ptr()
            .then(|| self.ctxt.handle_def(local, idx, location))
    }

    fn handle_use(&mut self, local: Local, idx: usize, location: Location) -> Self::Output {
        self.extra_handlers.handle_use(local, idx, location);
        self.ctxt.body.local_decls[local]
            .ty
            .is_any_ptr()
            .then(|| self.ctxt.handle_use(local, idx, location))
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        self.extra_handlers
            .handle_def_at_phi_node(local, idx, block);
        self.ctxt.body.local_decls[local]
            .ty
            .is_any_ptr()
            .then(|| self.ctxt.handle_def_at_phi_node(local, idx, block));
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, pos: usize) {
        self.extra_handlers
            .handle_use_at_phi_node(local, idx, block, pos);
        self.ctxt.body.local_decls[local]
            .ty
            .is_any_ptr()
            .then(|| self.ctxt.handle_use_at_phi_node(local, idx, block, pos));
    }
}

impl<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
    HasSSANameHandler for Infer<'infercx, 'tcx, DefUse, Handler>
{
    type Handler = Self;
    #[inline]
    fn ssa_name_handler(&mut self) -> &mut Self::Handler {
        self
    }
}

impl<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
    SSARename<'tcx> for Infer<'infercx, 'tcx, DefUse, Handler>
{
    type DefUse = DefUse;

    #[inline]
    fn rename_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        self.visit_statement(statement, location)
    }

    #[inline]
    fn rename_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        self.visit_terminator(terminator, location)
    }
}

impl<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
    Infer<'infercx, 'tcx, DefUse, Handler>
{
    fn process_projections(
        &mut self,
        base: Local,
        length: usize,
        projections: impl Iterator<Item = (PlaceRef<'tcx>, PlaceElem<'tcx>)> + DoubleEndedIterator,
    ) -> Lambda {
        projections
            .rev()
            .enumerate()
            .find_map(|(nested_level, (base, proj))| {
                log::debug!("visiting projection {:?}", proj);
                if let ProjectionElem::Field(field, _) = proj {
                    let place_ty = base.ty(self.ctxt.body, self.ctxt.tcx);
                    let ty = place_ty.ty;
                    let variant_idx = place_ty.variant_index.unwrap_or(VariantIdx::new(0));
                    let adt_def = ty.ty_adt_def().unwrap();
                    Some(
                        self.ctxt.lambda_ctxt.field_defs[&adt_def.did][variant_idx][field.index()]
                            [nested_level],
                    )
                } else {
                    None
                }
            })
            .unwrap_or_else(|| self.ctxt.lambda_ctxt.local_nested[base][length - 1])
    }

    fn process_rhs(&mut self, place: &Place<'tcx>, location: Location) -> Lambda {
        debug_assert!(place.ty(self.ctxt.body, self.ctxt.tcx).ty.is_any_ptr());

        log::debug!("processing rhs {:?}", place);

        let ssa_idx = self.r#use(place.local);
        let lambda = self
            .ssa_name_handler()
            .handle_use(place.local, ssa_idx, location);

        if place.projection.is_empty() {
            return lambda.unwrap();
        }

        self.process_projections(
            place.local,
            place.projection.len(),
            place.iter_projections(),
        )
    }

    fn process_lhs(&mut self, place: &Place<'tcx>, location: Location) -> Lambda {
        debug_assert!(place.ty(self.ctxt.body, self.ctxt.tcx).ty.is_any_ptr());
        log::debug!("processing lhs {:?}", place);

        if place.projection.is_empty() {
            let ssa_idx = self.define(place.local);
            self.ssa_name_handler()
                .handle_def(place.local, ssa_idx, location)
                .unwrap()
        } else {
            self.process_projections(
                place.local,
                place.projection.len(),
                place.iter_projections(),
            )
        }
    }
}

impl<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
    Visitor<'tcx> for Infer<'infercx, 'tcx, DefUse, Handler>
{
    fn visit_local(&mut self, &local: &Local, context: PlaceContext, location: Location) {
        if let Some(def_use) = DefUse::categorize(context) {
            if DefUse::defining(def_use) {
                let i = self.define(local);
                self.ssa_name_handler().handle_def(local, i, location);
            } else if DefUse::using(def_use) {
                let i = self.r#use(local);
                self.ssa_name_handler().handle_use(local, i, location);
            }
        }
    }

    fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        log::debug!("visiting statement {:?}", statement);
        self.super_statement(statement, location)
    }

    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        if place.ty(self.ctxt.body, self.ctxt.tcx).ty.is_any_ptr() {
            if let Rvalue::Use(Operand::Move(rhs)) | Rvalue::Use(Operand::Copy(rhs)) = rvalue {
                let lhs = self.process_lhs(place, location);
                let rhs = self.process_rhs(rhs, location);
                let constraint = Constraint(lhs, rhs);
                log::debug!("generate constraint {}", constraint);
                self.ctxt.constraints.push(constraint);
            }
        } else {
            self.super_assign(place, rvalue, location)
        }
    }

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        // log::debug!("visiting terminator {:?}", terminator);
        if let TerminatorKind::Call {
            ref func,
            args: _,
            destination: _,
            cleanup: _,
            from_hir_call: _,
            fn_span: _,
        } = terminator.kind
        {
            log::debug!("calling function {:?}", func);
        }
        self.super_terminator(terminator, location)
    }
}

pub struct CrateLambdaCtxtIntraView<'intracx> {
    pub body: usize,
    pub lambda_map: &'intracx mut IndexVec<Lambda, LambdaData>,
    pub field_defs: &'intracx FxHashMap<DefId, IndexVec<VariantIdx, Vec<Vec<Lambda>>>>,
    pub local: IndexVec<Local, Vec<Lambda>>,
    pub local_nested: IndexVec<Local, Vec<Lambda>>,
}

impl<'intracx> CrateLambdaCtxtIntraView<'intracx> {
    pub fn generate_local(&mut self, base: Local, ssa_idx: usize) -> Lambda {
        let lambda = self.lambda_map.push(LambdaData::Local {
            body: self.body,
            base,
            ssa_idx,
        });
        self.local[base].push(lambda);
        log::debug!("generate {:?} for Local {:?}^{}", lambda, base, ssa_idx);
        lambda
    }
}

pub struct InferCtxt<'infercx, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub body: &'infercx Body<'tcx>,
    lambda_ctxt: CrateLambdaCtxtIntraView<'infercx>,
    phi_joins: IndexVec<BasicBlock, SmallVec<[(Local, Vec<Lambda>); 3]>>,
    constraints: IndexVec<ConstraintIdx, Constraint>,
}

impl<'infercx, 'tcx> InferCtxt<'infercx, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        body_idx: usize,
        body: &'infercx Body<'tcx>,
        lambda_ctxt: &'infercx mut CrateLambdaCtxt,
        insertion_points: &PhiNodeInserted,
    ) -> Self {
        let phi_joins = insertion_points
            .iter()
            .map(|vec| {
                vec.iter()
                    .map(|&local| (local, vec![]))
                    .collect::<SmallVec<_>>()
            })
            .collect::<IndexVec<_, _>>();
        InferCtxt {
            tcx,
            body,
            lambda_ctxt: lambda_ctxt.intra_view(body, body_idx),
            phi_joins,
            constraints: IndexVec::new(),
        }
        .debug_initialise()
    }

    #[cfg(debug_assertions)]
    pub fn debug_initialise(self) -> Self {
        const INDENT: &str = "   in f, ";
        log::debug!(
            "for function {}:",
            self.tcx.def_path_debug_str(self.body.source.def_id())
        );
        for (local, x) in self.lambda_ctxt.local_nested.iter_enumerated() {
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
        for (local, y) in self.lambda_ctxt.local.iter_enumerated() {
            if self.body.local_decls[local].ty.is_any_ptr() {
                assert_eq!(y.len(), 1);
                log::debug!("{}{:?}^0 ==> {:?}", INDENT, local, y[0])
            } else {
                assert!(y.is_empty())
            }
        }
        self
    }
}

impl<'infercx, 'tcx> SSANameHandler for InferCtxt<'infercx, 'tcx> {
    type Output = Lambda;

    fn handle_def(&mut self, local: Local, idx: usize, _location: Location) -> Self::Output {
        log::debug!("IntraCtxt defining {:?}^{}", local, idx);
        debug_assert!(self.body.local_decls[local].ty.is_any_ptr());
        self.lambda_ctxt.generate_local(local, idx)
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        let lambda = self.lambda_ctxt.generate_local(local, idx);
        self.phi_joins[block]
            .iter_mut()
            .find_map(|&mut (this_local, ref mut lambdas)| {
                (this_local == local).then(|| lambdas.push(lambda))
            })
            .unwrap()
    }

    fn handle_use(&mut self, local: Local, idx: usize, _location: Location) -> Self::Output {
        log::debug!("IntraCtxt using {:?}^{}", local, idx);
        debug_assert!(self.body.local_decls[local].ty.is_any_ptr());
        self.lambda_ctxt.local[local][idx]
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, _pos: usize) {
        let lambda = self.lambda_ctxt.local[local][idx];
        self.phi_joins[block]
            .iter_mut()
            .find_map(|&mut (this_local, ref mut lambdas)| {
                (this_local == local).then(|| lambdas.push(lambda))
            })
            .unwrap()
    }
}
