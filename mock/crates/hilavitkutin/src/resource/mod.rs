//! Resource handling (domain 19).
//!
//! Convergence accumulators, pointer-provenance newtypes.

pub mod accumulator;
pub mod bindings;
pub mod provenance;

pub use accumulator::{AccumulatorSlot, ConvergenceBuffer};
pub use bindings::{
    ColumnBinding, BindingsFor, ResourceBinding, BindingNil, VirtualBinding, DrainStores,
};
pub use provenance::{ColumnPtr, ResourcePtr};
