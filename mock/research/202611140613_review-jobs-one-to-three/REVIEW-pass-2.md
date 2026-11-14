# Review pass 2: jobs 1 to 3, `work/hv-engine-path` at 7315943a

**Canon gate.** Rounds 1 and 3 still align with the ratified rows they cite. Finding 3 below is with op through question 82085a68780d. One new problem in the fix round's tests contradicts two ratified rows: `ordering_is_guaranteed_by_construction` and `a_fiber_is_commutative_only_if_all_its_units_are_…`. I checked both just now with `cargo mock canon show`. That is finding N1.

**Test gate.** I ran `run_parallel_wide_differential` with `--include-ignored`: 9 passed, 5 failed. The five that fail are the five committed ignored, and each fails as the src changelist says. `unit_panic_aborts`, `run_parallel_skip_differential` and `static_bump_memory` pass. The gate still fails on two new items: one test that cannot fail on what it claims (N2), and one count model tuned until it matched the code (N1).

## Verdict

**MERGE AFTER FIXES.** Both pass-1 blockers are fixed: the public surface and the api design's second statement of the trait. The new wide differential is a real step up, but it brings one canon-contradicting expectation and one test that cannot fail on what it claims. Both are confined to test files.

## Pass-1 findings, one by one

1. **Public surface: fixed.** `README.md:173` is rewritten, and `docs/` was regenerated at 06:59 (36dd78ca). Nothing under `docs/` or any `*.md.tmpl` still states a safe `MemoryProvider` trait.
2. **Api design restating the trait: fixed.** `hilavitkutin-api/DESIGN.md.tmpl:1586-1609` now states all three traits as `src/platform.rs` declares them. I checked `spawn` and `worker_count` at `platform.rs:86-102` and `now_ns` at `:191-193`.
3. **Where `StaticBumpMemory` lives: a ticket is the right disposition.** The canon call is op's, and question 82085a68780d carries the other option. While it is open, the branch has advertised the type in the root README and widened its API with `from_raw_parts`. That raises the cost if op answers "a test-support crate". Acceptable only because the placement is explicitly held.
4. **Sampled worker-count matrix: mostly fixed, with new defects N1, N2 and N4.** Six roots now give distinct ownership at 2, 3, 4 and `MAX_CORES`. `the_windowed_build_walks_several_windows_over_six_roots` asserts both six trunks and several windows. All three walks are entered, and the accumulator contents are judged against the model.
5. **Exact counts: fixed in the wide fixture.** The swap frames (`replace_value`, `replace_resource`) are still judged only by the yes/no executed set on the two-root fixture (`tests/skip_chain/history.rs:22-34`), since the wide history has no swaps. That is low severity, but it is the one place the old weakness survives.
6. **Alignment under contention: fixed.** The threaded test asserts alignment and bounds on a skewed buffer (`static_bump_memory.rs:136-142`). The hoisted-padding mutant is caught. That control is text only, see N3.
7. **Panic child: fixed.** No test is a child-only entry any more. The parent asserts SIGABRT, then the unit's message, then the refusal to unwind (`unit_panic_aborts.rs:238-250`). The abort mutant is caught. That control is also text only, see N3.
8. **`extern "C"` on `worker_main` untested: the ticket is acceptable.** No unit body can reach that boundary, and no test can reach the worker's own code without a fault hook the design does not have. 7285fd5aaa24 should say a hook is the likely route.
9. **Bench providers forging `'static`: fixed.** `from_raw_parts` (`memory.rs:95-113`) states the caller's contract, and the four benches drop the provider before freeing the block. Two pieces were missed, see N5.
10. **Breaking change under `fix:`: carried to the landing message.** Acceptable, since history is not rewritten.
11. **`sysconf` behind the feature gate only: the ticket is acceptable.** The gap predates the branch. The branch added one call under it.
12. **Unmeasured cost claim: fixed**, dropped in `wu_fn.rs`.

## New findings

N1. **The unit-outer count model expects the engine to split non-commutative fibers across cores** (`mock/crates/hilavitkutin/tests/run_parallel_wide_differential.rs:256-262` with `:100-109`; canon, tests; new). The model is `parallel: |base, n| if base > 0 { tiled_cores(RECORDS, n) } else { 0 }`: every unit runs once on each core whose slice is not empty.
   - No unit in `skip_wide` sets `COMMUTATIVE`, which defaults to false (`hilavitkutin-api/src/work_unit.rs:89`).
   - So the expected value is that non-commutative fibers are split across N cores. The ratified `ordering_is_guaranteed_by_construction` says "a non-commutative fiber runs on a single core". The ratified commutativity row gives multi-core morsel dispatch only to commutative fibers.
   - `PLAN.md:115-119` asks job 9 to land a test that fails on exactly this split.
   - `controls.txt:49` records how the model came about. The first model failed at frame 0 ("got 1, want 5 at one worker"). It was replaced by one that "moves the first red to frame 1", and the topic (`:19`) cites `per_core.rs:116-123` as the authority for the count.
   - So the expected value was fitted to what `accum_slice` does today and checked against nothing in the design or canon. It is ignored today, so it gives no false green yet. Once job 5 un-ignores it, it will pass only while the canon defect stays. Job 9's correct fix would then turn it red.
   - That frame-0 red was two real findings, absorbed:
     - the split of non-commutative fibers;
     - at one worker, `run_parallel` walks producer 0 once over the whole range where `run` walks it once per window. The ratified `the_engine_is_one_and_the_core_count_is_configuration` says one core runs the same code paths. The provisional mixed-carrier row says execution counts on mixed carriers change. Whether that divergence is a defect is a canon question for op, not a number for a test to absorb.
   - Fix: set the expected value from the canon. Under the ratified rows a non-commutative carrier is not split, so `|base, _| base` until the commutativity gate exists. Commit the frame-0 red under the job 9 catalogue row. File the one-worker divergence as a question.
   - Delete `the_unit_outer_tiling_counts_the_cores_with_a_slice` (`:409-421`). It checks the test's own copy of `accum_per` against hand numbers and says nothing about the engine.

