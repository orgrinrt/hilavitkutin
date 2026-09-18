//! hilavitkutin-build: shared build-dependency crate.
//!
//! A consumer's `build.rs` resolves a [`BuildConfig`] from cargo's
//! environment and asks this crate what to print and what to write.
//! The crate optimises HOW code is compiled (pragmas, profiles, rustc
//! wrapper), not what it does. Standalone, no runtime deps.
//!
//! # `no_std`
//!
//! Like every hilavitkutin crate, this one is `no_std` and does not
//! allocate. Everything that touches the host stays in the build
//! script, which owns it: reading `$PROFILE` and
//! `$CARGO_CFG_TARGET_FEATURE`, printing the `cargo::` directives, and
//! writing the generated config file. The crate takes the values the
//! script read, and hands back the text through `core::fmt::Write`.
//! [`bootstrap`] carries the whole `build.rs`.
//!
//! # Layout
//!
//! [`pragma`] defines the `Pragma` enum and `PragmaSet` bitmask.
//! [`profile`] defines `Profile` and its default pragma sets.
//! [`axis`] holds the three-axis classification (Target / Tier /
//! Passes). [`config`] exposes `BuildConfig::from_cargo()`.
//! [`requirements`] contains the static pragma-to-external-tool
//! table. [`bootstrap`] writes the directives and the config file.
//! [`guards`] exposes `compile_error!` macro helpers.
//!
//! # Pragma roster
//!
//! The 13 pragmas that `Pragma` exposes (definitions live in
//! [`pragma`]; external-tool requirements in [`requirements`]):
//!
//! | Pragma | Effect |
//! |--------|--------|
//! | `LoopOptimization` | IRCE, LoopPredication, SimplifyCFG, LoopInterchange, LoopDistribute, LoopDataPrefetch, SeparateConstOffsetFromGEP via `polka-passes.so`. |
//! | `Polly` | Polyhedral optimiser; requires Polly-enabled LLVM. |
//! | `MathPeephole` | Float peephole rewrites via `math-peephole.so`. |
//! | `FastMath` | LLVM `unsafe-fp-math` flag plus the `arvo_fast_math` cfg. |
//! | `ExpandedLto` | Fat LTO with codegen-units=1 (generated Cargo config). |
//! | `Pgo` | Consume PGO profiles when present on disk. |
//! | `Bolt` | Post-link binary rewriting (Linux ELF only). |
//! | `Profiling` | Run profiling benchmarks post-build. |
//! | `BuildStd` | Rebuild std from source with optimisation flags. |
//! | `ParallelCodegen` | `-Zthreads=N` (0 = auto-detect). |
//! | `SharedGenerics` | `-Zshare-generics=y`. |
//! | `LoopFusion` | Experimental; fuse adjacent loops. |
//! | `MimallocAllocator` | Advisory mimalloc recommendation. |
//!
//! The generated config file `Cargo.toml`-shaped overrides (Profiles
//! etc.) are deferred to a follow-up round per `BACKLOG`.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod axis;
pub mod bootstrap;
pub mod config;
pub mod guards;
pub mod pragma;
pub mod profile;
pub mod requirements;

pub use axis::{PassesAxis, TargetAxis, TierAxis};
pub use bootstrap::{write_config_file, write_directives};
pub use config::BuildConfig;
pub use pragma::{Pragma, PragmaIter, PragmaSet};
pub use profile::Profile;
pub use requirements::{PragmaRequirement, REQUIREMENTS, Requirement, requirements_for};
