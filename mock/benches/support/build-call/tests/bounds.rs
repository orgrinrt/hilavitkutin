//! The bound on one call and the disk budget of a cell, each read from what
//! earlier runs measured: a reading at its own point is the measured value, a
//! reading never shrinks as the point grows, and every sweep the two benches
//! declare has a bound and a budget at every point it runs.

use build_call::hooks::cell_of;
use build_call::measured::{FIRST_CALL_S, Row, TARGET_BYTES, call_key, scaled};
use build_call::routine::{BOUND_FACTOR, bound_us};
use build_call::space::budget;
use build_fixture::POINTS;

const S: u64 = 1_000_000;

#[test]
fn a_reading_at_its_own_point_is_the_value_measured_there() {
    for &(key, at, value) in FIRST_CALL_S.iter().chain(TARGET_BYTES.iter()) {
        let table: &[Row] = if FIRST_CALL_S.contains(&(key, at, value)) {
            &FIRST_CALL_S
        } else {
            &TARGET_BYTES
        };
        assert_eq!(scaled(table, key, at), Some(value), "{key:#x} at {at}");
    }
    for &(key, at, secs) in &FIRST_CALL_S {
        let (scenario, profile) = ((key >> 8) as u8, (key & 0xFF) as u8);
        assert_eq!(
            bound_us(scenario, profile, at),
            Some(BOUND_FACTOR * secs * S)
        );
    }
}

#[test]
fn a_reading_is_carried_up_by_the_crate_count_and_rounded_up_from_the_nearest_point_below() {
    let rows: [Row; 2] = [(7, 25, 17), (7, 97, 40)];
    // 17 at 25 is 65.96 at 97 by the count, but 97 has its own row
    assert_eq!(scaled(&rows, 7, 97), Some(40));
    // 40 at 97 is 69.69 at 169, rounded up
    assert_eq!(scaled(&rows, 7, 169), Some(70));
    // 17 at 25 is 33.32 at 49, rounded up
    assert_eq!(scaled(&rows, 7, 49), Some(34));
    assert_eq!(scaled(&rows, 7, 24), None, "below every measured point");
    assert_eq!(scaled(&rows, 8, 97), None, "a key with no row");
}

#[test]
fn no_reading_shrinks_as_the_point_grows() {
    for table in [&FIRST_CALL_S[..], &TARGET_BYTES[..]] {
        for &(key, ..) in table {
            let readings: Vec<Option<u64>> =
                POINTS.iter().map(|&n| scaled(table, key, n)).collect();
            assert!(
                readings.windows(2).all(|w| w[0] <= w[1]),
                "{key:#x}: {readings:?}"
            );
        }
    }
}

#[test]
fn every_declared_sweep_has_a_bound_and_a_budget_at_every_point_it_runs() {
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
        let sweeps = sweeps_of(toml);
        assert!(!sweeps.is_empty(), "{bench} declares no sweep");
        for (sweep, points) in sweeps {
            let (scenario, profile) = cell_of(bench, &sweep)
                .unwrap_or_else(|| panic!("{bench} sweep `{sweep}` has no cell"));
            for n in points {
                assert!(
                    bound_us(scenario.code(), profile.code(), n).is_some(),
                    "{bench} {sweep} n={n} has no bound and would run under the harness's fallback"
                );
                for incremental in [false, true] {
                    assert!(budget(n, incremental) > 0, "{bench} {sweep} n={n}");
                }
            }
        }
    }
    assert!(
        !FIRST_CALL_S.iter().any(|(k, ..)| *k == call_key(9, 0)),
        "the key below names no row"
    );
    assert_eq!(
        bound_us(9, 0, 25),
        None,
        "a cell with no row of its own is not bounded by another's"
    );
}

/// The `[sweep.<name>]` tables of a bench's `bench.toml` and their `points`.
fn sweeps_of(toml: &str) -> Vec<(String, Vec<usize>)> {
    let mut out: Vec<(String, Vec<usize>)> = Vec::new();
    for line in toml.lines().map(str::trim) {
        if let Some(name) = line
            .strip_prefix("[sweep.")
            .and_then(|r| r.strip_suffix(']'))
        {
            out.push((name.to_string(), Vec::new()));
        } else if let (Some(list), Some(last)) = (line.strip_prefix("points = ["), out.last_mut()) {
            last.1 = list
                .trim_end_matches(']')
                .split(',')
                .filter_map(|p| p.trim().parse().ok())
                .collect();
        }
    }
    out
}
