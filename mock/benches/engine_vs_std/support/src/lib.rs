//! The engine against optimal std, workload by workload, as arms the bench
//! harness times (#660, #664).
//!
//! The engine's complete single-core design targets parity or better against
//! an optimal hand-fused std loop (0.95x to 1.02x; see
//! `mock/research/202606052000_single-core-engine-ideal-vs-actual-audit.md`).
//! The mechanisms that earn it (dispatch devirtualisation, within-fiber
//! fusion, the adaptive trunk, fiber and morsel scheduling) are not built, so
//! the engine arms measure a baseline for a later comparison and decide
//! nothing about the engine.
//!
//! Four workloads form a gradient rather than a cliff, so the ceilings in
//! `expected` show progress mechanism by mechanism:
//!
//! 1. `element_wise`: a four-stage RAW chain, pure fusion territory.
//! 2. `branching`: two transforms of one input joined by a third, a diamond
//!    the engine materialises and std keeps in registers.
//! 3. `accumulator`: one transform feeding the append surface, the
//!    accumulator dispatch path against an optimal buffer fill.
//! 4. `wide_parallel`: four independent heavy chains, the path the engine is
//!    meant to win across cores.
//!
//! Every arm of a workload is held to one reference digest computed from the
//! kernels alone (`Workload::reference`), by the harness before any timing
//! counts and by the unit tests here at small sizes. A digest mismatch means
//! the arm is wrong, not that it is slow.
//!
//! std and alloc are allowed here: this is a standalone bench workspace outside
//! the no_std and no_alloc mock crates.

pub mod accumulator;
pub mod arena;
pub mod branching;
pub mod crew;
pub mod element_wise;
pub mod executor;
pub mod expected;
pub mod kernel;
pub mod routine;
pub mod wide_parallel;
pub mod workload;

pub use arena::*;
pub use bench_arm::{Fnv1a, TaggedRecords, fnv1a_by, fnv1a_u32};
pub use crew::Crew;
pub use expected::*;
pub use kernel::*;
pub use routine::*;
pub use workload::*;

#[cfg(test)]
mod tests;
