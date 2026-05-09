//! `Provider` — the unified registration contract for the
//! `SchedulerBuilder`.
//!
//! Every value passed to `SchedulerBuilder::with(value)` impls
//! `Provider`. The trait carries three associated items:
//!
//! - `Init` names the construction-time value type. For stateful
//!   wrappers (`Resource<T>`) it is the inner `T`. For the zero-data
//!   wrappers (`Column<T>`, `Virtual<T>`, `LinkedBin<dyn TraitFamily>`)
//!   it is `()`. For unit-struct WorkUnits, Kits, and platform-impl
//!   types it is `Self` (the registration value IS the construction
//!   value).
//! - `KIND: ProviderKind` is a documentation and debugging aid; the
//!   load-bearing dispatch is via `Provider::Dispatch`.
//! - `Dispatch: Dispatch` is the per-kind typestate router. The four
//!   shipped routers (`UnitDispatch`, `StoreDispatch`, `KitDispatch`,
//!   `PlatformDispatch`) compute the next typestate accumulator value
//!   when a provider of the corresponding kind registers.
//!
//! `Dispatch` declares three GATs (`NextWus<Wus>`, `NextStores<Stores>`,
//! `NextPlatform<Platform>`). Each router impls all three; routers
//! that do not affect a given accumulator pass it through unchanged.
//!
//! `LinkedBin<T: ?Sized>` is the type-system anchor for an
//! extension-loaded provider bin keyed by trait family. The runtime
//! instance population happens in `hilavitkutin-extensions` plus the
//! engine; this wrapper is the contract surface only.

use core::marker::PhantomData;

use crate::access::Cons;

/// The unified registration contract.
///
/// Every type accepted by `SchedulerBuilder::with(value)` impls
/// `Provider`. The blanket on `SchedulerBuilder` reads
/// `Provider::Dispatch` and walks the per-kind GATs to compute the
/// next typestate accumulator. Non-`Provider` values fail the trait
/// solver at the `.with` call site, surfacing the
/// `#[diagnostic::on_unimplemented]` message.
///
/// `Init` is the construction-time value type. The associated type
/// is informational at this layer; the runtime data plane consumes
/// it when wiring scheduler-owned storage. Until the data plane
/// lands (HILA-RUNTIME-C6), the value passed at construction is
/// stub-time only.
///
/// `KIND` is a documentation aid and lets diagnostics print
/// "you passed a Column provider where the bound expected a Memory
/// provider". It does not participate in dispatch; `Dispatch` does.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a Provider; pass a registered provider value to `SchedulerBuilder::with(...)`",
    label = "not a Provider",
    note = "Use `Resource::new(value)` for singleton state, `Column::<T>::new()` / `Virtual::<T>::new()` / `LinkedBin::<dyn TraitFamily>::new()` for type-keyed declarations, or impl `Provider` on your unit-struct WorkUnits / Kits / platform-impl types alongside the matching domain trait (`WorkUnit`, `Kit`, `MemoryProviderApi`, `ThreadPoolApi`, `ClockApi`)."
)]
pub trait Provider: Sized {
    /// Construction-time value type.
    ///
    /// Stateful wrappers set this to the inner `T`. Zero-data wrappers
    /// set it to `()`. Unit-struct shapes set it to `Self`.
    type Init;

    /// Documentation discriminator. Not load-bearing for dispatch.
    const KIND: ProviderKind;

    /// Per-kind typestate routing.
    ///
    /// The associated type names the router struct
    /// (`UnitDispatch<Self>` / `StoreDispatch<Self>` /
    /// `KitDispatch<Self>` / `PlatformDispatch<Self>`). The router
    /// impls `Dispatch<Wus, Stores, Platform>` for every
    /// accumulator triple it can be invoked against; the
    /// `SchedulerBuilder::with` blanket reads the per-impl
    /// associated types to compute the next typestate.
    type Dispatch;
}

/// Per-kind typestate routing. One impl per kind; the implementing
/// router struct is selected by `Provider::Dispatch`.
///
/// Three associated types, one per accumulator the
/// `SchedulerBuilder` typestate carries:
///
/// - `NextWus` — the next value of the WorkUnit accumulator after
///   registering this provider.
/// - `NextStores` — the next value of the store accumulator.
/// - `NextPlatform` — the next value of the platform-tuple
///   accumulator. The current `SchedulerBuilder` is two-parameter
///   (`SchedulerBuilder<Wus, Stores>`); the `NextPlatform` slot is
///   forward-compatible with a future `SchedulerBuilder<Wus, Stores,
///   Platform>` reshape (likely with HILA-RUNTIME-C4) and is
///   computed at every router impl regardless.
///
/// The trait is parameterised by the three accumulator types
/// (`Wus`, `Stores`, `Platform`) instead of using GATs because router
/// impls (notably `KitDispatch`) need where-clauses on the
/// accumulators that GAT-level where-clauses cannot express stricter
/// than the trait declaration. The trait-level parameter shape lets
/// each impl declare its own bounds at impl time.
///
/// Routers that do not affect a given accumulator pass it through
/// unchanged (identity).
pub trait Dispatch<Wus, Stores, Platform> {
    /// Next value of the WorkUnit accumulator.
    type NextWus;

