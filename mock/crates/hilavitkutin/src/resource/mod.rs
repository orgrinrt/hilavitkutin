//! Resource handling (domain 19).
//!
//! Convergence accumulators, pointer-provenance newtypes.

pub mod accumulator;
pub mod arena;
pub mod provenance;

pub use accumulator::{AccumulatorSlot, ConvergenceBuffer};
pub use arena::{
    ArenaColumnNode, ArenaFor, ArenaResourceNode, ArenaTail, ArenaVirtualNode, DrainStores,
    DropArena,
};
pub use provenance::{ColumnPtr, ResourcePtr};
