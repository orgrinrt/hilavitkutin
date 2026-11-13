//! Kernels written against the contracts and nothing else, each beside its model, frozen: once
//! landed they change only through a line in `BREAKS.toml`.
//!
//! Author 1 is [`Diffuse`], a value kernel on the two-axis domain [`Plane`], generic over its lane.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

mod diffuse;
pub mod model;
mod plane;

pub use diffuse::{Diffuse, Heat, Rate};
pub use plane::{East, Extent, North, Plane, Shift, South, Spot, Toward, West};
