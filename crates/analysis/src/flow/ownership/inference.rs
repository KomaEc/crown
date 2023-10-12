use either::Either::{Left, Right};
use rustc_middle::{
    mir::{
        visit::Visitor, AggregateKind, BasicBlock, BasicBlockData, BinOp, Body, BorrowKind, Local,
        Location, Operand, Place, Rvalue, Statement, Terminator,
    },
    ty::{Ty, TyCtxt},
};
use utils::data_structure::assoc::AssocExt;

use super::{
    access_path::{AccessPaths, Path},
    constraint::{Constraint, Database, OwnershipToken, StorageMode},
    flow_chain, Ctxt,
};
use crate::flow::{
    def_use::{Def, DefUseChain, Inspect, Update, UseKind},
    LocalMap, RichLocation, SSAIdx,
};

pub struct Intraprocedural<'analysis, 'tcx, const K_LIMIT: usize, Mode: StorageMode, DB> {
    ctxt: &'analysis mut Ctxt<K_LIMIT, Mode, DB>,
    /// `Local -> SSAIdx -> first token`
    // tokens: IndexVec<Local, IndexVec<SSAIdx, OwnershipToken>>,
    tokens: LocalMap<OwnershipToken>,
    flow_chain: DefUseChain,
    body: &'analysis Body<'tcx>,
    tcx: TyCtxt<'tcx>,
}

type Base = (Local, UseKind<SSAIdx>);
#[cfg(not(debug_assertions))]
const _: () = assert!(std::mem::size_of::<Base>() == 12);
type ExpandedBase = Update<OwnershipToken>;
#[cfg(not(debug_assertions))]
const _: () = assert!(std::mem::size_of::<ExpandedBase>() == 8);

impl<T> Path<Update<T>> {
    pub fn transpose(self) -> Update<Path<T>> {
        let Path { base, projections } = self;
        Update {
            r#use: Path::new(base.r#use, projections),
            def: Path::new(base.def, projections),
        }
    }
}

/// The set of ownership tokens of `path` under `context`
fn ownership_tokens<'a, const K_LIMIT: usize>(
    path: &Path<OwnershipToken>,
    context: usize,
    access_paths: &'a AccessPaths<K_LIMIT>,
    ty: Ty,
) -> impl Iterator<Item = OwnershipToken> + 'a {
    let base = path.base;
    let projection_offset = path.offset();
    if context == path.depth() {
        Left(base + projection_offset..base + projection_offset + path.num_pointers_reachable())
    } else {
        assert!(context > path.depth());
        Right(
            access_paths
                .patch_up(path.depth(), context - path.depth(), ty)
                .map(move |offset| base + projection_offset + offset),
        )
    }
}

impl<'analysis, 'tcx, const K_LIMIT: usize, Mode, DB>
    Intraprocedural<'analysis, 'tcx, K_LIMIT, Mode, DB>
where
    Mode: StorageMode,
    DB: Database<Mode>,
{
    pub fn new(
        ctxt: &'analysis mut Ctxt<K_LIMIT, Mode, DB>,
        body: &'analysis Body<'tcx>,
        tcx: TyCtxt<'tcx>,
    ) -> Self {
        let flow_chain = flow_chain(body, &ctxt.access_paths);
        use utils::data_structure::vec_vec::VecVec;
        let mut map = VecVec::with_indices_capacity(body.local_decls.len() + 1);

        // TODO monotonicity constraints!
        for (local, def_locs) in flow_chain.def_locs.iter_enumerated() {
            let size = ctxt
                .access_paths
                .path(&Place::from(local), body)
                .num_pointers_reachable();
            tracing::debug!("initialising {local:?} with {size} variables");
            tracing::error!("monotonicity constraints");
            for _ in def_locs.indices() {
                map.push_element(ctxt.database.new_tokens(size).start);
            }
            map.complete_cur_vec();
        }

        let map = map.complete();
        let tokens = LocalMap { map };
        Self {
            ctxt,
            tokens,
            flow_chain,
            body,
            tcx,
        }
    }
}

