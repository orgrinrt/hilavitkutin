//! WorkUnit trait: test-local impl with a stub Ctx.
//!
//! A Ctx type must implement all seven `HasX` accessor traits. Each
//! points at a provider type that implements the matching API trait.
//! One shared stub provider plays all seven roles.

#![no_std]

use arvo::USize;
use hilavitkutin_api::values::{RCons, RNil};
use hilavitkutin_api::{
    AccessSet,
    Always,
    Atomic,
    BatchApi,
    BuilderInput,
    Column,
    ColumnReaderApi,
    ColumnValue,
    ColumnWriterApi,
    EachApi,
    HasBatch,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
    HasReduce,
    HasResourceProvider,
    HasVirtualFirer,
    Holds,
    Immediate,
    Normal,
    Plannable,
    ReadVals,
    RecordOp,
    ReduceApi,
    ResolveColumnRead,
    ResolveColumnWrite,
    ResolveResource,
    ResolveVirtualFire,
    Resource,
    ResourceProviderApi,
    UnitDispatch,
    Virtual,
    VirtualFirerApi,
    WorkUnit,
    WriteVals,
    read,
    rec,
    write,
};

// --- Stub provider (all-in-one) --------------------------------------

struct Stub;

impl<R: AccessSet> ColumnReaderApi<R> for Stub {
    unsafe fn read<T: ColumnValue, I>(&self, _i: USize) -> T
    where
        R: Holds<Column<T>>,
        Self: ResolveColumnRead<T, I>,
    {
        // Not actually called in this compile-only test; return a
        // zeroed value via Default-free route by transmuting from
        // size_of bytes of zero. Simpler: require T: Default in real
        // test paths; here the WU body never actually calls.
        unsafe { core::mem::zeroed() }
    }
}

impl<W: AccessSet> ColumnWriterApi<W> for Stub {}

impl<R: AccessSet> ResourceProviderApi<R> for Stub {
    fn resource<T: 'static, I>(&self) -> &T
    where
        R: Holds<Resource<T>>,
        Self: ResolveResource<T, I>,
    {
        // SAFETY: compile-only test body does not invoke this path.
        unsafe { &*(self as *const _ as *const T) }
    }
}

// Bridge impls satisfying the accessor `Self: Resolve*<T, I>` bounds.
// Compile-only test stub; bodies mirror the dummy accessor bodies above.
impl<T: 'static, I> ResolveResource<T, I> for Stub {
    fn resolve_resource(&self) -> &T {
        // SAFETY: compile-only test body does not invoke this path.
        unsafe { &*(self as *const _ as *const T) }
    }
}

impl<T: ColumnValue, I> ResolveColumnRead<T, I> for Stub {
    unsafe fn resolve_read(&self, _i: USize) -> T {
        unsafe { core::mem::zeroed() }
    }
}

impl<T: ColumnValue, I> ResolveColumnWrite<T, I> for Stub {
    unsafe fn resolve_write(&self, _i: USize, _v: T) {}

    unsafe fn resolve_held(&self, _i: USize) -> T {
        unsafe { core::mem::zeroed() }
    }
}

impl<W: AccessSet> VirtualFirerApi<W> for Stub {
    fn fire<V: 'static, I>(&self)
    where
        W: Holds<Virtual<V>>,
        Self: ResolveVirtualFire<V, I>,
    {
    }
}

impl<V: 'static, I> ResolveVirtualFire<V, I> for Stub {
    fn resolve_fire(&self) {}
}

impl<R: AccessSet, W: AccessSet> EachApi<R, W> for Stub {
    fn run<F>(&self, _f: F)
    where
        F: FnMut(USize),
    {
    }
}

impl<R: AccessSet, W: AccessSet> BatchApi<R, W> for Stub {
    fn run<F>(&self, _f: F)
    where
        F: FnMut(USize, USize),
    {
    }
}

impl<R: AccessSet, W: AccessSet> ReduceApi<R, W> for Stub {
    fn run<A, F>(&self, init: A, _f: F) -> A
    where
        A: 'static,
        F: FnMut(A, USize) -> A,
    {
        init
    }
}

// --- Ctx struct binds all seven HasX accessors -----------------------

struct Ctx {
    p: Stub,
}

impl<R: AccessSet> HasColumnReader<R> for Ctx {
    type Provider = Stub;

