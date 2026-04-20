//! Resource handling (domain 19).
//!
//! Stack-local snapshot + typed cache, convergence accumulators,
//! pointer-provenance newtypes.

pub mod accumulator;
pub mod cache;
pub mod provenance;
pub mod snapshot;

pub use accumulator::{AccumulatorSlot, ConvergenceBuffer};
pub use cache::ResourceCache;
pub use provenance::{ColumnPtr, ResourcePtr};
pub use snapshot::{ResourceSnapshot, Slot};
