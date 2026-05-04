//! AdaptConfig + AdaptMetrics + AdaptMode tests (5a4 skeleton).

use hilavitkutin::adapt::{AdaptConfig, AdaptMetrics, AdaptMode};
use hilavitkutin::plan::PhaseId;
use hilavitkutin::strategy::PhaseStrategy;

#[test]
fn adapt_config_new_records_all_fields() {
    let c = AdaptConfig::new(PhaseId(3), 1024, 150, 8192);
    assert_eq!(c.phase_id, PhaseId(3));
    assert_eq!(c.max_fuse_threshold, 1024);
    assert_eq!(c.morsel_size_multiplier, 150);
    assert_eq!(c.split_threshold, 8192);
}

#[test]
fn adapt_config_default_is_zero() {
    let c = AdaptConfig::default();
    assert_eq!(c.phase_id, PhaseId(0));
    assert_eq!(c.max_fuse_threshold, 0);
    assert_eq!(c.morsel_size_multiplier, 0);
    assert_eq!(c.split_threshold, 0);
}

#[test]
fn adapt_mode_is_phase_strategy_alias() {
    // AdaptMode is a type alias for PhaseStrategy: the two
    // should be interchangeable. Spell each variant through
    // both paths and compare.
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
    let m = AdaptMetrics::default();
    assert_eq!(m.cache_miss_rate, 0);
    assert_eq!(m.branch_miss_rate, 0);
    assert_eq!(m.phase_completion_time_ns, 0);
}

#[test]
fn adapt_metrics_new_is_zero() {
    let m = AdaptMetrics::new();
    assert_eq!(m.cache_miss_rate, 0);
    assert_eq!(m.branch_miss_rate, 0);
    assert_eq!(m.phase_completion_time_ns, 0);
}

// update_adapt is `todo!()` this round: don't call it, just
// verify the signature compiles by taking a function pointer to
// it with an explicit const param.
#[test]
fn update_adapt_signature_compiles() {
    let _f: fn(&mut [AdaptConfig; 4], &AdaptMetrics) = hilavitkutin::adapt::update_adapt::<4>;
}
