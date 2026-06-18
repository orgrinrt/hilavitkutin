// GATE-2 G-e de-risk #2: the OUTER dispatcher driving the SHIPPED per-trunk walk.
//
// Sketch 202606080130 proved an outer dispatcher, but it REINVENTED the inner
// per-trunk walk (`run_one_trunk` keyed by a single `const POS`). The engine
// already ships that walk: `dispatch/trunk_gate.rs::RunGatedTrunk`, round 2a,
// keyed by `const PHASE` + `const TRUNK` over a PEANO position witness
// (`Here`/`There<..>`), chosen precisely because the `{POS+1}` generic constant
// overflowed the trait solver normalising through the recursion under the heavy
// `Full: const BundleMasks<..>` + `Member::IS` bound. use-the-stack: G-e must
// DRIVE `RunGatedTrunk`, not a parallel walk.
//
// That reuse surfaces the real un-proven seam: enumerating (phase, trunk) and
// calling a per-trunk mono forces materialising a trunk/phase id into a
// CONST-GENERIC ARGUMENT. Two risks:
//   (a) `PHASE = phase_of(POS)` as a const-arg is always a GCE. A trunk lies
//       wholly in one phase, so TRUNK-ONLY keying (gate `trunk_of(pos)==TRUNK`)
//       drops the redundant PHASE const and removes that GCE. Phase ORDER is the
//       runtime outer loop (proven in 080130), not a const on the walk.
//   (b) the outer enumeration itself. A `const POS` outer walk threading
//       `{POS+1}` passes `TRUNK=POS` by IDENTITY (no GCE) but re-introduces the
//       `{POS+1}` recursion that trunk_gate.rs reports overflowing under a heavy
//       gate. A Peano outer walk avoids `{POS+1}` but forces `TRUNK={Pos::INDEX}`
//       (a GCE const-arg).
//
// This sketch tests the CLEANEST candidate: TRUNK-ONLY Peano inner walk (shipped
// RunGatedTrunk shape, minus the redundant PHASE const) driven by a `const POS`
// outer walk (TRUNK=POS by identity, zero GCE const-args), under a
// representative-weight const member gate (a const fn over a fixed mask array,
// approximating is_member's compute_phases/compute_trunks weight). If it
// compiles + orders correctly, that is the G-e shape (with a small trunk-only
// refactor to RunGatedTrunk). WORKS / FAILS.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::cell::RefCell;
use std::marker::PhantomData;

#[derive(Copy, Clone)]
struct MorselRange;

thread_local! {
    static ORDER: RefCell<Vec<u32>> = RefCell::new(Vec::new());
}

// ---- carrier (cons-list of unit VALUES), mirrors WuCons/WuNil ----
struct WuNil;
struct WuCons<H, T> {
    head: H,
    tail: T,
}
trait Wu: Copy {
    fn exec(&self);
}

// ---- Peano position witness, mirrors engine_ctx::{Here,There} + WitnessIndex ----
struct Here;
struct There<I>(PhantomData<I>);
trait PosIndex {
    const INDEX: usize;
}
impl PosIndex for Here {
    const INDEX: usize = 0;
}
impl<I: PosIndex> PosIndex for There<I> {
    const INDEX: usize = I::INDEX + 1;
}

// ---- representative-weight grouping, computed by a const fn over a fixed mask
//      array (approximates is_member -> compute_phases_waist/compute_trunks). The
//      point is to put real const-fn evaluation behind the gate, like the engine.
const N: usize = 4;
//   pos0: phase0 trunk0 (root)      pos1: phase0 trunk1 (root)
//   pos2: phase0 trunk0 (member)    pos3: phase1 trunk3 (root, later phase)
// hardcoded read/write column masks per unit (bit = column):
const READS: [u64; N] = [0b0001, 0b0000, 0b0010, 0b0100];
const WRITES: [u64; N] = [0b0010, 0b1000, 0b0100, 0b1_0000];

