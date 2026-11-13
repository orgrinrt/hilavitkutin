//! What the design expects of each workload: the ceiling on the engine's
//! runtime over std's, per workload and execution mode, and the startup
//! ceiling at the largest size.
//!
//! These were the assertions of the ignored `tests/perf_gate.rs` oracles. A
//! timing assertion means something only over a release build timed under the
//! bench harness, so the ceilings now live here and the driver's `--verdict`
//! applies them to the medians a harness run wrote, cell by cell. A red cell
//! marks the remaining gap to the design, which is the oracle's purpose: the
//! engine as it stands lacks fusion and the adaptive trunk, fiber and morsel
//! scheduling, and nothing here decides anything about it.

use crate::{accumulator, branching, element_wise, wide_parallel};

/// Startup ceiling at the largest size, where the schedule-once design makes
/// startup parity reachable: std allocates the full buffers on every
/// get-ready, the engine's plan build is a fixed cost. At small record counts
/// the plan build cannot match a few `vec!` calls and the gap amortises across
/// reused frames by design, so the startup cells are configured at the largest
/// size only.
pub const STARTUP_TOLERANCE: f64 = 1.10;

/// Execution mode a gate expectation applies to.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Mode {
    /// Single-core `run()` / `run_fused()`.
    SingleCore,
    /// Multi-threaded `run_parallel()`.
    Parallel,
}

/// The maximum engine/std runtime ratio the design expects for `(workload, n,
/// mode)`. The gate is red when the measured ratio exceeds this. This encodes
/// the per-workload performance contract, not a uniform parity target: a value
/// above 1.0 tolerates an expected loss (a workload whose columnar/dispatch
/// shape the design does not win at this scale), and a value at or below 1.0
/// REQUIRES a win (the engine must beat optimal single-threaded std by that
/// factor).
///
/// Calibration: ceilings are the design ASPIRATION (where the arm lands when the
/// complete design reaches it), so red marks the remaining gap, per the standing
/// oracle. Pinned against a release run (Apple-silicon, 2026-06-07; see the
/// reporting bench output committed alongside). Observed runtime ratios that
/// firing, for reference:
///   element_wise   1.00 / 1.04 / 0.99 / 0.88   (single-core, crosses to a win)
///   wide_parallel  1.01 / 1.01 / 1.01 / 1.02   single-core
///   wide_parallel  0.77 / 0.80 / 1.54 / 0.86   parallel vs OPTIMAL PARALLEL std
///                                               (win except a 1M dip; 2026-06-08)
///   branching      1.58 / 2.32 / 2.69 / 3.61   single-core (loss, grows with N)
///   accumulator    6.30 / 6.27 / 6.21 / 4.11   single-core (loss)
///   accumulator    parallel vs OPTIMAL PARALLEL std: passes ~parity (2026-06-08)
/// at N = 4096 / 65536 / 1048576 / 4194304. The parallel rows are the
/// multi-threaded engine against optimal multi-threaded std (equal cores), the
/// fair bar; earlier parallel rows compared against a single-threaded loop.
pub fn expected_ratio(name: &str, _n: usize, mode: Mode) -> f64 {
    match (name, mode) {
        // Deep single fiber, within-fiber fusion: spec parity target. Already
        // green and crossing to a win at scale. Single-core only (one trunk).
        (element_wise::NAME, Mode::SingleCore) => 1.10,

        // Fan-in diamond: the engine materialises a branch output the fully
        // fused std keeps in registers (an honest columnar cost that grows with
        // N). Within-fiber fan-in fusion (co-locating one branch with the join)
        // can eliminate one of the two intermediates; the design aspiration is
        // to stay under ~2x. Green at small N (the loss is small there), red at
        // scale until that fusion lands.
        (branching::NAME, Mode::SingleCore) => 2.0,

        // Accumulator append, unit-outer dispatch vs an optimal buffer fill. The
        // gap is dispatch + per-record append accounting, not memory; the
        // compiled per-core dispatch aspiration is near parity. Red until then.
        (accumulator::NAME, Mode::SingleCore) => 1.30,

        // Accumulator, multi-threaded (deviation 9: per-core record-range split +
        // merge). The threaded path splits the single unit-outer trunk's records
        // across cores; measured it cuts the single-core loss sharply at scale
        // (2026-06-07: 5.54x -> 1.71x at 1M, 4.90x -> 1.39x at 4M) while small N
        // is dominated by the per-frame pool publish/barrier on trivial work. The
        // design end-state pairs this with adaptive single-vs-parallel selection
        // (small N falls back to single-core) and compiled per-core dispatch, so
        // the aspiration is single-core-parity at small N (adapt fallback) and
        // parity at scale (the record-split paying off against the memory-bound
        // std fill). Red until those later gates land; the gradient already shows
        // the threaded path moving the arm the right direction.
        // Recalibrated 2026-06-08: the bar is now optimal MULTI-threaded std
        // (chunk-fill across equal cores), not a serial loop. Since 2026-11-13 its
        // workers are a `Crew` parked between frames, as the engine's are; the
        // 1.10 below was measured against threads spawned per frame and is kept,
        // so a red here is the parked bar being the stronger one.
        // Both the engine deviation-9 path and parallel std are memory-bound at
        // scale; measured at parity, so the ceiling is parity within run-to-run
        // noise (1.10x). A real regression (engine clearly slower than parallel
        // std) trips it; ±8% measurement variance does not.
        (accumulator::NAME, Mode::Parallel) => 1.10,

        // Wide independent heavy trunks: the win path. Single-core is near
        // parity (heavy work amortises the per-trunk dispatch). Multi-threaded
        // WINS increasingly with N; the sub-1.0 ceilings make the gate REQUIRE
        // the win. Tiny N stays red: the per-frame barrier (main-orchestrated,
        // deviation 4) dominates, and the worker-side barrier is the fix.
        (wide_parallel::NAME, Mode::SingleCore) => 1.10,
        // Recalibrated 2026-06-08 against optimal MULTI-threaded std (K threads,
        // one chain each, matching the engine's K-trunk spread). The old sub-1.0
        // ceilings compared an N-core engine to a 1-thread loop, which flattered
        // the engine; the honest bar is parity vs parallel std. Measured at
        // parity: it wins at the extremes (0.73-0.78x at 4K and 4M, where std
        // respawns scope threads / work dominates) and sits within ~±8% noise at
        // 64K-1M (runs straddle 1.0x). Ceiling is parity within that noise
        // (1.10x); a real regression beyond noise trips it.
        // The bar's threads were spawned per frame then; since 2026-11-13 they
        // are a `Crew` parked between frames, and the ceiling is kept as it was.
        (wide_parallel::NAME, Mode::Parallel) => 1.10,

        // Unknown pairing: default to parity so a new arm cannot silently pass.
        _ => 1.10,
    }
}
