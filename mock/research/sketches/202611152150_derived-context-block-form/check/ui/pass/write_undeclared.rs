// The twin of `ui/fail/write_undeclared.rs`, differing in the one token: it
// writes `Glow`, which its write list holds.

use derived::{Block, Records};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::member::Written;
use hilavitkutin_api::store::Column;
use unit::{Glow, Pos};

pub struct Warm;

impl Block for Warm {
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Pos>, Empty>;
    type Write = Cons<Written<Column<Glow>>, Empty>;

    fn run(&self, records: &impl Records<Self::Read, Self::Write>) {
        records.each(|i| unsafe {
            let pos = records.read::<Pos, _>(i);
            records.write::<Glow, _>(i, Glow(pos.0 + pos.0));
        });
    }
}

fn main() {}
