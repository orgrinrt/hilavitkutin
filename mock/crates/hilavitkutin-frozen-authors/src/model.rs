//! The model of author 1: a plain function over arrays, written without the contracts, which every
//! run of `Diffuse` is held equal to.
//!
//! `heat[north][east]` is a signed integer of `bits` bits; a neighbour off the grid is zero; the
//! six-term sum is taken in 128 bits and clamped back into `bits` bits.

/// One step of `Diffuse` over a grid of `NORTH` lines of `EAST` places.
pub fn diffuse<const EAST: usize, const NORTH: usize>(
    bits: u32,
    heat: &[[i64; EAST]; NORTH],
    rate: i64,
) -> [[i64; EAST]; NORTH] {
    let low: i128 = -(1i128 << (bits - 1));
    let high: i128 = (1i128 << (bits - 1)) - 1;
    let at = |north: isize, east: isize| -> i128 {
        if north < 0 || east < 0 || north as usize >= NORTH || east as usize >= EAST {
            0
        } else {
            heat[north as usize][east as usize] as i128
        }
    };
    let mut out = [[0i64; EAST]; NORTH];
    for north in 0 .. NORTH {
        for east in 0 .. EAST {
            let (n, e) = (north as isize, east as isize);
            let sum =
                at(n, e) + at(n, e + 1) - at(n, e - 1) + at(n + 1, e) - at(n - 1, e) + rate as i128;
            out[north][east] = sum.clamp(low, high) as i64;
        }
    }
    out
}
