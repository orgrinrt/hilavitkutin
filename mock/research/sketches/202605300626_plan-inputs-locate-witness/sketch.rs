//! Sketch (HILA-RUNTIME Path B foundation): bundle->PlanInputs projection kernel.
//!
//! Hypothesis: a per-WU read/write access-set can be projected into a
//! runtime bitmask whose bit indices are each member's position in the
//! global Stores list, using a PURE TYPE-LEVEL frunk-style index witness
//! (`Locate<Target, Index>`, solver-infers `Index`), under nightly-2026-05-28.
//!
//! This mirrors the shipping `dispatch/engine_ctx.rs` Selector / Project
//! pattern, but `Locate` is driven by the List TYPE alone (no runtime
//! `self`, unlike the runtime-driven `Selector`). If WORKS, the
//! bundle->PlanInputs round reuses this. The const-IndexOf coherence wall
//! (a `#[marker]` trait cannot carry consts, and head-vs-tail const impls
//! overlap) is avoided because the index is an inferred WITNESS TYPE, not
//! a computed const.
//!
//! Critical correctness property under test: two WUs touching the same
//! store must project to the SAME bit, so dependency edges (W writes
//! store s, W' reads store s) are computable from the masks.
//!
//! Outcome recorded at the bottom of this file.

use core::marker::PhantomData;

// --- type-level cons-list (mirrors api `access::{Empty, Cons}`) ---
struct Empty;
struct Cons<H, T>(PhantomData<(H, T)>);

// --- peano index witnesses (mirror `engine_ctx::{Here, There}`) ---
struct Here;
struct There<I>(PhantomData<I>);

trait PeanoVal {
    const VAL: usize;
}
impl PeanoVal for Here {
    const VAL: usize = 0;
}
impl<I: PeanoVal> PeanoVal for There<I> {
    const VAL: usize = 1 + I::VAL;
}

// --- `Locate<Target, Index>`: List contains Target at position Index. ---
// Disjoint Here / There impls keyed on the Index param: no coherence
// conflict, and the solver infers Index for a given (List, Target). Pure
// type-level (the List type drives inference; there is no runtime self,
// unlike Selector which is driven by an arena-node structure value).
trait Locate<Target, Index> {}
impl<Target, Tail> Locate<Target, Here> for Cons<Target, Tail> {}
impl<Target, Head, Tail, I> Locate<Target, There<I>> for Cons<Head, Tail> where
    Tail: Locate<Target, I>
{
}

// --- runtime mask (stand-in for the engine `AccessMask`) ---
#[derive(Default, Debug, PartialEq, Eq)]
struct Mask(u64);
impl Mask {
    fn set(&mut self, bit: usize) {
        self.0 |= 1u64 << bit;
    }
}

// --- `MaskProject<Stores, Indices>`: walk Self (an access set), set the
// bit for each member at its located index in Stores. Indices is a
// parallel witness cons-list, solver-inferred, exactly the
// `Project<R, Indices>` shape (the Indices param constrains each index,
// dodging E0207). ---
trait MaskProject<Stores, Indices> {
    fn project(mask: &mut Mask);
}
// base: an empty access set sets no bits.
impl<Stores> MaskProject<Stores, Empty> for Empty {
    fn project(_: &mut Mask) {}
}
// head: member M at located index I in Stores; recurse on the tail.
impl<Stores, M, Tail, I, ITail> MaskProject<Stores, Cons<I, ITail>> for Cons<M, Tail>
where
    Stores: Locate<M, I>,
    I: PeanoVal,
    Tail: MaskProject<Stores, ITail>,
{
    fn project(mask: &mut Mask) {
        mask.set(I::VAL);
        <Tail as MaskProject<Stores, ITail>>::project(mask);
    }
}

// free helper that lets the solver infer the Indices witness list,
// exactly like the shipping `project_reads::<R, _, _>(arena)`.
fn project_mask<Set, Stores, Indices>() -> Mask
where
    Set: MaskProject<Stores, Indices>,
{
    let mut m = Mask::default();
    <Set as MaskProject<Stores, Indices>>::project(&mut m);
    m
}

// --- scenario: a global Stores list + two WUs with overlapping sets ---
struct SA;
struct SB;
struct SC;
struct SD;
type Stores = Cons<SA, Cons<SB, Cons<SC, Cons<SD, Empty>>>>; // bit indices 0,1,2,3

// WU0 reads {SA, SC} -> bits 0,2 ; writes {SB} -> bit 1
type W0Read = Cons<SA, Cons<SC, Empty>>;
type W0Write = Cons<SB, Empty>;
// WU1 reads {SB} -> bit 1 (the store W0 writes: MUST be the same bit) ; writes {SD} -> bit 3
type W1Read = Cons<SB, Empty>;
type W1Write = Cons<SD, Empty>;

fn main() {
    // Indices inferred via `_`, exactly like `project_reads::<R, _, _>`.
    let w0_read = project_mask::<W0Read, Stores, _>();
    let w0_write = project_mask::<W0Write, Stores, _>();
    let w1_read = project_mask::<W1Read, Stores, _>();
    let w1_write = project_mask::<W1Write, Stores, _>();

    assert_eq!(w0_read, Mask(0b0101), "W0 reads SA(0)+SC(2)");
    assert_eq!(w0_write, Mask(0b0010), "W0 writes SB(1)");
    assert_eq!(w1_read, Mask(0b0010), "W1 reads SB(1), same bit as W0 write");
    assert_eq!(w1_write, Mask(0b1000), "W1 writes SD(3)");

    // The dependency edge W0 -> W1 is computable: W0 writes bit 1, W1 reads bit 1.
    assert_ne!(
        w0_write.0 & w1_read.0,
        0,
        "W0 write overlaps W1 read => dep edge (consistent store bit indices)"
    );

    println!("WORKS: pure-type-level Locate index witness + MaskProject with inferred Indices");
}

// OUTCOME: WORKS (nightly-2026-05-28, rustc 1.98.0-nightly 57d06900f, edition 2021).
// Compiled clean (no features needed: plain traits + const fn, no
// generic_const_exprs) and ran; every assert passed. The solver infers the
// `Indices` witness list from the `_` turbofish with no annotation, exactly
// like the shipping `project_reads::<R, _, _>`. No E0207 (the Indices param
// is constrained through the recursive bound chain). No coherence conflict
// (Here vs There<I> are disjoint Index params). The same-store-same-bit
// property held: W0's write mask and W1's read mask shared bit 1.
//
// => the bundle->PlanInputs projection round reuses this `Locate` +
// `MaskProject` shape directly against the real api `AccessSet`/`Cons`/`Empty`
// + engine `AccessMask`. The const-IndexOf coherence wall is sidestepped; no
// specialization needed. The remaining round work is mechanical: a
// `BundleProject` walk over `Cons<W,T>: WorkUnitBundle` that, per WU, runs
// MaskProject for `W::Read`/`W::Write` into `PlanInputs.{reads,writes}[i]`,
// copies `W::COMMUTATIVE`, and tracks `unit_count` via `AccessSet::LEN`.
