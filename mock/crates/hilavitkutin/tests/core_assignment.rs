//! CoreAssignment + Convergence tests (5a4 skeleton).

use hilavitkutin::plan::FiberId;
use hilavitkutin::thread::{Convergence, CoreAssignment, ThreadHandle};

#[test]
fn core_assignment_new_is_empty() {
    let a: CoreAssignment<8> = CoreAssignment::new();
    assert_eq!(a.assigned_count, 0);
    for slot in a.trunk_index.iter() {
        assert_eq!(*slot, u16::MAX);
    }
    for m in a.morsel_size_multiplier.iter() {
        assert_eq!(*m, 100);
    }
    for f in a.fiber_assignments.iter() {
        assert_eq!(*f, FiberId(0));
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
    a.trunk_index[0] = 2;
    a.fiber_assignments[0] = FiberId(5);
    a.morsel_size_multiplier[0] = 200;
    a.assigned_count = 1;
    assert_eq!(a.trunk_index[0], 2);
    assert_eq!(a.fiber_assignments[0], FiberId(5));
    assert_eq!(a.morsel_size_multiplier[0], 200);
    assert_eq!(a.assigned_count, 1);
    // Untouched slots still defaulted.
    assert_eq!(a.trunk_index[1], u16::MAX);
    assert_eq!(a.morsel_size_multiplier[3], 100);
}

#[test]
fn convergence_new_records_threads_and_zero_counter() {
    let c = Convergence::new(ThreadHandle(3), ThreadHandle(7));
    assert_eq!(c.head_thread, ThreadHandle(3));
    assert_eq!(c.tail_thread, ThreadHandle(7));
    assert_eq!(c.meeting_record.load(), 0);
}
