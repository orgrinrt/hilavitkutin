//! hilavitkutin: pipeline execution engine.
//!
//! Morsel-driven pipeline engine. Consumes WorkUnit declarations
//! from `hilavitkutin-api`, analyses them into phases/trunks/fibers,
//! compiles dispatch programs, and runs them on a pre-allocated
//! thread pool.
//!
//! `#![no_std]`, no alloc, no `dyn`, no `TypeId`. Platform
//! integration is feature-gated via three mutually exclusive tiers
//! (`platform-os`, `platform-std`, `platform-no-os`).

#![no_std]
#![allow(incomplete_features)]
// generic_const_exprs is WATCH-tier per the unstable-feature soundness sweep
// (task #626). Post the capacity-as-type migration (#652) the engine's own
// plan/thread/dispatch arrays are sized by the arvo `Capacity` TYPE, so no
// `cap_size` sits in an array-length position internally. The one residual GCE
// use is `core_program::synthesise_core_programs`, which projects per-core
// capacities into the unmigrated hilavitkutin-api `CoreProgram`'s `usize`
// min-const-generic positions via `CoreProgram<{ cap_size(C::CAP) }, ...>`;
// that const-argument expression keeps the gate live. `adt_const_params` is no
// longer needed (no `Cap` const-generic params survive in the engine; the
// remaining const generics are plain `usize` on the api `CoreProgram`).
#![feature(generic_const_exprs)]
// const_trait_impl (vetted WATCH, unstable-features.md): the dispatch-order
// machinery (`plan::project::MaskProject`, `dispatch::order`) computes the
// per-unit access masks and the topological dispatch order in a const context,
// so the order is a compile-time fact (the devirtualisation precondition).
#![feature(const_trait_impl)]
#![recursion_limit = "512"]
#![deny(unsafe_op_in_unsafe_fn)]

// Platform tier exclusivity. Exactly one of the three platform
// features must be active at a time. Consumers opting out of the
// default `platform-os` feature must explicitly enable one of the
// alternatives.

#[cfg(all(feature = "platform-os", feature = "platform-std"))]
compile_error!(
    "hilavitkutin: features `platform-os` and `platform-std` are mutually exclusive"
);

#[cfg(all(feature = "platform-os", feature = "platform-no-os"))]
compile_error!(
    "hilavitkutin: features `platform-os` and `platform-no-os` are mutually exclusive"
);

#[cfg(all(feature = "platform-std", feature = "platform-no-os"))]
compile_error!(
    "hilavitkutin: features `platform-std` and `platform-no-os` are mutually exclusive"
);

#[cfg(not(any(feature = "platform-os", feature = "platform-std", feature = "platform-no-os")))]
compile_error!(
    "hilavitkutin: one of `platform-os`, `platform-std`, or `platform-no-os` must be enabled"
);

pub mod adapt;
pub mod dispatch;
pub mod intrinsics;
pub mod platform;
pub mod plan;
pub mod resource;
pub mod scheduler;
pub mod strategy;
pub mod thread;

pub use plan::{DefaultPlanDims, PlanDims};

#[cfg(feature = "platform-os")]
pub use platform::{OsClock, OsMemoryProvider, OsThreadPool};

#[cfg(feature = "platform-std")]
pub use platform::{StdClock, StdMemoryProvider, StdThreadPool};
