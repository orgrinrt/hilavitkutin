//! `ArenaInterner` + `StringInterner<A>` — runtime interning with
//! const-table short-circuit.

use arvo_bits::Bits;
use notko::Maybe;

use crate::handle::Str;
use crate::hash::const_fnv1a;
use crate::section::static_entries;

/// Host-implemented arena contract. Only handles runtime strings —
/// const strings are short-circuited by `StringInterner`.
pub trait ArenaInterner {
    /// Insert `s` into the arena and return its 28-bit ID.
    fn arena_intern(&self, s: &str) -> u32; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-bare-string) reason: interner boundary — &str is the input string the arena wraps; u32 is the 28-bit id width; tracked: #72
    /// Resolve a previously-returned arena ID back to the stored string.
    fn arena_resolve(&self, id: u32) -> &str; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-bare-string) reason: interner boundary — resolved &str; u32 is the 28-bit id width; tracked: #72
}

/// Wraps an [`ArenaInterner`] with const-table handling. The const
/// table (linker section) is always consulted first.
pub struct StringInterner<A: ArenaInterner> { // lint:allow(no-alloc) reason: interner wrapper name, not std `String`; tracked: #72
    arena: A,
}

impl<A: ArenaInterner> StringInterner<A> { // lint:allow(no-alloc) reason: interner wrapper name, not std `String`; tracked: #72
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
    pub fn intern(&self, s: &str) -> Str { // lint:allow(no-bare-string) reason: interner boundary — incoming &str; tracked: #72
        if let Maybe::Is(h) = lookup_const_by_value(s) {
            return h;
        }
        let id = self.arena.arena_intern(s);
        Str::__runtime((id as u64).into()) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: u32 id widened to u64 for `Bits<28>::from`; tracked: #72
    }

    /// Intern a `'static` string. Same semantics as `intern`; the
    /// arena may be able to avoid copying.
    pub fn intern_static(&self, s: &'static str) -> Str {
        if let Maybe::Is(h) = lookup_const_by_value(s) {
            return h;
        }
        let id = self.arena.arena_intern(s);
        Str::__runtime((id as u64).into()) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: u32 id widened to u64 for `Bits<28>::from`; tracked: #72
    }

    /// Resolve a handle back to a string.
    ///
    /// Const handles delegate to the linker-section table; a
    /// const-table miss returns `Maybe::Isnt`. Runtime handles
    /// always resolve via the arena — an arena that cannot resolve
    /// is outside the `ArenaInterner` contract (the interner hands
    /// out ids it can resolve), so the runtime branch returns
    /// `Maybe::Is(...)` unconditionally.
    pub fn resolve(&self, s: Str) -> Maybe<&str> { // lint:allow(no-bare-string) reason: interner boundary — resolved &str; tracked: #72
        if s.is_const().0 {
            lookup_const_by_handle(s)
        } else {
            Maybe::Is(self.arena.arena_resolve(s.id().bits() as u32)) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: u32 cast from Bits back to id width for arena resolve; tracked: #72
        }
    }
}

/// Linear scan for a const-section entry matching `s` (by hash, then
/// by content to rule out 28-bit truncation collisions).
fn lookup_const_by_value(s: &str) -> Maybe<Str> { // lint:allow(no-bare-string) reason: interner-internal &str math; mirrors boundary width; tracked: #72
    let want = Str::__make((const_fnv1a(s) & 0x0FFF_FFFF).into());
    for entry in static_entries() {
        if entry.hash == want && str_eq(entry.value, s) {
            return Maybe::Is(entry.hash);
        }
    }
    Maybe::Isnt
}

/// Linear scan for a const-section entry matching a handle.
fn lookup_const_by_handle(h: Str) -> Maybe<&'static str> {
    for entry in static_entries() {
        if entry.hash == h {
            return Maybe::Is(entry.value);
        }
    }
    Maybe::Isnt
}

/// `no_std`-safe string equality.
fn str_eq(a: &str, b: &str) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-bare-string) reason: interner-internal &str equality helper; returns bare bool because it is below the API boundary; tracked: #72
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
