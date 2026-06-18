// GATE-2 G-e de-risk: the const-range trunk-root dispatcher.
//
// The two halves of G-e are PROVEN: sketch 202606071230 (per-trunk
// `const { trunk_of(POS)==TRUNK }`-gated walk that DCEs to a member-only mono)
// and 202606071330 (grouping from access-set types). This sketch proves the
// remaining piece: the OUTER dispatcher that drives `run_one_trunk::<TRUNK>` for
// every trunk across every phase, in phase order, single-core, output-equivalent
// to the flat walk.
//
// FIRST DESIGN (numeric const recursion `POS->POS+1` to a bound `N`, and
// `PHASE->PHASE+1` to `NPHASES`) FAILED: a `const`-guarded recursive call still
// requires the next-depth trait bound at the type level regardless of the
// runtime `if const` guard, so `DispatchPosCg<POS>` required `DispatchPosCg<POS+1>`
// required ... infinitely ("unconstrained generic constant" / unbounded type
// recursion). That is the engine pattern's lesson: numeric const recursion to a
// bound does NOT terminate at the type level.
//
// WORKING DESIGN (this file): recurse STRUCTURALLY on the carrier cons-list
// (terminates at `WuNil`, exactly like the shipped `RunFiber`/`RunTrunkSel`
// walks), threading `POS` as `{ POS + 1 }` alongside; and drive the phase
// ordering as a RUNTIME loop (`for p in 0..nphases`), matching `phase_of(POS)==p`
// at runtime. So: no numeric-bound const recursion anywhere, no const recursion
// on PHASE; `run_one_trunk::<POS>` still monomorphises per trunk-root because POS
// is the structurally-threaded const generic. Hypothesis: compiles + reproduces
// the flat walk's phase-ordered execution. WORKS / FAILS.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::cell::RefCell;

#[derive(Copy, Clone)]
struct MorselRange;

thread_local! {
    static ORDER: RefCell<Vec<u32>> = RefCell::new(Vec::new());
}

struct WuNil;
struct WuCons<H, T> {
    head: H,
    tail: T,
}

trait Wu: Copy {
    fn exec(&self);
}

trait RunFiber {
    fn run(&self, b: &(), m: MorselRange);
}
impl RunFiber for WuNil {
    #[inline]
    fn run(&self, _b: &(), _m: MorselRange) {}
}
impl<H: Wu, T: RunFiber> RunFiber for WuCons<H, T> {
    #[inline]
    fn run(&self, b: &(), m: MorselRange) {
        self.head.exec();
        self.tail.run(b, m);
    }
}

// ---- const grouping (hardcoded; engine uses the proven const fns over masks) ----
const N: usize = 4;
//   pos0: phase0 trunk0 (root)      pos1: phase0 trunk1 (root)
//   pos2: phase0 trunk0 (member)    pos3: phase1 trunk3 (root, later phase)
const PHASE: [usize; N] = [0, 0, 0, 1];
const GROUPING: [usize; N] = [0, 1, 0, 3];
const NPHASES: usize = 2;

const fn trunk_of(pos: usize) -> usize {
    GROUPING[pos]
}
const fn phase_of(pos: usize) -> usize {
    PHASE[pos]
}

// ---- PROVEN per-trunk gated walk (sketch 071230 shape), structural on carrier ----
trait RunTrunkSel<const POS: usize, const TRUNK: usize> {
    fn run(&self, b: &(), m: MorselRange);
}
impl<const POS: usize, const TRUNK: usize> RunTrunkSel<POS, TRUNK> for WuNil {
    #[inline]
    fn run(&self, _b: &(), _m: MorselRange) {}
}
impl<H: Wu, T, const POS: usize, const TRUNK: usize> RunTrunkSel<POS, TRUNK> for WuCons<H, T>
where
    T: RunTrunkSel<{ POS + 1 }, TRUNK>,
{
    #[inline]
    fn run(&self, b: &(), m: MorselRange) {
        if const { trunk_of(POS) == TRUNK } {
            let single = WuCons { head: self.head, tail: WuNil };
            RunFiber::run(&single, b, m);
        }
        self.tail.run(b, m);
    }
}

