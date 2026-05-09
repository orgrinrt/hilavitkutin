//! Thread pool + wake + core class + thread handle tests
//! (5a4 skeleton).

use arvo::{Identity, USize};
use hilavitkutin::thread::{CoreClass, ThreadHandle, ThreadPool, WakeStrategy};

#[test]
fn thread_pool_new_records_core_count_and_wake() {
    let pool = ThreadPool::new(USize(8), WakeStrategy::HybridSpinPark { spin_iters: USize(256) }); // lint:allow(no-bare-numeric) reason: thread count + spin budget literal; tracked: #399
    assert_eq!(pool.thread_count, USize(8)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(pool.spin_budget, USize(256)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(
        pool.wake_strategy,
        WakeStrategy::HybridSpinPark { spin_iters: USize(256) } // lint:allow(no-bare-numeric) reason: spin budget literal; tracked: #399
    );
}

#[test]
fn thread_pool_default_constructs() {
    let pool = ThreadPool::default();
    assert_eq!(pool.thread_count, USize(1)); // lint:allow(no-bare-numeric) reason: default thread count; tracked: #399
    assert_eq!(pool.wake_strategy, WakeStrategy::default_hybrid());
}

#[test]
fn wake_strategy_variants_distinct() {
    let hybrid = WakeStrategy::HybridSpinPark { spin_iters: USize(128) }; // lint:allow(no-bare-numeric) reason: spin budget literal; tracked: #399
    let spin = WakeStrategy::PureSpin;
    let park = WakeStrategy::PurePark;
    assert_ne!(hybrid, spin);
    assert_ne!(hybrid, park);
    assert_ne!(spin, park);
}

#[test]
fn wake_strategy_default_hybrid_is_128_iters() {
    match WakeStrategy::default_hybrid() {
        WakeStrategy::HybridSpinPark { spin_iters } => assert_eq!(spin_iters, USize(128)), // lint:allow(no-bare-numeric) reason: default spin budget; tracked: #399
        _ => panic!("default_hybrid should be HybridSpinPark"),
    }
}

#[test]
fn core_class_variants_and_default() {
    assert_ne!(CoreClass::P, CoreClass::E);
    assert_eq!(CoreClass::default(), CoreClass::P);
}

#[test]
fn thread_handle_copy_and_eq() {
    let a = ThreadHandle(USize(3)); // lint:allow(no-bare-numeric) reason: handle id literal; tracked: #399
    let b = a;
    assert_eq!(a, b);
    assert_eq!(a, ThreadHandle(USize(3))); // lint:allow(no-bare-numeric) reason: handle id literal; tracked: #399
    assert_ne!(a, ThreadHandle(USize(4))); // lint:allow(no-bare-numeric) reason: handle id literal; tracked: #399
    // ThreadHandle is Copy: original still usable.
    let _ = a.0;
}

#[test]
fn thread_pool_pure_spin_budget_is_max() {
    let pool = ThreadPool::new(USize(4), WakeStrategy::PureSpin); // lint:allow(no-bare-numeric) reason: thread count literal; tracked: #399
    assert_eq!(pool.spin_budget, USize(usize::MAX)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: pure-spin sentinel matches source; tracked: #72
}

#[test]
fn thread_pool_pure_park_budget_is_zero() {
    let pool = ThreadPool::new(USize(4), WakeStrategy::PurePark); // lint:allow(no-bare-numeric) reason: thread count literal; tracked: #399
    assert_eq!(pool.spin_budget, USize::ZERO);
}
