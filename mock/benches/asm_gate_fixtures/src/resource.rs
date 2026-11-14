//! Fixture 6: a resource snapshot read per record in the hot loop.

use super::*;

// ----- fixture 6: resource snapshot in the hot loop (register promotion) -----

// The unit reads a `Resource` scalar per record through the by-value ctx
// snapshot (domain 19): the value is copied into the EngineCtx at projection,
// so the hot loop should keep it register-resident rather than reloading
// canonical storage. Check 1 gates devirtualisation; checks 2-5 surface the
// spill/addressing evidence for the promotion.

#[derive(Copy, Clone, Default)]
struct RIn(u32);
#[derive(Copy, Clone, Default)]
#[allow(dead_code)] // written by the WU, read back post-run as raw u32 for the hash
struct ROut(u32);

type ResRead = Cons<Resource<u32>, Cons<Column<RIn>, Empty>>;

#[derive(Copy, Clone)]
struct ResMix;
impl BuilderInput for ResMix {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for ResMix {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ResRead,
        One<ROut>,
        SnapCons<u32, SnapNil>,
        ColPtrCons<RIn, ColPtrNil>,
        ColPtrCons<ROut, ColPtrNil>,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = ResRead;
    type Write = One<ROut>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let gain: &u32 = ctx.resources().resource();
        let g = *gain;
        ctx.each().run(|i| {
            // SAFETY: RIn host-populated; ROut reserved and exclusively written
            // here over the reserved records.
            let inp = unsafe { ctx.reader().read::<RIn, _>(i) };
            unsafe { ctx.writer().write::<ROut, _>(i, ROut(stage1(inp.0) ^ g)) };
        });
    }
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn asm_gate_resource_snapshot() -> u64 {
    let provider = HeapBump::new(arena_bytes(3, N));
    let mut sched = Scheduler::builder()
        .with(Column::<ROut>::start())
        .with(Column::<RIn>::start())
        .with(Resource::new(0x5EED_u32))
        .with(ResMix)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));
    // Bindings head is the resource (last-registered); RIn one tail down.
    // SAFETY: RIn reserved for N records of repr-u32 values; scheduler alive.
    let rin = sched.__bindings().__tail().__ptr().as_ptr() as *mut u32;
    for i in 0 .. N {
        unsafe { *rin.add(i) = i as u32 };
    }
    let _ = sched.run();
    let rout = sched.__bindings().__tail().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: ROut holds N written records; scheduler alive.
    black_box(unsafe { fnv1a_u32(core::slice::from_raw_parts(rout, N)) })
}
