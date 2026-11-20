# Review pass 3: `work/hila-consumer-surface` at c8697c97

Third and last review pass over the branch, against `origin/dev` at merge base `ad6da29e`. The canon gate passed; the test gate failed on one count, which is finding 1 and is the reason the verdict is not clear. The rest of the review was done in full, since the failure is a missing test leg and not a defect in the bodies that exist. Pass 2 (`202611200500_review-pass-2-hila-consumer-surface.md`) stopped at the canon gate before op answered; this pass reads the branch after his answer and the zrf9 round that carries it.

## Verdict

MERGE AFTER FIXES. Findings 1 and 2 block. Findings 3 to 9 are cheap and belong in the same fix pass, since the branch has not landed and none of them costs another round of design.

## Canon gate

Aligned. Checked against op's verbatim answer in `.shared/tickets/closed/may_the_providers_crate_free_a_released_column_and_ship_a_no` (the quote in `mock/research/202611200520_default-impls-opt-in/OP-CALL.md:18` matches it word for word, options included), and the two ratified rows that record it, `memory-2.toml:239` and `:257`. `cargo mock canon check` passes both ratified rows and reports the three new `held` rows as held, which is their correct standing.

What op's words require and what the branch does:

| Op's words | On the branch |
|---|---|
| "Option a/b" (release frees, `allocate_zeroed` stays) | release row superseded; `storage.rs:285` gives the block back; `platform.rs:74` provided method |
| "make the default impls an opt in feature on their respective crates" | `static`, `portable` on providers; `os` on the engine |
| "Default to contracts only" | `default = []` in every crate with a `[features]` table; the four builds checked below compile |
| "The default impls we already have for other things should probably move there too" | `OsClock`, `OsMemoryProvider`, the os arms of parking and thread classes, every providers implementation |
| "the readmes should mention the different principled features" | root `README.md:176`, both crate `README.md.tmpl`, `WORKFLOW.md.tmpl` |

The two pass-2 grounds are answered by op's ruling and not by an agent's reading: the release row is superseded under a ratified ruling, and the providers in the family are what option a keeps, now behind a feature a consumer turns on. Two residues of agent judgement remain and are findings 4 and 5, not gate failures. This is the first read of the post-ruling state; a second independent read is owed before anyone cites this gate as settling what the canon permits.

## Test gate

Ran `CARGO_INCREMENTAL=0 cargo mock test` over everything: workspace 787 run, 786 passed, 1 failed (`hilavitkutin-build::refused`, the known cranelift red); the cross-target tool red is the known E0107 at `WriteSet`; bench crates 171 passed, 0 failed, which also shows every bench and support crate builds with the features its manifest names. Read the bodies of `tests/static_free_list_memory.rs`, the new tests of `tests/column_storage.rs`, `tests/interner_law.rs`, `hilavitkutin-str/tests/interner.rs` and `ergonomics.rs`, `hilavitkutin-persistence/tests/archive_str.rs` and the new test in `hilavitkutin/tests/platform_os.rs`. No tautology, no assertion-free test. The take-back laws are written as functions over any provider and each is shown failing on `StaticBumpMemory`, which is the right shape. The refusal boundary in the str tests is pinned on both sides (2^28 - 1 a handle, 2^28 a refusal).

The failure is configuration sampling, finding 1.

## Findings

1. `mock/crates/hilavitkutin/Cargo.toml:31`, `mock/crates/hilavitkutin-providers/Cargo.toml:20` [blocker, tests]: each crate dev-depends on itself with its features on, so every test of the engine runs with `os` and every test of providers with `static` and `portable`. The build op ratified as the one every consumer gets, default features, is tested by nothing, and the property his ruling states, that a default build carries the contracts alone, is asserted nowhere. Before the branch the default was the os tier and was tested; after it the default is the one configuration no test reaches. The self dev-dependency also makes the assertion inexpressible inside the crate's own tests, since a trybuild fixture compiles with the unified features. Owed: a check outside the crate's own test unit (a tool or lint under `mock/`, or an own-root crate as the cross-target probe is) that builds each crate per feature set and fails to name `StaticBumpMemory`, `ArenaColumnStorage` and `OsClock` without their feature, and one run of the engine suite without `os`, where parking spins and the builder starts at `NullClock`. My `cargo check` of the four sets (providers default, `static`, `portable`; engine default) passed with no warning, with a bad feature name as the failing control. That is an ad-hoc check by hand and stands in for none of this.

2. `mock/crates/hilavitkutin-providers/src/interner/arena.rs:21`, gated at `src/interner/mod.rs:14` [blocker, correctness]: the bridge `impl<A: ArenaInterner> InternerApi for StringInterner<A>` sits under `portable`. It rests on nothing; it is coherence glue between two contracts. On a contracts-only build a consumer with its own arena cannot write it, since neither the trait nor the type is theirs: E0117, shown in `202611200600_review-pass-3_probes/gated-interner-bridge/` (one spike instance, the shipped bridge compiling in the owning crate is its control). So "contracts only" leaves the str crate's interner unusable as the `HasInterner` provider without a newtype. The providers design never mentions the bridge or its feature, so its gating is also code the design does not say. Ungate it and add it to the surface table as a contract-side item.

3. `mock/design_rounds/202611200302-n8e0/202611200302-n8e0_changelist.src.lock.md:15` [defect, rules]: the locked changelist says ticket `the_memory_arena_s_entry_bound_is_one_short_of_the_28_bit_id_field` is closed. The registry file holds one move, to `backlog`, and `arena.rs:94` already reads `ENTRIES <= 1 << 28` (da0f6742). Add the move to done; a locked claim that does not match the tree is the defect.

