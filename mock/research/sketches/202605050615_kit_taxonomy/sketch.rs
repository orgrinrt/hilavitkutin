//! S5: Kit Owned-state taxonomy completeness.
//!
//! Compiles the three diverse kit shapes named in topic 4 (MockspaceKit,
//! BenchTracingKit, LintPackKit) under the realistic visibility-plus-
//! replaceability surface. Tests:
//!
//!   1. Replaceable opt-in on a subset of each kit's Owned types.
//!   2. Visibility model: all Owned types pub (forced by E0446 on
//!      `pub Kit` impls; see finding block below). The "kit-internal"
//!      annotation in this sketch lives in convention (clearly-marked
//!      types not exported from the kit's public re-export module),
//!      not in the typesystem.
//!   3. Cooperative-public path: a WorkUnit in BenchTracingKit reads a
//!      pub type from LintPackKit's module.
//!   4. Negative test (feature-gated): replace_resource fails when T does
//!      not impl Replaceable.
//!
//! Build success path:
//!   `rustup run nightly rustc --crate-type=lib --edition=2024 \
//!       sketch.rs --emit=metadata`
//!
//! Negative test (feature-gated, expected to FAIL compile):
//!   `--cfg feature="show_replace_bound_error"` triggers the unsatisfied
//!       T: Replaceable bound on a freely-nameable but non-Replaceable
//!       type.
//!
//! Finding (substantive, recorded in FINDINGS.md):
//!
//! Rust's E0446 ("private/crate-private type in public interface") forces
//! every type appearing in a `pub Kit` impl's `type Owned = ...` to be at
//! least as visible as the impl itself. Two compile attempts demonstrated
//! the rule:
//!
//!   First attempt: Owned types declared as bare `struct` (module-private).
//!     Result: 13 errors of form "private type X in public interface".
//!   Second attempt: Owned types declared as `pub(crate) struct`.
//!     Result: 9 errors of form "crate-private type X in public interface".
//!   Third attempt (this file): Owned types declared as `pub struct`.
//!     Result: compiles.
//!
//! The implication: topic 3's "pub(crate) wrapping" of locked-down kit
//! state does not compose with `pub Kit` impls. The two workable shapes
//! are:
//!
//!   (A) Kit declared `pub`, every Owned type `pub`. Visibility provides
//!       nothing beyond convention; only Replaceable distinguishes.
//!   (B) Kit declared `pub(crate)`, every Owned type `pub(crate)`. The
//!       kit is then not directly nameable from a consumer crate; access
//!       requires a pub helper-fn shaped to consume + return a builder
//!       without naming the kit. Kit-internal state is genuinely kit-
//!       private at the crate boundary.
//!
//! The substrate cannot mix levels in a single Kit impl. Round-4 substrate
//! plan must reflect: kit visibility = single axis spanning the Kit impl
//! AND its Owned types together, not a per-Owned-type knob. This sketch
//! exercises shape (A) as the simpler test surface.

#![allow(unused)]
#![no_std]
#![feature(marker_trait_attr)]

use core::marker::PhantomData;

// ----- AccessSet substrate (B-shape, marker-overlap, identical to S1) -----

pub struct Empty;
pub struct Cons<H, T>(PhantomData<(H, T)>);

pub trait AccessSet {}
impl AccessSet for Empty {}
impl<H, T: AccessSet> AccessSet for Cons<H, T> {}

#[marker]
pub trait Contains<X>: AccessSet {}
impl<X, T: AccessSet> Contains<X> for Cons<X, T> {}
impl<X, H, T: AccessSet> Contains<X> for Cons<H, T> where T: Contains<X> {}

#[marker]
pub trait ContainsAll<L>: AccessSet {}
impl<S: AccessSet> ContainsAll<Empty> for S {}
impl<S: AccessSet, H, T> ContainsAll<Cons<H, T>> for S
where
    S: Contains<H> + ContainsAll<T>,
{
}

pub trait Concat<R> {
    type Out;
}
impl<R> Concat<R> for Empty {
    type Out = R;
}
impl<H, T, R> Concat<R> for Cons<H, T>
where
    T: Concat<R>,
{
    type Out = Cons<H, <T as Concat<R>>::Out>;
}

pub trait WorkUnit {
    type Read: AccessSet;
    type Write: AccessSet;
}

