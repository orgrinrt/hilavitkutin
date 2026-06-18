//! s2b: cap_size(CU::CAP) scratch in const-fn LOCALS, threaded through
//! a 3-layer call chain with [const] recursion bounds.
//!
//! The grouping.rs comment on GATE2_MAX_UNITS says a cap_size(CU::CAP)
//! array-length bound "re-proven through the const-gated walk's
//! type-level recursion overflows the trait solver (a generic-constant
//! well-formedness loop)". This module reproduces the masks_of ->
//! final_phases_of -> phase_of shape with the scratch sized
//! `[_; cap_size(CU::CAP)]` and the `[(); cap_size(CU::CAP)]:` bound
//! repeated on each layer, while the same fns carry the [const]
//! recursion bound. The recursive TRAIT stays slice-taking (the shipped
//! shape); only the fn locals carry the generic constant. Contrast s2c,
//! which puts the generic constant INTO the recursive trait signature.

use core::marker::PhantomData;

use arvo::{Identity, USize};
use arvo_tensor::{cap_size, Capacity, Dim};

pub struct RankUnit<const R: usize>;

pub trait HasRank {
    const RANK: USize;
}

impl<const R: usize> HasRank for RankUnit<R> {
    const RANK: USize = USize(R);
}

pub struct UNil;
pub struct UCons<H, T>(PhantomData<(H, T)>);

/// Slice-taking const fold (the shipped BundleMasks shape).
pub const trait MiniFill {
    fn fill(ranks: &mut [USize], idx: USize) -> USize;
}

impl const MiniFill for UNil {
    fn fill(_ranks: &mut [USize], idx: USize) -> USize {
        idx
    }
}

impl<H: HasRank, T: [const] MiniFill> const MiniFill for UCons<H, T> {
    fn fill(ranks: &mut [USize], idx: USize) -> USize {
        ranks[idx.0] = H::RANK;
        <T as MiniFill>::fill(ranks, USize(idx.0 + 1))
    }
}

/// Layer 1: scratch local sized by the generic constant.
const fn ranks_of_gce<Wus, CU>() -> ([USize; cap_size(<CU as Capacity>::CAP)], USize)
where
    Wus: [const] MiniFill,
    CU: Capacity,
    [(); cap_size(<CU as Capacity>::CAP)]:,
{
    let mut ranks = [USize::ZERO; cap_size(<CU as Capacity>::CAP)];
    let n = <Wus as MiniFill>::fill(&mut ranks, USize::ZERO);
    (ranks, n)
}

/// Layer 2: re-proves both the [const] recursion bound and the
/// generic-constant bound while calling layer 1.
const fn phases_of_gce<Wus, CU>() -> ([USize; cap_size(<CU as Capacity>::CAP)], USize)
where
    Wus: [const] MiniFill,
    CU: Capacity,
    [(); cap_size(<CU as Capacity>::CAP)]:,
{
    let (ranks, n) = ranks_of_gce::<Wus, CU>();
    let mut phase = [USize::ZERO; cap_size(<CU as Capacity>::CAP)];
    let mut i = 0;
    while i < n.0 {
        phase[i] = USize(ranks[i].0 * 2);
        i += 1;
    }
    (phase, n)
}

/// Layer 3: the per-position reader the const-gated dispatch would call.
pub const fn phase_at_gce<Wus, CU>(pos: USize) -> USize
where
    Wus: [const] MiniFill,
    CU: Capacity,
    [(); cap_size(<CU as Capacity>::CAP)]:,
{
    let (phase, _n) = phases_of_gce::<Wus, CU>();
    phase[pos.0]
}

type Units8 = UCons<
    RankUnit<0>,
    UCons<
        RankUnit<0>,
        UCons<
            RankUnit<1>,
            UCons<RankUnit<1>, UCons<RankUnit<2>, UCons<RankUnit<2>, UCons<RankUnit<3>, UNil>>>>,
        >,
    >,
>;

const PHASE_AT_4: USize = phase_at_gce::<Units8, Dim<64>>(USize(4));
const PHASE_AT_6: USize = phase_at_gce::<Units8, Dim<64>>(USize(6));

// Projection-form threading: in the engine the capacity reaches the
// grouping as `D::Units` (a PlanDims associated-type projection), not a
// concrete Dim<N>, so the generic constant the caller must satisfy is
// `cap_size(<D::Units as Capacity>::CAP)` with an unnormalized
// projection inside the anon const. Two generic hops re-prove it.
pub trait MiniDims {
    type Units: Capacity;
}

pub struct Dims64;
impl MiniDims for Dims64 {
    type Units = Dim<64>;
}

pub const fn phase_at_via_dims<Wus, D>(pos: USize) -> USize
where
    Wus: [const] MiniFill,
    D: MiniDims,
    [(); cap_size(<D::Units as Capacity>::CAP)]:,
{
    phase_at_gce::<Wus, D::Units>(pos)
}

pub const fn phase_at_via_dims2<Wus, D>(pos: USize) -> USize
where
    Wus: [const] MiniFill,
    D: MiniDims,
    [(); cap_size(<D::Units as Capacity>::CAP)]:,
{
    phase_at_via_dims::<Wus, D>(pos)
}

const PHASE_VIA_DIMS: USize = phase_at_via_dims2::<Units8, Dims64>(USize(4));

pub fn run() {
    println!(
        "s2b: fn-local cap_size scratch through the 3-layer chain: phase_at(4)={} phase_at(6)={} via-dims phase_at(4)={}",
        PHASE_AT_4.0, PHASE_AT_6.0, PHASE_VIA_DIMS.0
    );
    assert_eq!(PHASE_AT_4.0, 4);
    assert_eq!(PHASE_AT_6.0, 6);
    assert_eq!(PHASE_VIA_DIMS.0, 4);
    println!("s2b: WORKS");
}
