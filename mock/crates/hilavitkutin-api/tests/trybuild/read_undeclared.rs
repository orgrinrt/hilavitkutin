//! A value function reads the members its unit declared and no other.

use hilavitkutin_api::{Column, ReadVals, read};

#[derive(Clone, Copy, PartialEq)]
struct Temp(u32);
#[derive(Clone, Copy, PartialEq)]
struct Damp(u32);
#[derive(Clone, Copy, PartialEq)]
struct Wind(u32);

fn body(r: &ReadVals<'_, read![Column<Temp>, Column<Damp>]>) -> Wind {
    r.get(Column::<Wind>::new())
}

fn main() {
    let _ = body;
}
