//! The two visiting orders: forward, and a permutation drawn from a seed.

use hilavitkutin_contracts::num::{Count, Index, USize};
use hilavitkutin_contracts::store::{Morsel, Visit, Visitor};

/// Every position in key order, cut into morsels of the asked size.
#[derive(Clone, Copy, Debug)]
pub struct Forward;

/// Every position in the order `p(i) = (a * i + b) mod extent`, `a` coprime with the extent and
/// both drawn from the seed, cut into morsels of the asked size.
#[derive(Clone, Copy, Debug)]
pub struct Permuted {
    pub seed: USize,
}

fn cut<V: Visitor>(extent: Count, morsel: Morsel, on: &mut V, mut at: impl FnMut(usize) -> usize) {
    let n = extent.host();
    let size = morsel.size().host();
    let mut start = 0;
    while start < n {
        let len = if n - start < size { n - start } else { size };
        on.morsel(Count::from_host(len));
        let mut i = start;
        while i < start + len {
            on.record(Index::from_host(at(i)));
            i += 1;
        }
        start += len;
    }
}

const fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

impl Visit for Forward {
    fn visit<V: Visitor>(&self, extent: Count, morsel: Morsel, on: &mut V) {
        cut(extent, morsel, on, |i| i);
    }
}

impl Visit for Permuted {
    fn visit<V: Visitor>(&self, extent: Count, morsel: Morsel, on: &mut V) {
        let n = extent.host();
        if n == 0 {
            return;
        }
        let seed = self.seed.host();
        let mut a = seed % n;
        if a == 0 {
            a = 1;
        }
        while gcd(a, n) != 1 {
            a = a % n + 1;
        }
        let b = seed.wrapping_mul(7) % n;
        cut(extent, morsel, on, |i| {
            ((a as u128 * i as u128 + b as u128) % n as u128) as usize
        });
    }
}
