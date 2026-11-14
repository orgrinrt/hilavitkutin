//! Fixtures 5 and 7: two fibers with per-fiber windows, through `run` and through `run_parallel`.

use super::*;

// ----- fixture 5: two-fiber windowed dispatch via run() (A2b fiber-outer) -----

// Two disjoint two-stage chains land in two fibers; the inverted `run` walks
// each fiber's own window sequence. The gate asserts the fiber-outer loop
// reintroduced no indirect call.

#[derive(Copy, Clone, Default)]
struct PIn(u32);
#[derive(Copy, Clone, Default)]
struct PMid(u32);
#[derive(Copy, Clone, Default)]
#[allow(dead_code)] // written by P2, read back post-run as raw u32 for the hash
struct POut(u32);
#[derive(Copy, Clone, Default)]
struct QIn(u32);
#[derive(Copy, Clone, Default)]
#[allow(dead_code)] // written by Q1, read back post-run as raw u32 for the hash
struct QOut(u32);

#[derive(Copy, Clone)]
struct P1;
impl BuilderInput for P1 {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for P1 {
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<PIn>,
        One<PMid>,
        SnapNil,
        ColPtrCons<PIn, ColPtrNil>,
        ColPtrCons<PMid, ColPtrNil>,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = One<PIn>;
    type Write = One<PMid>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: PIn host-populated for the record count; PMid reserved and
            // exclusively written here over the reserved records.
            let inp = unsafe { ctx.reader().read::<PIn, _>(i) };
            unsafe { ctx.writer().write::<PMid, _>(i, PMid(stage1(inp.0))) };
        });
    }
}

#[derive(Copy, Clone)]
struct P2;
impl BuilderInput for P2 {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for P2 {
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<PMid>,
        One<POut>,
        SnapNil,
        ColPtrCons<PMid, ColPtrNil>,
        ColPtrCons<POut, ColPtrNil>,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = One<PMid>;
    type Write = One<POut>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let m = unsafe { ctx.reader().read::<PMid, _>(i) };
            unsafe { ctx.writer().write::<POut, _>(i, POut(stage2(m.0))) };
        });
    }
}

#[derive(Copy, Clone)]
struct Q1;
impl BuilderInput for Q1 {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Q1 {
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<QIn>,
        One<QOut>,
        SnapNil,
        ColPtrCons<QIn, ColPtrNil>,
        ColPtrCons<QOut, ColPtrNil>,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = One<QIn>;
    type Write = One<QOut>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let inp = unsafe { ctx.reader().read::<QIn, _>(i) };
            unsafe { ctx.writer().write::<QOut, _>(i, QOut(stage3(inp.0))) };
        });
    }
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn asm_gate_windowed_fibers() -> u64 {
    let provider = HeapBump::new(arena_bytes(5, N));
    let mut sched = Scheduler::builder()
        .with(Column::<POut>::start())
        .with(Column::<PMid>::start())
        .with(Column::<PIn>::start())
        .with(Column::<QOut>::start())
        .with(Column::<QIn>::start())
        .with(P1)
        .with(P2)
        .with(Q1)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));
    // Bindings head is the last-registered store: QIn(0) <- QOut(1) <- PIn(2)
    // <- PMid(3) <- POut(4).
    // SAFETY: inputs reserved for N records of repr-u32 values; scheduler
    // alive; each reserved slot written once.
    let qin = sched.__bindings().__ptr().as_ptr() as *mut u32;
    let pin = sched.__bindings().__tail().__tail().__ptr().as_ptr() as *mut u32;
    for i in 0 .. N {
        unsafe {
            *qin.add(i) = i as u32;
            *pin.add(i) = (i as u32).wrapping_mul(3);
        }
    }
    let _ = sched.run();
    let qout = sched.__bindings().__tail().__ptr().as_ptr() as *const u32;
    let pout = sched
        .__bindings()
        .__tail()
        .__tail()
        .__tail()
        .__tail()
        .__ptr()
        .as_ptr() as *const u32;
    // SAFETY: both output columns hold N written records; scheduler alive.
    let h1 = unsafe { fnv1a_u32(core::slice::from_raw_parts(pout, N)) };
    let h2 = unsafe { fnv1a_u32(core::slice::from_raw_parts(qout, N)) };
    black_box(h1 ^ h2)
}

