//! `BuilderInput`: the unified registration contract for the
//! `SchedulerBuilder`.
//!
//! Every value passed to `SchedulerBuilder::with(value)` impls
//! `BuilderInput`. The trait carries two associated items:
//!
//! - `Init` names the construction-time value type. For stateful
//!   wrappers (the `Resource<T>` shape and bare types) it is `Self`.
//!   For zero-data markers
//!   (`ExtensionSurface<dyn TraitFamily>`) it is `()`. Each impl
//!   declares `Init` explicitly; the trait carries no default.
//! - `Dispatch` is the per-kind typestate router. The four shipped
//!   routers (`UnitDispatch`, `StoreDispatch`, `KitDispatch`,
//!   `PlatformDispatch`) compute the next typestate accumulator
//!   value when an input of the corresponding kind registers. A
//!   fifth router `RunCfgDispatch` (declared in `run_cfg.rs`) routes
//!   the consumer-supplied `RunCfg` into the run-config typestate
//!   slot.
//!
//! `Dispatch<Wus, Stores, Platform>` is parameterised by the three
//! accumulators and declares one associated type per accumulator
//! (`NextWus`, `NextStores`, `NextPlatform`). Each router impls all
//! three; routers that do not affect a given accumulator pass it
//! through unchanged.
//!
//! `ExtensionSurface<T: ?Sized>` is the type-system anchor for an
//! extension-loaded input bin keyed by trait family. The runtime
//! instance population happens in `hilavitkutin-extensions` plus the
//! engine; this wrapper is the contract surface only.
//!
//! Kind discrimination flows through trait identity (sub-traits
//! `Resource` / `Column` / `Virtual` / `WorkUnit` / `Kit` /
//! `MemoryProvider` / `ThreadPool` / `Clock` / `RunCfg` declare
//! their own `Dispatch` slot), not a const enum.

use core::marker::PhantomData;

use crate::access::{Concat, Cons, Empty};

/// The unified registration contract.
///
/// Every type accepted by `SchedulerBuilder::with(value)` impls
/// `BuilderInput`. The blanket on `SchedulerBuilder` reads
/// `BuilderInput::Dispatch` and reads its `Dispatch` associated types
/// to compute the next typestate accumulators. Non-`BuilderInput` values fail
/// the trait solver at the `.with` call site, surfacing the
/// `#[diagnostic::on_unimplemented]` message.
///
/// `Init` is the construction-time value type. Each impl declares it
/// explicitly: data-carrying inputs use `Self`, zero-data marker
/// types use `()`.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a BuilderInput; pass a registered input value to `SchedulerBuilder::with(...)`",
    label = "not a BuilderInput",
    note = "Impl one of the dispatch-determining sub-traits on `{Self}`: `Resource` / `Column` / `Virtual` / `WorkUnit` / `Kit` / `MemoryProvider` / `ThreadPool` / `Clock` / `RunCfg`. Pair the sub-trait impl with `impl BuilderInput for {Self} {{ type Dispatch = <the matching dispatch slot>; }}`. The `Dispatch` slot decides which accumulator the input joins, so it names the router of the one sub-trait the type impls.",
    note = "A column is registered with its start and never by its bare marker: `.with(Column::<T>::start())` or `.with(Column::<T>::start_from(value))`, and the same two on `Buffered<T, M>`. `Column::<T>::new()` names the column in a unit's lists and registers nothing."
)]
pub trait BuilderInput: Sized {
    /// Construction-time value type. Each impl declares it: `Self` for
    /// data-carrying inputs, `()` for zero-data markers.
    type Init;

    /// Per-kind typestate routing.
    ///
    /// The associated type names the router struct
    /// (`UnitDispatch<Self>` / `StoreDispatch<Self>` /
    /// `KitDispatch<Self>` / `PlatformDispatch<Self>` /
    /// `RunCfgDispatch<Self>`). The router impls
    /// `Dispatch<Wus, Stores, Platform>` for every accumulator
    /// triple it can be invoked against; the
    /// `SchedulerBuilder::with` blanket reads the per-impl
    /// associated types to compute the next typestate.
    type Dispatch;
}

/// Per-kind typestate routing. One impl per kind; the implementing
/// router struct is selected by `BuilderInput::Dispatch`.
///
/// Three associated types, one per accumulator the
/// `SchedulerBuilder` typestate carries:
///
/// - `NextWus`: the next value of the WorkUnit accumulator after
///   registering this input.
/// - `NextStores`: the next value of the store accumulator.
/// - `NextPlatform`: the next value of the platform-tuple
///   accumulator, the builder's `Platform` parameter.
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

/// Router for WorkUnit-kind inputs. Appends `W` onto the WU
/// accumulator (registration order becomes the carrier type order);
/// passes stores and platform through.
pub struct UnitDispatch<W>(PhantomData<W>);

impl<W, Wus, Stores, Platform> Dispatch<Wus, Stores, Platform> for UnitDispatch<W>
where
    Wus: Concat<Cons<W, Empty>>,
{
    type NextPlatform = Platform;
    type NextStores = Stores;
    type NextWus = <Wus as Concat<Cons<W, Empty>>>::Out;
}

/// Router for store-kind inputs. Prepends `S` onto the store
/// accumulator; passes WUs and platform through.
pub struct StoreDispatch<S>(PhantomData<S>);

impl<S, Wus, Stores, Platform> Dispatch<Wus, Stores, Platform> for StoreDispatch<S> {
    type NextPlatform = Platform;
    type NextStores = Cons<S, Stores>;
    type NextWus = Wus;
}

// `KitDispatch<K>` lives in `hilavitkutin-kit` next to the `Kit`
// trait it routes for. The api crate does not depend on the kit
// crate (api is the contract; kit is a thin layer above it). See
// `hilavitkutin_kit::KitDispatch` for the router struct and its
// `Dispatch` impl.

/// Router for platform-kind inputs. Prepends `P` onto the
/// platform-tuple accumulator; passes WUs and stores through.
pub struct PlatformDispatch<P>(PhantomData<P>);

impl<P, Wus, Stores, Platform> Dispatch<Wus, Stores, Platform> for PlatformDispatch<P> {
    type NextPlatform = Cons<P, Platform>;
    type NextStores = Stores;
    type NextWus = Wus;
}

/// Type-system anchor for an extension-loaded input bin keyed by
/// trait family.
///
/// `ExtensionSurface<dyn TraitFamily>` is a zero-data marker that
/// registers "the scheduler will own a bin of `TraitFamily` impls
/// loaded by the extension layer". The runtime instance population
/// happens in `hilavitkutin-extensions` plus the engine; the
/// wrapper here is the type-system contract.
///
/// Implements `BuilderInput<Init = ()>` with `Dispatch = StoreDispatch<Self>`.
/// Construct via `ExtensionSurface::<dyn TraitFamily>::new()` and
/// register it with `.with(...)` like any other store.
///
/// The `Extension` prefix names the semantics (the app declares an
/// extension-loadable surface), and `Surface` names the role inside
/// the typestate (a slot the extension layer fills at runtime).
#[repr(transparent)]
pub struct ExtensionSurface<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized> ExtensionSurface<T> {
    /// Construct an `ExtensionSurface` marker.
    pub const fn new() -> Self {
        ExtensionSurface(PhantomData)
    }
}

impl<T: ?Sized + 'static> BuilderInput for ExtensionSurface<T> {
    type Dispatch = StoreDispatch<Self>;
    type Init = ();
}

impl<T: ?Sized + 'static> notko::HasTrivialCtor for ExtensionSurface<T> {
    fn new() -> Self {
        ExtensionSurface(PhantomData)
    }
}
