//! Shared harness for the single-core engine-vs-std comparison (#660) and the
//! standing perf gate (#664).
//!
//! The engine's complete single-core design targets parity-or-better against an
//! optimal hand-fused std loop (0.95x to 1.02x; see
//! `mock/research/202606052000_single-core-engine-ideal-vs-actual-audit.md`).
//! The two mechanisms that earn that parity (dispatch devirtualisation and
//! within-fiber stage fusion) live in the unbuilt Phase D (#340), so the engine
//! is currently slower than std on every workload here. This crate measures the
//! gap (the `main` reporting bench) and asserts against it (the `tests/perf_gate`
//! oracle, red until #340 lands).
//!
//! Three workload shapes form a gradient rather than a cliff, so the gates show
//! progress through Phase D mechanism by mechanism:
//!
//! 1. `element_wise`: a four-stage RAW chain. Pure fusion territory: std keeps
//!    the three intermediates in registers, the engine materialises four
//!    columns. The widest gap, closed by within-fiber fusion (mechanism 2).
//! 2. `branching`: two independent transforms over the same input joined by a
//!    third. A multi-fiber DAG that exercises dispatch across fibers, closed by
//!    mega-dispatch devirtualisation (mechanism 1) plus fusion.
//! 3. `accumulator`: one transform feeding the append surface. Exercises the
//!    accumulator dispatch path (unit-outer) against an optimal std buffer-fill.
//!
//! Every workload validates an FNV-1a checksum equality between the two arms
//! OUTSIDE the timed region, so a measured ratio is only ever a comparison of
//! two arms proven to compute the identical result. A red checksum means the
//! workload itself is invalid, not that the engine is slow.
//!
//! std and alloc are allowed here: this is a standalone bench workspace outside
//! the no_std / no_alloc mock crates.

use core::cell::Cell;
use core::mem::MaybeUninit;
use std::time::Instant;

use arvo::USize;
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_providers::ArenaColumnStorage;

pub mod accumulator;
pub mod branching;
pub mod element_wise;
pub mod wide_parallel;

// ----- the design's parity target, expressed as gate tolerances -----

/// Runtime tolerance: `engine_runtime <= std_runtime * RUNTIME_TOLERANCE`.
///
/// The design target is 0.95x to 1.02x; the slack absorbs measurement noise so
/// a met goal does not flake. This is the headline gate: red now (2x to 5x),
/// green when within-fiber fusion + mega-dispatch land in Phase D (#340).
pub const RUNTIME_TOLERANCE: f64 = 1.10;

/// Startup tolerance for the largest workload, where the schedule-once design
/// makes startup parity genuinely reachable (std re-allocates the full buffers
/// each get-ready; the engine's plan build is a fixed cost). At small record
/// counts the engine's fixed plan-build cost cannot match two `vec!` calls, and
/// that gap amortises across reused frames by design, so raw startup is
/// reported at every size but asserted only at the largest. See the perf-gate
/// test module for the full reasoning.
pub const STARTUP_TOLERANCE: f64 = 1.10;

// ----- workload constants (shared across arms and workloads) -----

pub const M1: u32 = 2654435761; // Knuth multiplicative hash
pub const M2: u32 = 2246822519;
pub const SH: u32 = 13;
pub const M4: u32 = 3266489917;

#[inline(always)]
pub fn stage1(i: u32) -> u32 {
    i.wrapping_mul(M1)
}
#[inline(always)]
pub fn stage2(a: u32) -> u32 {
    a.wrapping_mul(M2).wrapping_add(1)
}
#[inline(always)]
pub fn stage3(b: u32) -> u32 {
    (b >> SH) ^ b
}
#[inline(always)]
pub fn stage4(c: u32) -> u32 {
    c.wrapping_mul(M4)
}

/// The full element-wise chain, shared by the element-wise and accumulator
/// workloads so both arms of each compute provably identical values.
#[inline(always)]
pub fn chain(seed: u32) -> u32 {
    stage4(stage3(stage2(stage1(seed))))
}

/// Heavy per-record kernel: the four-stage chain iterated `HEAVY_ROUNDS` times.
/// Win-path workloads use it so real per-record compute dominates dispatch, and
/// the engine's parallel spread (not the dispatch overhead) is what the bench
/// measures. Shared across arms so both compute provably identical values.
pub const HEAVY_ROUNDS: usize = 8;
#[inline(always)]
pub fn heavy(seed: u32) -> u32 {
    let mut x = seed;
    let mut r = 0;
    while r < HEAVY_ROUNDS {
        x = chain(x);
        r += 1;
    }
    x
}

// ----- FNV-1a over the u32 output (the cross-arm validity check) -----

#[inline(always)]
fn fnv1a(acc: u64, byte: u8) -> u64 {
    (acc ^ byte as u64).wrapping_mul(0x0000_0100_0000_01B3)
}

pub fn fnv1a_u32_slice(vals: &[u32]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for v in vals {
        for b in v.to_le_bytes() {
            h = fnv1a(h, b);
        }
    }
    h
}

// ----- heap-backed bump provider (the workload is too big for the stack) -----

pub struct HeapBump {
    base: *mut u8,
    cap: usize,
    used: Cell<usize>,
    // Owns the backing allocation; `base` points into it. The heap block does
    // not move when the Box is moved into the struct, so `base` stays valid.
    _buf: Box<[MaybeUninit<u8>]>,
}
impl HeapBump {
    pub fn new(bytes: usize) -> Self {
        let mut buf: Box<[MaybeUninit<u8>]> = vec![MaybeUninit::uninit(); bytes].into_boxed_slice();
        let base = buf.as_mut_ptr() as *mut u8;
        Self { base, cap: bytes, used: Cell::new(0), _buf: buf }
    }
}
unsafe impl Send for HeapBump {}
unsafe impl Sync for HeapBump {}
impl MemoryProviderApi for HeapBump {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let used = self.used.get();
        let align = align.0.max(1);
        let aligned = (used + align - 1) / align * align;
        if aligned + len.0 > self.cap {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        unsafe { self.base.add(aligned) }
    }
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: arvo::Bool, _write: arvo::Bool) {}
}

