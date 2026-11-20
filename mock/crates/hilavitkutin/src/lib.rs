//! hilavitkutin: pipeline execution engine.
//!
//! Morsel-driven pipeline engine. Consumes WorkUnit declarations
//! from `hilavitkutin-api`, analyses them into phases/trunks/fibers,
//! compiles dispatch programs, and runs them on the workers of an
//! executor the consumer supplies through `ThreadPoolApi`.
//!
//! `#![no_std]`, no alloc, no `dyn`, no `TypeId`. Platform
//! integration is the `os` feature, off by default: a build without it
//! carries the contracts alone and reaches no operating system. Neither
//! tier spawns threads.

#![no_std]
#![allow(incomplete_features)]
// FIXME(placeholders_tagged_2311ca65fd66): `generic_const_exprs` is not a feature this crate may
// keep, and it stays enabled for one residual use. The engine's own plan,
// thread and dispatch arrays are sized by the arvo `Capacity` type and need no
// const expression. `core_program::synthesise_core_programs` still projects
// the per-core capacities into the api's `CoreProgram`, whose const parameters
// are `usize`, through `CoreProgram<{ cap_size(C::CAP) }, ...>`, and that const
// argument expression keeps the gate live until `CoreProgram` takes the
// capacity type.
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

mod sealed {
    /// The engine's seal. Public in a module no path outside the crate names,
    /// so a trait bounded on it can be named outside the engine and
    /// implemented only inside it: the codegen traits take it, since their
    /// implementers are codegen artefacts.
    pub trait EngineOnly {}
}

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
#[cfg(feature = "os")]
pub use platform::{OsClock, OsMemoryProvider};
