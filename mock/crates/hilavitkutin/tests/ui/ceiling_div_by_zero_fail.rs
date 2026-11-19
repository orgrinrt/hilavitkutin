//! A zero divisor fails the build where the ceiling is read, rather than
//! giving a value.

use hilavitkutin::plan::{Ceil, CeilingDiv};

const BAD: usize = <Ceil<8, 0> as CeilingDiv<8, 0>>::RESULT;

fn main() {
    let _ = BAD;
}
