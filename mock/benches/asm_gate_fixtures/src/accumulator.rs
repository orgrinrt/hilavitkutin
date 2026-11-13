//! Fixture 3: an accumulator appender through `run`, the unit-outer path that is never gated.

use super::*;

// ----- fixture 3: accumulator appender via run() (unit-outer never-gated path) -----

#[derive(Copy, Clone)]
struct AInv(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)] // appended by the WU, read back post-run as raw u32 for the hash
struct AAv(u32);

type AccW = Cons<Accum<AAv>, Empty>;

struct AppendChain;
impl BuilderInput for AppendChain {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for AppendChain {
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<AInv>,
        AccW,
        SnapNil,
        ColPtrCons<AInv, ColPtrNil>,
        ColPtrNil,
        AccPtrCons<'frame, AAv, AccPtrNil>,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = One<AInv>;
    type Write = AccW;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated; append advances live-length under the
            // reserved capacity (= record count).
            let inp = unsafe { ctx.reader().read::<AInv, _>(i) };
            unsafe { ctx.accums().append::<AAv, _>(AAv(chain(inp.0))) };
        });
    }
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn asm_gate_accumulator() -> u64 {
    let provider = HeapBump::new(arena_bytes(2, N));
    let mut sched = Scheduler::builder()
        .with(Column::<AInv>::new())
        .with(Accum::<AAv>::new())
        .with(AppendChain)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));
    // Av accumulator is the head; AInv column is one tail down.
    // SAFETY: In reserved for N records of AInv (repr u32); scheduler alive.
    let in_base = sched.__bindings().__tail().__ptr().as_ptr() as *mut AInv;
    for i in 0 .. N {
        unsafe { *in_base.add(i) = AInv(i as u32) };
    }
    let _ = sched.run();
    let len = sched.__bindings().__len_cell().get().0;
    let av_base = sched.__bindings().__ptr().as_ptr() as *const u32;
    // SAFETY: the appender wrote `len` records into the Av buffer; scheduler alive.
    let slice = unsafe { core::slice::from_raw_parts(av_base, len) };
    black_box(fnv1a_u32_slice(slice))
}
