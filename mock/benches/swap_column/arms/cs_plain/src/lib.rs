//! Variant: the middle column is plain, so the reader follows the producer in the same pass and no boundary brings anything in.
//!
//! The floor of the swap_column bench. The carrier and the arm's body live
//! in `swap_common`; the arms differ in the middle column alone.

swap_common::column_arm!("cs_plain", run_cs_plain, swap_common::prepare_column_plain);
