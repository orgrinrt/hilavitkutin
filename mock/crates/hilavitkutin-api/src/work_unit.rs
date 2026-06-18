//! WorkUnit trait plus the two built-in schedule markers.
//!
//! `WorkUnit<Schedule>` is the consumer's unit of work. Composition
//! is static: the engine takes a tuple of WU types at compile time.
//! Identity is the type itself; no `NAME` const, no registry.

use core::marker::PhantomData;

use arvo::{Bool, USize};

use crate::access::AccessSet;
use crate::meta::{MetaVirtual, RANK_CONSUMER};
use crate::context::{
    HasBatch, HasColumnReader, HasColumnWriter, HasEach, HasReduce, HasResourceProvider,
    HasVirtualFirer,
};
use crate::hint::SchedulingHint;
use crate::builder_input::BuilderInput;

/// Schedule marker: the WU runs every pass.
#[derive(Copy, Clone, Default, Debug)]
pub struct Always;

/// Schedule marker: the WU runs when virtual `V` fires.
#[derive(Copy, Clone, Default, Debug)]
pub struct On<V>(PhantomData<V>);

/// Schedule marker: the WU runs when META lifecycle virtual `V` fires.
///
/// Distinct from `On<V>` so the const lifecycle classification stays disjoint:
/// a blanket `impl<V> Lifecycle for On<V>` (consumer rank) and a meta impl over
/// the same `On<meta::V>` would conflict (E0119), and the negative bound that
/// would separate them is not expressible (full specialization is forbidden).
/// `OnMeta<V>` is the toolchain-forced surface form of the canonical
/// `On<meta::V>`; behaviour is unchanged. See the self-hosting meta pipeline in
/// the engine DESIGN and sketch
/// `mock/research/sketches/202606082000_e4-slice2-lifecycle-classify`.
#[derive(Copy, Clone, Default, Debug)]
pub struct OnMeta<V>(PhantomData<V>);

/// A unit of work.
///
/// Declares its read/write access sets at type level, its
/// scheduling hint, and the provider-tuple shape it expects. The
/// engine composes WUs into fused per-core programs that LLVM
/// devirtualises into straight-line code.
///
/// `Schedule` picks the firing condition: `Always` runs every pass,
/// `On<V>` runs when virtual `V` fires.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a WorkUnit (or its `Schedule` does not match `{Schedule}`)",
    note = "Implement `WorkUnit` on `{Self}`. Declare `type Read`, `type Write`, `type Hint`, `type Ctx`, and `fn execute(&self, ctx: &Self::Ctx)`. The default `Schedule` is `Always`; override with `On<V>` for virtual-fired WUs. Pair the impl with `impl BuilderInput for {Self} {{ type Init = Self; type Dispatch = UnitDispatch<Self>; }}` so the type is accepted by `SchedulerBuilder::with(...)`."
)]
pub trait WorkUnit<Schedule = Always>: BuilderInput<Init = Self> + Send + Sync + 'static {
    /// Columns / resources this WU reads.
    type Read: AccessSet;
    /// Columns / virtuals this WU writes.
    type Write: AccessSet;
    /// Scheduling hint triple. Consumer provides; no default because
    /// the implementing tuple is marker-specific.
    type Hint: SchedulingHint;
    /// Provider-tuple shape this WU's body consumes.
    ///
    /// GAT-shaped per Topic 1 axis 4 + Topic 6 axis C: the `'frame`
    /// lifetime threads through from `Scheduler<'frame, ...>` so
    /// the Ctx can carry `Pin<&'frame PoolFrame>` and per-phase
    /// `ResourceSnapshot<'phase, R>` views without forcing every
    /// consumer WU declaration to also thread a lifetime.
    /// Monomorphisation resolves `HasX<...>` bounds to the concrete
    /// provider the engine wires up at plan time. Sketch:
    /// `mock/research/sketches/202605101036-poolframe-lifetime/`.
    type Ctx<'frame>: HasColumnReader<Self::Read>
        + HasColumnWriter<Self::Write>
        + HasResourceProvider<Self::Read>
        + HasVirtualFirer<Self::Write>
        + HasEach<Self::Read, Self::Write>
        + HasBatch<Self::Read, Self::Write>
        + HasReduce<Self::Read, Self::Write>;

    /// True if the WU's writes commute across record order.
    ///
    /// Enables the scheduler to emit a reduce-style pattern instead
    /// of serialising. Default `Bool::FALSE`: consumer opts in.
    const COMMUTATIVE: Bool = Bool::FALSE;

    /// Run this WU against the provided context for one morsel.
    ///
    /// The engine windows the record range into morsels and invokes
    /// `execute` once per morsel, so all per-record and accumulating work
    /// must flow through the morsel-aware Context accessors (`each` /
    /// `reduce` / `batch` for the per-record loop, `read` / `write` for
    /// columns, `append` for accumulators); those resolve record indices
    /// relative to the current morsel. A side effect placed in `execute`
    /// outside those accessors runs once per morsel, so it repeats when the
    /// record range spans more than one morsel.
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>);
}

// ---------------------------------------------------------------------
// E4 slice 1: schedule recovery for the dispatch carrier.
//
// A heterogeneous WU carrier (mixing `Always` and `On<V>` units) cannot
// recover a member's schedule from a free `WorkUnit<S>` bound (S is
// unconstrained, E0207). `Scheduled` names the schedule as an associated
// type so the engine's dispatch walk can branch on it at compile time;
// `ScheduleGate` is the marker the engine gates on (the runtime fired-flag
// read lives engine-side, since the contract crate cannot see the engine's
// bindings). Sketch: 202606081700_e4-scheduled-blanket-coherence.
// ---------------------------------------------------------------------

