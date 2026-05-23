//! Double-derive fixture per Topic 3 S2 + audit-2 M5: combining
//! `Resource` and `RunCfg` derive macros on the same type must
//! produce a coherent compile error.
//!
//! Pre-derive-macro shape: derives do not exist in
//! `hilavitkutin-api` yet. This fixture exercises the same shape
//! the eventual derives target by colliding the `BuilderInput`
//! `Dispatch` slot via two parallel `impl` blocks that pick
//! incompatible `Dispatch` routers. The trait solver should reject
//! the second impl as a conflicting blanket; the engine's design
//! intent (a type has exactly one role on the scheduler) lands as
//! a clear diagnostic.
//!
//! When the `Resource` + `RunCfg` derive macros ship, this fixture
//! migrates to actual `#[derive(Resource)] #[derive(RunCfg)]`
//! syntax and the expected `.stderr` updates accordingly.

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
