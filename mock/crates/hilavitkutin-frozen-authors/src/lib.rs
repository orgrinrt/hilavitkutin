//! Work units written the way a consumer writes them, beside a hand model of
//! what a frame of them computes, and frozen: once landed, a file of this crate
//! changes only through a line in `BREAKS.toml`.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
