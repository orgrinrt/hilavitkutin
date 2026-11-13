//! The comparison of every E1 runner against the plain model.

extern crate e1_run_fused as fused;
extern crate e1_run_ordered as ordered;
extern crate e1_run_permuted as permuted;
extern crate e1_run_registry as registry;
extern crate e1_run_threaded as threaded;
extern crate e1_unit as unit;
extern crate e1_vocab as vocab;

#[path = "../../../shared/model.rs"]
mod model;
#[path = "../../../shared/same_bytes.rs"]
mod same_bytes;
