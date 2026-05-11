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

use arvo::strategy::Identity;
use arvo::USize;

use hilavitkutin_api::{CoreProgram, FiberId, PhaseEntry, PhaseId, RecordRange, SyncRole, TrunkId};

use super::ExecutionPlan;

/// Synthesise per-core `CoreProgram`s from the execution plan.
///
/// `core_count` is the runtime number of cores to populate; slots in
/// the returned array past `core_count.0` are left as
/// `CoreProgram::new()` (all-zero).
///
/// Soundness gate: every assigned `progress_slot_idx` is verified to
/// fit within `MAX_FIBERS` via `debug_assert!`. The plan's morsel
/// distribution already constrains fiber count, so the assertion is
/// a defensive belt-and-suspenders check; a hand-crafted plan whose
/// fiber count exceeded its cap would trip it.
#[allow(clippy::too_many_arguments)]
pub fn synthesise_core_programs<
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_PHASES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_TRUNKS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_FIBERS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_LANES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_COLUMNS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_COMPONENTS_PER_TRUNK: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_UNITS_PER_FIBER: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_COLUMNS_PER_FIBER: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_TRUNKS_PER_PHASE: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_CORES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_PHASES_PER_CORE: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_TRUNKS_PER_CORE: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_FIBERS_PER_CORE: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
>(
    plan: &ExecutionPlan<
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
    >,
    core_count: USize,
) -> [CoreProgram<MAX_PHASES_PER_CORE, MAX_TRUNKS_PER_CORE, MAX_FIBERS_PER_CORE>; MAX_CORES] {
    let mut programs: [CoreProgram<
        MAX_PHASES_PER_CORE,
        MAX_TRUNKS_PER_CORE,
        MAX_FIBERS_PER_CORE,
    >; MAX_CORES] = [CoreProgram::new(); MAX_CORES];

    let cores = core_count.0.min(MAX_CORES);
    if cores == 0 {
        return programs;
    }

    // Count total fibers across all phases for the round-robin
    // assignment. The plan stores phases with their trunks; each trunk
    // holds components (Fiber / Branch / Bridge). The conservative
    // skeleton sums fiber components; honest accounting that walks
    // FiberGrouping lands when assign_cores threads through (Pass 3).
    let total_fibers = plan.morsel_sizes.iter().filter(|m| m.0 > 0).count(); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: count internal; tracked: #72

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
        // the core's cap MAX_FIBERS_PER_CORE.
        let range_count = my_fibers.min(MAX_FIBERS_PER_CORE);

        // Assign each owned fiber a slot. The progress_slots base for
        // this core is its first fiber's index; subsequent fibers on
        // the core read offsets relative to the base.
        let progress_slot_base = fiber_cursor;
        debug_assert!(
            progress_slot_base + range_count <= MAX_FIBERS,
            "progress_slot_idx + range exceeds MAX_FIBERS cap",
        );

        let mut r = 0;
        while r < range_count {
            let fid_idx = fiber_cursor + r;
            // Build a FiberId for this slot. UnitId/FiberId are
            // repr(transparent) over Uint<N> over Bits<N, Warm, Unsigned>.
            // FiberId is 2 bytes (Warm-7 picks u16 container per the
            // size assertion in dispatch_codegen.rs).
            let fid_u16 = fid_idx as u16; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bridging usize to u16 for repr(transparent) projection; tracked: #428
            let fid: FiberId = unsafe { core::mem::transmute_copy(&fid_u16) }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: repr(transparent) projection through guaranteed-layout FiberId chain; tracked: #428
            // Conservative initial range: Full. Head/Tail convergence
            // lands when Topic 4 axis D dispatch wires through.
            programs[c].fiber_ranges[r] = (fid, RecordRange::Full);
            r += 1;
        }
        programs[c].range_count = USize(range_count); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal count; tracked: #72

        // Phases: every core participates in every plan phase. The
        // sync role pattern: SignalOnly for the first phase (producer
        // only), WaitOnly for the last phase (consumer only),
        // WaitAndSignal for everything in between.
        let phase_n = plan.phase_count.0.min(MAX_PHASES_PER_CORE);
        let mut p = 0;
        while p < phase_n {
            let phase_u16 = p as u16; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bridging usize to u16 for repr(transparent) projection; tracked: #428
            let phase: PhaseId = unsafe { core::mem::transmute_copy(&phase_u16) }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: repr(transparent) projection through guaranteed-layout PhaseId chain; tracked: #428
            let sync_role = if phase_n == 1 {
                SyncRole::WaitAndSignal
            } else if p == 0 {
                SyncRole::SignalOnly
            } else if p == phase_n - 1 {
                SyncRole::WaitOnly
            } else {
                SyncRole::WaitAndSignal
            };
            programs[c].phases[p] = PhaseEntry { phase, sync_role };
            p += 1;
        }
        programs[c].phase_count = USize(phase_n); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal count; tracked: #72

        // Trunks: skeleton leaves the array zero-initialised. The
        // honest trunk-to-core mapping lands when assign_cores produces
        // its CoreAssignment, which Pass 3 will thread through to
        // populate this field.
        programs[c].trunk_count = USize::ZERO;
        let _ = TrunkId::ZERO; // keep the type in scope for the trunk-assignment follow-up

        programs[c].progress_slot_idx = USize(progress_slot_base); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
        programs[c].phase_arrived_offset = USize(c); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
        // estimated_icache_bytes stays at ZERO; Pass 3 codegen computes
        // the real value from the emitted closure size and uses it for
        // the ScheduleMega -> TrunkMega -> MonoTuple fallback ladder.

        fiber_cursor += range_count;
        c += 1;
    }

    programs
}
