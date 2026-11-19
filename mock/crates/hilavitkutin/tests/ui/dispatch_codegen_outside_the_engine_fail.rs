//! A codegen written outside the engine is refused: `DispatchCodegen`
//! takes the engine's seal, which no outside crate can name or implement.

use hilavitkutin::dispatch::codegen::DispatchCodegen;

struct MyCodegen;

impl<Cfg> DispatchCodegen<Cfg> for MyCodegen {
    type CoreDispatch = ();
}

fn main() {}
