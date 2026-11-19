//! `EngineCtx` writer side: the column writer bridge, the accumulator
//! append bridge, and the virtual-firer bridge.
//!
//! Reads `self.write_cols`, `self.write_accums`, `self.write_virtuals` and
//! `self.epoch`, all private `EngineCtx` fields: this module is a
//! descendant of `dispatch::engine_ctx`, where the struct is defined, so it
//! reaches them without any visibility widening.

use arvo::USize;
use hilavitkutin_api::access::{AccessSet, Holds};
use hilavitkutin_api::column_value::ColumnValue;
use hilavitkutin_api::context::{
    AccumWriterApi,
    ColumnWriterApi,
    ResolveAccumAppend,
    ResolveColumnWrite,
    ResolveVirtualFire,
    VirtualFirerApi,
};
use hilavitkutin_api::store::Virtual;

use super::col_select::ColSelector;
use super::selectors::{AccumSelector, VirtualFire};
use super::{AccumColPtr, EngineCtx, MetaRank};

// ColumnWriterApi: `write` is the api's own, and follows the member the unit
// listed the column under. The context supplies the two bridges below it.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    ColumnWriterApi<W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
}

// ResolveColumnWrite: resolve the plain column of `T` through the projected
// column bundle's `ColSelector<T, I>` witness, and store or read at the
// morsel offset. The selector's kind is `Plain`, so a buffered column has no
// bridge here and is written through the value function alone.
impl<
    'frame,
    R: AccessSet,
    W: AccessSet,
    RBundle,
    RCols,
    WCols,
    WAccum,
    WVirt,
    MP: MetaRank,
    T: ColumnValue,
    I,
> ResolveColumnWrite<T, I> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
where
    WCols: ColSelector<T, I>,
{
    #[inline]
    unsafe fn resolve_write(&self, i: USize, v: T) {
        let col = <WCols as ColSelector<T, I>>::get(&self.write_cols);
        let idx = USize(self.morsel.start.0 + i.0);
        // B3 treats the column buffer as `[T]`-shaped at stride
        // `size_of::<T>()`; sub-byte bitpacking (using `T::BIT_WIDTH`)
        // is a later round.
        // SAFETY: the column bundle holds the column at the witnessed index
        // `I` only because the unit's write list placed it there. The
        // engine's plan-time DAG analysis proves this WU holds the exclusive
        // writer slot for `T` at `idx`; no concurrent reader or writer
        // aliases it. `&self` (not `&mut self`) keeps LLVM from reordering
        // the write across fused WUs. Valid for `'frame`.
        unsafe { core::ptr::write(col.write.as_ptr().add(idx.0), v) }
        // A maintenance unit's write is a change its readers are seeded
        // from. An ordinary unit's readers follow it by the plan's edges,
        // and its context compiles no mark.
        if MP::MAINTAINS.0 {
            col.mark.set();
        }
    }

    #[inline]
    unsafe fn resolve_held(&self, i: USize) -> T {
        let col = <WCols as ColSelector<T, I>>::get(&self.write_cols);
        let idx = USize(self.morsel.start.0 + i.0);
        // SAFETY: as `resolve_write`; the slot is this unit's to write, so
        // nothing else writes it while it is read.
        unsafe { core::ptr::read(col.read.as_ptr().add(idx.0)) }
    }
}

// AccumWriterApi: `append` and `len` are the api's own, over the bridge
// below. The append is a self-relative grow, not a morsel-indexed write: the
// offset is the accumulator's own live count, not `morsel.start + i`.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP> AccumWriterApi<W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
}

