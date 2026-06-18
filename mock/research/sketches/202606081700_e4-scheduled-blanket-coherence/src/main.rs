//! E4 slice-1 api-integration sketch: blanket `Scheduled` coherence.
//!
//! Question: with `WorkUnit<Schedule = Always>` (default param, per-Schedule
//! associated types), can a BLANKET `impl<W: WorkUnit<Always>> Scheduled for W`
//! coexist with an EXPLICIT `impl Scheduled for SomeOnVWu` (where that WU impls
//! `WorkUnit<On<V>>`, not `WorkUnit<Always>`)? If yes, every existing Always WU
//! gets `Scheduled` for free (zero churn) and only On<V> WUs add the one-line
//! explicit impl. If the blanket conflicts with the explicit (coherence error),
//! the src CL must instead require an explicit `Scheduled` on EVERY WU (churn).
//!
//! Models the real hilavitkutin-api WorkUnit shape faithfully: a default
//! Schedule type param, the associated types hanging off `WorkUnit<S>`, and the
//! dispatch carrier as a WuCons/WuNil walk. Outcome in FINDINGS.md.

use core::marker::PhantomData;
use std::cell::Cell;

// ---- Schedule markers (mirror hilavitkutin-api Always / On<V>) ----
struct Always;
struct On<V>(PhantomData<V>);

// ---- Virtual marker carrying a const index (mirror domain-10 T::INDEX) ----
trait Virtual {
    const INDEX: usize;
}
struct Tick;
impl Virtual for Tick {
    const INDEX: usize = 0;
}

// ---- WorkUnit<Schedule = Always>, associated type per-Schedule (faithful) ----
trait WorkUnit<Schedule = Always> {
    fn run(&self, fired: &FiredSet);
}

// ---- The proposed api additions ----
trait ScheduleGate {
    fn should_run(fired: &FiredSet) -> bool;
}
impl ScheduleGate for Always {
    #[inline]
    fn should_run(_: &FiredSet) -> bool {
        true
    }
}
impl<V: Virtual> ScheduleGate for On<V> {
    #[inline]
    fn should_run(fired: &FiredSet) -> bool {
        fired.is_set::<V>()
    }
}

trait Scheduled {
    type Sched: ScheduleGate;
}

// THE COHERENCE QUESTION: blanket for the default-schedule case ...
impl<W: WorkUnit<Always>> Scheduled for W {
    type Sched = Always;
}
// ... must coexist with an explicit impl for an On<V> WU below. The blanket
// only fires for types satisfying `WorkUnit<Always>`; an On<V> WU impls
// `WorkUnit<On<V>>` and NOT `WorkUnit<Always>`, so the two impls must not
// overlap. This file compiling is the proof.

// ---- Fired-flag store (minimal; domain-10 epoch deferred to src CL) ----
const MAX_VIRTUALS: usize = 8;
struct FiredSet {
    flag: [Cell<bool>; MAX_VIRTUALS],
}
impl FiredSet {
    fn new() -> Self {
        Self { flag: [const { Cell::new(false) }; MAX_VIRTUALS] }
    }
    fn fire<V: Virtual>(&self) {
        self.flag[V::INDEX].set(true);
    }
    fn is_set<V: Virtual>(&self) -> bool {
        self.flag[V::INDEX].get()
    }
}

// ---- Two real WUs: an Always producer (fires Tick), an On<Tick> consumer ----
struct Producer {
    ran: Cell<u32>,
}
impl WorkUnit for Producer {
    // = WorkUnit<Always>
    fn run(&self, fired: &FiredSet) {
        self.ran.set(self.ran.get() + 1);
        fired.fire::<Tick>();
    }
}
// Producer gets `Scheduled` FROM THE BLANKET. No explicit impl here.

struct OnTick {
    ran: Cell<u32>,
}
impl WorkUnit<On<Tick>> for OnTick {
    fn run(&self, _: &FiredSet) {
        self.ran.set(self.ran.get() + 1);
    }
}
// OnTick is NOT `WorkUnit<Always>`, so the blanket does not cover it: explicit.
impl Scheduled for OnTick {
    type Sched = On<Tick>;
}

// ---- Carrier walk recovering each element's Sched + gating ----
struct WuNil;
struct WuCons<W, Tail> {
    head: W,
    tail: Tail,
}

trait RunGated {
    fn run_gated(&self, fired: &FiredSet);
}
impl RunGated for WuNil {
    #[inline]
    fn run_gated(&self, _: &FiredSet) {}
}
// The carrier element bound: each WU must (a) be runnable under its own
// Schedule, (b) name its Sched via Scheduled. The blanket supplies (b) for
// Always WUs, the explicit impl for On<V> WUs. The walk does not care which.
// The real shape: recover Sched via Scheduled, then use it as the WorkUnit
// schedule param so `run` resolves. No free S param (that is unconstrained).
impl<W, Tail> RunGated for WuCons<W, Tail>
where
    W: Scheduled + WorkUnit<<W as Scheduled>::Sched>,
    Tail: RunGated,
{
    #[inline]
    fn run_gated(&self, fired: &FiredSet) {
        if <<W as Scheduled>::Sched as ScheduleGate>::should_run(fired) {
            self.head.run(fired);
        }
        self.tail.run_gated(fired);
    }
}

fn main() {
    // Producer (Always, blanket Scheduled, fires Tick) precedes OnTick
    // (On<Tick>, explicit Scheduled). Same-pass: by the time the walk reaches
    // OnTick, Tick is set.
    let carrier = WuCons {
        head: Producer { ran: Cell::new(0) },
        tail: WuCons { head: OnTick { ran: Cell::new(0) }, tail: WuNil },
    };
    let fired = FiredSet::new();
    carrier.run_gated(&fired);

    let producer_ran = carrier.head.ran.get();
    let ontick_ran = carrier.tail.head.ran.get();
    println!("producer={producer_ran} ontick={ontick_ran}");
    assert_eq!(producer_ran, 1, "Always WU (blanket Scheduled) runs");
    assert_eq!(ontick_ran, 1, "On<Tick> WU (explicit Scheduled) runs: Tick fired before it");
    println!("WORKS: blanket Scheduled for WorkUnit<Always> coexists with explicit On<V> impl");
}
