//! The side that names current arvo, `arvo_numeral::USize` at arvo `dev`
//! `6def8831`, entered by `from_host` and read by `host`.

#![no_std]

use arvo_new_format::Adapt;
use arvo_new_format::overflow::Wrap;
use arvo_new_format::rounding::HalfEven;

/// The code's type: the new `USize` under an explicit adaptation, since no preset
/// names one yet.
pub type Code = arvo_new_numeral::USize<Adapt<HalfEven, Wrap>>;

/// An OS code carried as the new `USize`.
pub const fn new_code(code: usize) -> Code {
    Code::from_host(code)
}

/// The code read back through the unwrap door.
pub const fn new_code_value(code: Code) -> usize {
    code.host()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_new_usize_carries_a_code_through_its_doors() {
        assert_eq!(new_code_value(new_code(2)), 2);
        assert_eq!(new_code_value(new_code(1314)), 1314);
    }
}