#[inline]
fn run_one_trunk<C, const TRUNK: usize>(carrier: &C, b: &(), m: MorselRange)
where
    C: RunTrunkSel<0, TRUNK>,
{
    carrier.run(b, m);
}

// ---- THE NEW PIECE: structural trunk-root discovery + runtime phase loop ----
//
// `Discover` walks the carrier structurally (terminates at WuNil), threading POS
// as the const generic. At each position that is a trunk-root (`const`), and
// whose phase matches the runtime `p`, it dispatches the per-trunk mono on the
// FULL carrier. The full carrier is passed alongside (`Full`), distinct from the
// structural-walk receiver, because `run_one_trunk` walks from position 0.
trait Discover<Full, const POS: usize> {
    fn run(&self, full: &Full, p: usize, b: &(), m: MorselRange);
}
impl<Full, const POS: usize> Discover<Full, POS> for WuNil {
    #[inline]
    fn run(&self, _full: &Full, _p: usize, _b: &(), _m: MorselRange) {}
}
impl<Full, H, T, const POS: usize> Discover<Full, POS> for WuCons<H, T>
where
    T: Discover<Full, { POS + 1 }>,
    Full: RunTrunkSel<0, POS>,
{
    #[inline]
    fn run(&self, full: &Full, p: usize, b: &(), m: MorselRange) {
        if const { trunk_of(POS) == POS } {
            // POS is a trunk-root; run it in its phase's pass only.
            if phase_of(POS) == p {
                run_one_trunk::<Full, POS>(full, b, m);
            }
        }
        self.tail.run(full, p, b, m);
    }
}

// Top entry: runtime phase loop, structural discovery inner. Single-core: the
// waist between phases is a no-op here; the carrier `C` must be `Copy`-cheap to
// pass as both receiver and `full` (in the engine it is `&self.wu_values`).
fn dispatch_all<C>(carrier: &C, nphases: usize, b: &(), m: MorselRange)
where
    C: Discover<C, 0>,
{
    let mut p = 0;
    while p < nphases {
        carrier.run(carrier, p, b, m);
        // waist_barrier() here in the engine (degenerate single-core).
        p += 1;
    }
}

// ---- WU instances ----
#[derive(Copy, Clone)]
struct U0;
#[derive(Copy, Clone)]
struct U1;
#[derive(Copy, Clone)]
struct U2;
#[derive(Copy, Clone)]
struct U3;
macro_rules! wu {
    ($t:ty, $id:expr) => {
        impl Wu for $t {
            #[inline]
            fn exec(&self) {
                ORDER.with(|o| o.borrow_mut().push($id));
            }
        }
    };
}
wu!(U0, 0);
wu!(U1, 1);
wu!(U2, 2);
wu!(U3, 3);

fn main() {
    let carrier = WuCons {
        head: U0,
        tail: WuCons {
            head: U1,
            tail: WuCons { head: U2, tail: WuCons { head: U3, tail: WuNil } },
        },
    };

    dispatch_all(&carrier, NPHASES, &(), MorselRange);

    let got = ORDER.with(|o| o.borrow().clone());
    // Phase 0: trunk-root pos0 (trunk0 -> members pos0,pos2), then pos1 (trunk1 ->
    // pos1). Phase 1: pos3 (trunk3 -> pos3). Expected: [0, 2, 1, 3].
    let expected = vec![0u32, 2, 1, 3];
    if got == expected {
        println!("WORKS: dispatcher order = {got:?} (phase-ordered, per-trunk monos, structural recursion)");
        std::process::exit(0);
    } else {
        eprintln!("FAILS: got {got:?}, expected {expected:?}");
        std::process::exit(1);
    }
}
