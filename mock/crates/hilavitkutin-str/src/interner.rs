//! `ArenaInterner` + `StringInterner<A>` — runtime interning with
//! const-table short-circuit.

use arvo_bits::Bits;

use crate::handle::Str;
use crate::hash::const_fnv1a;
use crate::section::static_entries;

/// Host-implemented arena contract. Only handles runtime strings —
/// const strings are short-circuited by `StringInterner`.
pub trait ArenaInterner {
    /// Insert `s` into the arena and return its 28-bit ID.
    fn arena_intern(&self, s: &str) -> u32;
    /// Resolve a previously-returned arena ID back to the stored string.
    fn arena_resolve(&self, id: u32) -> &str;
}

/// Wraps an [`ArenaInterner`] with const-table handling. The const
/// table (linker section) is always consulted first.
pub struct StringInterner<A: ArenaInterner> { // lint:allow(no-alloc) -- interner wrapper name, not std `String`.
    arena: A,
}

impl<A: ArenaInterner> StringInterner<A> { // lint:allow(no-alloc) -- interner wrapper name, not std `String`.
    /// Construct a new interner wrapping `arena`.
    pub const fn new(arena: A) -> Self {
        Self { arena }
    }

    /// Borrow the wrapped arena.
    pub fn arena(&self) -> &A {
        &self.arena
    }

    /// Intern a string. Checks the const linker section first; on miss,
    /// falls back to the arena and tags the result as runtime.
    pub fn intern(&self, s: &str) -> Str {
        if let Some(h) = lookup_const_by_value(s) {
            return h;
        }
        let id = self.arena.arena_intern(s);
        Str::__runtime(Bits::<28>::new(id as u64))
    }

    /// Intern a `'static` string. Same semantics as `intern`; the
    /// arena may be able to avoid copying.
    pub fn intern_static(&self, s: &'static str) -> Str {
        if let Some(h) = lookup_const_by_value(s) {
            return h;
        }
        let id = self.arena.arena_intern(s);
        Str::__runtime(Bits::<28>::new(id as u64))
    }

    /// Resolve a handle back to a string.
    ///
    /// Const handles delegate to the linker-section table; a
    /// const-table miss returns `None`. Runtime handles always
    /// resolve via the arena — an arena that cannot resolve is
    /// outside the `ArenaInterner` contract (the interner hands
    /// out ids it can resolve), so the runtime branch returns
    /// `Some(...)` unconditionally.
    pub fn resolve(&self, s: Str) -> Option<&str> {
        if s.is_const() {
            lookup_const_by_handle(s)
        } else {
            Some(self.arena.arena_resolve(s.id().bits() as u32))
        }
    }
}

/// Linear scan for a const-section entry matching `s` (by hash, then
/// by content to rule out 28-bit truncation collisions).
fn lookup_const_by_value(s: &str) -> Option<Str> {
    let want = Str::__make(Bits::<28>::new(const_fnv1a(s) & 0x0FFF_FFFF));
    for entry in static_entries() {
        if entry.hash == want && str_eq(entry.value, s) {
            return Some(entry.hash);
        }
    }
    None
}

/// Linear scan for a const-section entry matching a handle.
fn lookup_const_by_handle(h: Str) -> Option<&'static str> {
    for entry in static_entries() {
        if entry.hash == h {
            return Some(entry.value);
        }
    }
    None
}

/// `no_std`-safe string equality.
fn str_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}
