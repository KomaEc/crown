use rustc_index::IndexVec;
use rustc_middle::mir::Local;

use super::SSAIdx;

pub struct SSAState {
    count: IndexVec<Local, SSAIdx>,
    stack: IndexVec<Local, Vec<SSAIdx>>,
}

impl SSAState {
    pub fn new(num_locals: usize) -> Self {
        let count = IndexVec::from_elem_n(SSAIdx::INIT, num_locals);
        let stack = IndexVec::from_elem_n(vec![SSAIdx::INIT], num_locals);
        SSAState { count, stack }
    }

    pub fn fresh_name(&mut self, var: Local) -> SSAIdx {
        self.count[var] += 1;
        let idx = self.count[var];
        self.stack[var].push(idx);
        idx
    }

    pub fn get_name(&self, var: Local) -> SSAIdx {
        self.try_get_name(var).unwrap_or_else(|| {
            panic!(
                "internal error: cannot find fresh name supply for {:?}",
                var
            )
        })
    }

    pub fn try_get_name(&self, var: Local) -> Option<SSAIdx> {
        self.stack[var].last().copied()
    }

    pub fn pop(&mut self, var: Local) -> SSAIdx {
        self.stack[var]
            .pop()
            .unwrap_or_else(|| panic!("internal error: poping non existing version for {:?}", var))
    }
}
