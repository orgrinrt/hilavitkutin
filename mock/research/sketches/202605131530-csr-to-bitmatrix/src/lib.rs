//! Sketch S3+S4: CSR-to-BitMatrix conversion + UnitId/NodeId bridge.
//!
//! Models the data conversion the engine-side wrapper must do before
//! calling arvo. The CSR shape mirrors `hilavitkutin::plan::graph::DependencyGraph`
//! (row_offsets / col_indices / unit_count / edge_count). The conversion
//! walks every populated unit, scans its edge row, and sets the
//! corresponding bit in the BitMatrix.
//!
//! UnitId is `Uint<16>` repr(transparent) over `Bits<16, Warm, Unsigned>`.
//! `Warm` at 16 bits picks a `u32` container per the substrate's
//! storage projection table. NodeId is `pub struct NodeId(pub USize)`,
//! usize-wide. The bridge: `UnitId -> u32 (via transmute_copy) -> usize
//! (via as cast) -> USize -> NodeId`.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, Identity, USize};
use arvo_bitmask::{BitMatrix, NodeId, cap_size};
use arvo_bits::Bits;
use arvo::Hot;
use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence};

use hilavitkutin_api::UnitId;

/// usize -> Cap bridge.
#[inline]
pub const fn cap_of(n: usize) -> Cap {
    Cap(USize(n))
}

/// Minimal CSR mirror of `DependencyGraph<MAX_UNITS, MAX_EDGES>`.
/// Same field layout as the real one: row_offsets, col_indices,
/// unit_count, edge_count. The sketch omits edge_kinds because
/// arvo's BitMatrix does not distinguish edge kinds (the wrapper
/// projects every kind to a set bit).
pub struct CsrLike<const MAX_UNITS: usize, const MAX_EDGES: usize> {
    pub row_offsets: [USize; MAX_UNITS],
    pub col_indices: [UnitId; MAX_EDGES],
    pub unit_count: USize,
    pub edge_count: USize,
}

impl<const MAX_UNITS: usize, const MAX_EDGES: usize> CsrLike<MAX_UNITS, MAX_EDGES> {
    pub const fn empty() -> Self {
        // UnitId::ZERO is `pub const ZERO: Self`; the engine's real
        // DependencyGraph initialises col_indices the same way.
        Self {
            row_offsets: [USize::ZERO; MAX_UNITS],
            col_indices: [UnitId::ZERO; MAX_EDGES],
            unit_count: USize::ZERO,
            edge_count: USize::ZERO,
        }
    }

    /// End-of-row index for unit `i` (exclusive). Mirrors
    /// `DependencyGraph::end_for`.
    #[inline]
    fn end_for(&self, i: usize) -> usize {
        let next = i + 1;
        let count = self.unit_count.0;
        if next < count { self.row_offsets[next].0 } else { self.edge_count.0 }
    }
}

/// Project a `UnitId` to its numeric value as a `usize`. Uses the
/// same `transmute_copy` projection the engine already uses
/// (`plan/graph.rs:120` and similar sites). UnitId is repr(transparent)
/// over Uint<16> over Bits<16, Warm, Unsigned>; Warm at 16 bits maps
/// to a u32 container. The transmute_copy reads four bytes; the upper
/// two are guaranteed zero by the construction path.
#[inline]
fn unit_id_to_usize(u: UnitId) -> usize {
    let raw: u32 = unsafe { core::mem::transmute_copy(&u) };
    raw as usize
}

/// Convert a CSR graph to a dense BitMatrix sized at MAX_UNITS rows
/// and columns. Edge i->j becomes bit j set in row i. The conversion
/// is O(unit_count + edge_count); MAX_UNITS-sized stack-array per
/// the BitMatrix shape.
#[inline]
pub fn csr_to_bitmatrix<const MAX_UNITS: usize, const MAX_EDGES: usize>(
    graph: &CsrLike<MAX_UNITS, MAX_EDGES>,
) -> BitMatrix<Bits<64, Hot>, { cap_of(MAX_UNITS) }>
where
    [(); cap_size(cap_of(MAX_UNITS))]:,
{
    let mut matrix: BitMatrix<Bits<64, Hot>, { cap_of(MAX_UNITS) }> = BitMatrix::empty();
    let n = graph.unit_count.0;
    let mut i = 0usize;
    while i < n {
        let start = graph.row_offsets[i].0;
        let end_excl = graph.end_for(i);
        let mut k = start;
        while k < end_excl {
            let from = NodeId::new(USize(i));
            let to_usize = unit_id_to_usize(graph.col_indices[k]);
            let to = NodeId::new(USize(to_usize));
            matrix.set_edge(from, to);
            k += 1;
        }
        i += 1;
    }
    matrix
}

/// End-to-end: CSR -> BitMatrix -> arvo_sparse::rcm_reorder ->
/// arvo-shape return. Compiling this monomorphisation proves the
/// full path works at MAX_UNITS = 64.
pub fn end_to_end_rcm_at_64(
    graph: &CsrLike<64, 128>,
) -> [NodeId; cap_size(cap_of(64))] {
    let matrix = csr_to_bitmatrix::<64, 128>(graph);
    arvo_sparse::rcm_reorder::<Bits<64, Hot>, { cap_of(64) }>(&matrix)
}

/// Compile-time soundness assertion on the transmute_copy projection.
/// If UnitId's size differs from u32, the const eval inside the
/// const block fails the compile. The engine's existing pattern
/// asserts the same invariant implicitly (no explicit assertion
/// shipped); this sketch makes the assertion explicit so any future
/// strategy-table change that re-sizes Warm at 16 bits surfaces here.
const _: () = {
    assert!(core::mem::size_of::<UnitId>() == core::mem::size_of::<u32>());
};
