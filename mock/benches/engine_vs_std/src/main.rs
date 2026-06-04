//! #660 macro bench: the single-core hilavitkutin engine vs an optimal
//! hand-fused std loop, on a heavy multi-stage columnar workload.
//!
//! Op directive (2026-06-04): before any multi-threaded work, check whether
//! the engine running single-core beats the same workload written on a std
//! base as optimally as possible, on STARTUP (get ready) and RUNTIME (process
//! to finish). If it does not, surface the inefficiency.
//!
//! WORKLOAD: four RAW-chained element-wise stages over N records.
//!   S1: A[i] = (i as u32).wrapping_mul(2654435761)
//!   S2: B[i] = A[i].wrapping_mul(2246822519).wrapping_add(1)
//!   S3: C[i] = (B[i] >> 13) ^ B[i]
//!   S4: D[i] = C[i].wrapping_mul(3266489917)
//! FNV-1a over D validates the two arms compute the identical result; it runs
//! OUTSIDE the timed region in both arms.
//!
//! ENGINE arm: S1..S4 as four WorkUnits writing Column<Av..Dv>; the plan adds
//! RAW edges so dispatch runs them in order; `run()` processes the frame. The
//! engine materialises all four intermediate columns (its staged design),
//! windowed per morsel to stay cache-resident.
//!
//! STD arm: one fused loop keeping A/B/C in registers, writing only D to a
//! pre-allocated output array. This is the optimal fused shape op asked for;
//! it materialises only the output.
//!
//! The headline comparison is the engine-vs-std DELTA. The engine is expected
//! to pay for intermediate-column materialisation + dispatch/morsel machinery
//! that the fused loop avoids by register-fusing; the magnitude of that gap is
//! the finding. Numbers are u32 in both arms so the delta isolates engine
//! machinery from the arvo-vs-bare-numeric question (a follow-up can re-run
//! the engine arm with arvo `Uint<32>` columns to test repr-transparency).
//!
//! Run: `caffeinate -dimsu cargo run --release` from this directory (darwin
//! pinning; release is opt3/lto-fat/cgu1 per Cargo.toml).

use core::cell::Cell;
use core::hint::black_box;
use core::mem::MaybeUninit;
use std::time::Instant;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, PtrNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

// ----- workload constants (shared by both arms) -----
const M1: u32 = 2654435761; // Knuth multiplicative hash
const M2: u32 = 2246822519;
const SH: u32 = 13;
const M4: u32 = 3266489917;

#[inline(always)]
fn stage1(i: u32) -> u32 {
    i.wrapping_mul(M1)
}
#[inline(always)]
fn stage2(a: u32) -> u32 {
    a.wrapping_mul(M2).wrapping_add(1)
}
#[inline(always)]
fn stage3(b: u32) -> u32 {
    (b >> SH) ^ b
}
#[inline(always)]
fn stage4(c: u32) -> u32 {
    c.wrapping_mul(M4)
}

#[inline(always)]
fn fnv1a(acc: u64, byte: u8) -> u64 {
    (acc ^ byte as u64).wrapping_mul(0x0000_0100_0000_01B3)
}

fn fnv1a_u32_slice(vals: &[u32]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for v in vals {
        for b in v.to_le_bytes() {
            h = fnv1a(h, b);
        }
    }
    h
}

// ----- column value newtypes (Copy over u32 -> blanket ColumnValue) -----
// Input column: the host pre-populates In[i] = i (the global record index)
// before the frame, per the engine's input-vs-accumulator model. WorkUnits
// transform element-wise; they never synthesise a value from the record's
// global position (ctx.each yields morsel-relative indices, so the position
// must arrive as pre-populated input data, not be computed in a WU body).
#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Av(u32);
#[derive(Copy, Clone)]
struct Bv(u32);
#[derive(Copy, Clone)]
struct Cv(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)] // written by S4, read back post-run as raw u32 for the hash
struct Dv(u32);

type One<T> = Cons<Column<T>, Empty>;

