//! Extension methods for Body<'tcx>

use rustc_index::bit_set::{BitSet, HybridBitSet};
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{BasicBlock, Body, Local};
use rustc_middle::ty::TyCtxt;
use rustc_mir_dataflow::{Analysis, ResultsCursor};
use smallvec::SmallVec;
use std::collections::VecDeque;
use std::marker::PhantomData;

use crate::def_use::{DefSites, DefSitesGatherer, IsDefUse};
use crate::liveness_analysis::MaybeLiveLocals;
use crate::ty_ext::TyExt;

const DOMINATOR_FRONTIER_ON_STACK_SIZE: usize = 3;
const PHI_NODE_INSERTED_ON_STACK_SIZE: usize = 3;

pub type DominanceFrontier =
    IndexVec<BasicBlock, SmallVec<[BasicBlock; DOMINATOR_FRONTIER_ON_STACK_SIZE]>>;

#[derive(Clone, Debug)]
pub struct PhiNodeInsertionPoints<Payload> {
    insertion_points: IndexVec<BasicBlock, BasicBlockInsersionPoints<Payload>>,
}

impl<Payload> PhiNodeInsertionPoints<Payload> {
    pub fn new(raw: IndexVec<BasicBlock, BasicBlockInsersionPoints<Payload>>) -> Self {
        PhiNodeInsertionPoints {
            insertion_points: raw,
        }
    }

    pub fn into_iter(self) -> impl Iterator<Item = BasicBlockInsersionPoints<Payload>> {
        self.insertion_points.into_iter()
    }

    pub fn repack<F, U>(&self, f: F) -> PhiNodeInsertionPoints<U>
    where
        F: Fn(Local, &Payload) -> U,
    {
        self.iter()
            .map(|bb_insertion_points| bb_insertion_points.repack(&f))
            .collect::<IndexVec<_, _>>()
            .into()
    }

    pub fn filter_repack<U, F>(&self, f: F) -> PhiNodeInsertionPoints<U>
    where
        F: Fn(Local, &Payload) -> Option<U>,
    {
        self.iter()
            .map(|bb_insertion_points| bb_insertion_points.filter_repack(&f))
            .collect::<IndexVec<_, _>>()
            .into()
    }
}

impl<Payload> From<IndexVec<BasicBlock, BasicBlockInsersionPoints<Payload>>>
    for PhiNodeInsertionPoints<Payload>
{
    fn from(insertion_points: IndexVec<BasicBlock, BasicBlockInsersionPoints<Payload>>) -> Self {
        Self { insertion_points }
    }
}

/// `PhiNodeInsertionPoints<Payload>` should act completely the same as
/// `IndexVec<BasicBlock, BasicBlockInsersionPoints<Payload>>`, so we implement
/// `Deref`
impl<Payload> std::ops::Deref for PhiNodeInsertionPoints<Payload> {
    type Target = IndexVec<BasicBlock, BasicBlockInsersionPoints<Payload>>;

    fn deref(&self) -> &Self::Target {
        &self.insertion_points
    }
}

impl<Payload> std::ops::DerefMut for PhiNodeInsertionPoints<Payload> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.insertion_points
    }
}

#[derive(Clone, Debug)]
pub struct BasicBlockInsersionPoints<Payload> {
    data: SmallVec<[(Local, Payload); PHI_NODE_INSERTED_ON_STACK_SIZE]>,
}

impl<Payload> FromIterator<(Local, Payload)> for BasicBlockInsersionPoints<Payload> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (Local, Payload)>>(iter: T) -> Self {
        Self {
            data: iter.into_iter().collect::<SmallVec<_>>(),
        }
    }
}

impl<T> BasicBlockInsersionPoints<T> {
    pub fn new() -> Self {
        BasicBlockInsersionPoints {
            data: SmallVec::new(),
        }
    }

    #[inline]
    pub fn push(&mut self, local: Local, payload: T) {
        self.data.push((local, payload))
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub fn locals(&self) -> impl Iterator<Item = Local> + '_ {
        self.data.iter().map(|&(local, _)| local)
    }

    #[inline]
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.data.into_iter().map(|(_, payload)| payload)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().map(|(_, payload)| payload)
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().map(|(_, payload)| payload)
    }

    #[inline]
    pub fn into_iter_enumerated(self) -> impl Iterator<Item = (Local, T)> {
        self.data.into_iter()
    }

    #[inline]
    pub fn iter_enumerated(&self) -> impl Iterator<Item = (Local, &T)> {
        self.data.iter().map(|(local, payload)| (*local, payload))
    }

    #[inline]
    pub fn iter_enumerated_mut(&mut self) -> impl Iterator<Item = (Local, &mut T)> {
        self.data
            .iter_mut()
            .map(|(local, payload)| (*local, payload))
    }
}

impl<T> std::ops::Index<Local> for BasicBlockInsersionPoints<T> {
    type Output = T;

    fn index(&self, local: Local) -> &Self::Output {
        self.data
            .iter()
            .find_map(|&(this_local, ref t)| (this_local == local).then(|| t))
            .expect(&format!("no phi node for {:?}", local))
    }
}

