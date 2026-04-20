//! hilavitkutin-api — consumer-facing contracts.
//!
//! Traits, marker types, and platform contracts that downstream
//! pipelines build WorkUnits against. The engine crate
//! (`hilavitkutin`) consumes the same surface.
//!
//! `#![no_std]`, no alloc, no dyn, no TypeId. Boundary index/count
//! types use arvo newtypes.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(specialization)]
#![feature(marker_trait_attr)]
#![allow(incomplete_features)]

mod sealed {
    /// Crate-private sealing supertrait. Consumers cannot impl traits
    /// that use it as a supertrait.
    pub(crate) trait Sealed {} // lint:allow(undocumented_type)
}

pub mod access;
pub mod capability;
pub mod codec;
pub mod column_value;
pub mod context;
pub mod hint;
pub mod id;
pub mod platform;
pub mod sink;
pub mod store;
pub mod work_unit;

pub use access::{AccessSet, Contains};
pub use capability::{BoundedPush, BulkPush, Capacity, Full, Len, Push};
pub use codec::{DecodeError, Decoder, DecoderExt, Encoder, EncoderExt};
pub use column_value::ColumnValue;
pub use context::{
    BatchApi, ColumnReaderApi, ColumnWriterApi, EachApi, HasBatch, HasColumnReader,
    HasColumnWriter, HasEach, HasReduce, HasResourceProvider, HasVirtualFirer, ReduceApi,
    ResourceProviderApi, VirtualFirerApi,
};
pub use hint::{
    Adaptive, Atomic, Critical, Deferred, DivisibilityValue, Immediate, Important,
    Interruptible, Normal, Opportunistic, Optional, Relaxed, SchedulingHint, SignificanceValue,
    Steady, UrgencyValue,
};
pub use id::{AccessMask, StoreId};
pub use platform::{
    ClockApi, HasClock, HasMemoryProvider, HasThreadPool, MemoryProviderApi, ThreadPoolApi,
};
pub use sink::{ByteEmitter, Collector, CountingSink, DiagnosticSink, NullSink, TeeSink};
pub use store::{Column, Field, Map, Resource, Seq, Virtual};
pub use work_unit::{Always, On, WorkUnit};
