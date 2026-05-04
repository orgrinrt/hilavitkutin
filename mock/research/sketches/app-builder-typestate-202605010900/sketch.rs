//! Sketch: SchedulerBuilder phantom-tuple type-state plus Kit preset.
//!
//! Validates the mechanism for round 202605010900 (#255) plus the
//! follow-up rounds 202605011000 (post-review polish) and
//! 202605011500 (Wus uncap, third-reviewer findings).
//!
//! Build with:
//!   rustup run nightly rustc --crate-type=lib --edition=2024 sketch.rs --emit=metadata
//!
//! Round 202605011500 changes vs prior:
//!   1. Buildable: per-arity 0..=12 macro replaced with one recursive
//!      impl + base case. Removes the 12-WU app cap.
//!   2. WuSatisfied: per-arity 0..=12 replaced with cons-list-only
//!      shape (() + (H, R) recursion). Removes the 12-store-per-WU
//!      cap. Consumers MUST use read! / write! macros to declare
//!      Read / Write as cons-lists rather than flat tuples.
//!   3. Kit::Output: structurally constrained via BuilderExtending<B>
//!      trait so a buggy Kit cannot wipe prior registrations.
//!   4. Recursion limit raised at the crate level.

#![no_std]
#![feature(marker_trait_attr)]
#![recursion_limit = "512"]
#![allow(dead_code, unused, incomplete_features)]

use core::marker::PhantomData;

type USize = usize;

// -----------------------------------------------------------------------
// AccessSet.
//
// Round 202605011500 finding: the per-arity flat-tuple impls 0..=12
// were sound but LEN reported the immediate-tuple-arity (always 2 for
// cons-list cells). Documented as "shape-based const" with a
// companion `Depth` trait that recursively counts cons-list nodes
// for cases where total depth matters.
// -----------------------------------------------------------------------

mod sealed {
    pub trait Sealed {}
}

#[allow(private_bounds)]
pub trait AccessSet: sealed::Sealed + 'static {
    /// Immediate tuple arity. For cons-list shapes `(H, R)` this
    /// reports 2 regardless of the recursive depth. Use `Depth` for
    /// the unfolded count.
    const LEN: USize;
}

#[marker]
pub trait Contains<S>: AccessSet {}

impl sealed::Sealed for () {}
impl AccessSet for () {
    const LEN: USize = 0;
}

impl<T0: 'static> sealed::Sealed for (T0,) {}
impl<T0: 'static> AccessSet for (T0,) {
    const LEN: USize = 1;
}
impl<T0: 'static> Contains<T0> for (T0,) {}

impl<T0: 'static, T1: 'static> sealed::Sealed for (T0, T1) {}
impl<T0: 'static, T1: 'static> AccessSet for (T0, T1) {
    const LEN: USize = 2;
}
impl<T0: 'static, T1: 'static> Contains<T0> for (T0, T1) {}
impl<T0: 'static, T1: 'static> Contains<T1> for (T0, T1) {}

// Higher-arity flat-tuple impls (3..=12 in real api) elided in this
// sketch; the 50-WU stress test below uses cons-list shapes which all
// reduce to arity-2 AccessSet at every cons cell. Real api keeps the
// 0..=12 flat impls for legacy compat plus the documented intent
// note.

// Cons-list Contains recursion. Allows membership to chain through
// arbitrary depth. #[marker] permits overlap with the arity-2 head
// match.
impl<H: 'static, R: 'static, T: 'static> Contains<T> for (H, R) where R: Contains<T> {}

// -----------------------------------------------------------------------
// Depth: recursive count of cons-list nodes. Plan-stage code queries
// `<Stores as Depth>::D` to get the actual registered-store count,
// since `AccessSet::LEN` reports immediate-tuple-arity (always 2 for
// cons-list cells).
//
// Defined ONLY on cons-list shapes (`()` and `(H, R)`). Flat tuples
// of arity 3+ do not impl Depth; they are not cons-lists. This is
// deliberate: every `Wus` / `Stores` accumulator in the engine is a
// cons-list by construction, and consumer-declared `Read` / `Write`
// uses the `read!` / `write!` macros which produce cons-list shapes.
//
// No specialization needed: `()` and `(H, R)` are structurally
// disjoint, so coherence holds with two simple impls. Sealed.
// -----------------------------------------------------------------------

