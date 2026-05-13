//! Sketch S2: witness-propagation cost through engine-shaped caller graph.
//!
//! Models the real call structure in `mock/crates/hilavitkutin/src/plan/`:
//!
//! ```text
//! compute_execution_plan_proxy   (11 const generics; mimics compute_execution_plan)
//!   -> step_rcm_proxy            (2 const generics; mimics steps::rcm_reorder)
//!     -> arvo_wrapper            (1 const generic; the option-1 bridge)
//!       -> arvo_sparse::rcm_reorder  (real arvo call)
//! ```
//!
//! The witness `[(); cap_size(cap_of(MAX_UNITS))]:` originates at the leaf
//! wrapper and propagates up through the two intermediate fns. The 10-generic
//! return type (`PlanProxy`) is shaped to match `ExecutionPlan`'s
//! substitution-table cost.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

use arvo::{Bool, Cap, Identity, USize};
use arvo_bitmask::{BitMatrix, NodeId, cap_size};
use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence};

/// usize -> Cap bridge. Const-fn so it appears in generic-const-arg
/// position under generic_const_exprs.
#[inline]
pub const fn cap_of(n: usize) -> Cap {
    Cap(USize(n))
}

/// Proxy for the engine's `ExecutionPlan`. 10 const generics so the trait
/// solver's substitution table at every reference matches the real shape.
/// Field layout is irrelevant; the witness propagation depends only on
/// the type's presence in signatures.
pub struct PlanProxy<
    const MAX_UNITS: usize,
    const MAX_PHASES: usize,
    const MAX_TRUNKS: usize,
    const MAX_FIBERS: usize,
    const MAX_LANES: usize,
    const MAX_COLUMNS: usize,
    const MAX_COMPONENTS_PER_TRUNK: usize,
    const MAX_UNITS_PER_FIBER: usize,
    const MAX_COLUMNS_PER_FIBER: usize,
    const MAX_TRUNKS_PER_PHASE: usize,
> {
    pub unit_count: USize,
    pub phase_count: USize,
    // Single substantive field to keep the struct non-ZST and ensure the
    // monomorphisation actually does work; one [USize; MAX_UNITS] mirrors
    // ExecutionPlan's smallest array field.
    pub topo: [USize; MAX_UNITS],
    // Pad with phantoms over the remaining generics so each one is "used"
    // by the type. Without this, rustc treats unused generics as errors.
    _phases: core::marker::PhantomData<[(); MAX_PHASES]>,
    _trunks: core::marker::PhantomData<[(); MAX_TRUNKS]>,
    _fibers: core::marker::PhantomData<[(); MAX_FIBERS]>,
    _lanes: core::marker::PhantomData<[(); MAX_LANES]>,
    _columns: core::marker::PhantomData<[(); MAX_COLUMNS]>,
    _components: core::marker::PhantomData<[(); MAX_COMPONENTS_PER_TRUNK]>,
    _units_per_fiber: core::marker::PhantomData<[(); MAX_UNITS_PER_FIBER]>,
    _columns_per_fiber: core::marker::PhantomData<[(); MAX_COLUMNS_PER_FIBER]>,
    _trunks_per_phase: core::marker::PhantomData<[(); MAX_TRUNKS_PER_PHASE]>,
}

impl<
    const MAX_UNITS: usize,
    const MAX_PHASES: usize,
    const MAX_TRUNKS: usize,
    const MAX_FIBERS: usize,
    const MAX_LANES: usize,
    const MAX_COLUMNS: usize,
    const MAX_COMPONENTS_PER_TRUNK: usize,
    const MAX_UNITS_PER_FIBER: usize,
    const MAX_COLUMNS_PER_FIBER: usize,
    const MAX_TRUNKS_PER_PHASE: usize,
> PlanProxy<
    MAX_UNITS,
    MAX_PHASES,
    MAX_TRUNKS,
    MAX_FIBERS,
    MAX_LANES,
    MAX_COLUMNS,
    MAX_COMPONENTS_PER_TRUNK,
    MAX_UNITS_PER_FIBER,
    MAX_COLUMNS_PER_FIBER,
    MAX_TRUNKS_PER_PHASE,
>
{
    pub const fn new() -> Self {
        Self {
            unit_count: USize::ZERO,
            phase_count: USize::ZERO,
            topo: [USize::ZERO; MAX_UNITS],
            _phases: core::marker::PhantomData,
            _trunks: core::marker::PhantomData,
            _fibers: core::marker::PhantomData,
            _lanes: core::marker::PhantomData,
            _columns: core::marker::PhantomData,
            _components: core::marker::PhantomData,
            _units_per_fiber: core::marker::PhantomData,
            _columns_per_fiber: core::marker::PhantomData,
            _trunks_per_phase: core::marker::PhantomData,
        }
    }
}

// ---- LEAF: the option-1 wrapper from sketch S1 ----

/// Leaf wrapper. The witness originates here.
#[inline]
pub fn arvo_wrapper<W, const MAX_UNITS: usize>(
    adjacency: &BitMatrix<W, { cap_of(MAX_UNITS) }>,
) -> [NodeId; cap_size(cap_of(MAX_UNITS))]
where
    W: BitSequence + BitAccess + BitLogic + Identity + Copy + Default,
    [(); cap_size(cap_of(MAX_UNITS))]:,
{
    arvo_sparse::rcm_reorder::<W, { cap_of(MAX_UNITS) }>(adjacency)
}

// ---- LEVEL 1: mimics steps::rcm_reorder ----

