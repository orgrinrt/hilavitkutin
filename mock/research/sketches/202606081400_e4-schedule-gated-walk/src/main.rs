//! E4 slice 1 sketch: schedule-gated dispatch walk (round 202606081200).
//!
//! Hypothesis: the const-gated cons walk can recover each element's `Schedule`
//! (Always vs On<V>) and branch the run-gate at COMPILE TIME (Always -> run;
//! On<V> -> run iff V's fired flag is set), over a carrier mixing both, with no
//! runtime type dispatch. The engine's `RunFiber for WuCons<W, Tail>` bounds
//! `W: WorkUnit` (= WorkUnit<Always>), so On<V> WUs are currently undispatchable;
//! this proves the trait shape that lifts that bound.
//!
//! Crux: how the walk recovers a UNIQUE Schedule per element. `W: WorkUnit<S>`
//! with S a free inferred param is ambiguous (a type could impl several). The
//! clean answer proven here is a companion associated-type trait `Scheduled {
//! type Sched }` impl'd once per WU, so the walk reads `<W as Scheduled>::Sched`
//! unambiguously and dispatches the gate on it.
//!
//! FiredSet here is deliberately minimal (a per-virtual bit). The exact domain-10
//! semantics (per-(virtual,consumer) bits, epoch-based reset, clear-on-dispatch,
//! same-pass-vs-next-pass) is a src-CL design detail, orthogonal to the trait
//! mechanism this sketch de-risks. Outcome in FINDINGS.md.

use core::marker::PhantomData;
use std::cell::Cell;

// ---- Schedule markers (mirror hilavitkutin-api Always / On<V>) ----
struct Always;
struct On<V>(PhantomData<V>);

// ---- Virtuals carry a const INDEX (mirror domain-10 T::INDEX const-fold) ----
trait Virtual {
    const INDEX: usize;
}
struct Tick;
impl Virtual for Tick {
    const INDEX: usize = 0;
}
struct Tock;
impl Virtual for Tock {
    const INDEX: usize = 1;
}

// ---- Fired-flag store (minimal; epoch/clear semantics deferred to src CL) ----
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

// ---- ScheduleGate: compile-time dispatched run decision per Schedule ----
// Always::should_run const-folds to `true` (the if vanishes); On<V>::should_run
// is a flag read. The dispatch is static (monomorphised per Sched), no vtable.
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

// ---- WU + companion Scheduled trait naming the WU's Schedule unambiguously ----
trait Wu {
    fn run(&self, fired: &FiredSet);
}
trait Scheduled {
    type Sched: ScheduleGate;
}

// ---- Cons carrier + the schedule-gated walk ----
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
impl<W, Tail> RunGated for WuCons<W, Tail>
where
    W: Wu + Scheduled,
    Tail: RunGated,
{
    #[inline]
    fn run_gated(&self, fired: &FiredSet) {
        // Compile-time branch on the head's Schedule; On<V> reads the flag.
        if <<W as Scheduled>::Sched as ScheduleGate>::should_run(fired) {
            self.head.run(fired);
        }
        self.tail.run_gated(fired);
    }
}

// ---- Concrete WUs: Always (also fires Tick), On<Tick>, On<Tock> ----
struct Producer {
    ran: Cell<u32>,
}
impl Wu for Producer {
    fn run(&self, fired: &FiredSet) {
        self.ran.set(self.ran.get() + 1);
        fired.fire::<Tick>();
    }
}
impl Scheduled for Producer {
    type Sched = Always;
}

struct OnTick {
    ran: Cell<u32>,
}
impl Wu for OnTick {
    fn run(&self, _: &FiredSet) {
        self.ran.set(self.ran.get() + 1);
    }
}
impl Scheduled for OnTick {
    type Sched = On<Tick>;
}

struct OnTock {
    ran: Cell<u32>,
}
impl Wu for OnTock {
    fn run(&self, _: &FiredSet) {
        self.ran.set(self.ran.get() + 1);
    }
}
impl Scheduled for OnTock {
    type Sched = On<Tock>;
}

fn main() {
    // Carrier order: Producer (fires Tick) -> OnTick (gated Tick) -> OnTock
    // (gated Tock, never fired). Producer precedes OnTick, so same-pass: by the
    // time the walk reaches OnTick, Tick is set.
    let carrier = WuCons {
        head: Producer { ran: Cell::new(0) },
        tail: WuCons {
            head: OnTick { ran: Cell::new(0) },
            tail: WuCons { head: OnTock { ran: Cell::new(0) }, tail: WuNil },
        },
    };
    let fired = FiredSet::new();
    carrier.run_gated(&fired);

    let producer_ran = carrier.head.ran.get();
    let ontick_ran = carrier.tail.head.ran.get();
    let ontock_ran = carrier.tail.tail.head.ran.get();
    println!("producer={producer_ran} ontick={ontick_ran} ontock={ontock_ran}");
    assert_eq!(producer_ran, 1, "Always WU always runs");
    assert_eq!(ontick_ran, 1, "On<Tick> runs: Producer fired Tick before it in the walk");
    assert_eq!(ontock_ran, 0, "On<Tock> does NOT run: Tock never fired");
    println!("WORKS: schedule recovered per element, gate branched at compile time");
}
