//! AccessSet and Contains: compile-time store membership.
//!
//! `AccessSet` is sealed on the cons-list typestate: `Empty` and
//! `Cons<H, T>`. A WU that reads `Column<X>` constrains its `Read`
//! set to implement `Contains<Column<X>>`. Violating that at a call
//! site fails to compile.

use arvo::USize;
use arvo::strategy::Identity;

use crate::sealed;

/// Sealed marker on the cons-list typestate.
///
/// Implemented for `Empty` and recursively for `Cons<H, T>` where
/// `T: AccessSet`. `LEN` reports the cons-list cardinality.
#[allow(private_bounds)]
pub trait AccessSet: sealed::Sealed + 'static {
    /// Number of member types in the set.
    const LEN: USize;
}

/// Sealed membership witness: the implementing `AccessSet` contains
/// `S` as one of its cons-list members.
///
/// Provider API where-clauses reference `Contains<Column<T>>` or
/// `Contains<Resource<T>>` on the WU-declared set. Accessing a store
/// that was not declared is a compile error.
///
/// Marker trait (`#[marker]`) so the head-match and tail-recurse
/// impls coexist: a cons-list whose head IS `S` and whose tail also
/// contains `S` resolves through whichever impl the trait solver
/// finds first, without coherence conflict.
#[marker]
#[diagnostic::on_unimplemented(
    message = "store `{Self}` does not contain `{S}`",
    note = "Register it with `.add_resource::<T>(initial)`, `.add_column::<T>()`, `.add_virtual::<T>()`, or install a Kit that registers it. If `.build()` reports `overflow evaluating the requirement`, declare `#![recursion_limit = \"1024\"]` at your crate root."
)]
pub trait Contains<S>: AccessSet {}

// ---------------------------------------------------------------------
// HList substrate: Empty + Cons<H, T> primitives.
//
// `Empty` is the cons-list leaf. `Cons<H, T>` is the cons-cell.
// `read![T0, T1]` and `write![T0, T1]` macros (in macros.rs) expand
// to nested `Cons<...>` chains terminated by `Empty`.
// ---------------------------------------------------------------------

/// Cons-list leaf (empty).
pub struct Empty;

/// Cons-list cell carrying head `H` and tail `T`.
pub struct Cons<H, T>(core::marker::PhantomData<(H, T)>);

impl sealed::Sealed for Empty {}
impl AccessSet for Empty {
    const LEN: USize = USize::ZERO;
}

impl<H: 'static, T: 'static> sealed::Sealed for Cons<H, T> {}
impl<H: 'static, T: 'static + AccessSet> AccessSet for Cons<H, T> {
    const LEN: USize = USize(T::LEN.0 + 1);
}

// Head match: Cons<H, T> contains H. Coexists with the recursive
// tail match below under #[marker].
impl<H: 'static, T: 'static + AccessSet> Contains<H> for Cons<H, T> {}

// Recursive tail match: Cons<H, T> contains M if T does.
impl<H: 'static, T: 'static + AccessSet, M: 'static> Contains<M> for Cons<H, T> where T: Contains<M> {}

/// Type-level Cons-list append.
///
/// `<Self as Concat<L>>::Out` produces a cons list with every
/// element of Self followed by every element of L. Used by Kit
/// composition to grow `Wus` and `Stores` bundles.
#[diagnostic::on_unimplemented(
    message = "cannot concatenate `{L}` onto `{Self}`",
    note = "Concat is implemented for `Empty` and `Cons<H, T>` where `T: Concat<L>`. Make sure both sides are cons-lists built from `Empty` and `Cons<H, T>` (the `read!` / `write!` macros emit this shape)."
)]
pub trait Concat<L> {
    type Out;
}

impl<L> Concat<L> for Empty {
    type Out = L;
}

impl<H, T, L> Concat<L> for Cons<H, T>
where
    T: Concat<L>,
{
    type Out = Cons<H, <T as Concat<L>>::Out>;
}

/// Sealed-via-AccessSet membership: every element of `L` is in
/// `Self`.
///
/// Marker trait so per-position impls coexist without coherence
/// overlap. Two impls: base (`L = Empty`) and recursive
/// (`L = Cons<H, T>`, requires `Self: Contains<H> + ContainsAll<T>`).
/// `.build()` carries `Stores: ContainsAll<AccumRead> +
/// ContainsAll<AccumWrite>` as the proof shape.
#[marker]
#[diagnostic::on_unimplemented(
    message = "store bundle `{Self}` does not contain every element of `{L}`",
    note = "Register the missing store with `.add_resource::<T>(initial)`, `.add_column::<T>()`, `.add_virtual::<T>()`, or install a Kit that registers it. If `.build()` reports `overflow evaluating the requirement`, declare `#![recursion_limit = \"1024\"]` at your crate root."
)]
pub trait ContainsAll<L>: AccessSet {}

impl<S: AccessSet> ContainsAll<Empty> for S {}

impl<S, H: 'static, T: 'static> ContainsAll<Cons<H, T>> for S
where
    S: AccessSet + Contains<H> + ContainsAll<T>,
{
}
