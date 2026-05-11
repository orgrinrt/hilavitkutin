//! `DispatchCodegen<Cfg>` trait + TAIT `CoreDispatch` + sealed
//! `FiberShape` marker family + `LockFreeDispatch` / `Scheduled`
//! marker traits + `CoreProgram` / `PhaseEntry` / `RecordRange` /
//! `SyncRole` data shapes.
//!
//! Topic 4 axes A, D, G. Topic 3 axis F + Topic 10 consolidation.
//!
//! `DispatchCodegen` is the trait the engine implements (single
//! shipped impl `StandardCodegen` in `hilavitkutin::dispatch::standard`)
//! to translate `(ExecutionPlan, CoreProgram, Cfg::Units)` into a
//! monomorphised per-core dispatch closure. The trait is sealed
//! (Topic 3 S2) so only substrate-controlled codegen variants
//! can ship. The `CoreDispatch` TAIT keeps the closure type opaque
//! to consumers.
//!
//! `FiberShape` is the sealed marker family per-shape codegen
//! emits at build time. Each unique fiber-shape (WU sequence)
//! across the plan gets one impl; LLVM monomorphises per shape.
//! Consumers cannot impl FiberShape; impls are codegen artefacts.
//!
//! `LockFreeDispatch` is the sealed marker codegen output carries
//! to prove "this dispatch path uses zero CAS / zero RMW in the
//! inner loop". `Scheduled` is the trait-alias unified API
//! constraint per audit-2 m4 that bundles `LockFreeDispatch` with
//! the other constraints `Scheduler::run` requires.

use core::marker::PhantomData;

use arvo::strategy::Identity;
use arvo::traits::FromConstant;
use arvo::{Uint, USize};

use crate::id::StoreId;

mod sealed {
    pub trait Sealed {}
}

// Engine-id layout assertions.
//
// Every Debug impl below uses `transmute_copy` over the repr(transparent)
// chain `<Id> -> Uint<N> -> Bits<N, Warm, Unsigned> -> <container>`.
// `transmute_copy::<Src, Dst>` requires `size_of::<Dst>() <= size_of::<Src>()`,
// so the Debug impls MUST read at the container's full byte width. Asserting
// the actual sizes here makes the invariant self-enforcing: if arvo's Warm
// container dispatch table ever moves these widths, the next compile fails
// here with a load-bearing diagnostic instead of producing a silently lossy
// Debug projection (or, worse, an off-by-N byte read).
//
// Probed sizes (2026-05-11): UnitId = 4, FiberId = 2, PhaseId = 2, TrunkId = 2.
// Each Debug impl below reads at exactly the asserted size.
const _: () = assert!( // lint:allow(no-bare-numeric) reason: const-context layout assertion; rust grammar requires raw usize literals here; tracked: #428
    core::mem::size_of::<PhaseId>() == 2,
    "PhaseId layout drift: Debug impl in this file reads 2 bytes; update both if arvo changes the Warm container for Uint<5>.",
);
const _: () = assert!( // lint:allow(no-bare-numeric) reason: const-context layout assertion; rust grammar requires raw usize literals here; tracked: #428
    core::mem::size_of::<TrunkId>() == 2,
    "TrunkId layout drift: Debug impl in this file reads 2 bytes; update both if arvo changes the Warm container for Uint<6>.",
);
const _: () = assert!( // lint:allow(no-bare-numeric) reason: const-context layout assertion; rust grammar requires raw usize literals here; tracked: #428
    core::mem::size_of::<FiberId>() == 2,
    "FiberId layout drift: Debug impl in this file reads 2 bytes; update both if arvo changes the Warm container for Uint<7>.",
);
const _: () = assert!( // lint:allow(no-bare-numeric) reason: const-context layout assertion; rust grammar requires raw usize literals here; tracked: #428
    core::mem::size_of::<UnitId>() == 4,
    "UnitId layout drift: Debug impl in this file reads 4 bytes; update both if arvo changes the Warm container for Uint<16>.",
);

/// `PhaseId` newtype carrying the plan-stage-assigned phase index.
/// Topic 3 axis B.
///
/// Bit budget 5: phases rarely exceed 20 in any plan. arvo picks
/// the underlying container per strategy.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct PhaseId(pub Uint<5>);

impl core::fmt::Debug for PhaseId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // PhaseId is repr(transparent) over Uint<5> over Bits<5, Warm, Unsigned).
        // Warm's container dispatch for 5-bit widths picks u16 (verified via size
        // probe; the size_of assert above is the durable check). Read at the
        // container's full width to satisfy `transmute_copy::<Src, Dst>`'s
        // `size_of::<Dst>() <= size_of::<Src>()` precondition. Pending the arvo
        // Debug substrate addition, this is the dogfooded projection door.
        let raw: u16 = unsafe { core::mem::transmute_copy(self) }; // lint:allow(no-bare-numeric) reason: arvo Debug substrate gap; tracked: #428
        write!(f, "PhaseId({})", raw)
    }
}