pub trait WorkUnitBundle {
    type AccumRead: AccessSet;
    type AccumWrite: AccessSet;
}
impl WorkUnitBundle for Empty {
    type AccumRead = Empty;
    type AccumWrite = Empty;
}
impl<W: WorkUnit, T: WorkUnitBundle> WorkUnitBundle for Cons<W, T>
where
    W::Read: Concat<T::AccumRead>,
    W::Write: Concat<T::AccumWrite>,
    <W::Read as Concat<T::AccumRead>>::Out: AccessSet,
    <W::Write as Concat<T::AccumWrite>>::Out: AccessSet,
{
    type AccumRead = <W::Read as Concat<T::AccumRead>>::Out;
    type AccumWrite = <W::Write as Concat<T::AccumWrite>>::Out;
}

pub trait StoreBundle {}
impl StoreBundle for Empty {}
impl<H, T: StoreBundle> StoreBundle for Cons<H, T> {}

pub trait Kit {
    type Units: WorkUnitBundle;
    type Owned: StoreBundle;
}

// ----- App-level shared resources (visible to every kit) -----

pub struct StringInterner;
pub struct Clock;

// ----- Replaceability axis: opt-in marker -----

pub trait Replaceable {}

// ----- MockspaceKit: multi-marker Owned with mixed visibility -----

pub mod mockspace_kit {
    use super::{Clock, Cons, Empty, Kit, Replaceable, StringInterner, WorkUnit};

    /// Pub, marked Replaceable: app-overridable for test fixtures.
    pub struct LintConfig;

    // Pub by necessity (E0446 forces it; see top of file). Convention:
    // these would be kept out of the kit's public re-export surface so
    // consumers do not casually reach for them. The substrate cannot
    // enforce that with the current `pub Kit { type Owned: ... }` shape.
    pub struct RoundState;
    pub struct DesignRound;

    impl Replaceable for LintConfig {}

    pub struct RoundProcessor;
    impl WorkUnit for RoundProcessor {
        type Read = Cons<StringInterner, Empty>;
        type Write = Cons<RoundState, Cons<DesignRound, Empty>>;
    }

    pub struct LintConfigLoader;
    impl WorkUnit for LintConfigLoader {
        type Read = Cons<Clock, Empty>;
        type Write = Cons<LintConfig, Empty>;
    }

    pub struct MockspaceKit;
    impl Kit for MockspaceKit {
        type Units = Cons<RoundProcessor, Cons<LintConfigLoader, Empty>>;
        type Owned = Cons<RoundState, Cons<DesignRound, Cons<LintConfig, Empty>>>;
    }
}

// ----- LintPackKit: cooperative-public Diagnostic, kit-private Statistics -----

pub mod lint_pack_kit {
    use super::{Cons, Empty, Kit, Replaceable, StringInterner, WorkUnit};

    /// Pub, marked Replaceable: cooperative-public diagnostic stream
    /// also overridable for structured-output collectors during tests.
    pub struct Diagnostic;

    // Pub by necessity. Convention-internal.
    pub struct Statistics;

    impl Replaceable for Diagnostic {}

    pub struct LintEmitter;
    impl WorkUnit for LintEmitter {
        type Read = Cons<StringInterner, Empty>;
        type Write = Cons<Diagnostic, Empty>;
    }

    pub struct DiagnosticAggregator;
    impl WorkUnit for DiagnosticAggregator {
        type Read = Cons<Diagnostic, Empty>;
        type Write = Cons<Statistics, Empty>;
    }

    pub struct LintPackKit;
    impl Kit for LintPackKit {
        type Units = Cons<LintEmitter, Cons<DiagnosticAggregator, Empty>>;
        type Owned = Cons<Diagnostic, Cons<Statistics, Empty>>;
    }
}

// ----- BenchTracingKit: pub Tracer, kit-private TraceSample, plus a -----
// ----- cooperative-public consumption of LintPackKit's Diagnostic.    -----

pub mod bench_tracing_kit {
    use super::lint_pack_kit::Diagnostic;
    use super::{Cons, Empty, Kit, Replaceable, WorkUnit};

    /// Pub, marked Replaceable: cooperative Tracer, app may swap for an
    /// FFI-bound or no-op implementation.
    pub struct Tracer;

    // Pub by necessity. Convention-internal.
    pub struct TraceSample;

    impl Replaceable for Tracer {}

    /// TracerFiber reads LintPackKit's pub Diagnostic to attach trace
    /// metadata. This is the cooperative-public path the audit requires
    /// the sketch to exercise.
    pub struct TracerFiber;
    impl WorkUnit for TracerFiber {
        type Read = Cons<Diagnostic, Empty>;
        type Write = Cons<Tracer, Cons<TraceSample, Empty>>;
    }

    pub struct SampleCollector;
    impl WorkUnit for SampleCollector {
        type Read = Cons<Tracer, Empty>;
        type Write = Cons<TraceSample, Empty>;
    }

