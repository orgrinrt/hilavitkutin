//! GATE-2 const grouping: compile-time phase + trunk from the registered bundle.
//!
//! The grouping is a pure function of the registered access-set types. Each
//! unit's read and write sets fold to store-bit `AccessMask`s over the global
//! `Stores` numbering (the existing const `MaskProject`); read-after-write edges
//! between masks give a phase depth (longest dependency path); within a phase,
//! units that conflict on a written column form one trunk (column-disjoint
//! writes run with zero cross-trunk synchronisation). The result drives the
//! const-gated dispatch walk in a following round: `const { phase_of(POS) ==
//! PHASE && trunk_of(POS) == TRUNK }` collapses a flat walk into one member-only
//! program per phase and trunk.
//!
//! Proven shapes: const-trait fold + const-fn grouping (sketch `202606070800`),
//! the real `MaskProject` fold with a threaded witness list (sketch
//! `202606070950`), the canonical phase + within-phase-component const fns
//! (sketch `202606071330`).
//!
//! The grouping is exposed as const fns keyed by `(Wus, Stores, Witnesses, CU,
//! CS)`. `Witnesses` is the per-unit projection-index list threaded from
//! `build()` / `run()` exactly as `compute_plan` threads `BWit`, so the grouping
//! is a concrete const at the dispatch gate. The fns build `CU`-capacity locals
//! (`generic_const_exprs`), not generic-length const items, so no
//! `generic_const_items` feature is needed.

use arvo::Bool;
use arvo::USize;
use arvo::strategy::Identity;
use arvo_bitmask::{BitAccess, NodeId};
use arvo_graph::waist_detect_const;
use arvo_tensor::{Capacity, ConstCapacity, cap_size};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::meta::{RANK_CONSUMER, RANK_PLAN_STAGE};
use hilavitkutin_api::work_unit::{HasSchedule, Lifecycle, WorkUnit};
use hilavitkutin_api::work_unit_values::{WuCons, WuNil};

use super::access::AccessMask;
use super::project::{MaskProject, WitnessIndex};

/// Fixed upper bound on registered units for the GATE-2 const grouping arrays.
///
/// The grouping fns size their scratch by this constant rather than
/// `cap_size(CU::CAP)`: a `cap_size(CU::CAP)` array-length bound, re-proven
/// through the const-gated walk's type-level recursion, overflows the trait
/// solver (a generic-constant well-formedness loop). A plain const sidesteps
/// that, and the fold/grouping loops bound by the actual unit count (not this
/// size), so a generous fixed cap costs only stack scratch, not compile time. A
/// consumer registering more units than this raises the constant; the default
/// covers the engine's `DefaultPlanDims` unit capacity.
pub const GATE2_MAX_UNITS: usize = 256; // lint:allow(no-bare-numeric) reason: const array-length bound (GCE grammar requires usize); tracked: #121

/// Per-core accumulator-live scratch bound (GATE-2 deviation 9 threaded
/// accumulator path). Bounds the worker-stack live-count array and the
/// scheduler's per-core publish array (`MAX_CORES * GATE2_MAX_ACCUMS` cells). A
/// pipeline registers far fewer accumulators than this; a generous cap costs
/// only fixed scratch. A consumer with more accumulators raises the constant.
pub const GATE2_MAX_ACCUMS: usize = 16; // lint:allow(no-bare-numeric) reason: const array-length bound; tracked: #121

/// What the grouping fold needs from each unit: its read and write access sets.
///
/// Blanket-implemented for every `WorkUnit`, so a registered work-unit bundle
/// satisfies it without any per-unit boilerplate. The indirection lets the
/// grouping be exercised by lightweight fixtures (access sets without the full
/// dispatch machinery) while the engine folds over real units.
pub trait UnitAccess {
    /// The unit's read access set.
    type Read;
    /// The unit's write access set.
    type Write;
    /// The unit's lifecycle rank (E4 slice 2): the outer phase key the grouping
    /// renumbers by, so plan-stage meta units land before consumers and the
    /// schedule-end epilogue after. Defaults to consumer rank, so lightweight
    /// access-set fixtures (which do not carry a schedule) are consumer-ranked.
    const RANK: USize = RANK_CONSUMER;
}

