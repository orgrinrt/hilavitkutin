// The twin of `unit::Warm` that writes a column its write list does not hold:
// `Pos`, which it only reads. Names the block form's contract and the api only.

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
            records.write::<Pos, _>(i, Pos(pos.0 + pos.0));
        });
    }
}

fn main() {}
