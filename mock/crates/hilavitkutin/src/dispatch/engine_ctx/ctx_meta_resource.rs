//! `EngineCtx` reader side: the E4 slice-3 meta accessor, the resource
//! provider bridge, and the column reader bridge.
//!
//! Split out of `dispatch/engine_ctx.rs` (file-size lint). No behaviour
//! change. Reads `self.meta_ptr`, `self.reads`, `self.read_cols`, and
//! `self.morsel`, all still-private `EngineCtx` fields: this module is a
//! descendant of `dispatch::engine_ctx`, where the struct is defined, so
//! it reaches them without any visibility widening.

use arvo::USize;
use hilavitkutin_api::access::{AccessSet, Contains};
use hilavitkutin_api::column_value::ColumnValue;
use hilavitkutin_api::context::{
    ColumnReaderApi,
    ResolveColumnRead,
    ResolveResource,
    ResourceProviderApi,
};
use hilavitkutin_api::meta::MetaAccess;
use hilavitkutin_api::store::{Column, Resource};

use super::selectors::{ColSelector, SnapSelector};
use super::{EngineCtx, MetaRef};
use crate::meta::MetaField;

// E4 slice 3: the meta accessor, present ONLY on a Ctx carrying a `MetaRef`
// (an `OnMeta` work unit's Ctx). A consumer Ctx (`MP = MetaNil`) has no `meta`
// method, so a consumer cannot reach meta state at compile time. The
// `MetaAccess` enforcement falls out of the gating for free: no negative bound,
// no specialization. Proven by sketch `202606090300`.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt>
    EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MetaRef<'frame>>
{
    /// Read an engine-owned meta resource through the bridge.
    ///
    /// `T` is a meta resource (`MetaAccess`) with a `MetaField` projection out of
    /// the engine-owned `MetaBlock`. Available only on an `OnMeta` work unit's
    /// Ctx; a consumer Ctx does not have this method (compile-time `MetaAccess`
    /// enforcement).
    ///
    /// ```compile_fail
    /// use hilavitkutin::dispatch::engine_ctx::{EngineCtx, SnapNil, ColPtrNil, MetaNil};
    /// use hilavitkutin_api::access::Empty;
    /// use hilavitkutin_api::meta::SchedulerMetrics;
    ///
    /// // A consumer Ctx (the default `MetaNil` meta pointer) has no `meta`
    /// // accessor: the impl is only on a Ctx carrying `MetaRef`. So a consumer
    /// // cannot reach meta state. This does not compile.
    /// fn consumer_reaches_meta(
    ///     ctx: &EngineCtx<'_, Empty, Empty, SnapNil, ColPtrNil, ColPtrNil>,
    /// ) {
    ///     let _ = ctx.meta::<SchedulerMetrics>();
    /// }
    /// ```
    #[inline]
    pub fn meta<T: MetaAccess + MetaField>(&self) -> &T {
        T::project(self.meta_ptr.0)
    }
}

// ResourceProviderApi: resolve `&T` via the resource bundle Selector.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    ResourceProviderApi<R> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    #[inline]
    fn resource<T: 'static, I>(&self) -> &T
    where
        R: Contains<Resource<T>>,
        Self: ResolveResource<T, I>,
    {
        <Self as ResolveResource<T, I>>::resolve_resource(self)
    }
}

// ResolveResource: borrow the snapshot value through the bundle's
// `SnapSelector<T, I>` witness. `I` is the per-`T` bundle index, inferred
// at the concrete WU call site (the bundle is a concrete cons-list there,
// so exactly one index applies). No unsafe: the bundle holds the value
// itself (the projection-time snapshot), and the borrow ties to `&self`.
impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP, T: 'static, I>
    ResolveResource<T, I> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
where
    RBundle: SnapSelector<T, I>,
{
    #[inline]
    fn resolve_resource(&self) -> &T {
        <RBundle as SnapSelector<T, I>>::get(&self.reads)
    }
}

// ColumnReaderApi: resolve the column pointer, read at the morsel
// offset. B3 treats the column buffer as `[T]`-shaped at stride
// `size_of::<T>()`; sub-byte bitpacking is a later round.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    ColumnReaderApi<R> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    #[inline]
    unsafe fn read<T: ColumnValue, I>(&self, i: USize) -> T
    where
        R: Contains<Column<T>>,
        Self: ResolveColumnRead<T, I>,
    {
        // SAFETY: forwarded to the bridge; the caller's obligation (the
        // engine proved slot ownership at plan time) carries through.
        unsafe { <Self as ResolveColumnRead<T, I>>::resolve_read(self, i) }
    }
}

// ResolveColumnRead: resolve the `ColumnPtr<T>` through the projected
// column bundle's `ColSelector<T, I>` witness, read at the morsel offset.
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
> ResolveColumnRead<T, I> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
where
    RCols: ColSelector<T, I>,
{
    #[inline]
    unsafe fn resolve_read(&self, i: USize) -> T {
        let ptr = <RCols as ColSelector<T, I>>::get(&self.read_cols);
        let idx = USize(self.morsel.start.0 + i.0);
        // B3 treats the column buffer as `[T]`-shaped at stride
        // `size_of::<T>()`; sub-byte bitpacking (using `T::BIT_WIDTH`)
        // is a later round.
        // SAFETY: the column bundle holds a `ColumnPtr<T>` at the
        // witnessed index `I` only because `R: Contains<Column<T>>`
        // placed it there. The caller (the engine, via plan-time DAG
        // analysis) guarantees the slot at `idx` is initialised and the
        // buffer is at least `start + len` records long. Valid for
        // `'frame`.
        unsafe { core::ptr::read(ptr.as_ptr().add(idx.0)) }
    }
}
