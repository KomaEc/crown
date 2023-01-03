use common::data_structure::vec_vec::{VecVec, VecVecConstruction};
use rustc_data_structures::sso::SsoHashSet;
use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor},
        BasicBlock, BasicBlockData, Body, CastKind, Local, LocalInfo, Location, Place,
        ProjectionElem, Rvalue, TerminatorKind,
    },
    ty::TyCtxt,
};
use smallvec::SmallVec;

use crate::{
    ptr::Measurable,
    ssa::{constraint::local_has_non_zero_measure, state::SSAIdx},
    CrateCtxt,
};

/// TODO unify call argument temporaries and deref copy temporaries as proxy
/// temporaries
pub struct Definitions {
    /// BasicBlock -> statement_index -> possible definitions
    ///
    /// We've made an assumption that a local can only be used or defined
    /// once in a statement/terminator
    pub consumes: VecVec<SmallVec<[(Local, Consume<SSAIdx>); 2]>>,
    pub def_sites: IndexVec<Local, BitSet<BasicBlock>>,
    /// Caching the results of calling [local_has_non_zero_measure]
    pub locals_with_defs: BitSet<Local>,
    pub call_arg_temps: SsoHashSet<Local>,
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
}

impl<Iter, T> Consume<Iter>
where
    Iter: Iterator<Item = T>,
{
    pub fn transpose(self) -> impl Iterator<Item = Consume<T>> {
        self.r#use
            .zip(self.def)
            .map(|(r#use, def)| Consume { r#use, def })
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
    pub consumes: VecVec<SmallVec<[(Local, Consume<SSAIdx>); 2]>>,
    /// location of each definition
    ///
    /// Those locals with empty entries definitely do not contain pointers
    pub locs: IndexVec<Local, IndexVec<SSAIdx, RichLocation>>,

    pub call_arg_temps: SsoHashSet<Local>,
}

impl ConsumeChain {
    pub fn new(body: &Body, definitions: Definitions) -> Self {
        let Definitions {
            consumes,
            locals_with_defs: maybe_owning,
            call_arg_temps,
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

        ConsumeChain {
            consumes,
            locs,
            call_arg_temps,
        }
    }

    pub fn reset(&mut self) {
        for locs in &mut self.locs {
            if !locs.is_empty() {
                locs.truncate(1);
            }
        }
    }

    pub fn mk_dummy(&mut self) {
        for consumes in self.consumes.everything_mut() {
            consumes.clear()
        }
        self.locs = IndexVec::new();
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

    let mut call_arg_temps: SsoHashSet<Local> = SsoHashSet::default();
    for bb_data in body.basic_blocks.iter() {
        let Some(terminator) = &bb_data.terminator else { continue; };
        if let TerminatorKind::Call { args, .. } = &terminator.kind {
            call_arg_temps.extend(
                args.iter()
                    .filter_map(|arg| arg.place().and_then(|arg| arg.as_local())),
            )
        }
    }

    let mut consumes = VecVec::with_indices_capacity(body.basic_blocks.len());

    struct Vis<'me, 'tcx> {
        call_arg_temps: &'me SsoHashSet<Local>,
        maybe_consume_sites: &'me mut IndexVec<Local, BitSet<BasicBlock>>,
        consumes: &'me mut VecVecConstruction<SmallVec<[(Local, Consume<SSAIdx>); 2]>>,
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
                self.consumes.push_inner(defs_in_cur_stmt);
                index += 1;
            }

            if let Some(terminator) = terminator {
                let location = Location {
                    block,
                    statement_index: index,
                };
                self.visit_terminator(terminator, location);
                let defs_in_cur_stmt = std::mem::take(&mut self.consumes_in_cur_stmt);
                self.consumes.push_inner(defs_in_cur_stmt);
            }
            self.consumes.push();
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
                    // FIXME this is dangerouse. Need better way to categorise definition context
                    // this works if only [`Rvalue::Len`] never appears
                    | NonMutatingUseContext::Inspect
                ) | PlaceContext::MutatingUse(
                    MutatingUseContext::Call
                        | MutatingUseContext::Store
                        | MutatingUseContext::Borrow
                        | MutatingUseContext::AddressOf
                )
            ) {
                return;
            }

            let ty = place.ty(self.body, self.tcx).ty;
            let local_info = self.body.local_decls[place.local].local_info.as_deref();

            // generate consumes with base local non-empty
            if self
                .crate_ctxt
                .struct_ctxt
                .contains_ptr(self.body.local_decls[place.local].ty)
                && !matches!(local_info, Some(LocalInfo::DerefTemp))
                && !self.call_arg_temps.contains(&place.local)
            {
                let consume = if self.crate_ctxt.struct_ctxt.contains_ptr(ty)
                    && place_not_reachable_to_union(place, self.body)
                {
                    Consume::new()
                } else {
                    // For places that do not contain pointer, we generate pure use. This is useful
                    // for sanity check
                    Consume::pure_use()
                };
                self.maybe_consume_sites[place.local].insert(location.block);
                self.consumes_in_cur_stmt.push((place.local, consume));
            }
        }
    }

    Vis {
        call_arg_temps: &call_arg_temps,
        maybe_consume_sites: &mut maybe_consume_sites,
        consumes: &mut consumes,
        consumes_in_cur_stmt: SmallVec::default(),
        tcx,
        body,
        crate_ctxt,
    }
    .visit_body(body);

    let mut maybe_owning = BitSet::new_empty(body.local_decls.len());

    for (local, local_decl) in body.local_decls.iter_enumerated() {
        if local_has_non_zero_measure(local_decl, &crate_ctxt.struct_ctxt) {
            maybe_owning.insert(local);
        }
    }

    let consumes = consumes.done();

    Definitions {
        consumes,
        def_sites: maybe_consume_sites,
        locals_with_defs: maybe_owning,
        call_arg_temps,
    }
}

fn place_not_reachable_to_union<'tcx>(place: &Place<'tcx>, body: &Body<'tcx>) -> bool {
    let mut base_ty = body.local_decls[place.local].ty;
    for projection_elem in place.projection {
        match projection_elem {
            ProjectionElem::Deref => {
                base_ty = base_ty.builtin_deref(true).unwrap().ty;
            }
            ProjectionElem::Field(_, ty) => {
                if let Some(adt_def) = base_ty.ty_adt_def() {
                    if adt_def.is_union() {
                        return false;
                    }
                };
                base_ty = ty
            }
            ProjectionElem::Index(_) => {
                base_ty = base_ty.builtin_index().unwrap();
            }
            ProjectionElem::ConstantIndex { .. } => unreachable!(),
            ProjectionElem::Subslice { .. } => unreachable!(),
            ProjectionElem::OpaqueCast(_) => unreachable!("unexpected opaque cast"),
            ProjectionElem::Downcast(_, _) => {}
        }
    }
    true
}
