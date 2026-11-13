//! The contracts a kernel author and a store implementor write against.
//!
//! An author names a domain, its columns and resources, and writes a value kernel: a function
//! handed the values of its declared reads and returning the values of its declared writes. An
//! implementor writes a store and a visiting order. Between the two sits the checked binding
//! table, made only by [`bind::bind`], which is the one source of the typed column handles a store
//! is read and written through. A runner takes a checked table, a store and a visiting order; the
//! one-thread reference runner is a separate crate and the oracle every other runner is held to.
//!
//! `no_std`, no allocator, no `dyn`, no feature gate.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod bind;
pub mod digest;
pub mod domain;
pub mod kernel;
pub mod list;
pub mod num;
pub mod store;

/// The optional and the fallible value every contract returns, re-exported so a crate written
/// against these contracts names one dependency.
pub use notko::{Maybe, Outcome};