impl PhaseId {
    /// Zero-valued default.
    pub const ZERO: Self = Self(<Uint<5> as Identity>::ZERO);

    /// Typed-const constructor for non-zero indices.
    pub const fn from_constant<const C: USize>() -> Self {
        Self(<Uint<5> as FromConstant>::from_constant::<C>())
    }
}

impl Default for PhaseId {
    fn default() -> Self {
        Self::ZERO
    }
}

/// `TrunkId` newtype carrying the plan-stage-assigned trunk index.
/// Topic 3 axis B.
///
/// Bit budget 6: trunks per phase are fewer than fibers; 64 is
/// generous for any realistic plan.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct TrunkId(pub Uint<6>);

impl core::fmt::Debug for TrunkId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // TrunkId -> Uint<6> -> Bits<6, Warm, Unsigned) -> u16 container.
        // See PhaseId Debug for the full reasoning + the size_of assert above.
        let raw: u16 = unsafe { core::mem::transmute_copy(self) }; // lint:allow(no-bare-numeric) reason: arvo Debug substrate gap; tracked: #428
        write!(f, "TrunkId({})", raw)
    }
}

impl TrunkId {
    /// Zero-valued default.
    pub const ZERO: Self = Self(<Uint<6> as Identity>::ZERO);

    /// Typed-const constructor for non-zero indices.
    pub const fn from_constant<const C: USize>() -> Self {
        Self(<Uint<6> as FromConstant>::from_constant::<C>())
    }
}

impl Default for TrunkId {
    fn default() -> Self {
        Self::ZERO
    }
}

/// `FiberId` newtype carrying the plan-stage-assigned fiber index.
/// Distinct from `StoreId` / `UnitId` for type-safety at access
/// sites. Topic 3 axis B.
///
/// Bit budget 7: even a 64-core plan rarely exceeds 100 fibers.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct FiberId(pub Uint<7>);

impl core::fmt::Debug for FiberId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // FiberId -> Uint<7> -> Bits<7, Warm, Unsigned) -> u16 container.
        // See PhaseId Debug for the full reasoning + the size_of assert above.
        let raw: u16 = unsafe { core::mem::transmute_copy(self) }; // lint:allow(no-bare-numeric) reason: arvo Debug substrate gap; tracked: #428
        write!(f, "FiberId({})", raw)
    }
}

impl FiberId {
    /// Zero-valued default.
    pub const ZERO: Self = Self(<Uint<7> as Identity>::ZERO);

    /// Typed-const constructor for non-zero indices.
    pub const fn from_constant<const C: USize>() -> Self {
        Self(<Uint<7> as FromConstant>::from_constant::<C>())
    }
}

impl Default for FiberId {
    fn default() -> Self {
        Self::ZERO
    }
}

/// `UnitId` newtype carrying the plan-stage-assigned WorkUnit index.
///
/// Named `UnitId` (not `NodeId`) to keep engine vocabulary distinct
/// from arvo graph-substrate vocabulary. Topic 3 axis B.
///
/// Bit budget 16: WU count grows into the thousands across larger
/// consumers (viola plugin sets, loimu pipelines); 65K is the cap.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct UnitId(pub Uint<16>);

impl core::fmt::Debug for UnitId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // UnitId -> Uint<16> -> Bits<16, Warm, Unsigned) -> u32 container
        // (Warm picks a 32-bit container for 16-bit widths; size probe-verified).
        // See PhaseId Debug for the full reasoning + the size_of assert above.
        let raw: u32 = unsafe { core::mem::transmute_copy(self) }; // lint:allow(no-bare-numeric) reason: arvo Debug substrate gap; tracked: #428
        write!(f, "UnitId({})", raw)
    }
}

impl UnitId {
    /// Zero-valued default.
    pub const ZERO: Self = Self(<Uint<16> as Identity>::ZERO);

    /// Typed-const constructor for non-zero indices.
    pub const fn from_constant<const C: USize>() -> Self {
        Self(<Uint<16> as FromConstant>::from_constant::<C>())
    }
}

impl Default for UnitId {
    fn default() -> Self {
        Self::ZERO
    }
}

/// Per-core projection of `ExecutionPlan`. Topic 3 axis F +
/// Topic 10 consolidation. Each core's worker walks this at
/// runtime to issue phase-sync points + dispatch morsels.
///
/// Plan-stage produces one `CoreProgram` per physical core (laid
/// out in the plan-stage scratch arena). Codegen (`Pass 3`) emits
/// a per-core closure parameterised by const-known fields of this
/// program.
pub struct CoreProgram<
    const MAX_PHASES_PER_CORE: usize,
    const MAX_TRUNKS_PER_CORE: usize,
    const MAX_FIBERS_PER_CORE: usize,
