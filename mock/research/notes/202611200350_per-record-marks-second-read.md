# Per-record marks across record sets: a second, blind expert read

Date: 2026-11-20. Seat: one expert (sebastian-aaltonen persona), worktree seat/hila-marks-2 cut from dev at 9918113f, before the first seat's commits, without reading the first seat's note. It is the second independent read of loru's three asks on ticket hilavitkutin_host_side_column_read_and_write_on_a_built_sche, after 202611200350_per-record-marks-expert-read.md.

The two reads agree on every gate call that would change the work. Carrying a mark across record sets is held by op's answer "Bench both first" to how_record_sets_of_different_counts_live_together. A reverse relation held in the engine has no canon behind it, since no row names a gather. A mark at morsel grain inside one record set is ratified intent, and turning it on needs the design sketch the stamped row the_per_morsel_generation_counters_are_not_built_and_the_engine_has_a_coarse_per_store_dirty_only asks for. The red test of ask 3 cannot tell the right window from a wrong one and is widened before anything is built against it. They differ in emphasis: this read adds that per-fiber windows make a mark keyed by morsel index wrong across fibers, that loru's pass_schedule bench is not the bench op ordered, and that a frame over a selection is unsound on any mark cleared at the end of a frame.

What follows is the seat's hand-back as returned, unedited.

Seat 2 read of loru's three asks on ticket hilavitkutin_host_side_column_read_and_write_on_a_built_sche, worktree /home/user/.worktrees/hila-marks-seat2 at 9918113f (3 commits behind origin/dev, all docs and lint scoping, none touching this surface). Nothing was edited, timed, pushed or written to the repo. I am the first reader here as far as I know, so every canon call below is owed a second independent read. I did not read the other seat.

Short answer. Ask 3 passes the canon gate, and its red test is already on dev. That test is a sampled law and needs widening before anyone builds against it. Ask 1 splits: carrying the change mark at morsel grain along declared reads inside one record set passes, while the per-record grain, the crossing of record sets, the reverse index held in the engine and the incremental page fold all fail or wait on op. Ask 2 is ambiguous inside one record set and fails across record sets.

## 0. Test gate (run first)

I ran the whole suite with `CARGO_INCREMENTAL=0 cargo mock test` (log at /tmp/claude-0/-home-user/51d3f9bb-9682-5795-b7da-7af7718c3c35/scratchpad/mocktest.log).
- Workspace members: 798 run, 797 passed, 1 failed, 40 skipped.
- The failure is hilavitkutin-build::refused a_requirement_of_each_form_is_refused_where_unmet (crates/hilavitkutin-build/tests/refused.rs:51). Its "kept" set is refused because this container's toolchain has no cranelift component. So this assertion depends on the environment, and no known_red row says so.
- Tool tree mock/tools/cross-target: 2 of 8 failed (the_host_and_wasm_leave_the_same_stream, a_wasm_build_that_computes_otherwise_is_told_apart). The host probe fails to compile hilavitkutin-api with E0107 at `WriteSet: AccessSet + Project<...>`, while the workspace build of the same crate is green. The probe most likely resolves a different notko or arvo than the workspace lock does. I did not confirm that.
- Bench and support crates: 167 passed.
- known_red/reds.toml still lists hilavitkutin_compile_fail_fixtures as red on dev. It passed here, so the entry is stale.
- I ran the two ignored tests in host_import.rs and both are red as their tickets say. an_import_of_one_record_reruns_only_the_morsel_that_covers_it gives left 4, right 1. The exchange-mode test gives A(0) against A(1).

I read the test bodies in the surface: all of host_import.rs, incremental_skip.rs, and the skip_wide fixture header. None is tautological. The skip differentials compare against a model and carry a mutant twin, which is real work.

The ask-3 red test at host_import.rs:806-828 does fail the gate as a sampled law whose setup helps:
- It imports only at record 0 (`import(USize(0), &[A(1)])`, :819).
- It asserts only that the number of `execute` calls grew by 1 (:823-827). A gate that reruns the wrong window, say the last of the four, passes it. Nothing checks that B[0] became 2 through `dump`.
- It never tries the middle window, the short last window (20,000 records over a 6,144 window), an import straddling a window boundary (which should rerun 2), a whole-store import, a second reader fiber with a different window, or a reader one hop down the cone.
- It runs under `run` alone, not `run_parallel` or `run_fused`. It never checks that `Lone` stays rested here.

The current coarse mark cannot pass it, but a wrong fine mark can. So the first deliverable owed is a widened test: a matrix of import position, run path, store kind (Column, Buffered Snapshot, Buffered Exchange) and hop, judged on the identity of the window through dumped values. Nothing on asks 1 to 3 should be built until that exists. The rest of this report is a read, not work built on that test.

