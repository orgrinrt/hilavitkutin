//! The kernel of the sketch: one integer pass over a growable line of cells,
//! stepped many times, with a digest after every step.
//!
//! The pass is a three-tap weighted sum with a widened product, rounded half
//! to even and saturated. The digest folds the cells by ascending index with a
//! 64x64 to 128 bit multiply. Nothing here reads the target: no `usize` in a
//! value, no float, no host call. The same source gives the same digests on
//! any target, which is what the front ends check.
//!
//! The cells live in a fixed-capacity array. A full array refuses with a
//! value and never truncates.

#![no_std]
#![forbid(unsafe_code)]

use core::fmt;

/// The tap weights in Q16, applied to the left, middle and right cell.
/// They sum to 0.875, so a smooth line decays. The gain on a line that
/// alternates in sign is 1.375, so that one grows until it saturates.
pub const WEIGHTS: [i64; 3] = [0x9000, -0x4000, 0x9000];

/// Fractional bits of the weights.
pub const FRAC_BITS: u32 = 16;

/// Steps of the pass in one case.
pub const STEPS: usize = 1000;

/// The seed of every case.
pub const SEED: u64 = 0x5EED_0000_0000_0001;

/// The cases a front end runs, in this order.
pub const CASES: u32 = 2;

/// A line of cells in a fixed-capacity array.
#[derive(Clone, Copy, Debug)]
pub struct Cells<const CAP: usize> {
    data: [i64; CAP],
    len:  usize,
}

/// The array is full and cannot take a cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Full {
    pub needed: usize,
    pub cap:    usize,
}

