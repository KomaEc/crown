pub mod infer;
pub mod whole_program;

use rustc_middle::mir::{Body, Rvalue, StatementKind};

use self::infer::InferCtxt;
use crate::{
    ssa::{
        constraint::{infer::Renamer, CadicalDatabase, Database, Gen},
        consume::{initial_definitions, Consume, Voidable},
        dom::compute_dominance_frontier,
        state::SSAState,
        AnalysisResults,
    },
    type_qualifier::output_params::OutputParams,
    CrateCtxt,
};

pub trait OwnershipSchemes<'analysis>:
    AnalysisResults<'analysis, Value = Ownership, Param = Param<&'analysis [Ownership]>>
{
}

impl<'analysis, Results> OwnershipSchemes<'analysis> for Results where
    Results: AnalysisResults<'analysis, Value = Ownership, Param = Param<&'analysis [Ownership]>>
{
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ownership {
    Owning,
    Transient,
    Unknown,
}

impl Ownership {
    #[inline]
    pub fn is_owning(&self) -> bool {
        *self == Ownership::Owning
    }
}

impl From<Option<bool>> for Ownership {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(true) => Ownership::Owning,
            Some(false) => Ownership::Transient,
            None => Ownership::Unknown,
        }
    }
}

impl std::fmt::Display for Ownership {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ownership::Owning => write!(f, "&move"),
            Ownership::Transient => write!(f, "&"),
            Ownership::Unknown => write!(f, "&any"),
        }
    }
}

impl Voidable for &[Ownership] {
    const VOID: Self = &[];

    #[inline]
    fn is_void(&self) -> bool {
        self.is_empty()
    }
}

#[derive(Clone, Debug)]
pub enum Param<Var> {
    Output(Consume<Var>),
    Normal(Var),
}

#[cfg(not(debug_assertions))]
const _: () = assert!(
    std::mem::size_of::<Option<Param<std::ops::Range<crate::ssa::constraint::Var>>>>() == 16
);

impl<Value> Param<Value> {
    #[inline]
    pub fn map<U>(self, f: impl Fn(Value) -> U) -> Param<U> {
        match self {
            Param::Output(output_param) => Param::Output(output_param.repack(f)),
            Param::Normal(param) => Param::Normal(f(param)),
        }
    }

    #[inline]
    pub fn expect_normal(self) -> Value {
        match self {
            Param::Normal(sigs) => sigs,
            Param::Output(..) => panic!("expect normal parameter"),
        }
    }

    #[inline]
    pub fn expect_output(self) -> Consume<Value> {
        match self {
            Param::Output(consume) => consume,
            Param::Normal(..) => panic!("expect output parameter"),
        }
    }

    #[inline]
    pub fn to_input(self) -> Value {
        match self {
            Param::Output(Consume { r#use, .. }) => r#use,
            Param::Normal(normal) => normal,
        }
    }

    #[inline]
    pub fn to_output(self) -> Option<Value> {
        if let Param::Output(Consume { def, .. }) = self {
            Some(def)
        } else {
            None
        }
    }

    pub fn is_output(&self) -> bool {
        matches!(self, Param::Output(..))
    }
}

pub trait AnalysisKind<'analysis, 'db, 'tcx> {
    /// Analysis results
    type Results;
    /// Interprocedural context
    type InterCtxt;
    type DB: Database;
    fn analyze(
        crate_ctxt: CrateCtxt<'tcx>,
        output_params: &OutputParams,
    ) -> anyhow::Result<Self::Results>;
}

pub type Precision = u8;

pub enum Modular {}
impl<'analysis, 'db, 'tcx> AnalysisKind<'analysis, 'db, 'tcx> for Modular {
    type Results = ();
    type InterCtxt = ();
    type DB = ();
    fn analyze(_: CrateCtxt, _: &OutputParams) -> anyhow::Result<Self::Results> {
        // TODO implement this
        anyhow::bail!("modular analysis is not implemented")
    }
}

pub enum IntraProcedural {}
impl<'analysis, 'db, 'tcx> AnalysisKind<'analysis, 'db, 'tcx> for IntraProcedural {
    type Results = ();
    type InterCtxt = ();
    type DB = CadicalDatabase;
    fn analyze(crate_ctxt: CrateCtxt, _: &OutputParams) -> anyhow::Result<Self::Results> {
        // let mut databases = Vec::with_capacity(crate_ctxt.fns().len());
        for &did in crate_ctxt.fns() {
            println!("solving {:?}", did);
            let body = crate_ctxt.tcx.optimized_mir(did);

            let dominance_frontier = compute_dominance_frontier(body);
            let definitions = initial_definitions(body, &crate_ctxt);
            let ssa_state = SSAState::new(body, &dominance_frontier, definitions);
            let mut rn = Renamer::new(body, ssa_state, crate_ctxt.tcx);

            let mut gen = Gen::new();
            let mut database = CadicalDatabase::new();
            let global_assumptions = crate::ssa::constraint::GlobalAssumptions::new(
                &crate_ctxt.struct_ctxt,
                crate_ctxt.tcx,
                &mut gen,
                &mut database,
            );
            let mut infer_cx = InferCtxt::new(
                crate_ctxt.tcx,
                crate_ctxt.struct_ctxt.with_precision(0),
                body,
                &mut database,
                &mut gen,
                (),
                &global_assumptions,
            );

            rn.go::<Self>(&mut infer_cx);
        }
        Ok(())
    }
}

pub fn max_deref_level(body: &Body) -> Precision {
    let mut max_deref_level = 1;
    for bb_data in body.basic_blocks.iter() {
        let rustc_middle::mir::BasicBlockData {
            statements,
            terminator: _,
            ..
        } = bb_data;
        let mut deref_level = None;
        for statement in statements {
            if matches!(&statement.kind, StatementKind::Assign(box (_, rvalue)) if matches!(rvalue, Rvalue::CopyForDeref(_)))
            {
                *deref_level.get_or_insert(1) += 1;
                continue;
            }
            if let Some(deref_level) = deref_level.take() {
                max_deref_level = std::cmp::max(max_deref_level, deref_level);
            }
        }
    }
    max_deref_level
}