// trunk = within-(implicit-)phase column-conflict component, canonicalised to the
// smallest member position (union-find), mirroring compute_trunks. Hardcoded to
// the known grouping but computed by a const fn so the gate carries real weight.
const fn trunk_of(pos: usize) -> usize {
    // tiny union-find over WRITES/READS conflicts, same shape as compute_trunks.
    let mut parent = [0usize; N];
    let mut k = 0;
    while k < N {
        parent[k] = k;
        k += 1;
    }
    let mut a = 0;
    while a < N {
        let mut b = a + 1;
        while b < N {
            let conflict = (WRITES[a] & READS[b]) != 0
                || (WRITES[a] & WRITES[b]) != 0
                || (READS[a] & WRITES[b]) != 0;
            // same-phase guard: real compute_trunks only merges units in the
            // same phase (a trunk lies wholly in one phase). PHASE is a const
            // layer here so trunk_of stays a const fn.
            let same_phase = PHASE[a] == PHASE[b];
            if same_phase && conflict {
                let mut ra = a;
                while parent[ra] != ra {
                    ra = parent[ra];
                }
                let mut rb = b;
                while parent[rb] != rb {
                    rb = parent[rb];
                }
                if ra < rb {
                    parent[rb] = ra;
                } else if rb < ra {
                    parent[ra] = rb;
                }
            }
            b += 1;
        }
        a += 1;
    }
    let mut r = pos;
    while parent[r] != r {
        r = parent[r];
    }
    r
}

// phase: RUNTIME-queried for ordering (not a const on the walk). Hardcoded layer.
const PHASE: [usize; N] = [0, 0, 0, 1];
fn phase_of(pos: usize) -> usize {
    PHASE[pos]
}
const NPHASES: usize = 2;

// ---- per-unit RunFiber step (mirrors RunFiber::run_head) ----
trait RunHead {
    fn run_head(&self, b: &(), m: MorselRange);
}
impl RunHead for WuNil {
    #[inline]
    fn run_head(&self, _b: &(), _m: MorselRange) {}
}
impl<H: Wu, T> RunHead for WuCons<H, T> {
    #[inline]
    fn run_head(&self, _b: &(), _m: MorselRange) {
        self.head.exec();
    }
}

// ---- SHIPPED-SHAPE inner walk, TRUNK-ONLY keyed, Peano position ----
// Mirrors RunGatedTrunk minus the redundant PHASE const. `Member::IS` is the
// representative-weight const gate (const-fn over the mask arrays). The recursion
// threads `There<Pos>` (a type, no const arithmetic) exactly like trunk_gate.rs.
struct Member<Pos, const TRUNK: usize>(PhantomData<Pos>);
impl<Pos: PosIndex, const TRUNK: usize> Member<Pos, TRUNK> {
    const IS: bool = trunk_of(Pos::INDEX) == TRUNK;
}

trait RunGatedTrunk<const TRUNK: usize, Pos> {
    fn run_trunk(&self, b: &(), m: MorselRange);
}
impl<const TRUNK: usize, Pos> RunGatedTrunk<TRUNK, Pos> for WuNil {
    #[inline]
    fn run_trunk(&self, _b: &(), _m: MorselRange) {}
}
impl<H: Wu, T, const TRUNK: usize, Pos: PosIndex> RunGatedTrunk<TRUNK, Pos> for WuCons<H, T>
where
    T: RunGatedTrunk<TRUNK, There<Pos>>,
{
    #[inline]
    fn run_trunk(&self, b: &(), m: MorselRange) {
        if Member::<Pos, TRUNK>::IS {
            self.run_head(b, m);
        }
        self.tail.run_trunk(b, m);
    }
}

// ---- OUTER dispatcher: `const POS` outer walk, TRUNK=POS by identity ----
// At each carrier position POS that is a trunk-root (`const { trunk_of(POS)==POS
// }`) and whose phase matches the current runtime pass `p`, dispatch the inner
// per-trunk mono `RunGatedTrunk::<TRUNK=POS, Here>` over the FULL carrier (passed
// alongside, since the inner walk starts from position 0). POS is the in-scope
// const generic, passed to TRUNK by IDENTITY: no GCE const-arg. The outer
// recursion threads `{POS+1}` (the construct trunk_gate.rs feared); this sketch
// tests whether it survives WITH the inner RunGatedTrunk instantiated at each
// step under the weighted gate.
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
    Full: RunGatedTrunk<POS, Here>,
{
    #[inline]
    fn run(&self, full: &Full, p: usize, b: &(), m: MorselRange) {
        if const { trunk_of(POS) == POS } {
            // POS is a trunk-root; fire it only in its phase's runtime pass.
            if phase_of(POS) == p {
                full.run_trunk(b, m); // RunGatedTrunk::<TRUNK=POS, Here> on Full
            }
        }
        self.tail.run(full, p, b, m);
    }
}

fn dispatch_all<C>(carrier: &C, nphases: usize, b: &(), m: MorselRange)
where
    C: Discover<C, 0> + RunGatedTrunk<0, Here>,
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
        println!("WORKS: outer drives shipped-shape RunGatedTrunk, order = {got:?} (trunk-only keyed, const-POS outer, zero GCE const-args)");
        std::process::exit(0);
    } else {
        eprintln!("FAILS: got {got:?}, expected {expected:?}");
        std::process::exit(1);
    }
}
