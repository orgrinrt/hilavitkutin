//! `hilavitkutin-sym`: generic interned-identity core.
//!
//! A `Sym` is a 4-byte integer-equality handle whose 32 bits divide
//! according to a [`SymShape`]: a domain `kind`, a domain-private `flag`, and
//! an `id` split between an origin naming the minter and a counter within it.
//! [`Standard`] is the default and is three, one and twenty-eight. A domain
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
pub mod shape;

pub use crate::domain::{Domain, GenerativeDomain, InterningDomain};
pub use crate::generator::Generator;
pub use crate::handle::{Sym, SymLayout, WideKindLayout};
pub use crate::interner::Interner;
pub use crate::kind::SymKind;
pub use crate::shape::{
    MinterId, OneOrigin, SixteenMinters, Standard, SymLayoutOps, SymShape, WideKind,
};
