//! Step 13: synthesise per-core programs from the execution plan.
//!
//! `synthesise_core_programs` walks an `ExecutionPlan` and projects
//! it into one `CoreProgram` per physical core. Each `CoreProgram`
//! captures (a) the phases the core participates in with the right
//! sync role, (b) the trunks it owns, (c) per-fiber record ranges,
//! (d) the `progress_slots[]` base offset for the core's fibers,
//! (e) the `phase_arrived` bit offset. Pass 3 dispatch codegen
//! takes one `CoreProgram` per core and emits a monomorphised
//! per-core closure.
//!
//! The current shape ships a conservative initial projection: a
//! round-robin fiber-to-core assignment with `RecordRange::Full` for
//! every fiber. The honest per-fiber range computation (head/tail
//! convergence, micro-morsel boundaries) lands when Topic 4 axis D
//! head+tail dispatch wires through (HILA-RUNTIME-C2 follow-up). The
//! shape is committed; the body refines.
//!
//! The plan dimensions arrive bundled as one `D: PlanDims` (`D::Cores`
//! sizes the per-core program array). The per-core sub-capacities
//! (`PC` / `TC` / `FC`) are NOT plan dimensions: they size the
//! hilavitkutin-api `CoreProgram`'s min-const-generic arrays, so they
//! stay their own `Capacity` type parameters projected into the api's
//! `usize` positions via `cap_size`.

use arvo::strategy::Identity;
use arvo::USize;
use arvo_tensor::{cap_size, Capacity};

use hilavitkutin_api::{CoreProgram, FiberId, PhaseEntry, PhaseId, RecordRange, SyncRole, TrunkId};

use super::dims::PlanDims;
use super::ExecutionPlan;

/// Synthesise per-core `CoreProgram`s from the execution plan.
///
/// `core_count` is the runtime number of cores to populate; slots in
/// the returned array past `core_count.0` are left as
/// `CoreProgram::new()` (all-zero).
///
/// Soundness gate: every assigned `progress_slot_idx` is verified to
/// fit within the fiber capacity via `debug_assert!`. The plan's morsel
/// distribution already constrains fiber count, so the assertion is
/// a defensive belt-and-suspenders check; a hand-crafted plan whose
/// fiber count exceeded its cap would trip it.
#[allow(clippy::too_many_arguments)]
pub fn synthesise_core_programs<D: PlanDims, PC: Capacity, TC: Capacity, FC: Capacity>(
    plan: &ExecutionPlan<D>,
    core_count: USize,
) -> <D::Cores as Capacity>::Array<
    CoreProgram<{ cap_size(PC::CAP) }, { cap_size(TC::CAP) }, { cap_size(FC::CAP) }>,
>
where
    <D::Cores as Capacity>::Array<
        CoreProgram<{ cap_size(PC::CAP) }, { cap_size(TC::CAP) }, { cap_size(FC::CAP) }>,
    >: Sized,
{
    let mut programs: <D::Cores as Capacity>::Array<
        CoreProgram<{ cap_size(PC::CAP) }, { cap_size(TC::CAP) }, { cap_size(FC::CAP) }>,
    > = <D::Cores as Capacity>::filled(CoreProgram::new());

    let cores = core_count.0.min(cap_size(<D::Cores as Capacity>::CAP));
    if cores == 0 {
        return programs;
    }

    // Count total fibers across all phases for the round-robin
    // assignment. The plan stores phases with their trunks; each trunk
    // holds components (Fiber / Branch / Bridge). The conservative
    // skeleton sums fiber components; honest accounting that walks
    // FiberGrouping lands when assign_cores threads through (Pass 3).
    let total_fibers = plan.morsel_sizes.as_ref().iter().filter(|m| m.0 > 0).count(); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: count internal; tracked: #72

    // Round-robin: distribute `total_fibers` across `cores`. Core c
    // gets fibers [start_c .. end_c) where the remainder is spread
    // across the first `(total_fibers % cores)` cores (same shape
    // as size_morsels remainder distribution).
    let per_core = total_fibers / cores;
    let remainder = total_fibers % cores;

    let mut fiber_cursor: usize = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal cursor; tracked: #72
    let mut c = 0;
    while c < cores {
        let extra = if c < remainder { 1 } else { 0 }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: remainder distribution literal; tracked: #72
        let my_fibers = per_core + extra;

        // Per-core: range_count is bounded by both `my_fibers` and
        // the core's fiber capacity.
        let range_count = my_fibers.min(cap_size(FC::CAP));

        // Assign each owned fiber a slot. The progress_slots base for
        // this core is its first fiber's index; subsequent fibers on
        // the core read offsets relative to the base.
        let progress_slot_base = fiber_cursor;
        debug_assert!(
            progress_slot_base + range_count <= cap_size(<D::Fibers as Capacity>::CAP),
            "progress_slot_idx + range exceeds fiber capacity",
        );

        let prog = &mut programs.as_mut()[c];
        let mut r = 0;
        while r < range_count {
            let fid_idx = fiber_cursor + r;
            // Build a FiberId for this slot from the array-index value
            // via the typed accessor.
            let fid = FiberId::from_index(USize(fid_idx));
            // Conservative initial range: Full. Head/Tail convergence
            // lands when Topic 4 axis D dispatch wires through.
            prog.fiber_ranges[r] = (fid, RecordRange::Full);
            r += 1;
        }
        prog.range_count = USize(range_count); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal count; tracked: #72

        // Phases: every core participates in every plan phase. The
        // sync role pattern: SignalOnly for the first phase (producer
        // only), WaitOnly for the last phase (consumer only),
        // WaitAndSignal for everything in between.
        let phase_n = plan.phase_count.0.min(cap_size(PC::CAP));
        let mut p = 0;
        while p < phase_n {
            let phase = PhaseId::from_index(USize(p));
            let sync_role = if phase_n == 1 {
                SyncRole::WaitAndSignal
            } else if p == 0 {
                SyncRole::SignalOnly
            } else if p == phase_n - 1 {
                SyncRole::WaitOnly
            } else {
                SyncRole::WaitAndSignal
            };
            prog.phases[p] = PhaseEntry { phase, sync_role };
            p += 1;
        }
        prog.phase_count = USize(phase_n); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal count; tracked: #72

        // Trunks: skeleton leaves the array zero-initialised. The
        // honest trunk-to-core mapping lands when assign_cores produces
        // its CoreAssignment, which Pass 3 will thread through to
        // populate this field.
        prog.trunk_count = USize::ZERO;
        let _ = TrunkId::ZERO; // keep the type in scope for the trunk-assignment follow-up

        prog.progress_slot_idx = USize(progress_slot_base); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
        prog.phase_arrived_offset = USize(c); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
        // estimated_icache_bytes stays at ZERO; Pass 3 codegen computes
        // the real value from the emitted closure size and uses it for
        // the ScheduleMega -> TrunkMega -> MonoTuple fallback ladder.

        fiber_cursor += range_count;
        c += 1;
    }

    programs
}
