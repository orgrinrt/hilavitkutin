use api::{Cons, Empty, Holds};
use author::HeatAt;

fn needs<L: Holds<HeatAt>>() {}

fn main() {
    needs::<Cons<HeatAt, Empty>>();
}
