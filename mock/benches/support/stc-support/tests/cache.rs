//! `cache::with`: the same key reuses the state, and a different key value,
//! a different key type or a different state type rebuilds it, dropping the
//! old state before the new one is built.

use std::cell::Cell;
use std::rc::Rc;

use stc_support::cache;

/// State that records, on drop, how many states were alive at that moment.
struct Probe {
    id:    u32,
    alive: Rc<Cell<u32>>,
}

impl Probe {
    fn new(id: u32, alive: &Rc<Cell<u32>>) -> Self {
        alive.set(alive.get() + 1);
        Probe {
            id,
            alive: Rc::clone(alive),
        }
    }
}

impl Drop for Probe {
    fn drop(&mut self) {
        self.alive.set(self.alive.get() - 1);
    }
}

/// Run `with` and report the state's id and whether `make` ran.
fn visit<K: PartialEq + 'static>(key: K, id: u32, alive: &Rc<Cell<u32>>) -> (u32, bool) {
    let built = Cell::new(false);
    let seen = cache::with(
        key,
        || {
            // The old state is gone before the new one exists.
            assert_eq!(alive.get(), 0, "two states alive at once");
            built.set(true);
            Probe::new(id, alive)
        },
        |p: &mut Probe| p.id,
    );
    (seen, built.get())
}

#[test]
fn the_same_key_reuses_the_state() {
    cache::clear();
    let alive = Rc::new(Cell::new(0));
    assert_eq!(visit(1u32, 10, &alive), (10, true));
    assert_eq!(visit(1u32, 11, &alive), (10, false));
    assert_eq!(alive.get(), 1);
    cache::clear();
    assert_eq!(alive.get(), 0);
}

#[test]
fn another_key_value_rebuilds_dropping_the_old_state_first() {
    cache::clear();
    let alive = Rc::new(Cell::new(0));
    assert_eq!(visit(1u32, 10, &alive), (10, true));
    assert_eq!(visit(2u32, 20, &alive), (20, true));
    assert_eq!(alive.get(), 1);
    cache::clear();
}

/// A key of another type whose value would compare equal as bits is still
/// another key.
#[test]
fn a_key_of_another_type_rebuilds() {
    cache::clear();
    let alive = Rc::new(Cell::new(0));
    assert_eq!(visit(1u32, 10, &alive), (10, true));
    assert_eq!(visit(1i32, 20, &alive), (20, true));
    assert_eq!(visit(1i32, 21, &alive), (20, false));
    assert_eq!(alive.get(), 1);
    cache::clear();
}

/// The same key with a state of another type rebuilds.
#[test]
fn a_state_of_another_type_rebuilds() {
    cache::clear();
    let alive = Rc::new(Cell::new(0));
    assert_eq!(visit(1u32, 10, &alive), (10, true));
    let n = cache::with(1u32, || 7u64, |s: &mut u64| *s);
    assert_eq!(n, 7);
    assert_eq!(alive.get(), 0, "the probe was dropped for the new state");
    cache::clear();
}
