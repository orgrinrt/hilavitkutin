//! `hilavitkutin-providers` — default Resource-backed providers
//! for the hilavitkutin scheduler.
//!
//! Standalone ecosystem crate. Consumed by the engine via Kit
//! installation (or, in v0, via direct `builder.resource(...)`
//! wiring). No reverse dep on the engine.
//!
//! v0 ships the interner value-type surface only:
//! [`InternerApi`], [`HasInterner`], [`MemoryArena`], plus the
//! free [`default_interner`] constructor. The matching
//! `InternerKit` Kit impl ships in v0.1 once api gains a
//! `BuilderResource<T>` bridging trait. See the BACKLOG entry.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod interner;

pub use crate::interner::{HasInterner, InternerApi, MemoryArena, default_interner};
