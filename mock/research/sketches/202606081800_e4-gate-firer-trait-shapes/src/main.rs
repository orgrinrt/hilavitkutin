//! E4 slice-1 trait-shape de-risk: virtual firing (firer) + On<V> dispatch gate.
//!
//! Hypothesis: the engine can gate a heterogeneous Always/On<V> carrier and fire
//! virtuals without specialization and without E0207, reusing the codebase's
//! witness-list-inference idiom. Two mechanisms share one V-keyed cell:
//!
//!   * FIRER: a producer's `W` writes `Virtual<V>`; `VirtualProject` mirrors
//!     `AccumProject` to pull the `&Cell` stamp into a bundle, `VirtualSelector`
//!     (inferred index) fires it. Method-generic index inference, like `append`.
//!   * GATE: an `On<V>` consumer's `W` does NOT contain `V`; the gate resolves
//!     the SAME cell from the full bindings `A`. `GateWith<A, GI>` dispatches on
//!     the unit's `Sched` (Always vs On<V>); `GI` is destructured from a parallel
//!     per-unit witness list (constrained, not a free impl param), and infers at
//!     the call site with no turbofish.
//!
//! The two reach the same `Cell` by identity (both index into the same binding
//! nodes), so a fire is observed by the gate. No global virtual index.
//!
//! Outcome: see FINDINGS.md.

use core::cell::Cell;
use core::marker::PhantomData;

// ---- peano position witnesses (mirror engine_ctx Here/There) ----
struct Here;
struct There<I>(PhantomData<I>);

// ---- access-set cons list (mirror hilavitkutin-api access) ----
struct Cons<H, T>(PhantomData<(H, T)>);
struct Nil;

// ---- store marker (mirror Virtual<T>) ----
struct Virtual<T>(PhantomData<T>);

// ---- schedule markers (mirror hilavitkutin-api work_unit) ----
#[derive(Default)]
struct Always;
struct On<V>(PhantomData<V>);

// ---- HasSchedule: blanket for Always units, explicit for On units ----
trait ScheduleGate {}
impl ScheduleGate for Always {}
impl<V> ScheduleGate for On<V> {}

trait WorkUnit<Schedule = Always> {
    fn exec(&self);
}

trait HasSchedule {
    type Sched: ScheduleGate;
}
// Blanket: every Always WU gains HasSchedule with zero churn.
impl<W: WorkUnit<Always>> HasSchedule for W {
    type Sched = Always;
}

// ---- binding nodes (mixed kinds, mirror VirtualBinding / other bindings) ----
struct VBind<T, Tail> {
    stamp: Cell<u64>, // the per-virtual fired epoch-stamp (CHANGE 3 shipped)
    tail: Tail,
    _t: PhantomData<T>,
}
struct XBind<U, Tail> {
    // stands in for ResourceBinding / ColumnBinding / AccumBinding
    tail: Tail,
    _u: PhantomData<U>,
}
struct BNil;

// ---- VirtualStampSelector<V, I>: structural, inferred index, returns &Cell ----
// This is the SHARED keying primitive: both firer and gate resolve through it.
trait VirtualStampSelector<V, Index> {
    fn vstamp(&self) -> &Cell<u64>;
}
impl<V, Tail> VirtualStampSelector<V, Here> for VBind<V, Tail> {
    fn vstamp(&self) -> &Cell<u64> {
        &self.stamp
    }
}
impl<V, U, Tail, I> VirtualStampSelector<V, There<I>> for VBind<U, Tail>
where
    Tail: VirtualStampSelector<V, I>,
{
    fn vstamp(&self) -> &Cell<u64> {
        self.tail.vstamp()
    }
}
impl<V, U, Tail, I> VirtualStampSelector<V, There<I>> for XBind<U, Tail>
where
    Tail: VirtualStampSelector<V, I>,
{
    fn vstamp(&self) -> &Cell<u64> {
        self.tail.vstamp()
    }
}

