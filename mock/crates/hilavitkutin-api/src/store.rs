//! Store marker types.
//!
//! Zero-sized type-level descriptors for the three store shapes
//! (`Resource`, `Column`, `Virtual`) and the three resource layouts
//! (`Field`, `Seq`, `Map`). The scheduler sees `StoreId` pairs at
//! runtime; these types are the compile-time identity erased to
//! plain indices by execution time.

use core::marker::PhantomData;

use arvo::Cap;

use crate::column_value::ColumnValue;
use crate::provider::{Provider, ProviderKind, StoreDispatch};

/// Singleton store: one value shared across the pipeline.
#[repr(transparent)]
pub struct Resource<T>(PhantomData<T>);

impl<T> Copy for Resource<T> {}
impl<T> Clone for Resource<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Default for Resource<T> {
    #[inline(always)]
    fn default() -> Self {
        Resource(PhantomData)
    }
}

impl<T> Resource<T> {
    /// Construct a `Resource<T>` carrying the given value.
    ///
    /// The `_value: T` is consumed by ownership. At this round the
    /// scheduler data plane is not yet built (HILA-RUNTIME-C6 lands
    /// resource resolution + persistence wiring), so the value drops
    /// here. The semantic is preserved at the type level via
    /// `Provider::Init = T`: when the data plane lands, the
    /// constructor will route the value into the scheduler-owned
    /// resource registry.
    ///
    /// Not `const fn` because dropping a `T` with a destructor at
    /// compile-time is forbidden. Consumers that need a const-callable
    /// constructor for stateless markers use `Column<T>` or
    /// `Virtual<T>` (both impl `notko::HasTrivialCtor`).
    #[inline]
    pub fn new(_value: T) -> Self {
        Resource(PhantomData)
    }
}

impl<T: 'static> Provider for Resource<T> {
    type Init = T;
    const KIND: ProviderKind = ProviderKind::Resource;
    type Dispatch = StoreDispatch<Self>;
}

/// Collection store: N records per column, morsel-chunked.
#[repr(transparent)]
pub struct Column<T>(PhantomData<T>);

impl<T> Copy for Column<T> {}
impl<T> Clone for Column<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Default for Column<T> {
    #[inline(always)]
    fn default() -> Self {
        Column(PhantomData)
    }
}

impl<T> Column<T> {
    /// Construct a `Column<T>` marker.
    pub const fn new() -> Self {
        Column(PhantomData)
    }
}

impl<T: 'static> Provider for Column<T> {
    type Init = ();
    const KIND: ProviderKind = ProviderKind::Column;
    type Dispatch = StoreDispatch<Self>;
}

impl<T> notko::HasTrivialCtor for Column<T> {
    fn new() -> Self {
        Column(PhantomData)
    }
}

/// Zero-data store: DAG edge only. Used for fire flags.
#[repr(transparent)]
pub struct Virtual<T>(PhantomData<T>);

impl<T> Copy for Virtual<T> {}
impl<T> Clone for Virtual<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Default for Virtual<T> {
    #[inline(always)]
    fn default() -> Self {
        Virtual(PhantomData)
    }
}

impl<T> Virtual<T> {
    /// Construct a `Virtual<T>` marker.
    pub const fn new() -> Self {
        Virtual(PhantomData)
    }
}

impl<T: 'static> Provider for Virtual<T> {
    type Init = ();
    const KIND: ProviderKind = ProviderKind::Virtual;
    type Dispatch = StoreDispatch<Self>;
}

impl<T> notko::HasTrivialCtor for Virtual<T> {
    fn new() -> Self {
        Virtual(PhantomData)
    }
}

/// Scalar resource layout: ≤16-byte value, LLVM-promotable.
#[repr(transparent)]
pub struct Field<T: ColumnValue>(PhantomData<T>);

impl<T: ColumnValue> Copy for Field<T> {}
impl<T: ColumnValue> Clone for Field<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: ColumnValue> Default for Field<T> {
    #[inline(always)]
    fn default() -> Self {
        Field(PhantomData)
    }
}

/// Const-sized sequence resource layout. Arena-allocated.
#[repr(transparent)]
pub struct Seq<T, const N: Cap>(PhantomData<T>);

impl<T, const N: Cap> Copy for Seq<T, N> {}
impl<T, const N: Cap> Clone for Seq<T, N> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T, const N: Cap> Default for Seq<T, N> {
    #[inline(always)]
    fn default() -> Self {
        Seq(PhantomData)
    }
}

/// Const-sized key/value resource layout. Arena-allocated.
#[repr(transparent)]
pub struct Map<K, V, const N: Cap>(PhantomData<(K, V)>);

impl<K, V, const N: Cap> Copy for Map<K, V, N> {}
impl<K, V, const N: Cap> Clone for Map<K, V, N> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<K, V, const N: Cap> Default for Map<K, V, N> {
    #[inline(always)]
    fn default() -> Self {
        Map(PhantomData)
    }
}

// ---------------------------------------------------------------------
// Round 4 substrate: StoreBundle + Replaceable.
//
// StoreBundle marks Cons-list bundles of store markers (Resource /
// Column / Virtual / Field / Seq / Map / mixed). Used by Kit's
// 'type Owned' bound; the engine's add_kit reads K::Owned at compile
// time.
//
// Replaceable opts a store-typed value into Scheduler::replace_resource
// override semantics. Apps impl Replaceable per-type to opt their
// Resources into override semantics; non-opt-in types stay locked.
// Single substrate-enforced annotation on Owned types per
// topic_round_4_layered_enforcement.md.
// ---------------------------------------------------------------------

use crate::access::{Cons, Empty};

/// Marker for a Cons-list of stores.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a StoreBundle",
    note = "StoreBundle is auto-implemented for `Empty` and `Cons<H, T>` where `T: StoreBundle`. Build the bundle through the scheduler builder's `.add_resource::<T>(initial)`, `.add_column::<T>()`, and `.add_virtual::<T>()` calls, or install a Kit whose `Owned` declares it."
)]
pub trait StoreBundle {}

impl StoreBundle for Empty {}
impl<H, T: StoreBundle> StoreBundle for Cons<H, T> {}

/// Opt-in marker for store types whose Resource value can be
/// replaced via Scheduler::replace_resource.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not opt-in for Scheduler::replace_resource",
    note = "Mark the type with `impl Replaceable for {Self} {{}}` to opt in. Replaceable is intentionally explicit: stores omitted from the replacement set are stable for plan-time analysis."
)]
pub trait Replaceable {}

