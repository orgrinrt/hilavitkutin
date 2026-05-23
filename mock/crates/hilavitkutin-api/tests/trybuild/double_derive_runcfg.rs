//! Harness check for the trybuild test framework wiring.
//!
//! The eventual `Resource` and `RunCfg` derive macros (Topic 3 S2 +
//! audit-2 M5) must reject the double-derive combination on the
//! same type with a coherent compile error. That real fixture
//! lands when the derive macros ship.
//!
//! Until then, this file holds two duplicate `impl BuilderInput
//! for DoubleRole` blocks. The rustc E0119 "conflicting
//! implementations" diagnostic this triggers is a generic trait-
//! solver rejection (it would fire for any trait with two
//! duplicate impls), not a probe of the eventual derive-macro
//! design. The point of shipping this file today is to wire
//! trybuild + golden `.stderr` capture into the test harness so
//! the real fixture has a working pipeline to slot into.
//!
//! Migration when the macros land: replace the two `impl` blocks
//! with `#[derive(Resource)] #[derive(RunCfg)] struct DoubleRole;`
//! and re-capture the `.stderr` golden against whichever derive-
//! macro-emitted error reflects the design intent.

use hilavitkutin_api::builder_input::{BuilderInput, StoreDispatch};
use hilavitkutin_api::run_cfg::RunCfgDispatch;

struct DoubleRole;

impl BuilderInput for DoubleRole {
    type Init = Self;
    type Dispatch = StoreDispatch<Self>;
}

impl BuilderInput for DoubleRole {
    type Init = Self;
    type Dispatch = RunCfgDispatch<Self>;
}

fn main() {}
