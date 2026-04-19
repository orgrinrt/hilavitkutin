//! `AsStr` + `IntoStr` across `Str` and `&'static str`.

use std::cell::RefCell;

use hilavitkutin_str::{
    str_const, ArenaInterner, AsStr, IntoStr, Str, StringInterner,
};

struct NullArena {
    next: RefCell<u32>,
    last: RefCell<String>,
}

impl NullArena {
    fn new() -> Self {
        Self {
            next: RefCell::new(0),
            last: RefCell::new(String::new()),
        }
    }
}

impl ArenaInterner for NullArena {
    fn arena_intern(&self, s: &str) -> u32 {
        *self.last.borrow_mut() = s.to_string();
        let mut n = self.next.borrow_mut();
        let id = *n;
        *n += 1;
        id
    }
    fn arena_resolve(&self, _id: u32) -> &str {
        // SAFETY: test-only; we only call resolve after a matching
        // intern and don't mutate `last` between the two.
        let s: &str = &self.last.borrow();
        unsafe { core::mem::transmute::<&str, &str>(s) }
    }
}

#[test]
fn as_str_on_str_returns_self() {
    let s = str_const!("as-str-const");
    assert_eq!(AsStr::as_str(&s), s);
}

#[test]
fn into_str_passes_through_existing_str() {
    let interner = StringInterner::new(NullArena::new());
    let s = str_const!("into-str-pass");
    let out: Str = s.into_str(&interner);
    assert_eq!(out, s);
}

#[test]
fn into_str_interns_static_str() {
    let interner = StringInterner::new(NullArena::new());
    let out: Str = "into-str-new-runtime".into_str(&interner);
    // Not previously declared in any str_const!, so runtime.
    assert!(out.is_runtime());
}

#[test]
fn into_str_short_circuits_on_known_const() {
    let c = str_const!("ergo-const-hit");
    let interner = StringInterner::new(NullArena::new());
    let out: Str = "ergo-const-hit".into_str(&interner);
    assert_eq!(out, c);
    assert!(out.is_const());
}
