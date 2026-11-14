//! The value function on the engine's context.
//!
//! `map` hands a unit's function the values of its read list at each record
//! and stores what it returns for its write list. Both halves are walks over
//! the lists in step with the projected bundles: a column member takes the
//! head of the column bundle, a resource the head of the snapshot bundle, an
//! accumulator the head of the accumulator bundle and a virtual the head of
//! the stamp bundle. The bundles were projected from the same lists in the
//! same sequence, so no walk needs a place witness.
//!
//! Whether a write is compared and whether it marks its store is settled per
//! member and per context when the walk is compiled. An ordinary unit's plain
//! column is stored and nothing else. A buffered column, and any column a
//! maintenance unit writes, follows the member's rule: bare or `Differs`
//! marks the store where the value differs from the one held, and `Written`
//! marks it for having been written.

use arvo::{Bool, Cap, USize};
use hilavitkutin_api::access::{AccessSet, Cons, Empty};
use hilavitkutin_api::context::ResolveMap;
use hilavitkutin_api::store::{Accum, BufferMode, Buffered, Column, Resource, Virtual};
use hilavitkutin_api::values::{RCons, RNil, Readable, Writable};
use hilavitkutin_api::{
    ColumnValue,
    Differs,
    Reach,
    ReadSet,
    ReadVals,
    ResourceRead,
    Window,
    WriteSet,
    WriteVals,
    Written,
};
use notko::Maybe;
use notko_hlist::{Elem, Project, Val};

use super::col_select::ColMember;
use super::{AccPtrCons, ColPtrCons, ColRef, EngineCtx, MetaRank, SnapCons, VirtCons};
use crate::resource::bindings::ColKind;

// ---------------------------------------------------------------------
// The read half.
// ---------------------------------------------------------------------

/// A column member a function reads: the record, or a window around it.
pub trait ColRead: ColMember + Readable {
    /// What the function is handed for the member at record `at`.
    ///
    /// # Safety
    ///
    /// `col` is the member's column, `at` is a record of it, and nothing
    /// writes the base it reads from while the returned value lives.
    unsafe fn read_at<'a>(col: &'a ColRef<Self::T>, at: USize) -> <Self as Readable>::Val<'a>;
}

/// The record at `at` of the base every read of the pass sees.
#[inline(always)]
unsafe fn record<T: ColumnValue>(col: &ColRef<T>, at: USize) -> T {
    // SAFETY: the caller holds that `at` is a record of the column and that
    // the base is unwritten for the read.
    unsafe { core::ptr::read(col.read.as_ptr().add(at.0)) }
}

/// A window around the record at `at`, bounded by the column's record count.
#[inline(always)]
unsafe fn window<'a, T: ColumnValue, const N: Cap>(col: &ColRef<T>, at: USize) -> Window<'a, T, N> {
    // SAFETY: the base holds `count` records and `at` is one of them. For a
    // plain column the plan put every writer of it behind a phase boundary
    // and refused a unit that writes what it reads with a reach; for a
    // buffered column the base is the frozen side, which no unit writes.
    unsafe { Window::over(col.read.as_ptr(), at, col.count) }
}

impl<T: ColumnValue> ColRead for Column<T> {
    #[inline(always)]
    unsafe fn read_at(col: &ColRef<T>, at: USize) -> T {
        // SAFETY: forwarded.
        unsafe { record(col, at) }
    }
}

impl<T: ColumnValue, M: BufferMode + ColKind> ColRead for Buffered<T, M> {
    #[inline(always)]
    unsafe fn read_at(col: &ColRef<T>, at: USize) -> T {
        // SAFETY: forwarded.
        unsafe { record(col, at) }
    }
}

impl<T: ColumnValue, const N: Cap> ColRead for Reach<Column<T>, N> {
    #[inline(always)]
    unsafe fn read_at<'a>(col: &'a ColRef<T>, at: USize) -> Window<'a, T, N> {
        // SAFETY: forwarded.
        unsafe { window(col, at) }
    }
}

impl<T: ColumnValue, M: BufferMode + ColKind, const N: Cap> ColRead for Reach<Buffered<T, M>, N> {
    #[inline(always)]
    unsafe fn read_at<'a>(col: &'a ColRef<T>, at: USize) -> Window<'a, T, N> {
        // SAFETY: forwarded.
        unsafe { window(col, at) }
    }
}

