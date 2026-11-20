//! The law the fold runs under: arvo's signed saturating addition.
//!
//! `IFixed<7, 0, Precise>` holds eight logical bits and clamps at -128 and
//! 127 on every add. It is not associative: `(100 + 100) + -100` is 27 and
//! `100 + (100 + -100)` is 100, which `tests/law.rs` holds. So a fold over
//! records gives a result that depends on how the records were grouped, and
//! that is what makes the fold split a question.

use arvo::ifixed::IFixed;
use arvo::strategy::Precise;
use arvo::{FBits, ibits};

/// One signed value under the saturating law.
pub type Sat = IFixed<{ ibits(7) }, { FBits::ZERO }, Precise>;

/// The additive identity a fold starts from.
pub fn zero() -> Sat {
    Sat::from_raw(0)
}

/// The left fold of `values` from zero, one add at a time, in sequence.
pub fn left_fold(values: &[Sat]) -> Sat {
    values.iter().fold(zero(), |acc, &v| acc + v)
}

/// The bytes `values` occupies in memory, for comparing two results.
pub fn bytes_of(values: &[Sat]) -> &[u8] {
    // SAFETY: `IFixed` is `repr(transparent)` over an integer container, so a
    // slice of it has no padding and every byte is initialised.
    unsafe {
        core::slice::from_raw_parts(values.as_ptr().cast::<u8>(), core::mem::size_of_val(values))
    }
}