// ----- the four pipeline stages -----
struct S1;
impl BuilderInput for S1 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S1 {
    type Read = One<Inv>;
    type Write = One<Av>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Inv>, One<Av>, PtrNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Av, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In was host-populated for the record count; read adds the
            // morsel start, so this is In[global i]. Av reserved + exclusively
            // written here; the morsel covers only reserved records.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Av, _>(i, Av(stage1(inp.0))) };
        });
    }
}

struct S2;
impl BuilderInput for S2 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S2 {
    type Read = One<Av>;
    type Write = One<Bv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Av>, One<Bv>, PtrNil, ColPtrCons<Av, ColPtrNil>, ColPtrCons<Bv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: S1 (ordered before by the RAW edge) wrote every record;
            // exclusive writer of Bv over reserved records.
            let a = unsafe { ctx.reader().read::<Av, _>(i) };
            unsafe { ctx.writer().write::<Bv, _>(i, Bv(stage2(a.0))) };
        });
    }
}

struct S3;
impl BuilderInput for S3 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S3 {
    type Read = One<Bv>;
    type Write = One<Cv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Bv>, One<Cv>, PtrNil, ColPtrCons<Bv, ColPtrNil>, ColPtrCons<Cv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let b = unsafe { ctx.reader().read::<Bv, _>(i) };
            unsafe { ctx.writer().write::<Cv, _>(i, Cv(stage3(b.0))) };
        });
    }
}

struct S4;
impl BuilderInput for S4 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S4 {
    type Read = One<Cv>;
    type Write = One<Dv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Cv>, One<Dv>, PtrNil, ColPtrCons<Cv, ColPtrNil>, ColPtrCons<Dv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let c = unsafe { ctx.reader().read::<Cv, _>(i) };
            unsafe { ctx.writer().write::<Dv, _>(i, Dv(stage4(c.0))) };
        });
    }
}

