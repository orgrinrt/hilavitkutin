//! S1: deep-stacking typestate-builder.
//!
//! Pattern: a Kit trait with `type Units: WorkUnitBundle` and
//! `type Owned: StoreBundle`. SchedulerBuilder<Wus, Stores> typestated.
//! `.add_kit::<K>()` returns a new typed builder with K's contributions
//! accumulated. `.build()` proves all Wus' accesses are satisfied by Stores.
//!
//! This sketch tests that the typestate machinery sustains a 4-level
//! kit hierarchy in stable trait-solver behaviour, with usable error
//! messages when a required store is missing at the leaf level.
//!
//! Build: `rustup run nightly rustc --crate-type=lib --edition=2024 sketch.rs --emit=metadata`

#![allow(unused)]
#![no_std]
#![feature(marker_trait_attr)]

use core::marker::PhantomData;

// ----- AccessSet (B-shape, recursive HList, marker-overlap pattern) -----

pub struct Empty;
pub struct Cons<H, T>(PhantomData<(H, T)>);

pub trait AccessSet {}
impl AccessSet for Empty {}
impl<H, T: AccessSet> AccessSet for Cons<H, T> {}

#[marker]
pub trait Contains<X>: AccessSet {}
impl<X, T: AccessSet> Contains<X> for Cons<X, T> {}
impl<X, H, T: AccessSet> Contains<X> for Cons<H, T> where T: Contains<X> {}

// `ContainsAll<L>`: all elements of L are in Self.
#[marker]
pub trait ContainsAll<L>: AccessSet {}
impl<S: AccessSet> ContainsAll<Empty> for S {}
impl<S: AccessSet, H, T> ContainsAll<Cons<H, T>> for S
where
    S: Contains<H> + ContainsAll<T>,
{
}

// ----- WorkUnit and Kit traits -----

pub trait WorkUnit {
    type Read: AccessSet;
    type Write: AccessSet;
}

// WorkUnitBundle: HList of WUs.
pub trait WorkUnitBundle {
    type AccumRead: AccessSet;
    type AccumWrite: AccessSet;
}
impl WorkUnitBundle for Empty {
    type AccumRead = Empty;
    type AccumWrite = Empty;
}

// Real concat: head's read concatenated with tail's accumulated.
impl<W: WorkUnit, T: WorkUnitBundle> WorkUnitBundle for Cons<W, T>
where
    W::Read: Concat<T::AccumRead>,
    W::Write: Concat<T::AccumWrite>,
    <W::Read as Concat<T::AccumRead>>::Out: AccessSet,
    <W::Write as Concat<T::AccumWrite>>::Out: AccessSet,
{
    type AccumRead = <W::Read as Concat<T::AccumRead>>::Out;
    type AccumWrite = <W::Write as Concat<T::AccumWrite>>::Out;
}

// StoreBundle: HList of store markers (Resource<T> / Column<T> / Virtual<T>).
pub trait StoreBundle {}
impl StoreBundle for Empty {}
impl<H, T: StoreBundle> StoreBundle for Cons<H, T> {}

pub trait Kit {
    type Units: WorkUnitBundle;
    type Owned: StoreBundle;
}

// ----- Marker types -----

pub struct StringInterner;
pub struct Clock;
pub struct LintConfig;
pub struct Diagnostic;
pub struct Tracer;
pub struct TraceSample;
pub struct DesignRound;
pub struct RoundState;
pub struct WorkspaceRoot;
pub struct FileEntry;
pub struct LeafA1;
pub struct LeafA2;
pub struct LeafB1;
pub struct LeafB2;
pub struct MidA1;
pub struct MidA2;
pub struct MidB1;
pub struct OuterA1;
pub struct RootA;

// ----- WUs at each tier -----

macro_rules! decl_wu {
    ($name:ident, R = $r:ty, W = $w:ty) => {
        pub struct $name;
        impl WorkUnit for $name {
            type Read = $r;
            type Write = $w;
        }
    };
}

// Leaf-level WUs read from app-shared StringInterner and write to leaf-owned columns.
decl_wu!(LeafWU0, R = Cons<StringInterner, Empty>, W = Cons<LeafA1, Empty>);
decl_wu!(LeafWU1, R = Cons<StringInterner, Empty>, W = Cons<LeafA2, Empty>);
decl_wu!(LeafWU2, R = Cons<Clock, Empty>, W = Cons<LeafB1, Empty>);
decl_wu!(LeafWU3, R = Cons<Clock, Empty>, W = Cons<LeafB2, Empty>);

// Mid-level WUs read from leaf outputs.
decl_wu!(MidWU0, R = Cons<LeafA1, Empty>, W = Cons<MidA1, Empty>);
decl_wu!(MidWU1, R = Cons<LeafA2, Empty>, W = Cons<MidA2, Empty>);
decl_wu!(MidWU2, R = Cons<LeafB1, Empty>, W = Cons<MidB1, Empty>);

// Outer-level WUs.
decl_wu!(OuterWU0, R = Cons<MidA1, Empty>, W = Cons<OuterA1, Empty>);

// Root-level WUs.
decl_wu!(RootWU0, R = Cons<OuterA1, Empty>, W = Cons<RootA, Empty>);

