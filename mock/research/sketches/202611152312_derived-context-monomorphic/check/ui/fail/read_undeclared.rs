// The twin of `unit::Drift` that reads a column its read list does not hold:
// `Glow` in place of `Vel`. Names the api only.

use arvo::USize;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::block::{Block, Records};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::Column;
use unit::{Glow, Pos, Vel};

pub struct Drift;

impl Block for Drift {
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;
    type Write = Cons<Column<Pos>, Empty>;

    fn run(&self, records: &impl Records<Self::Read, Self::Write>) {
        let mut i = USize(0);
        while i.0 < records.len().0 {
            unsafe {
                let pos = records.read::<Pos, _>(i);
                let vel = records.read::<Glow, _>(i);
                records.write(i, Pos(pos.0 + vel.0));
            }
            i = USize(i.0 + 1);
        }
    }
}

fn main() {}
