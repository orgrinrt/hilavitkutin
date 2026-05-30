//! Sketch (numeric-position convention, T13): capacity-as-a-TYPE, array-as-an
//! associated-type, consumed from fully-generic code.
//!
//! Hypothesis: the engine's fixed-capacity arrays can be expressed so that the
//! dimension is a TYPE (not a `Cap` const generic) and the array length is a
//! LITERAL inside a concrete impl. Then no const expression is evaluated in
//! type position, so `generic_const_exprs` never runs and cannot ICE/overflow,
//! even when the storage is constructed, filled, indexed, and walked from
//! fully-generic code (the shape the scheduler needs and that ICE'd under the
//! `Cap`-const-generic form via `cap_size(cap(N))`).
//!
//! What this proves (or fails to):
//! 1. A `Capacity` trait with a GAT `type Array<T>` whose concrete impls bind
//!    a literal-length array (`[T; 4]`).
//! 2. A `PlanDims` trait bundling several capacities into ONE type param
//!    (op's "normalise the generics to reduce boilerplate").
//! 3. A `Plan<D: PlanDims>` struct whose fields are nested associated-type
//!    projections `<D::Units as Capacity>::Array<T>`.
//! 4. GENERIC construction, fill, slice access, and topological walk over those
//!    arrays, with NO const generic and NO `cap_size` anywhere in generic code.
//! 5. Consumption both from a non-generic caller with the DEFAULT dims (mirrors
//!    `Scheduler::builder()` defaulting) and from a deeply-generic wrapper.
//!
//! No `#![feature(...)]` gates: if it compiles plain, the pattern escapes the
//! whole generic-const-exprs surface that produced every ICE tonight. Outcome
//! recorded at the bottom of this file.

#![allow(dead_code)]

use std::cell::RefCell;

// Toy stand-in for arvo's `Cap` (a newtype over usize). The real one is
// `arvo::Cap`; only its newtype-ness matters here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cap(usize);
const fn cap(n: usize) -> Cap {
    Cap(n)
}

// ---------------------------------------------------------------------
// Capacity: the dimension is a TYPE; the array is an associated type whose
// concrete impl binds a LITERAL-length array. `CAP` is the typed surface
// capacity; `N` is the runtime length used only as a loop bound, never as an
// array length in generic code.
// ---------------------------------------------------------------------
trait Capacity {
    // The `AsRef`/`AsMut` bound (stable, no const-eval) gives slice access and
    // iteration without a const-generic `Index` HKT, and — crucially — lets a
    // 2-D matrix be the COMPOSITION `R::Array<C::Array<T>>` with no new trait
    // (the architect's objection: genuinely-nested storage).
    type Array<T>: AsRef<[T]> + AsMut<[T]>;
    const CAP: Cap;
    const N: usize;
    fn empty<T: Copy>(fill: T) -> Self::Array<T>;
    fn as_slice<T>(a: &Self::Array<T>) -> &[T];
    fn as_mut_slice<T>(a: &mut Self::Array<T>) -> &mut [T];
}

// ONE generic capacity marker parameterised by a bare-`usize` dimension
// (op's refinement: no rung ladder, no macro, no round-up — any exact N via
// `Dim<N>`). The implementing TYPE is generic over `const DIM: usize`; the
// `Capacity` TRAIT stays non-generic, so consumers still bind `C: Capacity`
// with no const param. `DIM` is used directly as the array length
// (`[T; DIM]` is plain min-const-generics: no `cap_size`, no GCE). The typed
// `CAP: Cap = cap(DIM)` maps the bare dimension to the surface capacity in a
// VALUE position (associated const), never in type position. This `cap(DIM)`
// associated-const-of-a-const-param is the one bit being proven here.
struct Dim<const DIM: usize>;
impl<const DIM: usize> Capacity for Dim<DIM> {
    type Array<T> = [T; DIM];
    const CAP: Cap = cap(DIM);
    const N: usize = DIM;
    fn empty<T: Copy>(fill: T) -> [T; DIM] {
        [fill; DIM]
    }
    fn as_slice<T>(a: &[T; DIM]) -> &[T] {
        a
    }
    fn as_mut_slice<T>(a: &mut [T; DIM]) -> &mut [T] {
        a
    }
}