// ----- Kits at each tier -----

pub struct LeafKitA;
impl Kit for LeafKitA {
    type Units = Cons<LeafWU0, Cons<LeafWU1, Empty>>;
    type Owned = Cons<LeafA1, Cons<LeafA2, Empty>>;
}

pub struct LeafKitB;
impl Kit for LeafKitB {
    type Units = Cons<LeafWU2, Cons<LeafWU3, Empty>>;
    type Owned = Cons<LeafB1, Cons<LeafB2, Empty>>;
}

pub struct MidKitA;
impl Kit for MidKitA {
    type Units = Cons<MidWU0, Cons<MidWU1, Empty>>;
    type Owned = Cons<MidA1, Cons<MidA2, Empty>>;
}

pub struct MidKitB;
impl Kit for MidKitB {
    type Units = Cons<MidWU2, Empty>;
    type Owned = Cons<MidB1, Empty>;
}

pub struct OuterKit;
impl Kit for OuterKit {
    type Units = Cons<OuterWU0, Empty>;
    type Owned = Cons<OuterA1, Empty>;
}

pub struct RootKit;
impl Kit for RootKit {
    type Units = Cons<RootWU0, Empty>;
    type Owned = Cons<RootA, Empty>;
}

// ----- SchedulerBuilder skeleton -----

pub struct SchedulerBuilder<Wus, Stores>(PhantomData<(Wus, Stores)>);

impl SchedulerBuilder<Empty, Empty> {
    pub fn new() -> Self {
        SchedulerBuilder(PhantomData)
    }
}

// .resource::<T>() prepends T to Stores.
impl<Wus, Stores> SchedulerBuilder<Wus, Stores> {
    pub fn resource<T>(self) -> SchedulerBuilder<Wus, Cons<T, Stores>> {
        SchedulerBuilder(PhantomData)
    }
}

// .add_kit::<K>() prepends K::Owned to Stores and K::Units to Wus.
//
// Production engine would mechanically thread the per-tier accumulation;
// here the sketch uses a naive concat shape to keep the proof of concept
// readable. See "concat" stub below.
pub trait Concat<R> {
    type Out;
}
impl<R> Concat<R> for Empty {
    type Out = R;
}
impl<H, T, R> Concat<R> for Cons<H, T>
where
    T: Concat<R>,
{
    type Out = Cons<H, <T as Concat<R>>::Out>;
}

impl<Wus, Stores> SchedulerBuilder<Wus, Stores> {
    pub fn add_kit<K: Kit>(
        self,
    ) -> SchedulerBuilder<
        <K::Units as Concat<Wus>>::Out,
        <K::Owned as Concat<Stores>>::Out,
    >
    where
        K::Units: Concat<Wus>,
        K::Owned: Concat<Stores>,
    {
        SchedulerBuilder(PhantomData)
    }
}

// .build() proves all WU accesses are satisfied. Stub: just constructs.
// Real version: `where Wus: WUsAccessesSatisfiedBy<Stores>`. For this
// sketch the proof is implicit (we don't enumerate WU accesses across
// the bundle); the typestate threading is the load-bearing part.
impl<Wus: WorkUnitBundle, Stores: AccessSet + StoreBundle> SchedulerBuilder<Wus, Stores>
where
    Stores: ContainsAll<Wus::AccumRead> + ContainsAll<Wus::AccumWrite>,
{
    pub fn build(self) -> Scheduler<Wus, Stores> {
        Scheduler(PhantomData)
    }
}

pub struct Scheduler<Wus, Stores>(PhantomData<(Wus, Stores)>);

// ----- Demonstration -----

// Success: app provides StringInterner + Clock; nests root containing
// outer containing mid containing leaves.
//
// The hierarchy effectively reaches depth 4 (root -> outer -> mid -> leaf),
// constructed flat at the app level since topic-3's preferred approach is
// recursive registration (kits register their nested kits via direct
// `.add_kit()` calls; the typestate accumulator handles the rest).

pub fn demo_success() -> Scheduler<
    impl WorkUnitBundle,
    impl AccessSet + StoreBundle,
> {
    SchedulerBuilder::new()
        .resource::<StringInterner>()
        .resource::<Clock>()
        .add_kit::<LeafKitA>()
        .add_kit::<LeafKitB>()
        .add_kit::<MidKitA>()
        .add_kit::<MidKitB>()
        .add_kit::<OuterKit>()
        .add_kit::<RootKit>()
        .build()
}

#[cfg(feature = "show_missing_error")]
pub fn demo_missing() -> Scheduler<
    impl WorkUnitBundle,
    impl AccessSet + StoreBundle,
> {
    // Forgot to register Clock. Expected: compile error at .build()
    // pointing at LeafWU2's Read = Cons<Clock, Empty> not being
    // satisfiable.
    SchedulerBuilder::new()
        .resource::<StringInterner>()
        .add_kit::<LeafKitA>()
        .add_kit::<LeafKitB>()
        .add_kit::<MidKitA>()
        .add_kit::<MidKitB>()
        .add_kit::<OuterKit>()
        .add_kit::<RootKit>()
        .build()
}