## 1. Canon gate per ask

### Ask 3: aligned

The red test exists already (host_import.rs:806-828, ticket a_host_import_marks_its_whole_store_changed). The rows that bind:
- change_detection_is_per_morsel_and_skips_what_is_unchanged, ratified: "a write marks the morsel as changed and an unchanged morsel is skipped".
- the_engine_owns_the_generation_counters_and_the_skip_logic, ratified.
- column_data_moves_out_of_and_into_the_engine_through_evict_dump_and_inject_import, ratified, and related to change detection. `import` is that inject path.
- the_per_morsel_generation_counters_are_not_built_and_the_engine_has_a_coarse_per_store_dirty_only, provisional and stamped: "asks for a design sketch before any implementation". A red test is not an implementation, so it does not breach this row, but code to turn the test green does need the sketch first.

One subtlety goes beyond loru's ask. A host-side mark over a range without a write (loru-coord's `mark_dirty_at(range)`) waits on question who_says_that_a_store_changed, which is open. The mark that `import` makes over its own range holds under either option, because the engine performs that write.

### Ask 1: splits

1a. Carrying a mark along declared reads inside one record set, at morsel grain: aligned.
- a_unit_whose_inputs_are_all_clean_skips_the_pass, ratified: "the mark propagates along every edge of the graph".
- a_morsel_is_reprocessed_only_when_its_generation_is_past_the_one_last_processed, provisional and stamped.

1b. "Per-record" as the grain of the mark: misaligned as the grain the skip uses.
- change_detection_is_per_morsel_and_skips_what_is_unchanged, ratified: "Change detection is per morsel".
- a_morsel_is_a_three_dimensional_l1_sized_window_into_a_fiber_s_arena, ratified, puts the morsel on a fiber's arena.
- Whether a mark finer than the morsel, held on the store and used only to decide which morsels rerun, also breaches the first row is not settled. That is call C1 below.

1c. "Across record sets": misaligned now.
- Question how_record_sets_of_different_counts_live_together was answered "Bench both first". The reading recorded with it: "The api revision takes neither shape until a sketch and a bench of both exist."
- The api design says one count per scheduler (mock/crates/hilavitkutin-api/DESIGN.md.tmpl:300-307, FIXME placeholders_tagged_8bcd16133d14).
- No such bench exists. mock/research/sketches has none, and mock/benches has none.
- loru-coord claims "loru's pass_schedule bench is that bench for loru's shape" (/home/user/.shared/scripts/life/tickets-hilavitkutin.toml:3130). That is false. Every pass_schedule arm copies the string hashes into a node column on the host before the frame (FINDINGS-202611191854-p92u.md:17, :21). It benches neither shape op named, and it does not run on kaski-like data. Passing it off as op's ordered bench lets a held decision through as if it had been made.

1d. "The reverse relation of each gather held in the engine": misaligned, and the canon is silent past that.
- "gather" appears in zero canon rows (already-said: 0 of 1027). I checked the zero with "reverse", "index column", "indirect", "indices" and "join": still zero.
- the_engine_does_not_load_data_and_does_not_know_record_counts_ahead, ratified: "does not allocate storage". An engine-built reverse index is storage the engine allocates.
- a_record_is_one_data_point_in_a_column_and_the_words_entity_and_row_are_dead, ratified: the dead term "implies tabular joins; columns are independent". A gather by key across sets is a join in everything but name. Whether that row forbids it or only forbids the tuple model is op's call (C5), and it only becomes reachable after 1c's bench.

1e. "They mark the pages that fold them": misaligned if the fold is an accumulator.
- a_mixed_carrier_keeps_incremental_skip_per_fiber_and_accumulator_fibers_always_run_the_whole_range, provisional and stamped: "accumulator fibers always run over the whole range".
- The code agrees: crates/hilavitkutin/src/scheduler/run.rs:249-252 keeps accumulator fibers out of the dirty gate.

### Ask 2: ambiguous inside one record set, misaligned across sets

Rows that bear on it:
- the_schedule_is_built_once_and_reused_across_frames, ratified. This rules out building a scheduler per request.
- at_ecs_scale_the_consumer_holds_separate_schedules_per_component_set, provisional and stamped: "several pipeline runs on subsets ... The library does not need to know about ... batching ... but the design must not assume a single all-encompassing run".
- hot_morsels_go_to_p_cores_and_cold_morsels_to_e_cores_or_are_deferred, ratified: "Cold morsels ... are deferred". The engine is already licensed not to run every morsel every frame.
- Op's own words in where_the_record_count_of_a_column_lives: "indices to process needed windows to it".
- whether_the_extent_of_a_record_domain_is_fixed, answered "Both, declared per domain".

