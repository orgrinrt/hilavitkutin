//! Standing perf gate (#664): the single-core engine must be no worse than an
//! optimal hand-fused std loop.
//!
//! RED until Phase D (#340) lands the two load-bearing mechanisms (dispatch
//! devirtualisation and within-fiber stage fusion); GREEN signals Gate-1 (#661)
//! perf-done. The failing state IS the specification: this is the executable
//! definition of "the single-core engine is complete", per the workspace's
//! strict-by-design discipline.
//!
//! These tests are `#[ignore]` by default for two reasons. They are timing
//! assertions that are EXPECTED red right now, so auto-running them would fail
//! every unrelated `cargo test`. And they are only meaningful under the release
//! profile (fat LTO, codegen-units=1, set in Cargo.toml); a debug build would
//! compare un-optimised arms and report noise. Run the oracle deliberately:
//!
//! ```text
//! cd mock/benches/engine_vs_std
//! caffeinate -dimsu cargo test --release -- --ignored --test-threads=1
//! ```
//!
//! `--test-threads=1` keeps the timed arms off contended cores so the measured
//! ratio is a clean single-core comparison rather than scheduler noise.
//!
//! Checksum equality is asserted first in every test, so a failure is
//! unambiguously "engine slower" (the gate doing its job) and never "the two
//! arms diverged" (a broken bench whose ratio would be meaningless).
//!
//! Two axes:
//!  - RUNTIME (steady-state, process to finish): asserted at every size. The
//!    headline drive-toward-parity gate. Red now (roughly 2x to 5x); green when
//!    fusion removes the intermediate-column traffic and mega-dispatch removes
//!    the per-unit dispatch overhead.
//!  - STARTUP (get ready): asserted only at the largest size, where the
//!    schedule-once design makes startup parity reachable (std re-allocates the
//!    full buffers each get-ready; the engine's plan build is a fixed cost that
//!    beats large allocations at scale). At small sizes the fixed plan-build
//!    cost cannot match two `vec!` calls, and that gap amortises across reused
//!    frames by design, so raw startup is reported by the bench at every size
//!    but not asserted as a forever-red gate Phase D cannot close.

use engine_vs_std::{
    RUNTIME_TOLERANCE, SIZES, STARTUP_TOLERANCE, WORKLOADS, WorkloadMeasure, accumulator,
    branching, element_wise, measure,
};

fn assert_checksum(m: &WorkloadMeasure) {
    assert!(
        m.checksum_ok,
        "workload `{}` N={} produced divergent results (engine={:#x} std={:#x}); the bench is \
         invalid. Fix the workload before reading the perf ratio: a perf comparison of two arms \
         that compute different values is meaningless.",
        m.name, m.n, m.eng_hash, m.std_hash
    );
}

/// Assert the runtime axis for one workload across the full size sweep. Reports
/// the gradient (every size) in the panic message so progress through Phase D
/// shows in the red output, not just the first size that exceeds tolerance.
fn runtime_gate(name: &'static str) {
    let mut report = String::new();
    let mut worst = 0.0f64;
    for &n in &SIZES {
        let m = measure(name, n);
        assert_checksum(&m);
        let r = m.runtime_ratio();
        worst = worst.max(r);
        report.push_str(&format!(
            "  N={n:>8}: runtime {r:.3}x  (engine {} ns, std {} ns)\n",
            m.eng_runtime.median_ns, m.std_runtime.median_ns
        ));
    }
    assert!(
        worst <= RUNTIME_TOLERANCE,
        "RUNTIME gate RED for `{name}` (worst {worst:.3}x > tolerance {RUNTIME_TOLERANCE:.2}x).\n\
         Expected red until Phase D (#340) lands within-fiber fusion + mega-dispatch. This gate \
         turns green at parity and signals Gate-1 (#661) perf-done.\n{report}"
    );
}

#[test]
#[ignore = "perf oracle, red until Phase D (#340); run: cargo test --release -- --ignored"]
fn runtime_element_wise() {
    runtime_gate(element_wise::NAME);
}

#[test]
#[ignore = "perf oracle, red until Phase D (#340); run: cargo test --release -- --ignored"]
fn runtime_branching() {
    runtime_gate(branching::NAME);
}

#[test]
#[ignore = "perf oracle, red until Phase D (#340); run: cargo test --release -- --ignored"]
fn runtime_accumulator() {
    runtime_gate(accumulator::NAME);
}

#[test]
#[ignore = "perf oracle, red until Phase D (#340); run: cargo test --release -- --ignored"]
fn startup_largest_size() {
    let n = *SIZES.iter().max().expect("SIZES is non-empty");
    let mut report = String::new();
    let mut worst = 0.0f64;
    for &name in &WORKLOADS {
        let m = measure(name, n);
        assert_checksum(&m);
        let r = m.startup_ratio();
        worst = worst.max(r);
        report.push_str(&format!(
            "  {name:>12}: startup {r:.3}x  (engine {} ns, std {} ns)\n",
            m.eng_startup.median_ns, m.std_startup.median_ns
        ));
    }
    assert!(
        worst <= STARTUP_TOLERANCE,
        "STARTUP gate RED at N={n} (worst {worst:.3}x > tolerance {STARTUP_TOLERANCE:.2}x).\n\
         At the largest size the schedule-once design predicts startup parity (std re-allocates \
         the full buffers; the engine plan build is fixed cost). Red here means a startup \
         regression, not the expected small-N plan-build cost.\n{report}"
    );
}
