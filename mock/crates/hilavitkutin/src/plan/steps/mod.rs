//! The 13-step plan algorithm chain.
//!
//! Each step is a free function with a stable signature. Steps
//! produce the per-stage intermediate analytical types and feed the
//! next step in the chain. The runner `compute_execution_plan`
//! orchestrates them and returns `Outcome<ExecutionPlan, PlanError>`.
//!
//! Step responsibilities (Topic 3 axis A + Domain 15):
//! 1. `build_dag`: AccessMask overlap to CSR `DependencyGraph`.
//! 2. `topo_sort`: Kahn's algorithm to produce a topological order.
//! 3. `compute_waists`: narrow cut detection to delimit phases.
//! 4. `rcm_reorder`: Reverse Cuthill-McKee bandwidth-reduction reordering.
//! 5. `block_diagonalise`: connected-component block detection to trunk skeletons.
//! 6. `spectral_partition`: spectral clustering via a symmetric Laplacian.
//! 7. `group_fibers`: greedy fiber assignment with bounded slack.
//! 8. `compute_upward_rank_and_dirty` (fused per Topic 3 S5):
//!    reverse-topo critical-path rank + per-fiber dirty propagation.
//! 9. `compute_fiber_morsel_windows`: per-fiber L1 morsel-window formula (domain 12).
//! 10. `select_phase_configs`: pick MaxFuse/Balanced/MaxSplit per phase.
//! 11. `classify_columns`: per-fiber column role (Internal/Input/Output).
//! 12. `assign_cores`: map trunks onto concrete cores by `CoreClass`.
//! 13. `synthesise_core_programs`: per-core projection from plan.
//!
//! Steps 4 to 6 (`rcm_reorder`, `block_diagonalise`, `spectral_partition`)
//! are wired to arvo-sparse / arvo-spectral through the
//! `DependencyGraph::to_csr_bidirectional` adapter. The runner consumes
//! the rcm renumber (step 4) and the block-detection trunk skeletons
//! (step 5); step 6's spectral grouping awaits the C1d bench and the
//! fiber projection (HILA-RUNTIME-C1 follow-up slices).
//!
//! Steps 13 ships its body in a follow-up commit alongside
//! `plan/core_program.rs` (Pass 3 codegen feeds it).
//!
//! Every step is generic over one `D: PlanDims` that bundles the
//! capacity dimensions it sizes by; the dimensions are types, so no
//! `cap_size` sits in an array-length position.
//!
//! Round (file-size lint): split into a module directory, one file per
//! step (steps sharing a natural seam, like 4+5 and 10+11, share a
//! file). Every function here was already `pub`, so the split needed
//! no visibility widening: `SpectralFloat` is the one type that moved
//! from module-private to `pub(super)`, since `step6_spectral` and
//! `step7_fibers` (siblings of this file, where it is defined) both
//! name it in a signature.

use arvo::{FastFloat, USize};
use arvo_tensor::Capacity;

use super::access::AccessMask;
use super::column::{ColumnClassMap, ColumnClassification};
use super::dims::PlanDims;
use super::dirty::DirtyMasks;
use super::fiber::{Fiber, FiberGrouping};
use super::graph::{DependencyGraph, EdgeKind};
use super::inputs::PlanInputs;
use super::laplacian::SymmetricLaplacian;
use super::phase::{PhaseBoundaries, PhaseConfig};
use super::trunk::{BlockPartition, Trunk};

mod plan_error;
mod step10_11_configs_columns;
mod step12_13_stubs;
mod step1_dag;
mod step2_topo;
mod step3_waists;
mod step4_5_rcm_block;
mod step6_spectral;
mod step7_fibers;
mod step8_rank;
mod step9_morsel;
#[cfg(test)]
mod tests;

pub use plan_error::PlanError;
pub use step1_dag::{build_dag, first_back_edge};
pub use step2_topo::topo_sort;
pub use step3_waists::compute_waists;
pub use step4_5_rcm_block::{block_diagonalise, phase_trunk_counts, rcm_reorder};
pub use step6_spectral::spectral_partition;
pub use step7_fibers::{fiber_grouping_from_trunks, group_fibers, project_fiber_components};
pub use step8_rank::{compute_predecessor_masks, compute_upward_rank_and_dirty};
pub use step9_morsel::compute_fiber_morsel_windows;
pub use step10_11_configs_columns::{classify_columns, select_phase_configs};
pub use step12_13_stubs::{assign_cores_stub, synthesise_core_programs_stub};

/// Eigenvector float for the spectral partition step. `f32` is the IEEE
/// width tag of arvo's `FastFloat`, not a bare numeric value.
///
/// `pub(super)` rather than private: `step6_spectral` and `step7_fibers`,
/// both siblings of this file, name it in a public function signature.
pub(super) type SpectralFloat = FastFloat<f32>; // lint:allow(no-bare-numeric) reason: f32 is the IEEE width tag of arvo FastFloat; tracked: #72

/// Flat CSR projection output: the plan-wide `trunks` and `fibers` pools
/// the projection writes, plus the per-phase trunk counts.
///
/// The runner copies the pools onto the `ExecutionPlan` and uses
/// `phase_trunks` to set each phase's `(trunk_offset, trunk_count)` CSR
/// range. `phase_trunks[p]` is the number of trunks the projection
/// actually emitted for phase `p` (capped by the plan-wide `D::Trunks`
/// budget), so the per-phase ranges always bracket the flat pool exactly.
pub struct FiberLayout<D: PlanDims> {
    pub trunks:       <D::Trunks as Capacity>::Array<Trunk>,
    pub trunk_count:  USize,
    pub fibers:       <D::Fibers as Capacity>::Array<Fiber<D>>,
    pub fiber_count:  USize,
    pub phase_trunks: <D::Phases as Capacity>::Array<USize>,
}