pub fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

/// Bytes the arena needs for `columns` typed `u32` stores over `n` records,
/// plus 64-byte alignment slack per column and headroom for the plan's own
/// stored columns.
pub fn arena_bytes(columns: usize, n: usize) -> usize {
    columns * n * 4 + columns * 64 + (1 << 16)
}

// ----- timing -----

#[derive(Copy, Clone)]
pub struct Stat {
    pub median_ns: u128,
    pub min_ns: u128,
}

pub fn bench<F: FnMut()>(warmup: usize, iters: usize, mut f: F) -> Stat {
    for _ in 0..warmup {
        f();
    }
    let mut samples = Vec::with_capacity(iters);
    for _ in 0..iters {
        let t = Instant::now();
        f();
        samples.push(t.elapsed().as_nanos());
    }
    samples.sort_unstable();
    Stat { median_ns: samples[samples.len() / 2], min_ns: samples[0] }
}

pub fn iters_for(n: usize) -> usize {
    (10_000_000 / n).clamp(20, 4000)
}

// ----- one workload's measured result -----

pub struct WorkloadMeasure {
    pub name: &'static str,
    pub n: usize,
    pub eng_startup: Stat,
    pub std_startup: Stat,
    pub eng_runtime: Stat,
    pub std_runtime: Stat,
    /// Multi-threaded engine runtime (`run_parallel`), `Some` only for
    /// workloads with multiple trunks the engine can spread across cores.
    /// Single-fiber / unit-outer arms leave it `None` (no trunk parallelism to
    /// measure).
    pub eng_runtime_par: Option<Stat>,
    /// Optimal multi-threaded std runtime: the same workload parallelised with
    /// idiomatic `std::thread::scope` across `std_threads()` threads, byte-
    /// identical output. This is the FAIR bar for the parallel engine arm (N-core
    /// engine vs N-core std), `Some` exactly when `eng_runtime_par` is.
    pub std_runtime_par: Option<Stat>,
    pub checksum_ok: bool,
    pub eng_hash: u64,
    pub std_hash: u64,
}

impl WorkloadMeasure {
    pub fn startup_ratio(&self) -> f64 {
        self.eng_startup.median_ns as f64 / self.std_startup.median_ns.max(1) as f64
    }
    pub fn runtime_ratio(&self) -> f64 {
        self.eng_runtime.median_ns as f64 / self.std_runtime.median_ns.max(1) as f64
    }
    /// The FAIR parallel ratio: multi-threaded engine vs optimal multi-threaded
    /// std (both across `std_threads()` cores). `None` when the workload was not
    /// measured parallel, or has no std-parallel baseline. This is what the
    /// parallel gate asserts: a parallel engine must be judged against parallel
    /// std, not a single-threaded loop.
    pub fn par_ratio(&self) -> Option<f64> {
        match (self.eng_runtime_par, self.std_runtime_par) {
            (Some(e), Some(s)) => Some(e.median_ns as f64 / s.median_ns.max(1) as f64),
            _ => None,
        }
    }

    /// Context only (not gated): multi-threaded engine vs single-threaded std,
    /// i.e. the raw speedup the engine's parallelism buys over a serial loop.
    /// Useful in the report to show the absolute speedup alongside the fair
    /// engine-vs-parallel-std ratio.
    pub fn par_speedup_vs_serial(&self) -> Option<f64> {
        self.eng_runtime_par
            .map(|p| p.median_ns as f64 / self.std_runtime.median_ns.max(1) as f64)
    }
}

/// Thread count for the optimal multi-threaded std baselines. Matches the
/// engine's `OsThreadPool` worker count (the machine's available parallelism) so
/// the parallel arms compare equal core budgets.
pub fn std_threads() -> usize {
    std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1)
}

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
pub fn expected_ratio(name: &'static str, n: usize, mode: Mode) -> f64 {
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
        // (std::thread::scope chunk-fill across equal cores), not a serial loop.
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
        (wide_parallel::NAME, Mode::Parallel) => 1.10,

        // Unknown pairing: default to parity so a new arm cannot silently pass.
        _ => 1.10,
    }
}

/// The record counts the reporting bench and the perf gate both sweep. Spans
/// from cache-resident small N (where the engine pays fixed overhead and is
/// expected to lose or tie) to multi-megabyte N (where columnar layout and,
/// for multi-trunk workloads, parallel spread are expected to win).
pub const SIZES: [usize; 4] = [4096, 65536, 1_048_576, 4_194_304];

/// Run one workload at one size with a warmup-and-iters budget derived from the
/// size. Each workload module supplies its own `measure`; this dispatches by
/// name so callers iterate a single list.
pub fn measure(name: &'static str, n: usize) -> WorkloadMeasure {
    let iters = iters_for(n);
    let warmup = (iters / 10).max(3);
    match name {
        element_wise::NAME => element_wise::measure(n, warmup, iters),
        branching::NAME => branching::measure(n, warmup, iters),
        accumulator::NAME => accumulator::measure(n, warmup, iters),
        wide_parallel::NAME => wide_parallel::measure(n, warmup, iters),
        other => panic!("unknown workload {other:?}"),
    }
}

pub const WORKLOADS: [&str; 4] =
    [element_wise::NAME, branching::NAME, accumulator::NAME, wide_parallel::NAME];

pub use core::hint::black_box as keep;
