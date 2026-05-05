//! S1b: deep-stacking typestate-builder at depth 5.
//!
//! Extends S1 (`../202605050530_deep_stacking/sketch.rs`) from a 4-tier,
//! 9-WU hierarchy to a 5-tier, 25-WU hierarchy with 13 store markers.
//! Audit C3 / M3 / M5 specified depth 5 with 25 WUs and 12-15 stores; this
//! file targets that exact shape.
//!
//! Tier counts:
//!   Leaf:  8 WUs writing into 4 leaf columns (heavy fan-in: each column
//!          has 2 writers, exercising AccumWrite duplication that M2
//!          flagged).
//!   Mid:   6 WUs writing into 3 mid columns.
//!   Outer: 5 WUs writing into 2 outer columns.
//!   Root:  3 WUs writing into 1 root column.
//!   Meta:  3 WUs writing into 1 meta column.
//!   Total: 25 WUs, 13 stores (4 leaf + 3 mid + 2 outer + 1 root + 1 meta
//!          + 2 shared resources).
//!
//! Build success path:
//!   `time rustup run nightly rustc --crate-type=lib --edition=2024 \
//!       sketch.rs --emit=metadata`
//!
//! Missing-resource error path (Clock omitted):
//!   `rustup run nightly rustc --crate-type=lib --edition=2024 \
//!       sketch.rs --emit=metadata --cfg feature="show_missing_error"`

#![allow(unused)]
#![no_std]
#![feature(marker_trait_attr)]

use core::marker::PhantomData;

// ----- AccessSet substrate (identical to S1) -----

pub struct Empty;
pub struct Cons<H, T>(PhantomData<(H, T)>);

pub trait AccessSet {}
impl AccessSet for Empty {}
impl<H, T: AccessSet> AccessSet for Cons<H, T> {}

#[marker]
pub trait Contains<X>: AccessSet {}
impl<X, T: AccessSet> Contains<X> for Cons<X, T> {}
impl<X, H, T: AccessSet> Contains<X> for Cons<H, T> where T: Contains<X> {}

#[marker]
pub trait ContainsAll<L>: AccessSet {}
impl<S: AccessSet> ContainsAll<Empty> for S {}
impl<S: AccessSet, H, T> ContainsAll<Cons<H, T>> for S
where
    S: Contains<H> + ContainsAll<T>,
{
}

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

pub trait WorkUnit {
    type Read: AccessSet;
    type Write: AccessSet;
}

pub trait WorkUnitBundle {
    type AccumRead: AccessSet;
    type AccumWrite: AccessSet;
}
impl WorkUnitBundle for Empty {
    type AccumRead = Empty;
    type AccumWrite = Empty;
}
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

pub trait StoreBundle {}
impl StoreBundle for Empty {}
impl<H, T: StoreBundle> StoreBundle for Cons<H, T> {}

pub trait Kit {
    type Units: WorkUnitBundle;
    type Owned: StoreBundle;
}

// ----- Marker types: 13 stores plus 2 shared resources -----

pub struct StringInterner;
pub struct Clock;

pub struct LeafA;
pub struct LeafB;
pub struct LeafC;
pub struct LeafD;

pub struct MidA;
pub struct MidB;
pub struct MidC;

pub struct OuterA;
pub struct OuterB;

pub struct RootR;

pub struct MetaR;

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

// Tier 1: Leaf. 8 WUs, 2 writers per leaf column.
decl_wu!(LeafWU0, R = Cons<StringInterner, Empty>, W = Cons<LeafA, Empty>);
decl_wu!(LeafWU1, R = Cons<StringInterner, Empty>, W = Cons<LeafA, Empty>);
decl_wu!(LeafWU2, R = Cons<Clock, Empty>,          W = Cons<LeafB, Empty>);
decl_wu!(LeafWU3, R = Cons<Clock, Empty>,          W = Cons<LeafB, Empty>);
decl_wu!(LeafWU4, R = Cons<StringInterner, Empty>, W = Cons<LeafC, Empty>);
decl_wu!(LeafWU5, R = Cons<StringInterner, Empty>, W = Cons<LeafC, Empty>);
decl_wu!(LeafWU6, R = Cons<Clock, Empty>,          W = Cons<LeafD, Empty>);
decl_wu!(LeafWU7, R = Cons<Clock, Empty>,          W = Cons<LeafD, Empty>);

// Tier 2: Mid. 6 WUs.
decl_wu!(MidWU0, R = Cons<LeafA, Empty>, W = Cons<MidA, Empty>);
decl_wu!(MidWU1, R = Cons<LeafA, Empty>, W = Cons<MidA, Empty>);
decl_wu!(MidWU2, R = Cons<LeafB, Empty>, W = Cons<MidB, Empty>);
decl_wu!(MidWU3, R = Cons<LeafB, Empty>, W = Cons<MidB, Empty>);
decl_wu!(MidWU4, R = Cons<LeafC, Cons<LeafD, Empty>>, W = Cons<MidC, Empty>);
decl_wu!(MidWU5, R = Cons<LeafC, Cons<LeafD, Empty>>, W = Cons<MidC, Empty>);

// Tier 3: Outer. 5 WUs.
decl_wu!(OuterWU0, R = Cons<MidA, Cons<MidB, Empty>>, W = Cons<OuterA, Empty>);
decl_wu!(OuterWU1, R = Cons<MidA, Cons<MidB, Empty>>, W = Cons<OuterA, Empty>);
decl_wu!(OuterWU2, R = Cons<MidB, Cons<MidC, Empty>>, W = Cons<OuterB, Empty>);
decl_wu!(OuterWU3, R = Cons<MidB, Cons<MidC, Empty>>, W = Cons<OuterB, Empty>);
decl_wu!(OuterWU4, R = Cons<MidB, Cons<MidC, Empty>>, W = Cons<OuterB, Empty>);