    /// Next value of the store accumulator.
    type NextStores;

    /// Next value of the platform-tuple accumulator.
    type NextPlatform;
}

/// Router for WorkUnit-kind providers. Prepends `W` onto the WU
/// accumulator; passes stores and platform through.
pub struct UnitDispatch<W>(PhantomData<W>);

impl<W, Wus, Stores, Platform> Dispatch<Wus, Stores, Platform> for UnitDispatch<W> {
    type NextWus = Cons<W, Wus>;
    type NextStores = Stores;
    type NextPlatform = Platform;
}

/// Router for store-kind providers. Prepends `S` onto the store
/// accumulator; passes WUs and platform through.
pub struct StoreDispatch<S>(PhantomData<S>);

impl<S, Wus, Stores, Platform> Dispatch<Wus, Stores, Platform> for StoreDispatch<S> {
    type NextWus = Wus;
    type NextStores = Cons<S, Stores>;
    type NextPlatform = Platform;
}

// `KitDispatch<K>` lives in `hilavitkutin-kit` next to the `Kit`
// trait it routes for. The api crate does not depend on the kit
// crate (api is the contract; kit is a thin layer above it). See
// `hilavitkutin_kit::KitDispatch` for the router struct and its
// `Dispatch` impl.

/// Router for platform-kind providers. Prepends `P` onto the
/// platform-tuple accumulator; passes WUs and stores through.
///
/// The current `SchedulerBuilder<Wus, Stores>` is two-parameter so
/// the `NextPlatform` slot is unused at the call site. Forward-
/// compatible with a future three-parameter builder reshape.
pub struct PlatformDispatch<P>(PhantomData<P>);

impl<P, Wus, Stores, Platform> Dispatch<Wus, Stores, Platform> for PlatformDispatch<P> {
    type NextWus = Wus;
    type NextStores = Stores;
    type NextPlatform = Cons<P, Platform>;
}

/// Documentation-aid kind discriminator. Not load-bearing for
/// dispatch; `Provider::Dispatch` is.
///
/// `Provider::KIND` lets diagnostics print "you passed a Column
/// provider where the bound expected a Memory provider" without
/// reflecting on the type.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ProviderKind {
    /// Sequential WorkUnit; routed via `UnitDispatch`.
    WorkUnit,
    /// Singleton store; routed via `StoreDispatch`.
    Resource,
    /// Collection store; routed via `StoreDispatch`.
    Column,
    /// Zero-data DAG-edge marker; routed via `StoreDispatch`.
    Virtual,
    /// Extension-loaded trait-family bin; routed via `StoreDispatch`.
    LinkedBin,
    /// Declarative bundle of WUs and stores; routed via `KitDispatch`.
    Kit,
    /// Platform memory provider; routed via `PlatformDispatch`.
    Memory,
    /// Platform thread pool; routed via `PlatformDispatch`.
    Threads,
    /// Platform clock; routed via `PlatformDispatch`.
    Clock,
}

/// Type-system anchor for an extension-loaded provider bin keyed by
/// trait family.
///
/// `LinkedBin<dyn TraitFamily>` is a zero-data marker that registers
/// "the scheduler will own a bin of `TraitFamily` impls loaded by
/// the extension layer". The runtime instance population happens in
/// `hilavitkutin-extensions` plus the engine; the wrapper here is
/// the type-system contract.
///
/// Implements `Provider<Init = ()>` with `Dispatch = StoreDispatch<Self>`.
/// Construct via `LinkedBin::<dyn TraitFamily>::new()`.
#[repr(transparent)]
pub struct LinkedBin<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized> LinkedBin<T> {
    /// Construct a `LinkedBin` marker.
    pub const fn new() -> Self {
        LinkedBin(PhantomData)
    }
}

impl<T: ?Sized + 'static> Provider for LinkedBin<T> {
    type Init = ();
    const KIND: ProviderKind = ProviderKind::LinkedBin;
    type Dispatch = StoreDispatch<Self>;
}

impl<T: ?Sized + 'static> notko::HasTrivialCtor for LinkedBin<T> {
    fn new() -> Self {
        LinkedBin(PhantomData)
    }
}