// E4 slice 1: schedule-recovered so an `On<V>` unit (which impls
// `WorkUnit<On<V>>`, not `WorkUnit<Always>`) is also covered by the blanket.
// E4 slice 2: RANK reads the schedule's lifecycle rank (Always / On<V> are
// consumer-rank; OnMeta<V> takes its meta virtual's rank).
impl<W: HasSchedule + WorkUnit<<W as HasSchedule>::Sched>> UnitAccess for W {
    type Read = <W as WorkUnit<<W as HasSchedule>::Sched>>::Read;
    type Write = <W as WorkUnit<<W as HasSchedule>::Sched>>::Write;
    const RANK: USize = <<W as HasSchedule>::Sched as Lifecycle>::RANK;
}

/// Const fold of a registered bundle's per-unit read/write masks.
///
/// The const analog of the runtime `BundleProject`: for each unit it projects
/// its read and write access sets into `AccessMask`s via the const `MaskProject`
/// and writes them at the unit's carrier position. `Witnesses` is the parallel
/// per-unit `(ReadIdx, WriteIdx)` projection-index list (the `BundleProject`
/// shape), threaded so each `MaskProject` index stays constrained. The mask
/// arrays are slices, so the trait carries no array-length parameter.
pub const trait BundleMasks<Stores, Witnesses, CS: Capacity> {
    /// Fill per-unit read/write masks and lifecycle ranks from carrier position
    /// `idx`, returning the next free position (so the top-level caller learns
    /// the unit count). The rank (E4 slice 2) is `UnitAccess::RANK`, the outer
    /// phase key; folding it here reuses the carrier walk the masks already do.
    fn fill(
        reads: &mut [AccessMask<CS>],
        writes: &mut [AccessMask<CS>],
        ranks: &mut [USize],
        idx: USize,
    ) -> USize;
}

const impl<Stores, CS: Capacity> BundleMasks<Stores, Empty, CS> for Empty {
    #[inline]
    fn fill(
        _reads: &mut [AccessMask<CS>],
        _writes: &mut [AccessMask<CS>],
        _ranks: &mut [USize],
        idx: USize,
    ) -> USize {
        idx
    }
}

const impl<Stores, Un, T, RI, WI, WT, CS: Capacity> BundleMasks<Stores, Cons<(RI, WI), WT>, CS>
    for Cons<Un, T>
where
    Un: UnitAccess,
    Un::Read: [const] MaskProject<Stores, RI, CS>,
    Un::Write: [const] MaskProject<Stores, WI, CS>,
    T: [const] BundleMasks<Stores, WT, CS>,
{
    #[inline]
    fn fill(
        reads: &mut [AccessMask<CS>],
        writes: &mut [AccessMask<CS>],
        ranks: &mut [USize],
        idx: USize,
    ) -> USize {
        let i = idx.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: slice index; tracked: #121
        reads[i] = <Un::Read as MaskProject<Stores, RI, CS>>::project_mask(AccessMask::empty());
        writes[i] = <Un::Write as MaskProject<Stores, WI, CS>>::project_mask(AccessMask::empty());
        ranks[i] = <Un as UnitAccess>::RANK;
        <T as BundleMasks<Stores, WT, CS>>::fill(reads, writes, ranks, USize(i + 1)) // lint:allow(no-bare-numeric) reason: carrier-position successor; tracked: #121
    }
}

