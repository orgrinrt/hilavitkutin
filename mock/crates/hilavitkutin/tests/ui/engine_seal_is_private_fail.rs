//! The engine's seal cannot be named outside the engine: its module is
//! private, so no outside type can take the seal and implement a codegen
//! trait. Were the module made public, this fixture would build.

use hilavitkutin::sealed::EngineOnly;

struct MyShape;

impl EngineOnly for MyShape {}

fn main() {}