impl<'frame, T: ColumnValue> AccumColPtr<'frame, T> {
    /// Append `v`, or count the append as refused.
    ///
    /// Every append is counted as asked. It is written only while nothing has
    /// been refused this frame (`wanted == live`) and the live length is below
    /// the reserved capacity; otherwise only `wanted` moves, so the refusal is
    /// sticky on this binding and the records it accepted are a prefix of the
    /// appends asked through it. Under `run_parallel` each core appends
    /// through its own binding, so that holds per region, not for the
    /// accumulator as a whole. Invariant: `wanted >= live`, and the binding
    /// has refused exactly when `wanted > live`. Both cells are written only
    /// by the thread that owns this binding for the frame. The run reads the
    /// refusal at the close (`FirstRefusal`).
    ///
    /// This branch is the one place the refusal is decided. Keeping an assert
    /// instead is a strategy read here, nothing else moves.
    #[inline(always)]
    pub(super) fn append(&self, v: T) {
        let live = self.len.get();
        let wanted = self.wanted.get();
        if wanted.0 == live.0 && live.0 < self.cap.0 {
            // B3 treats the capacity buffer as `[T]`-shaped at stride
            // `size_of::<T>()`; sub-byte bitpacking is a later round.
            // SAFETY: the handle was projected for a unit whose write list
            // holds the accumulator. The engine's plan-time DAG analysis
            // proves this WU holds the exclusive appender slot for `T`; no
            // concurrent appender aliases it. `live < cap` keeps the write
            // strictly within the reserved record count, so it lands in the
            // buffer. Valid for `'frame`.
            unsafe { core::ptr::write(self.base.as_ptr().add(live.0), v) };
            self.len.set(USize(live.0 + 1)); // lint:allow(no-bare-numeric) reason: live-length step; tracked: #345
            self.wanted.set(USize(wanted.0 + 1)); // lint:allow(no-bare-numeric) reason: wanted step, equal to live here; tracked: #345
        } else {
            // The truncating mutant drops the refused append without counting
            // it, which is the defect the overflow suite must catch.
            #[cfg(not(feature = "mutant-truncating-append"))]
            self.wanted.set(USize(wanted.0.saturating_add(1))); // lint:allow(no-bare-numeric) reason: wanted step on a refusal, saturating so the count never wraps below live; tracked: #345
        }
    }
}

// ResolveAccumAppend: resolve the `AccumColPtr<T>` through the projected
// accumulator bundle's `AccumSelector<T, I>` witness and append through it.
//
// SAFETY: `resolve_append` writes `v` at the live length only while that
// length is below the capacity the accumulator was reserved with, and
// advances the length by one when it does. Past the capacity it writes
// nothing and counts the refusal. That is the whole of what the trait asks.
unsafe impl<
    'frame,
    R: AccessSet,
    W: AccessSet,
    RBundle,
    RCols,
    WCols,
    WAccum,
    WVirt,
    MP,
    T: ColumnValue,
    I,
> ResolveAccumAppend<T, I> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
where
    WAccum: AccumSelector<T, I>,
{
    #[inline]
    fn resolve_append(&self, v: T) {
        <WAccum as AccumSelector<T, I>>::get(&self.write_accums).append(v);
    }

    #[inline]
    fn resolve_len(&self) -> USize {
        let acc = <WAccum as AccumSelector<T, I>>::get(&self.write_accums);
        acc.len.get()
    }
}

// VirtualFirerApi: stamp the projected `Virtual<V>` cell with the current epoch.
// The `On<V>` consumer's trunk-gate reads the same cell from the bindings and
// runs when `stamp == epoch`. Internal fire is non-atomic (spec :716-717).

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    VirtualFirerApi<W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    #[inline]
    fn fire<V: 'static, I>(&self)
    where
        W: Holds<Virtual<V>>,
        Self: ResolveVirtualFire<V, I>,
    {
        <Self as ResolveVirtualFire<V, I>>::resolve_fire(self);
    }
}

// ResolveVirtualFire: resolve the `Virtual<V>` stamp cell through the projected
// write-virtual bundle's `VirtualFire<V, I>` witness and set it to the live
// epoch. Mirrors `ResolveAccumAppend` over the accumulator bundle.
impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP, V: 'static, I>
    ResolveVirtualFire<V, I> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
where
    WVirt: VirtualFire<V, I>,
{
    #[inline]
    fn resolve_fire(&self) {
        <WVirt as VirtualFire<V, I>>::fire(&self.write_virtuals, self.epoch);
    }
}
