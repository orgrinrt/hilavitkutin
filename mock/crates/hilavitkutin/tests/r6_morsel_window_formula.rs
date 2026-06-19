//! Catalogue contract: per-fiber morsel WINDOW formula (spec domain 12).
//!
//! `ExecutionPlan::morsel_windows[f]` must be the per-fiber L1 window size
//! `(L1_usable / Σ write_bytes).clamp(MIN_MORSEL, MAX_MORSEL) & !3`, with each
//! fiber covering `[0, record_count)` in `ceil(record_count / window)` morsels
//! (multiple morsels per fiber, spec line 82). Slice 1 only renamed the field;
//! `compute_fiber_morsel_windows` still fills it with the old record-count
//! partition placeholder, so this contract is red until slice 4 lands the
//! formula (and slice 3 wires the dispatch to consume it). Catalogued per the
//! edge-cases-as-tests discipline so the case is never lost; the body asserts the
//! intended formula once the machinery (L1_usable source + per-fiber write-byte
//! sum + a plan-window accessor) exists.

#[test]
#[ignore = "catalogue: per-fiber L1 morsel-window formula (L1_usable / Σ write_bytes).clamp & !3; needs slice-4 formula + plan accessor; tracked #341"]
fn morsel_window_matches_l1_formula() {
    unimplemented!(
        "contract: morsel_windows[f] == (L1_usable / Σ write_bytes for fiber f's write columns) \
         .clamp(MIN_MORSEL, MAX_MORSEL) & !3, and a fiber with record_count > MAX_MORSEL gets \
         multiple morsels (window <= MAX_MORSEL, not the whole record count). Fill when \
         compute_fiber_morsel_windows lands the L1 formula (slice 4) and a plan-window accessor \
         exists. tracked #341"
    );
}
