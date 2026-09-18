//! Build the projected write-virtual bundle and the projected resource
//! snapshot bundle from the bindings.
//!
//! Split out of `dispatch/engine_ctx.rs` (file-size lint). No behaviour
//! change.

use core::marker::PhantomData;

use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::column_value::ColumnValue;
use hilavitkutin_api::store::{Accum, Column, Resource, Virtual};

use super::selectors::{Selector, VirtualStampSelector};
use super::{SnapCons, SnapNil, VirtCons, VirtNil};

// ---------------------------------------------------------------------
// VirtualProject: build the projected write-virtual bundle from the bindings.
//
// The virtual analogue of `AccumProject`: recurse on the `Virtual<T>` members of
// the write set `W`, pulling each matching stamp cell ref out of the bindings via
// `VirtualStampSelector`. `Resource<T>` / `Column<T>` / `Accum<T>` members
// contribute no virtual-bundle node. `Indices` is the parallel selector-index
// list, inferred at the call site (dodging E0207). The projected bundle borrows
// `&'s`, so it ties to the `'frame`-lived bindings, like the accumulator bundle.
// ---------------------------------------------------------------------

/// Project the `Virtual<T>` members of `W` out of a source `A` into a stamp-cell
/// bundle.
pub trait VirtualProject<'s, Set, Indices> {
    /// The projected write-virtual bundle.
    type Out;

    /// Build the projected bundle by pulling each stamp cell ref.
    fn virt_project(&'s self) -> Self::Out;
}

impl<'s, C> VirtualProject<'s, Empty, Empty> for C {
    type Out = VirtNil;

    #[inline]
    fn virt_project(&'s self) -> VirtNil {
        VirtNil
    }
}

impl<'s, C, T, I, STail, ITail> VirtualProject<'s, Cons<Virtual<T>, STail>, Cons<I, ITail>> for C
where
    C: VirtualStampSelector<T, I>,
    C: VirtualProject<'s, STail, ITail>,
{
    type Out = VirtCons<'s, T, <C as VirtualProject<'s, STail, ITail>>::Out>;

    #[inline]
    fn virt_project(&'s self) -> Self::Out {
        VirtCons {
            head: <C as VirtualStampSelector<T, I>>::vstamp(self),
            tail: <C as VirtualProject<'s, STail, ITail>>::virt_project(self),
            _h:   PhantomData,
        }
    }
}

// Skip non-virtual members of the write set (resource / column / accumulator).

impl<'s, C, T, STail, Indices> VirtualProject<'s, Cons<Resource<T>, STail>, Indices> for C
where
    C: VirtualProject<'s, STail, Indices>,
{
    type Out = <C as VirtualProject<'s, STail, Indices>>::Out;

    #[inline]
    fn virt_project(&'s self) -> Self::Out {
        <C as VirtualProject<'s, STail, Indices>>::virt_project(self)
    }
}

impl<'s, C, T, STail, Indices> VirtualProject<'s, Cons<Column<T>, STail>, Indices> for C
where
    C: VirtualProject<'s, STail, Indices>,
{
    type Out = <C as VirtualProject<'s, STail, Indices>>::Out;

    #[inline]
    fn virt_project(&'s self) -> Self::Out {
        <C as VirtualProject<'s, STail, Indices>>::virt_project(self)
    }
}

impl<'s, C, T, STail, Indices> VirtualProject<'s, Cons<Accum<T>, STail>, Indices> for C
where
    C: VirtualProject<'s, STail, Indices>,
{
    type Out = <C as VirtualProject<'s, STail, Indices>>::Out;

    #[inline]
    fn virt_project(&'s self) -> Self::Out {
        <C as VirtualProject<'s, STail, Indices>>::virt_project(self)
    }
}

// ---------------------------------------------------------------------
// Resource selector: type-keyed lookup over bindings nodes and over the
// projected resource bundle.
// ---------------------------------------------------------------------
//
// (The `Selector` trait itself lives in `super::selectors`; what follows
// is `Project`, which consumes it to build the resource snapshot bundle.)