mod depth_sealed {
    pub trait Sealed {}
}

#[allow(private_bounds)]
pub trait Depth: depth_sealed::Sealed {
    /// Total cons-list depth: number of registered elements in the
    /// accumulator. `<()>::D == 0`, `<(H, R)>::D == R::D + 1`.
    const D: USize;
}

impl depth_sealed::Sealed for () {}
impl<H, R: Depth> depth_sealed::Sealed for (H, R) {}

impl Depth for () {
    const D: USize = 0;
}

impl<H, R: Depth> Depth for (H, R) {
    const D: USize = R::D + 1;
}

// -----------------------------------------------------------------------
// Store markers.
// -----------------------------------------------------------------------

#[repr(transparent)]
pub struct Resource<T>(PhantomData<T>);
impl<T> Copy for Resource<T> {}
impl<T> Clone for Resource<T> {
    fn clone(&self) -> Self { *self }
}

#[repr(transparent)]
pub struct Column<T>(PhantomData<T>);
impl<T> Copy for Column<T> {}
impl<T> Clone for Column<T> {
    fn clone(&self) -> Self { *self }
}

#[repr(transparent)]
pub struct Virtual<T>(PhantomData<T>);
impl<T> Copy for Virtual<T> {}
impl<T> Clone for Virtual<T> {
    fn clone(&self) -> Self { *self }
}

// -----------------------------------------------------------------------
// read! / write! macros: convert flat tuple syntax to cons-list shape.
// Consumer writes:
//   type Read = read![Resource<X>, Column<Y>, Virtual<Z>];
// Macro emits:
//   (Resource<X>, (Column<Y>, (Virtual<Z>, ())))
// -----------------------------------------------------------------------

#[macro_export]
macro_rules! read {
    () => { () };
    ($T:ty $(,)?) => { ($T, ()) };
    ($T:ty, $($rest:ty),+ $(,)?) => { ($T, $crate::read!($($rest),+)) };
}

#[macro_export]
macro_rules! write {
    () => { () };
    ($T:ty $(,)?) => { ($T, ()) };
    ($T:ty, $($rest:ty),+ $(,)?) => { ($T, $crate::write!($($rest),+)) };
}

// -----------------------------------------------------------------------
// WorkUnit. Read / Write are cons-list shapes (use read! / write!).
// -----------------------------------------------------------------------

pub trait WorkUnit: 'static {
    type Read: AccessSet;
    type Write: AccessSet;
}

// -----------------------------------------------------------------------
// SchedulerBuilder type-state.
// -----------------------------------------------------------------------

pub struct SchedulerBuilder<Wus, Stores> {
    _phantom: PhantomData<(Wus, Stores)>,
}

