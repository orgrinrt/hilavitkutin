//! The twin: the null clock is named on every build.

use hilavitkutin::scheduler::NullClock as Clock;

fn main() {
    let _ = core::mem::size_of::<Clock>();
}
