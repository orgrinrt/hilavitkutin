# doc changelist

Round topic `202611200637-jibi_topic.default-build-tested-and-bridge-ungated.md`, the only flat topic in `design_rounds/`.

## Changes proposed

### 1. The interner bridge is a contract adapter, not an implementation

`mock/crates/hilavitkutin-providers/DESIGN.md.tmpl`, "Features": the paragraph naming the two contracts that are not behind a feature also names the bridge `impl InternerApi for StringInterner<A>`. It rests on nothing a feature gates, since `hilavitkutin-str` is an unconditional dependency, and only this crate may write it, since a consumer owns neither the trait nor the type. So a consumer with its own `ArenaInterner` reaches its interner through `InternerApi` on a default build.

The same file, "Surface": a row for the bridge, feature none.

The locked doc changelist of round 202611200518-zrf9, section 3, put `InternerApi` and `HasInterner` under `portable`; the design and code that landed leave them ungated, which is what holds. That round is closed, so its changelist stays as it is and this section records the difference.

### 2. Each crate's suite runs on its default build

`mock/crates/hilavitkutin-providers/DESIGN.md.tmpl`, a short section "Testing" after "Surface": the crate does not name itself as a dev-dependency. `cargo mock test -p hilavitkutin-providers` runs its suite on the default build, where only the contracts and the bridge exist; a test binary that needs an implementation names its feature as `required-features`, and `--features static,portable` runs those. A compile-fail fixture holds that a default build names neither `StaticBumpMemory` nor `MemoryArena`, and a pass test runs a consumer's own `ArenaInterner` through `InternerApi`.

`mock/crates/hilavitkutin/DESIGN.md.tmpl`, "Platform implementations", after the paragraph naming the features: the same for the engine. Its os tests carry `cfg(feature = "os")`, `cargo mock test -p hilavitkutin` runs the suite with the os tier off, and `--features os` with it on. A compile-fail twin holds that a default build cannot name `OsClock`; its pass twin names `NullClock`.

### 3. The engine names both its other features

`mock/crates/hilavitkutin/DESIGN.md.tmpl`, "Platform implementations", the sentence "The engine's one other feature, `intrinsics-std`, is not an implementation of a contract": the engine has two other features, `intrinsics-std` and the test-only `mutant-truncating-append`, and neither is an implementation of a contract.

### 4. The backlogs stop saying what the ruling made wrong

`mock/crates/hilavitkutin-api/BACKLOG.md.tmpl`, "Runtime-replaceable platform defaults": the default implementations sit behind features on the crates that carry them, the engine's `os` and the providers' `static` and `portable`, and a build on default features carries none; overriding through the scheduler builder is unchanged.

`mock/crates/hilavitkutin/BACKLOG.md.tmpl`, "no_os tier documentation": a consumer without an operating system can turn on `static` on `hilavitkutin-providers` for a memory provider over its own buffer; the deferred item is the worked embedded example.

## Not changed

The canon: no row changes. The feature names `os`, `static` and `portable`, and the reading of "a/b" in the row `op_ruled_that_a_released_column_gives_its_block_back_and_the_providers_over_a_consumer_buffer_stay_in_the_providers_crate`, are put to op as question tickets. The locked source changelist of round 202611200302-n8e0 says the ticket `the_memory_arena_s_entry_bound_is_one_short_of_the_28_bit_id_field` is closed while the registry has it in backlog; the round is closed and the ticket's state is the coordinator's to move.
