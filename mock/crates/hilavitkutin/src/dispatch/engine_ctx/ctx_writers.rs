//! `EngineCtx` writer side: the column writer bridge, the accumulator
//! append bridge, and the virtual-firer bridge.
//!
//! Split out of `dispatch/engine_ctx.rs` (file-size lint). No behaviour
//! change. Reads `self.write_cols`, `self.write_accums` and
//! `self.write_virtuals`, `self.epoch`, all still-private `EngineCtx`
//! fields: this module is a descendant of `dispatch::engine_ctx`, where
//! the struct is defined, so it reaches them without any visibility
//! widening.

use arvo::USize;
use hilavitkutin_api::access::{AccessSet, Contains};
use hilavitkutin_api::column_value::ColumnValue;
use hilavitkutin_api::context::{
    AccumWriterApi,
    ColumnWriterApi,
    ResolveAccumAppend,
    ResolveColumnWrite,
    ResolveVirtualFire,
    VirtualFirerApi,
};
use hilavitkutin_api::store::{Accum, Column, Virtual};

use super::EngineCtx;
use super::selectors::{AccumSelector, ColSelector, VirtualFire};

// ColumnWriterApi: resolve the column pointer, write at the morsel
// offset. Same stride simplification as the reader.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    ColumnWriterApi<W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    #[inline]
    unsafe fn write<T: ColumnValue, I>(&self, i: USize, v: T)
    where
        W: Contains<Column<T>>,
        Self: ResolveColumnWrite<T, I>,
    {
        // SAFETY: forwarded to the bridge; the caller's obligation (the
        // engine proved exclusive-writer ownership at plan time) carries
        // through.
        unsafe { <Self as ResolveColumnWrite<T, I>>::resolve_write(self, i, v) }
    }
}

// ResolveColumnWrite: resolve the `ColumnPtr<T>` through the projected
// column bundle's `ColSelector<T, I>` witness, write at the morsel offset.
impl<
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
> ResolveColumnWrite<T, I> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
where
    WCols: ColSelector<T, I>,
{
    #[inline]
    unsafe fn resolve_write(&self, i: USize, v: T) {
        let ptr = <WCols as ColSelector<T, I>>::get(&self.write_cols);
        let idx = USize(self.morsel.start.0 + i.0);
        // B3 treats the column buffer as `[T]`-shaped at stride
        // `size_of::<T>()`; sub-byte bitpacking (using `T::BIT_WIDTH`)
        // is a later round.
        // SAFETY: the column bundle holds a `ColumnPtr<T>` at the
        // witnessed index `I` only because `W: Contains<Column<T>>`
        // placed it there. The engine's plan-time DAG analysis proves
        // this WU holds the exclusive writer slot for `T` at `idx`; no
        // concurrent reader or writer aliases it. `&self` (not
        // `&mut self`) keeps LLVM from reordering the write across fused
        // WUs. Valid for `'frame`.
        unsafe { core::ptr::write(ptr.as_ptr().add(idx.0), v) }
    }
}

// AccumWriterApi: resolve the accumulator handle, append at the live offset,
// advance the live length. The append is a self-relative grow, not a
// morsel-indexed write: the offset is the accumulator's own live count, not
// `morsel.start + i`.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP> AccumWriterApi<W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    #[inline]
    unsafe fn append<T: ColumnValue, I>(&self, v: T)
    where
        W: Contains<Accum<T>>,
        Self: ResolveAccumAppend<T, I>,
    {
        // SAFETY: forwarded to the bridge; the caller's obligation (the engine
        // proved exclusive-appender ownership at plan time, and the live length
        // is within the reserved capacity) carries through.
        unsafe { <Self as ResolveAccumAppend<T, I>>::resolve_append(self, v) }
    }

    #[inline]
    fn len<T: ColumnValue, I>(&self) -> USize
    where
        W: Contains<Accum<T>>,
        Self: ResolveAccumAppend<T, I>,
    {
        <Self as ResolveAccumAppend<T, I>>::resolve_len(self)
    }
}

// ResolveAccumAppend: resolve the `AccumColPtr<T>` through the projected
// accumulator bundle's `AccumSelector<T, I>` witness, write at the live offset,
// advance the live length through the borrowed `Cell`.
impl<
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
    unsafe fn resolve_append(&self, v: T) {
        let acc = <WAccum as AccumSelector<T, I>>::get(&self.write_accums);
        let live = acc.len.get();
        let wanted = acc.wanted.get();
        // Every append is counted as asked. It is written only while nothing
        // has been refused this frame (`wanted == live`) and the live length is
        // below the reserved capacity; otherwise only `wanted` moves, so the
        // refusal is sticky and the accepted records are the prefix the frame
        // asked for first. Invariant: `wanted >= live`, and the accumulator has
        // refused exactly when `wanted > live`. Both cells are written only by
        // the thread that owns this binding for the frame. The run reads the
        // refusal at the close (`FirstRefusal`).
        //
        // This branch is the one place the refusal is decided. Keeping an
        // assert instead is a strategy read here, nothing else moves.
        if wanted.0 == live.0 && live.0 < acc.cap.0 {
            // B3 treats the capacity buffer as `[T]`-shaped at stride
            // `size_of::<T>()`; sub-byte bitpacking is a later round.
            // SAFETY: the accumulator bundle holds an `AccumColPtr<T>` at the
            // witnessed index `I` only because `W: Contains<Accum<T>>` placed it
            // there. The engine's plan-time DAG analysis proves this WU holds
            // the exclusive appender slot for `T`; no concurrent appender
            // aliases it. `live < cap` keeps the write strictly within the
            // reserved record count, so it lands in the buffer. `&self` (not
            // `&mut self`) keeps LLVM from reordering the write across fused
            // WUs. Valid for `'frame`.
            unsafe { core::ptr::write(acc.base.as_ptr().add(live.0), v) };
            acc.len.set(USize(live.0 + 1)); // lint:allow(no-bare-numeric) reason: live-length step; tracked: #345
            acc.wanted.set(USize(wanted.0 + 1)); // lint:allow(no-bare-numeric) reason: wanted step, equal to live here; tracked: #345
        } else {
            // The truncating mutant drops the refused append without counting
            // it, which is the defect the overflow suite must catch.
            #[cfg(not(feature = "mutant-truncating-append"))]
            acc.wanted.set(USize(wanted.0.saturating_add(1))); // lint:allow(no-bare-numeric) reason: wanted step on a refusal, saturating so the count never wraps below live; tracked: #345
        }
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
        W: Contains<Virtual<V>>,
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
