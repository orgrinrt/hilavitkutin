use api::{Cons, Empty, Holds};
use author::HeatAt;

struct Other;

fn needs<L: Holds<Other>>() {}

fn main() {
    needs::<Cons<HeatAt, Empty>>();
}
