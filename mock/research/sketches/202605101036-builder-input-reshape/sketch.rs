//! BuilderInput trait reshape sketch — round 202605101036.
//!
//! Self-contained. Nightly with associated_type_defaults, min_specialization,
//! marker_trait_attr (the latter two requested by the design spec; only
//! associated_type_defaults is load-bearing for the sketch itself).
//!
//! Compile with:
//!     rustc +nightly --edition=2024 sketch.rs -o /dev/null
//!
//! Proves:
//!  1. `BuilderInput` with `type Init = Self;` default and `type Dispatch`.
//!  2. Sub-traits (`Resource`, `Column`, `Virtual`, `WorkUnit`, `Kit`,
//!     `MemoryProvider`, `ThreadPool`, `Clock`) supertrait-bound on
//!     `BuilderInput<Dispatch = ...Dispatch<Self>>`.
//!  3. `SchedulerBuilder::with<P: BuilderInput>` routes via `P::Dispatch`.
//!  4. `.extend::<dyn LinterApi>()` works as a separate method using
//!     the wrapper `ExtensionSurface<T: ?Sized>`.
//!  5. `AccessSet` over consumer types directly (no wrapper) composes.
//!  6. A WrongDispatch case demonstrates the supertrait bound fires.

#![no_std]
#![feature(associated_type_defaults)]
#![feature(min_specialization)]
#![feature(marker_trait_attr)]
#![allow(dead_code)]
#![allow(incomplete_features)]

use core::marker::PhantomData;

// ============================================================
// 1. Cons-list typestate primitives.
// ============================================================

pub struct Empty;
pub struct Cons<H, T>(PhantomData<(H, T)>);

// AccessSet membership trait. Sealed to consumers; cons-list driven.
#[marker]
pub trait AccessSet {}
impl AccessSet for Empty {}
impl<H, T> AccessSet for Cons<H, T> {}

/// `S` contains `X` (cons-list membership). `#[marker]` so the head-match
/// and tail-recurse impls are allowed to overlap (the trait carries no
/// items, so picking either branch is sound).
#[marker]
pub trait Contains<X> {}
impl<X, T> Contains<X> for Cons<X, T> {}
impl<X, H, T> Contains<X> for Cons<H, T> where T: Contains<X> {}

// Bundle marker traits used in Kit: type-level lists of WUs / stores.
#[marker]
pub trait WorkUnitBundle {}
impl WorkUnitBundle for Empty {}
impl<H, T> WorkUnitBundle for Cons<H, T> {}

#[marker]
pub trait StoreBundle {}
impl StoreBundle for Empty {}
impl<H, T> StoreBundle for Cons<H, T> {}

// ============================================================
// 2. BuilderInput — the unified dispatch trait.
// ============================================================

/// The single trait `.with(value)` accepts.
///
/// `Init` defaults to `Self` (assoc-type default; the typical case is
/// "the value handed to .with is the value the builder owns"). Stateless
/// markers like Column/Virtual override `Init = ()`.
///
/// `Dispatch` is the per-kind router that updates the builder typestate.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a BuilderInput; pass a registered builder input value to `.with(...)`",
    label = "not a BuilderInput",
    note = "implement one of: Resource, Column, Virtual, WorkUnit, Kit, MemoryProvider, ThreadPool, Clock"
)]
pub trait BuilderInput: Sized {
    /// Construction-side value type. Defaults to `Self`.
    type Init = Self;
    /// Dispatcher routing the kind to the typestate update.
    type Dispatch: Dispatch;
}

/// Per-kind typestate routing. Three accumulator GATs.
pub trait Dispatch {
    type NextWus<Wus>;
    type NextStores<Stores>;
    type NextPlat<Plat>;
}

pub struct UnitDispatch<W>(PhantomData<W>);
impl<W> Dispatch for UnitDispatch<W> {
    type NextWus<Wus> = Cons<W, Wus>;
    type NextStores<Stores> = Stores;
    type NextPlat<Plat> = Plat;
}

pub struct StoreDispatch<S>(PhantomData<S>);
impl<S> Dispatch for StoreDispatch<S> {
    type NextWus<Wus> = Wus;
    type NextStores<Stores> = Cons<S, Stores>;
    type NextPlat<Plat> = Plat;
}

pub struct KitDispatch<K>(PhantomData<K>);
impl<K> Dispatch for KitDispatch<K> {
    // Kits expand to their declared Units + Owned at build time. For the
    // sketch we route the kit identity onto the Wus accumulator; the real
    // engine projects Units/Owned via WorkUnitBundle/StoreBundle accumulators.
    type NextWus<Wus> = Cons<K, Wus>;
    type NextStores<Stores> = Stores;
    type NextPlat<Plat> = Plat;
}

