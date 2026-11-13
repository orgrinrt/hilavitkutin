//! The `frame_wide` workload, defined once for every arm that runs it.
//!
//! One simulation frame over a wide DAG: three independent three-stage
//! pipelines off a shared input column, a fan-in stage joining their tails, and
//! two independent single-stage transforms off the same input. Twelve work
//! units over thirteen columns, so the engine's plan has real phases, trunks
//! and fibers to find rather than one linear pipeline.
//!
//! Every arm computes the same three output columns from the same input, and
//! the digest over those three columns is what the harness compares byte-exact
//! across arms before any timing counts. The stage kernels, the join, the
//! column identities and the digest live here and nowhere else: an arm that
//! disagreed with another would have to disagree with this file first.
//!
//! The arms measure an engine without its headline features. Fusing and the
//! adaptive trunk, fiber and morsel work are not built, so a number from this
//! bench is a baseline for a later comparison and decides nothing about the
//! engine.
//!
//! std and alloc are allowed here. This is a bench crate outside the no_std,
//! no_alloc `mock/crates` members, as `engine_vs_std` and `resource_storage`
//! both are.
//!
//! # Layout
//!
//! The five modules below were one file until it passed the workspace's
//! 500-line limit, and they split along what that file was already doing.
//! [`kernel`] is the per-record arithmetic and the constants that make each of
//! the twelve stages a different function. [`frame`] states the frame itself,
//! once, as the reference every arm is held to, with the negative control
//! beside it. [`image`] is the input one call receives and the digest taken
//! over the outputs. [`routine`] is what the harness drives, including the
//! per-call budget. [`arms`] is the scaffolding each arm's cdylib hangs off.
//!
//! Everything is re-exported here, so `frame_wide_support::X` reaches what it
//! always did and no arm needs to know which module an item ended up in.

pub mod engine;
pub mod plain;

pub mod arms;
pub mod frame;
pub mod image;
pub mod kernel;
pub mod routine;

pub use arms::*;
pub use frame::*;
pub use image::*;
pub use kernel::*;
pub use routine::*;
