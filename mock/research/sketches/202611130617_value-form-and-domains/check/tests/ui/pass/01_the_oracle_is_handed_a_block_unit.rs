use core::cell::Cell;

use author::Diffuse;
use check::{Tally, seeded};

fn main() {
    let mut world = seeded(2, 2, 1);
    let tally = Cell::new(0);
    // The engine runs both forms.
    engine::pass(&(Diffuse, (Tally, ())), &mut world, &tally);
    assert_eq!(tally.get(), 1);
}
