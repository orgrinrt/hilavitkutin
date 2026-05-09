//! CoreAssignment + Convergence tests (5a4 skeleton).

use arvo::{Identity, USize};
use hilavitkutin::plan::FiberId;
use hilavitkutin::thread::{Convergence, CoreAssignment, NO_TRUNK, ThreadHandle};

#[test]
fn core_assignment_new_is_empty() {
    let a: CoreAssignment<8> = CoreAssignment::new();
    assert_eq!(a.assigned_count, USize::ZERO);
    for slot in a.trunk_index.iter() {
        assert_eq!(*slot, NO_TRUNK);
    }
    for m in a.morsel_size_multiplier.iter() {
        assert_eq!(*m, USize(100)); // lint:allow(no-bare-numeric) reason: default morsel-size-multiplier value; tracked: #399
    }
    for f in a.fiber_assignments.iter() {
        assert_eq!(*f, FiberId(0)); // lint:allow(no-bare-numeric) reason: default fiber id; tracked: #399
    }
}

#[test]
fn core_assignment_default_matches_new() {
    let a: CoreAssignment<4> = CoreAssignment::default();
    let b: CoreAssignment<4> = CoreAssignment::new();
    assert_eq!(a.assigned_count, b.assigned_count);
    assert_eq!(a.trunk_index, b.trunk_index);
    assert_eq!(a.morsel_size_multiplier, b.morsel_size_multiplier);
    assert_eq!(a.fiber_assignments, b.fiber_assignments);
}

#[test]
fn core_assignment_per_core_slot_mutation_roundtrips() {
    let mut a: CoreAssignment<4> = CoreAssignment::new();
    a.trunk_index[0] = USize(2); // lint:allow(no-bare-numeric) reason: trunk index value; tracked: #399
    a.fiber_assignments[0] = FiberId(5); // lint:allow(no-bare-numeric) reason: fiber id value; tracked: #399
    a.morsel_size_multiplier[0] = USize(200); // lint:allow(no-bare-numeric) reason: multiplier value; tracked: #399
    a.assigned_count = USize(1); // lint:allow(no-bare-numeric) reason: assigned count value; tracked: #399
    assert_eq!(a.trunk_index[0], USize(2)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(a.fiber_assignments[0], FiberId(5)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(a.morsel_size_multiplier[0], USize(200)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(a.assigned_count, USize(1)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    // Untouched slots still defaulted.
    assert_eq!(a.trunk_index[1], NO_TRUNK);
    assert_eq!(a.morsel_size_multiplier[3], USize(100)); // lint:allow(no-bare-numeric) reason: default value check; tracked: #399
}

#[test]
fn convergence_new_records_threads_and_zero_counter() {
    let c = Convergence::new(ThreadHandle(USize(3)), ThreadHandle(USize(7))); // lint:allow(no-bare-numeric) reason: thread handle ids; tracked: #399
    assert_eq!(c.head_thread, ThreadHandle(USize(3))); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(c.tail_thread, ThreadHandle(USize(7))); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(c.meeting_record.load(), USize::ZERO);
}
