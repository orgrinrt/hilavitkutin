//! Sketch: does the `PlanDims` BUNDLE resolve through fully-generic engine code?
//!
//! The flat `Capacity` GAT is already proven (arvo #650/#651 ship on it). The
//! NEW risk for #652 is the two-level associated-type projection
//! `<<D as PlanDims>::Units as Capacity>::Array<T>` threaded through generic
//! code (the `Scheduler<D: PlanDims>` shape), including the nested 2-D case
//! `D::Edges::Array<D::Units::Array<T>>` over two DIFFERENT dims. If this
//! compiles and runs with NO `#![feature(...)]` gate, the bundle is GCE-free
//! and viable for the engine adoption. Outcome recorded in FINDINGS.md.

// Minimal stand-in for arvo's shipped `Capacity` (faithful to the projection
// shape; CAP is plain usize here since this throwaway does not pull arvo).
trait Capacity {
    type Array<T>: AsRef<[T]> + AsMut<[T]>;
    const CAP: usize;
    fn filled<T: Copy>(v: T) -> Self::Array<T>;
    fn from_fn<T, F: FnMut(usize) -> T>(f: F) -> Self::Array<T>;
}

struct Dim<const N: usize>;

impl<const N: usize> Capacity for Dim<N> {
    type Array<T> = [T; N];
    const CAP: usize = N;
    fn filled<T: Copy>(v: T) -> [T; N] {
        [v; N]
    }
    fn from_fn<T, F: FnMut(usize) -> T>(mut f: F) -> [T; N] {
        core::array::from_fn(|i| f(i))
    }
}

// The #652 bundle: the engine's ~12 capacity dims collapsed into one trait of
// `Capacity` associated types. Two dims suffice to exercise the projection.
trait PlanDims {
    type Units: Capacity;
    type Edges: Capacity;
}

struct DefaultPlanDims;
impl PlanDims for DefaultPlanDims {
    type Units = Dim<13>; // non-power-of-two on purpose
    type Edges = Dim<7>;
}

// Fully generic over the bundle, exactly the `Scheduler<D: PlanDims>` shape.
// Builds a 1-D unit array and a 2-D edges-of-units array (two DIFFERENT dims),
// the nested projection that is the load-bearing de-risk question.
fn build_and_walk<D: PlanDims>(live_units: usize) -> u32
where
    <D::Units as Capacity>::Array<u32>: Copy,
{
    // 1-D: D::Units::Array<u32>, built + written + read generically.
    let mut units: <D::Units as Capacity>::Array<u32> = D::Units::from_fn(|_| 0);
    {
        let slots: &mut [u32] = units.as_mut();
        let n = slots.len();
        let mut i = 0;
        while i < live_units && i < n {
            slots[i] = (i as u32) + 1;
            i += 1;
        }
    }
    let unit_sum: u32 = units.as_ref().iter().copied().sum();

    // 2-D over two different dims: D::Edges::Array<D::Units::Array<u32>>.
    let rows: <D::Edges as Capacity>::Array<<D::Units as Capacity>::Array<u32>> =
        D::Edges::filled(units);

    let cap_units = <D::Units as Capacity>::CAP;
    let cap_edges = <D::Edges as Capacity>::CAP;
    let row_count = rows.as_ref().len();

    let mut nested_sum: u32 = 0;
    for row in rows.as_ref() {
        nested_sum += row.as_ref().iter().copied().sum::<u32>();
    }

    assert_eq!(cap_units, 13);
    assert_eq!(cap_edges, 7);
    assert_eq!(row_count, 7);
    // each of the 7 edge rows is a copy of `units`, so nested_sum == 7 * unit_sum.
    assert_eq!(nested_sum, 7 * unit_sum);
    unit_sum
}

fn main() {
    // live_units=4 -> unit_sum = 1+2+3+4 = 10.
    let s = build_and_walk::<DefaultPlanDims>(4);
    assert_eq!(s, 10);
    println!("plandims-bundle sketch OK: unit_sum={s}");
}