// The same fold over the VALUE carrier `WuCons` / `WuNil`. `Scheduler::run`
// holds its units as `WuVals` (a `WuCons` list of values), not the builder-only
// `Cons`-shaped bundle, so the grouping at dispatch folds this shape. The bodies
// mirror the `Cons` / `Empty` impls above; only the carrier cell type differs.
const impl<Stores, CS: Capacity> BundleMasks<Stores, Empty, CS> for WuNil {
    #[inline]
    fn fill(
        _reads: &mut [AccessMask<CS>],
        _writes: &mut [AccessMask<CS>],
        _ranks: &mut [USize],
        idx: USize,
    ) -> USize {
        idx
    }
}

const impl<Stores, Un, T, RI, WI, WT, CS: Capacity> BundleMasks<Stores, Cons<(RI, WI), WT>, CS>
    for WuCons<Un, T>
where
    Un: UnitAccess,
    Un::Read: [const] MaskProject<Stores, RI, CS>,
    Un::Write: [const] MaskProject<Stores, WI, CS>,
    T: [const] BundleMasks<Stores, WT, CS>,
{
    #[inline]
    fn fill(
        reads: &mut [AccessMask<CS>],
        writes: &mut [AccessMask<CS>],
        ranks: &mut [USize],
        idx: USize,
    ) -> USize {
        let i = idx.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: slice index; tracked: #121
        reads[i] = <Un::Read as MaskProject<Stores, RI, CS>>::project_mask(AccessMask::empty());
        writes[i] = <Un::Write as MaskProject<Stores, WI, CS>>::project_mask(AccessMask::empty());
        ranks[i] = <Un as UnitAccess>::RANK;
        <T as BundleMasks<Stores, WT, CS>>::fill(reads, writes, ranks, USize(i + 1)) // lint:allow(no-bare-numeric) reason: carrier-position successor; tracked: #121
    }
}

