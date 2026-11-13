//! Sketch S3, the engine side: the committed frame view, the staged handles,
//! the two kernel forms, the close unit, and a one-thread reference runner.
//!
//! Stands in for hilavitkutin. Every check the sketch is about sits here, so
//! the crate standing in for an author (`author`) declares lanes, a registry
//! and kernels and needs no feature line of its own.
//!
//! The one feature line is `const_trait_impl`, for `const impl Cardinal for
//! Slot` in `slot.rs`: `notko-hlist`'s `Position::INDEX` is defined only for a
//! count implementing `Cardinal` as a const trait
//! (`notko-hlist/src/position_const_path.rs:64`), and the position mask is
//! folded from that index.

#![no_std]
#![feature(const_trait_impl)]

mod bound;
mod frame;
mod kernel;
mod lane;
mod registry;
mod sealed;
mod slot;
mod stage;
mod view;

pub use bound::{
    BoundKernel,
    Checked,
    Column,
    ColumnStaged,
    ColumnView,
    Columns,
    Overlap,
    Step,
    Undeclared,
};
pub use frame::Frame;
pub use kernel::{Block, BlockKernel, CloseUnit, Kernel, Staged, Value, ValueKernel, Values};
pub use lane::{FrameLane, FrameList, LandedList, LandedStore, Lane};
pub use registry::{Buf, Pick, RawLanes, Registry, Tag};
pub use slot::Slot;
pub use stage::Stage;
pub use view::{Landed, Row, Rows, StagedLane, View};
