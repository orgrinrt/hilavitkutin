---
round: 202605041257
phase: TOPIC
status: frozen
---

# Topic: hilavitkutin-providers v0 (InternerKit)

## Frame

Substrate prereq for viola becoming a hilavitkutin app (#254) and
for the broader workspace move where consumer crates stop rolling
their own arena, allocator, or storage. Per task #253 and
`viola/docs/HILAVITKUTIN-APP-SHAPE.md` line 29 (the StringInterner
row).

The provider crate name is `hilavitkutin-providers` and ships
sensible default Resource-backed providers consumers can wire onto
the SchedulerBuilder via `add_kit`. v0 ships the interner only.
Future providers (ColumnStorage, MemoryProvider default, ClockProvider)
land in their own rounds when actual consumer demand surfaces.

The Kit machinery from #255 plus the typed-const Depth shape from
#327 are both already merged on hilavitkutin dev. This round
consumes them. There are no remaining hilavitkutin-side prereqs.

## Surface

The crate ships one module: `interner`.

### `InternerApi` trait

A provider-facing operations surface paralleling the platform
contracts already in hilavitkutin-api (`MemoryProviderApi`,
`ClockApi`, `ThreadPoolApi`). Methods:

- `fn intern(&self, s: &str) -> Str`
- `fn resolve(&self, s: Str) -> Maybe<&str>`

The trait carries `Send + Sync + 'static` so the value can live in
a `Resource<T>`.

### `HasInterner` accessor

Mirrors `HasMemoryProvider` / `HasClock` / `HasThreadPool`:

```rust
pub trait HasInterner {
    type Provider: InternerApi;
    fn interner(&self) -> &Self::Provider;
}
```

Lives in the providers crate, not in hilavitkutin-api. The platform
contracts in api are the engine-required surfaces; the interner is
an opt-in provider, not an engine prerequisite.

### `MemoryArena<const CAP: USize>`

Default implementation of `hilavitkutin_str::ArenaInterner`. Inline
storage at the type level: `[u8; CAP]` byte buffer plus a fixed
entry table with offset and length for each interned string.
Interior mutability via `core::cell::Cell` for the cursor and entry
count.

The `M: HasMemoryProvider` parameter from the original 2026-05-01
task description is dropped in v0. Inline storage at the type
level is what consumers can actually wire today; pulling from a
runtime memory provider would require allocating-at-builder-time
plumbing the substrate does not yet have. v1 may reintroduce the M
parameter as `MemoryArena<M, CAP>` if a real consumer needs runtime
slab allocation. Tracked in BACKLOG, not deferred to next round
without trigger.

### `InternerKit<const CAP: USize, const ENTRIES: USize>`

A `Kit<B>` impl that registers `Resource<StringInterner<MemoryArena<CAP, ENTRIES>>>`
on the builder via `B::resource(...)`.

The Kit's `install` body is one line: `builder.resource(StringInterner::new(MemoryArena::new()))`.

Two const generics because the arena's byte capacity (CAP) and the
maximum number of distinct runtime-interned strings (ENTRIES) are
independent dimensions a consumer tunes.

## Decisions

### Decision 1: crate name + dependency direction

Name is `hilavitkutin-providers`. Crate deps:

- `hilavitkutin-api` (`HasMemoryProvider`, `MemoryProviderApi`, the
  platform-contract patterns this crate parallels).
- `hilavitkutin-str` (`Str`, `StringInterner`, `ArenaInterner`).
- `hilavitkutin-kit` (`Kit<B>` trait).
- `arvo`, `arvo-bits` (USize, Bool, Bits substrate types).
- `notko` (Maybe).

Forbidden imports: `hilavitkutin::*`, the engine crate. Per the
hilavitkutin-workunit-mental-model rule and the existing
forbidden-imports policy on every ecosystem crate. Providers ship
the Kit impl that the engine consumes, not the other way around.

### Decision 2: module layout is interner-only

`src/lib.rs` declares `pub mod interner` and re-exports the four
public types (`InternerApi`, `HasInterner`, `MemoryArena`,
`InternerKit`). Future providers add modules; the lib.rs
re-export list grows accordingly.

### Decision 3: id encoding inside MemoryArena

Each runtime-interned string gets a 28-bit id from
`StringInterner`'s arena boundary. The arena's id space is the
sequence number of the entry: `id = next_entry_index`. The arena
holds an entry table `[Entry; ENTRIES]` where `Entry { offset:
USize, len: USize }`. `arena_resolve(id)` looks up the entry and
slices `[offset..offset+len]` from the byte buffer.

### Decision 4: interior mutability

The arena exposes `&self` mutators (per the `ArenaInterner`
contract). Interior mutability uses `core::cell::Cell<USize>` for
the cursor and the entry count, plus `core::cell::UnsafeCell<[u8;
CAP]>` for the byte buffer and `core::cell::UnsafeCell<[Entry;
ENTRIES]>` for the entry table.

`UnsafeCell` access is gated by safe wrapper methods that uphold
the no-overlapping-borrows invariant (each mutator finishes
writing before any subsequent read). The arena is `!Sync` by
default; the eventual `Resource<StringInterner<MemoryArena<...>>>`
will need either a Mutex shim from the consumer or a future
SyncArena variant. v0 documents the !Sync property; consumers
running multi-thread will be flagged by the type system at the
HasInterner accessor's `Send + Sync + 'static` bound when wiring
fails.

This is a real v0 limitation. Captured in BACKLOG as "Sync arena
variant" so future rounds (when viola's multi-thread path
exercises it) have a tracked migration target.

### Decision 5: capacity overflow policy

If `CAP` runs out of bytes or `ENTRIES` runs out of slots, the
arena returns a sentinel id: `u32::MAX` from `arena_intern`. The
`StringInterner` already returns `Maybe::Isnt` from `resolve` for
ids the arena cannot resolve, so the failure path is observable.
Consumers tune `CAP` and `ENTRIES` to their workload.

A panicking-on-overflow variant is not appropriate for substrate
code (`no_std`, `no_alloc`, `no panic` ideology); the typed
sentinel through `Maybe::Isnt` is the right shape.

### Decision 6: providers crate inherits standard ecosystem rules

`#![no_std]`, no `alloc`, no runtime spawn, no dyn, no TypeId.
Forbidden imports per the hilavitkutin-* family; the new crate gets
its own entry in the ecosystem lint config so the engine can't
sneak in.

## Sketches

None. The four surfaces are direct compositions of existing
substrate (`Kit<B>`, `Resource<T>`, `ArenaInterner`,
`StringInterner<A>`, `Cell`, `UnsafeCell`). No trait-solver-cycle
risk, no generic-const-expr risk, no repr(transparent) layout
risk. The Cell-and-UnsafeCell pattern is well-understood
no_std-safe interior mutability.

## Cross-references

- `viola/docs/HILAVITKUTIN-APP-SHAPE.md`: the consumer doc that
  names the providers crate as a #254 prereq.
- `mock/crates/hilavitkutin-kit/src/lib.rs`: the Kit<B> trait
  declaration this crate consumes.
- `mock/crates/hilavitkutin-str/src/interner.rs`: the
  StringInterner<A> + ArenaInterner contract this crate's
  MemoryArena implements.
- `mock/crates/hilavitkutin-api/src/platform.rs`: the
  HasMemoryProvider / MemoryProviderApi pattern HasInterner /
  InternerApi parallels.
- `.claude/rules/use-the-stack-not-reinvent.md`: the workspace rule
  that forbids consumers rolling their own arena.
- `.claude/rules/hilavitkutin-workunit-mental-model.md`: the rule
  that puts state into Resources/Columns and forbids global refs.
