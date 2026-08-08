//! `hilavitkutin-sym`: generic interned-identity core.
//!
//! A `Sym` is a 4-byte integer-equality handle tagged by a 3-bit domain
//! `kind` (plus a 1-bit domain-private `flag`) over a 28-bit `id`. A domain
//! acquires a handle either by interning a value (dedup and resolve, the
//! [`Interner`] trait) or by minting a fresh one (no value, the [`Generator`]
//! producer). Two handles of different kind never compare equal, which keeps
//! domains disjoint. String interning is one domain, supplied by
//! `hilavitkutin-str`.

#![no_std]
#![feature(const_trait_impl)]
#![feature(macro_metavar_expr_concat)]
#![allow(incomplete_features)]

mod domain;
mod generator;
mod handle;
mod interner;
mod kind;
mod shape;

pub use crate::domain::{Domain, GenerativeDomain, InterningDomain};
pub use crate::generator::Generator;
pub use crate::handle::{Sym, SymLayout};
pub use crate::interner::Interner;
pub use crate::kind::SymKind;
pub use crate::shape::{Standard, SymShape};
