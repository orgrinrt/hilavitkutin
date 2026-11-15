# Sketch findings: an accumulator grouped by the data gives the same bytes at every worker count under signed saturating addition

**Date:** 2026-11-15
**Outcome:** WORKS on a copy of the engine with one function changed. On the engine as shipped it holds only where every worker's slice of the records is a whole number of groups, so for the shipped path alone the answer is INCONCLUSIVE, with the change named and sketched beside the engine.
**Unblocks:** the priced model of option 3 has a correctness half to rest on, and the change it prices is named: one function of the accumulator split, `accum_per`.

The sketch was written unattended by a dispatched seat. The dispatcher opened `out/final-test.txt` and `engine-aligned/EDITS.md` before writing this file. Nothing here is authority, and the section on the canon below says why this is evidence for a call of the maintainer's and not a design.

## Hypothesis

Under a law that is not associative, a fold split across workers gives a result that depends on how the records were grouped. If the groups are fixed by the data (records cut into groups of G from record zero, each group folded by one unit body in record sequence, the partials kept in group sequence), the result should be the same bytes at every worker count. If the groups are whatever range a worker was handed, the bytes should change with the count. It is settled by running one unit of each kind on the real engine at 1, 2, 3, 4 and the most workers the engine admits (`MAX_CORES = 256`, `crates/hilavitkutin/src/thread/class.rs:51`), on inputs where the grouping by worker is shown to differ.

## The canon, and what adopting this would take

The maintainer ordered the evidence: "Bench the option 3 and depending on its result something like option 1 or a new approach or bench finds one". The sketch changes no canon. What it would mean to make option 3 the engine's design is another matter, and it is said first so that WORKS is not read as a candidate.

Four ratified rows send a non-commutative accumulation to one core: `ordering_is_guaranteed_by_construction`, `a_fiber_is_commutative_only_if_all_its_units_are_and_resource_accumulation_uses_per_thread_accumulators_merged_after`, `each_convergence_thread_accumulates_into_its_own_stack_local_resource_and_non_commutative_accumulation_runs_sequentially` and `records_within_a_fiber_are_independent`. Option 3 splits such a fold across cores on a grid fixed by the data, which these rows do not permit as worded. Making it the design is his call, to qualify those rows or not. In code, neither unit here declares `COMMUTATIVE`, and the shipped engine tiles both across cores anyway, which is the subject of open tickets on the tiling.

The vehicle itself was checked against `dyn_is_banned_entirely_with_no_exceptions`, `std_any_is_banned_so_there_is_no_type_id_and_no_type_erasure` and `the_monomorphised_function_is_the_type_identity_of_a_unit_and_llvm_devirtualises_the_fused_pipeline_into_straight_line_code`: it adds no `dyn`, no type id, no erased pointer and no function pointer; the units are monomorphic and dispatched by the engine's shipped walk; the engine change is arithmetic inside one `const fn`. The test-side `bytes_of` reads a slice of partials as bytes to compare them, in the harness and not in the vehicle.

Two provisional rows bear on the change. `the_worker_s_region_bounds_and_the_merge_s_read_offsets_are_one_shared_computation` is kept: the change lives inside the one function both sides read. `each_core_appends_into_its_own_region_of_an_accumulator_and_the_main_thread_merges_the_regions_in_order` holds in what it says, but its mechanism line spells `per = ceil(total/ncores)`, which the change replaces; adopting the change would need that line restated. The canon is silent on the grouping of a fold's records (`canon find grouping` gives rows about grouping units into fibers, trunks and phases only).

## The suite audit

The surface rested on is the accumulator's per-core regions and their merge. `crates/hilavitkutin/tests/leading_band_appends.rs` and `tests/gate2_accumulator.rs:1-187` were read. `run_parallel_accumulator_matches_single_core` runs at the host's worker count with an appender that depends only on its own record, so it cannot see a grouping. `leading_band_appends.rs` does not test what a body sees across its range. Neither was run, and the sketch does not rest on them: its tests compare the merged accumulator bytes against a hand model at every count.

## The vehicle

Two units, written as a consumer writes them. Each is a `WorkUnit<Always>` whose context is the engine's concrete `EngineCtx`. Each reads `Column<Val>` and is the one appender of `Accum<Part>`. `Val` and `Part` hold arvo's `IFixed<7, 0, Precise>`, which saturates at -128 and 127.

`PerExec` folds the whole range one `execute` is handed and appends one partial. Under `run_parallel` that range is the core's slice, so this is the grouping by worker. `PerGroup` folds every G = 8 records from the start of its range, appends a partial per group, and appends a short last group where the range ends inside one. After each frame the test reads the merged accumulator and folds its partials left to right.

