//! AdaptConfig + AdaptMetrics + AdaptMode tests (5a4 skeleton).

use arvo::{Identity, USize};
use hilavitkutin::adapt::{AdaptConfig, AdaptMetrics, AdaptMode};
use hilavitkutin::plan::PhaseId;
use hilavitkutin::strategy::PhaseStrategy;

#[test]
fn adapt_config_new_records_all_fields() {
    let c = AdaptConfig::new(
        PhaseId::from_constant::<{ USize(3) }>(), // lint:allow(no-bare-numeric) reason: phase id literal; tracked: #426
        USize(1024),                              // lint:allow(no-bare-numeric) reason: tuning literal; tracked: #426
        USize(150),                               // lint:allow(no-bare-numeric) reason: tuning literal; tracked: #426
        USize(8192),                              // lint:allow(no-bare-numeric) reason: tuning literal; tracked: #426
    );
    assert_eq!(c.phase_id, PhaseId::from_constant::<{ USize(3) }>()); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #426
    assert_eq!(c.max_fuse_threshold, USize(1024)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #426
    assert_eq!(c.morsel_size_multiplier, USize(150)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #426
    assert_eq!(c.split_threshold, USize(8192)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #426
}

#[test]
fn adapt_config_default_is_zero() {
    let c = AdaptConfig::new(PhaseId::ZERO, USize::ZERO, USize::ZERO, USize::ZERO);
    assert_eq!(c.phase_id, PhaseId::ZERO);
    assert_eq!(c.max_fuse_threshold, USize::ZERO);
    assert_eq!(c.morsel_size_multiplier, USize::ZERO);
    assert_eq!(c.split_threshold, USize::ZERO);
}

#[test]
fn adapt_mode_is_phase_strategy_alias() {
    let a: AdaptMode = AdaptMode::MaxFuse;
    let b: PhaseStrategy = PhaseStrategy::MaxFuse;
    assert_eq!(a, b);

    let c: AdaptMode = AdaptMode::Balanced;
    let d: PhaseStrategy = PhaseStrategy::Balanced;
    assert_eq!(c, d);

    let e: AdaptMode = AdaptMode::MaxSplit;
    let f: PhaseStrategy = PhaseStrategy::MaxSplit;
    assert_eq!(e, f);

    assert_ne!(a, c);
    assert_ne!(c, e);
    assert_ne!(a, e);
}

#[test]
fn adapt_metrics_default_is_zero() {
    let m = AdaptMetrics::new();
    assert_eq!(m.cache_miss_rate, USize::ZERO);
    assert_eq!(m.branch_miss_rate, USize::ZERO);
    assert_eq!(m.phase_completion_time_ns.to_raw(), 0); // lint:allow(no-bare-numeric) reason: nanos zero raw literal; tracked: #426
}

#[test]
fn adapt_metrics_new_is_zero() {
    let m = AdaptMetrics::new();
    assert_eq!(m.cache_miss_rate, USize::ZERO);
    assert_eq!(m.branch_miss_rate, USize::ZERO);
    assert_eq!(m.phase_completion_time_ns.to_raw(), 0); // lint:allow(no-bare-numeric) reason: nanos zero raw literal; tracked: #426
}

#[test]
fn update_adapt_signature_compiles() {
    let _f: fn(&mut [AdaptConfig; 4], &AdaptMetrics) = hilavitkutin::adapt::update_adapt::<4>;
}