impl<'tcx, const K_LIMIT: usize, Mode, DB> Intraprocedural<'_, 'tcx, K_LIMIT, Mode, DB>
where
    Mode: StorageMode,
    DB: Database<Mode>,
{
    /// If the path is a `Some`, then its size > 0
    fn path(&self, place: &Place<'tcx>, location: Location) -> Option<Path<Base>> {
        let path = self.ctxt.access_paths.path(place, self.body);
        let base = self.flow_chain.uses[location]
            .get_by_key(&place.local)
            .copied()?;
        assert!(path.num_pointers_reachable() > 0);
        Some(path.map_base(|()| (place.local, base)))
    }

    fn expand(&mut self, path: &Path<Base>) -> Path<ExpandedBase> {
        path.map_base(|(local, base)| {
            match base {
                Inspect(ssa_idx) => {
                    let def_loc = self.flow_chain.def_locs[local][ssa_idx];
                    match def_loc {
                        RichLocation::Entry => unreachable!("Inspecting entry definition. How?"),
                        RichLocation::Phi(block) => {
                            let phi_node = self.flow_chain.join_points[block]
                                .get_by_key(&local)
                                .expect("Definition location does not have phi node. How?");
                            unimplemented!(
                                "How to get the pre-state of {:?} at phi node {block:?}. Potentially, \
                                store two sets of tokens when defining phi-node. The first set represents \
                                the post-state, while the second set represents the pre-state and is to be \
                                unified with rhs of a phi node.",
                                phi_node.lhs
                            );
                        }
                        RichLocation::Mir(location) => {
                            let update = self.flow_chain.uses[location]
                                .get_by_key(&local)
                                .copied()
                                .and_then(|use_kind| use_kind.update())
                                .expect("Definition location does not define. How?");
                            update
                        }
                    }
                }
                Def(update) => update,
            }
            .map(|ssa_idx| self.tokens[local][ssa_idx])
        })
    }

    /// path1 = move path2
    fn r#move(&mut self, path1: &Path<ExpandedBase>, path2: &Path<ExpandedBase>, ty: Ty<'tcx>) {
        tracing::debug!("move constraint: {path1:?} = move {path2:?}");
        let max_depth = std::cmp::max(path1.depth(), path2.depth());
        let path1 = path1.transpose();
        let path2 = path2.transpose();
        for x in ownership_tokens(&path1.r#use, max_depth, &self.ctxt.access_paths, ty) {
            self.ctxt.database.add(
                Constraint::Assume { x, sign: false },
                &mut self.ctxt.storage,
            )
        }
        for (x, y) in ownership_tokens(&path1.def, max_depth, &self.ctxt.access_paths, ty).zip(
            ownership_tokens(&path2.r#use, max_depth, &self.ctxt.access_paths, ty),
        ) {
            self.ctxt
                .database
                .add(Constraint::Equal { x, y }, &mut self.ctxt.storage)
        }
        for x in ownership_tokens(&path2.def, max_depth, &self.ctxt.access_paths, ty) {
            self.ctxt.database.add(
                Constraint::Assume { x, sign: false },
                &mut self.ctxt.storage,
            )
        }
    }

    fn transfer(&mut self, path1: &Path<ExpandedBase>, path2: &Path<ExpandedBase>, ty: Ty<'tcx>) {
        tracing::debug!("transfer constraint: {path1:?} = {path2:?}");
        let max_depth = std::cmp::max(path1.depth(), path2.depth());
        let path1 = path1.transpose();
        let path2 = path2.transpose();
        for x in ownership_tokens(&path1.r#use, max_depth, &self.ctxt.access_paths, ty) {
            self.ctxt.database.add(
                Constraint::Assume { x, sign: false },
                &mut self.ctxt.storage,
            )
        }

        for (x, y, z) in itertools::izip!(
            ownership_tokens(&path1.def, max_depth, &self.ctxt.access_paths, ty),
            ownership_tokens(&path2.def, max_depth, &self.ctxt.access_paths, ty),
            ownership_tokens(&path2.r#use, max_depth, &self.ctxt.access_paths, ty)
        ) {
            self.ctxt
                .database
                .add(Constraint::Linear { x, y, z }, &mut self.ctxt.storage)
        }
    }
}

impl<'tcx, const K_LIMIT: usize, Mode, DB> Visitor<'tcx>
    for Intraprocedural<'_, 'tcx, K_LIMIT, Mode, DB>
where
    Mode: StorageMode,
    DB: Database<Mode>,
{
    fn visit_basic_block_data(&mut self, block: BasicBlock, data: &BasicBlockData<'tcx>) {
        tracing::debug!("inferring basicblock {block:?}");
        for &(local, ref phi_node) in self.flow_chain.join_points[block].iter() {
            // let def = phi_node.lhs;
            // for r#use in phi_node.rhs.iter().copied() {

            // }
            tracing::error!(
                "Equate the ownership variables at phi-node {local:?}: {:?} = {}",
                phi_node.lhs,
                phi_node
                    .rhs
                    .iter()
                    .map(|ssa_idx| format!("{ssa_idx:?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
        self.super_basic_block_data(block, data);
    }

    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        // expand the path directly as it must be a definition
        let Some(lhs) = self.path(place, location).map(|path| self.expand(&path)) else {
            // if `lhs` is not a pointer, then `rhs` is unconstrained
            return;
        };
        let ty = place.ty(self.body, self.tcx).ty;
        match rvalue {
            // cast is unsafe anyway
            Rvalue::Use(operand) | Rvalue::Cast(_, operand, _) => match operand {
                Operand::Copy(rhs) => {
                    let Some(rhs) = self.path(rhs, location).map(|path| self.expand(&path)) else {
                        // if `rhs` is not a pointer, then `lhs` is unconstrained
                        return;
                    };
                    self.transfer(&lhs, &rhs, ty);
                }
                Operand::Move(rhs) => {
                    let Some(rhs) = self.path(rhs, location).map(|path| self.expand(&path)) else {
                        // if `rhs` is not a pointer, then `lhs` is unconstrained
                        return;
                    };
                    self.r#move(&lhs, &rhs, ty);
                }
                Operand::Constant(_) => return,
            },
            Rvalue::Repeat(_, _) => todo!(),
            Rvalue::Ref(_, BorrowKind::Mut { .. }, lender) => {
                todo!()
            }
            Rvalue::Ref(_, BorrowKind::Shared, lender) => {
                todo!()
            }
            Rvalue::AddressOf(_, _) => todo!(),
            Rvalue::BinaryOp(BinOp::Offset, box (operand1, operand2)) => todo!(),
            Rvalue::BinaryOp(BinOp::Eq, _) => todo!(),
            Rvalue::BinaryOp(_, _) => {
                unreachable!("LHS of an arithmetic binary operation contains pointers. How?")
            }
            Rvalue::CheckedBinaryOp(_, _) => {
                unreachable!("LHS of a checked arithmetic binary operation contains pointers. How?")
            }
            Rvalue::UnaryOp(_, _) => {
                unreachable!("LHS of an unary operation contains pointers. How?")
            }
            Rvalue::Discriminant(_) => {
                unreachable!("LHS of a discriminant expression contains pointers. How?")
            }
            Rvalue::Aggregate(box AggregateKind::Array(_), values) => {
                todo!()
            }
            Rvalue::Aggregate(box AggregateKind::Adt(..), values) => {
                todo!()
            }
            Rvalue::CopyForDeref(rhs) => {
                let Some(rhs) = self.path(rhs, location).map(|path| self.expand(&path)) else {
                    // if `rhs` is not a pointer, then `lhs` is unconstrained
                    return;
                };
                self.r#move(&lhs, &rhs, ty);
            }
            Rvalue::Ref(_, BorrowKind::Shallow, _)
            | Rvalue::ThreadLocalRef(_)
            | Rvalue::Len(_)
            | Rvalue::Aggregate(..)
            | Rvalue::NullaryOp(_, _)
            | Rvalue::ShallowInitBox(_, _) => unimplemented!("Rvalue type {:?}", rvalue),
        }
    }

    fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        tracing::debug!("infer statement {statement:?}");
        self.super_statement(statement, location)
    }

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        tracing::debug!("infer terminator {:?}", &terminator.kind);
        self.super_terminator(terminator, location)
    }
}
