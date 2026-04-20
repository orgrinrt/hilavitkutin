//! Stub provider shapes wired through the seven accessor traits.
//!
//! Confirms the accessor traits work on a real tuple-like container
//! type. Covers both single-parameter (ColumnReader/ColumnWriter/
//! ResourceProvider/VirtualFirer) and two-parameter (Each/Batch/
//! Reduce) accessor shapes.

#![no_std]

use arvo::newtype::USize;
use hilavitkutin_api::{
    AccessSet, BatchApi, Column, ColumnReaderApi, ColumnValue, ColumnWriterApi, Contains,
    EachApi, HasBatch, HasColumnReader, HasColumnWriter, HasEach, HasReduce,
    HasResourceProvider, HasVirtualFirer, ReduceApi, Resource, ResourceProviderApi, Virtual,
    VirtualFirerApi,
};

// Each provider is a distinct type to make the trait resolution
// unambiguous at the accessor-trait impl sites.

struct ReaderP;
struct WriterP;
struct ResourceP;
struct FirerP;
struct EachP;
struct BatchP;
struct ReduceP;

impl<R: AccessSet> ColumnReaderApi<R> for ReaderP {
    unsafe fn read<T: ColumnValue>(&self, _i: USize) -> T
    where
        R: Contains<Column<T>>,
    {
        unsafe { core::mem::zeroed() }
    }
}

impl<W: AccessSet> ColumnWriterApi<W> for WriterP {
    unsafe fn write<T: ColumnValue>(&self, _i: USize, _v: T)
    where
        W: Contains<Column<T>>,
    {
    }
}

impl<R: AccessSet> ResourceProviderApi<R> for ResourceP {
    fn resource<T: 'static>(&self) -> &T
    where
        R: Contains<Resource<T>>,
    {
        unsafe { &*(self as *const _ as *const T) }
    }
}

impl<W: AccessSet> VirtualFirerApi<W> for FirerP {
    fn fire<V: 'static>(&self)
    where
        W: Contains<Virtual<V>>,
    {
    }
}

impl<R: AccessSet, W: AccessSet> EachApi<R, W> for EachP {
    fn run<F>(&self, _f: F)
    where
        F: FnMut(USize),
    {
    }
}

impl<R: AccessSet, W: AccessSet> BatchApi<R, W> for BatchP {
    fn run<F>(&self, _f: F)
    where
        F: FnMut(USize, USize),
    {
    }
}

impl<R: AccessSet, W: AccessSet> ReduceApi<R, W> for ReduceP {
    fn run<A, F>(&self, init: A, _f: F) -> A
    where
        A: 'static,
        F: FnMut(A, USize) -> A,
    {
        init
    }
}

// A bespoke provider-bundle newtype. Seven fields, one per provider.
struct Bundle {
    reader: ReaderP,
    writer: WriterP,
    resources: ResourceP,
    firer: FirerP,
    each: EachP,
    batch: BatchP,
    reduce: ReduceP,
}

impl<R: AccessSet> HasColumnReader<R> for Bundle {
    type Provider = ReaderP;
    fn reader(&self) -> &ReaderP {
        &self.reader
    }
}

impl<W: AccessSet> HasColumnWriter<W> for Bundle {
    type Provider = WriterP;
    fn writer(&self) -> &WriterP {
        &self.writer
    }
}

impl<R: AccessSet> HasResourceProvider<R> for Bundle {
    type Provider = ResourceP;
    fn resources(&self) -> &ResourceP {
        &self.resources
    }
}

impl<W: AccessSet> HasVirtualFirer<W> for Bundle {
    type Provider = FirerP;
    fn virtuals(&self) -> &FirerP {
        &self.firer
    }
}

impl<R: AccessSet, W: AccessSet> HasEach<R, W> for Bundle {
    type Provider = EachP;
    fn each(&self) -> &EachP {
        &self.each
    }
}

impl<R: AccessSet, W: AccessSet> HasBatch<R, W> for Bundle {
    type Provider = BatchP;
    fn batch(&self) -> &BatchP {
        &self.batch
    }
}

impl<R: AccessSet, W: AccessSet> HasReduce<R, W> for Bundle {
    type Provider = ReduceP;
    fn reduce(&self) -> &ReduceP {
        &self.reduce
    }
}

#[test]
fn accessors_dispatch_through_bundle() {
    let b = Bundle {
        reader: ReaderP,
        writer: WriterP,
        resources: ResourceP,
        firer: FirerP,
        each: EachP,
        batch: BatchP,
        reduce: ReduceP,
    };

    // Fix the generic parameters by binding once through the
    // accessor trait. The accessor method call shape:
    let _r: &ReaderP = <Bundle as HasColumnReader<()>>::reader(&b);
    let _w: &WriterP = <Bundle as HasColumnWriter<()>>::writer(&b);
    let _rs: &ResourceP = <Bundle as HasResourceProvider<()>>::resources(&b);
    let _fi: &FirerP = <Bundle as HasVirtualFirer<()>>::virtuals(&b);
    let _ea: &EachP = <Bundle as HasEach<(), ()>>::each(&b);
    let _ba: &BatchP = <Bundle as HasBatch<(), ()>>::batch(&b);
    let _rd: &ReduceP = <Bundle as HasReduce<(), ()>>::reduce(&b);
}
