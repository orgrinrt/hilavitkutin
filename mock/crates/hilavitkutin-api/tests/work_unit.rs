//! WorkUnit trait: test-local impl with a stub Ctx.
//!
//! A Ctx type must implement all seven `HasX` accessor traits. Each
//! points at a provider type that implements the matching API trait.
//! One shared stub provider plays all seven roles.

#![no_std]

use arvo::USize;
use hilavitkutin_api::{
    AccessSet, Always, BatchApi, Column, ColumnReaderApi, ColumnValue, ColumnWriterApi,
    Contains, EachApi, HasBatch, HasColumnReader, HasColumnWriter, HasEach, HasReduce,
    HasResourceProvider, HasVirtualFirer, Immediate, Normal, ReduceApi, Resource,
    ResourceProviderApi, Atomic, Virtual, VirtualFirerApi, WorkUnit, read, write,
};

// --- Stub provider (all-in-one) --------------------------------------

struct Stub;

impl<R: AccessSet> ColumnReaderApi<R> for Stub {
    unsafe fn read<T: ColumnValue>(&self, _i: USize) -> T
    where
        R: Contains<Column<T>>,
    {
        // Not actually called in this compile-only test; return a
        // zeroed value via Default-free route by transmuting from
        // size_of bytes of zero. Simpler: require T: Default in real
        // test paths; here the WU body never actually calls.
        unsafe { core::mem::zeroed() }
    }
}

impl<W: AccessSet> ColumnWriterApi<W> for Stub {
    unsafe fn write<T: ColumnValue>(&self, _i: USize, _v: T)
    where
        W: Contains<Column<T>>,
    {
    }
}

impl<R: AccessSet> ResourceProviderApi<R> for Stub {
    fn resource<T: 'static>(&self) -> &T
    where
        R: Contains<Resource<T>>,
    {
        // SAFETY: compile-only test body does not invoke this path.
        unsafe { &*(self as *const _ as *const T) }
    }
}

impl<W: AccessSet> VirtualFirerApi<W> for Stub {
    fn fire<V: 'static>(&self)
    where
        W: Contains<Virtual<V>>,
    {
    }
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

impl WorkUnit<Always> for Integrate {
    type Read = read![Column<Pos>, Column<Vel>];
    type Write = write![Column<Pos>, Virtual<GravFired>];
    type Hint = (Immediate, Atomic, Normal);
    type Ctx = Ctx;

    fn execute(&self, _ctx: &Ctx) {
        // Body is trivial — the test is that this compiles.
    }
}

#[test]
fn wu_compiles_and_executes() {
    let wu = Integrate;
    let ctx = Ctx { p: Stub };
    wu.execute(&ctx);
}
