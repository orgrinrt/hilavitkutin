//! The default interner: the inline `MemoryArena`, the
//! `default_interner` constructor and the `InternerKit` preset. Under
//! the `portable` feature.

use core::cell::{Cell, UnsafeCell};

use arvo::USize;
use arvo::strategy::{Additive, Identity};
use hilavitkutin_api::builder_input::BuilderInput;
use hilavitkutin_api::{Cons, Empty, Resource};
use hilavitkutin_kit::{Kit, KitDispatch};
use hilavitkutin_str::{ArenaInterner, StringInterner};
use notko::Maybe;

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
/// `arena_intern` keeps the interning law of [`ArenaInterner`]: a
/// string equal to one already held gets that entry's id back, found
/// by a scan of the entries that compares length before bytes, so a
/// repeat spends neither bytes nor an entry slot.
///
/// On capacity overflow (out of bytes or out of entry slots),
/// `arena_intern` returns sentinel id `u32::MAX`, the arena's refusal,
/// which `StringInterner` answers with `Maybe::Isnt` and builds no handle
/// from. `arena_resolve` reads nothing for any id at or past the live
/// entry count, and answers it with `""`, which aliases `""` once the
/// arena holds it. A handle another arena issued is not told apart from
/// one of its own. Ids run below `ENTRIES`, so an `ENTRIES` of up to
/// 2^28 keeps every one inside the 28-bit `Str` id field and below the
/// refusal; one past 2^28 fails the build at [`MemoryArena::new`].
pub struct MemoryArena<
    const BYTES: usize,
    const ENTRIES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array sizes; rust grammar requires usize; tracked: #121
> {
    bytes:   UnsafeCell<[u8; BYTES]>, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic byte buffer; tracked: #121
    entries: UnsafeCell<[Entry; ENTRIES]>,
    cursor:  Cell<USize>,
    count:   Cell<USize>,
}

#[derive(Copy, Clone)]
struct Entry {
    offset: USize,
    len:    USize,
}

impl<
    const BYTES: usize,
    const ENTRIES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; tracked: #121
> MemoryArena<BYTES, ENTRIES>
{
    /// Every issued id is below `ENTRIES`, so up to 2^28 entries keep
    /// every id inside the 28-bit `Str` id field; the refusal is
    /// `u32::MAX`, outside it.
    const ENTRIES_FIT_THE_ID: () = assert!(
        ENTRIES <= 1 << 28, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the const-generic entry cap against the 28-bit id width; tracked: #121
        "MemoryArena: use at most 268435456 entries, as many as the 28-bit Str id field holds",
    );

    /// Construct an empty arena. Buffer initialised to zero, entry
    /// table initialised with zero offsets and zero lengths. An
    /// `ENTRIES` past 2^28 fails the build here.
    pub const fn new() -> Self {
        let () = Self::ENTRIES_FIT_THE_ID;
        Self {
            bytes:   UnsafeCell::new([0u8; BYTES]), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic byte buffer initialisation; tracked: #121
            entries: UnsafeCell::new(
                [Entry {
                    offset: <USize as Identity<Additive>>::IDENTITY,
                    len:    <USize as Identity<Additive>>::IDENTITY,
                }; ENTRIES],
            ),
            cursor:  Cell::new(<USize as Identity<Additive>>::IDENTITY),
            count:   Cell::new(<USize as Identity<Additive>>::IDENTITY),
        }
    }

    /// The entry index holding `bytes`, if one does: entries are
    /// compared by length first and by bytes only on an equal length.
    #[rustfmt::skip]
    fn held_id(&self, bytes: &[u8]) -> Maybe<USize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the byte view of the &str the ArenaInterner contract takes; tracked: #72
        let count = self.count.get().0;
        let mut idx = 0;
        while idx < count {
            // SAFETY: `idx < count <= ENTRIES`, and every entry below
            // `count` was written by a prior `arena_intern`. Read by
            // value through the raw pointer, as `arena_resolve` does, so
            // no reference into the table outlives the read.
            let entry: Entry = unsafe { (self.entries.get() as *const Entry).add(idx).read() };
            if entry.len.0 == bytes.len() {
                // SAFETY: `[offset..offset+len]` was written by the
                // `arena_intern` that recorded this entry and lies below
                // the cursor, so no write of this arena touches it.
                let held = unsafe {
                    core::slice::from_raw_parts((self.bytes.get() as *const u8).add(entry.offset.0), entry.len.0) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the arena's byte buffer is u8 by the ArenaInterner contract; tracked: #121
                };
                if held == bytes {
                    return Maybe::Is(USize(idx));
                }
            }
            idx += 1;
        }
        Maybe::Isnt
    }
}