// Tier 4: Root. 3 WUs.
decl_wu!(RootWU0, R = Cons<OuterA, Cons<OuterB, Empty>>, W = Cons<RootR, Empty>);
decl_wu!(RootWU1, R = Cons<OuterA, Cons<OuterB, Empty>>, W = Cons<RootR, Empty>);
decl_wu!(RootWU2, R = Cons<OuterA, Cons<OuterB, Empty>>, W = Cons<RootR, Empty>);

// Tier 5: Meta (new for depth 5). 3 WUs.
decl_wu!(MetaWU0, R = Cons<RootR, Empty>, W = Cons<MetaR, Empty>);
decl_wu!(MetaWU1, R = Cons<RootR, Empty>, W = Cons<MetaR, Empty>);
decl_wu!(MetaWU2, R = Cons<RootR, Empty>, W = Cons<MetaR, Empty>);

// ----- Kits at each tier -----

pub struct LeafKitAB;
impl Kit for LeafKitAB {
    type Units = Cons<LeafWU0, Cons<LeafWU1, Cons<LeafWU2, Cons<LeafWU3, Empty>>>>;
    type Owned = Cons<LeafA, Cons<LeafB, Empty>>;
}

pub struct LeafKitCD;
impl Kit for LeafKitCD {
    type Units = Cons<LeafWU4, Cons<LeafWU5, Cons<LeafWU6, Cons<LeafWU7, Empty>>>>;
    type Owned = Cons<LeafC, Cons<LeafD, Empty>>;
}

pub struct MidKitAB;
impl Kit for MidKitAB {
    type Units = Cons<MidWU0, Cons<MidWU1, Cons<MidWU2, Cons<MidWU3, Empty>>>>;
    type Owned = Cons<MidA, Cons<MidB, Empty>>;
}

pub struct MidKitC;
impl Kit for MidKitC {
    type Units = Cons<MidWU4, Cons<MidWU5, Empty>>;
    type Owned = Cons<MidC, Empty>;
}

pub struct OuterKit;
impl Kit for OuterKit {
    type Units = Cons<OuterWU0, Cons<OuterWU1, Cons<OuterWU2, Cons<OuterWU3, Cons<OuterWU4, Empty>>>>>;
    type Owned = Cons<OuterA, Cons<OuterB, Empty>>;
}

pub struct RootKit;
impl Kit for RootKit {
    type Units = Cons<RootWU0, Cons<RootWU1, Cons<RootWU2, Empty>>>;
    type Owned = Cons<RootR, Empty>;
}

pub struct MetaKit;
impl Kit for MetaKit {
    type Units = Cons<MetaWU0, Cons<MetaWU1, Cons<MetaWU2, Empty>>>;
    type Owned = Cons<MetaR, Empty>;
}

// ----- SchedulerBuilder typestate -----

pub struct SchedulerBuilder<Wus, Stores>(PhantomData<(Wus, Stores)>);

impl SchedulerBuilder<Empty, Empty> {
    pub fn new() -> Self {
        SchedulerBuilder(PhantomData)
    }
}

impl<Wus, Stores> SchedulerBuilder<Wus, Stores> {
    pub fn resource<T>(self) -> SchedulerBuilder<Wus, Cons<T, Stores>> {
        SchedulerBuilder(PhantomData)
    }

    pub fn add_kit<K: Kit>(
        self,
    ) -> SchedulerBuilder<<K::Units as Concat<Wus>>::Out, <K::Owned as Concat<Stores>>::Out>
    where
        K::Units: Concat<Wus>,
        K::Owned: Concat<Stores>,
    {
        SchedulerBuilder(PhantomData)
    }
}

pub struct Scheduler<Wus, Stores>(PhantomData<(Wus, Stores)>);

impl<Wus: WorkUnitBundle, Stores: AccessSet + StoreBundle> SchedulerBuilder<Wus, Stores>
where
    Stores: ContainsAll<Wus::AccumRead> + ContainsAll<Wus::AccumWrite>,
{
    pub fn build(self) -> Scheduler<Wus, Stores> {
        Scheduler(PhantomData)
    }
}

// ----- Demonstrations -----

pub fn demo_success() -> Scheduler<impl WorkUnitBundle, impl AccessSet + StoreBundle> {
    SchedulerBuilder::new()
        .resource::<StringInterner>()
        .resource::<Clock>()
        .add_kit::<LeafKitAB>()
        .add_kit::<LeafKitCD>()
        .add_kit::<MidKitAB>()
        .add_kit::<MidKitC>()
        .add_kit::<OuterKit>()
        .add_kit::<RootKit>()
        .add_kit::<MetaKit>()
        .build()
}

#[cfg(feature = "show_missing_error")]
pub fn demo_missing() -> Scheduler<impl WorkUnitBundle, impl AccessSet + StoreBundle> {
    // Forgot to register Clock. Expected: compile error pointing at one
    // of the leaf-tier WUs that reads Clock, with a depth-5 nested Cons
    // chain in the error type.
    SchedulerBuilder::new()
        .resource::<StringInterner>()
        .add_kit::<LeafKitAB>()
        .add_kit::<LeafKitCD>()
        .add_kit::<MidKitAB>()
        .add_kit::<MidKitC>()
        .add_kit::<OuterKit>()
        .add_kit::<RootKit>()
        .add_kit::<MetaKit>()
        .build()
}
