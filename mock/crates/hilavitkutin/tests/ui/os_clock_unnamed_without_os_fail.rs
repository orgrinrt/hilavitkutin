//! A build without `os` carries no operating-system clock.

use hilavitkutin::OsClock as Clock;

fn main() {
    let _ = core::mem::size_of::<Clock>();
}
