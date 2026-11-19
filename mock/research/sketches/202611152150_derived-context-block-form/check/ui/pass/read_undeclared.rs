// The twin of `ui/fail/read_undeclared.rs`, differing in the one token: it
// reads `Vel`, which its read list holds.

use arvo::USize;
use derived::{Block, Records};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::Column;
use unit::{Pos, Vel};

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
                let vel = records.read::<Vel, _>(i);
                records.write(i, Pos(pos.0 + vel.0));
            }
            i = USize(i.0 + 1);
        }
    }
}

fn main() {}
