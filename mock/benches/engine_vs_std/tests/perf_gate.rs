//! Standing perf gate (#664): the engine must compare to optimal std AS
//! EXPECTED for each workload, size, and execution mode.
//!
//! This is not a uniform parity target. Each `(workload, size, mode)` carries a
//! design-intent ceiling (`engine_vs_std::expected_ratio`): parity arms gate at
//! ~1.0x, known-loss arms gate at the loss the columnar/dispatch shape is
//! expected to incur (red only if WORSE than expected), and win arms gate below
//! 1.0x so the gate REQUIRES the engine to beat single-threaded std. Red means
//! "did not compare as expected here", which is the whole point of a standing
//! oracle: it stays red until the canonical mechanisms close each arm's gap, and
//! it would also catch a regression that made a met expectation slip.
//!
//! Two axes per workload:
//!  - SINGLE-CORE runtime: `run()` / `run_fused()` vs optimal std.
//!  - PARALLEL runtime: `run_parallel()` vs optimal MULTI-threaded std (idiomatic
//!    `std::thread::scope`, equal core budget), for the multi-trunk workloads the
//!    engine can spread across cores. This is the fair parallel bar: an N-core
//!    engine judged against N-core std, not a serial loop. The report also shows
//!    the raw speedup vs serial std as context.
//!
//! These tests are `#[ignore]` by default: they are timing assertions only
//! meaningful under the release profile (fat LTO, cgu=1, set in Cargo.toml), and
//! several are expected red until later gates land, so auto-running them would
//! fail every unrelated `cargo test`. Run the oracle deliberately:
//!
//! ```text
//! cd mock/benches/engine_vs_std
//! caffeinate -dimsu cargo test --release -- --ignored --test-threads=1
//! ```
//!
//! `--test-threads=1` keeps the single-core timed arms off contended cores. The
//! parallel arms intentionally use the machine's cores via the engine pool; the
//! std arm they compare against is single-threaded by construction.
//!
//! Checksum equality is asserted first in every test, so a failure is
//! unambiguously a perf result and never two arms that diverged.

use engine_vs_std::{
    Mode, SIZES, STARTUP_TOLERANCE, WORKLOADS, WorkloadMeasure, accumulator, branching,
    element_wise, expected_ratio, measure, wide_parallel,
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

/// Assert the single-core runtime axis for one workload across the size sweep,
/// each size against its own design-intent ceiling. Reports the full gradient so
/// progress (and which size regressed) shows in the panic message.
fn runtime_gate(name: &'static str) {
    let mut report = String::new();
    let mut breached = false;
    for &n in &SIZES {
        let m = measure(name, n);
        assert_checksum(&m);
        let r = m.runtime_ratio();
        let exp = expected_ratio(name, n, Mode::SingleCore);
        let ok = r <= exp;
        breached |= !ok;
        report.push_str(&format!(
            "  N={n:>8}: runtime {r:.3}x  (expect <= {exp:.2}x) {}  (engine {} ns, std {} ns)\n",
            if ok { "ok" } else { "RED" },
            m.eng_runtime.median_ns,
            m.std_runtime.median_ns
        ));
    }
    assert!(
        !breached,
        "SINGLE-CORE runtime gate RED for `{name}` (a size exceeded its expected ratio).\n\
         The ceiling per size encodes how the engine is expected to compare on this workload; \
         red means it did not. This is the standing oracle, red until the canonical mechanisms \
         close the gap.\n{report}"
    );
}

/// Assert the parallel runtime axis for one (multi-trunk) workload, each size
/// against its expected ceiling. Skips sizes the workload did not measure
/// parallel (it always does if the arm sets `eng_runtime_par`).
fn parallel_gate(name: &'static str) {
    let mut report = String::new();
    let mut breached = false;
    let mut measured_any = false;
    for &n in &SIZES {
        let m = measure(name, n);
        assert_checksum(&m);
        let Some(r) = m.par_ratio() else { continue };
        measured_any = true;
        let exp = expected_ratio(name, n, Mode::Parallel);
        let ok = r <= exp;
        breached |= !ok;
        let par_ns = m.eng_runtime_par.map(|p| p.median_ns).unwrap_or(0);
        let std_par_ns = m.std_runtime_par.map(|p| p.median_ns).unwrap_or(0);
        let speedup = m.par_speedup_vs_serial().unwrap_or(0.0);
        report.push_str(&format!(
            "  N={n:>8}: parallel {r:.3}x vs std-par  (expect <= {exp:.2}x) {}  \
             (engine {} ns, std-par {} ns; {:.2}x vs serial std)\n",
            if ok { "ok" } else { "RED" },
            par_ns,
            std_par_ns,
            speedup,
        ));
    }
    assert!(
        measured_any,
        "workload `{name}` declared a parallel gate but measured no parallel runtime"
    );
    assert!(
        !breached,
        "PARALLEL runtime gate RED for `{name}` (a size exceeded its expected ratio).\n\
         The ratio is the multi-threaded engine against OPTIMAL multi-threaded std (equal \
         core budgets); a ceiling at or below 1.0x requires the engine to match or beat \
         parallel std, red means it did not.\n{report}"
    );
}

#[test]
#[ignore = "perf oracle; run: cargo test --release -- --ignored"]
fn runtime_element_wise() {
    runtime_gate(element_wise::NAME);
}

#[test]
#[ignore = "perf oracle; run: cargo test --release -- --ignored"]
fn runtime_branching() {
    runtime_gate(branching::NAME);
}

#[test]
#[ignore = "perf oracle; run: cargo test --release -- --ignored"]
fn runtime_accumulator() {
    runtime_gate(accumulator::NAME);
}

#[test]
#[ignore = "perf oracle; run: cargo test --release -- --ignored"]
fn runtime_wide_parallel() {
    runtime_gate(wide_parallel::NAME);
}

#[test]
#[ignore = "perf oracle, the multi-threaded win path; run: cargo test --release -- --ignored"]
fn parallel_wide_parallel() {
    parallel_gate(wide_parallel::NAME);
}

#[test]
#[ignore = "perf oracle; run: cargo test --release -- --ignored"]
fn parallel_accumulator() {
    parallel_gate(accumulator::NAME);
}

#[test]
#[ignore = "perf oracle; run: cargo test --release -- --ignored"]
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
            "  {name:>14}: startup {r:.3}x  (engine {} ns, std {} ns)\n",
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
