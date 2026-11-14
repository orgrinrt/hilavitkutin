//! Two shapes the api revision names, tried outside the real crates.
//!
//! The first is a column held in two sides. Every read sees the frozen side
//! and every write lands on the live side. At the frame boundary the live
//! side becomes the frozen one, by a copy (`Snapshot`) or by the two sides
//! trading places (`Exchange`), and which of the two is a type parameter of
//! the store's marker.
//!
//! The second is the positional `write` following the member a unit lists a
//! column under: a bare column, `Written` or `Differs`. The call keeps the
//! two generic arguments it has on the trunk (`write::<T, _>`).
//!
//! Nothing here times anything. It is a feasibility sketch.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use core::cell::Cell;
use core::marker::PhantomData;

// ----- the type-level list, as the shared crate has it -----

/// The end of a list.
pub struct Empty;
/// One cell of a list.
pub struct Cons<H, T>(PhantomData<(H, T)>);

// ----- store markers -----

/// A column written in place.
pub struct Column<T>(PhantomData<T>);
/// A column held in two sides, read frozen and written live.
pub struct Buffered<T, M = Snapshot>(PhantomData<(T, M)>);

/// The live side is copied into the frozen side at the boundary.
pub struct Snapshot;
/// The two sides trade places at the boundary.
pub struct Exchange;

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Snapshot {}
    impl Sealed for super::Exchange {}
}

/// How the live side of a buffered store becomes its frozen side.
pub trait BufferMode: sealed::Sealed + 'static {
    /// Whether the boundary copies. Where it does not, the sides trade places.
    const COPIES: bool;
}
impl BufferMode for Snapshot {
    const COPIES: bool = true;
}
impl BufferMode for Exchange {
    #[cfg(not(feature = "mutant-exchange-copies"))]
    const COPIES: bool = false;
    #[cfg(feature = "mutant-exchange-copies")]
    const COPIES: bool = true;
}

// ----- write members -----

/// Write the store, changed wherever the unit wrote it.
pub struct Written<C>(PhantomData<C>);
/// Write the store, changed only where the value differs.
pub struct Differs<C>(PhantomData<C>);

// ----- part one: the two sides -----

/// What a write member says of a change.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Rule {
    /// Changed wherever a value was written.
    Written,
    /// Changed only where the written value differs from the frozen one.
    Differs,
}

/// The storage of one buffered store: two regions that never move, and
/// which of them is frozen.
pub struct Sides<'s, T, M> {
    a:           &'s mut [T],
    b:           &'s mut [T],
    frozen_is_a: bool,
    changed:     bool,
    mode:        PhantomData<M>,
}

/// A read of the frozen side at one record, with its neighbours.
pub struct Window<'w, T> {
    side: &'w [T],
    at:   usize,
}

impl<T: Copy> Window<'_, T> {
    /// The value at the record.
    pub fn here(&self) -> T {
        self.side[self.at]
    }

    /// The value `k` records earlier, nothing before the first record.
    pub fn before(&self, k: usize) -> Option<T> {
        self.at.checked_sub(k).map(|i| self.side[i])
    }

    /// The value `k` records later, nothing past the last.
    pub fn after(&self, k: usize) -> Option<T> {
        self.side.get(self.at + k).copied()
    }
}

impl<'s, T: Copy + PartialEq, M: BufferMode> Sides<'s, T, M> {
    /// Two regions of one length. Side `a` starts frozen.
    pub fn new(a: &'s mut [T], b: &'s mut [T]) -> Self {
        assert!(a.len() == b.len(), "the two sides hold one record count");
        Self {
            a,
            b,
            frozen_is_a: true,
            changed: false,
            mode: PhantomData,
        }
    }

    /// The frozen side: the store as it stood when the frame began.
    pub fn frozen(&self) -> &[T] {
        if self.frozen_is_a { self.a } else { self.b }
    }

    fn split(&mut self) -> (&[T], &mut [T]) {
        if self.frozen_is_a {
            (&*self.a, &mut *self.b)
        } else {
            (&*self.b, &mut *self.a)
        }
    }

    /// One writer's run: `f` is applied at every record of `records`, reads
    /// the frozen side through a window and returns the value for the live
    /// side at that record.
    pub fn step<I: Iterator<Item = usize>, F: Fn(Window<'_, T>) -> T>(
        &mut self,
        records: I,
        rule: Rule,
        f: F,
    ) {
        let mut changed = false;
        {
            let (frozen, live) = self.split();
            for at in records {
                #[cfg(not(feature = "mutant-read-live"))]
                let v = f(Window {
                    side: frozen,
                    at,
                });
                #[cfg(feature = "mutant-read-live")]
                let v = f(Window {
                    side: &*live,
                    at,
                });
                changed |= match rule {
                    Rule::Written => true,
                    Rule::Differs => v != frozen[at],
                };
                live[at] = v;
            }
        }
        self.changed |= changed;
    }

    /// The frame boundary. Returns whether the live side became the frozen
    /// one. A store nobody changed is left alone.
    pub fn close(&mut self) -> bool {
        #[cfg(not(feature = "mutant-swap-when-unwritten"))]
        if !self.changed {
            return false;
        }
        self.changed = false;
        if M::COPIES {
            if self.frozen_is_a {
                self.a.copy_from_slice(self.b);
            } else {
                self.b.copy_from_slice(self.a);
            }
        } else {
            self.frozen_is_a = !self.frozen_is_a;
        }
        true
    }
}

// ----- part two: the positional write follows the member -----

