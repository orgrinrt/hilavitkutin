//! SchedulerBuilder type-state tests over the stores.
//!
//! A Kit is declarative (`type Units; type Owned`), and `.build()` proves
//! the stores registered hold every store the units read and write. These
//! tests read back what each registration put in the store list: a value, a
//! column's records, a position.
//!
//! The units that read those stores have their own files:
//! `scheduler_builder_units.rs` registers a unit beside a resource and a kit,
//! and `scheduler_builder_depth.rs` registers fifty units and a unit reading
//! sixteen stores. The arena, the kits and the stub context they share live
//! under `tests/scheduler_builder/`.
//!
//! The negative case (a unit registered without the resource it reads) is
//! the compile-fail pair `tests/ui/unregistered_store_{fail,pass}.rs`, run
//! by `tests/ui.rs`.

#[path = "scheduler_builder/arena.rs"]
mod arena;
#[path = "scheduler_builder/kits.rs"]
mod kits;

use arena::store;
use arvo::USize;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::{Column, Resource, StartsFromDefault};
use kits::{InternerKit, WorkspaceKit};

// A column no kit owns, for the raw registration beside a kit. It carries a
// value so a smoke can read back what it registered.
#[derive(Copy, Clone, Default)]
pub struct Notes(u32);
// A second one, registered after a kit.
#[derive(Copy, Clone, Default)]
pub struct Marks;
// A resource no kit owns, with a value to read back.
#[derive(Copy, Clone)]
pub struct Tally(u32);

// One scalar field, register-cached, so it adds nothing to the L1 morsel budget.
impl hilavitkutin_api::footprint::ResourceFootprint for Tally {
    const L1_BYTES: USize = USize(0);
}

// An empty builder with a provider is the no-store base case: a scheduler
// requires a memory provider, and nothing else. Its plan and meta storage still
// come from the provider, and a frame of it runs.
#[test]
fn empty_build() {
    let built = Scheduler::builder().build(store(), USize(0));
    let notko::Outcome::Ok(mut scheduler) = built else {
        panic!("the builder must accept this registration");
    };
    assert!(matches!(scheduler.run::<(), _>(), notko::Outcome::Ok(())));
}

#[test]
fn raw_resource_registration_builds() {
    let built = Scheduler::builder()
        .with(Resource::new(Tally(7)))
        .build(store(), USize(0));
    let notko::Outcome::Ok(scheduler) = built else {
        panic!("the builder must accept this registration");
    };
    // SAFETY: the binding holds the registered value; the scheduler is alive.
    let tally = unsafe { *scheduler.__bindings().__ptr().as_ptr() };
    assert_eq!(
        tally.0, 7,
        "the resource holds the value it was registered with"
    );
}

#[test]
fn raw_column_registration_builds() {
    const RECORDS: usize = 4;
    let built = Scheduler::builder()
        .with(Column::<Notes>::start_from(Notes(5)))
        .build(store(), USize(RECORDS));
    let notko::Outcome::Ok(scheduler) = built else {
        panic!("the builder must accept this registration");
    };
    let base = scheduler.__bindings().__ptr().as_ptr() as *const Notes;
    for i in 0 .. RECORDS {
        // SAFETY: the column holds RECORDS reserved records; the scheduler is alive.
        let note = unsafe { *base.add(i) };
        assert_eq!(note.0, 5, "record {i} starts from the registered value");
    }
}

// A kit's stores have no binding node, so what a kit registers shows in the
// position of a raw column registered ahead of it: past every store the kit
// owns.
#[test]
fn a_kit_of_one_store_holds_one_position_ahead_of_a_column() {
    let built = Scheduler::builder()
        .with(Column::<Notes>::start())
        .with(InternerKit)
        .build(store(), USize(0));
    let notko::Outcome::Ok(scheduler) = built else {
        panic!("the builder must accept this registration");
    };
    assert_eq!(
        scheduler.__bindings().__pos(),
        USize(1),
        "the kit's one store sits ahead of the column in the store list"
    );
}

#[test]
fn chained_kits_hold_the_sum_of_their_stores_ahead_of_a_column() {
    let built = Scheduler::builder()
        .with(Column::<Notes>::start())
        .with(InternerKit)
        .with(WorkspaceKit)
        .build(store(), USize(0));
    let notko::Outcome::Ok(scheduler) = built else {
        panic!("the builder must accept this registration");
    };
    assert_eq!(
        scheduler.__bindings().__pos(),
        USize(3),
        "both kits' three stores sit ahead of the column in the store list"
    );
}

// The raw column is one the kit does not own. A column registered twice, once
// by a kit and once raw, has two positions in the store list, so `build`
// cannot give its binding one and the registration is refused for want of a
// type annotation.
#[test]
fn mixed_kit_and_raw_build() {
    const RECORDS: usize = 4;
    let built = Scheduler::builder()
        .with(WorkspaceKit)
        .with(Resource::new(Tally(7)))
        .with(Column::<Notes>::start_from(Notes(5)))
        .build(store(), USize(RECORDS));
    let notko::Outcome::Ok(scheduler) = built else {
        panic!("the builder must accept this registration");
    };
    let notes = scheduler.__bindings();
    assert_eq!(
        notes.__pos(),
        USize(0),
        "the last store registered leads the list"
    );
    let base = notes.__ptr().as_ptr() as *const Notes;
    for i in 0 .. RECORDS {
        // SAFETY: the column holds RECORDS reserved records; the scheduler is alive.
        let note = unsafe { *base.add(i) };
        assert_eq!(note.0, 5, "record {i} starts from the registered value");
    }
    // SAFETY: the binding holds the registered value; the scheduler is alive.
    let tally = unsafe { *notes.__tail().__ptr().as_ptr() };
    assert_eq!(tally.0, 7, "the resource beside the kit holds its value");
}

// A kit's stores have a place in the store list and no value of their own,
// so they have no node along the bindings. A column registered ahead of the
// kit is the second node and the fourth store, and its position is the
// store's: the bit the access masks know it by.
#[test]
fn a_column_is_placed_by_the_store_list_and_not_by_its_node() {
    let built = Scheduler::builder()
        .with(Column::<Notes>::start())
        .with(WorkspaceKit)
        .with(Column::<Marks>::start())
        .build(store(), USize(0));
    let notko::Outcome::Ok(scheduler) = built else {
        panic!("the builder must accept this registration");
    };
    let marks = scheduler.__bindings();
    let notes = marks.__tail();
    assert_eq!(
        marks.__pos(),
        USize(0),
        "the last store registered leads the list"
    );
    assert_eq!(notes.__pos(), USize(3), "the kit's two stores sit between");
}
