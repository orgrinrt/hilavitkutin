//! What the sketch's tests share and what names no engine: the value a record
//! holds, the law it folds by, the inputs, the hand model, and an executor that
//! reports a chosen worker count.
//!
//! The law is arvo's signed saturating addition, `IFixed<7, 0, Precise>`: eight
//! logical bits, clamped at -128 and 127 on every add. It is not associative,
//! and `tests/law.rs` holds the witness.

use arvo::ifixed::IFixed;
use arvo::strategy::Precise;
use arvo::{FBits, USize, ibits};
use hilavitkutin_api::platform::ThreadPoolApi;

/// One signed value under the saturating law.
pub type Sat = IFixed<{ ibits(7) }, { FBits::ZERO }, Precise>;

/// The value a record of the input column holds.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Val(pub Sat);

/// One partial fold, the value the accumulator holds.
#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Part(pub Sat);

impl core::fmt::Debug for Part {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0.to_raw())
    }
}

/// The additive identity the folds start from.
pub fn zero() -> Sat {
    Sat::from_raw(0)
}

/// What the accumulator's buffer holds before a frame writes it.
pub fn poison() -> Sat {
    Sat::from_raw(-77)
}

/// `n` input values spread over the whole range, so most folds saturate.
///
/// A fixed linear congruential sequence: the same `n` gives the same values on
/// every run.
pub fn inputs(n: usize) -> Vec<Val> {
    let mut x: u32 = 0x2545_F491;
    (0 .. n)
        .map(|_| {
            x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            Val(Sat::from_raw(((x >> 24) as i16) - 128))
        })
        .collect()
}

/// The left fold of `values` from zero, one add at a time, in sequence.
pub fn left_fold(values: impl Iterator<Item = Sat>) -> Sat {
    values.fold(zero(), |acc, v| acc + v)
}

/// The partials a grouping by the data gives: the records cut into groups of
/// `g` from record zero, each folded on its own, in record sequence.
pub fn model_grouped(inputs: &[Val], g: usize) -> Vec<Part> {
    inputs
        .chunks(g)
        .map(|c| Part(left_fold(c.iter().map(|v| v.0))))
        .collect()
}

/// The frame's one result: the partials folded in the sequence they were kept.
pub fn combine(parts: &[Part]) -> Sat {
    left_fold(parts.iter().map(|p| p.0))
}

/// The bytes `parts` occupies in memory.
pub fn bytes_of(parts: &[Part]) -> Vec<u8> {
    // SAFETY: `Part` is `repr(transparent)` over `IFixed`, itself
    // `repr(transparent)` over an integer container, so it has no padding and
    // every byte of the slice is initialised.
    unsafe {
        core::slice::from_raw_parts(parts.as_ptr().cast::<u8>(), core::mem::size_of_val(parts))
    }
    .to_vec()
}

/// The bytes of one value.
pub fn bytes_of_one(v: Sat) -> Vec<u8> {
    bytes_of(&[Part(v)])
}

/// Whether every entry carries the same bytes, or the first two worker counts
/// whose bytes differ.
///
/// The check both kinds of test use: the control asserts it refuses, the
/// grouping by the data asserts it accepts.
pub fn all_equal(results: &[(usize, Vec<u8>)]) -> Result<(), (usize, usize)> {
    let Some((first_count, first)) = results.first() else {
        return Ok(());
    };
    for (count, bytes) in &results[1 ..] {
        if bytes != first {
            return Err((*first_count, *count));
        }
    }
    Ok(())
}

/// An executor that reports `W` workers whatever the machine has, so a test
/// runs the same frame at a chosen worker count.
#[derive(Copy, Clone, Debug, Default)]
pub struct Workers<const W: usize>;

impl<const W: usize> ThreadPoolApi for Workers<W> {
    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Detached, as the scheduler expects: its own exit barrier orders the
        // shutdown.
        let _ = std::thread::spawn(f);
    }

    fn worker_count(&self) -> USize {
        USize(W)
    }
}