/// Two-const-generic intermediate. The witness must propagate here because
/// the body calls `arvo_wrapper` over MAX_UNITS. The witness clause is the
/// minimum to make rustc accept the call.
#[inline]
pub fn step_rcm_proxy<W, const MAX_UNITS: usize, const MAX_EDGES: usize>(
    adjacency: &BitMatrix<W, { cap_of(MAX_UNITS) }>,
    _edges_placeholder: [USize; MAX_EDGES],
) -> [NodeId; cap_size(cap_of(MAX_UNITS))]
where
    W: BitSequence + BitAccess + BitLogic + Identity + Copy + Default,
    [(); cap_size(cap_of(MAX_UNITS))]:,
{
    arvo_wrapper::<W, MAX_UNITS>(adjacency)
}

// ---- LEVEL 2: mimics compute_execution_plan ----

/// Eleven-const-generic orchestrator. Returns a 10-generic PlanProxy.
/// The witness MUST propagate here because the body calls step_rcm_proxy,
/// which carries the witness. This is the load-bearing question:
/// does the compiler get exponentially slow under this propagation?
#[inline]
pub fn compute_plan_proxy<
    W,
    const MAX_UNITS: usize,
    const MAX_EDGES: usize,
    const MAX_PHASES: usize,
    const MAX_TRUNKS: usize,
    const MAX_FIBERS: usize,
    const MAX_LANES: usize,
    const MAX_COLUMNS: usize,
    const MAX_COMPONENTS_PER_TRUNK: usize,
    const MAX_UNITS_PER_FIBER: usize,
    const MAX_COLUMNS_PER_FIBER: usize,
    const MAX_TRUNKS_PER_PHASE: usize,
>(
    adjacency: &BitMatrix<W, { cap_of(MAX_UNITS) }>,
    edges: [USize; MAX_EDGES],
) -> PlanProxy<
    MAX_UNITS,
    MAX_PHASES,
    MAX_TRUNKS,
    MAX_FIBERS,
    MAX_LANES,
    MAX_COLUMNS,
    MAX_COMPONENTS_PER_TRUNK,
    MAX_UNITS_PER_FIBER,
    MAX_COLUMNS_PER_FIBER,
    MAX_TRUNKS_PER_PHASE,
>
where
    W: BitSequence + BitAccess + BitLogic + Identity + Copy + Default,
    [(); cap_size(cap_of(MAX_UNITS))]:,
{
    let order = step_rcm_proxy::<W, MAX_UNITS, MAX_EDGES>(adjacency, edges);
    // The arvo result is `[NodeId; cap_size(cap_of(MAX_UNITS))]`. We do NOT
    // attempt to copy it into the engine-shape `[USize; MAX_UNITS]` here;
    // this sketch focuses purely on witness propagation. Just observe the
    // result and discard.
    let _ = order;
    PlanProxy::new()
}

// ---- CONCRETE MONOMORPHISATION ----
//
// Without a concrete instantiation, rustc may skip the full trait-solver
// work on the generic bodies. The real engine has consumer-app entrypoints
// that bind every const generic; this fn models that boundary.

/// Concrete monomorphisation at engine-realistic sizes. If this fn
/// compiles, the trait solver has done the full per-impl witness work
/// for every type substitution along the chain.
pub fn force_full_monomorphisation_64() -> USize {
    use arvo::Hot;
    use arvo_bits::Bits;

    let adjacency: BitMatrix<Bits<64, Hot>, { cap_of(64) }> = BitMatrix::empty();
    let edges: [USize; 128] = [USize::ZERO; 128];
    let plan = compute_plan_proxy::<
        Bits<64, Hot>,
        64,   // MAX_UNITS
        128,  // MAX_EDGES
        16,   // MAX_PHASES
        32,   // MAX_TRUNKS
        64,   // MAX_FIBERS
        8,    // MAX_LANES
        128,  // MAX_COLUMNS
        16,   // MAX_COMPONENTS_PER_TRUNK
        32,   // MAX_UNITS_PER_FIBER
        16,   // MAX_COLUMNS_PER_FIBER
        16,   // MAX_TRUNKS_PER_PHASE
    >(&adjacency, edges);
    plan.unit_count
}

/// Second concrete instantiation at different sizes. Forces a second
/// monomorphisation, which doubles the witness work the trait solver
/// must do. If the cost is linear in monomorphisations, two should
/// roughly double the warm-build time; if super-linear, it explodes.
pub fn force_full_monomorphisation_128() -> USize {
    use arvo::Hot;
    use arvo_bits::Bits;

    let adjacency: BitMatrix<Bits<64, Hot>, { cap_of(128) }> = BitMatrix::empty();
    let edges: [USize; 256] = [USize::ZERO; 256];
    let plan = compute_plan_proxy::<
        Bits<64, Hot>,
        128, 256, 32, 64, 128, 16, 256, 32, 64, 32, 32,
    >(&adjacency, edges);
    plan.unit_count
}

/// Third concrete instantiation at engine-typical-large sizes. If the
/// trait solver's cost compounds super-linearly, the third instantiation
/// is where exponential blowup starts to show.
pub fn force_full_monomorphisation_256() -> USize {
    use arvo::Hot;
    use arvo_bits::Bits;

    let adjacency: BitMatrix<Bits<64, Hot>, { cap_of(256) }> = BitMatrix::empty();
    let edges: [USize; 512] = [USize::ZERO; 512];
    let plan = compute_plan_proxy::<
        Bits<64, Hot>,
        256, 512, 64, 128, 256, 32, 512, 64, 128, 64, 64,
    >(&adjacency, edges);
    plan.unit_count
}