/// Form the read product of a read list at one record.
///
/// `RB` is the snapshot bundle and `RC` the read column bundle the context
/// projected for the list.
pub trait Gather<RB, RC>: ReadSet {
    /// The values of the list at record `at`.
    ///
    /// # Safety
    ///
    /// `at` is a record of every column of the list, and nothing writes a
    /// base the list reads while the product lives.
    unsafe fn gather<'a>(reads: &'a RB, cols: &'a RC, at: USize) -> Self::Vals<'a>;
}

impl<RB, RC> Gather<RB, RC> for Empty {
    #[inline(always)]
    unsafe fn gather<'a>(_reads: &'a RB, _cols: &'a RC, _at: USize) -> RNil {
        RNil
    }
}

impl<C: ColRead, Tail: Gather<RB, RC>, RB, RC> Gather<RB, ColPtrCons<C::T, RC, C::K>>
    for Cons<C, Tail>
{
    #[inline(always)]
    unsafe fn gather<'a>(
        reads: &'a RB,
        cols: &'a ColPtrCons<C::T, RC, C::K>,
        at: USize,
    ) -> Self::Vals<'a> {
        RCons {
            // SAFETY: the head of the bundle is this member's column, and the
            // caller's guarantee covers it.
            head: unsafe { C::read_at(&cols.head, at) },
            // SAFETY: forwarded.
            tail: unsafe { Tail::gather(reads, &cols.tail, at) },
        }
    }
}

impl<T: ResourceRead, Tail: Gather<RB, RC>, RB, RC> Gather<SnapCons<T, RB>, RC>
    for Cons<Resource<T>, Tail>
{
    #[inline(always)]
    unsafe fn gather<'a>(reads: &'a SnapCons<T, RB>, cols: &'a RC, at: USize) -> Self::Vals<'a> {
        RCons {
            // The snapshot the context holds, read by value or in place as
            // the value type says.
            head: reads.head.read(),
            // SAFETY: forwarded.
            tail: unsafe { Tail::gather(&reads.tail, cols, at) },
        }
    }
}

// ---------------------------------------------------------------------
// The write half.
// ---------------------------------------------------------------------

/// A column member a function returns a value for.
pub trait ColWrite:
    ColMember + Writable<Val = <Self as ColMember>::T> + Elem<Value = <Self as ColMember>::T>
{
    /// Store `v` at record `at` and say whether that changed the store, by
    /// the member's rule and the context's rank `MP`.
    ///
    /// # Safety
    ///
    /// `col` is the member's column, `at` is a record of it, and the unit
    /// holds the writer's slot for the record.
    unsafe fn land<MP: MetaRank>(col: &ColRef<Self::T>, at: USize, v: Self::T) -> Bool;
}

/// Store under a rule that compares: a bare member and `Differs`.
#[inline(always)]
unsafe fn land_compared<T: ColumnValue + PartialEq, K: ColKind, MP: MetaRank>(
    col: &ColRef<T>,
    at: USize,
    v: T,
) -> Bool {
    if K::BUFFERED.0 {
        // The live side takes the value whatever the frozen side holds, so
        // that the two are equal after the boundary. The change is against
        // the frozen side, the value the pass read.
        // SAFETY: the caller's guarantee. The frozen side holds a value at
        // every record from the drain on, which wrote the store's start to
        // both sides, and no unit writes it while the pass runs.
        let changed = unsafe { core::ptr::read(col.read.as_ptr().add(at.0)) != v };
        // SAFETY: the caller's guarantee.
        unsafe { core::ptr::write(col.write.as_ptr().add(at.0), v) };
        Bool(changed)
    } else if MP::MAINTAINS.0 {
        // SAFETY: the caller's guarantee; the slot is this unit's.
        let changed = unsafe { core::ptr::read(col.read.as_ptr().add(at.0)) != v };
        if changed {
            // SAFETY: the caller's guarantee.
            unsafe { core::ptr::write(col.write.as_ptr().add(at.0), v) };
        }
        Bool(changed)
    } else {
        // An ordinary unit's plain column: no compare, no mark.
        // SAFETY: the caller's guarantee.
        unsafe { core::ptr::write(col.write.as_ptr().add(at.0), v) };
        Bool::FALSE
    }
}