impl<T> std::ops::IndexMut<Local> for BasicBlockInsersionPoints<T> {
    fn index_mut(&mut self, local: Local) -> &mut Self::Output {
        self.data
            .iter_mut()
            .find_map(|&mut (this_local, ref mut t)| (this_local == local).then(|| t))
            .expect(&format!("no phi node for {:?}", local))
    }
}

impl<T> BasicBlockInsersionPoints<T> {
    pub fn repack<F, U>(&self, f: F) -> BasicBlockInsersionPoints<U>
    where
        F: Fn(Local, &T) -> U,
    {
        self.iter_enumerated()
            .map(|(local, t)| (local, f(local, t)))
            .collect::<BasicBlockInsersionPoints<_>>()
    }

    pub fn filter_repack<U, F>(&self, f: F) -> BasicBlockInsersionPoints<U>
    where
        F: Fn(Local, &T) -> Option<U>,
    {
        self.iter_enumerated()
            .filter_map(|(local, t)| Some((local, f(local, t)?)))
            .collect::<BasicBlockInsersionPoints<_>>()
    }
}

pub trait BodyExt<'tcx> {
    fn dominance_frontier(&self) -> DominanceFrontier;
    fn compute_phi_node<DefUse: IsDefUse>(
        &self,
        tcx: TyCtxt<'tcx>,
    ) -> PhiNodeInsertionPoints<PhantomData<*const DefUse>>;
}

impl<'tcx> BodyExt<'tcx> for Body<'tcx> {
    fn dominance_frontier(&self) -> DominanceFrontier {
        let dominators = self.dominators();
        let mut df = IndexVec::from_elem(
            HybridBitSet::new_empty(self.basic_blocks().len()),
            self.basic_blocks(),
        );

        for bb in self.basic_blocks().indices() {
            let preds = &self.predecessors()[bb];
            if preds.len() >= 2 {
                // if `bb` is a join point
                for &bb_p in preds {
                    let mut runner = bb_p;
                    while runner != dominators.immediate_dominator(bb) {
                        df[runner].insert(bb);
                        runner = dominators.immediate_dominator(runner)
                    }
                }
            }
        }
        IndexVec::from_iter(df.iter().map(|set| set.iter().collect::<SmallVec<_>>()))
    }

    fn compute_phi_node<DefUse: IsDefUse>(
        &self,
        tcx: TyCtxt<'tcx>,
    ) -> PhiNodeInsertionPoints<PhantomData<*const DefUse>> {
        minimal_ssa_form(
            self,
            DefSitesGatherer::<DefUse>::gather(self),
            self.dominance_frontier(),
            liveness_result(tcx, self),
        )
    }
}

/// Compute phi node insertion points for minimal SSA form.
/// Along with normal construction, check for liveness on entry
/// before inserting phi node to avoid redundant nodes.
/// FIXME: this is not a full scale dead code elimination, as adding
/// phi node will make more locals to be dead. So this needs to be a
/// worklist algorithm!
pub fn minimal_ssa_form<'tcx, DefUse: IsDefUse>(
    body: &Body<'tcx>,
    mut def_sites: DefSites,
    dominance_frontier: DominanceFrontier,
    mut liveness: ResultsCursor<'_, 'tcx, MaybeLiveLocals>,
) -> PhiNodeInsertionPoints<PhantomData<*const DefUse>> {
    let mut inserted: PhiNodeInsertionPoints<PhantomData<*const DefUse>> =
        PhiNodeInsertionPoints::new(IndexVec::from_elem(
            BasicBlockInsersionPoints::new(),
            body.basic_blocks(),
        ));
    let def_sites = def_sites
        .iter_mut()
        .map(|sites| {
            let mut sites = sites
                .iter_mut()
                .map(|location| location.block)
                .collect::<Vec<_>>();
            sites.sort();
            sites.dedup();
            sites
        })
        .collect::<IndexVec<Local, _>>();
    for a in body.local_decls.indices() {
        let mut already_added = BitSet::new_empty(body.basic_blocks().len());
        let mut work_list = VecDeque::from_iter(def_sites[a].iter().map(|&bb| bb));
        while let Some(bb) = work_list.pop_front() {
            for &bb_f in &dominance_frontier[bb] {
                liveness.seek_to_block_start(bb_f);
                if !already_added.contains(bb_f)
                    && (body.local_decls[a].ty.is_ptr_but_not_fn_ptr()
                        // matches!(
                        //     body.basic_blocks()[bb_f].terminator().kind,
                        //     rustc_middle::mir::TerminatorKind::Return
                        // ) 
                        || liveness.get().contains(a))
                {
                    inserted[bb_f].push(a, PhantomData);
                    assert!(already_added.insert(bb_f));
                    if !def_sites[a].contains(&bb_f) {
                        work_list.push_back(bb_f);
                    }
                }
            }
        }
    }
    inserted
}

fn liveness_result<'a, 'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &'a Body<'tcx>,
) -> ResultsCursor<'a, 'tcx, MaybeLiveLocals> {
    let liveness = MaybeLiveLocals
        .into_engine(tcx, body)
        .iterate_to_fixpoint()
        .into_results_cursor(body);
    liveness
}
