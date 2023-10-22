use rustc_middle::mir::{Location, Operand, Place};
use rustc_span::symbol::Ident;

use super::Intraprocedural;
use crate::flow::ownership::constraint::{Constraint, Database, StorageMode};

impl<'analysis, 'tcx, Mode, DB> Intraprocedural<'analysis, 'tcx, Mode, DB>
where
    Mode: StorageMode,
    DB: Database<Mode>,
{
    pub fn extern_call(
        &mut self,
        name: Ident,
        args: &Vec<Operand<'tcx>>,
        destination: &Place<'tcx>,
        location: Location,
    ) {
        match name.as_str() {
            "malloc" => self.malloc(args, destination, location),
            "calloc" => self.malloc(args, destination, location),
            "free" => self.free(args, destination, location),
            _ => tracing::debug!(
                "ignoring extern call {destination:?} = {name}({})",
                args.iter()
                    .map(|operand| format!("{operand:?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }

    fn malloc(&mut self, args: &Vec<Operand<'tcx>>, destination: &Place<'tcx>, location: Location) {
        let _ = args;
        let Some(dest) = self.path(destination, location) else {
            assert_eq!(self.k_limit, 0);
            return;
        };
        let dest = self.expand(&dest);
        assert_eq!(dest.num_pointers_reachable(), 1);
        self.ctxt.database.add(
            Constraint::Assume {
                x: dest.base.r#use,
                sign: false,
            },
            &mut self.ctxt.storage,
        );
        self.ctxt.database.add(
            Constraint::Assume {
                x: dest.base.def,
                sign: true,
            },
            &mut self.ctxt.storage,
        );
    }

    fn free(&mut self, args: &Vec<Operand<'tcx>>, destination: &Place<'tcx>, location: Location) {
        assert!(self.path(destination, location).is_none());
        assert_eq!(args.len(), 1);
        let arg = match args[0] {
            Operand::Move(place) | Operand::Copy(place) => place,
            _ => unreachable!(),
        };
        let Some(arg) = self.path(&arg, location) else {
            assert_eq!(self.k_limit, 0);
            return;
        };
        let arg = self.expand(&arg);
        assert_eq!(arg.num_pointers_reachable(), 1);
        self.ctxt.database.add(
            Constraint::Assume {
                x: arg.base.r#use,
                sign: true,
            },
            &mut self.ctxt.storage,
        );
        self.ctxt.database.add(
            Constraint::Assume {
                x: arg.base.def,
                sign: false,
            },
            &mut self.ctxt.storage,
        );
    }
}
