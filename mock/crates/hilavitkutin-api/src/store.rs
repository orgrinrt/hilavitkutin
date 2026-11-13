//! Store marker types.
//!
//! Zero-sized type-level descriptors for the store shapes (`Resource`,
//! `Column`, `Buffered` with its two modes, `Accum`, `Virtual`) and the
//! three resource layouts (`Field`, `Seq`, `Map`). The scheduler sees `StoreId` pairs at
//! runtime; these types are the compile-time identity erased to
//! plain indices by execution time.

use core::marker::PhantomData;

use arvo::Cap;

use crate::builder_input::{BuilderInput, StoreDispatch};
use crate::column_value::ColumnValue;

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
    /// Register a `Resource<T>` carrying the given value.
    ///
    /// Returns a `StagedResource<T>` that owns `value` until the
    /// builder moves it into scheduler-owned storage at `build()`.
    /// The `.with(Resource::new(value))` spelling is unchanged; the
    /// value is retained, not dropped.
    ///
    /// Not `const fn`: the carrier owns a `T` that may have a
    /// destructor. Stateless markers (`Column<T>`, `Virtual<T>`) keep
    /// their const constructors.
    #[inline]
    pub fn new(value: T) -> StagedResource<T> {
        StagedResource(value)
    }
}

/// Value carrier for a registered `Resource<T>`.
///
/// `Resource::new(value)` returns this. The builder retains it on its
/// staged value list, and the arena drain moves the inner `T` into
/// scheduler-owned storage. The `Resource<T>` marker, not this
/// carrier, is what lands in the store access set, routed via
/// `StoreDispatch<Resource<T>>`.
#[repr(transparent)]
pub struct StagedResource<T>(T);

impl<T> StagedResource<T> {
    /// Take the owned value out of the carrier.
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: 'static> BuilderInput for StagedResource<T> {
    type Dispatch = StoreDispatch<Resource<T>>;
    type Init = T;
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
    ///
    /// The marker is what a unit names the store by. A store is registered
    /// with [`start`](Column::start) or [`start_from`](Column::start_from),
    /// which say what its records hold before any unit has written them.
    pub const fn new() -> Self {
        Column(PhantomData)
    }

    /// Register the store with `value` at every record.
    ///
    /// There for every element type, also one that has a start of its own.
    #[inline]
    pub const fn start_from(value: T) -> Started<Self> {
        Started(value)
    }
}

impl<T: notko::HasTrivialCtor> Column<T> {
    /// Register the store with the element type's own start at every record.
    ///
    /// The start is the type's trivial constructor where it has one, else
    /// its `Default` through [`StartsFromDefault`], and for a type that has
    /// both it is the constructor:
    ///
    /// ```
    /// use hilavitkutin_api::store::{Column, StartsFromDefault};
    ///
    /// #[derive(Copy, Clone, PartialEq, Debug)]
    /// struct Both(u8);
    /// impl notko::HasTrivialCtor for Both {
    ///     fn new() -> Self {
    ///         Both(7)
    ///     }
    /// }
    /// impl Default for Both {
    ///     fn default() -> Self {
    ///         Both(9)
    ///     }
    /// }
    ///
    /// #[derive(Copy, Clone, PartialEq, Debug, Default)]
    /// struct Zeroed(u8);
    ///
    /// assert_eq!(Column::<Both>::start().into_inner(), Both(7));
    /// assert_eq!(Column::<Zeroed>::start().into_inner(), Zeroed(0));
    /// ```
    ///
    /// An element type with neither has no `start`, and
    /// [`start_from`](Column::start_from) is the one way to register it:
    ///
    /// ```compile_fail,E0599
    /// use hilavitkutin_api::store::{Column, StartsFromDefault};
    ///
    /// #[derive(Copy, Clone)]
    /// struct Neither(u8);
    /// let _ = Column::<Neither>::start();
    /// ```
    #[inline]
    pub const fn start() -> Started<Self>
    where
        T: [const] notko::HasTrivialCtor,
    {
        Started(<T as notko::HasTrivialCtor>::new())
    }
}

impl<T> notko::HasTrivialCtor for Column<T> {
    fn new() -> Self {
        Column(PhantomData)
    }
}

/// Accumulator store: records appended during the frame, grown from zero.
///
/// Sibling of `Column<T>`. Where a `Column<T>` is sized to the build-time
/// record count and written by index, an `Accum<T>` reserves a capacity at
/// build and grows from zero as WorkUnits append. The live length advances
/// through interior mutability (a `Cell` in the binding), so the `&self`
/// append accessor never needs `&mut`. A later PassEnd drain reads the
/// populated prefix `[0, live_length)`.
#[repr(transparent)]
pub struct Accum<T>(PhantomData<T>);