> {
    /// Phases this core participates in.
    pub phases: [PhaseEntry; MAX_PHASES_PER_CORE],
    pub phase_count: USize,

    /// Trunks this core owns.
    pub trunks: [TrunkId; MAX_TRUNKS_PER_CORE],
    pub trunk_count: USize,

    /// Per-fiber record range for this core. Full / Head / Tail.
    pub fiber_ranges: [(FiberId, RecordRange); MAX_FIBERS_PER_CORE],
    pub range_count: USize,

    /// Estimated icache footprint of the monomorphised per-core
    /// function in bytes. Topic 3 M2 invariant; plan stage uses
    /// this to fall back ScheduleMega → TrunkMega → IndirectPerFiber
    /// when budget exceeds platform L1 icache.
    pub estimated_icache_bytes: USize,

    /// Index into `PoolFrame.progress_slots[]` for this core's
    /// progress counter base. Topic 4 axis E + arena indirection.
    pub progress_slot_idx: USize,

    /// Offset of this core's bit within `PoolFrame.phase_arrived`.
    /// Topic 6 axis I.
    pub phase_arrived_offset: USize,
}

/// Per-phase entry on a `CoreProgram`. Topic 3 axis F.
pub struct PhaseEntry {
    pub phase: PhaseId,
    /// What this core does at the phase barrier: wait, signal, or
    /// both. Topic 3 line 738.
    pub sync_role: SyncRole,
}

/// Phase-sync role for a core at a given phase. Topic 3 axis F.
#[non_exhaustive]
pub enum SyncRole {
    /// This core waits for the producer counter.
    WaitOnly,
    /// This core only produces; downstream waits.
    SignalOnly,
    /// Midstream phase: waits AND signals.
    WaitAndSignal,
}

/// Per-core record range. Topic 3 axis F + Topic 4 axis D head+tail
/// convergence. Exactly three variants; no `Custom` fallback per the
/// Topic 3 axis F lock (consumer needing a different range shape
/// triggers a deprecation-replacement round on this enum).
#[non_exhaustive]
pub enum RecordRange {
    /// Full range `0..record_count`.
    Full,
    /// Head half: `0..mid` (head+tail convergence, head thread).
    Head { mid_slot: USize },
    /// Tail half: `mid..record_count` (head+tail convergence, tail thread).
    Tail { mid_slot: USize },
}

/// `DispatchCodegen<Cfg>` produces a monomorphised per-core dispatch
/// closure from an `ExecutionPlan` + `CoreProgram` projection.
/// Sealed; single shipped impl `StandardCodegen`. Topic 4 axis A.
///
/// The `CoreDispatch` TAIT keeps the closure type opaque (consumer
/// cannot name it); `#[inline]` on `build` forces consumer-site
/// re-codegen per audit-2 m2.
///
/// **Pre-stub:** the actual associated `type CoreDispatch = impl
/// Fn(...)` lowering lands in the engine impl (Pass 3). This trait
/// declaration just commits the surface.
pub trait DispatchCodegen<Cfg>: sealed::Sealed {
    /// The monomorphised per-core dispatch closure type. Topic 4
    /// axis A Rider 1. Engine impls set this via TAIT
    /// `type CoreDispatch = impl Fn(...) -> notko::Outcome<(), Cfg::Err>;`.
    type CoreDispatch;
}

/// Sealed marker: codegen output uses zero CAS / zero RMW in the
/// inner loop. Topic 4 axis G + Topic 5 audit-2 m4. Blanket-impl on
/// the engine's `StandardCodegen` output type.
pub trait LockFreeDispatch: sealed::Sealed {}

/// Unified API constraint per audit-2 m4: `Scheduled` is the bound
/// `Scheduler::run` requires. Bundles `LockFreeDispatch` with future
/// extension; consumers that name dispatch types in their API surface
/// use `Scheduled` to keep the bound stable.
pub trait Scheduled: LockFreeDispatch {}

/// Sealed marker family: per-fiber-shape monomorphisation key.
/// Topic 4 axis D + Topic 3 amendment + S7. Codegen emits one impl
/// per unique fiber shape encountered in the plan; consumers cannot
/// impl this. `WuTuple` is the typed tuple of WU types in fiber-
/// execution order; `SHAPE_ID` is the dedup hash.
pub trait FiberShape: sealed::Sealed {
    /// Type-level tuple of WU types in fiber-execution order.
    type WuTuple;

    /// Stable identity for de-duplication. Codegen hashes the
    /// `WuTuple` type-id sequence.
    const SHAPE_ID: USize;
}

/// Marker handle for a registered store. Distinct from `StoreId`
/// (a runtime ID); this is the type-level evidence.
pub struct StoreMarker<T>(PhantomData<T>);
