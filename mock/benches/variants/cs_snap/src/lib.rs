//! Variant: the middle column is buffered under `Snapshot`, so the boundary copies its live side over the frozen one.
//!
//! One arm of the swap_column bench. The carrier and the arm's body live in
//! `swap_common`; the arms differ in the middle column alone.

swap_common::column_arm!("cs_snap", run_cs_snap, swap_common::prepare_column_snapshot);