pub struct PlatformDispatch<P>(PhantomData<P>);
impl<P> Dispatch for PlatformDispatch<P> {
    type NextWus<Wus> = Wus;
    type NextStores<Stores> = Stores;
    type NextPlat<Plat> = Cons<P, Plat>;
}

// ============================================================
// 3. Sub-traits with supertrait Dispatch equality bounds.
// ============================================================

/// Singleton store carrying a value.
pub trait Resource: BuilderInput<Dispatch = StoreDispatch<Self>> + Sized + 'static {}

// ColumnValue: the per-record bit-width contract. Stub for the sketch.
pub trait ColumnValue: Copy + 'static {
    const BIT_WIDTH: u32;
}

/// Per-record column. `Init = ()` (stateless declaration; values land at runtime).
pub trait Column:
    BuilderInput<Dispatch = StoreDispatch<Self>, Init = ()> + Sized + 'static + ColumnValue
{
}

/// Fired/event marker. `Init = ()`.
pub trait Virtual: BuilderInput<Dispatch = StoreDispatch<Self>, Init = ()> + Sized + 'static {}

// Schedule marker (the WU schedule axis: Always, Cadence, OnEvent, ...).
// For the sketch we only need `Always` as a default.
pub trait Schedule: 'static {}
pub struct Always;
impl Schedule for Always {}

/// Engine-executed unit of work.
pub trait WorkUnit:
    BuilderInput<Dispatch = UnitDispatch<Self>> + Send + Sync + 'static
{
    type Schedule: Schedule = Always;
    type Read: AccessSet;
    type Write: AccessSet;
    type Hint;
    type Ctx;
    fn execute(&self, ctx: &Self::Ctx);
}

/// Declarative bundle of WUs and stores.
pub trait Kit: BuilderInput<Dispatch = KitDispatch<Self>> + Sized + 'static {
    type Units: WorkUnitBundle;
    type Owned: StoreBundle;
}

// Platform contracts. Each requires a concrete API trait the user impls
// alongside BuilderInput.
pub trait MemoryProviderApi {}
pub trait ThreadPoolApi {}

// Use a dummy `Nanos` type so we don't need arvo here.
#[derive(Copy, Clone)]
pub struct Nanos(pub u64);
pub trait ClockApi {
    fn now_ns(&self) -> Nanos;
}

pub trait MemoryProvider:
    BuilderInput<Dispatch = PlatformDispatch<Self>> + MemoryProviderApi + 'static
{
}
pub trait ThreadPool:
    BuilderInput<Dispatch = PlatformDispatch<Self>> + ThreadPoolApi + 'static
{
}
pub trait Clock: BuilderInput<Dispatch = PlatformDispatch<Self>> + ClockApi + 'static {}

// ============================================================
// 4. ExtensionSurface — the dyn-Trait wrapper.
// ============================================================

/// Wrapper carrying a `dyn Trait` family by phantom. Unsized payload means
/// the consumer cannot bare-pass; `.extend::<dyn Trait>()` constructs this
/// internally and routes via `StoreDispatch`.
pub struct ExtensionSurface<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized + 'static> BuilderInput for ExtensionSurface<T> {
    type Init = ();
    type Dispatch = StoreDispatch<Self>;
}

// ============================================================
// 5. Demo consumer types (all bare-passed to .with).
// ============================================================

// Resource (carries a value, Init defaults to Self).
pub struct Interner {
    pub count: u32,
}
impl BuilderInput for Interner {
    type Dispatch = StoreDispatch<Self>;
    // Init defaults to Self.
}
impl Resource for Interner {}

// Column (per-record, Init = ()).
#[derive(Copy, Clone)]
pub struct Player {
    pub hp: u32,
}
impl ColumnValue for Player {
    const BIT_WIDTH: u32 = 32;
}
impl BuilderInput for Player {
    type Init = ();
    type Dispatch = StoreDispatch<Self>;
}
impl Column for Player {}

// Virtual (event marker, Init = ()).
pub struct Tick;
impl BuilderInput for Tick {
    type Init = ();
    type Dispatch = StoreDispatch<Self>;
}
impl Virtual for Tick {}

