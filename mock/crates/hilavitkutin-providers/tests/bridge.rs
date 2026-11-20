//! A consumer's own arena reaches its interner through `InternerApi` on
//! a default build, where the crate carries no arena of its own.

use core::cell::Cell;

use hilavitkutin_providers::InternerApi;
use hilavitkutin_str::{ArenaInterner, StringInterner};
use notko::Maybe;

/// An arena of the test's own: one slot, holding the first string it is
/// given and refusing any other.
struct OneSlot {
    held: Cell<Option<&'static str>>,
}

impl ArenaInterner for OneSlot {
    fn arena_intern(&self, s: &str) -> u32 {
        match self.held.get() {
            None => {
                let kept: &'static str = std::boxed::Box::leak(s.into());
                self.held.set(Some(kept));
                0
            },
            Some(h) if h == s => 0,
            Some(_) => 1 << 28,
        }
    }

    fn arena_resolve(&self, id: u32) -> &str {
        match (id, self.held.get()) {
            (0, Some(h)) => h,
            _ => "",
        }
    }
}

fn through_the_contract<I: InternerApi>(interner: &I) {
    let Maybe::Is(handle) = interner.intern("a string no const table holds") else {
        panic!("the one slot was free, so interning must give a handle");
    };
    match interner.resolve(handle) {
        Maybe::Is(s) => assert_eq!(s, "a string no const table holds"),
        Maybe::Isnt => panic!("the handle the interner gave must resolve"),
    }
    assert!(matches!(
        interner.intern("another runtime string"),
        Maybe::Isnt
    ));
}

#[test]
fn a_consumer_arena_is_an_interner_api_provider_on_the_default_build() {
    let interner = StringInterner::new(OneSlot {
        held: Cell::new(None),
    });
    through_the_contract(&interner);
}
