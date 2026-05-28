//! Resource handling (domain 19).
//!
//! Convergence accumulators, pointer-provenance newtypes.

pub mod accumulator;
pub mod provenance;

pub use accumulator::{AccumulatorSlot, ConvergenceBuffer};
pub use provenance::{ColumnPtr, ResourcePtr};
