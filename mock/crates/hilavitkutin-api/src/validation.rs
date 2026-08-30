//! Compile-time access-set validation: Demand and Supply type-lists
//! plus the `SatisfiedBy` proof obligation that gates `.build()`.
//!
//! The scheduler builder accumulates two type-lists as the consumer
//! chains methods: `Demand` (what registered WorkUnits require) and
//! `Supply` (what providers, Resources, Columns, and Kit installations
//! have registered). At `.build()` time the bound `D: SatisfiedBy<S>`
//! resolves; if any required Demand leaf has no matching Supply leaf,
//! resolution fails and the consumer sees a trait-bound error at the
//! call site.
//!
//! v0 ships the type-list shape and a permissive `SatisfiedBy` impl.
//! The actual per-pair membership proof (Demand leaf to Supply leaf)
//! is iteration material tracked in the engine BACKLOG. Until that
//! lands, `.build()` accepts every (Demand, Supply) combination; the
//! contract surface and the chainable API are the load-bearing v0
//! deliverables.

use core::marker::PhantomData;

use crate::sealed::Sealed;

/// Marker trait for type-list nodes carrying builder demands.
pub trait Demand: Sealed {}

/// Marker trait for type-list nodes carrying builder supplies.
pub trait Supply: Sealed {}

/// Empty type-list. Implements both `Demand` and `Supply`.
pub struct Nil;
impl Sealed for Nil {}
impl Demand for Nil {}
impl Supply for Nil {}

/// Cons cell joining a head node to a tail list.
pub struct Cons<H, T>(PhantomData<(H, T)>);
impl<H, T> Sealed for Cons<H, T> {}
impl<H: DemandLeaf, T: Demand> Demand for Cons<H, T> {}
impl<H: SupplyLeaf, T: Supply> Supply for Cons<H, T> {}

/// Sealed marker for a Demand list head. Each demand kind impls this.
pub trait DemandLeaf: Sealed {}

/// Sealed marker for a Supply list head. Each supply kind impls this.
pub trait SupplyLeaf: Sealed {}

// --- Demand leaves ----------------------------------------------------

/// Demand: the registered WorkUnit reads access-set `R`.
pub struct DemandRead<R>(PhantomData<R>);
impl<R> Sealed for DemandRead<R> {}
impl<R> DemandLeaf for DemandRead<R> {}

/// Demand: the registered WorkUnit writes access-set `W`.
pub struct DemandWrite<W>(PhantomData<W>);
impl<W> Sealed for DemandWrite<W> {}
impl<W> DemandLeaf for DemandWrite<W> {}

/// Demand: a registered WorkUnit's `Ctx` requires `HasMemoryProvider`.
pub struct DemandHasMemoryProvider;
impl Sealed for DemandHasMemoryProvider {}
impl DemandLeaf for DemandHasMemoryProvider {}

/// Demand: a registered WorkUnit's `Ctx` requires `HasThreadPool`.
pub struct DemandHasThreadPool;
impl Sealed for DemandHasThreadPool {}
impl DemandLeaf for DemandHasThreadPool {}

/// Demand: a registered WorkUnit's `Ctx` requires `HasClock`.
pub struct DemandHasClock;
impl Sealed for DemandHasClock {}
impl DemandLeaf for DemandHasClock {}

// --- Supply leaves ----------------------------------------------------

/// Supply: a `Resource<T>` slot has been registered on the builder.
pub struct SupplyResource<T>(PhantomData<T>);
impl<T> Sealed for SupplyResource<T> {}
impl<T> SupplyLeaf for SupplyResource<T> {}

/// Supply: a `Column<T>` slot has been registered on the builder.
pub struct SupplyColumn<T>(PhantomData<T>);
impl<T> Sealed for SupplyColumn<T> {}
impl<T> SupplyLeaf for SupplyColumn<T> {}

/// Supply: a `MemoryProvider` has been bound on the builder.
pub struct SupplyMemory<M>(PhantomData<M>);
impl<M> Sealed for SupplyMemory<M> {}
impl<M> SupplyLeaf for SupplyMemory<M> {}

/// Supply: a `ThreadPool` has been bound on the builder.
pub struct SupplyThreads<P>(PhantomData<P>);
impl<P> Sealed for SupplyThreads<P> {}
impl<P> SupplyLeaf for SupplyThreads<P> {}

/// Supply: a `Clock` has been bound on the builder.
pub struct SupplyClock<C>(PhantomData<C>);
impl<C> Sealed for SupplyClock<C> {}
impl<C> SupplyLeaf for SupplyClock<C> {}

// --- Proof obligations ------------------------------------------------

/// Resolved at `.build()`: every Demand leaf in `Self` has a matching
/// Supply leaf in `S`. Sealed; consumers cannot widen the proof.
///
/// v0 is permissive: every Demand is satisfied by every Supply. The
/// per-pair membership proof rides a follow-up iteration; until then
/// the trait shape exists for downstream code to depend on.
pub trait SatisfiedBy<S: Supply>: Sealed {}

impl<D: Demand, S: Supply> SatisfiedBy<S> for D {}