None of these decides whether a selection is a parameter of a run or something the consumer builds (call C6). The engine design also states "the frame has one meaning": every unit over every record (mock/crates/hilavitkutin/DESIGN.md.tmpl:1754-1757). A selected frame would need a second law there. That is design tier, not canon.

The cross-set half (a page needs its nodes, which need their strings) is held by 1c.

## 2. Shapes for what survives (suggestions, all treated alike)

### Ask 3 and 1a: a mark finer than the store, inside one record set

The canon-level problem is that morsels are per fiber. The per-fiber window comes from the provisional row a_fiber_s_morsel_size_is_a_window_and_the_fiber_covers_the_one_record_range_in_count_over_window_morsels, and it is live in `run`:
- The window is desc.morsel_size (run.rs:262 onward), computed as L1_USABLE 24,576 divided by the bytes written per record (crates/hilavitkutin-api/src/run_cfg.rs:104).
- So morsel k of a writer fiber and morsel k of a reader fiber cover different records. A mark keyed by morsel index is wrong across fibers. It has to be keyed by record range on the store.

S1. A change bitmap per store at a fixed granule G, cleared at the end of the frame.
- Where: scheduler runtime state reserved at build through the consumer's provider; a test per window inside the window loop of run.rs, the run_parallel worker and run_fused; a range OR on write.
- Cost: count/G bits per store; per window, a bit-range test per member read store; for Differs, a compare per record (already designed at api DESIGN.md.tmpl:562 onward).
- Needs: a rule tying G to the per-fiber windows; atomic word OR, or per-core bitsets merged at the barrier, under run_parallel.
- Fails: deferral of cold morsels (hot_morsels row, ratified), temperature history (generation_counter_activity_classifies_morsels_by_temperature, ratified) and ask 2. A frame that skips a window clears the mark for readers that never saw it.

S2. A generation per store granule plus a verified epoch per (fiber, window).
- This is the founding mechanism, and the ratified row's own mechanism field says "generation counter".
- Collapsing "last processed generation per input morsel" to one verified epoch per window gives the same decision when epochs are global and monotonic. Reasoning: a reader runs after its writers in the same frame, so a granule changed at epoch E and read at E is clean; buffered boundaries stamp E+1.
- Cost: 4 to 8 bytes per granule per store and per window per fiber; the test is a max over the covering granules.
- Supports deferral, selected frames and recency. Temperature also needs a change count or EMA per window.

S3. Pending bits per (fiber, window), pushed when a mark is made.
- A mark of range R sets the pending bits of every reader fiber's windows that meet R. Readers are known statically from the read masks. A frame clears pending only for windows it ran.
- Cost: fibers x windows bits; each mark costs one push per reader fiber.
- Supports deferral and selected frames; keeps no temperature history.

S4. Force one window per store across every fiber that touches it. This conflicts with the per-fiber window row. It is a dead route unless op overturns that row.

Pieces common to S1 to S3:
- A Reach<C, N> read widens the window's range by N records on each side, statically, with no index (api DESIGN.md.tmpl:522).
- Inside a morsel-local fiber the mark stays in the same window (records_within_a_fiber_are_independent, ratified).
- Early cutoff is reading Differs and Written for ordinary units. It is designed and not built (engine DESIGN.md.tmpl:1485, "Left for later"). It is the piece that stops a node hash that did not move from waking the next leg, and it is owed whichever shape wins.

### Ask 2 inside one set, if op makes it the engine's

D1. Run over a selection: dispatch the windows that meet the backward-closed selection and are also dirty.
- Needs S2 or S3; S1 is unsound for this.
- Closing backward over record-aligned fibers means each upstream fiber gets the union of the downstream windows' ranges, widened by any reach, rounded to its own window.
- Accumulator fibers in the cone run the whole range.
- Needs a second conformance law: inside the selection the result equals a whole frame; outside it nothing moves; the next whole frame equals the reference.

D2. A kept demand scheduler, on the consumer's side.
- Capacity is the largest page; the extent is replaced per request, which is op's "Both, declared per domain". Import the selection, run, dump.
- Needs a replaceable extent, which is not built (one count at builder.rs:176).

D3. Build per request: misaligned with the_schedule_is_built_once_and_reused_across_frames. Dead.

D4. Mark the selection as changed: recomputes clean records and pollutes the marks other readers see. loru rejects it, and it rides the open who-says question. Dead.

### Ask 1 across sets, only after op's bench, listed so the bench has its arms

G1. Conservative: a change anywhere in the source marks every window of the gathering fiber. This is what loru measured: 37.07 us at 300 pages.

