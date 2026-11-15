//! The design's ceilings applied to the medians of one timed cell.
//!
//! The hooks library of the bench tree takes the median of `algo_ns` (the
//! timed region alone) per arm in the `warm` mode of every `engine_vs_std`
//! cell it runs, and holds the engine's median over std's to the ceiling
//! `expected` names:
//!
//! 1. `evs_engine` over `evs_std`, against the workload's single-core ceiling.
//! 2. `evs_engine_par` over `evs_std_par`, against its parallel ceiling, where the
//!    section times both. The bar for a threaded engine is threaded std on the
//!    same cores, never a serial loop.
//! 3. `evs_engine_startup` over `evs_std_startup`, against `STARTUP_TOLERANCE`.
//!
//! A red judgement marks the remaining gap to the design, which is what the
//! oracles were for; it is recorded beside the run, not a failed run.

use std::collections::BTreeMap;

use crate::{Mode, STARTUP_TOLERANCE, Workload, expected_ratio};

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
            "evs_engine",
            "evs_std",
            expected_ratio(workload.name(), n, Mode::SingleCore),
        ),
        (
            "engine_par / std_par",
            "evs_engine_par",
            "evs_std_par",
            expected_ratio(workload.name(), n, Mode::Parallel),
        ),
        (
            "engine_startup / std_startup",
            "evs_engine_startup",
            "evs_std_startup",
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
            &arms(&[("evs_engine", ceiling), ("evs_std", 1.0)]),
        );
        assert_eq!(at.len(), 1);
        assert!(at[0].green());
        let over = judge(
            Workload::Branching,
            4096,
            &arms(&[("evs_engine", ceiling * 1.01), ("evs_std", 1.0)]),
        );
        assert!(!over[0].green());
    }

    #[test]
    fn the_threaded_engine_is_judged_against_threaded_std_only() {
        let j = judge(
            Workload::WideParallel,
            65536,
            &arms(&[
                ("evs_engine", 1.0),
                ("evs_std", 1.0),
                ("evs_engine_par", 1.0),
                ("evs_std_par", 4.0),
            ]),
        );
        let par = j.iter().find(|j| j.pair == "engine_par / std_par").unwrap();
        assert_eq!(par.ratio, 0.25);
        assert_eq!(j.len(), 2);
    }

    #[test]
    fn a_pair_missing_an_arm_is_not_judged() {
        assert!(judge(Workload::ElementWise, 4096, &arms(&[("evs_engine", 1.0)])).is_empty());
        let startup = judge(
            Workload::ElementWise,
            4194304,
            &arms(&[("evs_engine_startup", 3.0), ("evs_std_startup", 1.0)]),
        );
        assert_eq!(startup.len(), 1);
        assert_eq!(startup[0].ceiling, STARTUP_TOLERANCE);
        assert!(!startup[0].green());
    }
}
