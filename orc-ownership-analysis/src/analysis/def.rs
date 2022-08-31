use crate::{analysis::ty_ext::TyExt, struct_topology::StructTopology};
use orc_common::data_structure::vec_array::{VecArray, VecArrayConstruction};
use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor},
        BasicBlock, BasicBlockData, Body, CastKind, Local, LocalInfo, Location, Place, Rvalue,
        Terminator, TerminatorKind, RETURN_PLACE,
    },
    ty::TyCtxt,
};
use smallvec::SmallVec;

use super::state::SSAIdx;

// e has type T and T coerces to U; coercion-cast
// e has type *T, U is *U_0, and either U_0: Sized or unsize_kind(T) = unsize_kind(U_0); ptr-ptr-cast
// e has type *T and U is a numeric type, while T: Sized; ptr-addr-cast
// e is an integer and U is *U_0, while U_0: Sized; addr-ptr-cast
// e has type T and T and U are any numeric types; numeric-cast
// e is a C-like enum and U is an integer type; enum-cast
// e has type bool or char and U is an integer; prim-int-cast
// e has type u8 and U is char; u8-char-cast
// e has type &[T; n] and U is *const T; array-ptr-cast
// e is a function pointer type and U has type *T, while T: Sized; fptr-ptr-cast
// e is a function pointer type and U is an integer; fptr-addr-cast

/// TODO: handle addr-ptr cast? Currently, definitions are accounted
/// for const addr to ptr cast.
pub(crate) struct Definitions {
    /// BasicBlock -> statement_index -> possible definitions
    ///
    /// We've made an assumption that a local can only be used or defined
    /// once in a statement/terminator
    pub(crate) defs: VecArray<SmallVec<[Local; 2]>>,
    pub(crate) sites: IndexVec<Local, BitSet<BasicBlock>>,
}

impl Definitions {
    #[inline]
    pub(crate) fn of_block(&self, block: BasicBlock) -> &[SmallVec<[Local; 2]>] {
        &self.defs[block.index()]
    }

    #[inline]
    pub(crate) fn of_location(&self, location: Location) -> &SmallVec<[Local; 2]> {
        let Location {
            block,
            statement_index,
        } = location;
        &self.defs[block.index()][statement_index]
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Consume {
    pub(crate) r#use: SSAIdx,
    pub(crate) def: SSAIdx,
}

impl Consume {
    #[inline]
    pub(crate) fn new() -> Self {
        Consume {
            r#use: SSAIdx::INIT,
            def: SSAIdx::INIT,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum RichLocation {
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
pub(crate) struct ConsumeChain {
    /// ssa index for each consumption
    pub(crate) consumes: VecArray<SmallVec<[(Local, Consume); 2]>>,
    /// location of each definition
    pub(crate) locs: IndexVec<Local, IndexVec<SSAIdx, RichLocation>>,
}

impl ConsumeChain {
    pub(crate) fn new<'tcx>(body: &Body<'tcx>, definitions: Definitions) -> Self {
        let mut names = VecArray::new(definitions.defs.array_count());
        for def in definitions.defs.iter() {
            names.push_array(def.iter().map(|vec| {
                vec.iter()
                    .copied()
                    .map(|local| (local, Consume::new()))
                    .collect()
            }));
        }
        let names = names.done();

        let locs = IndexVec::from_fn_n(
            |local| {
                matches!(
                    body.local_decls[local].local_info.as_deref(),
                    Some(LocalInfo::DerefTemp)
                )
                .then(|| IndexVec::new())
                .unwrap_or_else(|| IndexVec::from_raw(vec![RichLocation::Entry]))
            },
            body.local_decls.len(),
        );
        ConsumeChain {
            consumes: names,
            locs,
        }
    }

    // #[inline]
    // pub(crate) fn of_block(&self, block: BasicBlock) -> impl Iterator<Item = impl Iterator<Item = Local>>{//&[SmallVec<[Local; 2]>] {
    //     &self.defs[block.index()]
    // }

    #[inline]
    pub(crate) fn of_block(&self, block: BasicBlock) -> &[SmallVec<[(Local, Consume); 2]>] {
        &self.consumes[block.index()]
    }

    #[inline]
    pub(crate) fn of_location(&self, location: Location) -> &SmallVec<[(Local, Consume); 2]> {
        //&SmallVec<[Local; 2]> {
        let Location {
            block,
            statement_index,
        } = location;
        &self.consumes[block.index()][statement_index]
    }
}

pub(crate) fn initial_definitions<'tcx>(
    body: &Body<'tcx>,
    tcx: TyCtxt<'tcx>,
    struct_topology: &StructTopology,
) -> Definitions {
    let mut sites = IndexVec::from_elem(
        BitSet::new_empty(body.basic_blocks.len()),
        &body.local_decls,
    );

    let mut defs = VecArray::new(body.basic_blocks().len());

    struct Vis<'me, 'tcx> {
        sites: &'me mut IndexVec<Local, BitSet<BasicBlock>>,
        defs: &'me mut VecArrayConstruction<SmallVec<[Local; 2]>>,
        defs_in_cur_stmt: SmallVec<[Local; 2]>,
        body: &'me Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        struct_topology: &'me StructTopology,
        // basic_block: BasicBlock,
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
                let defs_in_cur_stmt = std::mem::take(&mut self.defs_in_cur_stmt);
                self.defs.add_item_to_array(defs_in_cur_stmt);
                index += 1;
            }

            if let Some(terminator) = terminator {
                let location = Location {
                    block,
                    statement_index: index,
                };
                self.visit_terminator(terminator, location);
                let defs_in_cur_stmt = std::mem::take(&mut self.defs_in_cur_stmt);
                self.defs.add_item_to_array(defs_in_cur_stmt);
            }
            self.defs.done_with_array();
        }

        fn visit_rvalue(&mut self, rvalue: &Rvalue<'tcx>, location: Location) {
            // if let Rvalue::Cast(CastKind::PointerFromExposedAddress, operand, _) = rvalue {
            //     // if let Some(constant) = operand.constant() {
            //     //     println!("{:?}", constant.literal);
            //     //     panic!("")
            //     // }
            //     // return;
            //     if operand.place().is_some() {
            //         return;
            //     }
            // }

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

        fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
            let TerminatorKind::Return = terminator.kind else {
                return self.super_terminator(terminator, location)
            };
            let return_place = Place::from(RETURN_PLACE);
            self.visit_place(
                &return_place,
                PlaceContext::NonMutatingUse(NonMutatingUseContext::Move),
                location,
            )
        }

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

            if ty.contains_ptr(self.struct_topology)
                && !place.is_indirect()
                && !matches!(local_info, Some(LocalInfo::DerefTemp))
            {
                // println!("defining {:?} at {:?}", place.local, location);
                self.defs_in_cur_stmt.push(place.local);
                self.sites[place.local].insert(location.block);
            }
        }
    }

    Vis {
        sites: &mut sites,
        defs: &mut defs,
        defs_in_cur_stmt: SmallVec::default(),
        tcx,
        body,
        struct_topology,
        // basic_block: BasicBlock::from_u32(0),
    }
    .visit_body(body);

    Definitions {
        defs: defs.done(),
        sites,
    }
}

#[cfg(test)]
mod test {

