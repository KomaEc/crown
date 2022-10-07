use common::data_structure::vec_array::{VecArray, VecArrayConstruction};
use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor},
        BasicBlock, BasicBlockData, Body, CastKind, Local, LocalInfo, Location, Place, Rvalue,
    },
    ty::TyCtxt,
};
use smallvec::SmallVec;

use crate::{
    ptr::Measurable,
    ssa::{constraint::local_has_non_zero_measure, state::SSAIdx},
    CrateCtxt,
};

/// TODO
/// 1. `def_sites` -> `maybe_consume_sites`. This is to avoid rebuild phi node join
/// points.
/// 2. `short_live_range: IndexVec<Local, Option<BasicBlock>>`
pub struct Definitions {
    /// BasicBlock -> statement_index -> possible definitions
    ///
    /// We've made an assumption that a local can only be used or defined
    /// once in a statement/terminator
    consumes: VecArray<SmallVec<[(Local, Consume<SSAIdx>); 2]>>,
    pub maybe_consume_sites: IndexVec<Local, BitSet<BasicBlock>>,
    /// Caching the results of calling [local_has_non_zero_measure]
    pub maybe_owning: BitSet<Local>,
}

/// [Voidable] types are only allowed to appear in the generic parameter of [`Consume`].
/// [`Consume`] is always valid in its first field `r#use`, but sometimes not in its second
/// field `def`. Changing the type of `def` to [`Option<T>`] is somehow awkward and hard to
/// work with.
pub trait Voidable: Clone + std::fmt::Debug {
    const VOID: Self;
    fn is_void(&self) -> bool;
    #[inline]
    fn valid(self) -> Option<Self> {
        (!self.is_void()).then_some(self)
    }
}

#[derive(Clone, Debug)]
pub struct Consume<T> {
    pub r#use: T,
    pub def: T,
}

impl<T> Consume<T> {
    #[inline]
    pub fn repack<U>(self, mut f: impl FnMut(T) -> U) -> Consume<U> {
        let r#use = f(self.r#use);
        let def = f(self.def);
        Consume { r#use, def }
    }
}

impl<T: Voidable> Consume<T> {
    #[inline]
    pub fn is_use(&self) -> bool {
        !self.r#use.is_void() && self.def.is_void()
    }

    #[inline]
    pub fn is_invalid(&self) -> bool {
        self.r#use.is_void() && self.def.is_void()
    }

    #[inline]
    pub fn valid(self) -> Option<Self> {
        (!self.is_invalid()).then_some(self)
    }

    /// Invalid argument preserving map. This is so because `f` may not work on invalid
    /// argument
    #[inline]
    pub fn map_valid<U: Voidable>(self, f: impl Fn(T) -> U) -> Consume<U> {
        self.map(|voidable| {
            if let Some(obj) = voidable {
                f(obj)
            } else {
                U::VOID
            }
        })
    }

    #[inline]
    pub fn map<U>(self, f: impl Fn(Option<T>) -> U) -> Consume<U> {
        let def = f(self.def.valid());
        let r#use = f(self.r#use.valid());
        Consume { r#use, def }
    }
}

impl<T: Copy> Copy for Consume<T> {}

impl Consume<SSAIdx> {
    #[inline]
    pub const fn new() -> Self {
        Consume {
            r#use: SSAIdx::INIT,
            def: SSAIdx::INIT,
        }
    }

    #[inline]
    pub const fn pure_use() -> Self {
        Consume {
            r#use: SSAIdx::INIT,
            def: SSAIdx::VOID,
        }
    }

    #[inline]
    pub fn enable_def(&mut self) {
        self.def = SSAIdx::INIT;
    }
}

const _: () = assert!(0 == std::mem::size_of::<Consume<()>>());

#[derive(Clone, Copy, Debug)]
pub enum RichLocation {
    Entry,
    Phi(BasicBlock),
    Mir(Location),
}

impl From<Location> for RichLocation {
    fn from(location: Location) -> Self {
        RichLocation::Mir(location)
    }
}

/// In ownership analysis, use happens only at definition
pub struct ConsumeChain {
    /// ssa index for each consumption
    pub consumes: VecArray<SmallVec<[(Local, Consume<SSAIdx>); 2]>>,
    /// location of each definition
    ///
    /// Those locals with empty entries definitely do not contain pointers
    pub locs: IndexVec<Local, IndexVec<SSAIdx, RichLocation>>,
}

impl ConsumeChain {
    pub fn new(body: &Body, definitions: Definitions) -> Self {
        let Definitions {
            consumes,
            maybe_owning,
            ..
        } = definitions;

        // Notice: this has to be in accordance with NameState.stack
        let locs = body
            .local_decls
            .indices()
            .map(|local| {
                maybe_owning
                    .contains(local)
                    .then(|| IndexVec::from_raw(vec![RichLocation::Entry]))
                    .unwrap_or_default()
                // .unwrap_or_else(IndexVec::new)
            })
            .collect();

        ConsumeChain { consumes, locs }
    }

    pub fn reset(&mut self) {
        for locs in &mut self.locs {
            if !locs.is_empty() {
                locs.truncate(1);
            }
        }
    }

    /// Note that return place is never finalised
    pub fn to_finalise(&self) -> impl Iterator<Item = Local> + '_ {
        self.locs
            .iter_enumerated()
            .skip(1)
            .filter_map(|(local, locs)| (!locs.is_empty()).then_some(local))
    }

    #[inline]
    pub fn of_block(&self, block: BasicBlock) -> &[SmallVec<[(Local, Consume<SSAIdx>); 2]>] {
        &self.consumes[block.index()]
    }

