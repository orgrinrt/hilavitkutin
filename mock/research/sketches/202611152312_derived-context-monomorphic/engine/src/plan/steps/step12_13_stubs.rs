//! Step 12 (core assignment) and step 13 (per-core program synthesis)
//! chain-consistency stubs; the real bodies live elsewhere.
//!
//! Split out of `plan/steps.rs` (file-size lint). No behaviour change.

/// Step 12: map plan trunks onto concrete cores. The body lives in
/// `crate::thread::assign_cores`; this is a re-export for chain
/// consistency. Actual signature parameterised on the `D: PlanDims`
/// ExecutionPlan there.
///
/// The chain treats `assign_cores` as a step but its implementation
/// lives elsewhere; this stub names the step explicitly so the chain
/// reads end-to-end in this file.
pub fn assign_cores_stub() {
    // Real impl: see `crate::thread::assign_cores`. Body lands in
    // HILA-RUNTIME-C4.
}

/// Step 13: per-core program synthesis. Real body needs the per-core
/// projection types from `plan/core_program.rs` (NEW file landing
/// alongside Pass 3 codegen). Stubbed for now.
pub fn synthesise_core_programs_stub() {
    // Real impl lands in HILA-RUNTIME-C2 + plan/core_program.rs.
}