    use super::{initial_definitions, Definitions};
    use rustc_middle::mir::{BasicBlock, Local, Location};

    impl Definitions {
        fn assert_round_tripping(&self) {
            for (local, sites) in self.sites.iter_enumerated() {
                for bb in sites.iter() {
                    self.of_block(bb)
                        .iter()
                        .flatten()
                        .copied()
                        .find(|&l| l == local)
                        .unwrap_or_else(|| {
                            panic!("{:?} should contain definition of {:?}", bb, local)
                        });
                }
            }

            for (bb, defs) in self.defs.iter().enumerate() {
                let bb = rustc_middle::mir::BasicBlock::from(bb);
                for local in defs.iter().flatten().copied() {
                    assert!(self.sites[local].contains(bb))
                }
            }
        }
    }

    // use std::path::PathBuf;
    #[test]
    fn test1() {
        const INPUT: &str = "
    static mut STATIC: usize = 0;
    const ADDR: usize = 47;

    unsafe fn f(mut p: *mut *mut usize, q: *mut *mut usize, mut r: *mut usize, addr: usize) -> *mut usize {
        p = ADDR as *mut _;
        *q = p as *mut _;
        r = *q;
        *p = r;

        STATIC = *r;

        r = addr as *mut _;

        return r;
    }";

        // let file = std::path::PathBuf::from("../workspace/def_site.rs");
        orc_common::test_infra::run_compiler_with(INPUT.into(), |tcx, functions, structs| {
            let program = crate::CrateInfo::new(tcx, functions, structs);
            let mut def_iter = program.functions().iter().copied().map(|did| {
                let body = tcx.optimized_mir(did);
                initial_definitions(body, tcx, &program.struct_topology)
            });
            let Some(definition) = def_iter.next() else { unreachable!() };
            assert!(def_iter.next().is_none());
            definition.assert_round_tripping();
            // do not count definition for rhs of addr-ptr cast
            assert_eq!(
                definition
                    .of_location(Location {
                        block: BasicBlock::from_u32(0),
                        statement_index: 13
                    })
                    .to_vec(),
                vec![Local::from_u32(13)]
            );
            // q is never defined in this program
            assert!(definition
                .defs
                .iter()
                .flatten()
                .flatten()
                .copied()
                .find(|local| local.index() == 2)
                .is_none())
        })
    }
}
