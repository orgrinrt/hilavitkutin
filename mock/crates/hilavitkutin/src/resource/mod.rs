//! Resource handling (domain 19).
//!
//! Convergence accumulators, pointer-provenance newtypes.

pub mod accumulator;
pub mod bindings;
pub mod provenance;
pub mod shape;

pub use accumulator::{AccumulatorSlot, ConvergenceBuffer};
pub use bindings::{
    ColumnBinding, BindingsFor, ResourceBinding, BindingNil, VirtualBinding, DrainStores,
};
pub use provenance::{ColumnPtr, ErasedResourcePtr, ResourcePtr};
pub use shape::ValueShape;
