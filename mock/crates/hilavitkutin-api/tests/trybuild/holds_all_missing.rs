//! `HoldsAll` names the store that is missing, and a wrapper is reported as
//! the store it stands for.

use arvo::Cap;
use hilavitkutin_api::{AccessSet, Column, HoldsAll, Reach, read};

#[derive(Clone, Copy, PartialEq)]
struct Temp(u32);
#[derive(Clone, Copy, PartialEq)]
struct Wind(u32);

const ONE: Cap = arvo_tensor::cap(1);

fn holds_all<L, Stores: AccessSet + HoldsAll<L>>() {}

fn main() {
    // Registered: the temperature column alone.
    holds_all::<read![Column<Temp>, Reach<Column<Wind>, ONE>], read![Column<Temp>]>();
}
