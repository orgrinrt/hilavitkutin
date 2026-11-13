//! The per-record kernels. Each is a type, so an arm monomorphises on it and
//! the timed loop carries no dispatch.

/// A per-record transform of one `u32` value.
pub trait Kernel: Copy + Send + Sync + 'static {
    fn apply(x: u32) -> u32;
}

/// The identity: a fold over it is a plain column sum, bound by memory
/// bandwidth once the column leaves cache.
#[derive(Copy, Clone, Debug, Default)]
pub struct Light;

impl Kernel for Light {
    #[inline(always)]
    fn apply(x: u32) -> u32 {
        x
    }
}

/// The four-stage multiplicative mix iterated `HEAVY_ROUNDS` times, shared
/// with `engine_vs_std`, so per-record compute dominates memory traffic.
#[derive(Copy, Clone, Debug, Default)]
pub struct Heavy;

impl Kernel for Heavy {
    #[inline(always)]
    fn apply(x: u32) -> u32 {
        engine_vs_std::heavy(x)
    }
}

/// One pass of the four-stage mix, shared with `engine_vs_std`: a cheap
/// transform whose output differs per record, for the append workload.
#[derive(Copy, Clone, Debug, Default)]
pub struct Mix;

impl Kernel for Mix {
    #[inline(always)]
    fn apply(x: u32) -> u32 {
        engine_vs_std::chain(x)
    }
}

/// Fold one record into a partial: the commutative step every fold arm
/// shares, so the arms differ only in how they walk the records.
#[inline(always)]
pub fn fold_step<K: Kernel>(acc: u64, x: u32) -> u64 {
    acc.wrapping_add(K::apply(x) as u64)
}

/// Fold a slice forward.
#[inline(always)]
pub fn fold_forward<K: Kernel>(acc: u64, xs: &[u32]) -> u64 {
    xs.iter().fold(acc, |a, &x| fold_step::<K>(a, x))
}

/// Fold a slice from its last record back to its first, the tail walker's
/// direction.
#[inline(always)]
pub fn fold_backward<K: Kernel>(acc: u64, xs: &[u32]) -> u64 {
    xs.iter().rev().fold(acc, |a, &x| fold_step::<K>(a, x))
}