/// Per-schedule lifecycle rank, the const the grouping reads to make the
/// lifecycle ordinal the outer phase key.
///
/// Three disjoint impls, no overlap and no specialization: `Always` and `On<V>`
/// are consumer-rank; `OnMeta<V>` takes its meta virtual's rank. The grouping
/// folds `<<W as HasSchedule>::Sched as Lifecycle>::RANK` per unit and renumbers
/// `(rank, waist_phase)` into contiguous phase bands, so plan-stage meta units
/// land before consumers and the schedule-end epilogue lands after. See sketch
/// `mock/research/sketches/202606082200_e4-slice2-rank-phase-renumber`.
pub trait Lifecycle {
    /// The lifecycle ordinal (`meta::RANK_*`).
    const RANK: USize;
}
impl Lifecycle for Always {
    const RANK: USize = RANK_CONSUMER;
}
impl<V> Lifecycle for On<V> {
    const RANK: USize = RANK_CONSUMER;
}
impl<V: MetaVirtual> Lifecycle for OnMeta<V> {
    const RANK: USize = <V as MetaVirtual>::RANK;
}

/// Marker for the schedule kinds the engine gates dispatch on.
///
/// Pure type-level. `Always`, `On<V>`, and `OnMeta<V>` are the kinds; the
/// runtime "has `V` fired this pass" read is the engine's job (it resolves the
/// virtual's fired flag from the scheduler's own state). The `Lifecycle`
/// supertrait gives the grouping each schedule's lifecycle rank.
pub trait ScheduleGate: Lifecycle {}
impl ScheduleGate for Always {}
impl<V> ScheduleGate for On<V> {}
impl<V: MetaVirtual> ScheduleGate for OnMeta<V> {}

/// Recovers a WorkUnit's schedule as an associated type.
///
/// Blanket-impl'd for every `WorkUnit<Always>` (the default schedule), so
/// existing Always WUs gain `HasSchedule` with no extra impl. An `On<V>` WU
/// (which impls `WorkUnit<On<V>>`, not `WorkUnit<Always>`) adds an explicit
/// `impl HasSchedule for ThatWu { type Sched = On<V>; }`. The two do not
/// overlap, so the blanket and the explicit impl cohere (proven by the
/// 202606081700 sketch). The dispatch carrier bounds members
/// `W: HasSchedule + WorkUnit<<W as HasSchedule>::Sched>`.
///
/// Named `HasSchedule` (not `Scheduled`) because `dispatch_codegen` already
/// ships an unrelated `Scheduled` trait-alias for `LockFreeDispatch`.
pub trait HasSchedule {
    /// This WU's schedule: `Always` or `On<V>`.
    type Sched: ScheduleGate;
}

impl<W: WorkUnit<Always>> HasSchedule for W {
    type Sched = Always;
}

// ---------------------------------------------------------------------
// Round 4 substrate: WorkUnitBundle.
//
// Cons-list bundle of WorkUnit types with accumulated Read / Write
// access sets. AccumRead is the Concat-projected union of every WU's
// Read over the bundle; AccumWrite is the symmetric projection over
// Write. Used by Kit's 'type Units' bound; the engine reads
// Wus::AccumRead and Wus::AccumWrite at compile time.
// ---------------------------------------------------------------------

use crate::access::{Concat, Cons, Empty};

/// Cons-list bundle of WorkUnit types with accumulated Read / Write
/// access sets.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a WorkUnitBundle",
    note = "WorkUnitBundle is auto-implemented for `Empty` and for `Cons<W, T>` where `W: WorkUnit` and `T: WorkUnitBundle`. Build the bundle through the scheduler builder's `.add_unit::<W>()` calls."
)]
pub trait WorkUnitBundle {
    type AccumRead: AccessSet;
    type AccumWrite: AccessSet;
}

impl WorkUnitBundle for Empty {
    type AccumRead = Empty;
    type AccumWrite = Empty;
}

// E4 slice 1: recover each unit's schedule so a mixed `Always` / `On<V>` carrier
// accumulates. `<W as WorkUnit<<W as HasSchedule>::Sched>>::{Read,Write}` reads
// the right impl for either kind; the blanket gives every Always WU `HasSchedule`
// with zero churn.
impl<W, T> WorkUnitBundle for Cons<W, T>
where
    W: HasSchedule + WorkUnit<<W as HasSchedule>::Sched>,
    T: WorkUnitBundle,
    <W as WorkUnit<<W as HasSchedule>::Sched>>::Read: Concat<T::AccumRead>,
    <W as WorkUnit<<W as HasSchedule>::Sched>>::Write: Concat<T::AccumWrite>,
    <<W as WorkUnit<<W as HasSchedule>::Sched>>::Read as Concat<T::AccumRead>>::Out: AccessSet,
    <<W as WorkUnit<<W as HasSchedule>::Sched>>::Write as Concat<T::AccumWrite>>::Out: AccessSet,
{
    type AccumRead = <<W as WorkUnit<<W as HasSchedule>::Sched>>::Read as Concat<T::AccumRead>>::Out;
    type AccumWrite =
        <<W as WorkUnit<<W as HasSchedule>::Sched>>::Write as Concat<T::AccumWrite>>::Out;
}

