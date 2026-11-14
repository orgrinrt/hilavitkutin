//! SchedulerBuilder type-state tests with a unit registered.
//!
//! Each unit names a stub context, so these exercise the load-bearing
//! `HoldsAll` proof reduction: `build` takes a unit when the stores registered
//! hold every store its read and write sets name, whether they are registered
//! raw or by a kit. A unit's body is never run.
//!
//! The store-only tests are in `scheduler_builder.rs`, and the stress shapes
//! (fifty units, sixteen stores) in `scheduler_builder_depth.rs`. The arena, the
//! kits and the stub context live under `tests/scheduler_builder/`.

#[path = "scheduler_builder/arena.rs"]
mod arena;
#[path = "scheduler_builder/kits.rs"]
mod kits;
#[path = "scheduler_builder/shim.rs"]
mod shim;

use arena::store;
use arvo::USize;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::{
    Always,
    Atomic,
    BuilderInput,
    Column,
    Immediate,
    Normal,
    Resource,
    UnitDispatch,
    WorkUnit,
    read,
    write,
};
use kits::{FileInfo, Interner, InternerKit, Workspace, WorkspaceKit};
use shim::TestCtx;

// A WU that reads the Interner resource.
struct ReadInterner;

impl BuilderInput for ReadInterner {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for ReadInterner {
    type Ctx<'frame> = TestCtx;
    type Hint = (Immediate, Atomic, Normal);
    type Read = read![Resource<Interner>];
    type Write = read![];

    fn execute<'frame>(&self, _ctx: &TestCtx) {}
}

// A WU with write set: writes Column<FileInfo>.
struct DiscoverFiles;

impl BuilderInput for DiscoverFiles {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for DiscoverFiles {
    type Ctx<'frame> = TestCtx;
    type Hint = (Immediate, Atomic, Normal);
    type Read = read![Resource<Workspace>];
    type Write = write![Column<FileInfo>];

    fn execute<'frame>(&self, _ctx: &TestCtx) {}
}

#[test]
fn wu_with_raw_resource_builds() {
    let built = Scheduler::builder()
        .with(Resource::new(Interner))
        .with(ReadInterner)
        .build(store(), USize(0));
    assert!(built.is_ok(), "the builder must accept this registration");
}

#[test]
fn wu_with_kit_builds() {
    let built = Scheduler::builder()
        .with(InternerKit)
        .with(ReadInterner)
        .build(store(), USize(0));
    assert!(built.is_ok(), "the builder must accept this registration");
}

#[test]
fn two_wus_with_two_kits_build() {
    let built = Scheduler::builder()
        .with(InternerKit)
        .with(WorkspaceKit)
        .with(ReadInterner)
        .with(DiscoverFiles)
        .build(store(), USize(0));
    assert!(built.is_ok(), "the builder must accept this registration");
}
