//! hilavitkutin-build — shared build-dependency crate.
//!
//! Every hilavitkutin crate and every consumer's `build.rs` calls
//! [`bootstrap_from_buildscript`]. The crate optimises HOW code is
//! compiled (pragmas, profiles, rustc wrapper), not what it does.
//! Standalone, no runtime deps.
//!
//! # `std` stance
//!
//! The src CL's original sketch suggested `#![no_std]` with the
//! `bootstrap_from_buildscript` body gated behind
//! `#[cfg(not(target_os = "none"))]`. In practice this crate only
//! ever runs at build time from `build.rs`, which always links
//! against `std`. Pragmatic call: drop `#![no_std]`, use `std` in
//! bootstrap, and treat "expose the type surface in a no_std-compatible
//! sub-module" as a BACKLOG item. The pragma / profile / axis types
//! are already `#[no_std]`-safe in practice (no heap, no `std::`
//! imports) so promoting them later is mechanical.
//!
//! # Layout
//!
//! - [`pragma`] — `Pragma` enum + `PragmaSet` bitmask.
//! - [`profile`] — `Profile` + default pragma sets.
//! - [`axis`] — three-axis classification (Target / Tier / Passes).
//! - [`config`] — `BuildConfig::from_cargo_env()`.
//! - [`requirements`] — static pragma -> external-tool table.
//! - [`bootstrap`] — build-script entry point.
//! - [`guards`] — `compile_error!` macro helpers.
//!
//! # Pragma roster
//!
//! The 13 pragmas that `Pragma` exposes (definitions live in
//! [`pragma`]; external-tool requirements in [`requirements`]):
//!
//! - `LoopOptimization` — IRCE, LoopPredication, SimplifyCFG,
//!   LoopInterchange, LoopDistribute, LoopDataPrefetch,
//!   SeparateConstOffsetFromGEP via `polka-passes.so`.
//! - `Polly` — polyhedral optimiser; requires Polly-enabled LLVM.
//! - `MathPeephole` — float peephole rewrites via `math-peephole.so`.
//! - `FastMath` — LLVM `unsafe-fp-math` flag + `arvo_fast_math` cfg.
//! - `ExpandedLto` — fat LTO + codegen-units=1 (generated Cargo config).
//! - `Pgo` — consume PGO profiles when present on disk.
//! - `Bolt` — post-link binary rewriting (Linux ELF only).
//! - `Profiling` — run profiling benchmarks post-build.
//! - `BuildStd` — rebuild std from source with optimisation flags.
//! - `ParallelCodegen` — `-Zthreads=N` (0 = auto-detect).
//! - `SharedGenerics` — `-Zshare-generics=y`.
//! - `LoopFusion` — experimental; fuse adjacent loops.
//! - `MimallocAllocator` — advisory mimalloc recommendation.
//!
//! The generated config file `Cargo.toml`-shaped overrides (Profiles
//! etc.) are deferred to a follow-up round per `BACKLOG`.

#![deny(unsafe_op_in_unsafe_fn)]

pub mod axis;
pub mod bootstrap;
pub mod config;
pub mod guards;
pub mod pragma;
pub mod profile;
pub mod requirements;

pub use axis::{PassesAxis, TargetAxis, TierAxis};
pub use bootstrap::bootstrap_from_buildscript;
pub use config::BuildConfig;
pub use pragma::{Pragma, PragmaIter, PragmaSet};
pub use profile::Profile;
pub use requirements::{PragmaRequirement, REQUIREMENTS, Requirement, requirements_for};