    #[inline]
    pub fn of_location(&self, location: Location) -> &SmallVec<[(Local, Consume<SSAIdx>); 2]> {
        let Location {
            block,
            statement_index,
        } = location;
        &self.consumes[block.index()][statement_index]
    }
}

pub fn initial_definitions<'tcx>(body: &Body<'tcx>, crate_ctxt: &CrateCtxt<'tcx>) -> Definitions {
    let tcx = crate_ctxt.tcx;
    let mut maybe_consume_sites = IndexVec::from_elem(
        BitSet::new_empty(body.basic_blocks.len()),
        &body.local_decls,
    );

    let mut consumes = VecArray::with_indices_capacity(body.basic_blocks.len());

    struct Vis<'me, 'tcx> {
        maybe_consume_sites: &'me mut IndexVec<Local, BitSet<BasicBlock>>,
        consumes: &'me mut VecArrayConstruction<SmallVec<[(Local, Consume<SSAIdx>); 2]>>,
        consumes_in_cur_stmt: SmallVec<[(Local, Consume<SSAIdx>); 2]>,
        body: &'me Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        crate_ctxt: &'me CrateCtxt<'tcx>,
    }
    // println!("visiting {:?}", body.source.def_id());
    impl<'me, 'tcx> Visitor<'tcx> for Vis<'me, 'tcx> {
        fn visit_basic_block_data(
            &mut self,
            block: BasicBlock,
            data: &rustc_middle::mir::BasicBlockData<'tcx>,
        ) {
            // println!("visiting {:?}", block);

            let BasicBlockData {
                statements,
                terminator,
                is_cleanup: _,
            } = data;

            let mut index = 0;
            for statement in statements {
                let location = Location {
                    block,
                    statement_index: index,
                };
                self.visit_statement(statement, location);
                let defs_in_cur_stmt = std::mem::take(&mut self.consumes_in_cur_stmt);
                self.consumes.add_item_to_array(defs_in_cur_stmt);
                index += 1;
            }

            if let Some(terminator) = terminator {
                let location = Location {
                    block,
                    statement_index: index,
                };
                self.visit_terminator(terminator, location);
                let defs_in_cur_stmt = std::mem::take(&mut self.consumes_in_cur_stmt);
                self.consumes.add_item_to_array(defs_in_cur_stmt);
            }
            self.consumes.done_with_array();
        }

        fn visit_rvalue(&mut self, rvalue: &Rvalue<'tcx>, location: Location) {
            // ignore rvalues of the following kinds
            match rvalue {
                Rvalue::Cast(CastKind::PointerFromExposedAddress, operand, _)
                    if operand.place().is_some() =>
                {
                    return
                }
                Rvalue::BinaryOp(_, _)
                | Rvalue::CheckedBinaryOp(_, _)
                | Rvalue::UnaryOp(_, _)
                | Rvalue::NullaryOp(_, _)
                | Rvalue::Aggregate(_, _)
                | Rvalue::Discriminant(_)
                | Rvalue::Len(_)
                | Rvalue::ShallowInitBox(_, _)
                | Rvalue::ThreadLocalRef(_)
                | Rvalue::Repeat(_, _) => return,
                _ => {}
            }

            self.super_rvalue(rvalue, location)
        }

        // Note that we didn't re-implement visit_local. This is because return place should not
        // be counted as a consumption at return clause.
        fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, location: Location) {
            // Deinit and SetDiscriminant are not definitions

            if !matches!(
                context,
                PlaceContext::NonMutatingUse(
                    NonMutatingUseContext::Copy | NonMutatingUseContext::Move
                ) | PlaceContext::MutatingUse(MutatingUseContext::Call | MutatingUseContext::Store)
            ) {
                return;
            }

            let ty = place.ty(self.body, self.tcx).ty;
            let local_info = self.body.local_decls[place.local].local_info.as_deref();

            if self.crate_ctxt.contains_ptr(ty)
                && !matches!(local_info, Some(LocalInfo::DerefTemp))
                // if a local type is union, we do not generate its usage, therefore, direct use/def or
                // dereferences of unions are treated as unsafe sources/sinks during infer
                && !self.body.local_decls[place.local].ty.is_union()
            {
                // println!("defining {:?} at {:?}", place.local, location);
                let consume = if place.is_indirect() {
                    Consume::pure_use()
                } else {
                    Consume::new()
                };
                self.maybe_consume_sites[place.local].insert(location.block);
                self.consumes_in_cur_stmt.push((place.local, consume));
            }
        }
    }

    Vis {
        maybe_consume_sites: &mut maybe_consume_sites,
        consumes: &mut consumes,
        consumes_in_cur_stmt: SmallVec::default(),
        tcx,
        body,
        crate_ctxt, // struct_topology,
                    // basic_block: BasicBlock::from_u32(0),
    }
    .visit_body(body);

    let mut maybe_owning = BitSet::new_empty(body.local_decls.len());

    for (local, local_decl) in body.local_decls.iter_enumerated() {
        // let ty = local_decl.ty;
        // let local_info = local_decl.local_info.as_deref();
        // if crate_ctxt.ty_contains_ptr(ty) && !matches!(local_info, Some(LocalInfo::DerefTemp)) {
        //     to_finalise.insert(local);
        // }
        if local_has_non_zero_measure(local_decl, crate_ctxt) {
            maybe_owning.insert(local);
        }
    }

    let consumes = consumes.done();

    Definitions {
        consumes,
        maybe_consume_sites,
        maybe_owning,
    }
}