// ----- heap-backed bump provider (the workload is too big for the stack) -----
struct HeapBump {
    base: *mut u8,
    cap: usize,
    used: Cell<usize>,
    // Owns the backing allocation; `base` points into it. The heap block does
    // not move when the Box is moved into the struct, so `base` stays valid.
    _buf: Box<[MaybeUninit<u8>]>,
}
impl HeapBump {
    fn new(bytes: usize) -> Self {
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

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// Bytes the arena needs: 5 columns (In + Av..Dv) * N * 4 bytes, plus 64-byte
// alignment slack per column and headroom for the plan's own stored columns.
fn arena_bytes(n: usize) -> usize {
    5 * n * 4 + 5 * 64 + (1 << 16)
}

// Build a fresh 4-stage scheduler over N records. Columns are registered
// Dv,Cv,Bv,Av,In so the bindings chain (prepend) is In(head) -> Av -> Bv ->
// Cv -> Dv: In at the head is populated via `__ptr()`, Dv at depth 4 is read
// back via `__tail()` x4.
fn build_engine(n: usize) -> impl FnMut() {
    move || {
        let provider = HeapBump::new(arena_bytes(n));
        let scheduler = Scheduler::builder()
            .with(Column::<Dv>::new())
            .with(Column::<Cv>::new())
            .with(Column::<Bv>::new())
            .with(Column::<Av>::new())
            .with(Column::<Inv>::new())
            .with(S1)
            .with(S2)
            .with(S3)
            .with(S4)
            .build(store(provider), USize(n))
            .unwrap_or_else(|_| panic!("engine build should succeed"));
        black_box(&scheduler);
    }
}

// ----- timing -----
struct Stat {
    median_ns: u128,
    min_ns: u128,
}
fn bench<F: FnMut()>(warmup: usize, iters: usize, mut f: F) -> Stat {
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

fn iters_for(n: usize) -> usize {
    (10_000_000 / n).clamp(20, 4000)
}

fn main() {
    let sizes = [4096usize, 65536, 1_048_576];
    println!("# engine_vs_std (#660): single-core engine vs optimal fused std");
    println!("# N, engine_startup_ns(med/min), std_startup_ns(med/min), engine_runtime_ns(med/min), std_runtime_ns(med/min), startup_ratio, runtime_ratio, checksum_ok");

    for &n in &sizes {
        let iters = iters_for(n);
        let warmup = (iters / 10).max(3);

        // ----- startup -----
        let eng_startup = bench(warmup, iters, build_engine(n));
        let std_startup = bench(warmup, iters, || {
            // std "get ready": allocate the input + output buffers the fused
            // loop reads/writes (the A/B/C intermediates live in registers).
            let in_buf: Vec<u32> = vec![0u32; n];
            let d: Vec<u32> = vec![0u32; n];
            black_box(&in_buf);
            black_box(&d);
        });

        // ----- runtime: engine (build once, run many) -----
        let provider = HeapBump::new(arena_bytes(n));
        let mut sched = Scheduler::builder()
            .with(Column::<Dv>::new())
            .with(Column::<Cv>::new())
            .with(Column::<Bv>::new())
            .with(Column::<Av>::new())
            .with(Column::<Inv>::new())
            .with(S1)
            .with(S2)
            .with(S3)
            .with(S4)
            .build(store(provider), USize(n))
            .unwrap_or_else(|_| panic!("engine build should succeed"));
        // Host-populate the input column In[i] = i (the global record index)
        // before the frame. In is the bindings head (last-registered column).
        // SAFETY: In's buffer was reserved for N records of Inv (repr u32); the
        // scheduler (hence the arena) is alive; each reserved slot is written
        // exactly once.
        let in_base = sched.__bindings().__ptr().as_ptr() as *mut Inv;
        for i in 0..n {
            unsafe { *in_base.add(i) = Inv(i as u32) };
        }
        let eng_runtime = bench(warmup, iters, || {
            let r = sched.run();
            black_box(&r);
        });
        // Validate: read the final Dv column (chain depth 4, In->Av->Bv->Cv->Dv)
        // and hash it (outside timing).
        let _ = sched.run();
        let dv_base = sched
            .__bindings()
            .__tail()
            .__tail()
            .__tail()
            .__tail()
            .__ptr()
            .as_ptr();
        let eng_hash = {
            // SAFETY: Dv is the deepest registered column; the buffer holds N
            // reserved records and the scheduler (hence storage) is alive.
            let slice = unsafe { core::slice::from_raw_parts(dv_base as *const u32, n) };
            fnv1a_u32_slice(slice)
        };

        // ----- runtime: std (alloc + fill input once, fused loop many) -----
        // Mirror the engine: read the seed from a pre-populated input buffer
        // (in_buf[i] = i), so both arms do the same input load. A/B/C stay in
        // registers; only D is materialised.
        let in_buf: Vec<u32> = (0..n as u32).collect();
        let mut d_out: Vec<u32> = vec![0u32; n];
        let std_runtime = bench(warmup, iters, || {
            // Zip the two slices so the bounds checks elide and LLVM can
            // autovectorise: the optimal fused single pass op asked for.
            // (Indexing `in_buf[i]`/`d_out[i]` over `0..n` does NOT prove
            // `n == len`, leaving per-iter bounds checks the engine's unchecked
            // column writes avoid; that would unfairly handicap the std arm.)
            for (d, &inv) in d_out.iter_mut().zip(in_buf.iter()) {
                let a = stage1(inv);
                let b = stage2(a);
                let c = stage3(b);
                *d = stage4(c);
            }
            black_box(&d_out);
        });
        let std_hash = fnv1a_u32_slice(&d_out);

        let su_ratio = eng_startup.median_ns as f64 / std_startup.median_ns.max(1) as f64;
        let rt_ratio = eng_runtime.median_ns as f64 / std_runtime.median_ns.max(1) as f64;
        let ok = eng_hash == std_hash;
        println!(
            "{n}, {}/{}, {}/{}, {}/{}, {}/{}, {su_ratio:.3}, {rt_ratio:.3}, {ok}",
            eng_startup.median_ns,
            eng_startup.min_ns,
            std_startup.median_ns,
            std_startup.min_ns,
            eng_runtime.median_ns,
            eng_runtime.min_ns,
            std_runtime.median_ns,
            std_runtime.min_ns,
        );
        if !ok {
            eprintln!("CHECKSUM MISMATCH at N={n}: engine={eng_hash:#x} std={std_hash:#x}");
        }
    }
}
