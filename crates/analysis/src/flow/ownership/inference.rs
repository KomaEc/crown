use either::Either::{Left, Right};
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{
        visit::Visitor, BasicBlock, BasicBlockData, Body, Local, Location, Operand, Place, Rvalue,
        Statement, Terminator,
    },
    ty::{Ty, TyCtxt},
};
use utils::data_structure::assoc::AssocExt;

use super::{
    access_path::{AccessPaths, Path},
    constraint::{Constraint, Database, OwnershipToken, StorageMode},
    Ctxt,
};
use crate::flow::{
    def_use::{Def, DefUseChain, Inspect, Update, UseKind},
    RichLocation,
    SSAIdx,
};

pub struct Analysis<'analysis, 'tcx, const K_LIMIT: usize, Mode: StorageMode, DB> {
    ctxt: &'analysis mut Ctxt<K_LIMIT, Mode, DB>,
    /// `Local -> SSAIdx -> first token`
    tokens: &'analysis IndexVec<Local, IndexVec<SSAIdx, OwnershipToken>>,
    flow_chain: &'analysis DefUseChain,
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

fn ownership_tokens<'a, const K_LIMIT: usize>(
    path: &Path<OwnershipToken>,
    expected_depth: usize,
    access_paths: &'a AccessPaths<K_LIMIT>,
    ty: Ty,
) -> impl Iterator<Item = OwnershipToken> + 'a {
    if expected_depth == path.depth() {
        Left(path.base..path.base + path.num_pointers_reachable())
    } else {
        assert!(expected_depth < path.depth());
        let base = path.base;
        Right(
            access_paths
                .patch_up(path.depth(), path.depth() - expected_depth, ty)
                .map(move |offset| base + offset),
        )
    }
}

impl<'tcx, const K_LIMIT: usize, Mode, DB> Analysis<'_, 'tcx, K_LIMIT, Mode, DB>
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
        let min_depth = std::cmp::min(path1.depth(), path2.depth());
        let path1 = path1.transpose();
        let path2 = path2.transpose();

        for x in ownership_tokens(&path1.r#use, min_depth, &self.ctxt.access_paths, ty) {
            self.ctxt.database.add(
                Constraint::Assume { x, sign: false },
                &mut self.ctxt.storage,
            )
        }
        for (x, y) in ownership_tokens(&path1.def, min_depth, &self.ctxt.access_paths, ty).zip(
            ownership_tokens(&path2.r#use, min_depth, &self.ctxt.access_paths, ty),
        ) {
            self.ctxt
                .database
                .add(Constraint::Equal { x, y }, &mut self.ctxt.storage)
        }
        for x in ownership_tokens(&path2.def, min_depth, &self.ctxt.access_paths, ty) {
            self.ctxt.database.add(
                Constraint::Assume { x, sign: false },
                &mut self.ctxt.storage,
            )
        }
    }

    fn transfer(&mut self, path1: &Path<ExpandedBase>, path2: &Path<ExpandedBase>, ty: Ty<'tcx>) {
        let min_depth = std::cmp::min(path1.depth(), path2.depth());
        let path1 = path1.transpose();
        let path2 = path2.transpose();
        for x in ownership_tokens(&path1.r#use, min_depth, &self.ctxt.access_paths, ty) {
            self.ctxt.database.add(
                Constraint::Assume { x, sign: false },
                &mut self.ctxt.storage,
            )
        }

        for (x, y, z) in itertools::izip!(
            ownership_tokens(&path1.def, min_depth, &self.ctxt.access_paths, ty),
            ownership_tokens(&path2.def, min_depth, &self.ctxt.access_paths, ty),
            ownership_tokens(&path2.r#use, min_depth, &self.ctxt.access_paths, ty)
        ) {
            self.ctxt
                .database
                .add(Constraint::Linear { x, y, z }, &mut self.ctxt.storage)
        }
    }
}

impl<'tcx, const K_LIMIT: usize, Mode, DB> Visitor<'tcx> for Analysis<'_, 'tcx, K_LIMIT, Mode, DB>
where
    Mode: StorageMode,
    DB: Database<Mode>,
{
    fn visit_basic_block_data(&mut self, block: BasicBlock, data: &BasicBlockData<'tcx>) {
        for &(local, ref phi_node) in self.flow_chain.join_points[block].iter() {
            // let def = phi_node.lhs;
            // for r#use in phi_node.rhs.iter().copied() {

            // }
            todo!(
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
            Rvalue::Use(operand) => match operand {
                Operand::Copy(rhs) => {
                    let Some(rhs) = self.path(rhs, location).map(|path| self.expand(&path)) else {
                        // if `rhs` is not a pointer, then `lhs` is unconstrained
                        return;
                    };
                    self.transfer(&lhs, &rhs, ty)
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
            Rvalue::Ref(_, _, _) => todo!(),
            Rvalue::ThreadLocalRef(_) => todo!(),
            Rvalue::AddressOf(_, _) => todo!(),
            Rvalue::Len(_) => todo!(),
            Rvalue::Cast(_, _, _) => todo!(),
            Rvalue::BinaryOp(_, _) => todo!(),
            Rvalue::CheckedBinaryOp(_, _) => todo!(),
            Rvalue::NullaryOp(_, _) => todo!(),
            Rvalue::UnaryOp(_, _) => todo!(),
            Rvalue::Discriminant(_) => todo!(),
            Rvalue::Aggregate(_, _) => todo!(),
            Rvalue::ShallowInitBox(_, _) => todo!(),
            Rvalue::CopyForDeref(_) => todo!(),
        }
        todo!("infer assignment {place:?} = {rvalue:?} at {location:?}")
    }

    fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        self.super_statement(statement, location)
    }

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        self.super_terminator(terminator, location)
    }
}
