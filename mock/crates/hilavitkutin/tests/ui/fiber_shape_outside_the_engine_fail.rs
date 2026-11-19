//! A fiber shape declared outside the engine is refused: `FiberShape`
//! is a codegen artefact and takes the engine's seal, which no outside
//! crate can name or implement.

use arvo::USize;
use hilavitkutin::dispatch::codegen::FiberShape;

struct MyShape;

impl FiberShape for MyShape {
    type WuTuple = ();

    const SHAPE_ID: USize = USize(0);
}

fn main() {}
