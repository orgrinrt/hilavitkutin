//! hilavitkutin: pipeline execution engine.
//!
//! Morsel-driven pipeline engine. Consumes WorkUnit declarations
//! from `hilavitkutin-api`, analyses them into phases/trunks/fibers,
//! compiles dispatch programs, and runs them on the workers of an
//! executor the consumer supplies through `ThreadPoolApi`.
//!
//! `#![no_std]`, no alloc, no `dyn`, no `TypeId`. Platform
//! integration is feature-gated via two mutually exclusive tiers
//! (`platform-os`, `platform-no-os`). Neither spawns threads.

#![no_std]
#![allow(incomplete_features)]
// generic_const_exprs is WATCH-tier per the unstable-feature soundness sweep
// (task #626). Post the capacity-as-type migration (#652) the engine's own
// plan/thread/dispatch arrays are sized by the arvo `Capacity` TYPE, so no
// `cap_size` sits in an array-length position internally. The one residual GCE
// use is `core_program::synthesise_core_programs`, which projects per-core
// capacities into the unmigrated hilavitkutin-api `CoreProgram`'s `usize`
// min-const-generic positions via `CoreProgram<{ cap_size(C::CAP) }, ...>`;
// that const-argument expression keeps the gate live.
#![feature(generic_const_exprs)]
// adt_const_params (vetted, unstable-features.md): the api's `Reach<C, N>`
// carries its reach as a `Cap` const parameter, and the engine implements its
// column walks for that member, which names the parameter.
#![feature(adt_const_params)]
// const_trait_impl (vetted WATCH, unstable-features.md): the dispatch-order
// machinery (`plan::project::MaskProject`, `dispatch::order`) computes the
// per-unit access masks and the topological dispatch order in a const context,
// so the order is a compile-time fact (the devirtualisation precondition).
#![feature(const_trait_impl)]
#![recursion_limit = "512"]
#![deny(unsafe_op_in_unsafe_fn)]

// Platform tier exclusivity. Exactly one of the two platform
// features must be active at a time. Consumers opting out of the
// default `platform-os` feature must explicitly enable
// `platform-no-os`.

#[cfg(all(feature = "platform-os", feature = "platform-no-os"))]
compile_error!("hilavitkutin: features `platform-os` and `platform-no-os` are mutually exclusive");

#[cfg(not(any(feature = "platform-os", feature = "platform-no-os")))]
compile_error!("hilavitkutin: one of `platform-os` or `platform-no-os` must be enabled");

// The truncating mutant drops a refused accumulator append without counting it,
// for the overflow suite's mutant check alone. Features unify across a build, so
// a dependent turning it on would silently break the refusal; it builds only
// when the check also sets `HILAVITKUTIN_MUTANT_CHECK` in the environment.
#[cfg(feature = "mutant-truncating-append")]
const _: () = assert!(
    option_env!("HILAVITKUTIN_MUTANT_CHECK").is_some(),
    "hilavitkutin: feature `mutant-truncating-append` breaks the accumulator refusal and \
     is for the overflow suite's mutant check only; set HILAVITKUTIN_MUTANT_CHECK=1 for \
     that check, and never enable the feature elsewhere"
);

pub mod adapt;
pub mod dispatch;
pub mod intrinsics;
pub mod meta;
pub mod plan;
pub mod platform;
pub mod resource;
pub mod scheduler;
pub mod strategy;
pub mod thread;

pub use plan::{DefaultPlanDims, PlanDims};
#[cfg(feature = "platform-os")]
pub use platform::{OsClock, OsMemoryProvider};
