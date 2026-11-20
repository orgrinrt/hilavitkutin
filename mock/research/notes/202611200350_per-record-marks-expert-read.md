# Per-record marks across record sets: an expert read of loru's ask

Date: 2026-11-20. Seat: one expert (frank-mcsherry persona), worktree seat/hila-marks-1 cut from dev at 9918113f. This seat is the first expert on every canon-fit call below. A second, independent read is owed before any of it is acted on.

The ask is loru's, on ticket hilavitkutin_host_side_column_read_and_write_on_a_built_sche, and it has three parts. First, that the engine carry a per-record change mark along declared reads across record sets (a string marked changed marks the nodes that gather it, which mark the pages that fold them), holding the reverse relation of each gather. Second, that it offer a frame over a chosen subset of records for a server's on-demand page. Third, that a red test ask only the morsels covering an import to rerun. This read answers the question the canon poses about each of the three, not the question as filed, after the two gates the dispatch requires.

The short form. The second and third asks can be separated from the first, and parts of them can land. The first is not an engine change anyone may make now. As filed it presupposes two engine surfaces the canon withholds: a data-dependent cross-record read, against a stamped row, and a read across record sets of different counts, against op's answer "Bench both first". Two things may proceed: the sketch and bench op ordered, and the per-morsel mark within one record set, which is ratified intent with a design sketch owed. A plain-Rust probe beside this note (mock/research/sketches/202611200350_per-record-marks/) finds that on loru's own 300-page site a per-morsel mark at the L1-derived window reruns exactly what the store mark reruns. The engine's own catalogued red agrees: it runs 4 windows over 20,000 records. So the ratified grain does not close loru's gap at the point loru measured. That is the number loru's findings say was not measured. It is a count, not a time.

## 1. The canon gate

Verdict: misaligned for the first ask as an engine change, and aligned for the per-morsel mark within one record set and for the red test. The demand selection is a call the canon does not settle. The rows, by id, with the words that bind:

Ratified, and governing:

1. change_detection_is_per_morsel_and_skips_what_is_unchanged: "Change detection is per morsel: a write marks the morsel as changed and an unchanged morsel is skipped. It propagates through the graph, so an unchanged root skips all its transitive dependents." This licenses a mark finer than the store and fixes its grain at the morsel. A record is finer than a morsel, so it does not license a per-record grain.
2. the_engine_owns_the_generation_counters_and_the_skip_logic: "hilavitkutin owns the per-morsel generation counters and the logic that skips what is unchanged across the graph, as a runtime scheduling concern." For whatever reads the engine has, the carry belongs in the engine crate, not in the api or the consumer.
3. a_unit_whose_inputs_are_all_clean_skips_the_pass: "the mark propagates along every edge of the graph, and a unit whose whole predecessor set is clean skips execution entirely", mechanism "dirty[j] |= dirty[i] for every edge i to j ... predecessors[N] & dirty_mask: one AND per unit". The ratified propagation is per unit, along graph edges. A per-morsel propagation would lift it to (unit, morsel) pairs, and no row spells that out.
4. version_stamps_tell_which_stores_a_unit_actually_modified: "The versions are captured before a unit runs and compared after, which gives the stores the unit actually modified." This is early cutoff at store grain, ratified and not built. The engine DESIGN.md.tmpl:1485-1487 says "within a pass the skip stays structural: a successor of a unit that ran is dirty whether or not the unit changed anything".
5. These rows fix what a morsel is: records_within_a_fiber_are_independent, storage_is_contiguous_and_a_fiber_s_columns_share_one_arena, morsel_boundaries_are_stable_unless_the_record_count_changes, morsel_boundaries_are_baked_into_the_compiled_per_core_programs, and the_morsel_size_is_the_l1_budget_divided_by_the_written_bytes_per_record_within_fixed_bounds ("(L1_usable / sum of write_sizes).clamp(MIN_MORSEL, MAX_MORSEL)", MAX_MORSEL = 8192). The last one is what makes the morsel grain no help to loru (section 3.1).
6. the_engine_owns_dag_scheduling_dispatch_morsels_stages_stealing_and_progress_and_not_storage_schema_queries_ui_or_io: "It does not own column storage, the data model, schema, query planning". This bears on the demand selection.

