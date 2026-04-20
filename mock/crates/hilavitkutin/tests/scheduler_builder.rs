//! SchedulerBuilder chaining tests.

use hilavitkutin::scheduler::Scheduler;

struct DummyWU;
struct DummyResource;
struct DummyMemory;
struct DummyPool;
struct DummyClock;

#[test]
fn builder_chains_and_builds() {
    let _ = Scheduler::<8, 16, 4>::builder()
        .add::<DummyWU>()
        .resource::<DummyResource>(DummyResource)
        .column::<DummyResource>()
        .memory(DummyMemory)
        .threads(DummyPool)
        .clock(DummyClock)
        .build();
}

#[test]
fn default_scheduler_constructs() {
    let _: Scheduler<4, 8, 2> = Scheduler::default();
}