impl<const CAP: usize> Cells<CAP> {
    pub const fn new() -> Self {
        Self {
            data: [0; CAP],
            len:  0,
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn as_slice(&self) -> &[i64] {
        self.data.split_at(self.len).0
    }

    /// Appends one cell, or refuses with the size it would have needed.
    pub const fn push(&mut self, value: i64) -> Result<(), Full> {
        if self.len == CAP {
            return Err(Full {
                needed: self.len + 1,
                cap:    CAP,
            });
        }
        self.data[self.len] = value;
        self.len += 1;
        Ok(())
    }

    /// A line of `len` cells drawn from `seed`, or a refusal when `len`
    /// does not fit.
    pub const fn from_seed(seed: u64, len: usize) -> Result<Self, Full> {
        let mut cells = Self::new();
        let mut x = seed;
        let mut i = 0;
        while i < len {
            x = lcg(x);
            if let Err(full) = cells.push(x as i64) {
                return Err(full);
            }
            i += 1;
        }
        Ok(cells)
    }

    /// Adds `delta` to the cell at `at` modulo the length, wrapping. Keeps
    /// the pass from settling into its saturated cycle.
    pub const fn nudge(&mut self, at: usize, delta: i64) {
        if self.len != 0 {
            let i = at % self.len;
            self.data[i] = self.data[i].wrapping_add(delta);
        }
    }

    /// One step of the pass. The two end cells repeat themselves as their
    /// missing neighbour.
    pub const fn step(&mut self) {
        let mut next = [0i64; CAP];
        let mut i = 0;
        while i < self.len {
            let left = self.data[if i == 0 { 0 } else { i - 1 }];
            let right = self.data[if i + 1 == self.len { i } else { i + 1 }];
            next[i] = tap(left, self.data[i], right);
            i += 1;
        }
        self.data = next;
    }

    /// The digest of the line as it stands.
    pub const fn digest(&self) -> u64 {
        digest(self.as_slice())
    }
}

impl<const CAP: usize> Default for Cells<CAP> {
    fn default() -> Self {
        Self::new()
    }
}

/// The generator of the seeded cells.
pub const fn lcg(x: u64) -> u64 {
    x.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

/// The three-tap weighted sum: a widened product, rounded half to even,
/// saturated to the `i64` range.
pub const fn tap(left: i64, middle: i64, right: i64) -> i64 {
    let s: i128 = (WEIGHTS[0] as i128) * (left as i128)
        + (WEIGHTS[1] as i128) * (middle as i128)
        + (WEIGHTS[2] as i128) * (right as i128);
    let q = s >> FRAC_BITS;
    let r = s & ((1i128 << FRAC_BITS) - 1);
    let half = 1i128 << (FRAC_BITS - 1);
    #[cfg(not(feature = "mutant-half-up"))]
    let up = r > half || (r == half && (q & 1) == 1);
    #[cfg(feature = "mutant-half-up")]
    let up = r >= half;
    let q = if up { q + 1 } else { q };
    if q > i64::MAX as i128 {
        i64::MAX
    } else if q < i64::MIN as i128 {
        i64::MIN
    } else {
        q as i64
    }
}

/// One fold of the digest: xor, a 64x64 to 128 bit multiply, then the two
/// halves xored together.
pub const fn fold(acc: u64, x: u64) -> u64 {
    let m = ((acc ^ x) as u128) * 0x9E37_79B9_7F4A_7C15u128;
    (m as u64) ^ ((m >> 64) as u64)
}

/// The digest of a line: the length first, then every cell by ascending index.
pub const fn digest(cells: &[i64]) -> u64 {
    let mut acc = 0xCBF2_9CE4_8422_2325u64;
    acc = fold_len(acc, cells.len());
    let mut i = 0;
    while i < cells.len() {
        acc = fold(acc, cells[i] as u64);
        i += 1;
    }
    acc
}

/// The length enters the digest as a 64-bit integer, whatever the width of
/// a pointer on the target.
#[cfg(not(feature = "control-usize-len"))]
const fn fold_len(acc: u64, len: usize) -> u64 {
    fold(acc, len as u64)
}

/// The negative control: the length enters as a pointer-width integer, one
/// byte at a time in native byte order, so the digest depends on the target.
#[cfg(feature = "control-usize-len")]
const fn fold_len(acc: u64, len: usize) -> u64 {
    let bytes = len.to_ne_bytes();
    let mut acc = acc;
    let mut i = 0;
    while i < bytes.len() {
        acc = fold(acc, bytes[i] as u64);
        i += 1;
    }
    acc
}

/// Why a run stopped short.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// The line grew past its capacity at this step.
    Cells {
        step:   usize,
        needed: usize,
        cap:    usize,
    },
    /// The digest buffer is shorter than the steps asked for.
    Out {
        needed: usize,
        have:   usize,
    },
}

/// What a run did: the digests written, and why it stopped short if it did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Report {
    pub written: usize,
    pub refusal: Option<Refusal>,
}

impl Report {
    /// The report as five 64-bit words: written, kind (0 none, 1 cells,
    /// 2 out), then three values of the refusal.
    pub const fn words(&self) -> [u64; 5] {
        let w = self.written as u64;
        match self.refusal {
            None => [w, 0, 0, 0, 0],
            Some(Refusal::Cells {
                step,
                needed,
                cap,
            }) => [w, 1, step as u64, needed as u64, cap as u64],
            Some(Refusal::Out {
                needed,
                have,
            }) => [w, 2, needed as u64, have as u64, 0],
        }
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "written={} ", self.written)?;
        match self.refusal {
            None => write!(f, "refusal=none"),
            Some(Refusal::Cells {
                step,
                needed,
                cap,
            }) => {
                write!(f, "refusal=cells step={step} needed={needed} cap={cap}")
            },
            Some(Refusal::Out {
                needed,
                have,
            }) => {
                write!(f, "refusal=out needed={needed} have={have}")
            },
        }
    }
}

/// Runs `steps` steps over a seeded line of `start` cells in an array of
/// `CAP`. Every `grow_every` steps one cell is appended first, and every
/// step starts with a small nudge to one cell. A full array
/// refuses and the run stops; the digests written so far stay in `out`.
pub fn run<const CAP: usize>(
    seed: u64,
    start: usize,
    steps: usize,
    grow_every: usize,
    out: &mut [u64],
) -> Report {
    if out.len() < steps {
        return Report {
            written: 0,
            refusal: Some(Refusal::Out {
                needed: steps,
                have:   out.len(),
            }),
        };
    }
    let mut cells = match Cells::<CAP>::from_seed(seed, start) {
        Ok(cells) => cells,
        Err(Full {
            needed,
            cap,
        }) => {
            return Report {
                written: 0,
                refusal: Some(Refusal::Cells {
                    step: 0,
                    needed,
                    cap,
                }),
            };
        },
    };
    let mut s = 1;
    while s <= steps {
        if grow_every != 0
            && s % grow_every == 0
            && let Err(Full {
                needed,
                cap,
            }) = cells.push(lcg(seed ^ s as u64) as i64)
        {
            return Report {
                written: s - 1,
                refusal: Some(Refusal::Cells {
                    step: s,
                    needed,
                    cap,
                }),
            };
        }
        cells.nudge(s, (lcg(seed.rotate_left(17) ^ s as u64) >> 40) as i64);
        cells.step();
        out[s - 1] = cells.digest();
        s += 1;
    }
    Report {
        written: steps,
        refusal: None,
    }
}

