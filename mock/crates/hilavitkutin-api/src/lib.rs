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
#![feature(associated_type_defaults)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(impl_trait_in_assoc_type)]
#![feature(marker_trait_attr)]
#![feature(min_specialization)]
#![feature(specialization)]
#![allow(incomplete_features)]

mod sealed {
    /// Crate-private sealing supertrait. Consumers cannot impl traits
    /// that use it as a supertrait.
    pub(crate) trait Sealed {} // lint:allow(undocumented_type) reason: crate-private sealing supertrait; semantics live on mod + parent trait docs; tracked: #72
}

pub mod access;
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
pub mod run_cfg;
pub mod sink;
pub mod store;
pub mod work_unit;

pub use access::{AccessSet, Concat, Cons, Contains, ContainsAll, Empty};
pub use builder::Depth;
pub use capability::{BoundedPush, BulkPush, Capacity, Full, Len, Push};
pub use codec::{DecodeError, Decoder, DecoderExt, Encoder, EncoderExt};
pub use column_value::ColumnValue;
pub use context::{
    BatchApi, ColumnReaderApi, ColumnWriterApi, EachApi, HasBatch, HasColumnReader,
    HasColumnWriter, HasEach, HasReduce, HasResourceProvider, HasVirtualFirer, ReduceApi,
    ResourceProviderApi, VirtualFirerApi,
};
pub use hint::{
    Adaptive, Atomic, Critical, Deferred, Divisibility, DivisibilityValue, Immediate, Important,
    Interruptible, Normal, Opportunistic, Optional, Relaxed, SchedulingHint, Significance,
    SignificanceValue, Steady, Urgency, UrgencyValue,
};
pub use id::StoreId;
pub use platform::{
    ClockApi, HasClock, HasMemoryProvider, HasThreadPool, MemoryProviderApi, Nanos, ThreadPoolApi,
};
pub use builder_input::{
    BuilderInput, Dispatch, ExtensionSurface, PlatformDispatch, StoreDispatch, UnitDispatch,
};
pub use ceiling_div::CeilingDiv;
pub use dispatch_codegen::{
    CoreProgram, DispatchCodegen, FiberShape, LockFreeDispatch, PhaseEntry, RecordRange,
    Scheduled, SyncRole,
};
pub use run_cfg::{
    AnomalyFired, DefaultRunCfg, HasRecordCount, PlanAffecting, RunCfg, RunCfgDispatch,
};
pub use sink::{ByteEmitter, Collector, CountingSink, DiagnosticSink, NullSink, TeeSink};
pub use store::{Column, Field, Map, Replaceable, Resource, Seq, StoreBundle, Virtual};
pub use work_unit::{Always, On, WorkUnit, WorkUnitBundle};
