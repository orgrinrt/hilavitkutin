## Verdict

**MERGE AFTER FIXES.** Two things block the merge. First, the published surface was not updated: the root README and every generated document still describe the old memory provider. Second, the round 1 differential is meant to judge every later job, and it covers far less than it claims.

**Test gate.** All 483 tests passed, 19 ignored, 0 failed, across `hilavitkutin`, `-api`, `-providers` and `-persistence` in the worktree. The suite still fails the gate on items 4, 6 and 7 below: a sampled matrix, a test that asserts nothing, and an alignment property that is never asserted.

**Canon gate.** Rounds 1 and 3 are aligned with the ratified rows they cite. I checked `a_unit_whose_inputs_are_all_clean_skips_the_pass`, `execute_returns_nothing_and_a_failing_unit_aborts_…` and `the_engine_is_one_…` with `cargo mock canon show`. Round 2's placement of the new provider is ambiguous and goes back to you (item 3). I am the first read on that; a second is owed.

## Findings

1. **Stale public surface, a blocker.**
   - `README.md:173` still says providers ship only `InternerKit` and "future modules add default ColumnStorage, Clock, and MemoryProvider".
   - `docs/` was not regenerated after the three design rounds. Its last change is 8236de3c, before the branch.
   - `docs/API.md:1564` still shows a safe trait, and `docs/PROVIDERS.md:22` still lists the memory default as future. Nothing under `docs/` names `StaticBumpMemory` or the unsafe trait.
   - Fix: run `cargo mock` and rewrite README:173.

2. **The api design contradicts itself** (`mock/crates/hilavitkutin-api/DESIGN.md.tmpl:1589`, spec). The trait's own section, "Platform traits (domain 02)", still declares `pub trait MemoryProvider: Send + Sync`. The new subsection at `:855` declares `pub unsafe trait MemoryProviderApi`. The round updated one statement of the trait and left the other. Fix `:1589` too, then grep every `*.md.tmpl` for any other statement of the trait.

3. **Canon is ambiguous on where `StaticBumpMemory` lives** (`mock/design_rounds/202611140513/202611140512_topic.sound-memory-provider.md:39`, canon). Handed back to you.
   - The provisional row `clock_and_libc_unix_memory_provider_defaults_land_in_their_own_rounds_when_consumer_demand_surfaces` says the crate's scope "at this stage is the interner and the arena column storage backing". It also says a libc/Unix memory default lands "when consumer demand surfaces".
   - The topic decides on its own reading: "Three test trees asking is that demand". On that reading it adds a static bump provider, not a libc/Unix one, to a published crate in order to serve the engine's own tests.
   - The held row binds nothing.
   - An alternative needs no canon call: a `publish = false` test-support crate as a dev-dependency. That would serve the ui fixtures and the persistence tests equally well.

4. **The worker-count matrix is effectively two counts, and two of the three worker paths are never run** (`tests/run_parallel_skip_differential.rs:12-14`, `tests/skip_chain/mod.rs:61`, tests).
   - Same-phase units that share a written column form one trunk (`src/plan/grouping.rs:184-211`). So the four-unit fixture has at most two trunk roots in any phase.
   - Ownership is `rank % ncores == core` (`src/dispatch/trunk_dispatch.rs:246-247`). With two roots, workers 2, 3, 4 and `MAX_CORES` place every unit identically.
   - The doc comment's claim that "an odd count and the widest one place the units differently" is therefore false for this fixture.
   - `RECORDS = 4` is a single morsel, so only the first window is ever walked.
   - The fixture has records and no accumulator, so neither the record-less path (`src/scheduler/worker.rs:427-448`) nor the accumulator carrier path (`worker.rs:124`) is entered.
   - The catalogue row and both ignore strings require the mask to be "gated on by both worker paths". Job 5 can therefore turn this green with the record-less path still ungated.
   - `PLAN.md:107` says the differential asserts accumulator contents. Nothing here does.
   - Fix: add a fixture with at least 5 in-phase roots, more records than one window, a record-less carrier and an accumulator carrier.

5. **"Which units ran" is a yes/no per unit** (`tests/skip_chain/mod.rs:143-146`, tests). A unit dispatched twice at two or more workers is invisible, because its idempotent writes leave the bytes unchanged. The counts already exist; assert the exact executions per frame.

