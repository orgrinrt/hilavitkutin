//! The side that names the old arvo, `arvo::USize` at `1b2796e9`, as the linking
//! crate wrote it.

#![no_std]

use arvo_old::USize;

/// An OS code carried as the old `USize`, entered by its tuple constructor.
pub const fn old_code(code: usize) -> USize {
    USize(code)
}

/// The code read back through the field.
pub const fn old_code_value(code: USize) -> usize {
    code.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_old_usize_carries_a_code_through_its_field() {
        assert_eq!(old_code_value(old_code(2)), 2);
        assert_eq!(old_code_value(old_code(1314)), 1314);
    }
}