    fn reader(&self) -> &Stub {
        &self.p
    }
}

impl<W: AccessSet> HasColumnWriter<W> for Ctx {
    type Provider = Stub;

    fn writer(&self) -> &Stub {
        &self.p
    }
}

impl<R: AccessSet> HasResourceProvider<R> for Ctx {
    type Provider = Stub;

    fn resources(&self) -> &Stub {
        &self.p
    }
}

impl<W: AccessSet> HasVirtualFirer<W> for Ctx {
    type Provider = Stub;

    fn virtuals(&self) -> &Stub {
        &self.p
    }
}

impl<R: AccessSet, W: AccessSet> HasEach<R, W> for Ctx {
    type Provider = Stub;

    fn each(&self) -> &Stub {
        &self.p
    }
}

impl<R: AccessSet, W: AccessSet> HasBatch<R, W> for Ctx {
    type Provider = Stub;

    fn batch(&self) -> &Stub {
        &self.p
    }
}

impl<R: AccessSet, W: AccessSet> HasReduce<R, W> for Ctx {
    type Provider = Stub;

    fn reduce(&self) -> &Stub {
        &self.p
    }
}

// --- Consumer WU ------------------------------------------------------

struct Pos;
struct Vel;
struct GravFired;

struct Integrate;

impl BuilderInput for Integrate {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for Integrate {
    type Ctx<'frame> = Ctx;
    type Hint = (Immediate, Atomic, Normal);
    type Read = read![Column<Pos>, Column<Vel>];
    type Write = write![Column<Pos>, Virtual<GravFired>];

    fn execute<'frame>(&self, _ctx: &Ctx) {
        // Body is trivial: the test is that this compiles.
    }
}

#[test]
fn wu_compiles_and_executes() {
    let wu = Integrate;
    let ctx = Ctx {
        p: Stub,
    };
    wu.execute(&ctx);
}

// The blanket `impl<W: WorkUnit<S>, S> Plannable<S> for W` gives every
// WorkUnit the declaration trait. Monomorphise it for a real WorkUnit and
// assert the associated types forward. `type_eq` fails to compile unless
// `Integrate as Plannable<Always>` recovers exactly its WorkUnit sets, so a
// forwarding bug is a build error, not a silent pass.
#[test]
fn workunit_is_plannable_via_blanket() {
    fn assert_plannable<T: Plannable<Always>>() {}
    assert_plannable::<Integrate>();

    fn type_eq<A: 'static, B: 'static>()
    where
        A: SameAs<B>,
    {
    }
    type_eq::<<Integrate as Plannable<Always>>::Read, <Integrate as WorkUnit<Always>>::Read>();
    type_eq::<<Integrate as Plannable<Always>>::Write, <Integrate as WorkUnit<Always>>::Write>();
    type_eq::<<Integrate as Plannable<Always>>::Hint, <Integrate as WorkUnit<Always>>::Hint>();
}

// Minimal reflexive same-type witness: `A: SameAs<B>` holds only when A == B.
trait SameAs<B> {}
impl<T> SameAs<T> for T {}

// --- The value function under a name ---------------------------------

#[derive(Clone, Copy, PartialEq, Debug)]
struct Level(u32);

struct Double;

impl BuilderInput for Double {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for Double {
    type Ctx<'frame> = Ctx;
    type Hint = (Immediate, Atomic, Normal);
    type Read = read![Column<Level>];
    type Write = write![Column<Level>];

    fn execute<'frame>(&self, _ctx: &Ctx) {}
}

impl RecordOp<Always> for Double {
    fn apply(
        &self,
        read: &ReadVals<'_, <Self as WorkUnit<Always>>::Read>,
    ) -> WriteVals<<Self as WorkUnit<Always>>::Write> {
        rec![Level(read.get(Column::<Level>::new()).0 * 2)]
    }
}

// `RecordOp` reads the unit's own two lists and declares none of its own:
// the function is applied to a read product and returns the write product.
#[test]
fn record_op_is_the_function_over_the_units_own_lists() {
    let read = ReadVals::<read![Column<Level>]>(RCons {
        head: Level(21),
        tail: RNil,
    });
    let out = Double.apply(&read);
    assert_eq!(out.get(Column::<Level>::new()), Level(42));
}