/// Renumber `(rank, waist_phase)` pairs into contiguous lifecycle-ordered phase
/// ids (E4 slice 2).
///
/// `out[i]` = the count of DISTINCT pairs present in the unit set lex-strictly
/// less than unit i's `(rank, waist_phase)`, counted by first occurrence. This
/// makes the rank the outer phase key (plan-stage bands before consumers,
/// schedule-end after) while preserving the waist order within a rank, with
/// equal pairs sharing an id. When all units are consumer-rank (no meta units)
/// and the waist phases are already contiguous, this is the identity, so the
/// common case is unchanged. Proven: sketch `202606082200`.
const fn compute_rank_renumber(ranks: &[USize], waist: &[USize], n: USize, out: &mut [USize]) {
    let count = n.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: loop bound; tracked: #121
    let mut i = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
    while i < count {
        let ri = ranks[i].0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: rank compare; tracked: #121
        let wi = waist[i].0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: waist compare; tracked: #121
        let mut c = 0; // lint:allow(no-bare-numeric) reason: distinct-smaller count; tracked: #121
        let mut j = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
        while j < count {
            let rj = ranks[j].0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: rank compare; tracked: #121
            let wj = waist[j].0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: waist compare; tracked: #121
            if rj < ri || (rj == ri && wj < wi) {
                // count j only if it is the first occurrence of its pair
                let mut first = true;
                let mut k = 0; // lint:allow(no-bare-numeric) reason: dedup scan index; tracked: #121
                while k < j {
                    if ranks[k].0 == rj && waist[k].0 == wj {
                        first = false;
                    }
                    k += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
                }
                if first {
                    c += 1; // lint:allow(no-bare-numeric) reason: distinct-smaller successor; tracked: #121
                }
            }
            j += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
        }
        out[i] = USize(c);
        i += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
}

/// Compute the canonical WAIST-BOUNDED phase of every unit into the pre-zeroed
/// `phase_out` slice.
///
/// The read-after-write edges (`reads[j]` overlaps `writes[i]` gives edge
/// `i -> j`) form the unit dependency DAG. Its waist-delimited sections are the
/// phases: a unit-by-unit RAW adjacency over the row word `Adj` is fed to arvo's
/// const `waist_detect_const` over the unit capacity `CU`, then the canonical
/// mapping (runtime `compute_waists`, `plan/steps.rs:314-326`) assigns each unit
/// the count of waist flags at positions strictly before it (phase 0 starts at
/// position 0; each waist position opens a new phase at the next position, so the
/// waist unit is the last of its phase). A producer to consumer chain with no
/// interior narrowing is therefore one phase, where the old longest-depth axis
/// would have split it.
///
/// The identity carrier order is the topological order (registration order,
/// build-validated topological). The caller sizes and zeroes `phase_out`.
const fn compute_phases_waist<CU, CS: Capacity, Adj>(
    reads: &[AccessMask<CS>],
    writes: &[AccessMask<CS>],
    n: USize,
    phase_out: &mut [USize],
) where
    CU: [const] ConstCapacity,
    Adj: [const] BitAccess + Identity,
{
    let count = n.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: loop bound; tracked: #121
    let cap = cap_size(<CU as ConstCapacity>::CAP);

    // Identity topo order over the full unit capacity (slack tail past `count`
    // is isolated, exactly as the runtime waist path walks `D::Units`).
    let mut order = CU::filled(NodeId(USize(0)));
    let mut k = 0; // lint:allow(no-bare-numeric) reason: order-fill index; tracked: #121
    while k < cap {
        CU::set(&mut order, USize(k), NodeId(USize(k)));
        k += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }

    // Unit-by-unit RAW adjacency: row `i` bit `j` set iff `reads[j]` overlaps
    // `writes[i]` (edge `i -> j`).
    let mut adj = CU::filled(<Adj as Identity>::ZERO);
    let mut i = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
    while i < count {
        let mut j = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
        while j < count {
            if i != j && reads[j].overlaps(&writes[i]).0 {
                let row = CU::get(&adj, USize(i));
                CU::set(&mut adj, USize(i), row.with_bit_set(USize(j)));
            }
            j += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
        }
        i += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }

    // Waist flags per topo position, then canonical prefix-count to phases.
    let flags = waist_detect_const::<CU, Adj>(&adj, &order);
    let mut c = 0; // lint:allow(no-bare-numeric) reason: running phase (waist count); tracked: #121
    let mut p = 0; // lint:allow(no-bare-numeric) reason: position index; tracked: #121
    while p < count {
        phase_out[p] = USize(c);
        if CU::get(&flags, USize(p)).0 {
            c += 1; // lint:allow(no-bare-numeric) reason: phase successor at waist; tracked: #121
        }
        p += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
}

/// Compute the trunk (within-phase column-conflict component, canonicalised to
/// the smallest member position) of every unit into `trunk_out`.
///
/// `trunk_out` doubles as the union-find parent scratch. Two same-phase units
/// conflict (one trunk) when one writes a column the other accesses; read-only
/// sharing does not conflict, so column-disjoint-write units land in distinct
/// trunks. Union points the larger root at the smaller, so each component's id
/// is its minimum member position. The caller sizes `trunk_out` (initial
/// contents ignored).
const fn compute_trunks<CS: Capacity>(
    reads: &[AccessMask<CS>],
    writes: &[AccessMask<CS>],
    phase: &[USize],
    n: USize,
    trunk_out: &mut [USize],
) {
    let count = n.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: loop bound; tracked: #121
    let mut k = 0; // lint:allow(no-bare-numeric) reason: parent-init index; tracked: #121
    while k < count {
        trunk_out[k] = USize(k); // lint:allow(no-bare-numeric) reason: parent = self; tracked: #121
        k += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
    let mut a = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
    while a < count {
        let mut b = a + 1; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
        while b < count {
            let same_phase = phase[a].0 == phase[b].0; // lint:allow(no-bare-numeric) reason: phase compare; tracked: #121
            let conflict = writes[a].overlaps(&reads[b]).0
                || writes[a].overlaps(&writes[b]).0
                || reads[a].overlaps(&writes[b]).0;
            if same_phase && conflict {
                let mut ra = a; // lint:allow(no-bare-numeric) reason: root walk; tracked: #121
                while trunk_out[ra].0 != ra {
                    ra = trunk_out[ra].0; // lint:allow(no-bare-numeric) reason: parent deref; tracked: #121
                }
                let mut rb = b; // lint:allow(no-bare-numeric) reason: root walk; tracked: #121
                while trunk_out[rb].0 != rb {
                    rb = trunk_out[rb].0; // lint:allow(no-bare-numeric) reason: parent deref; tracked: #121
                }
                if ra < rb {
                    trunk_out[rb] = USize(ra);
                } else if rb < ra {
                    trunk_out[ra] = USize(rb);
                }
            }
            b += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
        }
        a += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
    let mut u = 0; // lint:allow(no-bare-numeric) reason: canonicalise index; tracked: #121
    while u < count {
        let mut r = u; // lint:allow(no-bare-numeric) reason: root walk; tracked: #121
        while trunk_out[r].0 != r {
            r = trunk_out[r].0; // lint:allow(no-bare-numeric) reason: parent deref; tracked: #121
        }
        trunk_out[u] = USize(r); // lint:allow(no-bare-numeric) reason: component-min id; tracked: #121
        u += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
}

/// Fold the per-unit read/write masks and lifecycle ranks for a registered
/// bundle into `CU`-capacity locals, returning `(reads, writes, ranks,
/// unit_count)`.
const fn masks_of<Wus, Stores, Witnesses, CU: Capacity + [const] ConstCapacity, CS: Capacity>() -> (
    <CU as ConstCapacity>::Array<AccessMask<CS>>,
    <CU as ConstCapacity>::Array<AccessMask<CS>>,
    <CU as ConstCapacity>::Array<USize>,
    USize,
)
where
    Wus: [const] BundleMasks<Stores, Witnesses, CS>,
{
    // The grouping scratch is a `CU`-capacity GAT array, sliced into the
    // slice-based `BundleMasks::fill` via the `ConstCapacity` slice bridge.
    // `fill` is slice-based, so the per-cons-cell `MaskProject` recursion is
    // capacity-blind (CU appears only here, never in the recursive
    // obligation); the P0.1b probe confirmed no trait-solver overflow. Consumers
    // index the returned arrays through `<CU as ConstCapacity>::get`.
    let mut reads = <CU as ConstCapacity>::filled(AccessMask::<CS>::empty());
    let mut writes = <CU as ConstCapacity>::filled(AccessMask::<CS>::empty());
    let mut ranks = <CU as ConstCapacity>::filled(USize::ZERO);
    let n = <Wus as BundleMasks<Stores, Witnesses, CS>>::fill(
        <CU as ConstCapacity>::slice_mut(&mut reads),
        <CU as ConstCapacity>::slice_mut(&mut writes),
        <CU as ConstCapacity>::slice_mut(&mut ranks),
        USize::ZERO,
    );
    (reads, writes, ranks, n)
}

/// Compute the FINAL lifecycle-ordered phase of every unit (E4 slice 2).
///
/// The waist-bounded phase (`compute_phases_waist`) is the inner key; each
/// unit's lifecycle rank (folded by `BundleMasks::fill`) is the outer key;
/// `compute_rank_renumber` folds them into contiguous bands. Returns
/// `(phases, unit_count)`. When all units are consumer-rank the renumber is the
/// identity, so the waist phases are unchanged.
const fn final_phases_of<
    Wus,
    Stores,
    Witnesses,
    CU: Capacity + [const] ConstCapacity,
    CS: Capacity,
    Adj,
>() -> (<CU as ConstCapacity>::Array<USize>, USize)
where
    Wus: [const] BundleMasks<Stores, Witnesses, CS>,
    Adj: [const] BitAccess + Identity,
{
    let (reads, writes, ranks, n) = masks_of::<Wus, Stores, Witnesses, CU, CS>();
    let mut waist = <CU as ConstCapacity>::filled(USize::ZERO);
    compute_phases_waist::<CU, CS, Adj>(
        <CU as ConstCapacity>::slice(&reads),
        <CU as ConstCapacity>::slice(&writes),
        n,
        <CU as ConstCapacity>::slice_mut(&mut waist),
    );
    let mut phase = <CU as ConstCapacity>::filled(USize::ZERO);
    compute_rank_renumber(
        <CU as ConstCapacity>::slice(&ranks),
        <CU as ConstCapacity>::slice(&waist),
        n,
        <CU as ConstCapacity>::slice_mut(&mut phase),
    );
    (phase, n)
}

/// Number of units registered in the bundle.
pub const fn group_n<Wus, Stores, Witnesses, CU: Capacity + [const] ConstCapacity, CS: Capacity>()
-> USize
where
    Wus: [const] BundleMasks<Stores, Witnesses, CS>,
{
    let (_reads, _writes, _ranks, n) = masks_of::<Wus, Stores, Witnesses, CU, CS>();
    n
}

/// Lifecycle-ordered phase of the unit at carrier position `pos` (E4 slice 2:
/// the rank-outer renumber of the waist-bounded phase). `Adj` is the adjacency
/// row word (the plan's `D::AdjRow`).
pub const fn phase_of<
    Wus,
    Stores,
    Witnesses,
    CU: Capacity + [const] ConstCapacity,
    CS: Capacity,
    Adj,
>(
    pos: USize,
) -> USize
where
    Wus: [const] BundleMasks<Stores, Witnesses, CS>,
    Adj: [const] BitAccess + Identity,
{
    let (phase, _n) = final_phases_of::<Wus, Stores, Witnesses, CU, CS, Adj>();
    <CU as ConstCapacity>::get(&phase, pos)
}

/// Trunk (within-phase column-conflict component id) of the unit at `pos`,
/// keyed on the lifecycle-ordered phase. `Adj` is the adjacency row word.
pub const fn trunk_of<
    Wus,
    Stores,
    Witnesses,
    CU: Capacity + [const] ConstCapacity,
    CS: Capacity,
    Adj,
>(
    pos: USize,
) -> USize
where
    Wus: [const] BundleMasks<Stores, Witnesses, CS>,
    Adj: [const] BitAccess + Identity,
{
    let (reads, writes, _ranks, n) = masks_of::<Wus, Stores, Witnesses, CU, CS>();
    let (phase, _n) = final_phases_of::<Wus, Stores, Witnesses, CU, CS, Adj>();
    let mut trunk = <CU as ConstCapacity>::filled(USize::ZERO);
    compute_trunks::<CS>(
        <CU as ConstCapacity>::slice(&reads),
        <CU as ConstCapacity>::slice(&writes),
        <CU as ConstCapacity>::slice(&phase),
        n,
        <CU as ConstCapacity>::slice_mut(&mut trunk),
    );
    <CU as ConstCapacity>::get(&trunk, pos)
}

/// Whether the unit at carrier position `Pos` belongs to phase `PHASE`, trunk
/// `TRUNK`.
///
/// `Pos` is the carrier position as a type-level Peano witness
/// (`Here` / `There<..>`, `Pos::INDEX` is its usize), NOT a `const POS: usize`:
/// the const-gated walk recurses by threading `There<Pos>` (a type, no const
/// arithmetic), which avoids the `{POS + 1}` generic-constant the trait solver
/// overflows normalising through the recursion. A `const fn` so the gate reads
/// `Member::<..>::IS`, a simple associated const.
pub const fn is_member<
    Wus,
    Stores,
    Witnesses,
    CU: Capacity + [const] ConstCapacity,
    CS: Capacity,
    Adj,
    Pos: WitnessIndex,
    const PHASE: usize,
    const TRUNK: usize,
>() -> Bool
// lint:allow(no-bare-numeric) reason: const-generic phase/trunk carriers; tracked: #121
where
    Wus: [const] BundleMasks<Stores, Witnesses, CS>,
    Adj: [const] BitAccess + Identity,
{
    let p = phase_of::<Wus, Stores, Witnesses, CU, CS, Adj>(Pos::INDEX);
    let t = trunk_of::<Wus, Stores, Witnesses, CU, CS, Adj>(Pos::INDEX);
    Bool(p.0 == PHASE && t.0 == TRUNK) // lint:allow(no-bare-numeric) reason: grouping membership compare; tracked: #121
}

/// Number of phases in the bundle: max phase depth + 1, or zero for an empty
/// bundle. The dispatcher's phase-loop bound.
pub const fn phase_count<
    Wus,
    Stores,
    Witnesses,
    CU: Capacity + [const] ConstCapacity,
    CS: Capacity,
    Adj,
>() -> USize
where
    Wus: [const] BundleMasks<Stores, Witnesses, CS>,
    Adj: [const] BitAccess + Identity,
{
    let (phase, n) = final_phases_of::<Wus, Stores, Witnesses, CU, CS, Adj>();
    let count = n.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: loop bound; tracked: #121
    if count == 0 {
        return USize::ZERO;
    }
    let mut maxp = 0; // lint:allow(no-bare-numeric) reason: running max phase; tracked: #121
    let mut i = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
    while i < count {
        let pi = <CU as ConstCapacity>::get(&phase, USize(i)).0; // lint:allow(no-bare-numeric) reason: phase value read; tracked: #121
        if pi > maxp {
            maxp = pi;
        }
        i += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
    USize(maxp + 1) // lint:allow(no-bare-numeric) reason: phase count = max depth + 1; tracked: #121
}

/// Number of leading plan-stage (`RANK_PLAN_STAGE`) phase bands (E4 slice 2).
///
/// The rank-outer renumber gives plan-stage units the lowest phase ids, so the
/// plan band is the contiguous block `0..plan_phase_count`. The kernel skips
/// these phases on a clean (not plan-dirty) frame, so `OnMeta<PlanStage>` units
/// run only when the plan is recomputed. Zero when no plan-stage unit exists.
pub const fn plan_phase_count<
    Wus,
    Stores,
    Witnesses,
    CU: Capacity + [const] ConstCapacity,
    CS: Capacity,
    Adj,
>() -> USize
where
    Wus: [const] BundleMasks<Stores, Witnesses, CS>,
    Adj: [const] BitAccess + Identity,
{
    let (_reads, _writes, ranks, _n) = masks_of::<Wus, Stores, Witnesses, CU, CS>();
    let (phase, n) = final_phases_of::<Wus, Stores, Witnesses, CU, CS, Adj>();
    let count = n.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: loop bound; tracked: #121
    let mut maxp1 = 0; // lint:allow(no-bare-numeric) reason: plan-band phase count (max plan phase + 1); tracked: #121
    let mut i = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
    while i < count {
        if <CU as ConstCapacity>::get(&ranks, USize(i)).0 == RANK_PLAN_STAGE.0
            && <CU as ConstCapacity>::get(&phase, USize(i)).0 + 1 > maxp1
        {
            maxp1 = <CU as ConstCapacity>::get(&phase, USize(i)).0 + 1; // lint:allow(no-bare-numeric) reason: plan-band phase successor; tracked: #121
        }
        i += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
    USize(maxp1)
}

/// Number of leading phases occupied by units below `RANK_CONSUMER` (E4 parity).
///
/// The rank-outer renumber places every pre-consumer lifecycle band (plan,
/// schedule-ready, pass-start) in the lowest phase ids, so the leading meta
/// block is the contiguous range `0..pre_consumer_phase_count`. Zero when the
/// carrier has no pre-consumer meta unit.
pub const fn pre_consumer_phase_count<
    Wus,
    Stores,
    Witnesses,
    CU: Capacity + [const] ConstCapacity,
    CS: Capacity,
    Adj,
>() -> USize
where
    Wus: [const] BundleMasks<Stores, Witnesses, CS>,
    Adj: [const] BitAccess + Identity,
{
    let (_reads, _writes, ranks, _n) = masks_of::<Wus, Stores, Witnesses, CU, CS>();
    let (phase, n) = final_phases_of::<Wus, Stores, Witnesses, CU, CS, Adj>();
    let count = n.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: loop bound; tracked: #121
    let mut maxp1 = 0; // lint:allow(no-bare-numeric) reason: leading-band phase count (max pre-consumer phase + 1); tracked: #121
    let mut i = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
    while i < count {
        if <CU as ConstCapacity>::get(&ranks, USize(i)).0 < RANK_CONSUMER.0
            && <CU as ConstCapacity>::get(&phase, USize(i)).0 + 1 > maxp1
        {
            maxp1 = <CU as ConstCapacity>::get(&phase, USize(i)).0 + 1; // lint:allow(no-bare-numeric) reason: leading-band phase successor; tracked: #121
        }
        i += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
    USize(maxp1)
}

/// End of the consumer band: max phase among units at or below `RANK_CONSUMER`,
/// plus one (E4 parity).
///
/// The trailing meta block (schedule-end epilogue) is the contiguous range
/// `consumer_phase_end..phase_count`. Equals `pre_consumer_phase_count` when no
/// consumer unit exists.
pub const fn consumer_phase_end<
    Wus,
    Stores,
    Witnesses,
    CU: Capacity + [const] ConstCapacity,
    CS: Capacity,
    Adj,
>() -> USize
where
    Wus: [const] BundleMasks<Stores, Witnesses, CS>,
    Adj: [const] BitAccess + Identity,
{
    let (_reads, _writes, ranks, _n) = masks_of::<Wus, Stores, Witnesses, CU, CS>();
    let (phase, n) = final_phases_of::<Wus, Stores, Witnesses, CU, CS, Adj>();
    let count = n.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: loop bound; tracked: #121
    let mut maxp1 = 0; // lint:allow(no-bare-numeric) reason: consumer-band end (max phase at-or-below consumer rank + 1); tracked: #121
    let mut i = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
    while i < count {
        if <CU as ConstCapacity>::get(&ranks, USize(i)).0 <= RANK_CONSUMER.0
            && <CU as ConstCapacity>::get(&phase, USize(i)).0 + 1 > maxp1
        {
            maxp1 = <CU as ConstCapacity>::get(&phase, USize(i)).0 + 1; // lint:allow(no-bare-numeric) reason: consumer-band phase successor; tracked: #121
        }
        i += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
    USize(maxp1)
}

/// Per-carrier-position mask of consumer-rank units (E4 parity).
///
/// Bit `i` is set when carrier position `i` holds a unit of `RANK_CONSUMER`.
/// The unit-outer per-core slice walk gates on this mask so meta units do not
/// ride the slice once per core; the designated thread dispatches them once per
/// frame instead. All live positions are set when the carrier has no meta unit.
pub const fn consumer_mask<
    Wus,
    Stores,
    Witnesses,
    CU: Capacity + [const] ConstCapacity,
    CS: Capacity,
    Adj,
>() -> Adj
where
    Wus: [const] BundleMasks<Stores, Witnesses, CS>,
    Adj: [const] BitAccess + Identity,
{
    let (_reads, _writes, ranks, n) = masks_of::<Wus, Stores, Witnesses, CU, CS>();
    let count = n.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: loop bound; tracked: #121
    let mut m = <Adj as Identity>::ZERO;
    let mut i = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
    while i < count {
        if <CU as ConstCapacity>::get(&ranks, USize(i)).0 == RANK_CONSUMER.0 {
            m = m.with_bit_set(USize(i));
        }
        i += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
    m
}