// ---- GateWith<A, GI>: dispatch on Sched; GI from the parallel witness list ----
// Always pins GI=Here (so an Always unit's witness element is forced to Here and
// infers cleanly); On<V> resolves the cell via the shared selector at GI.
trait GateWith<A, GI> {
    fn open(bindings: &A, epoch: u64) -> bool;
}
impl<A> GateWith<A, Here> for Always {
    #[inline]
    fn open(_bindings: &A, _epoch: u64) -> bool {
        true // const-foldable: Always always opens
    }
}
impl<A, V, GI> GateWith<A, GI> for On<V>
where
    A: VirtualStampSelector<V, GI>,
{
    #[inline]
    fn open(bindings: &A, epoch: u64) -> bool {
        <A as VirtualStampSelector<V, GI>>::vstamp(bindings).get() == epoch
    }
}

// ---- value carrier (mirror WuCons / WuNil) ----
struct WuCons<W, Tail> {
    head: W,
    tail: Tail,
}
struct WuNil;

// ---- gated walk over a fixed bindings A, parallel SchedW witness list ----
// Mirrors RunGatedTrunk: per cell, gate the head on its schedule, recurse tail.
trait RunGated<A, SchedW> {
    fn run(&self, bindings: &A, epoch: u64);
}
impl<A> RunGated<A, Nil> for WuNil {
    fn run(&self, _bindings: &A, _epoch: u64) {}
}
impl<A, W, Tail, GI, SWTail> RunGated<A, Cons<GI, SWTail>> for WuCons<W, Tail>
where
    W: HasSchedule + WorkUnit<<W as HasSchedule>::Sched>,
    <W as HasSchedule>::Sched: GateWith<A, GI>,
    Tail: RunGated<A, SWTail>,
{
    fn run(&self, bindings: &A, epoch: u64) {
        if <<W as HasSchedule>::Sched as GateWith<A, GI>>::open(bindings, epoch) {
            self.head.exec();
        }
        self.tail.run(bindings, epoch);
    }
}

// ---- FIRER: VirtualProject mirrors AccumProject; VirtualSelector fires ----
// Project a unit's written Virtual<T> set out of the bindings into a bundle of
// &Cell refs (the EngineCtx::write_virtuals shape). The exec body fires through
// the bundle by type, index inferred (like ctx.append).
struct VCons<'a, T, Tail> {
    head: &'a Cell<u64>,
    tail: Tail,
    _t: PhantomData<T>,
}
struct VNil;

trait VirtualProject<'a, WSet, Idx> {
    type Out;
    fn vproject(&'a self) -> Self::Out;
}
impl<'a, A> VirtualProject<'a, Nil, Nil> for A {
    type Out = VNil;
    fn vproject(&'a self) -> VNil {
        VNil
    }
}
impl<'a, A, T, I, STail, ITail> VirtualProject<'a, Cons<Virtual<T>, STail>, Cons<I, ITail>> for A
where
    A: VirtualStampSelector<T, I>,
    A: VirtualProject<'a, STail, ITail>,
{
    type Out = VCons<'a, T, <A as VirtualProject<'a, STail, ITail>>::Out>;
    fn vproject(&'a self) -> Self::Out {
        VCons {
            head: <A as VirtualStampSelector<T, I>>::vstamp(self),
            tail: <A as VirtualProject<'a, STail, ITail>>::vproject(self),
            _t: PhantomData,
        }
    }
}

// VirtualSelector over the projected bundle: fire (set the cell) by type T.
trait VirtualFire<T, Index> {
    fn fire(&self, epoch: u64);
}
impl<'a, T, Tail> VirtualFire<T, Here> for VCons<'a, T, Tail> {
    fn fire(&self, epoch: u64) {
        self.head.set(epoch);
    }
}
impl<'a, T, U, Tail, I> VirtualFire<T, There<I>> for VCons<'a, U, Tail>
where
    Tail: VirtualFire<T, I>,
{
    fn fire(&self, epoch: u64) {
        self.tail.fire(epoch);
    }
}

