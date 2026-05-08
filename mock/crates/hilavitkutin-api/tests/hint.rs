//! Scheduling hint axis discriminant ordering.

#![no_std]

use hilavitkutin_api::{
    Adaptive, Atomic, Critical, Deferred, DivisibilityValue, Immediate, Important,
    Interruptible, Normal, Opportunistic, Optional, Relaxed, SchedulingHint, SignificanceValue,
    Steady, UrgencyValue,
};

#[test]
fn urgency_ordering() {
    assert!(
        <Immediate as UrgencyValue>::VALUE.to_raw() > <Steady as UrgencyValue>::VALUE.to_raw()
    );
    assert!(<Steady as UrgencyValue>::VALUE.to_raw() > <Relaxed as UrgencyValue>::VALUE.to_raw());
    assert!(<Relaxed as UrgencyValue>::VALUE.to_raw() > <Deferred as UrgencyValue>::VALUE.to_raw());
}

#[test]
fn urgency_values() {
    assert_eq!(<Immediate as UrgencyValue>::VALUE.to_raw(), 3);
    assert_eq!(<Steady as UrgencyValue>::VALUE.to_raw(), 2);
    assert_eq!(<Relaxed as UrgencyValue>::VALUE.to_raw(), 1);
    assert_eq!(<Deferred as UrgencyValue>::VALUE.to_raw(), 0);
}

#[test]
fn divisibility_ordering() {
    assert!(
        <Atomic as DivisibilityValue>::VALUE.to_raw()
            > <Adaptive as DivisibilityValue>::VALUE.to_raw()
    );
    assert!(
        <Adaptive as DivisibilityValue>::VALUE.to_raw()
            > <Interruptible as DivisibilityValue>::VALUE.to_raw()
    );
}

#[test]
fn divisibility_values() {
    assert_eq!(<Atomic as DivisibilityValue>::VALUE.to_raw(), 2);
    assert_eq!(<Adaptive as DivisibilityValue>::VALUE.to_raw(), 1);
    assert_eq!(<Interruptible as DivisibilityValue>::VALUE.to_raw(), 0);
}

#[test]
fn significance_ordering() {
    assert!(
        <Critical as SignificanceValue>::VALUE.to_raw()
            > <Important as SignificanceValue>::VALUE.to_raw()
    );
    assert!(
        <Important as SignificanceValue>::VALUE.to_raw()
            > <Normal as SignificanceValue>::VALUE.to_raw()
    );
    assert!(
        <Normal as SignificanceValue>::VALUE.to_raw()
            > <Opportunistic as SignificanceValue>::VALUE.to_raw()
    );
    assert!(
        <Opportunistic as SignificanceValue>::VALUE.to_raw()
            > <Optional as SignificanceValue>::VALUE.to_raw()
    );
}

#[test]
fn significance_values() {
    assert_eq!(<Critical as SignificanceValue>::VALUE.to_raw(), 4);
    assert_eq!(<Important as SignificanceValue>::VALUE.to_raw(), 3);
    assert_eq!(<Normal as SignificanceValue>::VALUE.to_raw(), 2);
    assert_eq!(<Opportunistic as SignificanceValue>::VALUE.to_raw(), 1);
    assert_eq!(<Optional as SignificanceValue>::VALUE.to_raw(), 0);
}

fn require_hint<H: SchedulingHint>() {}

#[test]
fn tuple_is_scheduling_hint() {
    require_hint::<(Immediate, Atomic, Critical)>();
    require_hint::<(Deferred, Interruptible, Optional)>();
    require_hint::<(Steady, Adaptive, Normal)>();
}