The engine change is in the copy `engine-aligned/`, listed against the shipped files in `engine-aligned/EDITS.md`: a crate constant `ACCUM_GROUP = USize(8)`, and `accum_per` rewritten from `ceil(total / ncores)` to `ceil(ceil(total / G) / ncores) * G`, so every slice is whole groups. The two differ in nothing else.

## What was tried, in order

No earlier sketch splits a fold: `202606041800_accumulator-append-selector` proved the append surface compiles, `202611132010_oracle-over-record-op` runs one thread and folds nothing, `202611140610_parked-worker-plane` is about aliasing.

1. The first run built all five counts of one unit inside one function and overflowed a test thread's stack (`out/attempt1-test.txt`): a scheduler sized for `MAX_CORES` is large in an unoptimised build. Each count now builds in its own function. Nothing else changed.

2. The control on the shipped engine, `PerExec`, 6144 records. Combined results: 1 worker -1, 2 workers 23, 3 workers 126, 4 workers 126, 256 workers 39. The grouping by worker gives different bytes, as the brief requires before an equality may count.

3. `PerGroup` on the shipped engine, 6144 records, where every count's slice is whole groups, on the same inputs. 768 partials and combined -102 at every count, equal to `run`'s and to the hand model.

4. `PerGroup` on the shipped engine, 1001 records, where a slice boundary falls inside a group. 1 worker -110, 2 workers -128, 3 workers -10, 4 workers 9, 256 workers 0. The same unit on the shipped engine is not grouped by the data once the core count cuts a group. This is the case that must fail for step 5.

5. `PerGroup` on the copy at 1, 3, 8, 9, 255, 1001, 2049 and 6144 records. At every record count the partial bytes and the combined bytes are equal at all five worker counts, equal to `run`'s and equal to the hand model: 1001 gives 126 partials, combined -110; 6144 gives 768, combined -102.

6. `PerExec` on the copy, 1001 records: -42, -128, -126, -31, -110. The change does not make a grouping by worker safe. The equality in step 5 comes from the data-fixed grouping together with the aligned split.

The printed results of steps 2 to 6 are in `out/final-test.txt`.

The law and the inputs, with no engine (`tests/law.rs`). The law saturates at both bounds. It is not associative: (100 + 100) + -100 = 27 and 100 + (100 + -100) = 100. On the engine inputs, no grouping, groups of 8 and groups of 7 give three different results: -42, -110 and 9 at 1001 records; -1, -102 and -128 at 6144. So an equality found between counts reflects the same grouping, not inputs the law happened to forgive.

## The result

A fold whose groups are fixed by the data gives the same bytes at 1, 2, 3, 4 and 256 workers under signed saturating addition, on the engine's accumulator path, for every record count tried, once `accum_per` rounds its stride up to whole groups. The engine as shipped gives such a grouping only when the record count makes every core's slice a whole number of groups.

The grouped result is not the sequential left fold. At 1001 records the grouping gives -110 at every count, while a single-thread left fold over the records gives -42. Option 3 fixes the answer across worker counts by changing what the answer is, at one worker too.

Holds for: toolchain `nightly-2026-05-28`, x86_64 linux, test profile unoptimised; hilavitkutin `dev` at 2b137e9d; arvo rev 1b2796e; notko `dev` at 8a74dbcd; the law `IFixed<7, 0, Precise>`; G = 8; one input column, one accumulator, one unit, a carrier with no reach reader and no meta units, no leading-band appends; `run_parallel` at worker counts 1, 2, 3, 4 and 256 through an executor that reports the count, and `run`; two frames each; record counts 1, 3, 8, 9, 255, 1001, 2049 and 6144 on the copy, and 6144 alone for the equality on the shipped engine; the fixed input sequence of `src/lib.rs`. The build crate's rustc flags were applied to the copy and dropped for the shipped engine and its path dependencies.

## What is NOT established

- A unit written against the api alone cannot do this. `PerGroup` relies on two facts that are not in the api: its range starts on a group boundary, and G, which on the copy is an engine name. An engine that calls `execute` once per group, a morsel grid fixed from record zero, would let a unit stay unaware of G. That shape is not built.
- Where G lives in a design (the morsel, a plan dimension, the run configuration) and what it costs. The cost is a bench.
- A carrier with several units, a reach reader, meta units, or leading-band appends into the same accumulator; a phase with two trunks; `run_fused`.
- A fold into a `Resource` read and written per record. In code, a tiled carrier shares one resource binding across cores (ticket `hilavitkutin_a_tiled_accumulator_carrier_shares_one_resource`), and this sketch does not go near it.
- The refusal and capacity behaviour under the rounded split, when a region fills.
- Miri, a release build, any other target, any G other than 8, worker counts above 256.
- The engine's own suites, which were not run, and the repository's lints over the sketch.
