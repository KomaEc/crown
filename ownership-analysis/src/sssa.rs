//! Syntactic SSA form
//! Normal SSA-form cannot be constructed without pointer analysis, as
//! the def (or use) of expressions like `*p` is indirect, and cannot
//! be extracted from syntax-level.
//! In Syntactic SSA form, a place `*(*p).f` is treated as a syntactic
//! local variable `**p`.

use std::collections::VecDeque;

use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::mir::{visit::Visitor, BasicBlock, Body, Local};
use smallvec::SmallVec;

use core::{body_ext::BodyExt, place_ext::PlaceExt};

use super::{constants::PHI_NODES_SIZE_HINT, OwnershipAnalysisCtxt};

fn syntactic_ssa_form<'octxt, 'tcx>(
    body: &Body<'tcx>,
    octxt: &OwnershipAnalysisCtxt<'octxt, 'tcx>,
) -> PhiNodeInsertionPoints<u32> {
    let mut inserted = PhiNodeInsertionPoints::from_raw(IndexVec::from_elem(
        BasicBlockInsersionPoints::new(),
        body.basic_blocks(),
    ));
    let dominance_frontier = body.calculate_dominance_frontier();

    fn collect_def_sites_for<'tcx>(body: &Body<'tcx>) -> IndexVec<Local, Vec<BitSet<BasicBlock>>> {
        use rustc_middle::mir::{
            visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext},
            Place,
        };
        struct Vis(IndexVec<Local, Vec<BitSet<BasicBlock>>>, Option<BasicBlock>);
        impl<'tcx> Visitor<'tcx> for Vis {
            fn visit_basic_block_data(
                &mut self,
                block: BasicBlock,
                data: &rustc_middle::mir::BasicBlockData<'tcx>,
            ) {
                self.1 = Some(block);
                self.super_basic_block_data(block, data)
            }

            fn visit_place(
                &mut self,
                place: &Place<'tcx>,
                context: PlaceContext,
                _: rustc_middle::mir::Location,
            ) {
                let place_abs = place.r#abstract();

                if matches!(
                    context,
                    PlaceContext::NonMutatingUse(
                        NonMutatingUseContext::Copy | NonMutatingUseContext::Move
                    ) | PlaceContext::MutatingUse(MutatingUseContext::Store)
                ) {
                    self.0[place_abs.base_local][place_abs.num_derefs as usize]
                        .insert(self.1.unwrap());
                }
            }
        }
        let mut vis = Vis(
            max_deref_levels
                .into_iter()
                .map(|deref_level| {
                    vec![BitSet::new_empty(body.basic_blocks.len()); deref_level as usize]
                })
                .collect(),
            None,
        );
        vis.visit_body(body);
        vis.0
    }
    let place_abs_def_sites = collect_def_sites_for(body);

    let mut already_added = BitSet::new_empty(body.basic_blocks().len());
    for (a, def_sites_family) in place_abs_def_sites.iter_enumerated() {
        for (deref_level, def_sites) in def_sites_family.iter().enumerate() {
            let deref_level = deref_level as u32;
            let mut work_list = def_sites.iter().collect::<VecDeque<_>>();
            while let Some(bb) = work_list.pop_front() {
                for &bb_f in &dominance_frontier[bb] {
                    if !already_added.contains(bb_f) {
                        inserted[bb_f].push(a, deref_level);
                        already_added.insert(bb_f);
                        if !def_sites.contains(bb_f) {
                            work_list.push_back(bb_f);
                        }
                    }
                }
            }

            already_added.clear()
        }
    }

    // for basic_block_insertions in inserted.iter_mut() {
    //     basic_block_insertions.data.sort_by(
    //         |&(a, al), &(b, bl)| {
    //             if a == b {
    //                 al.cmp(&bl)
    //             } else {
    //                 a.cmp(&b)
    //             }
    //         },
    //     );

    //     let mut new_local_found = true;
    //     let mut cur: Local;
    //     basic_block_insertions.data.retain(|&mut (a, al)| {

    //     });
    // }
    inserted
}

#[derive(Clone, Debug)]
pub struct PhiNodeInsertionPoints<Payload> {
    insertion_points: IndexVec<BasicBlock, BasicBlockInsersionPoints<Payload>>,
}

impl<Payload> PhiNodeInsertionPoints<Payload> {
    pub fn from_raw(raw: IndexVec<BasicBlock, BasicBlockInsersionPoints<Payload>>) -> Self {
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
    data: SmallVec<[(Local, Payload); PHI_NODES_SIZE_HINT]>,
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