// ---------------------------------------------------------------------
// Project: build the projected resource bundle from the bindings.
//
// `Project<R, Indices>` recurses on the `Resource<T>` members of the
// access set `R`, pulling each matching `ResourcePtr<T>` out of the
// bindings via `Selector`. `Indices` is a parallel cons-list whose
// elements are the per-member selector indices; carrying it as a trait
// type parameter constrains each index (dodging E0207).
//
// `Column<T>` and `Virtual<T>` members of `R` produce no resource-
// bundle node here: only the resource members contribute. The free
// `project_reads::<R, _, _>(bindings)` helper pins `R` by turbofish.
// ---------------------------------------------------------------------

/// Project the `Resource<T>` members of `R` out of a source `A` into a
/// by-value snapshot bundle.
pub trait Project<R, Indices> {
    /// The projected snapshot bundle.
    type Out;

    /// Build the snapshot bundle by copying each resource value out of
    /// the canonical blob (the domain-19 stack-local caching: the bundle
    /// lands inside the Context, on the dispatch stack frame).
    fn project(&self) -> Self::Out;
}

impl<A> Project<Empty, Empty> for A {
    type Out = SnapNil;

    #[inline(always)]
    fn project(&self) -> SnapNil {
        SnapNil
    }
}

// Resource head: snapshot the value through the binding's backcast
// pointer, recurse on the tail. `T: ColumnValue` (`Copy`), so the copy
// duplicates a plain value; for a collection-bearing value the copy
// carries the member's pointer-plus-length view only, elements stay in
// their own column (live-streamed).
// FIXME: Seq/Map collection members are not yet wired into resource values; the
//        live-stream accessor over the snapshot's ptr+len view lands with that
//        wiring (tracked #344; catalogued test in tests/resource_snapshot.rs).
// FIXME: resources are read-only through the Context; when a mutable resource
//        surface ships, the dispatcher writes mutated snapshots back to
//        canonical storage after the morsel loop (domain 19 write-back half).
impl<A, T: ColumnValue, I, RTail, ITail> Project<Cons<Resource<T>, RTail>, Cons<I, ITail>> for A
where
    A: Selector<T, I>,
    A: Project<RTail, ITail>,
{
    type Out = SnapCons<T, <A as Project<RTail, ITail>>::Out>;

    #[inline(always)]
    fn project(&self) -> Self::Out {
        let ptr = <A as Selector<T, I>>::get(self);
        SnapCons {
            // SAFETY: the selector yielded the binding's backcast pointer
            // for `T`: non-null, aligned, initialised at drain, valid for
            // the bindings' lifetime, which contains this projection.
            head: unsafe { core::ptr::read(ptr.as_ptr()) },
            tail: <A as Project<RTail, ITail>>::project(self),
        }
    }
}

// Column head: no resource node, recurse on the tail with the same
// index list (columns do not consume a resource selector index).
impl<A, T, RTail, Indices> Project<Cons<Column<T>, RTail>, Indices> for A
where
    A: Project<RTail, Indices>,
{
    type Out = <A as Project<RTail, Indices>>::Out;

    #[inline(always)]
    fn project(&self) -> Self::Out {
        <A as Project<RTail, Indices>>::project(self)
    }
}

// Virtual head: no resource node, recurse on the tail.
impl<A, T, RTail, Indices> Project<Cons<Virtual<T>, RTail>, Indices> for A
where
    A: Project<RTail, Indices>,
{
    type Out = <A as Project<RTail, Indices>>::Out;

    #[inline(always)]
    fn project(&self) -> Self::Out {
        <A as Project<RTail, Indices>>::project(self)
    }
}

// Accum head: no resource node, recurse on the tail.
impl<A, T, RTail, Indices> Project<Cons<Accum<T>, RTail>, Indices> for A
where
    A: Project<RTail, Indices>,
{
    type Out = <A as Project<RTail, Indices>>::Out;

    #[inline(always)]
    fn project(&self) -> Self::Out {
        <A as Project<RTail, Indices>>::project(self)
    }
}

/// Project the resource members of `R` out of `bindings` into a bundle.
///
/// Pins `R` by turbofish at the call site; inference fills the parallel
/// `Indices` list and the source type `A`.
#[inline(always)]
pub fn project_reads<R, A, Indices>(bindings: &A) -> <A as Project<R, Indices>>::Out
where
    A: Project<R, Indices>,
{
    bindings.project()
}