/// Store under `Written`: no compare, and the store is marked for having
/// been written wherever a mark is kept.
#[inline(always)]
unsafe fn land_written<T: ColumnValue, K: ColKind, MP: MetaRank>(
    col: &ColRef<T>,
    at: USize,
    v: T,
) -> Bool {
    // SAFETY: the caller's guarantee.
    unsafe { core::ptr::write(col.write.as_ptr().add(at.0), v) };
    Bool(K::BUFFERED.0 || MP::MAINTAINS.0)
}

impl<T: ColumnValue + PartialEq> ColWrite for Column<T> {
    #[inline(always)]
    unsafe fn land<MP: MetaRank>(col: &ColRef<T>, at: USize, v: T) -> Bool {
        // SAFETY: forwarded.
        unsafe { land_compared::<T, <Self as ColMember>::K, MP>(col, at, v) }
    }
}

impl<T: ColumnValue + PartialEq, M: BufferMode + ColKind> ColWrite for Buffered<T, M> {
    #[inline(always)]
    unsafe fn land<MP: MetaRank>(col: &ColRef<T>, at: USize, v: T) -> Bool {
        // SAFETY: forwarded.
        unsafe { land_compared::<T, M, MP>(col, at, v) }
    }
}

impl<C: ColMember> ColWrite for Differs<C>
where
    C::T: PartialEq,
    Differs<C>: Writable<Val = C::T> + Elem<Value = C::T>,
{
    #[inline(always)]
    unsafe fn land<MP: MetaRank>(col: &ColRef<C::T>, at: USize, v: C::T) -> Bool {
        // SAFETY: forwarded.
        unsafe { land_compared::<C::T, C::K, MP>(col, at, v) }
    }
}

impl<C: ColMember> ColWrite for Written<C>
where
    Written<C>: Writable<Val = C::T> + Elem<Value = C::T>,
{
    #[inline(always)]
    unsafe fn land<MP: MetaRank>(col: &ColRef<C::T>, at: USize, v: C::T) -> Bool {
        // SAFETY: forwarded.
        unsafe { land_written::<C::T, C::K, MP>(col, at, v) }
    }
}

/// Store the write product of a write list at one record.
///
/// `WC`, `WA` and `WV` are the write column, accumulator and virtual bundles
/// the context projected for the list, and `MP` its meta pointer.
pub trait Scatter<WC, WA, WV, MP>: WriteSet {
    /// One flag per column of the list: whether a record changed it.
    type Flags: Copy;

    /// No column changed.
    const NONE: Self::Flags;

    /// Store `vals` at record `at`, fold each column's change into `flags`,
    /// append each present accumulator value and fire each set virtual at
    /// `epoch`.
    ///
    /// # Safety
    ///
    /// `at` is a record of every column of the list, and the unit holds the
    /// writer's slot for it in each.
    unsafe fn scatter(
        vals: &<Self as Project>::Prod<Val>,
        cols: &WC,
        accs: &WA,
        virts: &WV,
        epoch: USize,
        at: USize,
        flags: &mut Self::Flags,
    );

    /// Set the changed mark of each column whose flag is set.
    fn commit(cols: &WC, flags: &Self::Flags);
}

impl<WC, WA, WV, MP> Scatter<WC, WA, WV, MP> for Empty {
    type Flags = ();

    const NONE: () = ();

    #[inline(always)]
    unsafe fn scatter(
        _vals: &<Self as Project>::Prod<Val>,
        _cols: &WC,
        _accs: &WA,
        _virts: &WV,
        _epoch: USize,
        _at: USize,
        _flags: &mut (),
    ) {
    }

    #[inline(always)]
    fn commit(_cols: &WC, _flags: &()) {}
}

impl<C: ColWrite, Tail: Scatter<WC, WA, WV, MP>, WC, WA, WV, MP: MetaRank>
    Scatter<ColPtrCons<C::T, WC, C::K>, WA, WV, MP> for Cons<C, Tail>
{
    type Flags = (Bool, Tail::Flags);

    const NONE: Self::Flags = (Bool::FALSE, Tail::NONE);

    #[inline(always)]
    unsafe fn scatter(
        vals: &<Self as Project>::Prod<Val>,
        cols: &ColPtrCons<C::T, WC, C::K>,
        accs: &WA,
        virts: &WV,
        epoch: USize,
        at: USize,
        flags: &mut Self::Flags,
    ) {
        // SAFETY: the head of the bundle is this member's column, and the
        // caller's guarantee covers it.
        let changed = unsafe { C::land::<MP>(&cols.head, at, vals.head) };
        flags.0 = Bool(flags.0.0 | changed.0);
        // SAFETY: forwarded.
        unsafe { Tail::scatter(&vals.tail, &cols.tail, accs, virts, epoch, at, &mut flags.1) };
    }

    #[inline(always)]
    fn commit(cols: &ColPtrCons<C::T, WC, C::K>, flags: &Self::Flags) {
        if flags.0.0 {
            cols.head.mark.set();
        }
        Tail::commit(&cols.tail, &flags.1);
    }
}

