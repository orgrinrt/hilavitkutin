//! `Differs` compares the value written with the value held, so a value type
//! with no comparison is refused as a member and the list is no access set.

use hilavitkutin_api::{AccessSet, Column, Differs, write};

#[derive(Clone, Copy)]
struct Raw(u32);

fn is_access_set<L: AccessSet>() {}

fn main() {
    is_access_set::<write![Differs<Column<Raw>>]>();
}