/// The index of a bare column in a write list.
pub struct AtPlain;
/// The index of a column listed under `Written`.
pub struct AtWritten;
/// The index of a column listed under `Differs`.
pub struct AtDiffers;
/// One cell further along.
pub struct Next<I>(PhantomData<I>);

/// A write list that holds a column of `T`, found at `I`, under one of the
/// three write members.
pub trait WritesColumn<T, I> {
    /// Store `v` in `cell` and say whether the store counts as changed.
    fn land(cell: &mut T, v: T) -> bool;
}

impl<T, Tail> WritesColumn<T, AtPlain> for Cons<Column<T>, Tail> {
    // The default of a bare member under the positional write. The sketch
    // takes "written"; which of the two it should be waits on a bench.
    fn land(cell: &mut T, v: T) -> bool {
        *cell = v;
        true
    }
}

impl<T, Tail> WritesColumn<T, AtWritten> for Cons<Written<Column<T>>, Tail> {
    fn land(cell: &mut T, v: T) -> bool {
        *cell = v;
        true
    }
}

impl<T: PartialEq, Tail> WritesColumn<T, AtDiffers> for Cons<Differs<Column<T>>, Tail> {
    fn land(cell: &mut T, v: T) -> bool {
        let differs = *cell != v;
        if differs {
            *cell = v;
        }
        differs
    }
}

impl<T, H, Tail: WritesColumn<T, I>, I> WritesColumn<T, Next<I>> for Cons<H, Tail> {
    fn land(cell: &mut T, v: T) -> bool {
        Tail::land(cell, v)
    }
}

// The runner's side. On the trunk a context resolves a column through a
// pointer bundle of its own, keyed by the value type, with an index of its
// own. The bundle need not be in the list's sequence.

/// The end of a pointer bundle.
pub struct PtrNil;
/// One pointer of a bundle, for the column of `T`.
pub struct PtrCons<T, Tail>(PhantomData<(T, Tail)>);
/// The bundle's index of its first pointer.
pub struct Here;
/// One pointer further along.
pub struct There<I>(PhantomData<I>);

/// A bundle that holds the pointer of the column of `T`, found at `I`.
pub trait ResolveSlot<T, I> {
    /// The pointer's position in the bundle.
    const SLOT: usize;
}
impl<T, Tail> ResolveSlot<T, Here> for PtrCons<T, Tail> {
    const SLOT: usize = 0;
}
impl<T, H, Tail: ResolveSlot<T, I>, I> ResolveSlot<T, There<I>> for PtrCons<H, Tail> {
    const SLOT: usize = 1 + Tail::SLOT;
}

/// The list's member and the bundle's pointer for one column, found
/// together under one inferred index, so the call names the value type and
/// nothing else.
pub trait WriteAt<T, I> {
    /// The pointer's position in the bundle.
    const SLOT: usize;
    /// The member's rule.
    fn land(cell: &mut T, v: T) -> bool;
}
impl<W, P, T, Ik, Ib> WriteAt<T, (Ik, Ib)> for (W, P)
where
    W: WritesColumn<T, Ik>,
    P: ResolveSlot<T, Ib>,
{
    const SLOT: usize = P::SLOT;

    fn land(cell: &mut T, v: T) -> bool {
        W::land(cell, v)
    }
}

/// The write side of one unit's context: its write list `W`, and a base
/// and a changed mark for each pointer of its bundle `P`.
pub struct Writer<'f, W, P, const N: usize> {
    bases: [*mut u8; N],
    marks: &'f [Cell<bool>; N],
    types: PhantomData<(W, P)>,
}

impl<'f, W, P, const N: usize> Writer<'f, W, P, N> {
    /// # Safety
    ///
    /// `bases[k]` points at the column the `k`-th pointer of `P` is for,
    /// valid for writes for `'f` at every position the caller will name.
    pub unsafe fn new(bases: [*mut u8; N], marks: &'f [Cell<bool>; N]) -> Self {
        Self {
            bases,
            marks,
            types: PhantomData,
        }
    }

    /// Write `v` at record `i` of the unit's column of `T`.
    ///
    /// # Safety
    ///
    /// `i` is below the column's record count and the plan gave the unit
    /// the record.
    pub unsafe fn write<T, I>(&self, i: usize, v: T)
    where
        (W, P): WriteAt<T, I>,
    {
        let slot = <(W, P) as WriteAt<T, I>>::SLOT;
        // SAFETY: the constructor's promise gives a valid column of `T` at
        // this slot, and the caller's gives a valid `i` in it.
        let cell = unsafe { &mut *self.bases[slot].cast::<T>().add(i) };
        if <(W, P) as WriteAt<T, I>>::land(cell, v) {
            self.marks[slot].set(true);
        }
    }
}

#[cfg(feature = "refused-differs-without-eq")]
mod refused_differs_without_eq {
    use super::*;

    pub struct NoEq(pub u32);

    type List = Cons<Differs<Column<NoEq>>, Empty>;
    type Bundle = PtrCons<NoEq, PtrNil>;

    pub fn must_not_compile(w: &Writer<'_, List, Bundle, 1>) {
        // SAFETY: never run; the function exists to be refused.
        unsafe { w.write::<NoEq, _>(0, NoEq(1)) };
    }
}

#[cfg(feature = "refused-positional-write-of-buffered")]
mod refused_positional_write_of_buffered {
    use super::*;

    type List = Cons<Buffered<u32, Exchange>, Empty>;
    type Bundle = PtrCons<u32, PtrNil>;

    pub fn must_not_compile(w: &Writer<'_, List, Bundle, 1>) {
        // SAFETY: never run; the function exists to be refused.
        unsafe { w.write::<u32, _>(0, 1) };
    }
}

#[cfg(test)]
mod tests;
