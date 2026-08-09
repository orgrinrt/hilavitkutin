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

/// The states this crate refuses, pinned so they stay refused.
///
/// Every claim here is one the design document makes in prose. **A refusal that
/// nothing guards can be removed by loosening a bound**, and every other test in
/// the suite still passes, because none of them names the case that stopped
/// being refused.
///
/// **Each refusal is paired with the nearest thing that must still compile.** A
/// `compile_fail` block passes when its contents fail for any reason at all, a
/// typo included, so the error code is pinned and a positive twin shows the
/// surrounding construction is sound.
///
/// # Two shapes' handles do not compare
///
/// `Sym<Standard>` and `Sym<WideKind>` divide the same 32 bits differently, so
/// an equality between them is meaningless rather than merely unwise.
///
/// ```compile_fail,E0308
/// use hilavitkutin_sym::{Standard, Sym, WideKind};
/// let a: Sym<Standard> = Sym::default();
/// let b: Sym<WideKind> = Sym::default();
/// let _ = a == b;
/// ```
///
/// Two handles of the same shape do compare, which is what makes the refusal
/// above about the shapes rather than about `Sym`:
///
/// ```
/// use hilavitkutin_sym::{Sym, WideKind};
/// let a: Sym<WideKind> = Sym::default();
/// let b: Sym<WideKind> = Sym::default();
/// assert_eq!(a, b);
/// ```
///
/// # The literal tag constructor exists on `Standard` alone
///
/// It takes a three-bit value, and a shape with a wider tag would truncate the
/// literal silently, so the method is scoped rather than generic.
///
/// ```compile_fail,E0599
/// use hilavitkutin_sym::{SymKind, WideKind};
/// let _ = SymKind::<WideKind>::from_raw(0b10000);
/// ```
///
/// The same shape builds its tag through `new`, at its own width, which is the
/// route a non-default shape is meant to take:
///
/// ```
/// use hilavitkutin_sym::{SymKind, WideKind};
/// let _ = SymKind::<WideKind>::new(arvo_bits::Bits::from_raw(0b10000));
/// ```
///
/// And it exists on the default shape, so the refusal is about the scoping
/// rather than about the method:
///
/// ```
/// use hilavitkutin_sym::SymKind;
/// let _ = SymKind::from_raw(0b001);
/// ```
pub mod refusals {}
