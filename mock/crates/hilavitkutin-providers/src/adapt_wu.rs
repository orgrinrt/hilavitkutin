//! `AdaptWu`: substrate-default adapt meta-WorkUnit.
//!
//! Observes `Virtual<ScheduleEnd>`, reads the nine per-axis metrics
//! Resources for the pass-end samples, sets the per-axis anomaly bools
//! on the matching Resource snapshot, and fires `Virtual<AnomalyFired>`
//! when any axis crosses its threshold. The fired Virtual gates
//! observer WorkUnits that react to anomalies in the following pass.
//!
//! Consumers register the substrate default via
//! `.with(AdaptWu::default())`; consumers wanting custom adapt logic
//! ship their own `WorkUnit<On<ScheduleEnd>>` with the same access
//! shape (or a subset, when only a few axes are used).
//!
//! The execute body is empty. The Read/Write/Hint declaration in this
//! file is the structural contract that the engine routes against.

use core::marker::PhantomData;

use arvo::USize;
use hilavitkutin_api::access::AccessSet;
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::column_value::ColumnValue;
use hilavitkutin_api::context::{
    BatchApi,
    ColumnReaderApi,
    ColumnWriterApi,
    EachApi,
    HasBatch,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
    HasReduce,
    HasResourceProvider,
    HasVirtualFirer,
    ReduceApi,
    ResolveColumnRead,
    ResolveColumnWrite,
    ResolveResource,
    ResolveVirtualFire,
    ResourceProviderApi,
    VirtualFirerApi,
};
use hilavitkutin_api::hint::{Adaptive, Important, Steady};
use hilavitkutin_api::store::{Column, Resource, Virtual};
use hilavitkutin_api::work_unit::{On, WorkUnit};
use hilavitkutin_api::{AnomalyFired, Holds, ScheduleEnd, read, write};

use crate::metrics::{
    CacheResidencyMetrics,
    ChangeClassMetrics,
    CoreIdleTimeMetrics,
    FiberEmaMetrics,
    MemoryWatermarkMetrics,
    PassDurationMetrics,
    PhaseEmaMetrics,
    PredictiveParkingMetrics,
    ThroughputMetrics,
};

/// Substrate-default adapt meta-WorkUnit.
///
/// Observes `Virtual<ScheduleEnd>` and updates per-axis metrics
/// snapshots at every pass boundary. Anomaly bools live on the
/// individual metrics Resources; the single `Virtual<AnomalyFired>`
/// gates observer WUs.
pub struct AdaptWu;

impl Default for AdaptWu {
    fn default() -> Self {
        Self
    }
}

impl BuilderInput for AdaptWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<On<ScheduleEnd>> for AdaptWu {
    type Ctx<'frame> = AdaptCtxUnimplementedStub<'frame>;
    type Hint = (Steady, Adaptive, Important);
    type Read = read![
        Resource<PassDurationMetrics>,
        Resource<PhaseEmaMetrics>,
        Resource<FiberEmaMetrics>,
        Resource<ChangeClassMetrics>,
        Resource<CacheResidencyMetrics>,
        Resource<ThroughputMetrics>,
        Resource<PredictiveParkingMetrics>,
        Resource<MemoryWatermarkMetrics>,
        Resource<CoreIdleTimeMetrics>,
    ];
    type Write = write![
        Resource<PassDurationMetrics>,
        Resource<PhaseEmaMetrics>,
        Resource<FiberEmaMetrics>,
        Resource<ChangeClassMetrics>,
        Resource<CacheResidencyMetrics>,
        Resource<ThroughputMetrics>,
        Resource<PredictiveParkingMetrics>,
        Resource<MemoryWatermarkMetrics>,
        Resource<CoreIdleTimeMetrics>,
        Virtual<AnomalyFired>,
    ];

    fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {
        // Empty today. The body is to walk the
        // nine metrics Resources, threshold-compare each `last_sample`
        // against per-axis thresholds, set the anomaly bool on every
        // tripped axis, and fire `Virtual<AnomalyFired>` once if any
        // tripped.
    }
}

// ---------------------------------------------------------------------
// Placeholder Ctx + Stub provider.
//
// The WorkUnit trait requires Ctx<'frame> to satisfy seven HasX
// accessor bounds. Until the engine's real Ctx is wired into
// meta-WU dispatch, the placeholder + Stub form a trivially-satisfied
// type that lets the AdaptWu impl typecheck. The engine-generated Ctx
// is to replace this; consumer code does not see the placeholder.
// ---------------------------------------------------------------------

/// Placeholder Ctx for `AdaptWu`, standing in until the engine's Ctx is wired.
pub struct AdaptCtxUnimplementedStub<'frame> {
    _phantom: PhantomData<&'frame ()>,
    stub:     Stub,
}

impl Default for AdaptCtxUnimplementedStub<'_> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
            stub:     Stub,
        }
    }
}

/// Stub provider satisfying the seven `*Api` traits with no-op bodies.
pub struct Stub;

impl<R: AccessSet> ColumnReaderApi<R> for Stub {
    unsafe fn read<T: ColumnValue, I>(&self, _i: USize) -> T
    where
        R: Holds<Column<T>>,
        Self: ResolveColumnRead<T, I>,
    {
        unimplemented!("AdaptCtxUnimplementedStub is pre-Pass-6 stub; engine Ctx supersedes it")
    }
}

