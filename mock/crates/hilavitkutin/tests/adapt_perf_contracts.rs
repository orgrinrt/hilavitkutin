//! Adapt subsystem behavioural contracts: the make-or-break performance
//! properties.
//!
//! The whole point of the adapt subsystem is that adaptation IMPROVES
//! performance and never degrades it. An adaptive scheduler that loses to its
//! own non-adaptive baseline is worse than no adaptation, because the runtime
//! sampling + re-selection cost buys nothing. These contracts formalise that,
//! per the catalogue-edge-cases-as-tests discipline: the cases are defined NOW,
//! ahead of the machinery, so the heuristics are built against the contract
//! rather than retrofitted to whatever they happen to do.
//!
//! Each contract is `#[ignore]`d (catalogued-red): discoverable via `cargo test
//! -- --ignored`, non-blocking on the gate, and it FAILS on run until the
//! machinery lands (the adapt on/off toggle `StandardAdaptKit` vs `OffAdaptKit`,
//! `select_adapt_config`, and the actuation paths). The body documents the exact
//! comparison each test will assert; when the machinery exists, the body is
//! filled with the real comparative measurement and the `#[ignore]` removed. The
//! contract (the name + doc + the comparison) is the durable artefact; it is not
//! softened to pass today.
//!
//! Shared shape: run a workload twice on the same input, once with adaptation
//! OFF (baseline) and once with adaptation ON (converged over enough frames),
//! and compare. Tolerances and workload shapes are named per contract. Tracked
//! by #341 (adapt subsystem implementation).

/// CONTRACT: EMA adaptation never degrades a balanced workload.
///
/// On a workload already balanced across phases and cores, the adaptive run must
/// be no slower than the non-adaptive baseline: `adapted_time <= baseline_time *
/// (1 + EPS)` for a small EPS that covers sampling jitter. Adaptation that loses
/// on the easy case is disqualifying: it means the sampling + re-selection
/// overhead is not amortised.
#[test]
#[ignore = "catalogue: adapt non-degradation on balanced workloads; needs the adapt on/off toggle; tracked #341"]
fn ema_adaptation_never_degrades_balanced_workload() {
    unimplemented!(
        "contract: adapted_time <= baseline_time * (1 + EPS) on a phase/core-balanced \
         workload. Fill when StandardAdaptKit vs OffAdaptKit + a measurement harness land. \
         tracked #341"
    );
}

/// CONTRACT: EMA adaptation improves an imbalanced workload (the core value).
///
/// On a workload with a skewed per-phase / per-core load, the adaptive run must
/// converge to a strictly faster steady state than the baseline:
/// `adapted_steady_time < baseline_time`. If this does not hold the subsystem
/// fails its entire purpose.
#[test]
#[ignore = "catalogue: adapt improves imbalanced workloads (core value prop); needs select_adapt_config; tracked #341"]
fn ema_adaptation_improves_imbalanced_workload() {
    unimplemented!(
        "contract: adapted steady-state time < baseline time on a load-skewed workload. \
         Fill when select_adapt_config + actuation land. tracked #341"
    );
}

/// CONTRACT: `select_adapt_config` targets the bottleneck phase.
///
/// After re-selection, the bottleneck phase's `phase_ema` is lower than before
/// re-selection: the tuning targets the slow phase, not an arbitrary one.
#[test]
#[ignore = "catalogue: select_adapt_config reduces the bottleneck phase_ema; needs select_adapt_config; tracked #341"]
fn select_adapt_config_reduces_bottleneck_phase_ema() {
    unimplemented!(
        "contract: bottleneck phase_ema after re-selection < before. Fill when \
         select_adapt_config lands. tracked #341"
    );
}

/// CONTRACT: tier-1 morsel re-chunk reduces worst-core idle.
///
/// Re-chunking an imbalanced morsel distribution lowers `idle_ns` (the
/// core-balance signal) without raising total frame time.
#[test]
#[ignore = "catalogue: morsel re-chunk lowers idle_ns without raising total time; needs tier-1 re-chunk; tracked #341"]
fn morsel_rechunk_reduces_idle_ns() {
    unimplemented!(
        "contract: idle_ns after re-chunk < before, total_time not raised. Fill when the \
         tier-1 morsel re-chunk actuation lands. tracked #341"
    );
}

/// CONTRACT: strategy reselect never regresses throughput.
///
/// Switching a phase's strategy between frames leaves throughput non-decreasing
/// at steady state (a reselect must not trade a faster phase for a slower whole).
#[test]
#[ignore = "catalogue: strategy reselect leaves throughput non-decreasing; needs domain-14 strategy plan-shaping + reselect; tracked #341"]
fn strategy_reselect_never_regresses_throughput() {
    unimplemented!(
        "contract: steady-state throughput after a strategy reselect >= before. Fill when \
         strategy plan-shaping + between-frame reselect land. tracked #341"
    );
}

/// CONTRACT: predictive parking cuts wasted wait without adding latency.
///
/// The wait-tier selection (spin / backoff / park by predicted wait) lowers
/// spin/park waste versus a fixed strategy without raising frame latency.
#[test]
#[ignore = "catalogue: predictive parking cuts wait waste without added latency; needs the wait-tier actuation; tracked #341"]
fn predictive_parking_reduces_idle_without_added_latency() {
    unimplemented!(
        "contract: wasted wait after predictive parking < fixed-strategy, frame latency not \
         raised. Fill when the predicted-wait tier selection lands. tracked #341"
    );
}

/// CONTRACT: P/E-core routing improves throughput on heterogeneous cores.
///
/// Routing hot work to performance cores beats uniform routing on a machine with
/// a P/E split. Guarded off on uniform-core hosts (no split to exploit).
#[test]
#[ignore = "catalogue: P/E-core routing beats uniform routing on a P/E machine; needs morsel-temperature + CoreClass routing; tracked #341"]
fn pe_core_routing_improves_throughput_on_hot_work() {
    unimplemented!(
        "contract: throughput with P-core routing > uniform routing on a P/E host; equal on \
         a uniform-core host. Fill when temperature -> CoreClass routing lands. tracked #341"
    );
}
