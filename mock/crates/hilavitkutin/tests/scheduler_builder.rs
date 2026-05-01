//! SchedulerBuilder type-state tests.
//!
//! These tests exercise the build-time type-state machinery
//! introduced in round 202605010900: phantom-tuple `Wus` / `Stores`
//! accumulation through the registration methods, Kit installation
//! returning the Kit's `Output` type-state, and `.build()`'s
//! `Buildable<Stores>` proof reducing trivially when `Wus = ()`.
//!
//! Tests that need actual `WorkUnit` impls (the full
//! HasColumnReader / HasColumnWriter / HasResourceProvider / ...
//! `Ctx` bound) are deferred until the runtime side (5a2 / 5a3 /
//! 5a4) ships test infrastructure for fake providers. The
//! research sketch at
//! `mock/research/sketches/app-builder-typestate-202605010900/`
//! validates the full flow against simplified WU stand-ins; the
//! mechanism is the same one shipped in src/scheduler/mod.rs.

use hilavitkutin::scheduler::{Scheduler, SchedulerBuilder};
use hilavitkutin_api::access::AccessSet;
use hilavitkutin_api::store::{Column, Resource};
use hilavitkutin_kit::Kit;

// ---------------------------------------------------------------------
// Fake stores.
// ---------------------------------------------------------------------

pub struct Interner;
pub struct Workspace;
pub struct FileInfo;

// ---------------------------------------------------------------------
// Kits.
// ---------------------------------------------------------------------

pub struct InternerKit;

impl<
    const MAX_UNITS: usize,
    const MAX_STORES: usize,
    const MAX_LANES: usize,
    Wus,
    Stores,
> Kit<SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, Stores>> for InternerKit
where
    Wus: AccessSet,
    Stores: AccessSet,
    (Resource<Interner>, Stores): AccessSet,
{
    type Output =
        SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, (Resource<Interner>, Stores)>;

    fn install(
        self,
        builder: SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, Stores>,
    ) -> Self::Output {
        builder.resource(Interner)
    }
}

pub struct WorkspaceKit;

impl<
    const MAX_UNITS: usize,
    const MAX_STORES: usize,
    const MAX_LANES: usize,
    Wus,
    Stores,
> Kit<SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, Stores>> for WorkspaceKit
where
    Wus: AccessSet,
    Stores: AccessSet,
    (Resource<Workspace>, Stores): AccessSet,
    (Column<FileInfo>, (Resource<Workspace>, Stores)): AccessSet,
{
    type Output = SchedulerBuilder<
        MAX_UNITS,
        MAX_STORES,
        MAX_LANES,
        Wus,
        (Column<FileInfo>, (Resource<Workspace>, Stores)),
    >;

    fn install(
        self,
        builder: SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, Stores>,
    ) -> Self::Output {
        builder.resource(Workspace).column::<FileInfo>()
    }
}

// ---------------------------------------------------------------------
// Positive smoke tests.
//
// All build with `Wus = ()` so `Buildable<Stores>` reduces
// trivially via the arity-0 impl. The Stores-accumulation path is
// exercised independently from WU declarations.
// ---------------------------------------------------------------------

#[test]
fn empty_build() {
    let _ = Scheduler::<8, 16, 4>::builder().build();
}

#[test]
fn raw_resource_registration_builds() {
    let _ = Scheduler::<8, 16, 4>::builder()
        .resource(Interner)
        .build();
}

#[test]
fn raw_column_registration_builds() {
    let _ = Scheduler::<8, 16, 4>::builder().column::<FileInfo>().build();
}

#[test]
fn kit_only_builds() {
    let _ = Scheduler::<8, 16, 4>::builder().add_kit(InternerKit).build();
}

#[test]
fn two_kits_chained_build() {
    let _ = Scheduler::<8, 16, 4>::builder()
        .add_kit(InternerKit)
        .add_kit(WorkspaceKit)
        .build();
}

#[test]
fn mixed_kit_and_raw_build() {
    let _ = Scheduler::<8, 16, 4>::builder()
        .add_kit(WorkspaceKit)
        .resource(Interner)
        .column::<FileInfo>()
        .build();
}

#[test]
fn default_scheduler_constructs() {
    let _: Scheduler<4, 8, 2> = Scheduler::default();
}

// ---------------------------------------------------------------------
// Type-state shape verification.
//
// Asserts the Kit's `Output` type matches the documented contract:
// `InternerKit::install(builder)` returns a builder with
// `Resource<Interner>` prepended onto the previous `Stores`.
// ---------------------------------------------------------------------

#[test]
fn kit_extends_stores_type() {
    fn _type_check_only<const M: usize, const N: usize, const L: usize, W, S>(
        b: SchedulerBuilder<M, N, L, W, S>,
    ) -> SchedulerBuilder<M, N, L, W, (Resource<Interner>, S)>
    where
        W: AccessSet,
        S: AccessSet,
        (Resource<Interner>, S): AccessSet,
    {
        b.add_kit(InternerKit)
    }
}
