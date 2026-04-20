//! Scheduling hint axis discriminant ordering.

#![no_std]

use hilavitkutin_api::{
    Adaptive, Atomic, Critical, Deferred, DivisibilityValue, Immediate, Important,
    Interruptible, Normal, Opportunistic, Optional, Relaxed, SchedulingHint, SignificanceValue,
    Steady, UrgencyValue,
};

#[test]
fn urgency_ordering() {
    assert!(<Immediate as UrgencyValue>::VALUE > <Steady as UrgencyValue>::VALUE);
    assert!(<Steady as UrgencyValue>::VALUE > <Relaxed as UrgencyValue>::VALUE);
    assert!(<Relaxed as UrgencyValue>::VALUE > <Deferred as UrgencyValue>::VALUE);
}

#[test]
fn urgency_values() {
    assert_eq!(<Immediate as UrgencyValue>::VALUE, 255);
    assert_eq!(<Steady as UrgencyValue>::VALUE, 170);
    assert_eq!(<Relaxed as UrgencyValue>::VALUE, 85);
    assert_eq!(<Deferred as UrgencyValue>::VALUE, 0);
}

#[test]
fn divisibility_ordering() {
    assert!(<Atomic as DivisibilityValue>::VALUE > <Adaptive as DivisibilityValue>::VALUE);
    assert!(<Adaptive as DivisibilityValue>::VALUE > <Interruptible as DivisibilityValue>::VALUE);
}

#[test]
fn divisibility_values() {
    assert_eq!(<Atomic as DivisibilityValue>::VALUE, 255);
    assert_eq!(<Adaptive as DivisibilityValue>::VALUE, 128);
    assert_eq!(<Interruptible as DivisibilityValue>::VALUE, 0);
}

#[test]
fn significance_ordering() {
    assert!(<Critical as SignificanceValue>::VALUE > <Important as SignificanceValue>::VALUE);
    assert!(<Important as SignificanceValue>::VALUE > <Normal as SignificanceValue>::VALUE);
    assert!(<Normal as SignificanceValue>::VALUE > <Opportunistic as SignificanceValue>::VALUE);
    assert!(<Opportunistic as SignificanceValue>::VALUE > <Optional as SignificanceValue>::VALUE);
}

#[test]
fn significance_values() {
    assert_eq!(<Critical as SignificanceValue>::VALUE, 255);
    assert_eq!(<Important as SignificanceValue>::VALUE, 192);
    assert_eq!(<Normal as SignificanceValue>::VALUE, 128);
    assert_eq!(<Opportunistic as SignificanceValue>::VALUE, 64);
    assert_eq!(<Optional as SignificanceValue>::VALUE, 0);
}

fn require_hint<H: SchedulingHint>() {}

#[test]
fn tuple_is_scheduling_hint() {
    require_hint::<(Immediate, Atomic, Critical)>();
    require_hint::<(Deferred, Interruptible, Optional)>();
    require_hint::<(Steady, Adaptive, Normal)>();
}