G2. A forward test at the reader: per window, read the index column and test each index against the source's marks.
- No reverse relation and nothing extra held; O(N) per frame.
- loru's "a scan of every reader costs what the column mark costs" was never measured. The engine arm re-hashed 6,000 nodes and refolded 300 pages; it did not scan. Arithmetic only: 6,000 u32 indices is 24 KB sequential plus bit tests against a 530-byte bitmap, so single-digit microseconds. It is unpriced.

G3. Engine-held reverse CSR: offsets of |S|+1 and targets of |N|, both sized from the counts and so reservable at build (the_engine_never_calls_the_allocator_at_runtime_and_plan_time_allocation_happens_once_per_pipeline_lifetime, ratified). Rebuilt by a counting sort, which is a pipeline breaker (an_op_that_needs_all_its_input_records_before_output_is_a_pipeline_breaker_and_a_materialisation_boundary, ratified). This collides with "does not allocate storage".

G4. The same CSR as two declared consumer columns, maintained by a consumer unit, with the engine reading a declared relation.

G5. One scheduler per set, the host carrying the crossing: loru's `columns` arm plus record-range import marks. This is the founding shape.

Direction matters. Only the gather direction needs a reverse relation. In a fold where the child holds its parent's index (a node knows its page), the mark follows the forward index. So the page leg needs no reverse index; it needs a fold that is not an accumulator, or one that can retract.

## 3. Calls the canon leaves to op

C1. The grain of the mark on the store. Options: the reader fiber's morsel, via S3; a fixed store granule finer than any window; the ratified micro-morsel (a_micro_morsel_is_an_inner_tiling_that_is_single_level_by_default_and_activates_at_ecs_scale) reused as the grain for change; or per record for gather sources. This is the call that decides loru at its scale; see F1.

C2. Generations against bits against pending bits. This is a performance fork. a_fork_over_which_implementation_performs_better_is_registered_open_with_a_named_bench_oracle_and_ruled_from_the_findings (provisional) says to register it open with a bench. S1 also fails two ratified rows outright.

C3. Question who_says_that_a_store_changed (open). It gates a host range mark without a write.

C4. Several record sets in one schedule. Op ordered "bench both first", and that bench is not done.

C5. If shape (a) wins: is a keyed gather a forbidden tabular join? And who holds the relation: engine (G3), consumer columns (G4), nobody (G2), or the host (G5)?

C6. Is a selected frame a run parameter (D1) or a consumer construction (D2)? If D1, the second conformance law it needs.

C7. An incremental fold: a fold that is not an accumulator (a page reads a range of nodes), or an accumulator that can retract. Either way it is against a_mixed_carrier... as it stands.

## 4. Findings and where they hold

F1. At loru's site the canon's morsel grain buys little.
- The window of the node-hash fiber is 24,576 divided by the bytes written per record: 1,536 at 16 B, 6,144 at 4 B (run_cfg.rs:104; MIN_MORSEL 64 at :109). 6,000 nodes is 1 to 4 windows.
- So a one-string edit reruns 25 to 100 percent of the node column, even with an exact reverse relation at morsel grain. This is arithmetic, not measured.
- The 4.66x gap loru measured sits where morsel grain helps least, and the win grows with record count over window.
- Moves with: the bytes written per record, L1_USABLE, the record count, how the changed records are spread, and C1.

F2. The skip on dev works per unit, not per morsel.
- dirty_units seeds by store bit (dirty.rs:207). run.rs:252 masks whole fibers.
- Both host.rs:96-100 and engine DESIGN.md.tmpl:1731 say the same thing.
- Holds at 9918113f on all three run paths.

F3. A frame over a selection is unsound on any shape that clears its mark at the end of the frame, the dev shape and S1 alike. It holds whatever the grain is.

F4. The red test cannot tell the right window from a wrong one (section 0). It holds under `run`, for a single reader with the import at record 0.

F5. loru's evidence has three faults (/home/user/loru origin/dev mock/benches/FINDINGS-202611191854-p92u.md):
- It never benches the reverse relation against a forward scan.
- "A per-record mark alone does not" close the gap (:47) is reasoning, not an arm.
- Its standing is one run, one container class, and a hand-arm drift of up to 6.40 us (:30) on a 29.1 us gap.

F6. Two reds are unregistered and one known_red entry is stale (section 0).

## 5. Dead routes tried

- Keying the mark by morsel index across fibers: wrong under per-fiber windows.
- A per-request scheduler: violates the build-once row.
- Treating demand as a change mark.
- Forcing one window per store.
- Treating pass_schedule as op's bench.
- Making a page fold incremental through an accumulator.
- Gate-time evidence: the sketch 202611130617_value-form-and-domains refuses two domains in one unit, so it cannot express a gather either. It is research, not canon.

The open route that unblocks everything else is the bench op ordered for record sets, with G1 to G5 as its arms, and before it the widened red test.