N2. **The record-less "stores" test cannot fail on stores** (`run_parallel_wide_differential.rs:355-360`, with `skip_wide/history.rs:114-127` and `skip_wide/mod.rs:390-406`; tests; new).
   - With zero records, every snapshot column and every model column is an empty vector, and the plain build has no accumulator. So `first_bytes_difference` compares empty to empty in every frame. It can fail only through the "frame did not complete" assert.
   - The record-less build's one live judge is the count. That count is also relational today: green because `run` and `run_parallel` both run everything, while `run`'s own cone test is red.
   - Fix: delete the record-less `stores` test, or give the record-less build something observable, such as a unit writing a `Resource`.
   - Separately, set every build's expected counts from `cone_trace` times the walk's factor, not from `run`'s trace (`:196-200`). Then `run_parallel` is held to the skip rule directly, not to another entry the suite already shows wrong.

N3. **The negative controls are prose, not tests** (`mock/research/202611140613_review-jobs-one-to-three/controls.txt:1-36`; tests; new).
   - The hoisted-padding allocator and the abort-without-panic child were each shown red once by hand and then dropped.
   - `PLAN.md:137` and `a-hand-check-becomes-a-test-every-time` say a case shown red is committed as a negative test.
   - Both are cheap to commit:
     - In `static_bump_memory.rs`, a test-local allocator with the stale padding, run through the same checking function, asserting it is rejected.
     - A third child mode that calls `std::process::abort()`, asserting `assert_aborted_at_the_boundary` rejects it.

N4. **Nothing shows the accumulator comparison can see a wrong accumulator** (`run_parallel_wide_differential.rs:372-392`; tests; new).
   - The only broken-runner control alters the sink on `windowed`.
   - On `unit_outer`, the accumulator contents are the only judge of the writer, since the cone check leaves the writer unjudged. The length-mismatch branch of `first_bytes_difference` (`history.rs:200`) is never exercised.
   - Add a control that drops or alters one accumulator record after a frame and asserts `Bytes { column: 7, .. }`.

N5. **Two misses on finding 9** (low; new).
   - `crates/hilavitkutin-providers/DESIGN.md.tmpl:13-14`, rendered to `docs/PROVIDERS.md:22-23`, still says the buffer is handed over "for the life of the program". That is false under `from_raw_parts`.
   - `from_raw_parts` has no test of its own. The drop-the-provider-then-free pattern the benches depend on has never run under Miri.

N6. **The wide fixture reads through `u32` casts** (`tests/skip_wide/mod.rs:309`, `:392-404`; tests; new).
   - The seed is written, and `Out<K>` and `Sink` are read, through `*mut u32` / `*const u32`, but those types are `repr(Rust)` newtypes with no layout guarantee.
   - Round 1 removed exactly this pattern from `skip_chain` (src CL `202611140452`, item 1).
   - Fix: add `#[repr(transparent)]`, or read through the value type.

N7. **A commit subject misnames what it carries** (e6cd7de4, rules; new). "fix: hold the panic and provider tests to what they claim" also adds the public `unsafe` constructor `StaticBumpMemory::from_raw_parts` and rewrites the four bench providers. A new public API under a subject about tests is drift in the history. Name it in the landing message alongside finding 10. Every other subject in 9cb2871c..7315943a is correctly formed and under 72 characters, with no byline, trailer or em-dash.

## Verified

- Every `MemoryProviderApi` impl is still `unsafe impl`.
- `HeapBump` in the three benches that build drops `inner` before freeing `block`. The fourth, `axis_ce`, fails to build before and after the change, task 32aa15243d4e.
- The new catalogue row `hilavitkutin_run_skips_nothing_on_a_record_less_frame` is cited by the ignore it backs.
- Five ignored tests fail with `--include-ignored` exactly as the src changelist describes: at frame 1, on producer 0, at every count.
- The sink control reports `{ frame: 3, column: 6, record: 0 }`.
- `skip_wide/mod.rs` (439 lines) and the wide test (421) are under the file-size limit.

## Notes

N1 is the one that matters. A count model adjusted until frame 0 went green is the exact pattern the strict-by-design rule exists to stop. Here the result is a test whose expected value violates two ratified rows, written on the branch that is meant to judge the job fixing them. Fix N1 to N4 in this branch. N5 to N7 are small.
