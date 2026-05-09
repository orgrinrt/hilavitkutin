//! Builder-provider-shape sketch — round 202605091700 topic 2.
//!
//! Self-contained. Compile with:
//!     rustc +nightly --edition=2024 sketch.rs -o /dev/null
//!
//! Proves the unified `.with(value)` shape works with one dispatch
//! trait (`Provider`) plus a const marker trait (`Marker`) for
//! library-side wrappers that need a uniform `const fn new()`.

#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![allow(dead_code)]

use core::marker::PhantomData;

// ============================================================
// 1. Cons-list typestate primitives.
// ============================================================

pub struct Empty;
pub struct Cons<H, T>(PhantomData<(H, T)>);

// ============================================================
// 2. Provider — the dispatch trait the builder asks for.
// ============================================================

/// Sealed marker. Kind discriminator carried in the typestate.
pub enum Kind {
    Unit,         // WorkUnit
    Resource,     // singleton store
    Column,       // record store
    Virtual,      // event marker
    LinkedBin,    // dyn extension family
    Kit,          // bundle preset
    Memory,       // platform memory provider
    Threads,      // platform thread pool
    Clock,        // platform clock
}

#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a Provider; pass a registered provider value to `.with(...)`",
    label = "not a Provider",
    note = "use `Resource::new(value)` for singleton state, `Column::<T>::new()` / `Virtual::<T>::new()` / `LinkedBin::<dyn Trait>::new()` for type-keyed declarations, or impl `Provider` on your unit-struct WUs/Kits/platform impls."
)]
pub trait Provider: Sized {
    /// Construction-side value type (Init = () for stateless markers).
    type Init;
    /// What the typestate accumulates onto.
    const KIND: Kind;
    /// Dispatcher routing kind to typestate update.
    type Dispatch: Dispatch;
}

/// Per-kind typestate routing. One impl per kind; selected by
/// `Provider::Dispatch`.
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

pub struct PlatDispatch<P>(PhantomData<P>);
impl<P> Dispatch for PlatDispatch<P> {
    type NextWus<Wus> = Wus;
    type NextStores<Stores> = Stores;
    type NextPlat<Plat> = Cons<P, Plat>;
}

// ============================================================
// 3. Marker — const trait for library-side wrappers needing
//    a no-arg const constructor.
// ============================================================

pub const trait Marker: Provider<Init = ()> + Sized {
    fn new() -> Self;
}

// ============================================================
// 4. Library-side wrapper providers.
// ============================================================

pub struct Resource<T>(T);
impl<T: 'static> Resource<T> {
    pub const fn new(t: T) -> Self { Resource(t) }
}
impl<T: 'static> Provider for Resource<T> {
    type Init = T;
    const KIND: Kind = Kind::Resource;
    type Dispatch = StoreDispatch<Self>;
}

pub struct Column<T>(PhantomData<T>);
impl<T: 'static> Provider for Column<T> {
    type Init = ();
    const KIND: Kind = Kind::Column;
    type Dispatch = StoreDispatch<Self>;
}
impl<T: 'static> const Marker for Column<T> {
    fn new() -> Self { Column(PhantomData) }
}

pub struct Virtual<T>(PhantomData<T>);
impl<T: 'static> Provider for Virtual<T> {
    type Init = ();
    const KIND: Kind = Kind::Virtual;
    type Dispatch = StoreDispatch<Self>;
}
impl<T: 'static> const Marker for Virtual<T> {
    fn new() -> Self { Virtual(PhantomData) }
}

pub struct LinkedBin<T: ?Sized>(PhantomData<T>);
impl<T: ?Sized + 'static> Provider for LinkedBin<T> {
    type Init = ();
    const KIND: Kind = Kind::LinkedBin;
    type Dispatch = StoreDispatch<Self>;
}
impl<T: ?Sized + 'static> const Marker for LinkedBin<T> {
    fn new() -> Self { LinkedBin(PhantomData) }
}

// ============================================================
// 5. User-authored providers (unit structs and platform impls).
// ============================================================

pub trait WorkUnit: Provider<Init = Self> + Sized {}
pub trait Kit: Provider<Init = Self> + Sized {}

