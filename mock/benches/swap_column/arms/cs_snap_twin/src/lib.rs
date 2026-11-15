//! Variant: the `Snapshot` arm again, under a second name.
//!
//! The distance between this arm and `cs_snap` is the noise floor of a run
//! of the swap_column bench: no smaller difference between two arms counts.

swap_common::column_arm!(
    "cs_snap_twin",
    run_cs_snap_twin,
    swap_common::prepare_column_snapshot
);