impl SchedulerBuilder<(), ()> {
    pub const fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<Wus, Stores> SchedulerBuilder<Wus, Stores>
where
    Wus: AccessSet,
    Stores: AccessSet,
{
    pub fn add<W: WorkUnit>(self) -> SchedulerBuilder<(W, Wus), Stores>
    where
        (W, Wus): AccessSet,
    {
        SchedulerBuilder { _phantom: PhantomData }
    }

    pub fn resource<T: 'static>(self, _init: T) -> SchedulerBuilder<Wus, (Resource<T>, Stores)>
    where
        (Resource<T>, Stores): AccessSet,
    {
        SchedulerBuilder { _phantom: PhantomData }
    }

    pub fn column<T: 'static>(self) -> SchedulerBuilder<Wus, (Column<T>, Stores)>
    where
        (Column<T>, Stores): AccessSet,
    {
        SchedulerBuilder { _phantom: PhantomData }
    }

    pub fn add_virtual<T: 'static>(self) -> SchedulerBuilder<Wus, (Virtual<T>, Stores)>
    where
        (Virtual<T>, Stores): AccessSet,
    {
        SchedulerBuilder { _phantom: PhantomData }
    }

    /// Install a Kit. Output is constrained by `BuilderExtending<Self>`
    /// so a buggy Kit cannot wipe prior registrations: the Output
    /// type must be a SchedulerBuilder with the SAME Wus and a
    /// Stores cons-list that contains every store from input Stores.
    pub fn add_kit<K>(self, k: K) -> K::Output
    where
        K: Kit<Self>,
        K::Output: BuilderExtending<Self>,
    {
        k.install(self)
    }
}

// -----------------------------------------------------------------------
// Buildable: recursive over the Wus cons-list. No arity cap.
// -----------------------------------------------------------------------

mod build_sealed {
    pub trait Sealed {}
}

#[allow(private_bounds)]
pub trait Buildable<Stores: AccessSet>: build_sealed::Sealed {}

impl build_sealed::Sealed for () {}
impl<Stores: AccessSet> Buildable<Stores> for () {}

impl<H, R> build_sealed::Sealed for (H, R) {}
impl<H, R, Stores> Buildable<Stores> for (H, R)
where
    H: WorkUnit,
    R: Buildable<Stores>,
    Stores: AccessSet + WuSatisfied<H::Read> + WuSatisfied<H::Write>,
{
}

// -----------------------------------------------------------------------
// WuSatisfied: recursive over the cons-list shape of A. No arity cap.
// Consumer's WU::Read / WU::Write must be cons-list (use read!/write!).
// -----------------------------------------------------------------------

mod wu_sealed {
    pub trait Sealed<A> {}
}

#[allow(private_bounds)]
pub trait WuSatisfied<A: AccessSet>: wu_sealed::Sealed<A> {}

impl<S: AccessSet> wu_sealed::Sealed<()> for S {}
impl<S: AccessSet> WuSatisfied<()> for S {}

impl<S, H: 'static, R> wu_sealed::Sealed<(H, R)> for S
where
    S: Contains<H>,
    R: AccessSet,
    S: WuSatisfied<R>,
{
}
impl<S, H: 'static, R> WuSatisfied<(H, R)> for S
where
    S: Contains<H> + AccessSet + WuSatisfied<R>,
    R: AccessSet,
{
}

// -----------------------------------------------------------------------
// BuilderExtending<B>: proves Self extends B. Used to constrain
// Kit::Output so a Kit cannot return an unrelated builder type.
//
// For input SchedulerBuilder<Wus, Stores>, output must be
// SchedulerBuilder<Wus, NewStores> where NewStores: WuSatisfied<Stores>
// (every store in Stores is still present in NewStores).
// -----------------------------------------------------------------------

mod extending_sealed {
    pub trait Sealed<B> {}
}

#[allow(private_bounds)]
pub trait BuilderExtending<B>: extending_sealed::Sealed<B> {}

impl<Wus, Stores, NewStores> extending_sealed::Sealed<SchedulerBuilder<Wus, Stores>>
    for SchedulerBuilder<Wus, NewStores>
where
    Wus: AccessSet,
    Stores: AccessSet,
    NewStores: AccessSet + WuSatisfied<Stores>,
{
}
impl<Wus, Stores, NewStores> BuilderExtending<SchedulerBuilder<Wus, Stores>>
    for SchedulerBuilder<Wus, NewStores>
where
    Wus: AccessSet,
    Stores: AccessSet,
    NewStores: AccessSet + WuSatisfied<Stores>,
{
}

// -----------------------------------------------------------------------
// Scheduler + .build()
// -----------------------------------------------------------------------

pub struct Scheduler;

impl<Wus, Stores> SchedulerBuilder<Wus, Stores>
where
    Wus: Buildable<Stores>,
    Stores: AccessSet,
{
    pub fn build(self) -> Scheduler {
        Scheduler
    }
}

// -----------------------------------------------------------------------
// Kit trait: method-only Bevy-style.
// -----------------------------------------------------------------------

pub trait Kit<B> {
    type Output;
    fn install(self, builder: B) -> Self::Output;
}

// -----------------------------------------------------------------------
// Concrete examples.
// -----------------------------------------------------------------------

pub struct Interner;
pub struct Workspace;
pub struct FileInfo;
pub struct Diagnostic;

pub struct InternerKit;

impl<Wus, Stores> Kit<SchedulerBuilder<Wus, Stores>> for InternerKit
where
    Wus: AccessSet,
    Stores: AccessSet,
    (Resource<Interner>, Stores): AccessSet,
{
    type Output = SchedulerBuilder<Wus, (Resource<Interner>, Stores)>;
    fn install(self, builder: SchedulerBuilder<Wus, Stores>) -> Self::Output {
        builder.resource(Interner)
    }
}

pub struct WorkspaceKit;

impl<Wus, Stores> Kit<SchedulerBuilder<Wus, Stores>> for WorkspaceKit
where
    Wus: AccessSet,
    Stores: AccessSet,
    (Resource<Workspace>, Stores): AccessSet,
    (Column<FileInfo>, (Resource<Workspace>, Stores)): AccessSet,
{
    type Output = SchedulerBuilder<Wus, (Column<FileInfo>, (Resource<Workspace>, Stores))>;
    fn install(self, builder: SchedulerBuilder<Wus, Stores>) -> Self::Output {
        builder.resource(Workspace).column::<FileInfo>()
    }
}

// -----------------------------------------------------------------------
// WUs declared with read! / write! macros (cons-list shape).
// -----------------------------------------------------------------------

pub struct ReadInterner;
impl WorkUnit for ReadInterner {
    type Read = read![Resource<Interner>];
    type Write = ();
}

pub struct DiscoverFiles;
impl WorkUnit for DiscoverFiles {
    type Read = read![Resource<Workspace>];
    type Write = write![Column<FileInfo>];
}

pub struct EmitDiagnostics;
impl WorkUnit for EmitDiagnostics {
    type Read = read![Column<Diagnostic>, Resource<Interner>];
    type Write = ();
}

// A WU with 16 stores in Read (exceeds the prior 12 cap on WuSatisfied).
pub struct VehjeTypecheck;
pub struct TypeEnv;
pub struct ConstEnv;
pub struct ImportTable;
pub struct Definitions;
pub struct Bodies;
pub struct Mir;
pub struct Spans;
pub struct Errors;
pub struct ResolvedTypes;
pub struct NameTables;
pub struct Macros;
pub struct ScopeStack;
pub struct SealedRegistry;
pub struct OrphanRules;
impl WorkUnit for VehjeTypecheck {
    type Read = read![
        Resource<Interner>,
        Resource<TypeEnv>,
        Resource<ConstEnv>,
        Resource<ImportTable>,
        Resource<ScopeStack>,
        Resource<SealedRegistry>,
        Resource<OrphanRules>,
        Column<Definitions>,
        Column<Bodies>,
        Column<Mir>,
        Column<Spans>,
        Column<ResolvedTypes>,
        Column<NameTables>,
        Column<Macros>,
        Column<Diagnostic>,
        Column<Errors>
    ];
    type Write = write![Column<Errors>, Column<Diagnostic>];
}

// -----------------------------------------------------------------------
// Smoke tests: positive cases.
// -----------------------------------------------------------------------

pub fn smoke_kit_only() -> Scheduler {
    SchedulerBuilder::new()
        .add_kit(InternerKit)
        .add::<ReadInterner>()
        .build()
}

pub fn smoke_two_kits_chained() -> Scheduler {
    SchedulerBuilder::new()
        .add_kit(InternerKit)
        .add_kit(WorkspaceKit)
        .add::<ReadInterner>()
        .add::<DiscoverFiles>()
        .build()
}

pub fn smoke_mixed_kit_and_raw() -> Scheduler {
    SchedulerBuilder::new()
        .add_kit(WorkspaceKit)
        .resource(Interner)
        .column::<Diagnostic>()
        .add::<DiscoverFiles>()
        .add::<EmitDiagnostics>()
        .build()
}

// -----------------------------------------------------------------------
// 50-WU stress test for the Wus uncap fix (round 202605011500).
// -----------------------------------------------------------------------

macro_rules! decl_dummy_wus {
    ($($name:ident),+ $(,)?) => {
        $(
            pub struct $name;
            impl WorkUnit for $name {
                type Read = ();
                type Write = ();
            }
        )+
    };
}

decl_dummy_wus!(D00, D01, D02, D03, D04, D05, D06, D07, D08, D09);
decl_dummy_wus!(D10, D11, D12, D13, D14, D15, D16, D17, D18, D19);
decl_dummy_wus!(D20, D21, D22, D23, D24, D25, D26, D27, D28, D29);
decl_dummy_wus!(D30, D31, D32, D33, D34, D35, D36, D37, D38, D39);
decl_dummy_wus!(D40, D41, D42, D43, D44, D45, D46, D47, D48, D49);

pub fn smoke_fifty_wus() -> Scheduler {
    SchedulerBuilder::new()
        .add::<D00>().add::<D01>().add::<D02>().add::<D03>().add::<D04>()
        .add::<D05>().add::<D06>().add::<D07>().add::<D08>().add::<D09>()
        .add::<D10>().add::<D11>().add::<D12>().add::<D13>().add::<D14>()
        .add::<D15>().add::<D16>().add::<D17>().add::<D18>().add::<D19>()
        .add::<D20>().add::<D21>().add::<D22>().add::<D23>().add::<D24>()
        .add::<D25>().add::<D26>().add::<D27>().add::<D28>().add::<D29>()
        .add::<D30>().add::<D31>().add::<D32>().add::<D33>().add::<D34>()
        .add::<D35>().add::<D36>().add::<D37>().add::<D38>().add::<D39>()
        .add::<D40>().add::<D41>().add::<D42>().add::<D43>().add::<D44>()
        .add::<D45>().add::<D46>().add::<D47>().add::<D48>().add::<D49>()
        .build()
}

// Compile-time Depth assertions. cons! macro emits a cons-list type
// from a flat type list (mirrors the read! / write! macros for
// consumer use; here used only for the test fixtures).
macro_rules! cons {
    () => { () };
    ($T:ty $(,)?) => { ($T, ()) };
    ($T:ty, $($rest:ty),+ $(,)?) => { ($T, cons!($($rest),+)) };
}

type FiftyWusType = cons![
    D49, D48, D47, D46, D45, D44, D43, D42, D41, D40,
    D39, D38, D37, D36, D35, D34, D33, D32, D31, D30,
    D29, D28, D27, D26, D25, D24, D23, D22, D21, D20,
    D19, D18, D17, D16, D15, D14, D13, D12, D11, D10,
    D09, D08, D07, D06, D05, D04, D03, D02, D01, D00
];
const _: () = assert!(<FiftyWusType as Depth>::D == 50);
const _: () = assert!(<() as Depth>::D == 0);
const _: () = assert!(<(D00, ()) as Depth>::D == 1);
const _: () = assert!(<(D00, (D01, (D02, ()))) as Depth>::D == 3);

// 16-store-per-WU stress test for the WuSatisfied uncap fix.
pub fn smoke_wu_with_sixteen_stores() -> Scheduler {
    SchedulerBuilder::new()
        .resource(Interner)
        .resource(TypeEnv)
        .resource(ConstEnv)
        .resource(ImportTable)
        .resource(ScopeStack)
        .resource(SealedRegistry)
        .resource(OrphanRules)
        .column::<Definitions>()
        .column::<Bodies>()
        .column::<Mir>()
        .column::<Spans>()
        .column::<ResolvedTypes>()
        .column::<NameTables>()
        .column::<Macros>()
        .column::<Diagnostic>()
        .column::<Errors>()
        .add::<VehjeTypecheck>()
        .build()
}

// -----------------------------------------------------------------------
// Negative cases (commented out; uncomment to confirm compile-fail).
// -----------------------------------------------------------------------

// pub fn smoke_fail_missing_store() -> Scheduler {
//     SchedulerBuilder::new()
//         .add::<ReadInterner>()  // declares Read = read![Resource<Interner>]
//         .build()                // no Resource<Interner> registered
// }
// Expected error: WuSatisfied<(Resource<Interner>, ())> not satisfied
// because (): Contains<Resource<Interner>> not satisfied.

// pub struct BuggyKit;
// impl<Wus, Stores> Kit<SchedulerBuilder<Wus, Stores>> for BuggyKit
// where Wus: AccessSet, Stores: AccessSet,
// {
//     type Output = SchedulerBuilder<(), ()>;  // wipes Wus + Stores
//     fn install(self, _: SchedulerBuilder<Wus, Stores>) -> Self::Output {
//         SchedulerBuilder::new()
//     }
// }
// pub fn smoke_buggy_kit_rejected() -> Scheduler {
//     SchedulerBuilder::new()
//         .resource(Interner)
//         .add_kit(BuggyKit)  // BuilderExtending bound rejects: () ≠ Wus
//         .build()
// }
// Expected error: BuilderExtending<...> not satisfied for the Kit's
// Output type because the Wus / Stores params don't match input.