// ---------------------------------------------------------------------
// PlanDims: bundle the engine's dimensions into ONE type parameter. The real
// engine would have ~12 associated capacities (units, stores, edges, ...).
// ---------------------------------------------------------------------
trait PlanDims {
    type Units: Capacity;
    type Stores: Capacity;
}

// The default engine budget: one impl, the analogue of `DefaultRunCfg`.
struct DefaultPlanDims;
impl PlanDims for DefaultPlanDims {
    type Units = Dim<4>;
    type Stores = Dim<8>;
}

// A second, larger budget, to prove the genericity is real (not just the
// default specialised away).
struct BigPlanDims;
impl PlanDims for BigPlanDims {
    type Units = Dim<16>;
    type Stores = Dim<16>;
}

// ---------------------------------------------------------------------
// A plan structure generic over the dims bundle. Fields are nested
// associated-type projections; no const generic, no `cap_size`.
// ---------------------------------------------------------------------
struct Plan<D: PlanDims> {
    /// Topological dispatch permutation: `unit_topo[step]` = unit index to run.
    unit_topo: <D::Units as Capacity>::Array<usize>,
    unit_count: usize,
    /// Per-store access mask (toy).
    store_masks: <D::Stores as Capacity>::Array<u64>,
    store_count: usize,
}

impl<D: PlanDims> Plan<D> {
    /// Generic construction: build the fixed arrays without ever naming a
    /// length or a const generic.
    fn new() -> Self {
        Plan {
            unit_topo: <D::Units as Capacity>::empty(0usize),
            unit_count: 0,
            store_masks: <D::Stores as Capacity>::empty(0u64),
            store_count: 0,
        }
    }
}

// Recorder so we can assert the generic walk produced the right order.
thread_local! {
    static OBSERVED: RefCell<Vec<usize>> = RefCell::new(Vec::new());
}

// ---------------------------------------------------------------------
// The scheduler-shaped consumption: a fn generic over the dims bundle that
// builds a topo permutation into the typed storage and walks it. This is the
// exact shape (generic build + generic walk over a fixed-capacity array) that
// ICE'd under the `Cap`-const-generic form.
// ---------------------------------------------------------------------
fn build_and_walk<D: PlanDims>(n: usize) -> Vec<usize> {
    let mut plan = Plan::<D>::new();
    plan.unit_count = n;

    // Fill a toy topo order (reverse) through the typed slice accessor.
    {
        let topo = <D::Units as Capacity>::as_mut_slice(&mut plan.unit_topo);
        let mut i = 0;
        while i < n && i < <D::Units as Capacity>::N {
            topo[i] = n - 1 - i;
            i += 1;
        }
    }

    // Walk in topo order through the typed slice accessor.
    let topo = <D::Units as Capacity>::as_slice(&plan.unit_topo);
    OBSERVED.with(|o| o.borrow_mut().clear());
    let mut out = Vec::new();
    let mut step = 0;
    while step < plan.unit_count && step < <D::Units as Capacity>::N {
        out.push(topo[step]);
        OBSERVED.with(|o| o.borrow_mut().push(topo[step]));
        step += 1;
    }
    out
}

// A deeply-generic wrapper, to prove the dims thread through nested generic
// frames (not just one) with no const-eval pressure.
fn outer<D: PlanDims>(n: usize) -> Vec<usize> {
    build_and_walk::<D>(n)
}

