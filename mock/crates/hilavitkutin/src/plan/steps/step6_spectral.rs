//! Step 6: spectral partitioning.
//!
//! Split out of `plan/steps.rs` (file-size lint). No behaviour change.

use arvo::USize;
use arvo_bitmask::NodeId;
use arvo_spectral::k_way_partition;
use arvo_tensor::Capacity;

use super::{DependencyGraph, FiberGrouping, PlanDims, SpectralFloat, SymmetricLaplacian};

/// Step 6: spectral partitioning.
///
/// Builds the symmetric graph Laplacian over the bidirectional CSR
/// (`SymmetricLaplacian`) and runs arvo-spectral's `k_way_partition`
/// to assign each unit a fiber by spectral cut, with the fiber
/// capacity as `K`. Returns a `FiberGrouping` mapping each unit to its
/// spectral partition id.
///
/// The spectral-versus-greedy `group_fibers` (step 7) choice and the
/// projection onto trunk components land in later C1d slices; the
/// runner does not consume this output yet. `k_way_partition` operates
/// over the full unit capacity; on a loose CSR the slack rows are
/// isolated and a live-node-count-aware spectral path is a follow-up
/// gated on the bench adopting spectral.
pub fn spectral_partition<D: PlanDims>(graph: &DependencyGraph<D>) -> FiberGrouping<D>
where
    <D::Units as Capacity>::Array<SpectralFloat>: Copy,
    <D::Units as Capacity>::Array<USize>: Copy,
    <D::Edges as Capacity>::Array<NodeId>: Copy,
{
    use hilavitkutin_api::FiberId;
    let csr = graph.to_csr_bidirectional();
    let lap: SymmetricLaplacian<D::Units, D::Edges, SpectralFloat> = SymmetricLaplacian::new(&csr);
    let sigma = lap.lambda_max_bound();
    let iterations = USize(100); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: spectral power-iteration count; tracked: #72
    let (count, partition) =
        k_way_partition::<_, D::Units, D::Fibers, SpectralFloat>(&lap, sigma, iterations);
    let mut grouping: FiberGrouping<D> = FiberGrouping::new();
    grouping.fiber_count = count;
    // Map each unit's spectral partition id to its fiber.
    for (slot, part) in grouping
        .assignment
        .as_mut()
        .iter_mut()
        .zip(partition.as_ref().iter())
    {
        *slot = FiberId::from_index(*part);
    }
    grouping
}
