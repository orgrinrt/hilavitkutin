//! Symmetric graph Laplacian over a bidirectional CSR.
//!
//! arvo-spectral's `SparseLaplacian` walks forward edges only, giving
//! the directed Laplacian. Spectral partitioning needs the symmetric
//! (undirected) Laplacian: nodes `i` and `j` couple if `i -> j` or
//! `j -> i`. This engine-local operator wraps the `CsrBidirectional`
//! (forward plus transpose) and implements `LinearOperator` with the
//! symmetric matvec, using unit edge weights (every dependency edge is
//! one unit of coupling for partitioning purposes). It is composition
//! over the `LinearOperator` contract, not a reimplementation of an
//! arvo primitive.
//!
//! The dependency graph is a DAG by the time step 6 runs (topo sort and
//! the cycle check pass first), so `successors` and `predecessors` are
//! disjoint per node and the matvec walks both halves without
//! deduplication. `live_dim` reports the graph's live node count
//! (`node_count()`), so the spectral iteration excludes a loose CSR's
//! empty slack rows; on a full graph the live count equals the cap.

use core::marker::PhantomData;
use core::ops::{Add, Mul, Sub};

use arvo::traits::{FromConstant, TotalOrd};
use arvo::{Cap, USize};
use arvo_bitmask::NodeId;
use arvo_sparse::{BidirectionalSparseAdjacency, CsrBidirectional, SparseAdjacency};
use arvo_spectral::LinearOperator;
use arvo_tensor::cap_size;

use crate::plan::graph::EdgeKind;

/// Symmetric (undirected) graph Laplacian `L = D - A` over the engine
/// dependency graph's bidirectional CSR, with unit edge weights.
///
/// `F` is the spectral eigenvector float (`FastFloat<f32>` in the
/// engine). The operator borrows the adjacency; no copy, no rebuild
/// per iteration.
pub struct SymmetricLaplacian<'data, const N: Cap, const E: Cap, F>
where
    [(); cap_size(N)]:,
    [(); cap_size(E)]:,
{
    adjacency: &'data CsrBidirectional<N, E, EdgeKind>,
    _f: PhantomData<F>,
}

impl<'data, const N: Cap, const E: Cap, F> SymmetricLaplacian<'data, N, E, F>
where
    [(); cap_size(N)]:,
    [(); cap_size(E)]:,
    F: Add<Output = F> + Mul<Output = F> + TotalOrd + Copy + FromConstant,
{
    /// Wrap a bidirectional CSR as a symmetric Laplacian operator.
    #[inline]
    pub fn new(adjacency: &'data CsrBidirectional<N, E, EdgeKind>) -> Self {
        Self { adjacency, _f: PhantomData }
    }

    /// Gershgorin upper bound on `lambda_max(L)`: `max_i 2 * deg(i)`,
    /// where `deg(i)` is the undirected degree (successors plus
    /// predecessors). A valid `sigma` shift for the power / Fiedler
    /// iteration, which needs `sigma >= lambda_max` so `sigma*I - L`
    /// stays positive semidefinite.
    pub fn lambda_max_bound(&self) -> F {
        let n = cap_size(N);
        let zero = F::from_constant::<{ USize(0) }>();
        let one = F::from_constant::<{ USize(1) }>();
        let two = F::from_constant::<{ USize(2) }>();
        let mut sigma = zero;
        let mut i = 0usize;
        while i < n {
            let node = NodeId::new(USize(i));
            let mut deg = zero;
            for _ in self.adjacency.successors(node) {
                deg = deg + one;
            }
            for _ in self.adjacency.predecessors(node) {
                deg = deg + one;
            }
            let candidate = two * deg;
            if let core::cmp::Ordering::Less = sigma.total_cmp(candidate) {
                sigma = candidate;
            }
            i += 1;
        }
        sigma
    }
}

impl<'data, const N: Cap, const E: Cap, F> LinearOperator<F, N>
    for SymmetricLaplacian<'data, N, E, F>
where
    [(); cap_size(N)]:,
    [(); cap_size(E)]:,
    F: Add<Output = F> + Sub<Output = F> + Copy + FromConstant,
{
    #[inline]
    fn apply(&self, x: &[F; cap_size(N)], y: &mut [F; cap_size(N)]) {
        let n = cap_size(N);
        let zero = F::from_constant::<{ USize(0) }>();
        let mut i = 0usize;
        while i < n {
            let node = NodeId::new(USize(i));
            let xi = x[i];
            // (L x)[i] = sum over undirected neighbours of (x[i] - x[j]),
            // unit weights. DAG so successors and predecessors disjoint.
            let mut acc = zero;
            for nb in self.adjacency.successors(node) {
                let j = (nb.0).0;
                if j < n {
                    acc = acc + (xi - x[j]);
                }
            }
            for nb in self.adjacency.predecessors(node) {
                let j = (nb.0).0;
                if j < n {
                    acc = acc + (xi - x[j]);
                }
            }
            y[i] = acc;
            i += 1;
        }
    }

    #[inline]
    fn live_dim(&self) -> USize {
        // The live node count of the bidirectional CSR: loose-CSR slack
        // rows are not real nodes, so spectral iteration excludes them.
        self.adjacency.node_count()
    }
}
