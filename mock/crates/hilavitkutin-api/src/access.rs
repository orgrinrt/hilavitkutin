//! Access sets: the lists a unit reads and writes, and what a list holds.
//!
//! The list itself is `notko-hlist`'s. `Empty`, `Cons`, `Contains`,
//! `ContainsAll` and `Concat` are that crate's items, re-exported here so
//! that a unit's crate names one crate. This module adds what the engine's
//! contract needs on top: `AccessSet`, the bound saying a list is made of
//! members and carrying its count, and `Holds` and `HoldsAll`, the two
//! membership bounds that report a missing store in this crate's words.

use arvo::USize;
use arvo::strategy::{Additive, Identity};
use notko_hlist::List;
pub use notko_hlist::{Concat, Cons, Contains, ContainsAll, Empty};

use crate::member::Member;

/// A list whose every cell is a member.
///
/// Implemented for `Empty` and for `Cons<H, T>` where `H` is a `Member` and
/// `T` is an `AccessSet`. A store marker is a member that stands for itself,
/// and `Reach`, `Written` and `Differs` are members that stand for the store
/// they wrap. `LEN` is the count of cells.
///
/// Closed without a seal of its own: the supertrait `List` is sealed in the
/// shared crate, and neither `Empty` nor `Cons` is a type another crate may
/// implement a foreign trait for.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not an access set",
    note = "An access set is `Empty`, or `Cons<H, T>` where `H` is a store marker (`Resource`, `Column`, `Buffered`, `Accum`, `Virtual`) or a wrapper over a `Column` or a `Buffered` (`Reach`, `Written`, `Differs`) and `T` is an access set. `Differs` asks `PartialEq` of the value type, and `Written` asks nothing of it. The `read!` and `write!` macros build one from a flat list."
)]
pub trait AccessSet: List + 'static {
    /// Number of members in the set.
    const LEN: USize;
}

#[diagnostic::do_not_recommend]
impl AccessSet for Empty {
    const LEN: USize = <USize as Identity<Additive>>::IDENTITY;
}

#[diagnostic::do_not_recommend]
impl<H: Member, T: AccessSet> AccessSet for Cons<H, T> {
    const LEN: USize = USize(T::LEN.0 + 1); // lint:allow(no-bare-numeric) reason: successor step of the const count; tracked: #121
}

/// The list holds the store marker `S`, bare.
///
/// Holds exactly where `Contains<S>` does. It exists for its message: the
/// shared crate's `Contains` knows nothing of stores, so every bound an
/// author or a `build` caller can fail is written on `Holds`, and the
/// failure is reported in this crate's words with the store named.
#[diagnostic::on_unimplemented(
    message = "store `{Self}` does not contain `{S}`",
    note = "Register it with `.with(Resource::new(value))`, `.with(Column::<T>::new())`, `.with(Virtual::<T>::new())`, or install a Kit that owns it via `.with(MyKit)`. If `.build()` reports `overflow evaluating the requirement`, declare `#![recursion_limit = \"1024\"]` at your crate root.",
    note = "In a unit's body the list is the unit's own `type Read` or `type Write`: a positional accessor reaches a store the unit lists there bare. A store read with a reach, and a `Buffered<T, M>` under any member, are reached through the value function."
)]
pub trait Holds<S>: List {}

#[diagnostic::do_not_recommend]
impl<L: Contains<S>, S> Holds<S> for L {}

mod seal {
    use notko_hlist::{Cons, Empty, List};

    /// The seal on `HoldsAll`: the walk has these two steps and no other.
    pub trait Walk<L> {}

    impl<S: List> Walk<Empty> for S {}
    impl<S: List, H, T> Walk<Cons<H, T>> for S {}
}

/// The list holds the store of every member of `L`.
///
/// A walk of `L` in two steps: the empty list is held by any list, and
/// `Cons<H, T>` is held where the store `H` stands for is held and `T` is
/// held. Each step asks `Holds`, so the store that is missing is named in
/// the failure, and a wrapper is resolved to its store through
/// `Member::Store`. The two impls are on `Empty` and on `Cons` and do not
/// overlap. Sealed: the walk is the only legitimate shape.
#[allow(private_bounds)]
#[diagnostic::on_unimplemented(
    message = "store bundle `{Self}` does not hold every store of `{L}`",
    note = "Each member of `{L}` is checked on its own, and the store that is missing is named in the `Holds` failure reported with this one."
)]
pub trait HoldsAll<L>: seal::Walk<L> {}

// Neither impl is kept out of a diagnostic: the compiler walks into the step
// that failed and reports the `Holds` it asked, which names the store.
impl<S: List> HoldsAll<Empty> for S {}

impl<S, H: Member, T> HoldsAll<Cons<H, T>> for S where S: List + Holds<H::Store> + HoldsAll<T> {}
