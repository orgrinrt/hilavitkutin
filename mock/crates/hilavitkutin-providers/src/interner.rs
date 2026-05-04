//! Interner provider surface and default arena impl.
//!
//! Provides the [`InternerApi`] provider trait, the
//! [`HasInterner`] accessor, the [`MemoryArena`] inline-storage
//! arena, and the [`default_interner`] constructor. WorkUnits that
//! handle [`Str`] declare `HasInterner` in their `Ctx` tuple bound
//! and call `ctx.interner().intern(s)` /
//! `ctx.interner().resolve(handle)` from `execute()`.

use core::cell::{Cell, UnsafeCell};

use arvo::USize;
use hilavitkutin_str::{ArenaInterner, Str, StringInterner};
use notko::Maybe;

/// Provider-shape trait for the interner.
///
/// Parallels the platform contracts [`MemoryProviderApi`] /
/// [`ClockApi`] / [`ThreadPoolApi`] in `hilavitkutin-api`. The
/// `'static` bound lets the value live in a `Resource<T>`. v0 does
/// not require `Send + Sync`; the engine's resource access during
/// single-thread morsel dispatch is sequential. The Sync arena
/// variant (BACKLOG) generalises this for multi-thread schedules.
///
/// [`MemoryProviderApi`]: hilavitkutin_api::MemoryProviderApi
/// [`ClockApi`]: hilavitkutin_api::ClockApi
/// [`ThreadPoolApi`]: hilavitkutin_api::ThreadPoolApi
pub trait InternerApi: 'static {
    /// Intern a string. Const-table short-circuit applies before
    /// the arena lookup.
    fn intern(&self, s: &str) -> Str; // lint:allow(no-bare-string) reason: interner boundary; the input is the &str the consumer passes; tracked: #72

    /// Resolve a handle back to a string. Returns `Maybe::Isnt` if
    /// the handle's source is no longer reachable.
    fn resolve(&self, s: Str) -> Maybe<&str>; // lint:allow(no-bare-string) reason: interner boundary; resolved &str; tracked: #72
}

/// Accessor for the interner provider in a Context tuple.
///
/// Mirrors `HasMemoryProvider` / `HasClock` / `HasThreadPool` from
/// `hilavitkutin-api`. WorkUnits that need to intern or resolve
/// strings declare `HasInterner` in their `Ctx` tuple bound.
pub trait HasInterner {
    /// Concrete interner implementation type.
    type Provider: InternerApi;

    /// Borrow the interner provider.
    fn interner(&self) -> &Self::Provider;
}

/// Blanket: any `StringInterner<A>` satisfies the InternerApi
/// surface. Consumers wire `Resource<StringInterner<A>>` and the
/// HasInterner accessor returns it; this impl bridges the trait.
impl<A> InternerApi for StringInterner<A>
where
    A: ArenaInterner + 'static,
{
    #[inline(always)]
    fn intern(&self, s: &str) -> Str { // lint:allow(no-bare-string) reason: interner boundary mirror of the trait method; tracked: #72
        StringInterner::intern(self, s)
    }

    #[inline(always)]
    fn resolve(&self, s: Str) -> Maybe<&str> { // lint:allow(no-bare-string) reason: interner boundary mirror of the trait method; tracked: #72
        StringInterner::resolve(self, s)
    }
}

/// Inline-storage arena for runtime-interned strings.
///
/// `BYTES` is the byte-buffer capacity; `ENTRIES` is the maximum
/// distinct runtime-interned string count. Consumers tune both to
/// their workload.
///
/// Interior mutability: [`Cell`] for the cursor and entry count,
/// [`UnsafeCell`] for the byte buffer and entry table. The arena
/// is `!Sync` by construction; multi-thread access needs a
/// consumer-supplied Mutex shim or a future SyncArena variant
/// (tracked in BACKLOG).
///
/// On capacity overflow (out of bytes or out of entry slots),
/// `arena_intern` returns sentinel id `u32::MAX`. The companion
/// `arena_resolve` returns an empty slice for the sentinel; the
/// `StringInterner` wrapping this arena observes the empty
/// resolution as `Maybe::Is(&"")`.
pub struct MemoryArena<const BYTES: usize, const ENTRIES: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array sizes; rust grammar requires usize; tracked: #121
    bytes: UnsafeCell<[u8; BYTES]>, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic byte buffer; tracked: #121
    entries: UnsafeCell<[Entry; ENTRIES]>,
    cursor: Cell<USize>,
    count: Cell<USize>,
}

#[derive(Copy, Clone)]
struct Entry {
    offset: USize,
    len: USize,
}

