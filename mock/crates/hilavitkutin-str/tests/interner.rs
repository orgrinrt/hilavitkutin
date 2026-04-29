//! `StringInterner` exercised with a test-local `VecInterner`
//! implementing `ArenaInterner`.

use std::cell::RefCell;

use hilavitkutin_str::{str_const, ArenaInterner, Str, StringInterner};

struct VecInterner {
    strings: RefCell<Vec<String>>,
}

impl VecInterner {
    fn new() -> Self {
        Self {
            strings: RefCell::new(Vec::new()),
        }
    }
}

impl ArenaInterner for VecInterner {
    fn arena_intern(&self, s: &str) -> u32 {
        let mut v = self.strings.borrow_mut();
        for (i, existing) in v.iter().enumerate() {
            if existing == s {
                return i as u32;
            }
        }
        let id = v.len() as u32;
        v.push(s.to_string());
        id
    }

    fn arena_resolve(&self, id: u32) -> &str {
        let v = self.strings.borrow();
        // SAFETY: test-only. `Vec<String>` reallocations would invalidate
        // the returned reference, but each test intern a bounded number
        // of strings with adequate slack so no reallocations occur
        // within a single test body.
        let s: &str = &v[id as usize];
        unsafe { core::mem::transmute::<&str, &str>(s) }
    }
}

#[test]
fn runtime_intern_roundtrip() {
    let interner = StringInterner::new(VecInterner::new());
    let h = interner.intern("ephemeral-runtime-a");
    assert!(h.is_runtime().0);
    assert_eq!(interner.resolve(h), Some("ephemeral-runtime-a"));
}

#[test]
fn runtime_intern_dedups() {
    let interner = StringInterner::new(VecInterner::new());
    let a = interner.intern("dup-runtime");
    let b = interner.intern("dup-runtime");
    assert_eq!(a, b);
    assert!(a.is_runtime().0);
}

#[test]
fn runtime_different_strings_different_handles() {
    let interner = StringInterner::new(VecInterner::new());
    let a = interner.intern("one");
    let b = interner.intern("two");
    assert_ne!(a, b);
}

#[test]
fn const_short_circuits_on_intern() {
    // Ensure the const literal is registered by referencing it.
    let c = str_const!("interner-const-hit");
    let interner = StringInterner::new(VecInterner::new());
    let i = interner.intern("interner-const-hit");
    assert_eq!(c, i);
    assert!(i.is_const().0);
    // Resolve should come back via the linker-section table.
    assert_eq!(interner.resolve(i), Some("interner-const-hit"));
}

#[test]
fn intern_static_short_circuits_on_const() {
    let c = str_const!("interner-const-static");
    let interner = StringInterner::new(VecInterner::new());
    let i = interner.intern_static("interner-const-static");
    assert_eq!(c, i);
    assert!(i.is_const().0);
}

#[test]
fn resolve_runtime_delegates_to_arena() {
    let interner = StringInterner::new(VecInterner::new());
    let h: Str = interner.intern("arena-delegate");
    assert!(h.is_runtime().0);
    assert_eq!(interner.resolve(h), Some("arena-delegate"));
}