impl<T> Copy for Accum<T> {}
impl<T> Clone for Accum<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Default for Accum<T> {
    #[inline(always)]
    fn default() -> Self {
        Accum(PhantomData)
    }
}

impl<T> Accum<T> {
    /// Construct an `Accum<T>` marker.
    pub const fn new() -> Self {
        Accum(PhantomData)
    }
}

impl<T: 'static> BuilderInput for Accum<T> {
    type Dispatch = StoreDispatch<Self>;
    type Init = ();
}

impl<T> notko::HasTrivialCtor for Accum<T> {
    fn new() -> Self {
        Accum(PhantomData)
    }
}

/// The live side of a buffered column is copied into its frozen side.
///
/// The default mode. The live side always holds the whole store, so a unit
/// may write any part of it.
pub struct Snapshot;

/// The two sides of a buffered column trade places, with no copy.
///
/// The side a unit then writes into holds the values of two passes before,
/// so the store is right only where a unit that writes it writes every
/// record, which a value function does.
pub struct Exchange;

/// How the live side of a buffered column becomes its frozen side.
///
/// `Snapshot` and `Exchange`, and no other.
#[allow(private_bounds)]
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a mode of a buffered column",
    note = "The modes are `Snapshot` (the live side is copied into the frozen side, the default) and `Exchange` (the two sides trade places)."
)]
pub trait BufferMode: crate::sealed::Sealed + 'static {}

impl crate::sealed::Sealed for Snapshot {}
impl BufferMode for Snapshot {}
impl crate::sealed::Sealed for Exchange {}
impl BufferMode for Exchange {}

/// A column held in two sides: read frozen, written live.
///
/// Every read sees the frozen side, which is the store as it stood when the
/// pass began, and every write lands on the live side. After the last unit
/// of a pass the live side of every buffered store the pass changed becomes
/// its frozen side. `M` says how, and is settled at compile time.
#[repr(transparent)]
pub struct Buffered<T, M = Snapshot>(PhantomData<(T, M)>);

impl<T, M> Copy for Buffered<T, M> {}
impl<T, M> Clone for Buffered<T, M> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T, M> Default for Buffered<T, M> {
    #[inline(always)]
    fn default() -> Self {
        Buffered(PhantomData)
    }
}

impl<T, M> Buffered<T, M> {
    /// Construct a `Buffered<T, M>` marker.
    ///
    /// The marker is what a unit names the store by. It is not what a
    /// scheduler is built with, since it says nothing of what the first pass
    /// reads, so it is no `BuilderInput`. A store is registered with
    /// [`start`](Buffered::start) or [`start_from`](Buffered::start_from).
    pub const fn new() -> Self {
        Buffered(PhantomData)
    }
}

impl<T, M: BufferMode> Buffered<T, M> {
    /// Register the store with `value` at every record of both sides.
    ///
    /// There for every element type, also one that has a start of its own.
    #[inline]
    pub const fn start_from(value: T) -> Started<Self> {
        Started(value)
    }
}

impl<T: notko::HasTrivialCtor, M: BufferMode> Buffered<T, M> {
    /// Register the store with the element type's own start at every record.
    ///
    /// The start is the type's trivial constructor where it has one, else
    /// its `Default` through [`StartsFromDefault`], and for a type that has
    /// both it is the constructor:
    ///
    /// ```
    /// use hilavitkutin_api::store::{Buffered, StartsFromDefault};
    ///
    /// #[derive(Copy, Clone, PartialEq, Debug)]
    /// struct Both(u8);
    /// impl notko::HasTrivialCtor for Both {
    ///     fn new() -> Self {
    ///         Both(7)
    ///     }
    /// }
    /// impl Default for Both {
    ///     fn default() -> Self {
    ///         Both(9)
    ///     }
    /// }
    ///
    /// #[derive(Copy, Clone, PartialEq, Debug, Default)]
    /// struct Plain(u8);
    ///
    /// assert_eq!(Buffered::<Both>::start().into_inner(), Both(7));
    /// assert_eq!(Buffered::<Plain>::start().into_inner(), Plain(0));
    /// ```
    ///
    /// An element type with neither has no `start`, and
    /// [`start_from`](Buffered::start_from) is the one way to register it:
    ///
    /// ```compile_fail,E0599
    /// use hilavitkutin_api::store::{Buffered, StartsFromDefault};
    ///
    /// #[derive(Copy, Clone)]
    /// struct Neither(u8);
    /// let _ = Buffered::<Neither>::start();
    /// ```
    #[inline]
    pub const fn start() -> Started<Self>
    where
        T: [const] notko::HasTrivialCtor,
    {
        Started(<T as notko::HasTrivialCtor>::new())
    }
}

