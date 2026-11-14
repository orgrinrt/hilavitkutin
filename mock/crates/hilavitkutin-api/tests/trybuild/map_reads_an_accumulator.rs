//! An accumulator is not read by a value function: a maintenance unit reads
//! what a pass appended.

use hilavitkutin_api::{Accum, Column, ReadSet, read};

#[derive(Clone, Copy, PartialEq)]
struct Temp(u32);

fn a_function_reads<R: ReadSet>() {}

fn main() {
    a_function_reads::<read![Column<Temp>]>();
    a_function_reads::<read![Column<Temp>, Accum<Temp>]>();
}
