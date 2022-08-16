use std::io::Write;

use rustc_middle::{
    mir::{pretty::*, Body},
    ty::TyCtxt,
};

use crate::def_use::IsDefUse;

use super::rename::handler::SSANameSourceMap;

const INDENT: &str = "    ";

pub fn write_ssa_mir_fn<'tcx, DefUse: IsDefUse>(
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    ssa_name: &SSANameSourceMap<DefUse>,
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
                for (local, (def, uses)) in ssa_name.names_for_phi_nodes[bb].iter_enumerated() {
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
                let (ref def, ref uses) = ssa_name.names[location];
                let has_any = !def.is_empty() || !uses.is_empty();
                if has_any {
                    writeln!(w, "{0}{0}[SSA", INDENT)?;
                }
                for &def in def {
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
            PassWhere::AfterTerminator(_) => Ok(()),
        },
        w,
    )
}