// User-side WU
pub struct SpawnerWu;
impl Provider for SpawnerWu {
    type Init = Self;
    const KIND: Kind = Kind::Unit;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit for SpawnerWu {}

pub struct PhysicsWu;
impl Provider for PhysicsWu {
    type Init = Self;
    const KIND: Kind = Kind::Unit;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit for PhysicsWu {}

// User-side Kit
pub struct InputKit;
impl Provider for InputKit {
    type Init = Self;
    const KIND: Kind = Kind::Kit;
    type Dispatch = UnitDispatch<Self>;
}
impl Kit for InputKit {}

// Platform impls — user-defined, with their own constructors.
pub trait MemoryProviderApi {}
pub struct MyMemory { _arena: usize }
impl MyMemory { pub fn new(arena: usize) -> Self { Self { _arena: arena } } }
impl MemoryProviderApi for MyMemory {}
impl Provider for MyMemory {
    type Init = Self;
    const KIND: Kind = Kind::Memory;
    type Dispatch = PlatDispatch<Self>;
}

pub trait ThreadPoolApi {}
pub struct MyThreadPool { _n: usize }
impl MyThreadPool { pub fn new(n: usize) -> Self { Self { _n: n } } }
impl ThreadPoolApi for MyThreadPool {}
impl Provider for MyThreadPool {
    type Init = Self;
    const KIND: Kind = Kind::Threads;
    type Dispatch = PlatDispatch<Self>;
}

pub trait ClockApi {}
pub struct MyClock;
impl ClockApi for MyClock {}
impl Provider for MyClock {
    type Init = Self;
    const KIND: Kind = Kind::Clock;
    type Dispatch = PlatDispatch<Self>;
}

// User-side data shapes (live inside Column<T>, Resource<T>, Virtual<T>)
pub struct Player;
pub struct GameState { _x: u32 }
impl GameState {
    pub fn default() -> Self { Self { _x: 0 } }
}
pub struct Tick;

// User-side dyn family for LinkedBin
pub trait LinterApi {}

// ============================================================
// 6. Linked-extension example. Blanket impl of LinterApi for
//    LinkedProvider<{ID}> (round 202605091700 topic 1 mechanism).
// ============================================================

pub struct LinkedProvider<const ID: u64> {
    _vtable: *const (),
}
unsafe impl<const ID: u64> Send for LinkedProvider<{ID}> {}
unsafe impl<const ID: u64> Sync for LinkedProvider<{ID}> {}

impl<const ID: u64> LinterApi for LinkedProvider<{ID}> {}

// ============================================================
// 7. The SchedulerBuilder with the unified `.with` method.
// ============================================================

pub struct SchedulerBuilder<Wus, Stores, Plat> {
    _phantom: PhantomData<(Wus, Stores, Plat)>,
}

impl SchedulerBuilder<Empty, Empty, Empty> {
    pub const fn new() -> Self {
        SchedulerBuilder { _phantom: PhantomData }
    }
}

/// Internal dispatch — per-kind typestate update.
///
/// The trick: one `.with` method, but the typestate update path
/// varies by Provider::KIND. We encode the per-kind output via
/// an associated-type trait keyed on KIND.
///
/// In production this would use sealed trait specialization or
/// a `const KIND` dispatch via a helper trait. For sketch purposes
/// we show the three categories (WU adds to Wus; Resource/Column/
/// Virtual/LinkedBin add to Stores; platform binds to Plat) via
/// kind-specific `.with_*_kind` methods, then prove a blanket
/// `.with<P: Provider>` can route via a per-kind dispatch trait.

impl<Wus, Stores, Plat> SchedulerBuilder<Wus, Stores, Plat> {
    /// The one verb. `_provider` is consumed by ownership; per-kind
    /// typestate routing happens through `P::Dispatch`. Non-Provider
    /// types fail with the Provider `on_unimplemented` diagnostic.
    pub fn with<P: Provider>(
        self,
        _provider: P,
    ) -> SchedulerBuilder<
        <P::Dispatch as Dispatch>::NextWus<Wus>,
        <P::Dispatch as Dispatch>::NextStores<Stores>,
        <P::Dispatch as Dispatch>::NextPlat<Plat>,
    > {
        SchedulerBuilder { _phantom: PhantomData }
    }
}

pub struct Scheduler;

impl<Wus, Stores, Plat> SchedulerBuilder<Wus, Stores, Plat> {
    pub fn build(self) -> Scheduler {
        Scheduler
    }
}

// ============================================================
// 8. Call site — the actual ergonomics test.
// ============================================================

fn build_app() -> Scheduler {
    SchedulerBuilder::new()
        .with(MyMemory::new(4096))
        .with(MyThreadPool::new(8))
        .with(MyClock)
        .with(Resource::new(GameState::default()))
        .with(Column::<Player>::new())
        .with(Virtual::<Tick>::new())
        .with(LinkedBin::<dyn LinterApi>::new())
        .with(InputKit)
        .with(SpawnerWu)
        .with(PhysicsWu)
        .build()
}

fn main() {
    let _scheduler = build_app();
}
