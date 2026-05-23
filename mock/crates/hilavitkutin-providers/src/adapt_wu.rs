//! `AdaptWu`: substrate-default adapt meta-WorkUnit.
//!
//! Topic 1 axis 5 + Topic 5 axes A/B/E. Observes `Virtual<ScheduleEnd>`
//! per audit-2 M7, reads the nine per-axis metrics Resources for the
//! pass-end samples, sets the per-axis anomaly bools on the matching
//! Resource snapshot, and fires `Virtual<AnomalyFired>` when any axis
//! crosses its threshold. The fired Virtual gates observer WorkUnits
//! that react to anomalies in the following pass.
//!
//! Consumers register the substrate default via
//! `.with(AdaptWu::default())`; consumers wanting custom adapt logic
//! ship their own `WorkUnit<On<ScheduleEnd>>` with the same access
//! shape (or a subset, when only a few axes are used).
//!
//! The execute body is wired through `Scheduler::run` (Pass 6 of the
//! megaround). The Read/Write/Hint declaration in this file freezes
//! the structural contract that the engine routes against; the body
//! lands when the executor begins driving meta-WUs at phase
//! boundaries.

use core::marker::PhantomData;

use arvo::USize;
use hilavitkutin_api::access::AccessSet;
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::column_value::ColumnValue;
use hilavitkutin_api::context::{
    BatchApi, ColumnReaderApi, ColumnWriterApi, EachApi, HasBatch, HasColumnReader,
    HasColumnWriter, HasEach, HasReduce, HasResourceProvider, HasVirtualFirer, ReduceApi,
    ResourceProviderApi, VirtualFirerApi,
};
use hilavitkutin_api::hint::{Adaptive, Important, Steady};
use hilavitkutin_api::store::{Column, Resource, Virtual};
use hilavitkutin_api::Contains;
use hilavitkutin_api::work_unit::{On, WorkUnit};
use hilavitkutin_api::{AnomalyFired, ScheduleEnd, read, write};

use crate::metrics::{
    CacheResidencyMetrics, ChangeClassMetrics, CoreIdleTimeMetrics, FiberEmaMetrics,
    MemoryWatermarkMetrics, PassDurationMetrics, PhaseEmaMetrics, PredictiveParkingMetrics,
    ThroughputMetrics,
};

/// Substrate-default adapt meta-WorkUnit. Topic 1 axis 5 + Topic 5.
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
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<On<ScheduleEnd>> for AdaptWu {
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
    type Hint = (Steady, Adaptive, Important);
    type Ctx<'frame> = AdaptCtxUnimplementedStub<'frame>;

    fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {
        // Body lands at Pass 6 wiring (Scheduler::run): walk the nine
        // metrics Resources, threshold-compare each `last_sample`
        // against per-axis thresholds, set the anomaly bool on every
        // tripped axis, and fire `Virtual<AnomalyFired>` once if any
        // tripped. The structural shape above freezes the contract;
        // the executor learns to route the body when meta-WU dispatch
        // arrives.
    }
}

// ---------------------------------------------------------------------
// Placeholder Ctx + Stub provider.
//
// The WorkUnit trait requires Ctx<'frame> to satisfy seven HasX
// accessor bounds. Until Pass 6 wires the engine's real Ctx into
// meta-WU dispatch, the placeholder + Stub form a trivially-satisfied
// type that lets the AdaptWu impl typecheck. Pass 6 replaces this
// with the engine-generated Ctx; consumer code does not see the
// placeholder.
// ---------------------------------------------------------------------

/// Placeholder Ctx for `AdaptWu`. Pre-Pass 6 stub.
pub struct AdaptCtxUnimplementedStub<'frame> {
    _phantom: PhantomData<&'frame ()>,
    stub: Stub,
}

impl Default for AdaptCtxUnimplementedStub<'_> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
            stub: Stub,
        }
    }
}

/// Stub provider satisfying the seven `*Api` traits with no-op bodies.
pub struct Stub;

impl<R: AccessSet> ColumnReaderApi<R> for Stub {
    unsafe fn read<T: ColumnValue>(&self, _i: USize) -> T
    where
        R: Contains<Column<T>>,
    {
        unimplemented!("AdaptCtxUnimplementedStub is pre-Pass-6 stub; engine Ctx supersedes it")
    }
}

impl<W: AccessSet> ColumnWriterApi<W> for Stub {
    unsafe fn write<T: ColumnValue>(&self, _i: USize, _v: T)
    where
        W: Contains<Column<T>>,
    {
        unimplemented!("AdaptCtxUnimplementedStub is pre-Pass-6 stub; engine Ctx supersedes it")
    }
}

impl<R: AccessSet> ResourceProviderApi<R> for Stub {
    fn resource<T: 'static>(&self) -> &T
    where
        R: Contains<Resource<T>>,
    {
        unimplemented!("AdaptCtxUnimplementedStub is pre-Pass-6 stub; engine Ctx supersedes it")
    }
}

impl<W: AccessSet> VirtualFirerApi<W> for Stub {
    fn fire<V: 'static>(&self)
    where
        W: Contains<Virtual<V>>,
    {
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
