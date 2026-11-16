//! What the bench tree's hooks library asks of the build and link benches:
//! which cell a sweep is, the routine for it, and the check after it.

use std::path::Path;

use mockspace_bench_core::RoutineBridge;

use crate::{Profile, Scenario, record, root, routine};

/// The benches this crate serves.
pub const BENCHES: [&str; 2] = ["build-static", "link-static"];

/// The scenario and profile of `sweep` in `bench`: `build-static`'s sweeps are
/// its scenarios in the fixture's `dev` profile, `link-static`'s are the
/// fixture's two profiles.
pub fn cell_of(bench: &str, sweep: &str) -> Option<(Scenario, Profile)> {
    match bench {
        "build-static" => {
            Scenario::ALL
                .into_iter()
                .filter(|s| *s != Scenario::Link)
                .find(|s| s.label() == sweep)
                .map(|s| (s, Profile::Dev))
        },
        "link-static" => {
            [Profile::Dev, Profile::Release]
                .into_iter()
                .find(|p| p.label() == sweep)
                .map(|p| (Scenario::Link, p))
        },
        _ => None,
    }
}

/// The routine bridge of a cell of one of [`BENCHES`].
pub fn bridge_for(bench: &str, sweep: &str) -> Option<RoutineBridge> {
    cell_of(bench, sweep).map(|(s, p)| routine::bridge(s, p))
}

/// Empty every record, before a run.
pub fn on_init() {
    for b in BENCHES {
        record::reset(&root(), b);
    }
}

/// After a cell: its record copied into `out_dir` as `calls.tsv`, the record
/// emptied, and every line held to the cell's scenario.
pub fn after_cell(bench: &str, sweep: &str, n: usize, out_dir: &Path) -> Result<String, String> {
    let (scenario, profile) =
        cell_of(bench, sweep).ok_or_else(|| format!("`{bench}` has no sweep `{sweep}`"))?;
    let (lines, text) = record::read(&root(), bench);
    record::reset(&root(), bench);
    std::fs::write(
        out_dir.join("calls.tsv"),
        format!("{}{text}", record::HEADER),
    )
    .map_err(|e| format!("calls.tsv was not written: {e}"))?;
    record::check(&lines, bench, scenario, profile, n)
}
