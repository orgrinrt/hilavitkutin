//! Step 9: per-fiber L1 morsel-window formula (domain 12).
//!
//! Split out of `plan/steps.rs` (file-size lint). No behaviour change.

use arvo::USize;
use arvo::strategy::{Additive, Identity};
use arvo_tensor::{Capacity, cap_size};

use super::{AccessMask, FiberGrouping, PlanDims, PlanInputs};

/// Step 9: per-fiber L1 morsel-window formula (domain 12).
///
/// Per fiber, `window = (l1_usable / sum of write sizes)
/// .clamp(min_morsel, max_morsel) & !3`, where the write sum walks the
/// UNION of the fiber's units' write masks against
/// `inputs.store_sizes` (a store written by two units in one fiber
/// counts once). A fiber with no write bytes takes the upper clamp
/// (nothing constrains its L1 write budget); an unbudgeted plan (a
/// direct call whose inputs carry `MorselBudget::ZERO`) falls back to
/// the whole record range in one morsel per fiber. Slots past the
/// fiber count stay zero. The window is NOT a record-count partition:
/// a fiber covers `[0, record_count)` in `ceil(record_count / window)`
/// morsels.
pub fn compute_fiber_morsel_windows<D: PlanDims>(
    inputs: &PlanInputs<D::Units, D::Stores>,
    fibers: &FiberGrouping<D>,
) -> <D::Fibers as Capacity>::Array<USize> {
    let mut windows: <D::Fibers as Capacity>::Array<USize> =
        <D::Fibers as Capacity>::filled(<USize as Identity<Additive>>::IDENTITY);
    let budget = inputs.morsel_budget;
    let sizes = inputs.store_sizes.as_ref();
    let assignment = fibers.assignment.as_ref();
    let writes = inputs.writes.as_ref();
    let fiber_cap = cap_size(<D::Fibers as Capacity>::CAP);
    let store_cap = cap_size(<D::Stores as Capacity>::CAP);
    let mut f = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: fiber index internal; tracked: #72
    while f < fibers.fiber_count.0 && f < fiber_cap {
        // Union the write masks of this fiber's units so a store written
        // by two units in one fiber counts once toward the L1 budget.
        let mut union: AccessMask<D::Stores> = AccessMask::empty();
        let mut u = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: unit index internal; tracked: #72
        while u < inputs.unit_count.0 && u < assignment.len() {
            if assignment[u].index().0 == f {
                union.union_with(&writes[u]);
            }
            u += 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: unit index step; tracked: #72
        }
        // Sum the per-store L1 byte sizes over the union bits.
        let mut sum = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: write-byte accumulator internal; tracked: #72
        let mut st = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: store index internal; tracked: #72
        while st < store_cap && st < sizes.len() {
            if union.contains(USize(st)).0 {
                sum += sizes[st].0;
            }
            st += 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: store index step; tracked: #72
        }
        // Domain 12: window = (L1_usable / sum).clamp(min, max) & !3. A
        // fiber with no write bytes has nothing constraining its L1 write
        // budget and takes the upper clamp. An unbudgeted plan (a direct
        // `compute_execution_plan` call whose inputs carry
        // `MorselBudget::ZERO`; every build path threads real RunCfg
        // consts) falls back to the whole record range in one morsel.
        if budget.max_morsel.0 == 0 {
            windows.as_mut()[f] = inputs.record_count; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: unbudgeted sentinel test; tracked: #72
        } else {
            // A misconfigured consumer const would panic inside std's
            // clamp with a generic message; pin the invariant here so the
            // failure names the actual knobs.
            debug_assert!(
                budget.min_morsel.0 <= budget.max_morsel.0,
                "RunCfg morsel budget misconfigured: MIN_MORSEL must not exceed MAX_MORSEL"
            );
            let raw = if sum == 0 { budget.max_morsel.0 } else { budget.l1_usable.0 / sum }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: formula arithmetic internal; tracked: #72
            let clamped = raw.clamp(budget.min_morsel.0, budget.max_morsel.0);
            windows.as_mut()[f] = USize(clamped & !3); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 4-record cache-line alignment mask per spec; tracked: #72
        }
        f += 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: fiber index step; tracked: #72
    }
    windows
}
