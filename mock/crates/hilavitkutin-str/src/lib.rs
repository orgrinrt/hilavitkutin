//! `hilavitkutin-str`: `no_std` interned string system.
//!
//! Shared across the hilavitkutin ecosystem. All construction paths
//! for `Str` go through [`str_const!`] (compile-time) or
//! [`StringInterner`] (runtime).

#![no_std]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(macro_metavar_expr_concat)]
#![allow(incomplete_features)]

mod entry;
mod ergonomics;
mod handle;
mod hash;
mod interner;
mod macros;
mod section;

pub use crate::entry::StaticStrEntry;
pub use crate::ergonomics::{AsStr, IntoStr};
pub use crate::handle::Str;
pub use crate::hash::{const_fnv1a, FNV_OFFSET, FNV_PRIME};
pub use crate::interner::{ArenaInterner, StringInterner}; // lint:allow(no-alloc) reason: `StringInterner` is the no-alloc interner wrapper, not std `String`; tracked: #72
pub use crate::section::static_entries;
