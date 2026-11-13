//! Twin: the store side, a visiting order's visitor, is handed positions and may use them.
use hilavitkutin_contracts::num::{Count, Index};
use hilavitkutin_contracts::store::{Visit, Visitor};
use hilavitkutin_frozen_implementors::Forward;

struct Last(Index);

impl Visitor for Last {
    fn morsel(&mut self, _len: Count) {}

    fn record(&mut self, at: Index) {
        self.0 = self.0.max(at);
    }
}

fn main() {
    let mut last = Last(Index::FIRST);
    Forward.visit(Count::from_host(4), Count::ONE, &mut last);
    assert_eq!(last.0, Index::from_host(3));
}
