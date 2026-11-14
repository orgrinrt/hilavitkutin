//! The `Stub` provider and the `TestCtx` that hands it out, the context a unit
//! names when the test only needs `build` to prove its access sets, and never
//! runs its body.

use arvo::USize;
use hilavitkutin_api::{
    AccessSet,
    BatchApi,
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
    ReduceApi,
    ResolveColumnRead,
    ResolveColumnWrite,
    ResolveResource,
    ResolveVirtualFire,
    Resource,
    ResourceProviderApi,
    Virtual,
    VirtualFirerApi,
};

pub struct Stub;

impl<R: AccessSet> ColumnReaderApi<R> for Stub {
    unsafe fn read<T: ColumnValue, I>(&self, _i: USize) -> T
    where
        R: Holds<Column<T>>,
        Self: ResolveColumnRead<T, I>,
    {
        unimplemented!()
    }
}

impl<T: ColumnValue, I> ResolveColumnRead<T, I> for Stub {
    unsafe fn resolve_read(&self, _i: USize) -> T {
        unimplemented!()
    }
}

impl<W: AccessSet> ColumnWriterApi<W> for Stub {}

impl<T: ColumnValue, I> ResolveColumnWrite<T, I> for Stub {
    unsafe fn resolve_write(&self, _i: USize, _v: T) {}

    unsafe fn resolve_held(&self, _i: USize) -> T {
        unimplemented!()
    }
}

impl<R: AccessSet> ResourceProviderApi<R> for Stub {
    fn resource<T: 'static, I>(&self) -> &T
    where
        R: Holds<Resource<T>>,
        Self: ResolveResource<T, I>,
    {
        unimplemented!()
    }
}

impl<T: 'static, I> ResolveResource<T, I> for Stub {
    fn resolve_resource(&self) -> &T {
        unimplemented!()
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

impl<V, I> ResolveVirtualFire<V, I> for Stub {
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

pub struct TestCtx {
    p: Stub,
}

impl<R: AccessSet> HasColumnReader<R> for TestCtx {
    type Provider = Stub;

    fn reader(&self) -> &Stub {
        &self.p
    }
}

impl<W: AccessSet> HasColumnWriter<W> for TestCtx {
    type Provider = Stub;

    fn writer(&self) -> &Stub {
        &self.p
    }
}

impl<R: AccessSet> HasResourceProvider<R> for TestCtx {
    type Provider = Stub;

    fn resources(&self) -> &Stub {
        &self.p
    }
}

impl<W: AccessSet> HasVirtualFirer<W> for TestCtx {
    type Provider = Stub;

    fn virtuals(&self) -> &Stub {
        &self.p
    }
}

impl<R: AccessSet, W: AccessSet> HasEach<R, W> for TestCtx {
    type Provider = Stub;

    fn each(&self) -> &Stub {
        &self.p
    }
}

impl<R: AccessSet, W: AccessSet> HasBatch<R, W> for TestCtx {
    type Provider = Stub;

    fn batch(&self) -> &Stub {
        &self.p
    }
}

impl<R: AccessSet, W: AccessSet> HasReduce<R, W> for TestCtx {
    type Provider = Stub;

    fn reduce(&self) -> &Stub {
        &self.p
    }
}