// ----- fixture 7: per-fiber windows on the parallel path (A4) -----

// Same two-chain carrier as fixture 5, driven through `run_parallel` instead of
// `run`, so the A4 per-fiber window loop compiles and runs under the gate build.
//
// The per-record work runs in `run_core_phase` and `worker_main`, which inline
// into the closure the pool spawns, and std runs that closure inside
// `__rust_begin_short_backtrace::<{closure}>`. That instantiation is the worker
// body, and the gate finds it by its mangled name and checks it beside this
// wrapper, so the parallel walk is scanned and not only the scheduler side.
//
// The executor goes through `OutOfLineSpawn`, whose constructor and `spawn` are
// kept out of line. Inlined, they pull std into this wrapper: the constructor's
// `available_parallelism` walks a table of cgroup quota readers and drops
// `io::Error`s through their vtables, and `std::thread::spawn` adds a `dlsym`
// lookup. Those are std's own calls through stored pointers, not the engine's
// dispatch, and the gate found all seven of them. Out of line, this wrapper
// holds the scheduler side of `run_parallel` and the readback.
#[unsafe(no_mangle)]
pub extern "C" fn asm_gate_parallel_fiber_windows() -> u64 {
    let provider = HeapBump::new(arena_bytes(5, N));
    let sched = Scheduler::builder()
        .with(Column::<POut>::start())
        .with(Column::<PMid>::start())
        .with(Column::<PIn>::start())
        .with(Column::<QOut>::start())
        .with(Column::<QIn>::start())
        .with(P1)
        .with(P2)
        .with(Q1)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));
    // Bindings head is the last-registered store: QIn(0) <- QOut(1) <- PIn(2)
    // <- PMid(3) <- POut(4).
    // SAFETY: inputs reserved for N records of repr-u32 values; scheduler
    // alive; each reserved slot written once.
    let qin = sched.__bindings().__ptr().as_ptr() as *mut u32;
    let pin = sched.__bindings().__tail().__tail().__ptr().as_ptr() as *mut u32;
    for i in 0 .. N {
        unsafe {
            *qin.add(i) = i as u32;
            *pin.add(i) = (i as u32).wrapping_mul(3);
        }
    }
    let pool = OutOfLineSpawn::new();
    let mut sched = core::pin::pin!(sched);
    let _ = sched.as_mut().run_parallel(&pool);
    let qout = sched.as_ref().__bindings().__tail().__ptr().as_ptr() as *const u32;
    let pout = sched
        .as_ref()
        .__bindings()
        .__tail()
        .__tail()
        .__tail()
        .__tail()
        .__ptr()
        .as_ptr() as *const u32;
    // SAFETY: both output columns hold N written records; scheduler alive.
    let h1 = unsafe { fnv1a_u32(core::slice::from_raw_parts(pout, N)) };
    let h2 = unsafe { fnv1a_u32(core::slice::from_raw_parts(qout, N)) };
    black_box(h1 ^ h2)
}

/// The bench executor with its `spawn` kept out of line, so the thread
/// creation std does there stays out of the fixture wrapper the gate checks.
struct OutOfLineSpawn(evs_support::executor::BenchExecutor);

impl OutOfLineSpawn {
    /// The bench executor sized to the machine, built out of line.
    #[inline(never)]
    fn new() -> Self {
        Self(evs_support::executor::BenchExecutor::new())
    }
}

impl ThreadPoolApi for OutOfLineSpawn {
    #[inline(never)]
    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.0.spawn(f);
    }

    fn worker_count(&self) -> USize {
        self.0.worker_count()
    }
}
