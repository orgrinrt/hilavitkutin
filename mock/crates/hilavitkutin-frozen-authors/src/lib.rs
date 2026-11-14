//! Work units written the way a consumer writes them, beside a hand model of
//! what a frame of them computes, and frozen: once landed, a file of this crate
//! changes only through a line in `BREAKS.toml`.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod model;
pub mod record;
pub mod units;
pub mod values;

pub use model::Model;
pub use units::{Alarm, Crest, Cross, Drift, Fade, Smooth, Spread, Tally, Warm};
pub use values::{
    Alarms,
    Ash,
    Crossed,
    Edge,
    Field,
    Glow,
    Heat,
    LIMIT,
    Level,
    Num,
    ONE,
    Overheat,
    Peak,
    Pool,
    Pos,
    RECORDS,
    TALL,
    THRESHOLD,
    Vel,
    WIDE,
};