// `write` is the api's own, over the `ResolveColumnWrite` bridge below.
impl<W: AccessSet> ColumnWriterApi<W> for Stub {}

impl<R: AccessSet> ResourceProviderApi<R> for Stub {
    fn resource<T: 'static, I>(&self) -> &T
    where
        R: Holds<Resource<T>>,
        Self: ResolveResource<T, I>,
    {
        unimplemented!("AdaptCtxUnimplementedStub is pre-Pass-6 stub; engine Ctx supersedes it")
    }
}

// Bridge impls: the stub never resolves (it is a placeholder), so the
// keyed lookups are blanket-unimplemented. They exist only to satisfy the
// `Self: Resolve*<T, I>` accessor bounds.
impl<T: 'static, I> ResolveResource<T, I> for Stub {
    fn resolve_resource(&self) -> &T {
        unimplemented!("AdaptCtxUnimplementedStub is pre-Pass-6 stub; engine Ctx supersedes it")
    }
}

impl<T: ColumnValue, I> ResolveColumnRead<T, I> for Stub {
    unsafe fn resolve_read(&self, _i: USize) -> T {
        unimplemented!("AdaptCtxUnimplementedStub is pre-Pass-6 stub; engine Ctx supersedes it")
    }
}

impl<T: ColumnValue, I> ResolveColumnWrite<T, I> for Stub {
    unsafe fn resolve_write(&self, _i: USize, _v: T) {
        unimplemented!("AdaptCtxUnimplementedStub is pre-Pass-6 stub; engine Ctx supersedes it")
    }

    unsafe fn resolve_held(&self, _i: USize) -> T {
        unimplemented!("AdaptCtxUnimplementedStub is pre-Pass-6 stub; engine Ctx supersedes it")
    }
}

impl<W: AccessSet> VirtualFirerApi<W> for Stub {
    fn fire<V: 'static, I>(&self)
    where
        W: Holds<Virtual<V>>,
        Self: ResolveVirtualFire<V, I>,
    {
        unimplemented!("AdaptCtxUnimplementedStub is pre-Pass-6 stub; engine Ctx supersedes it")
    }
}

impl<V, I> ResolveVirtualFire<V, I> for Stub {
    fn resolve_fire(&self) {
        unimplemented!("AdaptCtxUnimplementedStub is pre-Pass-6 stub; engine Ctx supersedes it")
    }
}

impl<R: AccessSet, W: AccessSet> EachApi<R, W> for Stub {
    fn run<F>(&self, _f: F)
    where
        F: FnMut(USize),
    {
        unimplemented!("AdaptCtxUnimplementedStub is pre-Pass-6 stub; engine Ctx supersedes it")
    }
}

impl<R: AccessSet, W: AccessSet> BatchApi<R, W> for Stub {
    fn run<F>(&self, _f: F)
    where
        F: FnMut(USize, USize),
    {
        unimplemented!("AdaptCtxUnimplementedStub is pre-Pass-6 stub; engine Ctx supersedes it")
    }
}

impl<R: AccessSet, W: AccessSet> ReduceApi<R, W> for Stub {
    fn run<A, F>(&self, init: A, _f: F) -> A
    where
        A: 'static,
        F: FnMut(A, USize) -> A,
    {
        let _ = init;
        unimplemented!("AdaptCtxUnimplementedStub is pre-Pass-6 stub; engine Ctx supersedes it")
    }
}

impl<'frame, R: AccessSet> HasColumnReader<R> for AdaptCtxUnimplementedStub<'frame> {
    type Provider = Stub;

    fn reader(&self) -> &Stub {
        &self.stub
    }
}

impl<'frame, W: AccessSet> HasColumnWriter<W> for AdaptCtxUnimplementedStub<'frame> {
    type Provider = Stub;

    fn writer(&self) -> &Stub {
        &self.stub
    }
}

impl<'frame, R: AccessSet> HasResourceProvider<R> for AdaptCtxUnimplementedStub<'frame> {
    type Provider = Stub;

    fn resources(&self) -> &Stub {
        &self.stub
    }
}

impl<'frame, W: AccessSet> HasVirtualFirer<W> for AdaptCtxUnimplementedStub<'frame> {
    type Provider = Stub;

    fn virtuals(&self) -> &Stub {
        &self.stub
    }
}

impl<'frame, R: AccessSet, W: AccessSet> HasEach<R, W> for AdaptCtxUnimplementedStub<'frame> {
    type Provider = Stub;

    fn each(&self) -> &Stub {
        &self.stub
    }
}

impl<'frame, R: AccessSet, W: AccessSet> HasBatch<R, W> for AdaptCtxUnimplementedStub<'frame> {
    type Provider = Stub;

    fn batch(&self) -> &Stub {
        &self.stub
    }
}

impl<'frame, R: AccessSet, W: AccessSet> HasReduce<R, W> for AdaptCtxUnimplementedStub<'frame> {
    type Provider = Stub;

    fn reduce(&self) -> &Stub {
        &self.stub
    }
}