impl<
    const BYTES: usize,
    const ENTRIES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; tracked: #121
> Default for MemoryArena<BYTES, ENTRIES>
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

const SENTINEL: u32 = u32::MAX; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: arena id width is the ArenaInterner contract (u32); tracked: #72

impl<
    const BYTES: usize,
    const ENTRIES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; tracked: #121
> ArenaInterner for MemoryArena<BYTES, ENTRIES>
{
    #[rustfmt::skip] // keeps the allow on the signature it governs
    fn arena_intern(
        &self,
        s: &str, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-bare-string) reason: ArenaInterner trait method signature; tracked: #72
    ) -> u32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-bare-string) reason: ArenaInterner trait method signature; tracked: #72
        let bytes = s.as_bytes();
        let len = bytes.len();
        let cursor = self.cursor.get().0;
        let count = self.count.get().0;

        // The interning law: a string already held keeps its id.
        if let Maybe::Is(held) = self.held_id(bytes) {
            return held.0 as u32; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: ArenaInterner contract returns u32 ids; an entry index is below 2^28 by ENTRIES_FIT_THE_ID; tracked: #72
        }

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
            entries_ptr.add(count).write(Entry {
                offset: USize(cursor),
                len:    USize(len),
            });
        }

        let id = count as u32; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: ArenaInterner contract returns u32 ids; tracked: #72
        self.cursor.set(USize(cursor + len));
        self.count.set(USize(count + 1));
        id
    }

    #[rustfmt::skip] // keeps the allow on the signature it governs
    fn arena_resolve(
        &self,
        id: u32, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-bare-string) reason: ArenaInterner trait method signature; tracked: #72
    ) -> &str { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-bare-string) reason: ArenaInterner trait method signature; tracked: #72
        let idx = id as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: u32 id widens to usize for array index; tracked: #121
        // Only ids below the live count are read. The sentinel never
        // reaches a `Str`; an id another arena issued is caught by the
        // count.
        if id == SENTINEL || idx >= self.count.get().0 {
            // FIXME(evict_str_turns_an_unresolvable_runtime_str_into_the_hash_of): the empty string stands in for nothing held; a `&str` return cannot say the id was not issued.
            return "";
        }
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
/// `.with(Resource::new(default_interner::<BYTES, ENTRIES>()))`.
///
/// Two const generics because the byte buffer's capacity (BYTES)
/// and the maximum number of distinct runtime-interned strings
/// (ENTRIES) are independent dimensions a consumer tunes for
/// their workload.
#[inline(always)]
pub const fn default_interner<const BYTES: usize, const ENTRIES: usize>()
-> StringInterner<MemoryArena<BYTES, ENTRIES>> {
    StringInterner::new(MemoryArena::new()) // lint:allow(no-alloc) reason: StringInterner is the no-alloc interner wrapper, not std String; tracked: #72
}

/// Kit preset declaring a `Resource<StringInterner<MemoryArena<BYTES, ENTRIES>>>`
/// among its Owned types.
///
/// Declares the type-level commitment only. The default value is
/// supplied by the app at the call site via
/// `.with(Resource::new(default_interner::<BYTES, ENTRIES>()))`.
pub struct InternerKit<const BYTES: usize, const ENTRIES: usize>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array sizes; tracked: #121

impl<
    const BYTES: usize,
    const ENTRIES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array sizes; tracked: #121
> BuilderInput for InternerKit<BYTES, ENTRIES>
{
    type Dispatch = KitDispatch<Self>;
    type Init = Self;
}

impl<
    const BYTES: usize,
    const ENTRIES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array sizes; tracked: #121
> Kit for InternerKit<BYTES, ENTRIES>
{
    type Owned = Cons<Resource<StringInterner<MemoryArena<BYTES, ENTRIES>>>, Empty>;
    type Units = Empty;
}
