//! The marker of a plain column names the store in a unit's lists. It says
//! nothing of what the first pass reads, so it is not what a store is
//! registered with.

use hilavitkutin_api::Column;
use hilavitkutin_api::builder_input::BuilderInput;

#[derive(Clone, Copy, PartialEq, Default)]
struct Temp(u32);

fn registered<I: BuilderInput>(_: I) {}

fn main() {
    registered(Column::<Temp>::new());
}
