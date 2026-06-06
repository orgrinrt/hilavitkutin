//! hilavitkutin-api: consumer-facing contracts.
//!
//! Traits, marker types, and platform contracts that downstream
//! pipelines build WorkUnits against. The engine crate
//! (`hilavitkutin`) consumes the same surface.
//!
//! `#![no_std]`, no alloc, no dyn, no TypeId. Boundary index/count
//! types use arvo newtypes.

#![no_std]
#![recursion_limit = "512"]
#![deny(unsafe_op_in_unsafe_fn)]
#![feature(adt_const_params)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(impl_trait_in_assoc_type)]
#![feature(marker_trait_attr)]
#![allow(incomplete_features)]

mod sealed {
    /// Crate-private sealing supertrait. Consumers cannot impl traits
    /// that use it as a supertrait.
    pub(crate) trait Sealed {} // lint:allow(undocumented_type) reason: crate-private sealing supertrait; semantics live on mod + parent trait docs; tracked: #72
}

pub mod access;
pub mod adapt;
pub mod builder;
pub mod builder_input;
pub mod capability;
pub mod ceiling_div;
pub mod codec;
pub mod column_value;
pub mod context;
pub mod dispatch_codegen;
pub mod hint;
pub mod id;
pub mod macros;
pub mod platform;
pub mod prelude;
pub mod record_op;
pub mod run_cfg;
pub mod sink;
pub mod storage;
pub mod store;
pub mod store_values;
pub mod work_unit;
pub mod work_unit_values;

pub use access::{AccessSet, Concat, Cons, Contains, ContainsAll, Empty};
pub use adapt::{
    AdaptAxis, AdaptAxisDispatch, CacheResidencyAxis, ChangeClassAxis, CoreIdleTimeAxis,
    FiberEmaAxis, MemoryWatermarkAxis, PassDurationAxis, PhaseEmaAxis, PredictiveParkingAxis,
    ThroughputAxis,
};
pub use builder::Depth;
pub use capability::{BoundedPush, BulkPush, Capacity, Full, Len, Push};
pub use codec::{DecodeError, Decoder, DecoderExt, Encoder, EncoderExt};
pub use column_value::ColumnValue;
pub use record_op::RecordOp;
pub use context::{
    BatchApi, ColumnReaderApi, ColumnWriterApi, EachApi, HasBatch, HasColumnReader,
    HasColumnWriter, HasEach, HasReduce, HasResourceProvider, HasVirtualFirer, ReduceApi,
    ResolveColumnRead, ResolveColumnWrite, ResolveResource, ResourceProviderApi, VirtualFirerApi,
};
pub use hint::{
    Adaptive, Atomic, Critical, Deferred, Divisibility, DivisibilityValue, Immediate, Important,
    Interruptible, Normal, Opportunistic, Optional, Relaxed, SchedulingHint, Significance,
    SignificanceValue, Steady, Urgency, UrgencyValue,
};
pub use id::StoreId;
pub use platform::{
    ClockApi, Executor, ExecutorError, HasClock, HasMemoryProvider, HasThreadPool,
    MemoryProviderApi, Nanos, PoolFrame, ThreadPoolApi, WakeStrategy,
};
pub use builder_input::{
    BuilderInput, Dispatch, ExtensionSurface, PlatformDispatch, StoreDispatch, UnitDispatch,
};
pub use ceiling_div::CeilingDiv;
pub use dispatch_codegen::{
    CoreProgram, DispatchCodegen, FiberId, FiberShape, LockFreeDispatch, PhaseEntry, PhaseId,
    RecordRange, Scheduled, SyncRole, TrunkId, UnitId,
};
pub use run_cfg::{
    AnomalyFired, DefaultRunCfg, HasRecordCount, PassStart, PlanAffecting, PlanStage, RunCfg,
    RunCfgDispatch, ScheduleEnd, ScheduleReady,
};
pub use sink::{ByteEmitter, Collector, CountingSink, DiagnosticSink, NullSink, TeeSink};
pub use storage::{ColumnStorage, Decompose};
pub use store::{
    Column, Field, Map, Replaceable, Resource, Seq, StagedResource, StoreBundle, Virtual,
};
pub use store_values::{
    Place, PlatformKind, RouterKind, StoreKind, StoreValues, Sv, SvEmpty, UnitKind, WorkUnitKind,
};
pub use work_unit::{Always, On, WorkUnit, WorkUnitBundle};
pub use work_unit_values::{WuCons, WuNil};
