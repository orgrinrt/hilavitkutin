//! Build the projected column pointer bundle and the projected
//! accumulator handle bundle from a column source / the bindings.
//!
//! Split out of `dispatch/engine_ctx.rs` (file-size lint). No behaviour
//! change.

use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::store::{Accum, Column, Resource, Virtual};

use super::selectors::{AccumSelector, ColSelector};
use super::{AccPtrCons, AccPtrNil, ColPtrCons, ColPtrNil};

// ---------------------------------------------------------------------
// ColProject: build the projected column bundle from a column source.
//
// `ColProject<Set, Indices>` recurses on the `Column<T>` members of an
// access set `Set`, pulling each matching `ColumnPtr<T>` out of a column
// source via `ColSelector`. `Indices` is a parallel cons-list whose
// elements are the per-member selector indices; carrying it as a trait
// type parameter constrains each index (dodging E0207). This is the
// column analogue of `Project`: it forces the projected column bundle to
// be the projection of the access set over the supplied source, so a
// caller cannot hand a mismatched bundle.
//
// `Resource<T>` and `Virtual<T>` members produce no column-bundle node:
// only the column members contribute.
// ---------------------------------------------------------------------

/// Project the `Column<T>` members of `Set` out of a column source `C`
/// into a column pointer bundle.
pub trait ColProject<Set, Indices> {
    /// The projected column bundle.
    type Out;

    /// Build the projected bundle by pulling each column pointer.
    fn col_project(&self) -> Self::Out;
}

impl<C> ColProject<Empty, Empty> for C {
    type Out = ColPtrNil;

    #[inline(always)]
    fn col_project(&self) -> ColPtrNil {
        ColPtrNil
    }
}

// Column head: pull the pointer, recurse on the tail.
impl<C, T, I, STail, ITail> ColProject<Cons<Column<T>, STail>, Cons<I, ITail>> for C
where
    C: ColSelector<T, I>,
    C: ColProject<STail, ITail>,
{
    type Out = ColPtrCons<T, <C as ColProject<STail, ITail>>::Out>;

    #[inline(always)]
    fn col_project(&self) -> Self::Out {
        ColPtrCons {
            head: <C as ColSelector<T, I>>::get(self),
            tail: <C as ColProject<STail, ITail>>::col_project(self),
        }
    }
}

// Resource head: no column node, recurse on the tail with the same
// index list (resources do not consume a column selector index).
impl<C, T, STail, Indices> ColProject<Cons<Resource<T>, STail>, Indices> for C
where
    C: ColProject<STail, Indices>,
{
    type Out = <C as ColProject<STail, Indices>>::Out;

    #[inline(always)]
    fn col_project(&self) -> Self::Out {
        <C as ColProject<STail, Indices>>::col_project(self)
    }
}

// Virtual head: no column node, recurse on the tail.
impl<C, T, STail, Indices> ColProject<Cons<Virtual<T>, STail>, Indices> for C
where
    C: ColProject<STail, Indices>,
{
    type Out = <C as ColProject<STail, Indices>>::Out;

    #[inline(always)]
    fn col_project(&self) -> Self::Out {
        <C as ColProject<STail, Indices>>::col_project(self)
    }
}

// Accum head: no column node, recurse on the tail (accumulators project
// through `AccumProject`, not `ColProject`).
impl<C, T, STail, Indices> ColProject<Cons<Accum<T>, STail>, Indices> for C
where
    C: ColProject<STail, Indices>,
{
    type Out = <C as ColProject<STail, Indices>>::Out;

    #[inline(always)]
    fn col_project(&self) -> Self::Out {
        <C as ColProject<STail, Indices>>::col_project(self)
    }
}

// ---------------------------------------------------------------------
// AccumProject: build the projected accumulator bundle from the bindings.
//
// `AccumProject<'s, Set, Indices>` recurses on the `Accum<T>` members of an
// access set `Set`, pulling each matching `AccumColPtr<'s, T>` out of a source
// via `AccumSelector`. The accumulator analogue of `ColProject`, with one
// difference: the projected bundle retains a `'s` borrow of the source (the
// live-length cells), so the trait carries the lifetime `'s` and projects via
// `&'s self`. Carrying `'s` at the trait (rather than a GAT `Out<'s>`) keeps
// `Out` a plain associated type, so the `project` constructor can tie it to
// the Context's `WAccum` parameter through an `Out = WAccum` equality bound;
// the de-risk sketch used a free function that named the GAT directly, which a
// method on the `WAccum`-generic `EngineCtx` cannot.
//
// `Indices` is a parallel cons-list of per-member selector indices, carried as
// a trait type parameter to constrain each index (dodging E0207). `Resource<T>`
// / `Column<T>` / `Virtual<T>` members produce no accumulator-bundle node.
// ---------------------------------------------------------------------

/// Project the `Accum<T>` members of `Set` out of a source `C` into an
/// accumulator pointer bundle borrowing `C` for `'s`.
pub trait AccumProject<'s, Set, Indices> {
    /// The projected accumulator bundle (borrows the source for `'s`).
    type Out;

    /// Build the projected bundle by pulling each accumulator handle.
    fn acc_project(&'s self) -> Self::Out;
}

impl<'s, C> AccumProject<'s, Empty, Empty> for C {
    type Out = AccPtrNil;

    #[inline(always)]
    fn acc_project(&'s self) -> AccPtrNil {
        AccPtrNil
    }
}

// Accum head: pull the handle, recurse on the tail.
impl<'s, C, T, I, STail, ITail> AccumProject<'s, Cons<Accum<T>, STail>, Cons<I, ITail>> for C
where
    C: AccumSelector<T, I>,
    C: AccumProject<'s, STail, ITail>,
{
    type Out = AccPtrCons<'s, T, <C as AccumProject<'s, STail, ITail>>::Out>;

    #[inline(always)]
    fn acc_project(&'s self) -> Self::Out {
        AccPtrCons {
            head: <C as AccumSelector<T, I>>::get(self),
            tail: <C as AccumProject<'s, STail, ITail>>::acc_project(self),
        }
    }
}

// Resource head: no accumulator node, recurse on the tail with the same
// index list (resources do not consume an accumulator selector index).
impl<'s, C, T, STail, Indices> AccumProject<'s, Cons<Resource<T>, STail>, Indices> for C
where
    C: AccumProject<'s, STail, Indices>,
{
    type Out = <C as AccumProject<'s, STail, Indices>>::Out;

    #[inline(always)]
    fn acc_project(&'s self) -> Self::Out {
        <C as AccumProject<'s, STail, Indices>>::acc_project(self)
    }
}

// Column head: no accumulator node, recurse on the tail.
impl<'s, C, T, STail, Indices> AccumProject<'s, Cons<Column<T>, STail>, Indices> for C
where
    C: AccumProject<'s, STail, Indices>,
{
    type Out = <C as AccumProject<'s, STail, Indices>>::Out;

    #[inline(always)]
    fn acc_project(&'s self) -> Self::Out {
        <C as AccumProject<'s, STail, Indices>>::acc_project(self)
    }
}

// Virtual head: no accumulator node, recurse on the tail.
impl<'s, C, T, STail, Indices> AccumProject<'s, Cons<Virtual<T>, STail>, Indices> for C
where
    C: AccumProject<'s, STail, Indices>,
{
    type Out = <C as AccumProject<'s, STail, Indices>>::Out;

    #[inline(always)]
    fn acc_project(&'s self) -> Self::Out {
        <C as AccumProject<'s, STail, Indices>>::acc_project(self)
    }
}
