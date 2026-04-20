//! Thread pool + wake + core class + thread handle tests
//! (5a4 skeleton).

use hilavitkutin::thread::{CoreClass, ThreadHandle, ThreadPool, WakeStrategy};

#[test]
fn thread_pool_new_records_core_count_and_wake() {
    let pool = ThreadPool::new(8, WakeStrategy::HybridSpinPark { spin_iters: 256 });
    assert_eq!(pool.thread_count, 8);
    assert_eq!(pool.spin_budget, 256);
    assert_eq!(
        pool.wake_strategy,
        WakeStrategy::HybridSpinPark { spin_iters: 256 }
    );
}

#[test]
fn thread_pool_default_constructs() {
    let pool = ThreadPool::default();
    assert_eq!(pool.thread_count, 1);
    assert_eq!(pool.wake_strategy, WakeStrategy::default_hybrid());
}

#[test]
fn wake_strategy_variants_distinct() {
    let hybrid = WakeStrategy::HybridSpinPark { spin_iters: 128 };
    let spin = WakeStrategy::PureSpin;
    let park = WakeStrategy::PurePark;
    assert_ne!(hybrid, spin);
    assert_ne!(hybrid, park);
    assert_ne!(spin, park);
}

#[test]
fn wake_strategy_default_hybrid_is_128_iters() {
    match WakeStrategy::default_hybrid() {
        WakeStrategy::HybridSpinPark { spin_iters } => assert_eq!(spin_iters, 128),
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
    let a = ThreadHandle(3);
    let b = a;
    assert_eq!(a, b);
    assert_eq!(a, ThreadHandle(3));
    assert_ne!(a, ThreadHandle(4));
    // ThreadHandle is Copy — original still usable.
    let _ = a.0;
}

#[test]
fn thread_pool_pure_spin_budget_is_max() {
    let pool = ThreadPool::new(4, WakeStrategy::PureSpin);
    assert_eq!(pool.spin_budget, u32::MAX);
}

#[test]
fn thread_pool_pure_park_budget_is_zero() {
    let pool = ThreadPool::new(4, WakeStrategy::PurePark);
    assert_eq!(pool.spin_budget, 0);
}
