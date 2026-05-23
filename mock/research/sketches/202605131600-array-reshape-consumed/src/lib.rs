//! Sketch S5: end-to-end consumption with array reshape.
//!
//! For each of the three stubs, validates the FULL consumption path:
//! engine input -> wrapper -> arvo -> wrapper-side reshape -> engine
//! consumer reads the reshaped output. No discarding. The caller-side
//! consumer is what makes this sketch authoritative.
//!
//! Three wrappers, three reshape stories:
//!
//! 1. rcm_reorder: arvo `[NodeId; cap_size(cap_of(MAX_UNITS))]` ->
//!    engine `[UnitId; MAX_UNITS]`. Per-element copy with width
//!    conversion (usize -> u32 -> UnitId transmute_copy).
//! 2. block_diagonalise: arvo `(USize, [USize; cap_size(...)])` ->
//!    engine `Bool`. Scalar projection; array discarded but the
//!    projection logic is explicit.
//! 3. spectral_partition: arvo `(USize, [USize; cap_size(...)])` ->
//!    engine `FiberGrouping<MAX_UNITS, MAX_FIBERS>`. Per-element copy
//!    with class-id to FiberId conversion.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Bool, Cap, Identity, USize};
use arvo::Hot;
use arvo_bitmask::{BitMatrix, NodeId, cap_size};
use arvo_bits::Bits;
use arvo_numeric_contracts::{FromConstant, TotalOrd};

use hilavitkutin_api::{FiberId, UnitId};

#[inline]
pub const fn cap_of(n: usize) -> Cap {
    Cap(USize(n))
}

// ---- Engine-side mirrors (from real engine; minimal repro) ----

pub struct CsrLike<const MAX_UNITS: usize, const MAX_EDGES: usize> {
    pub row_offsets: [USize; MAX_UNITS],
    pub col_indices: [UnitId; MAX_EDGES],
    pub unit_count: USize,
    pub edge_count: USize,
}

impl<const MAX_UNITS: usize, const MAX_EDGES: usize> CsrLike<MAX_UNITS, MAX_EDGES> {
    pub const fn empty() -> Self {
        Self {
            row_offsets: [USize::ZERO; MAX_UNITS],
            col_indices: [UnitId::ZERO; MAX_EDGES],
            unit_count: USize::ZERO,
            edge_count: USize::ZERO,
        }
    }
    #[inline]
    fn end_for(&self, i: usize) -> usize {
        let next = i + 1;
        let count = self.unit_count.0;
        if next < count { self.row_offsets[next].0 } else { self.edge_count.0 }
    }
}

/// Mirror of `hilavitkutin::plan::fiber::FiberGrouping`.
pub struct FiberGroupingLike<const MAX_UNITS: usize, const MAX_FIBERS: usize> {
    pub assignment: [FiberId; MAX_UNITS],
    pub fiber_count: USize,
    _max_fibers: core::marker::PhantomData<[(); MAX_FIBERS]>,
}

impl<const MAX_UNITS: usize, const MAX_FIBERS: usize> FiberGroupingLike<MAX_UNITS, MAX_FIBERS> {
    pub const fn new() -> Self {
        Self {
            assignment: [FiberId::ZERO; MAX_UNITS],
            fiber_count: USize::ZERO,
            _max_fibers: core::marker::PhantomData,
        }
    }
}

// ---- Width conversion helpers ----

/// UnitId -> usize. Mirrors the engine's existing transmute_copy
/// pattern at `plan/graph.rs:120`. UnitId is Uint<16> -> Bits<16,Warm,Unsigned>
/// -> u32 container. Read four bytes; upper two are zero by construction.
#[inline]
fn unit_id_to_usize(u: UnitId) -> usize {
    let raw: u32 = unsafe { core::mem::transmute_copy(&u) };
    raw as usize
}

/// usize -> UnitId. Inverse of the above. The narrow from usize to
/// u32 is safe because UnitId's value range is bounded by MAX_UNITS,
/// which the engine guarantees to fit in u16 (and therefore u32).
#[inline]
fn usize_to_unit_id(n: usize) -> UnitId {
    let raw_u32 = n as u32;
    unsafe { core::mem::transmute_copy(&raw_u32) }
}

/// usize -> FiberId. FiberId is Uint<7> -> Bits<7,Warm,Unsigned>.
/// Warm picks containers wider than the logical width for codegen
/// reasons; the engine's existing pattern at `plan/steps.rs:288-292`
/// uses u16 for the transmute_copy. The const-block size assertion
/// below confirms u16 is correct; if the substrate's storage
/// projection ever shifts FiberId's container width, this assertion
/// surfaces the change immediately.
#[inline]
fn usize_to_fiber_id(n: usize) -> FiberId {
    let raw_u16 = n as u16;
    unsafe { core::mem::transmute_copy(&raw_u16) }
}

const _: () = {
    assert!(core::mem::size_of::<UnitId>() == core::mem::size_of::<u32>());
    assert!(core::mem::size_of::<FiberId>() == core::mem::size_of::<u16>());
};