/// Case 0 fits: 54 cells growing by one every 100 steps reach 64 at the
/// last growth, in an array of 64. Case 1 is the same run in an array of 60,
/// which fills at step 700.
pub fn run_case(case: u32, out: &mut [u64]) -> Report {
    match case {
        0 => run::<64>(SEED, 54, STEPS, 100, out),
        _ => run::<60>(SEED, 54, STEPS, 100, out),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ties_round_to_even_in_both_signs() {
        assert_eq!(tap(0, -2, 0), 0, "0.5 goes to 0");
        assert_eq!(tap(0, -6, 0), 2, "1.5 goes to 2");
        assert_eq!(tap(0, 2, 0), 0, "-0.5 goes to 0");
        assert_eq!(tap(0, 6, 0), -2, "-1.5 goes to -2");
        assert_eq!(tap(0, -3, 0), 1, "0.75 is no tie and goes up");
    }

    #[test]
    fn the_product_saturates_instead_of_wrapping() {
        assert_eq!(tap(i64::MAX, 0, i64::MAX), i64::MAX);
        assert_eq!(tap(i64::MIN, 0, i64::MIN), i64::MIN);
        assert_eq!(
            tap(i64::MAX, i64::MIN, i64::MAX),
            i64::MAX,
            "the negative middle weight adds here"
        );
        assert_eq!(tap(i64::MIN, i64::MAX, i64::MIN), i64::MIN);
        assert_eq!(
            tap(i64::MIN, i64::MIN, i64::MIN),
            -8070450532247928832,
            "0.875 of the minimum, no saturation"
        );
    }

    #[test]
    fn the_digest_sees_order_and_length() {
        assert_ne!(digest(&[1, 2]), digest(&[2, 1]));
        assert_ne!(digest(&[0]), digest(&[0, 0]));
    }

    #[test]
    fn a_full_array_refuses_with_a_value_and_keeps_its_cells() {
        let mut cells = Cells::<2>::new();
        assert_eq!(cells.push(7), Ok(()));
        assert_eq!(cells.push(8), Ok(()));
        assert_eq!(
            cells.push(9),
            Err(Full {
                needed: 3,
                cap:    2,
            })
        );
        assert_eq!(cells.as_slice(), &[7, 8]);
        assert_eq!(
            Cells::<2>::from_seed(SEED, 3).err(),
            Some(Full {
                needed: 3,
                cap:    2,
            })
        );
    }

    #[test]
    fn the_small_case_stops_where_the_array_fills() {
        let mut out = [0u64; STEPS];
        let report = run_case(1, &mut out);
        assert_eq!(report.written, 699);
        assert_eq!(
            report.refusal,
            Some(Refusal::Cells {
                step:   700,
                needed: 61,
                cap:    60,
            })
        );
        let mut full = [0u64; STEPS];
        let report = run_case(0, &mut full);
        assert_eq!(report, Report {
            written: STEPS,
            refusal: None,
        });
        assert_eq!(
            out[.. 699],
            full[.. 699],
            "the two cases share every step before the refusal"
        );
    }

    #[test]
    fn a_short_digest_buffer_is_refused_before_any_step() {
        let mut out = [0u64; 10];
        let report = run::<64>(SEED, 54, STEPS, 100, &mut out);
        assert_eq!(report.written, 0);
        assert_eq!(
            report.refusal,
            Some(Refusal::Out {
                needed: STEPS,
                have:   10,
            })
        );
    }

    #[test]
    fn the_main_case_reaches_saturation() {
        let mut cells = Cells::<64>::from_seed(SEED, 54).unwrap();
        let mut saturated_steps = 0;
        let mut s = 1usize;
        while s <= STEPS {
            if s % 100 == 0 {
                cells.push(lcg(SEED ^ s as u64) as i64).unwrap();
            }
            cells.nudge(s, (lcg(SEED.rotate_left(17) ^ s as u64) >> 40) as i64);
            cells.step();
            if cells
                .as_slice()
                .iter()
                .any(|&c| c == i64::MAX || c == i64::MIN)
            {
                saturated_steps += 1;
            }
            s += 1;
        }
        assert!(saturated_steps > 0, "the pass never saturated");
    }
}
