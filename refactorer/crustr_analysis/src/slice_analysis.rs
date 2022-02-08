//! Global effect: function summary and field def summary -> interprocedural analysis
//! Local effect: local value -> intraprocedural data flow analysis
//! The question to think about: in the buffer example, why buffer_t.alloc
//! is a slice and why buffer_t.data is not? If we treat it as an equational
//! value flow (bidirectional), now that buffer_t.data is ever assigned by
//! buffer_t.alloc, how can we block this flow?
//! Or if we must treat it as directional value flow? Flow from fat to thin
//! only!!!!

pub mod ptr_type;

pub enum Reason {
    FatReason,
    ThinReason,
}

pub enum FatReason {
    ArgumentsForCalloc,
    ArgumentsForRealloc,
    /// p = q, p fat ==> q fat
    Propagated,
}

pub enum ThinReason {
    ReturnOfOffset,
    /// p = q, q thin ==> p thin
    Propagated,
}
