//! s2c: cap_size(CU::CAP) arrays in the recursive TRAIT signature.
//!
//! The shape the grouping comment names as the overflow: the generic
//! constant rides INSIDE the recursive trait obligation (fixed-array
//! refs in the fold's method signature, the CU param threaded through
//! every cons-cell impl), so the solver must re-prove the
//! generic-constant well-formedness at every recursion step instead of
//! once per fn. Expected per the comment: trait-solver overflow / a
//! generic-constant WF loop. Whatever actually happens on
//! nightly-2026-05-28 is the finding.

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

/// The fold with the generic constant in its METHOD SIGNATURE.
pub const trait MiniFillArr<CU: Capacity>
where
    [(); cap_size(<CU as Capacity>::CAP)]:,
{
    fn fill(ranks: &mut [USize; cap_size(<CU as Capacity>::CAP)], idx: USize) -> USize;
}

impl<CU: Capacity> const MiniFillArr<CU> for UNil
where
    [(); cap_size(<CU as Capacity>::CAP)]:,
{
    fn fill(_ranks: &mut [USize; cap_size(<CU as Capacity>::CAP)], idx: USize) -> USize {
        idx
    }
}

impl<H: HasRank, T, CU: Capacity> const MiniFillArr<CU> for UCons<H, T>
where
    T: [const] MiniFillArr<CU>,
    [(); cap_size(<CU as Capacity>::CAP)]:,
{
    fn fill(ranks: &mut [USize; cap_size(<CU as Capacity>::CAP)], idx: USize) -> USize {
        ranks[idx.0] = H::RANK;
        <T as MiniFillArr<CU>>::fill(ranks, USize(idx.0 + 1))
    }
}

/// The masks_of analog over the array-signature fold.
const fn ranks_arr<Wus, CU>() -> ([USize; cap_size(<CU as Capacity>::CAP)], USize)
where
    CU: Capacity,
    Wus: [const] MiniFillArr<CU>,
    [(); cap_size(<CU as Capacity>::CAP)]:,
{
    let mut ranks = [USize::ZERO; cap_size(<CU as Capacity>::CAP)];
    let n = <Wus as MiniFillArr<CU>>::fill(&mut ranks, USize::ZERO);
    (ranks, n)
}

/// Second layer, re-proving the obligation chain.
pub const fn rank_at_arr<Wus, CU>(pos: USize) -> USize
where
    CU: Capacity,
    Wus: [const] MiniFillArr<CU>,
    [(); cap_size(<CU as Capacity>::CAP)]:,
{
    let (ranks, _n) = ranks_arr::<Wus, CU>();
    ranks[pos.0]
}

type Units8 = UCons<
    RankUnit<0>,
    UCons<
        RankUnit<1>,
        UCons<
            RankUnit<2>,
            UCons<RankUnit<3>, UCons<RankUnit<4>, UCons<RankUnit<5>, UCons<RankUnit<6>, UNil>>>>,
        >,
    >,
>;

const RANK_AT_5: USize = rank_at_arr::<Units8, Dim<64>>(USize(5));

// Depth stress: the engine registers tens of units; if the overflow is
// depth-driven, a 64-deep walk should show it.
macro_rules! unit_list {
    () => { UNil };
    ($r:literal $(, $rest:literal)*) => { UCons<RankUnit<$r>, unit_list!($($rest),*)> };
}

type Units64 = unit_list!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63
);

const RANK_DEEP: USize = rank_at_arr::<Units64, Dim<128>>(USize(63));

pub fn run() {
    println!(
        "s2c: trait-signature cap_size arrays: rank_at(5)={} deep rank_at(63)={}",
        RANK_AT_5.0, RANK_DEEP.0
    );
    assert_eq!(RANK_AT_5.0, 5);
    assert_eq!(RANK_DEEP.0, 63);
    println!("s2c: WORKS");
}
