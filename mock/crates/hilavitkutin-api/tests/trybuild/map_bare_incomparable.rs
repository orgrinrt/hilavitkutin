//! A value function takes a bare column as under `Differs`, so a bare column
//! of a value type with no comparison is refused in its write list. The same
//! column under `Written` is taken.

use hilavitkutin_api::{Column, WriteSet, Written, write};

#[derive(Clone, Copy)]
struct Raw(u32);

fn a_function_returns<W: WriteSet>() {}

fn main() {
    a_function_returns::<write![Written<Column<Raw>>]>();
    a_function_returns::<write![Column<Raw>]>();
}