impl<const BYTES: usize, const ENTRIES: usize> MemoryArena<BYTES, ENTRIES> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; tracked: #121
    /// Construct an empty arena. Buffer initialised to zero, entry
    /// table initialised with zero offsets and zero lengths.
    pub const fn new() -> Self {
        Self {
            bytes: UnsafeCell::new([0u8; BYTES]), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic byte buffer initialisation; tracked: #121
            entries: UnsafeCell::new(
                [Entry { offset: USize(0), len: USize(0) }; ENTRIES],
            ),
            cursor: Cell::new(USize(0)),
            count: Cell::new(USize(0)),
        }
    }
}

impl<const BYTES: usize, const ENTRIES: usize> Default for MemoryArena<BYTES, ENTRIES> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; tracked: #121
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

const SENTINEL: u32 = u32::MAX; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: arena id width is the ArenaInterner contract (u32); tracked: #72

impl<const BYTES: usize, const ENTRIES: usize> ArenaInterner for MemoryArena<BYTES, ENTRIES> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; tracked: #121
    fn arena_intern(&self, s: &str) -> u32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-bare-string) reason: ArenaInterner trait method signature; tracked: #72
        let bytes = s.as_bytes();
        let len = bytes.len();
        let cursor = self.cursor.get().0;
        let count = self.count.get().0;

        if count >= ENTRIES || cursor.saturating_add(len) > BYTES {
            return SENTINEL;
        }

        // Append bytes to the buffer.
        // SAFETY: pointer-arithmetic-only write avoids `&mut`
        // reborrow over the buffer place. Stacked Borrows /
        // Tree Borrows admit overlapping `&self` calls (e.g.
        // arena_intern issued while a previously-returned `&str`
        // from arena_resolve is still live); creating `&mut` over
        // the same allocation under those borrow stacks would be
        // UB. The append-only allocator invariant guarantees the
        // bytes at `[cursor..cursor+len]` are not read by any
        // outstanding `&str` (which can only point into prior
        // ranges `[old_offset..old_offset+old_len]` with
        // `old_offset+old_len <= cursor`). `cursor + len <= BYTES`
        // checked above; the arena is `!Sync` so no concurrent
        // mutator on another thread either.
        unsafe {
            let buf_ptr = (self.bytes.get() as *mut u8).add(cursor);
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), buf_ptr, len);
        }

        // Record the entry.
        // SAFETY: pointer-arithmetic-only write to the entry table
        // for the same Stacked Borrows reason as above. `count <
        // ENTRIES` checked above; previously-returned `Entry`
        // values were `read()` by value (Copy), so no shared borrow
        // remains pointing into `self.entries`.
        unsafe {
            let entries_ptr = self.entries.get() as *mut Entry;
            entries_ptr.add(count).write(Entry { offset: USize(cursor), len: USize(len) });
        }

        let id = count as u32; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: ArenaInterner contract returns u32 ids; tracked: #72
        self.cursor.set(USize(cursor + len));
        self.count.set(USize(count + 1));
        id
    }

    fn arena_resolve(&self, id: u32) -> &str { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-bare-string) reason: ArenaInterner trait method signature; tracked: #72
        if id == SENTINEL {
            return "";
        }
        let idx = id as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: u32 id widens to usize for array index; tracked: #121
        // SAFETY: pointer-arithmetic-only access into the entry
        // table avoids the implicit-autoref pitfall on raw pointer
        // deref. The arena is !Sync so no concurrent mutator can
        // be writing.
        let entry: Entry = unsafe {
            let entries_ptr = self.entries.get() as *const Entry;
            entries_ptr.add(idx).read()
        };
        let offset = entry.offset.0;
        let len = entry.len.0;
        // SAFETY: same single-threaded invariant; offset+len was
        // recorded by a prior arena_intern that wrote valid utf-8
        // bytes into [offset..offset+len]. The slice is built from
        // a raw pointer to avoid the autoref-of-deref shape.
        let slice = unsafe {
            let buf_ptr = self.bytes.get() as *const u8;
            core::slice::from_raw_parts(buf_ptr.add(offset), len)
        };
        // SAFETY: bytes came from the original `&str` argument to
        // arena_intern, which is guaranteed valid utf-8.
        unsafe { core::str::from_utf8_unchecked(slice) } // lint:allow(no-bare-string) reason: trait method returns &str; tracked: #72
    }
}

/// Build a default interner (StringInterner wrapping a fresh
/// MemoryArena). Consumers wire it onto the scheduler with
/// `builder.resource(default_interner::<BYTES, ENTRIES>())`.
///
/// Two const generics because the byte buffer's capacity (BYTES)
/// and the maximum number of distinct runtime-interned strings
/// (ENTRIES) are independent dimensions a consumer tunes for
/// their workload.
#[inline(always)]
pub const fn default_interner<const BYTES: usize, const ENTRIES: usize>() // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array sizes; tracked: #121
-> StringInterner<MemoryArena<BYTES, ENTRIES>> { // lint:allow(no-alloc) reason: StringInterner is the no-alloc interner wrapper, not std String; tracked: #72
    StringInterner::new(MemoryArena::new())
}
