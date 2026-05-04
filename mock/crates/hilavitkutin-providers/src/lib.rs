//! `hilavitkutin-providers`: default Resource-backed providers
//! for the hilavitkutin scheduler.
//!
//! Standalone ecosystem crate. Consumed by the engine via Kit
//! installation or via direct `builder.resource(...)` wiring. No
//! reverse dep on the engine.
//!
//! Ships the interner surface today: [`InternerApi`],
//! [`HasInterner`], [`MemoryArena`], the [`default_interner`]
//! constructor, and the [`InternerKit`] Kit preset that registers
//! the default interner as a `Resource<...>` on the scheduler
//! builder via `add_kit`.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod interner;

pub use crate::interner::{
    HasInterner, InternerApi, InternerKit, MemoryArena, default_interner,
};