// A firer Ctx holds the projected write-virtual bundle (mirror EngineCtx field).
struct FireCtx<WVirt> {
    write_virtuals: WVirt,
    epoch: u64,
}
impl<WVirt> FireCtx<WVirt> {
    #[inline]
    fn fire<T, I>(&self)
    where
        WVirt: VirtualFire<T, I>,
    {
        <WVirt as VirtualFire<T, I>>::fire(&self.write_virtuals, self.epoch);
    }
}

// ---- test units ----
struct Tick; // a virtual marker type

// Unit 0: Always producer that fires Virtual<Tick>. (In the engine the fire goes
// through the projected ctx; here exec is a no-op and we fire via FireCtx below
// to prove the firer path independently.)
struct Producer;
impl WorkUnit<Always> for Producer {
    fn exec(&self) {
        RAN.with(|r| r.borrow_mut().push("producer"));
    }
}

// Unit 1: On<Tick> consumer.
struct Consumer;
impl WorkUnit<On<Tick>> for Consumer {
    fn exec(&self) {
        RAN.with(|r| r.borrow_mut().push("consumer"));
    }
}
impl HasSchedule for Consumer {
    type Sched = On<Tick>;
}

// Unit 2: plain Always unit.
struct Plain;
impl WorkUnit<Always> for Plain {
    fn exec(&self) {
        RAN.with(|r| r.borrow_mut().push("plain"));
    }
}

thread_local! {
    static RAN: core::cell::RefCell<std::vec::Vec<&'static str>> =
        core::cell::RefCell::new(std::vec::Vec::new());
}

fn main() {
    // bindings: [ VBind<Tick>, XBind<i32>, BNil ]  (Tick virtual at position 0)
    let bindings = VBind::<Tick, _> {
        stamp: Cell::new(0),
        tail: XBind::<i32, _> {
            tail: BNil,
            _u: PhantomData,
        },
        _t: PhantomData,
    };

    // carrier: Producer (Always) -> Consumer (On<Tick>) -> Plain (Always)
    let carrier = WuCons {
        head: Producer,
        tail: WuCons {
            head: Consumer,
            tail: WuCons {
                head: Plain,
                tail: WuNil,
            },
        },
    };

    // --- pass 1: epoch=1, producer fires Tick before the gated walk ---
    let epoch: u64 = 1;
    // FIRER: project Producer's written virtual set {Virtual<Tick>} and fire.
    // (SchedW / WSet inferred at the call site, no turbofish.)
    let fctx = FireCtx {
        write_virtuals: VirtualProject::<Cons<Virtual<Tick>, Nil>, _>::vproject(&bindings),
        epoch,
    };
    fctx.fire::<Tick, _>();
    // GATED WALK: Consumer should run because Tick fired this epoch.
    carrier.run(&bindings, epoch);
    let pass1 = RAN.with(|r| r.borrow().clone());
    assert_eq!(pass1, ["producer", "consumer", "plain"], "pass1: consumer gated open");

    // --- pass 2: epoch=2, producer does NOT fire; consumer must skip ---
    RAN.with(|r| r.borrow_mut().clear());
    let epoch: u64 = 2;
    carrier.run(&bindings, epoch);
    let pass2 = RAN.with(|r| r.borrow().clone());
    assert_eq!(pass2, ["producer", "plain"], "pass2: stale stamp gates consumer shut (epoch decay)");

    // --- pass 3: epoch=3, fire again; consumer runs again ---
    RAN.with(|r| r.borrow_mut().clear());
    let epoch: u64 = 3;
    let fctx = FireCtx {
        write_virtuals: VirtualProject::<Cons<Virtual<Tick>, Nil>, _>::vproject(&bindings),
        epoch,
    };
    fctx.fire::<Tick, _>();
    carrier.run(&bindings, epoch);
    let pass3 = RAN.with(|r| r.borrow().clone());
    assert_eq!(pass3, ["producer", "consumer", "plain"], "pass3: re-fire reopens gate");

    println!("WORKS: firer + gate share the V-keyed cell; epoch decay gates On<V> correctly");
}
