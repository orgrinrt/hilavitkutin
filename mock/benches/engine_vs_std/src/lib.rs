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
}

/// The record counts the reporting bench and the perf gate both sweep.
pub const SIZES: [usize; 3] = [4096, 65536, 1_048_576];

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
        other => panic!("unknown workload {other:?}"),
    }
}

pub const WORKLOADS: [&str; 3] = [element_wise::NAME, branching::NAME, accumulator::NAME];

pub use core::hint::black_box as keep;