6. **Alignment is never asserted under contention** (`crates/hilavitkutin-providers/tests/static_bump_memory.rs:129-131`, tests).
   - The concurrency test checks only that blocks do not overlap. The buffer is 64-aligned, and nothing checks alignment or bounds on the threaded path.
   - Suppose the padding computation (`src/memory.rs:224`) were moved above the retry loop. A retried block would then be misaligned but still disjoint, and all five tests would pass.
   - Fix: assert `addr % align == 0` and the bounds for every block in the threaded test, on a skewed buffer.

7. **A test that asserts nothing passes in every run** (`tests/unit_panic_aborts.rs:164-188`, tests). `child_runs_the_panicking_frame` reports `ok` in the parent run and asserts nothing.
   - Fix: mark it `#[ignore]` and start the child with `--ignored`.
   - Also, `:195-196` discards the child's stderr, so an abort from any other cause also passes. Assert that the stderr contains the unit's panic message and "cannot unwind".

8. **Nothing tests the `extern "C"` on `worker_main`** (`src/scheduler/worker.rs:65`, tests). `unit-panic-worker-entry-only.txt` covers only the other direction. With the boundary in `wu_fn` in place, removing `extern "C"` from `worker_main` leaves the suite green. Write a test that panics in the worker's own code, or catalogue the gap.

9. **The bench providers break the provider's own safety argument** (`mock/benches/engine_vs_std/src/lib.rs:141-145`, the same in three more bench crates, unsafe).
   - Each builds `StaticBumpMemory` over a `&'static mut` made from a `Box` that its `Drop` frees.
   - `src/memory.rs:209-211` argues safety from "borrowed for `'static` and never released".
   - The bench comment "borrowed only for the call" contradicts the `'static` signature.
   - As written this is not UB: the reference dies inside `new`, and nothing touches a block after the provider drops. But the contract now rests on a lifetime the caller made up. Either the design adds an `unsafe` constructor over a pointer and length with a stated contract, or the benches stop doing this.

10. **A breaking change is committed as a plain `fix:`** (30888615, rules). Making a public trait `unsafe` breaks every downstream implementation. The repo's style uses `!` for breaking changes. Every other subject line passes: correct format, under 72 characters, no byline, no trailer, no em-dash.

11. **A new libc call has only the feature gate** (`src/platform/os.rs:51-52`, drift). The added `sysconf` sits behind `feature = "platform-os"` alone (`platform/mod.rs:14`), with no target gate, against `gate-per-target-and-per-feature`. The gap existed before; this branch extends it.

12. **An unmeasured cost claim in a doc comment** (`src/dispatch/wu_fn.rs` doc on `execute_without_unwinding`, drift). "Costs nothing… one cold landing pad per inlined call" is a codegen claim with no assembly or bench committed. Commit the evidence or drop the claim.

Follow-up, not in this set: the overlap with PR #4 is the coordinator's call, ticket `hilavitkutin_pr_4_recover_branch_overlaps_the_engine_path_an` (ff15f8cbbdae).

## Verified

- Round 1's two ignored tests fail at all five worker counts with `--include-ignored`, as the changelist says. The byte comparison and its broken-runner control pass.
- The broken-runner control's expected result (frame 3, `Cd`, record 0) holds whether or not job 5 lands.
- `StaticBumpMemory` aligns by address, checks overflow, and treats align 0 and non-powers of two as refusals. Relaxed ordering on the cursor is enough, since no data is published through it. The `Send`/`Sync` arguments hold.
- `OsMemoryProvider` correctly refuses alignments wider than a page; a zero length maps to `MAP_FAILED` and returns null.
- Every `MemoryProviderApi` impl outside the sketches is `unsafe impl` with a safety comment. The E0200 compile-fail fixture works. The Miri evidence files are committed.
- `wu_fn.rs:65` is the only call of `execute` in the src trees I searched (`hilavitkutin`, `-api`, `-kit`, `-providers`, `-conformance`). The fused chain reaches `apply` through the chain unit's own `execute`, so the design's claim of "the one shim" holds.
- The run-path evidence shows the timeout firing at 30 s before the change and SIGABRT after it.
- The provisional stamp on the no-unwind boundary is cited as provisional everywhere; nothing calls it ratified.
- No exemption lists name any touched crate.