impl<'f, T: ColumnValue, Tail: Scatter<WC, WA, WV, MP>, WC, WA, WV, MP>
    Scatter<WC, AccPtrCons<'f, T, WA>, WV, MP> for Cons<Accum<T>, Tail>
{
    type Flags = Tail::Flags;

    const NONE: Self::Flags = Tail::NONE;

    #[inline(always)]
    unsafe fn scatter(
        vals: &<Self as Project>::Prod<Val>,
        cols: &WC,
        accs: &AccPtrCons<'f, T, WA>,
        virts: &WV,
        epoch: USize,
        at: USize,
        flags: &mut Self::Flags,
    ) {
        // A present value is appended, an absent one is not.
        if let Maybe::Is(v) = vals.head {
            accs.head.append(v);
        }
        // SAFETY: forwarded.
        unsafe { Tail::scatter(&vals.tail, cols, &accs.tail, virts, epoch, at, flags) };
    }

    #[inline(always)]
    fn commit(cols: &WC, flags: &Self::Flags) {
        Tail::commit(cols, flags);
    }
}

impl<'f, V: 'static, Tail: Scatter<WC, WA, WV, MP>, WC, WA, WV, MP>
    Scatter<WC, WA, VirtCons<'f, V, WV>, MP> for Cons<Virtual<V>, Tail>
{
    type Flags = Tail::Flags;

    const NONE: Self::Flags = Tail::NONE;

    #[inline(always)]
    unsafe fn scatter(
        vals: &<Self as Project>::Prod<Val>,
        cols: &WC,
        accs: &WA,
        virts: &VirtCons<'f, V, WV>,
        epoch: USize,
        at: USize,
        flags: &mut Self::Flags,
    ) {
        // A set virtual is fired, an unset one is not.
        if vals.head.0 {
            virts.head.set(epoch);
        }
        // SAFETY: forwarded.
        unsafe { Tail::scatter(&vals.tail, cols, accs, &virts.tail, epoch, at, flags) };
    }

    #[inline(always)]
    fn commit(cols: &WC, flags: &Self::Flags) {
        Tail::commit(cols, flags);
    }
}

// ---------------------------------------------------------------------
// The bridge.
// ---------------------------------------------------------------------

impl<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP> ResolveMap<R, W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
where
    R: AccessSet + Gather<RBundle, RCols>,
    W: AccessSet + Scatter<WCols, WAccum, WVirt, MP>,
{
    #[inline]
    fn resolve_map<F>(&self, f: F)
    where
        F: for<'a> Fn(&ReadVals<'a, R>) -> WriteVals<W>,
    {
        let start = self.morsel.start;
        let len = self.morsel.len;
        // The flags live in registers across the morsel, and each store's
        // mark is set once after it, not per record.
        let mut flags = <W as Scatter<WCols, WAccum, WVirt, MP>>::NONE;
        let mut i = 0; // lint:allow(no-bare-numeric) reason: morsel-relative record index; tracked: #121
        while i < len.0 {
            let at = USize(start.0 + i); // lint:allow(no-bare-numeric) reason: absolute record index of the morsel's record; tracked: #121
            // SAFETY: `at` is inside the morsel the dispatch handed this
            // unit, which lies inside every column of its lists. The plan
            // proved no other unit writes what this one reads at these
            // records while it runs.
            let read = ReadVals::<R>(unsafe { R::gather(&self.reads, &self.read_cols, at) });
            let out = f(&read);
            // SAFETY: the plan proved this unit holds the writer's slot for
            // each of its write columns at the morsel's records.
            unsafe {
                W::scatter(
                    &out.0,
                    &self.write_cols,
                    &self.write_accums,
                    &self.write_virtuals,
                    self.epoch,
                    at,
                    &mut flags,
                );
            }
            i += 1; // lint:allow(no-bare-numeric) reason: record index step; tracked: #121
        }
        W::commit(&self.write_cols, &flags);
    }
}