4. `mock/registry/tenet/memory-2.toml:241` [drift, canon]: a ratified row's `what` says the providers "stay where they are". Op wrote "Option a/b" and "on their respective crates"; "stay where they are" is the agent's reconciliation of options a and b, written at ratified standing. I read it the same way, but the ratified field should hold what he said and the reading should be marked as the reading, as `mechanism` already half does.

5. `mock/design_rounds/202611200518-zrf9/202611200518-zrf9_topic.default-impls-behind-opt-in-features.md:25` [follow-up, canon]: `os`, `static` and `portable` are "a proposal of this round", posted to the coordinator. No question ticket to op names them. Cargo feature names are public surface and renaming one later breaks every consumer. File the stamp as a question ticket with `--holder op`, the three names and the alternatives considered.

6. `mock/crates/hilavitkutin-frozen-authors/Cargo.toml:12` [defect, drift]: the frozen crate's normal dependency names `os`, though nothing in its `src/` or `tests/` names an os item. That forces the os tier on every dependent of the library, against "default to contracts only", for parking and a clock its units do not read. The break was needed for the providers dev-dependency; the `os` was not. Drop it, or move it to `[dev-dependencies]`, and rewrite the one `BREAKS.toml` line and hash before landing. `mock/tools/cross-target/probe/Cargo.toml:1-5` still explains itself by "that crate takes the default tier and cargo would turn both on"; there is no second feature to turn on any more.

7. `mock/crates/hilavitkutin-api/BACKLOG.md.tmpl:77`, `mock/crates/hilavitkutin/BACKLOG.md.tmpl:19` [drift]: "Default no_os / std implementations are intended to live in the engine crate" now contradicts op's ruling (the no_os providers live in providers behind `static`) and the ratified row that there is no std tier; the engine backlog says no_os consumers ship their own, with no word of `static`. The ruling was applied to the gaps it closes and not to the standing claims it makes wrong.

8. `mock/crates/hilavitkutin/DESIGN.md.tmpl:1788` [drift]: "The engine's one other feature, `intrinsics-std`" is false; `mutant-truncating-append` is the third. Say "the other features" and why neither is an implementation.

9. `mock/design_rounds/202611200518-zrf9/202611200518-zrf9_changelist.doc.lock.md:27` [drift, rules]: the locked doc changelist puts `InternerApi` and `HasInterner` under `portable`; the design at `DESIGN.md.tmpl:35` and the code leave them ungated. The landed state is the right one; the locked claim disagrees with it.

10. `mock/crates/hilavitkutin-providers/src/memory/free_list.rs:153` [follow-up, unsafe]: the `node.write` block has no SAFETY comment of its own; the one at `:149` documents the `base.add(lead)` block above it.

11. `mock/benches/asm_gate_fixtures/asm_gate_report.x86_64.md:19`, `:89` [follow-up]: check 3 went 394 to 397 and 414 to 419 stack-relative accesses in d2deab88, a commit with no body. Report-only check, so not a gate, but an unexplained move in the one number that tracks inner-loop spills should be named or ticketed.

12. Commit subjects d96a2192 ("make MemoryArena deduplicate") and 1c899d37 ("run_parallel") [follow-up, rules]: identifiers not backticked. Pushed already; noted for the record.

13. Process [follow-up]: PR #4 (`recover/unlanded-rounds-and-benches` into `dev`, open since 2026-08-30) is still outside every review set. Pass 2 named it too. Thirty-six research sketch manifests depend on the engine on default features, and 68 sketch sources name `OsClock`, `OsMemoryProvider`, `StaticBumpMemory`, `ArenaColumnStorage` or `default_interner`; none of those builds as written any more. They are records, but nothing on the branch says so.

## Verified

- `StaticFreeListMemory`: node starts and lengths stay multiples of the grain through a split with pad and tail, so every piece can hold a node; merges join only touching free blocks, left and right; `used` adds and subtracts the same rounded length. The lock is a compare-exchange with Acquire and a Release store in the guard's drop, and every list access goes through `list()` under it. `used` is Relaxed and only a statistic.
- `StaticBumpMemory`: Relaxed compare-exchange on the cursor is enough, since no data is published through it; overflow of pad and length is checked.
- `ArenaColumnStorage`: the length overflow is checked before the give-back, the slot is emptied before `deallocate`, drop goes through the same path, so every block is freed once. The engine never calls `release`.
- `allocate_zeroed`: the provided method writes `len` zeros into a non-null block; the os override skips it on a fresh mapping, tested over three pages.
- Interner: `runtime_handle` refuses any id of 2^28 or more, a held string is found before the capacity check, and the `ENTRIES <= 2^28` bound keeps every issued id in the field.
- The host import and dump path in the engine is not in this diff; its content landed on `dev` as 9918113f and `scheduler/plan_handle.rs` is identical on both sides.
- Every dependent in the workspace, every bench and support crate, and the cross-target probe names the features it builds with; the only manifests naming none are the workspace table and the probe's wasm arm, which wants the default.
- `LEDGER.toml` and `BREAKS.toml` agree on the one break and its hash, and `frozen-ledger` is green in the lint run.
- `cargo mock --lint-only` exits 0, with no finding in a file the branch touches. No em-dash in the touched prose.
- All 122 commits read one at a time: no byline, no trailer, no tool advert, no parentheses or trailing period in any subject; the six merge commits carry git's default message.

## Notes

The bodies of the new tests are good and the allocator is correct as far as reading it goes. What fails is the coverage of the configuration the ruling made the default, and one gating decision that makes that configuration less usable than the ruling meant. Both are additive fixes. The superseded rows losing `stamped_by` and `stamp` follows the precedent on `dev` (252c5190) and is not a finding.