// ---- CSR -> BitMatrix conversion (from S3) ----

#[inline]
pub fn csr_to_bitmatrix<const MAX_UNITS: usize, const MAX_EDGES: usize>(
    graph: &CsrLike<MAX_UNITS, MAX_EDGES>,
) -> BitMatrix<Bits<64, Hot>, { cap_of(MAX_UNITS) }>
where
    [(); cap_size(cap_of(MAX_UNITS))]:,
{
    let mut matrix: BitMatrix<Bits<64, Hot>, { cap_of(MAX_UNITS) }> = BitMatrix::empty();
    let n = graph.unit_count.0;
    let mut i = 0usize;
    while i < n {
        let start = graph.row_offsets[i].0;
        let end_excl = graph.end_for(i);
        let mut k = start;
        while k < end_excl {
            let from = NodeId::new(USize(i));
            let to = NodeId::new(USize(unit_id_to_usize(graph.col_indices[k])));
            matrix.set_edge(from, to);
            k += 1;
        }
        i += 1;
    }
    matrix
}

// ---- Wrapper 1: rcm_reorder with full reshape ----

/// rcm engine wrapper. Takes engine CSR, returns engine-shape array.
/// Internal flow: CSR -> BitMatrix -> arvo::rcm_reorder -> reshape
/// to engine UnitId array. The reshape uses Strategy A (per-element
/// copy) because Strategy B (whole-array transmute) is unsound: the
/// element sizes differ (NodeId = USize = 8 bytes on 64-bit, UnitId
/// = u32 = 4 bytes).
#[inline]
pub fn rcm_engine_wrapper<const MAX_UNITS: usize, const MAX_EDGES: usize>(
    graph: &CsrLike<MAX_UNITS, MAX_EDGES>,
    _topo: &[UnitId; MAX_UNITS],
) -> [UnitId; MAX_UNITS]
where
    [(); cap_size(cap_of(MAX_UNITS))]:,
{
    let matrix = csr_to_bitmatrix::<MAX_UNITS, MAX_EDGES>(graph);
    let arvo_out: [NodeId; cap_size(cap_of(MAX_UNITS))] =
        arvo_sparse::rcm_reorder::<Bits<64, Hot>, { cap_of(MAX_UNITS) }>(&matrix);

    // Reshape: per-element copy with width conversion.
    // The two arrays are numerically equal length but syntactically
    // distinct to rustc. We iterate by raw index, which is sound
    // because MAX_UNITS == cap_size(cap_of(MAX_UNITS)) at runtime.
    let mut out: [UnitId; MAX_UNITS] = [UnitId::ZERO; MAX_UNITS];
    let mut i = 0usize;
    while i < MAX_UNITS {
        // Index into arvo_out with i. rustc accepts this because the
        // array bounds are runtime-checked; the const-arg unification
        // is not required at the indexing site.
        let raw_usize = arvo_out[i].0.0;
        out[i] = usize_to_unit_id(raw_usize);
        i += 1;
    }
    out
}

// ---- Wrapper 2: block_diagonalise with scalar projection ----

/// block engine wrapper. Returns engine-shape Bool. The arvo output's
/// array half is discarded; the block_count is projected to Bool.
/// Policy: feasible iff block_count >= 1 (every input produces at
/// least one block). The real engine policy may differ; this sketch
/// validates the scalar-projection shape, not the policy choice.
#[inline]
pub fn block_engine_wrapper<const MAX_UNITS: usize, const MAX_EDGES: usize>(
    graph: &CsrLike<MAX_UNITS, MAX_EDGES>,
) -> Bool
where
    [(); cap_size(cap_of(MAX_UNITS))]:,
{
    let matrix = csr_to_bitmatrix::<MAX_UNITS, MAX_EDGES>(graph);
    let (block_count, _per_node): (USize, [USize; cap_size(cap_of(MAX_UNITS))]) =
        arvo_sparse::block_diagonal::<Bits<64, Hot>, { cap_of(MAX_UNITS) }>(&matrix);

    if block_count.0 >= 1 { Bool::TRUE } else { Bool::FALSE }
}

// ---- Wrapper 3: spectral_partition with FiberGrouping reshape ----

/// spectral engine wrapper. Takes engine CSR, returns FiberGrouping.
/// Internal flow: CSR -> dummy fiedler vector -> arvo::spectral_bisection
/// -> reshape to FiberGrouping<MAX_UNITS, MAX_FIBERS>.
///
/// The dummy fiedler vector is acceptable for the sketch because the
/// reshape is what's under test, not the spectral correctness. The
/// real wrapper will compose `fiedler_vector` (or accept it from a
/// SparseLaplacian operator) before calling spectral_bisection.
#[inline]
pub fn spectral_engine_wrapper<
    const MAX_UNITS: usize,
    const MAX_EDGES: usize,
    const MAX_FIBERS: usize,
