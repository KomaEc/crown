use rustc_index::bit_set::BitSet;
use rustc_middle::mir::{visit::Visitor, Body, Local, Location, Place, Rvalue};

struct Vis<'this>(&'this mut BitSet<Local>);

pub fn collect_copies(body: &Body) -> BitSet<Local> {
    let mut copies = BitSet::new_empty(body.local_decls.len());
    let mut vis = Vis(&mut copies);
    vis.visit_body(body);
    copies
}

impl<'tcx> Visitor<'tcx> for Vis<'_> {
    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, _: Location) {
        if let Rvalue::CopyForDeref(..) = rvalue {
            assert!(place.as_local().is_some());
            let local = place.local;
            self.0.insert(local);
        }
    }
}
