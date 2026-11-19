//! The block form under the engine.
//!
//! Sketch addition, not shipped source. Three things. The engine's projected
//! column bundle is what the engine holds for a list of plain columns
//! (`ColSlots`). The engine's context is the records of one window
//! (`Records`), for every pair of such lists, with each read and write
//! resolved by the list's own witness over the bundle the context already
//! carries. `Blk<U>` registers a block unit: a `WorkUnit` whose context is the
//! computed `CtxFor` over the unit's own lists, and whose `execute` hands that
//! context to the unit's body as it is.
//!
//! Nothing is erased: the body is monomorphised over `EngineCtx`, and a read
//! is the same pointer read `ResolveColumnRead` does.

use arvo::{Bool, USize};
use hilavitkutin_api::access::{AccessSet, Cons, Empty};
use hilavitkutin_api::block::{Block, ColList, ColSlots, ReadsColumn, Records, WritesAt};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::column_value::ColumnValue;
use hilavitkutin_api::work_unit::WorkUnit;

use super::bundles::{AccumBundleOf, ColBundleOf, CtxFor, ResourceBundleOf, VirtBundleOf};
use super::col_select::ColMember;
use super::{ColPtrCons, ColPtrNil, EngineCtx, MetaRank};
use crate::resource::bindings::Plain;

// ---------------------------------------------------------------------
// The projected column bundle, as the slots of a list.
// ---------------------------------------------------------------------

impl ColSlots<Empty> for ColPtrNil {
    type Rest = ColPtrNil;

    #[inline(always)]
    fn rest(&self) -> &ColPtrNil {
        self
    }

    #[inline(always)]
    unsafe fn get(&self, _at: USize) {}

    #[inline(always)]
    unsafe fn put(&self, _at: USize, _v: (), _mark: Bool) {}
}

impl<C, Tail, TB> ColSlots<Cons<C, Tail>> for ColPtrCons<C::T, TB, Plain>
where
    C: ColMember<K = Plain>,
    Tail: ColList,
    Cons<C, Tail>: ColList<Value = C::T, Rest = Tail>,
    TB: ColSlots<Tail>,
{
    type Rest = TB;

    #[inline(always)]
    fn rest(&self) -> &TB {
        &self.tail
    }

    #[inline(always)]
    unsafe fn get(&self, at: USize) -> C::T {
        // SAFETY: the caller's obligation: `at` is a record of this column
        // and the plan gave the unit the record. The same read
        // `ResolveColumnRead::resolve_read` makes.
        unsafe { core::ptr::read(self.head.read.as_ptr().add(at.0)) }
    }

    #[inline(always)]
    unsafe fn put(&self, at: USize, v: C::T, mark: Bool) {
        // SAFETY: the caller's obligation: `at` is a record of this column
        // and the plan gave the unit its exclusive writer slot. The same write
        // `ResolveColumnWrite::resolve_write` makes.
        unsafe { core::ptr::write(self.head.write.as_ptr().add(at.0), v) }
        if mark.0 {
            self.head.mark.set();
        }
    }
}

// ---------------------------------------------------------------------
// The context, as the records of one window.
// ---------------------------------------------------------------------

impl<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP> Records<R, W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
where
    R: AccessSet + ColList,
    W: AccessSet + ColList,
    RCols: ColSlots<R>,
    WCols: ColSlots<W>,
    MP: MetaRank,
{
    #[inline(always)]
    fn len(&self) -> USize {
        // The mutant hands the body a window one record short, which the run
        // test must catch.
        #[cfg(feature = "mutant-short-window")]
        {
            USize(self.morsel.len.0.saturating_sub(1))
        }
        #[cfg(not(feature = "mutant-short-window"))]
        {
            self.morsel.len
        }
    }

    #[inline(always)]
    fn each<F: FnMut(USize)>(&self, mut f: F) {
        let len = <Self as Records<R, W>>::len(self);
        let mut i = 0usize;
        while i < len.0 {
            f(USize(i));
            i += 1;
        }
    }

    #[inline(always)]
    unsafe fn read<T: ColumnValue, K>(&self, i: USize) -> T
    where
        R: ReadsColumn<T, K>,
    {
        let at = USize(self.morsel.start.0 + i.0);
        // SAFETY: the caller's obligation on `i`, moved to the column's own
        // index by the window start.
        unsafe { <R as ReadsColumn<T, K>>::read_in(&self.read_cols, at) }
    }

    #[inline(always)]
    unsafe fn write<T: ColumnValue, K>(&self, i: USize, v: T)
    where
        W: WritesAt<T, K>,
    {
        let at = USize(self.morsel.start.0 + i.0);
        // SAFETY: the caller's obligation on `i`, moved to the column's own
        // index by the window start. A maintenance unit's context marks the
        // column, as `resolve_write` does.
        unsafe { <W as WritesAt<T, K>>::write_in(&self.write_cols, at, v, MP::MAINTAINS) }
    }
}

// ---------------------------------------------------------------------
// Registration.
// ---------------------------------------------------------------------

/// A block unit, as the scheduler builder takes it.
///
/// The wrapper is where the context type is named: `CtxFor` over the unit's
/// own lists. The unit names none.
pub struct Blk<U>(pub U);

impl<U: Block> BuilderInput for Blk<U> {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl<U> WorkUnit for Blk<U>
where
    U: Block,
    U::Read: ColList + ResourceBundleOf + ColBundleOf,
    U::Write: ColList + ColBundleOf + for<'f> AccumBundleOf<'f> + for<'f> VirtBundleOf<'f>,
    <U::Read as ColBundleOf>::Out: ColSlots<U::Read>,
    <U::Write as ColBundleOf>::Out: ColSlots<U::Write>,
{
    type Ctx<'frame> = CtxFor<'frame, U::Read, U::Write>;
    type Hint = U::Hint;
    type Read = U::Read;
    type Write = U::Write;

    #[inline(always)]
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        self.0.run(ctx);
    }
}