>(
    _graph: &CsrLike<MAX_UNITS, MAX_EDGES>,
) -> FiberGroupingLike<MAX_UNITS, MAX_FIBERS>
where
    [(); cap_size(cap_of(MAX_UNITS))]:,
{
    // Dummy fiedler vector. Real flow computes this via
    // arvo_spectral::fiedler_vector(SparseLaplacian::from(&matrix), ...).
    // FastFloat<f32> picks f32; FastFloat<f64> picks f64. The spectral
    // routine is generic over `F: TotalOrd + Copy + FromConstant`.
    type Fl = arvo::FastFloat<f32>;
    let fiedler: [Fl; cap_size(cap_of(MAX_UNITS))] =
        [<Fl as Identity>::ZERO; cap_size(cap_of(MAX_UNITS))];

    let (class_count, per_node_class): (USize, [USize; cap_size(cap_of(MAX_UNITS))]) =
        arvo_spectral::spectral_bisection::<{ cap_of(MAX_UNITS) }, Fl>(&fiedler);

    // Reshape: per-element copy with class-id to FiberId conversion.
    // Class ids from arvo are in [0, class_count), bounded by 2 for
    // bisection. FiberId is Uint<7>, holds 0..=127, comfortably
    // covers both class IDs.
    let mut grouping: FiberGroupingLike<MAX_UNITS, MAX_FIBERS> = FiberGroupingLike::new();
    let mut i = 0usize;
    while i < MAX_UNITS {
        let class_id = per_node_class[i].0;
        // Cap fiber id by MAX_FIBERS; collapse to last fiber if
        // arvo's class id exceeds the engine's fiber cap. Sketch
        // policy; real policy probably wants to error or assert.
        let bounded = if class_id < MAX_FIBERS { class_id } else { MAX_FIBERS.saturating_sub(1) };
        grouping.assignment[i] = usize_to_fiber_id(bounded);
        i += 1;
    }
    grouping.fiber_count = if class_count.0 <= MAX_FIBERS {
        class_count
    } else {
        USize(MAX_FIBERS)
    };
    grouping
}

// ---- The caller: simulates compute_execution_plan consuming all three ----

pub struct PlanProxy<const MAX_UNITS: usize, const MAX_FIBERS: usize> {
    pub reordered: [UnitId; MAX_UNITS],
    pub feasible: Bool,
    pub fibers: FiberGroupingLike<MAX_UNITS, MAX_FIBERS>,
    pub unit_count: USize,
}

/// Top-level orchestrator. Mirrors compute_execution_plan's shape:
/// it calls each wrapper, reads each return value, threads results
/// through to its own return. The witness clause originates at the
/// wrappers and propagates here. Reading the returns proves the
/// engine-shape outputs are consumable by downstream code.
#[inline]
pub fn compute_plan_proxy<
    const MAX_UNITS: usize,
    const MAX_EDGES: usize,
    const MAX_FIBERS: usize,
>(
    graph: &CsrLike<MAX_UNITS, MAX_EDGES>,
    topo: &[UnitId; MAX_UNITS],
) -> PlanProxy<MAX_UNITS, MAX_FIBERS>
where
    [(); cap_size(cap_of(MAX_UNITS))]:,
{
    let reordered = rcm_engine_wrapper::<MAX_UNITS, MAX_EDGES>(graph, topo);
    let feasible = block_engine_wrapper::<MAX_UNITS, MAX_EDGES>(graph);
    let fibers = spectral_engine_wrapper::<MAX_UNITS, MAX_EDGES, MAX_FIBERS>(graph);

    // Consume reordered: read the first element. Real engine reads
    // through the whole array in step 7 (group_fibers); a one-element
    // read is enough to prove the consumer side typechecks.
    let _first_unit: UnitId = reordered[0];

    // Consume feasible: branch on it. Real engine errors out on
    // PlanError::PhaseAlignmentMismatch when this is false; the
    // sketch just stores it.
    let _is_feasible: Bool = feasible;

    // Consume fibers: read the assignment of unit 0. Real engine
    // walks the whole assignment in step 12 (assign_cores).
    let _first_fiber: FiberId = fibers.assignment[0];

    PlanProxy {
        reordered,
        feasible,
        fibers,
        unit_count: graph.unit_count,
    }
}

// ---- Concrete monomorphisations ----

pub fn monomorphise_at_64(graph: &CsrLike<64, 128>) -> PlanProxy<64, 16> {
    let topo: [UnitId; 64] = [UnitId::ZERO; 64];
    compute_plan_proxy::<64, 128, 16>(graph, &topo)
}

pub fn monomorphise_at_128(graph: &CsrLike<128, 256>) -> PlanProxy<128, 32> {
    let topo: [UnitId; 128] = [UnitId::ZERO; 128];
    compute_plan_proxy::<128, 256, 32>(graph, &topo)
}

pub fn monomorphise_at_256(graph: &CsrLike<256, 512>) -> PlanProxy<256, 64> {
    let topo: [UnitId; 256] = [UnitId::ZERO; 256];
    compute_plan_proxy::<256, 512, 64>(graph, &topo)
}
