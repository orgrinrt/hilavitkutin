# doc changelist

Round topic `202611200201-yszg_topic.memory-the-consumer-takes-back.md`.

## Changes proposed

### 1. A zeroed allocation on the memory provider

`mock/crates/hilavitkutin-api/DESIGN.md.tmpl`, section "The memory provider is an unsafe trait" and the listing under "Platform traits": `MemoryProviderApi` gains `unsafe fn allocate_zeroed(&self, len: USize, align: USize) -> *mut u8`, a provided method whose default calls `allocate` and writes `len` zero bytes into a non-null block. A fifth promise: a non-null block from `allocate_zeroed` reads as `len` zero bytes. A provider whose fresh blocks are already zero overrides it and skips the write; one that reuses blocks keeps the default.

`mock/crates/hilavitkutin/DESIGN.md.tmpl`, "Platform implementations": the os provider's `allocate_zeroed` is its `allocate`, since a fresh anonymous mapping reads as zero. The null provider keeps the default, which writes nothing into a null block.

### 2. Release gives the block back, and a re-reserve says what it keeps

`mock/crates/hilavitkutin-api/DESIGN.md.tmpl`, the `ColumnStorage` listing: re-reserving an id gives its block back and takes a new one, and carries no record over. `release(id)` may give the column's block back to the provider; after it every pointer the column handed out is dead and the id reads as unreserved, with a count of zero, until it is reserved again. Whether a re-reserve for more keeps the first records waits on the question `when_a_column_s_capacity_is_reserved_and_whether_a_live_coun`.

`mock/crates/hilavitkutin-providers/DESIGN.md.tmpl`, `ArenaColumnStorage`: `release` deallocates the slot's block through the provider with the 64-byte alignment it was reserved at and empties the slot. A release of an unreserved id or one past the cap does nothing. The text that called it a no-op goes.

### 3. A memory provider over a buffer that takes blocks back

`mock/crates/hilavitkutin-providers/DESIGN.md.tmpl`: a section `StaticFreeListMemory` and a row in the surface table. It is a provider over a buffer the consumer hands it, with the same two constructors and the same promise about the buffer as `StaticBumpMemory`; the two share one private buffer type that carries the constructors and the reasoning once. Free space is an address-ordered list of free blocks kept inside the free bytes themselves, so the provider holds no table. Every block's start and length are multiples of a grain, the size of a free-list node (two machine words), so any piece a split leaves can hold a node. `allocate(len, align)` takes the first free block that fits `len` rounded up to the grain at `align` (at least the grain), and returns the space before and after the block to the list. `deallocate` puts the block back in address order and merges it with a free neighbour on either side. The list is guarded by a one-flag spin lock, tracked as ticket `the_stack_has_no_lock_so_a_provider_keeps_a_spin_flag_of_its_own`. `used()` reports the bytes handed out and not given back, padding to the grain included; `capacity()` the buffer's length. `allocate_zeroed` keeps the default, since a block given back and handed out again is not zero. `StaticBumpMemory`'s text says it is the provider for a consumer that never gives memory back, and points at `StaticFreeListMemory` for one that does. `mock/crates/hilavitkutin-providers/README.md.tmpl` names it.

### 4. A full interner refuses, and the caller sees it

`mock/crates/hilavitkutin-sym/DESIGN.md.tmpl`, "`Interner`, the dedup-and-resolve producer": `intern` returns `Maybe<Sym<D::Shape>>`, `Maybe::Isnt` when the producer has no room left for a value it does not hold, the shape `Generator::mint` already gives its exhaustion. A value already held is never refused.

`mock/crates/hilavitkutin-str/DESIGN.md.tmpl`: `StringInterner::intern` and `intern_static` return `Maybe<Str>`. The `ArenaInterner` contract says an id of 2^28 or more is the arena's refusal, which `MemoryArena`'s sentinel is; `StringInterner` returns `Maybe::Isnt` for it and never builds a handle from it. `IntoStr::into_str` returns the same, a `Str` passing through as `Maybe::Is`. The list item naming the interner's contract reads `intern(&str) -> Maybe<Str>`.

`mock/crates/hilavitkutin-providers/DESIGN.md.tmpl`: `InternerApi::intern` returns the same. The `MemoryArena` paragraph on capacity overflow says the sentinel reaches `StringInterner` and is refused there, so no handle carries it; the build check on `ENTRIES` against 2^28 stays, since an issued id must stay below the refusal.

`mock/crates/hilavitkutin-persistence/DESIGN.md.tmpl`: inject's intern can be refused, and `inject_str` answers a refusal with `PersistenceError::Full`, a new case: the interner had no room for the string.

## Not changed

The canon. Capacity stays a const generic on `MemoryArena`, and a re-reserve still keeps no record; both wait on the maintainer's answers to `whether_an_interner_s_capacity_may_be_set_when_it_is_built_a` and `when_a_column_s_capacity_is_reserved_and_whether_a_live_coun`. Per-record mark propagation is its own round.

### 5. The count of promises

`mock/crates/hilavitkutin-api/DESIGN.md.tmpl`, "Platform traits", and `mock/crates/hilavitkutin/DESIGN.md.tmpl`, "Platform implementations": the two places that count the provider's promises say five.
