# src changelist

Round topic `202611200201-yszg_topic.memory-the-consumer-takes-back.md`, doc changelist `202611200201-yszg_changelist.doc.lock.md`.

## Changes proposed

### 1. The zeroed allocation

`crates/hilavitkutin-api/src/platform.rs`: `MemoryProviderApi::allocate_zeroed`, a provided method calling `allocate` and writing `len` zero bytes into a non-null block, with its `# Safety` section; the trait's doc lists the fifth promise. `crates/hilavitkutin/src/platform/os.rs`: `OsMemoryProvider` overrides it with `allocate`, its safety comment naming the zeroed mapping.

Tests: `crates/hilavitkutin/tests/platform_os.rs` gains a test that a block from `allocate_zeroed` reads as zero. `crates/hilavitkutin-providers/tests/static_free_list_memory.rs` (below) holds the default's test: a block written, given back and taken again through `allocate_zeroed` reads as zero, and the same block through `allocate` does not, so the test sees the write.

### 2. Release and re-reserve

`crates/hilavitkutin-api/src/storage.rs`: the doc comments of `reserve` and `release` say what the design says. `crates/hilavitkutin-providers/src/storage.rs`: `release` deallocates a live slot's block at `CACHE_LINE_ALIGN` and sets the slot to `Maybe::Isnt`; an id past the cap or unreserved does nothing. The module doc stops calling release a no-op.

Tests, `crates/hilavitkutin-providers/tests/column_storage.rs`: a released column reads count zero and null pointers; under `StaticFreeListMemory` a release gives the bytes back (`used()` returns to what it was before the reserve); a store over a buffer that holds two columns' worth reserves, releases and reserves again many times without refusal, which is ticket `arenacolumnstorage_release_is_a_no_op_so_a_store_under_a_bum`'s case; a release of an unreserved id and of one past the cap changes nothing.

### 3. `StaticFreeListMemory`

`crates/hilavitkutin-providers/src/memory.rs` becomes a module directory: `memory/mod.rs` holds the private buffer type both providers keep (base and length, the two constructors' safety reasoning once), `memory/bump.rs` holds `StaticBumpMemory` over it unchanged in behaviour, and `memory/free_list.rs` holds `StaticFreeListMemory`: the grain, the free-list node, the one-flag spin lock with its FIXME naming ticket `the_stack_has_no_lock_so_a_provider_keeps_a_spin_flag_of_its_own`, first-fit `allocate`, merging `deallocate`, `used`, `capacity`, and the `unsafe impl` with a safety comment per promise. `crates/hilavitkutin-providers/src/lib.rs` exports it.

Tests, `crates/hilavitkutin-providers/tests/static_free_list_memory.rs`: blocks are aligned and inside the buffer; live blocks do not overlap; a block given back is handed out again; three neighbouring blocks given back in each order merge into one that takes their whole length; a zero length takes a grain; an `align` that is not a power of two and a length past the free space return null and change nothing; `used()` counts grain padding and returns to zero when everything is given back; several threads allocating and freeing at once get disjoint blocks and leave `used()` at zero. Each test that a merge or reuse holds is shown failing against a provider that does not do it, by a negative arm in the same file over a provider written to skip it.

### 4. The intern refusal

`crates/hilavitkutin-sym/src/interner.rs`: `Interner::intern` returns `Maybe<Sym<D::Shape>>`, the doc saying `Maybe::Isnt` is no room. `crates/hilavitkutin-str/src/interner.rs`: the `ArenaInterner` doc says an id of 2^28 or more is the refusal; `intern` and `intern_static` return `Maybe<Str>`, `Maybe::Isnt` for such an id, and build no handle from it; the `Interner<StrDomain>` impl maps through. `crates/hilavitkutin-str/src/ergonomics.rs`: `IntoStr::into_str` returns `Maybe<Str>`. `crates/hilavitkutin-providers/src/interner.rs`: `InternerApi::intern` returns `Maybe<Str>`, and `MemoryArena`'s comments stop calling the sentinel masked. `crates/hilavitkutin-persistence/src/error.rs` gains `PersistenceError::Full`, and `archive_str.rs`'s `inject_str` answers a refusal with it.

Tests: every caller in the crates' tests and examples takes the `Maybe`. `crates/hilavitkutin-providers/tests/interner_law.rs`: the overflow tests say the overflowing string is refused and the strings held before still resolve and still intern to their handles; `two_overflowing_strings_get_distinct_handles` becomes `two_overflowing_strings_are_both_refused` and runs; `an_overflow_handle_resolves_to_nothing` is deleted, since no overflow handle can be built any more; `an_unissued_id_does_not_alias_a_held_empty_string` keeps its first half and stays ignored on its ticket, its second half, which built the masked id by hand, going. `crates/hilavitkutin-str/tests/interner.rs`: a full arena refuses a new string through `intern`, `intern_static`, the sym trait and `into_str`, and a const string is never refused. `crates/hilavitkutin-persistence/tests/archive_str.rs`: injecting a string into a full interner is `PersistenceError::Full`.

## Not changed

`MemoryArena`'s sentinel and its const capacities; `reserve`'s keeping no record on a re-reserve; the engine's dispatch.
