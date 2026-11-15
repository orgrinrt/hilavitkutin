//! Variant: the middle column is buffered under `Exchange`, so the boundary turns its two sides and copies nothing.
//!
//! One arm of the swap_column bench. The carrier and the arm's body live in
//! `swap_common`; the arms differ in the middle column alone.

swap_common::column_arm!("cs_exch", run_cs_exch, swap_common::prepare_column_exchange);
