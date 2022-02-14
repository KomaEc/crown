use std::io::Write;

use rustc_middle::{
    mir::{pretty::*, Body},
    ty::TyCtxt,
};

use super::rename::impls::SSANameMap;

const INDENT: &str = "    ";

pub fn write_ssa_mir_fn<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    ssa_name: &SSANameMap,
    w: &mut dyn Write,
) -> std::io::Result<()> {
    write_mir_fn(
        tcx,
        body,
        &mut |pass_where, w| match pass_where {
            PassWhere::BeforeCFG => Ok(()),
            PassWhere::AfterCFG => Ok(()),
            PassWhere::BeforeBlock(bb) => {
                let has_any = !ssa_name.names_for_phi_nodes[bb].is_empty();
                if has_any {
                    writeln!(w, "{0}{0}[SSA", INDENT)?;
                }
                for &(local, def, ref uses) in &ssa_name.names_for_phi_nodes[bb] {
                    writeln!(
                        w,
                        "{}{}{:?}^{} = ϕ({})",
                        INDENT,
                        INDENT,
                        local,
                        def,
                        uses.iter()
                            .map(|idx| format!("{:?}^{}", local, idx))
                            .collect::<Vec<_>>()
                            .join(",")
                    )?;
                }
                if has_any {
                    writeln!(w, "{0}{0}]", INDENT)?;
                }
                Ok(())
            }
            PassWhere::BeforeLocation(_) => Ok(()),
            PassWhere::AfterLocation(location) => {
                let (def, ref uses) = ssa_name.names[location];
                let has_any = def.is_some() || !uses.is_empty();
                if has_any {
                    writeln!(w, "{0}{0}[SSA", INDENT)?;
                }
                if let Some(def) = def {
                    writeln!(
                        w,
                        "{}{}def of {:?} ⟼  {:?}^{}",
                        INDENT, INDENT, def.0, def.0, def.1
                    )?;
                }
                for &uze in uses {
                    writeln!(
                        w,
                        "{}{}use of {:?} ⟼  {:?}^{}",
                        INDENT, INDENT, uze.0, uze.0, uze.1
                    )?;
                }
                if has_any {
                    writeln!(w, "{0}{0}]", INDENT)?;
                }
                Ok(())
            }
            PassWhere::AfterTerminator(_) => 
                Ok(())
        },
        w,
    )
}

/*
pub fn string_of_ssa_place(place: Place<'tcx>, ssa_name: &SSANameMap) -> String {
    let mut ret = String::new();
    for elem in place.projection.iter().rev() {
        match elem {
            ProjectionElem::Downcast(_, _) | ProjectionElem::Field(_, _) => {
                write!(&mut ret, "(").unwrap();
            }
            ProjectionElem::Deref => {
                write!(&mut ret, "(*").unwrap();
            }
            ProjectionElem::Index(_)
            | ProjectionElem::ConstantIndex { .. }
            | ProjectionElem::Subslice { .. } => {}
        }
    }

    write!(&mut ret, "{:?}", self.local)?;

    for elem in self.projection.iter() {
        match elem {
            ProjectionElem::Downcast(Some(name), _index) => {
                write!(&mut ret, " as {})", name)?;
            }
            ProjectionElem::Downcast(None, index) => {
                write!(&mut ret, " as variant#{:?})", index)?;
            }
            ProjectionElem::Deref => {
                write!(&mut ret, ")")?;
            }
            ProjectionElem::Field(field, ty) => {
                write!(&mut ret, ".{:?}: {:?})", field.index(), ty)?;
            }
            ProjectionElem::Index(ref index) => {
                write!(&mut ret, "[{:?}]", index)?;
            }
            ProjectionElem::ConstantIndex { offset, min_length, from_end: false } => {
                write!(&mut ret, "[{:?} of {:?}]", offset, min_length)?;
            }
            ProjectionElem::ConstantIndex { offset, min_length, from_end: true } => {
                write!(&mut ret, "[-{:?} of {:?}]", offset, min_length)?;
            }
            ProjectionElem::Subslice { from, to, from_end: true } if to == 0 => {
                write!(&mut ret, "[{:?}:]", from)?;
            }
            ProjectionElem::Subslice { from, to, from_end: true } if from == 0 => {
                write!(&mut ret, "[:-{:?}]", to)?;
            }
            ProjectionElem::Subslice { from, to, from_end: true } => {
                write!(&mut ret, "[{:?}:-{:?}]", from, to)?;
            }
            ProjectionElem::Subslice { from, to, from_end: false } => {
                write!(&mut ret, "[{:?}..{:?}]", from, to)?;
            }
        }
    }
}

pub fn string_of_ssa_statement<'tcx>(statement: &Statement<'tcx>, ssa_name: &SSANameMap) -> String {
    match statement.kind {
        StatementKind::Assign(_) => todo!(),
        StatementKind::FakeRead(_) => todo!(),
        StatementKind::SetDiscriminant { place, variant_index } => todo!(),
        StatementKind::StorageLive(_) => todo!(),
        StatementKind::StorageDead(_) => todo!(),
        StatementKind::LlvmInlineAsm(_) => todo!(),
        StatementKind::Retag(_, _) => todo!(),
        StatementKind::AscribeUserType(_, _) => todo!(),
        StatementKind::Coverage(_) => todo!(),
        StatementKind::CopyNonOverlapping(_) => todo!(),
        StatementKind::Nop => todo!(),
    }
}

struct Wrapped<T>(T);

impl<'tcx> Debug for Wrapped<Statement<'tcx>> {
        fn &mut ret(&self, &mut ret: &mut Formatter<'_>) -> &mut ret::Result {
            match self.kind {
                Assign(box (ref place, ref rv)) => write!(&mut ret, "{:?} = {:?}", place, rv),
                FakeRead(box (ref cause, ref place)) => {
                    write!(&mut ret, "FakeRead({:?}, {:?})", cause, place)
                }
                Retag(ref kind, ref place) => write!(
                    &mut ret,
                    "Retag({}{:?})",
                    match kind {
                        RetagKind::FnEntry => "[fn entry] ",
                        RetagKind::TwoPhase => "[2phase] ",
                        RetagKind::Raw => "[raw] ",
                        RetagKind::Default => "",
                    },
                    place,
                ),
                StorageLive(ref place) => write!(&mut ret, "StorageLive({:?})", place),
                StorageDead(ref place) => write!(&mut ret, "StorageDead({:?})", place),
                SetDiscriminant { ref place, variant_index } => {
                    write!(&mut ret, "discriminant({:?}) = {:?}", place, variant_index)
                }
                AscribeUserType(box (ref place, ref c_ty), ref variance) => {
                    write!(&mut ret, "AscribeUserType({:?}, {:?}, {:?})", place, variance, c_ty)
                }
                Coverage(box self::Coverage { ref kind, code_region: Some(ref rgn) }) => {
                    write!(&mut ret, "Coverage::{:?} for {:?}", kind, rgn)
                }
                Coverage(box ref coverage) => write!(&mut ret, "Coverage::{:?}", coverage.kind),
                CopyNonOverlapping(box crate::mir::CopyNonOverlapping {
                    ref src,
                    ref dst,
                    ref count,
                }) => {
                    write!(&mut ret, "copy_nonoverlapping(src={:?}, dst={:?}, count={:?})", src, dst, count)
                }
                Nop => write!(&mut ret, "nop"),
            }
        }
}
*/