    pub struct BenchTracingKit;
    impl Kit for BenchTracingKit {
        type Units = Cons<TracerFiber, Cons<SampleCollector, Empty>>;
        type Owned = Cons<Tracer, Cons<TraceSample, Empty>>;
    }
}

// ----- SchedulerBuilder typestate (mirrors S1's surface) -----

pub struct SchedulerBuilder<Wus, Stores>(PhantomData<(Wus, Stores)>);

impl SchedulerBuilder<Empty, Empty> {
    pub fn new() -> Self {
        SchedulerBuilder(PhantomData)
    }
}

impl<Wus, Stores> SchedulerBuilder<Wus, Stores> {
    pub fn resource<T>(self) -> SchedulerBuilder<Wus, Cons<T, Stores>> {
        SchedulerBuilder(PhantomData)
    }

    pub fn add_kit<K: Kit>(
        self,
    ) -> SchedulerBuilder<<K::Units as Concat<Wus>>::Out, <K::Owned as Concat<Stores>>::Out>
    where
        K::Units: Concat<Wus>,
        K::Owned: Concat<Stores>,
    {
        SchedulerBuilder(PhantomData)
    }
}

pub struct Scheduler<Wus, Stores>(PhantomData<(Wus, Stores)>);

impl<Wus: WorkUnitBundle, Stores: AccessSet + StoreBundle> SchedulerBuilder<Wus, Stores>
where
    Stores: ContainsAll<Wus::AccumRead> + ContainsAll<Wus::AccumWrite>,
{
    pub fn build(self) -> Scheduler<Wus, Stores> {
        Scheduler(PhantomData)
    }
}

impl<Wus, Stores> Scheduler<Wus, Stores> {
    /// Replace API: app-only, static `T: Replaceable` bound.
    pub fn replace_resource<T: Replaceable>(&mut self, _new: T) {}
}

// ----- Success path -----

pub fn demo_success() {
    use bench_tracing_kit::BenchTracingKit;
    use lint_pack_kit::LintPackKit;
    use mockspace_kit::MockspaceKit;

    let mut s = SchedulerBuilder::new()
        .resource::<StringInterner>()
        .resource::<Clock>()
        .add_kit::<MockspaceKit>()
        .add_kit::<LintPackKit>()
        .add_kit::<BenchTracingKit>()
        .build();

    // Replaceable + nameable: works.
    s.replace_resource(mockspace_kit::LintConfig);
    s.replace_resource(lint_pack_kit::Diagnostic);
    s.replace_resource(bench_tracing_kit::Tracer);
}

// Negative test 1: T: Replaceable bound failure.
//
// AppPublicValueButNotReplaceable is pub and freely nameable, but does
// not impl Replaceable. The compile error proves the bound (not the
// visibility) is doing the work for this case.
pub struct AppPublicValueButNotReplaceable;

#[cfg(feature = "show_replace_bound_error")]
pub fn demo_replace_bound_error() {
    let mut s = SchedulerBuilder::new()
        .resource::<StringInterner>()
        .resource::<Clock>()
        .add_kit::<mockspace_kit::MockspaceKit>()
        .add_kit::<lint_pack_kit::LintPackKit>()
        .add_kit::<bench_tracing_kit::BenchTracingKit>()
        .build();
    s.replace_resource(AppPublicValueButNotReplaceable);
}

// Note on the visibility axis (the substantive finding):
//
// E0446 forced every Owned type to pub. Topic 3's "two committed axes"
// (visibility plus replaceability) survives only as a one-and-a-half
// axis surface: Replaceable is the lone substrate-enforced axis;
// visibility is convention plus crate-boundary mechanics applied to the
// Kit-as-a-whole, not a per-Owned-type knob.
//
// To make a type genuinely kit-internal at the crate boundary, the
// kit's whole shape (Kit struct + Owned types + WU types) must move
// to pub(crate) at the kit's own crate root. Consumer code outside that
// crate then registers via a pub helper-fn that consumes a builder and
// returns one without naming the kit. That structural pattern is not
// what this single-file sketch validates; recording it here for the
// doc CL.
//
// ----- Counts (manual) -----
// Owned types declared: 7 across 3 kits, all pub.
//   MockspaceKit:    RoundState, DesignRound, LintConfig.       [3]
//   LintPackKit:     Diagnostic, Statistics.                    [2]
//   BenchTracingKit: Tracer,     TraceSample.                   [2]
// Replaceable opt-ins: 3 (LintConfig, Diagnostic, Tracer).
// Convention-internal types (no Replaceable, kit-author intent kept
//   out of the public re-export surface): 4.
// Cooperative-public reads: 1 (TracerFiber reads LintPackKit::Diagnostic).
