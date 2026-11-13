//! The comparison of every E2 runner against the plain model.

extern crate e2_run_fused as fused;
extern crate e2_run_ordered as ordered;
extern crate e2_run_permuted as permuted;
extern crate e2_run_registry as registry;
extern crate e2_run_threaded as threaded;
extern crate e2_unit as unit;
extern crate e2_vocab as vocab;

#[path = "../../../shared/model.rs"]
mod model;
#[path = "../../../shared/same_bytes.rs"]
mod same_bytes;