/// A store that holds a value of its element type at every record.
///
/// `Column<T>` and `Buffered<T, M>`, and no other. Such a store is registered
/// with the value its records start from, so no read of it reaches a record
/// nothing wrote.
#[allow(private_bounds)]
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a store that is registered with a start",
    note = "The stores with a start are `Column<T>` and `Buffered<T, M>`."
)]
pub trait Starts: crate::sealed::Sealed + Sized {
    /// The element type, which is the type of the start.
    type Value;
}

impl<T> crate::sealed::Sealed for Column<T> {}
impl<T> Starts for Column<T> {
    type Value = T;
}

impl<T, M: BufferMode> crate::sealed::Sealed for Buffered<T, M> {}
impl<T, M: BufferMode> Starts for Buffered<T, M> {
    type Value = T;
}

/// The second way a column's element type gives it a start.
///
/// Implemented for every store that [`Starts`] whose element type has a
/// `Default`, and by nothing else. Its `start` is found only where the
/// inherent [`Column::start`] or [`Buffered::start`] does not apply, which is
/// where the element type has no trivial constructor, so the two make one
/// sequence. The choice is made where `start()` is written, from what is known
/// of the type there: a generic caller that knows only `T: Default` gets the
/// default.
pub trait StartsFromDefault: Starts {
    /// Register the store with the element type's default at every record.
    fn start() -> Started<Self>;
}

impl<S: Starts> StartsFromDefault for S
where
    S::Value: Default,
{
    #[inline]
    fn start() -> Started<S> {
        Started(<S::Value as Default>::default())
    }
}

/// A column, plain or buffered, with the value every record of it starts from.
///
/// `start` and `start_from` on [`Column`] and on [`Buffered`] return this.
/// The builder keeps it on its value list and writes the value to every
/// record of the store when the scheduler is built. The marker `S`, not this
/// carrier, is what lands in the store access set, routed via
/// `StoreDispatch<S>`.
pub struct Started<S: Starts>(S::Value);

impl<S: Starts> Started<S> {
    /// Take the start value out of the carrier.
    #[inline]
    pub fn into_inner(self) -> S::Value {
        self.0
    }
}

impl<S: Starts + 'static> BuilderInput for Started<S>
where
    S::Value: 'static,
{
    type Dispatch = StoreDispatch<S>;
    type Init = S::Value;
}

impl<T, M> notko::HasTrivialCtor for Buffered<T, M> {
    fn new() -> Self {
        Buffered(PhantomData)
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

impl<T: 'static> BuilderInput for Virtual<T> {
    type Dispatch = StoreDispatch<Self>;
    type Init = ();
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
// `StoreBundle` and `Replaceable`.
//
// StoreBundle marks Cons-list bundles of store markers (Resource /
// Column / Virtual / Field / Seq / Map / mixed). Used by Kit's
// 'type Owned' bound; the engine's add_kit reads K::Owned at compile
// time.
//
// Replaceable opts a store-typed value into Scheduler::replace_resource
// override semantics. Apps impl Replaceable per-type to opt their
// Resources into override semantics; non-opt-in types stay locked.
// Single substrate-enforced annotation on Owned types.
// ---------------------------------------------------------------------

use crate::access::{Cons, Empty};

/// Marker for a Cons-list of stores.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a StoreBundle",
    note = "StoreBundle is auto-implemented for `Empty` and `Cons<H, T>` where `T: StoreBundle`. Register each store on the scheduler builder with `.with(...)`, or install a Kit whose `Owned` declares it."
)]
pub trait StoreBundle {}

impl StoreBundle for Empty {}
impl<H, T: StoreBundle> StoreBundle for Cons<H, T> {}

/// Opt-in marker for store types whose Resource value can be
/// replaced via Scheduler::replace_value.
///
/// Mutually exclusive with [`PlanAffecting`](crate::run_cfg::PlanAffecting),
/// which is dominant: a plan-affecting value routes through
/// `Scheduler::replace_resource`, never `replace_value`, and the
/// negative impl below turns a double opt-in into E0751 at the
/// consumer's own impl site:
///
/// ```compile_fail,E0751
/// use hilavitkutin_api::store::Replaceable;
/// use hilavitkutin_api::run_cfg::PlanAffecting;
///
/// struct Both;
/// impl PlanAffecting for Both {}
/// impl Replaceable for Both {}
/// ```
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not opt-in for Scheduler::replace_value",
    note = "Mark the type with `impl Replaceable for {Self} {{}}` to opt in. Replaceable is intentionally explicit: stores omitted from the replacement set are stable for plan-time analysis."
)]
pub trait Replaceable {}

// PlanAffecting dominance: one type never carries both replacement
// semantics. Sound subset of OIBIT per the workspace unstable-features
// tables (negative_impls, WATCH tier); the exclusivity sketch
// (mock/research/pre_seed/sketches/202607193310_planaffecting-replaceable-exclusivity)
// confirmed both arms.
impl<T: crate::run_cfg::PlanAffecting> !Replaceable for T {}
