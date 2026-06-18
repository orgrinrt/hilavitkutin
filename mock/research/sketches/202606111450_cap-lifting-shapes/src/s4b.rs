//! s4b: the s2c recursion with a bare-usize associated const in the
//! trait-signature array length.
//!
//! Same recursive-trait shape as s2c, but the generic constant is the
//! PATH form `<CU as WCap>::W` (an associated const that is already a
//! bare usize) instead of the CALL form `cap_size(CU::CAP)`. Probes
//! whether the solver treats the two anon-const shapes differently when
//! re-proven through the cons recursion.

use core::marker::PhantomData;

use arvo::{Identity, USize};
use arvo_tensor::Dim;

/// Bare-usize width carrier (what a Caps helper trait would expose).
pub trait WCap {
    const W: usize;
}

impl<const N: usize> WCap for Dim<N> {
    const W: usize = N;
}

pub struct RankUnit<const R: usize>;

pub trait HasRank {
    const RANK: USize;
}

impl<const R: usize> HasRank for RankUnit<R> {
    const RANK: USize = USize(R);
}

pub struct UNil;
pub struct UCons<H, T>(PhantomData<(H, T)>);

/// The fold with the associated-const path in its method signature.
pub const trait MiniFillW<CU: WCap>
where
    [(); <CU as WCap>::W]:,
{
    fn fill(ranks: &mut [USize; <CU as WCap>::W], idx: USize) -> USize;
}

impl<CU: WCap> const MiniFillW<CU> for UNil
where
    [(); <CU as WCap>::W]:,
{
    fn fill(_ranks: &mut [USize; <CU as WCap>::W], idx: USize) -> USize {
        idx
    }
}

impl<H: HasRank, T, CU: WCap> const MiniFillW<CU> for UCons<H, T>
where
    T: [const] MiniFillW<CU>,
    [(); <CU as WCap>::W]:,
{
    fn fill(ranks: &mut [USize; <CU as WCap>::W], idx: USize) -> USize {
        ranks[idx.0] = H::RANK;
        <T as MiniFillW<CU>>::fill(ranks, USize(idx.0 + 1))
    }
}

const fn ranks_w<Wus, CU>() -> ([USize; <CU as WCap>::W], USize)
where
    CU: WCap,
    Wus: [const] MiniFillW<CU>,
    [(); <CU as WCap>::W]:,
{
    let mut ranks = [USize::ZERO; <CU as WCap>::W];
    let n = <Wus as MiniFillW<CU>>::fill(&mut ranks, USize::ZERO);
    (ranks, n)
}

pub const fn rank_at_w<Wus, CU>(pos: USize) -> USize
where
    CU: WCap,
    Wus: [const] MiniFillW<CU>,
    [(); <CU as WCap>::W]:,
{
    let (ranks, _n) = ranks_w::<Wus, CU>();
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

const RANK_AT_3: USize = rank_at_w::<Units8, Dim<64>>(USize(3));

// Depth stress, parallel to s2c.
macro_rules! unit_list {
    () => { UNil };
    ($r:literal $(, $rest:literal)*) => { UCons<RankUnit<$r>, unit_list!($($rest),*)> };
}

type Units64 = unit_list!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63
);

const RANK_DEEP: USize = rank_at_w::<Units64, Dim<128>>(USize(63));

pub fn run() {
    println!(
        "s4b: trait-signature assoc-const arrays: rank_at(3)={} deep rank_at(63)={}",
        RANK_AT_3.0, RANK_DEEP.0
    );
    assert_eq!(RANK_AT_3.0, 3);
    assert_eq!(RANK_DEEP.0, 63);
    println!("s4b: WORKS");
}