// WorkUnit (unit struct).
pub struct SpawnerWu;
impl BuilderInput for SpawnerWu {
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit for SpawnerWu {
    type Read = Empty;
    type Write = Empty;
    type Hint = Always;
    type Ctx = ();
    fn execute(&self, _ctx: &Self::Ctx) {}
}

// Kit (declarative bundle, empty Units/Owned for the sketch).
pub struct InputKit;
impl BuilderInput for InputKit {
    type Dispatch = KitDispatch<Self>;
}
impl Kit for InputKit {
    type Units = Empty;
    type Owned = Empty;
}

// Clock platform impl.
pub struct MyClock;
impl ClockApi for MyClock {
    fn now_ns(&self) -> Nanos {
        Nanos(0)
    }
}
impl BuilderInput for MyClock {
    type Dispatch = PlatformDispatch<Self>;
}
impl Clock for MyClock {}

// User-side dyn family for ExtensionSurface.
pub trait LinterApi: 'static {}

// ============================================================
// 6. SchedulerBuilder — the unified `.with` + separate `.extend`.
// ============================================================

pub struct SchedulerBuilder<Wus, Stores, Plat> {
    _phantom: PhantomData<(Wus, Stores, Plat)>,
}

impl SchedulerBuilder<Empty, Empty, Empty> {
    pub const fn new() -> Self {
        SchedulerBuilder {
            _phantom: PhantomData,
        }
    }
}

impl<Wus, Stores, Plat> SchedulerBuilder<Wus, Stores, Plat> {
    /// Consume one BuilderInput and update the typestate accumulators
    /// via `P::Dispatch`. Non-BuilderInput types fail with the
    /// `on_unimplemented` diagnostic above.
    pub fn with<P: BuilderInput>(
        self,
        _input: P,
    ) -> SchedulerBuilder<
        <P::Dispatch as Dispatch>::NextWus<Wus>,
        <P::Dispatch as Dispatch>::NextStores<Stores>,
        <P::Dispatch as Dispatch>::NextPlat<Plat>,
    > {
        SchedulerBuilder {
            _phantom: PhantomData,
        }
    }

    /// Add a `dyn Trait` extension family. Wrapped internally as
    /// `ExtensionSurface<T>`. The trait family does not have to be
    /// `Sized`, so `.with` cannot accept it; this method is the route.
    pub fn extend<T: ?Sized + 'static>(
        self,
    ) -> SchedulerBuilder<
        <<ExtensionSurface<T> as BuilderInput>::Dispatch as Dispatch>::NextWus<Wus>,
        <<ExtensionSurface<T> as BuilderInput>::Dispatch as Dispatch>::NextStores<Stores>,
        <<ExtensionSurface<T> as BuilderInput>::Dispatch as Dispatch>::NextPlat<Plat>,
    > {
        SchedulerBuilder {
            _phantom: PhantomData,
        }
    }

    pub fn build(self) -> Scheduler {
        Scheduler
    }
}

pub struct Scheduler;

// ============================================================
// 7. Call site — proves bare-pass ergonomics.
// ============================================================

fn build_demo() -> Scheduler {
    SchedulerBuilder::new()
        .with(Interner { count: 0 })
        .with(Player { hp: 100 })
        .with(Tick)
        .with(MyClock)
        .with(SpawnerWu)
        .with(InputKit)
        .extend::<dyn LinterApi>()
        .build()
}

// ============================================================
// 8. AccessSet over bare consumer types.
// ============================================================

type ReadSet = Cons<Interner, Cons<Player, Empty>>;

fn check_contains_interner<S: Contains<Interner>>() {}
fn check_contains_player<S: Contains<Player>>() {}

fn _checks() {
    check_contains_interner::<ReadSet>();
    check_contains_player::<ReadSet>();
}

// ============================================================
// 9. Negative case — supertrait Dispatch equality enforcement.
// ============================================================
//
// This block, if uncommented, MUST fail to compile because:
//
//   `Resource` is `BuilderInput<Dispatch = StoreDispatch<Self>>`.
//   `WrongDispatch` impls `BuilderInput` with `Dispatch = UnitDispatch<Self>`.
//   The `impl Resource for WrongDispatch` line cannot satisfy the
//   supertrait equality constraint, so trait-resolution rejects it.
//
// Verified by uncommenting locally during sketch development. Left commented
// here so the sketch compiles cleanly as a single artefact.
//
// ```rust,compile_fail
// pub struct WrongDispatch;
// impl BuilderInput for WrongDispatch {
//     type Dispatch = UnitDispatch<Self>;
// }
// impl Resource for WrongDispatch {}  // ERROR: dispatch type mismatch.
// ```

// ============================================================
// 10. Tiny driver so we can compile as a binary if desired (not used in
//     the cargo lib check; harmless for `rustc --crate-type lib`).
// ============================================================

#[allow(dead_code)]
fn _entry() {
    let _scheduler = build_demo();
}
