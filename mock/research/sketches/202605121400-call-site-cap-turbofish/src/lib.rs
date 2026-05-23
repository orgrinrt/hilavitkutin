//! Sketch: call-site Cap turbofish from a usize-typed engine wrapper.
//!
//! Iteration 2: the first iteration tried `{ Cap(USize(MAX_UNITS)) }`
//! as the inline generic const-arg. rustc rejected it with "overly
//! complex generic constant: struct/enum construction is not supported
//! in generic constants" and pointed at the const-fn-wrapping fix.
//! This iteration uses a local `const fn cap_of(n: usize) -> Cap`.
//!
//! The prior project memory (attempt 2 of round 202605101036) recorded
//! that this shape ICE'd inside arvo_sparse with associated-type
//! propagation. A minimal sketch may or may not hit the same ICE; this
//! sketch finds out.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

use arvo::{Cap, Identity, USize};
use arvo_bitmask::{BitMatrix, NodeId, cap_size};
use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence};
use arvo_numeric_contracts::{FromConstant, TotalOrd};

/// Bridge: usize -> Cap. Lives at the wrapper call site only. Const-fn
/// so it can appear in const-generic position under generic_const_exprs.
#[inline]
pub const fn cap_of(n: usize) -> Cap {
    Cap(USize(n))
}

/// Hypothesis A1: turbofish Cap construction from usize via const fn.
/// arvo-shape input, arvo-shape output.
#[inline]
pub fn rcm_arvo_shape<W, const MAX_UNITS: usize>(
    adjacency: &BitMatrix<W, { cap_of(MAX_UNITS) }>,
) -> [NodeId; cap_size(cap_of(MAX_UNITS))]
where
    W: BitSequence + BitAccess + BitLogic + Identity + Copy + Default,
    [(); cap_size(cap_of(MAX_UNITS))]:,
{
    arvo_sparse::rcm_reorder::<W, { cap_of(MAX_UNITS) }>(adjacency)
}

/// Hypothesis A2: same for block_diagonal. Tuple return shape.
#[inline]
pub fn block_diagonal_arvo_shape<W, const MAX_UNITS: usize>(
    adjacency: &BitMatrix<W, { cap_of(MAX_UNITS) }>,
) -> (USize, [USize; cap_size(cap_of(MAX_UNITS))])
where
    W: BitSequence + BitAccess + BitLogic + Identity + Copy + Default,
    [(); cap_size(cap_of(MAX_UNITS))]:,
{
    arvo_sparse::block_diagonal::<W, { cap_of(MAX_UNITS) }>(adjacency)
}

/// Hypothesis A3: same for spectral_bisection. Different input shape.
#[inline]
pub fn spectral_bisection_arvo_shape<F, const MAX_UNITS: usize>(
    fiedler: &[F; cap_size(cap_of(MAX_UNITS))],
) -> (USize, [USize; cap_size(cap_of(MAX_UNITS))])
where
    F: TotalOrd + Copy + FromConstant,
    [(); cap_size(cap_of(MAX_UNITS))]:,
{
    arvo_spectral::spectral_bisection::<{ cap_of(MAX_UNITS) }, F>(fiedler)
}

/// Concrete monomorphisation test: a `pub` fn with no generic
/// parameters that constructs a real `BitMatrix` at MAX_UNITS = 64
/// and invokes `rcm_arvo_shape::<arvo_bits::Bits<64, arvo::Hot>, 64>`.
///
/// If this compiles, rustc has actually traversed the wrapper's body
/// and resolved the const-arg chain end to end. That is the real
/// proof, not just signature acceptance.
pub fn monomorphise_at_64() -> NodeId {
    use arvo::Hot;
    use arvo_bits::Bits;
    let matrix: BitMatrix<Bits<64, Hot>, { cap_of(64) }> = BitMatrix::empty();
    let order = rcm_arvo_shape::<Bits<64, Hot>, 64>(&matrix);
    order[0]
}
