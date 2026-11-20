# source changelist

Round topic `202611200637-jibi_topic.default-build-tested-and-bridge-ungated.md`, doc changelist `202611200637-jibi_changelist.doc.lock.md`.

## Changes

### 1. The bridge leaves `portable`

`crates/hilavitkutin-providers/src/interner/bridge.rs`, new and compiled on every build: the impl of `InternerApi` for `StringInterner<A>` over any `ArenaInterner + 'static`, moved whole out of `src/interner/arena.rs`, which keeps `MemoryArena`, `default_interner` and `InternerKit` under `portable`. `src/interner/mod.rs` declares the module without a feature and its module doc says so.

### 2. The providers suite runs on its default build

`crates/hilavitkutin-providers/Cargo.toml`: the dev-dependency on the crate itself goes. Each test binary that names an implementation carries `required-features` in a `[[test]]` table (`column_storage` on `static` and `portable`, `interner_law` and `smoke` on `portable`, `static_bump_memory` and `static_free_list_memory` on `static`), and each example in an `[[example]]` table on `portable`. The engine stays a dev-dependency with `os`, which the examples run on.

`tests/ui.rs` runs the arena's entry-bound refusal when `portable` is on, and two new fixtures under `tests/ui/default/` where their feature is off: `memory_arena_unnamed_without_portable_fail.rs` and `static_bump_memory_unnamed_without_static_fail.rs`, each with its `.stderr`.

`tests/bridge.rs`, new and with no required feature: an `ArenaInterner` of the test's own, wrapped in `StringInterner`, interns and resolves through `InternerApi` and is refused once its one slot is taken.

### 3. The engine suite runs on its default build

`crates/hilavitkutin/Cargo.toml`: the dev-dependency on the engine itself with `os` goes. `tests/ui.rs` gains the twin `os_clock_unnamed_without_os_pass.rs`, naming `NullClock`, on every build, and `os_clock_unnamed_without_os_fail.rs`, naming `OsClock`, when `os` is off, with its `.stderr`.

### 4. The frozen crate takes the engine's default build

`crates/hilavitkutin-frozen-authors/Cargo.toml`: the engine dependency drops `features = ["os"]`, which nothing in the crate uses, and the dev-dependency on `hilavitkutin-providers` turns on `static` beside `portable`, since `tests/common/mod.rs` names `StaticBumpMemory` and had that feature only through the providers crate naming itself. `BREAKS.toml` gains the line for it and `LEDGER.toml` the new hash and break count.

### 5. Comments

`tools/cross-target/probe/Cargo.toml`: the header says why the probe is a root of its own as things stand, crates of the workspace naming the engine with `os` on. `crates/hilavitkutin-providers/src/memory/free_list.rs`: the first node's write carries its own SAFETY comment.

`mock/Cargo.lock` follows the manifests.
