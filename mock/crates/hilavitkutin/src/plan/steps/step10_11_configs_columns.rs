//! Step 10 (per-phase config selection) and step 11 (per-fiber column
//! classification).
//!
//! Split out of `plan/steps.rs` (file-size lint). No behaviour change.

use arvo::USize;
use arvo_tensor::{Capacity, cap_size};

use super::{
    ColumnClassMap,
    ColumnClassification,
    FiberGrouping,
    PhaseBoundaries,
    PhaseConfig,
    PlanDims,
    PlanInputs,
};

/// Heuristic threshold below which a phase is treated as "small" and
/// picks `MaxFuse`. Substrate-default; consumers will be able to tune
/// this once `RunCfg`-level phase-policy lands in Pass 3 / Pass 6.
/// Tracked as a follow-up under task #429 (review-driven).
const SMALL_RECORD_COUNT_THRESHOLD: usize = 10_000; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: substrate-default policy threshold; rust grammar requires usize; tracked: #429

/// Heuristic phase-width threshold above which a phase picks
/// `MaxSplit`. Substrate-default; same tuning story as
/// `SMALL_RECORD_COUNT_THRESHOLD`. Tracked under #429.
const WIDE_PHASE_WIDTH_THRESHOLD: usize = 8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: substrate-default policy threshold; rust grammar requires usize; tracked: #429

/// Step 10: per-phase config selection (MaxFuse / Balanced / MaxSplit).
///
/// Picks based on phase width (number of fibers in the phase) and
/// record count: small phases pick `MaxFuse` to minimise dispatch
/// overhead; wide phases pick `MaxSplit` to maximise parallelism;
/// everything in between picks `Balanced`. Threshold values live as
/// substrate-default constants near this fn; consumer-tunable
/// policy lands when `RunCfg` ships its phase-policy axis (Pass 3 /
/// Pass 6 follow-up).
pub fn select_phase_configs<D: PlanDims>(
    phases: &PhaseBoundaries<D>,
    record_count: USize,
    unit_count: USize,
) -> <D::Phases as Capacity>::Array<PhaseConfig> {
    let mut configs: <D::Phases as Capacity>::Array<PhaseConfig> =
        <D::Phases as Capacity>::filled(PhaseConfig::Balanced);
    let n = phases.phase_count.0;
    let boundaries = phases.boundaries.as_ref();
    let mut i = 0;
    while i < n && i < cap_size(<D::Phases as Capacity>::CAP) {
        // Compute the width of this phase (units it spans).
        let start = boundaries[i].0;
        let end_excl = if i + 1 < n {
            boundaries[i + 1].0
        } else {
            // Last phase spans from its start through the total unit
            // count. Threading `unit_count` in from the runner avoids
            // the prior `start + 1` lower-bound that misclassified a
            // wide last phase as a singleton.
            unit_count.0
        };
        let width = if end_excl > start {
            end_excl - start
        } else {
            1 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: degenerate-width floor for malformed boundaries; tracked: #72
        };
        configs.as_mut()[i] = if record_count.0 < SMALL_RECORD_COUNT_THRESHOLD // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: explicit-singleton case bound; tracked: #72
            || width == 1
        {
            PhaseConfig::MaxFuse
        } else if width > WIDE_PHASE_WIDTH_THRESHOLD {
            PhaseConfig::MaxSplit
        } else {
            PhaseConfig::Balanced
        };
        i += 1;
    }
    configs
}

/// Step 11: per-fiber column classification.
///
/// Walks the fiber assignment and PlanInputs.access masks; classifies
/// each column relative to each fiber as `Internal` (touched only by
/// units in this fiber), `Input` (touched by a unit upstream and read
/// by this fiber), or `Output` (written by this fiber and read by a
/// downstream fiber). The skeleton classifies conservatively as
/// `Internal`; refinement lands in HILA-RUNTIME-C1.
pub fn classify_columns<D: PlanDims>(
    fibers: &FiberGrouping<D>,
    inputs: &PlanInputs<D::Units, D::Stores>,
) -> ColumnClassMap<D>
where
    <D::ColumnsPerFiber as Capacity>::Array<ColumnClassification>: Copy,
{
    let mut map: ColumnClassMap<D> = ColumnClassMap::new();
    let n_fibers = fibers.fiber_count.0;
    let n_units = inputs.unit_count.0;
    let assignment = fibers.assignment.as_ref();
    let access = inputs.access.as_ref();
    // First pass: collect each fiber's touched stores into its
    // column slot list. We treat each touched store as `Internal`
    // initially; the upgrade-to-Input/Output pass would compare
    // across-fiber overlap. The conservative default is sound: it
    // produces correct dispatch shape, just misses some dead-store-
    // elimination opportunities.
    let mut u = 0;
    while u < n_units {
        let f = assignment[u].index().0;
        if f < cap_size(<D::Fibers as Capacity>::CAP) && f < n_fibers {
            // Walk this unit's access mask, register touched stores
            // as columns for fiber f.
            let mut store = 0;
            while store < cap_size(<D::Stores as Capacity>::CAP) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: AccessMask 64-bit window per skeleton; tracked: #72
                && store < 64
            {
                if access[u]
                    .contains(USize(store)) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
                    .0
                {
                    let slot = map.column_count.as_ref()[f].0;
                    if slot < cap_size(<D::ColumnsPerFiber as Capacity>::CAP) {
                        map.class.as_mut()[f].as_mut()[slot] = ColumnClassification::Internal;
                        map.column_count.as_mut()[f] = USize(slot + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
                    }
                }
                store += 1;
            }
        }
        u += 1;
    }
    map
}
