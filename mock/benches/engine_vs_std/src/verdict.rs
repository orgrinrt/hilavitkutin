//! The design's ceilings applied to the medians a harness run promoted.
//!
//! For every section and size in `bench.toml` it reads
//! `<results>/<section>/<section>_n<N>.csv`, takes the median of `algo_ns`
//! (the timed region alone) per arm in the `warm` mode, and holds the
//! engine's median over std's to the ceiling `evs_support::expected` names:
//!
//! 1. `engine` over `std`, against the workload's single-core ceiling.
//! 2. `engine_par` over `std_par`, against its parallel ceiling, where the
//!    section times both. The bar for a threaded engine is threaded std on the
//!    same cores, never a serial loop.
//! 3. `engine_startup` over `std_startup`, against `STARTUP_TOLERANCE`.
//!
//! A cell whose CSV is absent was not run and is said so; it is neither red
//! nor green. The answer is false when any judged cell is red, which is what
//! the oracles were for: red marks the remaining gap to the design.

use std::collections::BTreeMap;
use std::path::Path;

use evs_support::{Mode, STARTUP_TOLERANCE, Workload, expected_ratio};
use mockspace_bench_harness::{BenchManifest, load_samples_csv};

/// One ratio held to one ceiling.
#[derive(Debug, PartialEq)]
pub struct Judgement {
    /// What over what: `engine / std` and the like.
    pub pair:    &'static str,
    pub ratio:   f64,
    pub ceiling: f64,
}

impl Judgement {
    pub fn green(&self) -> bool {
        self.ratio <= self.ceiling
    }
}

/// The pairs a cell is judged on, given the median of each arm it timed.
///
/// Only pairs whose two arms are both present are judged, so a section lists
/// the arms it means to compare and the judgement follows the manifest.
pub fn judge(workload: Workload, n: usize, medians: &BTreeMap<String, f64>) -> Vec<Judgement> {
    let pairs: [(&'static str, &str, &str, f64); 3] = [
        (
            "engine / std",
            "engine",
            "std",
            expected_ratio(workload.name(), n, Mode::SingleCore),
        ),
        (
            "engine_par / std_par",
            "engine_par",
            "std_par",
            expected_ratio(workload.name(), n, Mode::Parallel),
        ),
        (
            "engine_startup / std_startup",
            "engine_startup",
            "std_startup",
            STARTUP_TOLERANCE,
        ),
    ];
    pairs
        .into_iter()
        .filter_map(|(pair, num, den, ceiling)| {
            let (a, b) = (medians.get(num)?, medians.get(den)?);
            Some(Judgement {
                pair,
                ratio: a / b.max(f64::MIN_POSITIVE),
                ceiling,
            })
        })
        .collect()
}

/// The median `algo_ns` of every arm in the `warm` mode of one CSV.
fn medians(csv: &Path) -> Result<BTreeMap<String, f64>, String> {
    let samples = load_samples_csv(csv).map_err(|e| format!("{}: {e}", csv.display()))?;
    let mut by_arm: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    for s in samples.into_iter().filter(|s| s.mode == "warm") {
        by_arm.entry(s.variant).or_default().push(s.algo_ns);
    }
    Ok(by_arm
        .into_iter()
        .map(|(arm, ns)| (arm, mockspace_bench_harness::analysis::median(&ns)))
        .collect())
}

/// Judge every configured cell that has results, print one line per pair,
/// and answer whether every judged pair is green.
pub fn run(manifest: &Path, results: &Path) -> Result<bool, String> {
    let manifest = BenchManifest::load(manifest).map_err(|e| e.to_string())?;
    let member_dir = std::env::current_dir().map_err(|e| e.to_string())?;
    let (mut judged, mut red) = (0usize, 0usize);
    for bench in manifest.bench_names() {
        let sizes = manifest.bench[&bench].sizes.len();
        for idx in 0 .. sizes {
            let config = manifest
                .for_size(&bench, idx, &member_dir)
                .map_err(|e| e.to_string())?;
            let workload = Workload::from_name(&config.workload).ok_or_else(|| {
                format!("section `{bench}` names no workload: `{}`", config.workload)
            })?;
            let n = config.n;
            let csv = results.join(&bench).join(format!("{bench}_n{n}.csv"));
            if !csv.exists() {
                println!("{bench} n={n}: not run");
                continue;
            }
            for j in judge(workload, n, &medians(&csv)?) {
                judged += 1;
                let colour = if j.green() { "green" } else { "RED" };
                if !j.green() {
                    red += 1;
                }
                println!(
                    "{bench} n={n}: {} = {:.3}, ceiling {:.2}, {colour}",
                    j.pair, j.ratio, j.ceiling
                );
            }
        }
    }
    if judged == 0 {
        return Err(format!(
            "no cell under {} has results to judge; run the bench first",
            results.display()
        ));
    }
    println!("{judged} judged, {red} red");
    Ok(red == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arms(pairs: &[(&str, f64)]) -> BTreeMap<String, f64> {
        pairs.iter().map(|(a, ns)| (a.to_string(), *ns)).collect()
    }

    #[test]
    fn a_ratio_at_or_under_its_ceiling_is_green_and_over_it_red() {
        let ceiling = expected_ratio(Workload::Branching.name(), 4096, Mode::SingleCore);
        let at = judge(
            Workload::Branching,
            4096,
            &arms(&[("engine", ceiling), ("std", 1.0)]),
        );
        assert_eq!(at.len(), 1);
        assert!(at[0].green());
        let over = judge(
            Workload::Branching,
            4096,
            &arms(&[("engine", ceiling * 1.01), ("std", 1.0)]),
        );
        assert!(!over[0].green());
    }

    #[test]
    fn the_threaded_engine_is_judged_against_threaded_std_only() {
        let j = judge(
            Workload::WideParallel,
            65536,
            &arms(&[("engine", 1.0), ("std", 1.0), ("engine_par", 1.0), ("std_par", 4.0)]),
        );
        let par = j.iter().find(|j| j.pair == "engine_par / std_par").unwrap();
        assert_eq!(par.ratio, 0.25);
        assert_eq!(j.len(), 2);
    }

    #[test]
    fn a_pair_missing_an_arm_is_not_judged() {
        assert!(judge(Workload::ElementWise, 4096, &arms(&[("engine", 1.0)])).is_empty());
        let startup = judge(
            Workload::ElementWise,
            4194304,
            &arms(&[("engine_startup", 3.0), ("std_startup", 1.0)]),
        );
        assert_eq!(startup.len(), 1);
        assert_eq!(startup[0].ceiling, STARTUP_TOLERANCE);
        assert!(!startup[0].green());
    }
}