Provisional, stamped (binds as op's word for now, and is never a ratification):

7. the_only_cross_record_surface_is_the_accumulator_append_path: "the only surface that crosses records is the accumulator append path." A gather is a cross-record read, and carrying a mark along "the reverse relation of each gather" presupposes one. The api design already contradicts this row with Reach<C, N> (hilavitkutin-api/DESIGN.md.tmpl:505, shipped at hilavitkutin-api/src/member.rs:93-98). That is drift in the design tier against a stamped row (section 6). It is no license for a second surface, let alone an unbounded one.
8. the_per_morsel_generation_counters_are_not_built_and_the_engine_has_a_coarse_per_store_dirty_only: "It asks for a design sketch before any implementation, on where the counters live with no allocation and a morsel count per fiber that varies with the window, and on how they compose with the existing store dirty mask." No implementation of any finer mark comes before that sketch.
9. a_morsel_is_reprocessed_only_when_its_generation_is_past_the_one_last_processed: "Each fiber ... records the generation it last processed for each of its input morsels." This row is the pull carry of section 3.1. The push carry is in no row.
10. the_dirty_seed_is_set_by_the_consumer_marking_a_store_dirty_or_by_a_resource_replacement, and at_ecs_scale_the_consumer_holds_separate_schedules_per_component_set ("separate schedules per component set ... several pipeline runs on subsets ... the design must not assume a single all-encompassing run").

Answered questions (op's words):

11. how_record_sets_of_different_counts_live_together: "Bench both first", recorded as "The api revision takes neither shape until a sketch and a bench of both exist." Strings, nodes and pages are three record sets with three counts: 4,240, 6,000 and 300 per locale at loru's 300-page point. A node gathering a string and a page folding nodes are both cross-set reads, which is option a ("a cross-set reach a declared read the plan orders"). Carrying a mark along them in the engine is option a with change detection on top. Under option b ("crossings through resources or accumulators the consumer carries"), the reverse relation belongs to the consumer by construction, and that is exactly loru's columns arm. So the first ask, as filed, takes option a before the bench op ordered, against "takes neither shape".
12. whether_hilavitkutin_owns_geometry: "Opt-in crate over the api", whose option text says it is "built on the api's declared cross-record read". This presupposes a declared read with a reach in records, which is bounded and arithmetic. A gather is neither, so this answer does not license one, and it sits in tension with row 7 (call 1).
13. which_stores_declare_what_changed_means ("Last option", the writing unit declares) and what_changed_means_for_a_computed_column ("Declared per column"). These settle cutoff for unit writes and say nothing of host writes.
14. who_says_that_a_store_changed is open. A host import that marks a range is one of its cases.

The zero for gather. cargo mock canon find gather returns 0 tenets, and so do inverse, scatter, cross-set, record set and record domain. reverse returns seven tenets, none about a read relation. already-said for "reverse index" finds nothing in the rows or the research. The only text about a gather near the canon is unratified research from yesterday, mock/research/202611191955_consumer-surface-seat/NOTES.md:75 ("Per-morsel skip through a gather needs an inverse index from target granules to reader morsels"). It is agent output and presumed wrong. That it agrees with this read on that sentence is not corroboration.

What follows from the gate. The first ask is not designed here. What section 3.1 delivers for it is input to the sketch and bench op ordered: what each arm of that bench must carry for change detection, with the costs counted in the probe. No shape is proposed for landing.

## 2. The suite audit

The suite gate came first: CARGO_INCREMENTAL=0 cargo mock test from the worktree root at 9918113f, the whole suite, unfiltered.

- Workspace members: "798 tests run: 797 passed (2 slow), 1 failed, 40 skipped". The failure is hilavitkutin-build::refused a_requirement_of_each_form_is_refused_where_unmet (crates/hilavitkutin-build/tests/refused.rs:51, "none: the kept set was refused"). This container's toolchain lacks cranelift ("a pragma of this build needs cranelift, which this toolchain lacks").
- The tool crate mock/tools/cross-target fails 2 of 8: the_host_and_wasm_leave_the_same_stream and a_wasm_build_that_computes_otherwise_is_told_apart. Its wasm build of hilavitkutin-api stops on E0107.
- Bench crates: 182 crates, 33 carrying tests, 167 passed, 0 failed.

Neither failure touches change detection. Neither was chased further, which is a gap in this read. The 40 skipped are catalogued ignores, and the ones in this area are listed below.

Read in full or in their assertions: tests/host_import.rs, tests/incremental_skip.rs, tests/skip_chain/history.rs, tests/run_parallel_skip_differential.rs, tests/run_parallel_wide_differential.rs, tests/gate2_parallel_plan_skip.rs and tests/r3b_change_class_count.rs. Also read: the header of the skip_wide fixture, the frozen-authors drive, and every reader of src/plan/dirty.rs. No test in this area is tautological. The differentials hold a skipping run against an every-store-dirty run, which is itself checked against an independent model (skip_chain/history.rs:179-201) and a hand-written cone (:208-231).

Findings:

1. The file the dispatch names as the dirty mask is not the dirty mask.
   - src/plan/dirty.rs:22-166 defines DirtyMask and DirtyMasks.
   - The runtime change seed is the AccessMask in Scheduler::store_dirty (src/scheduler/mod.rs:224). dirty_units propagates it per unit (src/scheduler/dirty.rs:186-220).
   - DirtyMasks is computed at plan step 8 (src/plan/steps/step8_rank.rs:32-89), where it holds the union of each fiber's writes. It is copied bit by bit onto ExecutionPlan::dirty (src/plan/mod.rs:385-406, field at :97). Nothing reads ExecutionPlan::dirty.
   - So there are two structures named for one concern, one of them dead and misnamed, and the dead one has no test. Whoever builds the counters will find two dirty types.
2. DirtyMask::set and contains (src/plan/dirty.rs:64-66, :75-77) silently drop an index of 64 or more, while the assert at :43-46 bounds only the capacity. This is untested, and moot while the code is dead.
3. The red test for the third ask exists and is too weak. It is tests/host_import.rs:806-828, an_import_of_one_record_reruns_only_the_morsel_that_covers_it. It fails as catalogued: "assertion `left == right` failed: only the window holding record 0 reruns, left: 4, right: 1" (output/engine-red-host-import.txt in the probe).
   - Its setup helps: it imports at record 0 only (:819), so a mark that maps every record to window 0 passes it.
   - It asserts a count (:823-827) and never the output.
   - It has no unit downstream of the reader, so propagation along an edge is never asked.
   - It runs under run alone, and it does not check that Lone stays rested.
   - Missing instances: a record mid-column, a window's first record, the last record of a partial last window, a span crossing a boundary, and a store read by two fibers with different windows.
   - This is a sampled law: one shape of five, and the easiest one.
4. tests/incremental_skip.rs:67-113 and :115-155 are redundant with :161-181. The latter's first_ran_difference against cone_trace asserts, frame by frame, exactly which units ran, including the two-hop cone (skip_chain/history.rs:208-231).
5. Missing fundamental: run_parallel never asks the dirty mask. This is catalogued as ignored reds at tests/run_parallel_skip_differential.rs:147 and :162, and at tests/run_parallel_wide_differential.rs:370, :372, :381 and :396 (ticket run_parallel_dispatches_every_unit_each_pass_and_never_asks). Any finer mark helps run and run_fused only, and loru's engine_pool arm gets nothing from any mark today.
6. Missing fundamental: run runs a record-less frame whole (tests/run_parallel_wide_differential.rs:379, ticket hilavitkutin_run_skips_nothing_on_a_record_less_frame).
7. Missing fundamental: early cutoff. No test anywhere, red or green, asserts that a unit whose writes changed nothing leaves its successors rested, although version_stamps_tell_which_stores_a_unit_actually_modified is ratified. The engine design says the skip stays structural (DESIGN.md.tmpl:1485-1487). The api design marks the unit's declaration of what changed as unread (hilavitkutin-api/DESIGN.md.tmpl:593, placeholders_tagged_8c7cc9768579).
8. Missing fundamental: incremental equals recomputation under a partial mark. The conformance drive marks a whole column every frame (crates/hilavitkutin-frozen-authors/tests/common/mod.rs:403-406), and the host_import differential compares whole-store histories (tests/host_import.rs:734-801). When a partial mark exists, this is the first test it needs.
9. tests/host_import.rs:256-282 and tests/incremental_skip.rs:184-199 pin that an unmarked raw write is invisible. They are the mutants that show the differentials can fail, and they hold under either option of who_says_that_a_store_changed. This is not a finding against them, and they do not settle that question.

## 3. The three asks

### 3.1 The mark carried along declared reads, across record sets

What the engine recomputes today when one string changes, in loru's bench: none of the gather or the fold. The host gathers each node's string hash into a per-node column before the frame (loru FINDINGS-202611191854-p92u.md, lines 17 and 21). The edited locale's column is marked whole, and every window of the node unit runs. The page fold and the render run on the host after the frame. The engine holds neither of the two reads the mark is asked to travel. So the ask is not "carry the mark further". It is "move two cross-set reads into the engine, then carry the mark along them", and the first half is the bench-gated record-set question.

The probe models loru's site, the gather and the fold incrementally:
- Model: one locale, loru-bench-data's slot layout.
- Grains: the store, a window of W records, and one record.
- Carries: push, through a reverse relation, and pull, through the forward relation and a generation compare.
- Cutoffs: structural and differs.
- Demand frames.

Its differential holds 1,152 runs to recomputation after every frame: windows of 1, 2, 3, 7, 19, 20, 21, 40, 64 and 1000 against the twenty-node page, at 1, 3, 7 and 13 pages, with 24 seeds of 300 steps each. Four mutants are each caught: a stale inverse after a rekey, a demand frame that forgets marks it did not run, a fold reverse that misses the last page, and a cutoff that drops every change. Outcome WORKS. It is one instance, by one author, and shares no code with the engine.

Counts from output/counts.tsv, as node and page records recomputed (push, structural):

| scenario | pages | store | W 64 | W 256 | W 2304 | W 4608 | record |
|---|---|---|---|---|---|---|---|
| one page-unique string | 300 | 6000, 300 | 128, 64 | 512, 256 | 6000, 300 | 6000, 300 | 1, 1 |
| one page-unique string | 3000 | 60000, 3000 | 192, 64 | 768, 256 | 6912, 2304 | 13824, 3000 | 1, 1 |
| the string every page carries | 300 | 6000, 300 | 6000, 300 | 6000, 300 | 6000, 300 | 6000, 300 | 300, 300 |
| one node rekeyed | 300 | 6000, 300 | 64, 64 | 256, 256 | 2304, 300 | 4608, 300 | 1, 1 |

Finding 3.1.a, the grain. Under the ratified formula, a fiber writing 8 or 16 bytes per record on a 48 KiB L1 gets W of 4608 or 2304. At 300 pages one page-unique edit then reruns all 6000 nodes and 300 pages, the same work as the store mark loru measured. The engine gives a second instance: its red runs 4 windows over 20,000 records for a unit writing a 4-byte value, so its window there is at least 5,000 records, and loru's 6,000 nodes would be one or two windows.
- Predicate: holds where N_nodes / W is about 3 or less.
- It moves with L1 size, bytes written per node record, MIN_MORSEL, MAX_MORSEL, page count, locale count (each locale is its own store) and nodes per page.
- At 3000 pages, W 2304 reruns 6912 of 60000 nodes, about a ninth.
- Only the record grain, and the 64-record micro-morsel interval (held row micro_morsels_end_every_64_records_...), come near loru's columns arm. The record grain is below what the canon says.

Finding 3.1.b, the gather target's tiling. No fiber windows a gather's target, so the canon leaves its granule undefined, and in the model that granule decides the cone.
- At W 2304 and 3000 pages, one page-unique edit reruns three node windows, because the string window holds strings of about 165 pages.
- At W 256, a page-unique string that shares a window with the 40 shared keys inherits their fan-out: every node window is marked. A demand for page 150 after an edit on page 7 then reruns 5120 nodes (all 20 node windows under the demanded page window). At W 64 the same demand reruns none.
- Predicate: holds where a target granule mixes records of high and low fan-out. It moves with key layout, the target granule and fan-out.

Finding 3.1.c, push against pull at record grain. Push touches one inverse link and recomputes one node and one page, at both 300 and 3000 pages. Pull recomputes the same records but reads the whole key column every frame (6000 and 60000 entries). Pull is the "per-record mark alone" that loru's findings (line 47) say does not close the gap. Here it costs an index scan, not a recompute, which is cheaper per entry and still proportional to the site. Which one beats the column mark is unpriced: it needs the harness.

Finding 3.1.d, the reverse relation is arithmetic where the read is arithmetic. For an identity read, the reverse of a morsel is the morsel. Across fibers with different windows it is range overlap, for Reach<C, N> it is the morsel widened by N, and for a contiguous fold it is range arithmetic. None of these needs storage. Only a gather, whose relation is data, needs storage:
- At record grain: three fixed columns (a head per target record, a previous and next link per reader record), two link writes per rekey.
- At window grain: one bit per (target window, reader window) pair. That one matrix is both the forward and the reverse relation, rebuilt from W key reads per index change.

In both cases the relation goes stale when the index column changes, and the first mutant shows the differential catching it.

Finding 3.1.e, early cutoff is half the ask, and it was not filed. Under the differs cutoff, a same-value rewrite stops at the node hash and 0 pages refold. The structural skip refolds every page (counts.tsv, noop_edit). Every candidate below needs the ratified version stamp read per granule, or it propagates every recompute as a change.

The candidates. Each of them needs the per-morsel counters and their design sketch (row 8) for its grain. Each needs the dirty mask on run_parallel (audit 5) before it helps a pooled frame.

1. Store mark, as shipped.
   - Cost: the whole cone on every edit. loru measured 37.07 us against 7.96 us for its columns arm (FINDINGS lines 41 and 44: one thread, hilavitkutin 252c5190, their second run).
   - Needs: nothing.
2. Consumer-held reverse relation, with an engine-held per-morsel mark on one store. The consumer keeps its index, finds the affected node records, and marks a range of the node store. The engine carries the mark within the node set by arithmetic (3.1.d).
   - Cost: the engine side has the grain of 3.1.a, so at 300 pages the engine saves nothing.
   - Needs: a range mark on the host path, the counters, and the who_says_that_a_store_changed call.
   - Canon: option b on the crossing, ratified intent on the grain. It is the only candidate that may land before the bench. loru says it keeps no index beside the engine, so this serves loru's measurement and not loru's architecture.
3. Engine-held reverse relation at window grain, as a bitmatrix, push or pull over the same bits.
   - Cost: T_m times R_m bits per gather edge, W key reads per index change, and the inflation of 3.1.a and 3.1.b.
   - Needs: a declared gather (against row 7), the cross-set read (option a), a tiling for a gather target, and the counters.
   - It is the lift of predecessors[N] & dirty_mask (row 3) from units to windows, the shape most continuous with the ratified text.
4. Engine-held reverse relation at record grain, as intrusive lists, push.
   - Cost: three columns per gather edge, two link writes per rekey, and work proportional to changed records times fan-out.
   - Needs: everything 3 needs, a grain below the morsel, the lists placed in consumer-provided storage (the_consumer_provides_the_backing_memory_of_a_column, ratified), and an answer for concurrent rekeys under run_parallel. The model has no such answer.
5. Pull at record grain, with per-record generations and the forward index.
   - Cost: no inverse, but an index scan per frame and generation columns on every tracked store.
   - Needs: everything 3 needs except the reverse relation, plus the sub-morsel grain. It is row 9 at a finer grain.

The dependency on the record-set answer, exactly. Candidates 3, 4 and 5 put the gather and the fold inside one schedule over sets of different counts, which is option a. They may exist only as arms of op's bench, beside option b's arm, which for change detection is candidate 2. The bench that decides the record-set question must carry a change-detection workload (changed fraction per frame, a rekey, a same-value write, a demand frame), with loru's columns arm as the outside competitor. A bench of cold frames cannot decide a against b for loru.

What would close loru's gap, as far as can be said without a timer: not the morsel grain (3.1.a), and not a pulled per-record mark (3.1.c). On the counts it is candidate 4 with the differs cutoff, which is the columns arm moved into the engine. Its canon cost is three refusals deep: a gather surface, a cross-set read, and a grain below the morsel. That is a reason to put 2 and 4 side by side on the harness against loru's columns arm, not a reason to build 4.

### 3.2 The demand selection

A demand frame must make a chosen set of output records current and may leave the rest stale. The probe holds it to recomputation over the demanded pages, and it decides what to run from forward relations only: the fold's forward is arithmetic, and the gather's forward is the index the reader already holds. That mirrors 3.1. Change propagates forward and needs each read's reverse. Demand propagates backward and needs each read's forward, which the engine has wherever it has the read. Staleness inside the demand still needs a carry. A demand frame must also leave the marks of what it did not run in place; the second mutant is caught under both carries.

Whether it belongs in the engine is op's call. The engine design says "The incremental skip may rest a unit only where its output would have been the same, so an entry that skips equals a reference that never does" (DESIGN.md.tmpl:1764-1765), and a demand frame breaks that outside the demand by construction. The canon is silent. a_unit_whose_inputs_are_all_clean_skips_the_pass licenses skipping clean units and says nothing about dirty ones. Whether choosing a backward cone is scheduling or "query planning", which is excluded, is a reading and not a fact. Op's words in where_the_record_count_of_a_column_lives ("indices to process needed windows") read most naturally as a list of windows to process. But they answered another question, and they were phrased as a question back.

The candidates:

1. Selection over the baked tiling, at morsel grain. The run window loop already ANDs a unit mask per window (src/scheduler/run.rs:249-262, :252), and a demand adds one more mask.
   - Cost: the 3.1.a inflation. At W 2304 and 3000 pages one demanded page reruns 46080 nodes.
   - Needs: the counters, a restated conformance contract, and, for loru, option a.
2. Index-list dispatch: a window walks a list of record indices.
   - Cost: one index load per record and lost contiguity, but no inflation.
   - Needs: a dispatch form the canon lacks (morsel_boundaries_are_baked_into_the_compiled_per_core_programs, ratified), and the contract restatement.
3. On the consumer side, per the stamped at_ecs_scale row: a separate small schedule, or a run over an imported subset.
   - Cost: a build per subset shape, and a copy in and out per demand.
   - Needs: nothing new. Dump and import have landed. For size, the zero-copy half is wanted (ticket the_zero_copy_half_of_the_host_path_evict_and_inject_is_not).

Candidates 1 and 2 serve loru only if the page's backward cone is inside the engine, which is option a. Within one record set they do not depend on the record-set answer.

### 3.3 The red test

It belongs in the engine's tests, it exists (tests/host_import.rs:806-828, ticket a_host_import_marks_its_whole_store_changed), it is aligned, and it is red as stated. Audit finding 3 lists what it lacks. The strong form, for whoever holds the ticket:
- One red per shape: record 0, a mid-window record, a window's first record, the last record of a partial last window, a span over one boundary, and two fibers with different windows reading one store.
- Each asserts that only the covering windows of the reader and of its downstream unit reran.
- Each asserts that a dump equals an every-store-dirty twin's.
- Each runs under run, run_fused and run_parallel.

A red asking that only the covering records rerun would contradict the ratified grain, and it would stay red under a correct implementation. It should not be written.

## 4. Calls the canon leaves to op

1. Cross-record read surface. Options: a, none, as the stamped the_only_cross_record_surface_is_the_accumulator_append_path says, and Reach leaves the api design; b, a bounded reach only, as op's geometry answer presupposes, with the row amended; c, a reach and a declared data-dependent gather. The first ask needs c.
2. The change-detection workload of the record-set bench. Options: a, the layout is decided on cold frames, and change detection is a later round; b, the bench carries a changed-fraction sweep, with candidate 2 as option b's arm, candidate 3 or 4 as option a's arm, and loru's columns arm as the outside competitor; c, a separate change-detection bench after the layout bench.
3. Grain of the mark. Options: a, the reading fiber's morsel window, as ratified; b, a fixed granule per store (64 records is the held micro-morsel interval); c, the record.
4. Tiling of a store no fiber windows (a gather target, a host input). Options: a, the writing fiber's window; b, a fixed granule per store; c, each reader's window, with a generation set per reader.
5. Demand frames. Options: a, every frame equals the reference, and demand stays on the consumer side; b, a demand frame is a distinct entry, held to the reference over its demand, keeping the marks it did not run; c, as b, but only at morsel grain on the baked tiling.
6. Early cutoff in the carry. Options: a, built with the counters, reading the writing unit's declaration per granule; b, store-grain cutoff first, granule cutoff later; c, the skip stays structural, as the engine design says today.
7. Who marks a host-written range (who_says_that_a_store_changed, narrowed to the host path). Options: a, import marks exactly its span; b, the host names the store and the engine compares to find the range; c, import marks its span, and a raw write marks nothing.
8. ExecutionPlan::dirty and DirtyMask(s). Options: a, delete; b, rename them to the per-fiber write union and give them a reader; c, make them the home of the per-morsel generations.

## 5. Predicates

1. 3.1.a holds for one locale of loru's site at 300 pages, at W of 2304 and wider. The engine instance holds for a 4-byte write on this container at 9918113f. It moves with L1 size, bytes written per record, MIN_MORSEL, MAX_MORSEL, page count, locale count and nodes per page.
2. 3.1.b holds where shared and page-unique strings interleave in one set, as loru-bench-data lays them out. It moves with key layout, the target granule and fan-out.
3. 3.1.c holds for one single-valued gather. It moves with the number of gathers, fan-out, and the unpriced cost of a relation read against a recompute.
4. 3.1.d holds for single-valued gathers and contiguous folds. A multi-target read, or a non-contiguous fold, moves the storage term.
5. 3.1.e holds for a same-value rewrite. It moves with whether the value differs, which the writer declares.
6. Every claim about the engine holds for run and run_fused at 9918113f. It does not hold for run_parallel, which ignores the mask, or for a record-less frame.
7. The probe is one model by one author, one locale, with no time. It establishes shapes and counts and decides no fork.
8. loru's figures carry their own predicates: one thread, a shared four-core container, hilavitkutin 252c5190, their second run.

## 6. Unlicensed mechanisms noticed

1. ExecutionPlan::dirty and DirtyMasks (src/plan/mod.rs:97 and :385-406, src/plan/steps/step8_rank.rs:32-89). They are computed on every plan and read by nothing. No design sentence licenses a second dirty structure beside store_dirty.
2. Reach<C, N> (hilavitkutin-api/src/member.rs:93-98, hilavitkutin-api/DESIGN.md.tmpl:505 and :524-552) and ReachRule (src/scheduler/reach_rule.rs). This is a cross-record surface, shipped and designed against a stamped row, and the design moved before op did.
3. mock/research/202611191955_consumer-surface-seat/NOTES.md:64 and :112-124 propose concrete Gather and Table types and a 64-record granule. This is unratified, and nothing should be built from it, this read included.

## Evidence

mock/research/sketches/202611200350_per-record-marks/ (commit bbf2b790): src/lib.rs, tests/differential.rs, src/main.rs, output/tests.txt, output/counts.tsv and output/engine-red-host-import.txt. Nothing is timed. loru's figures are from loru origin/dev, mock/benches/FINDINGS-202611191854-p92u.md, site_incremental section, lines 35 to 47.