// ---------------------------------------------------------------------
// 2-D / nested storage (the architect's objection): a matrix is just the
// COMPOSITION of two 1-D capacities, `R::Array<C::Array<T>>`. No new trait,
// no `Capacity2D`. The `AsRef<[T]>`/`AsMut<[T]>` bound makes both levels
// indexable; the inner-array-is-Copy-when-T-is-Copy fact makes generic
// construction work. This is `[[Trunk; TRUNKS_PER_PHASE]; PHASES]` and
// `DirtyMasks<FIBERS, COLUMNS>` in the engine.
// ---------------------------------------------------------------------
struct Matrix<R: Capacity, C: Capacity, T> {
    rows: <R as Capacity>::Array<<C as Capacity>::Array<T>>,
}

impl<R: Capacity, C: Capacity, T: Copy> Matrix<R, C, T> {
    // The `inner-row: Copy` bound is the one real residue of the 2-D case: the
    // outer `empty([inner; R])` needs the inner array to be Copy. It is, for
    // every engine POD element type, so the bound discharges at every concrete
    // use; it just has to be stated on the constructor.
    fn filled(v: T) -> Self
    where
        <C as Capacity>::Array<T>: Copy,
    {
        // inner row = [v; C::N]; outer = [inner; R::N].
        Matrix {
            rows: <R as Capacity>::empty(<C as Capacity>::empty(v)),
        }
    }
    fn get(&self, r: usize, c: usize) -> T {
        let rows: &[<C as Capacity>::Array<T>] = self.rows.as_ref();
        rows[r].as_ref()[c]
    }
    fn set(&mut self, r: usize, c: usize, v: T) {
        let rows: &mut [<C as Capacity>::Array<T>] = self.rows.as_mut();
        rows[r].as_mut()[c] = v;
    }
}

// Generic over BOTH row and column capacities: fill a sub-matrix and read the
// diagonal back. Mirrors `[[Trunk; C]; R]` access in plan code.
fn matrix_diagonal_sum<R: Capacity, C: Capacity>(rows: usize, cols: usize) -> u32
where
    <C as Capacity>::Array<u32>: Copy,
{
    let mut m = Matrix::<R, C, u32>::filled(0);
    let mut r = 0;
    while r < rows && r < <R as Capacity>::N {
        let mut c = 0;
        while c < cols && c < <C as Capacity>::N {
            m.set(r, c, (r * 10 + c) as u32);
            c += 1;
        }
        r += 1;
    }
    let mut sum = 0;
    let mut d = 0;
    while d < rows && d < cols && d < <R as Capacity>::N && d < <C as Capacity>::N {
        sum += m.get(d, d);
        d += 1;
    }
    sum
}

fn main() {
    // Consume from a non-generic caller with the DEFAULT dims (mirrors
    // `Scheduler::builder()` defaulting to the engine budget). The Cap4 units
    // capacity bounds n at 4.
    let order = outer::<DefaultPlanDims>(4);
    assert_eq!(
        order,
        vec![3, 2, 1, 0],
        "generic build+walk over the default dims produced the topo order"
    );

    // Consume with a different dims bundle to prove real genericity.
    let big = outer::<BigPlanDims>(6);
    assert_eq!(big, vec![5, 4, 3, 2, 1, 0]);

    // The typed surface capacity is reachable and distinct per dims.
    assert_eq!(<DefaultPlanDims as PlanDims>::Units::CAP, cap(4));
    assert_eq!(<BigPlanDims as PlanDims>::Units::CAP, cap(16));

    // 2-D nested storage (composition of two 1-D capacities), generic over
    // both row and column capacities. Diagonal of a 3x3 sub-matrix:
    // get(0,0)=0, get(1,1)=11, get(2,2)=22 -> 33.
    let diag = matrix_diagonal_sum::<Dim<4>, Dim<8>>(3, 3);
    assert_eq!(diag, 33, "2-D nested storage built + indexed + walked generically");

    println!("WORKS: default={:?} big={:?} diag={}", order, big, diag);
}
