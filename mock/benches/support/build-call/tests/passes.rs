//! Each bench runs enough passes for the sign test over its pairs to reach
//! significance: with `p` pairs all on one side the smallest two-sided p is
//! `2 * 0.5^p`, so three passes can never go below 0.25 and six reach 0.031.

const ALPHA: f64 = 0.05;

/// The smallest two-sided p a sign test over `pairs` pairs can give.
fn smallest_sign_p(pairs: i32) -> f64 {
    2.0 * 0.5_f64.powi(pairs)
}

/// The `passes` of a bench's `timing` line.
fn passes_of(toml: &str) -> Option<i32> {
    let line = toml
        .lines()
        .find(|l| l.trim_start().starts_with("timing"))?;
    let after = &line[line.find("passes")? + "passes".len() ..];
    let value = after.trim_start().strip_prefix('=')?.trim_start();
    value[.. value.find(|c: char| !c.is_ascii_digit())?]
        .parse()
        .ok()
}

#[test]
fn three_passes_cannot_reach_significance_and_six_can() {
    assert_eq!(smallest_sign_p(3), 0.25);
    assert!(smallest_sign_p(3) >= ALPHA);
    assert!(smallest_sign_p(5) >= ALPHA);
    assert!(smallest_sign_p(6) < ALPHA);
    assert_eq!(
        passes_of("timing = { passes = 3, runs_per_pass = 1 }"),
        Some(3)
    );
    assert_eq!(passes_of("title = \"no timing\""), None);
}

#[test]
fn every_bench_runs_enough_passes_for_its_sign_test_to_reach_significance() {
    for (bench, toml) in [
        (
            "build-static",
            include_str!("../../../build-static/bench.toml"),
        ),
        (
            "link-static",
            include_str!("../../../link-static/bench.toml"),
        ),
    ] {
        let passes = passes_of(toml).unwrap_or_else(|| panic!("{bench} declares no passes"));
        assert!(
            smallest_sign_p(passes) < ALPHA,
            "{bench} runs {passes} passes, whose sign test cannot go below {}",
            smallest_sign_p(passes)
        );
    }
}
