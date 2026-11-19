# H-status: The Api Revision Against The Nine Model Questions

This is roadmap step H-status. In the words of the roadmap (clause-dev `.shared/research/202611150830_stack-path/40-the-roadmap-final.md:79`), the step is "One read of the api revision (`8236de3c`) against the nine model questions of the first audit, [...] and says whether a store kind for an ordered mutation queue exists." The nine questions are the paragraphs of `mock/research/202611131215_stack-audit/DOWNSTREAM.md:13-29`. For each one this read sets out what the audit found missing, what the revision and today's `origin/dev` (`3177527a`) provide, which tenets govern it, and what the three committed sketches showed. It also answers the question that roadmap step B2 waits on, which is whether any bench of the two record-set shapes ran before the api took a form.

It rests on five sources. The first is the revision itself: `git show 8236de3c` of `mock/crates/hilavitkutin-api/DESIGN.md.tmpl` and `src/`. The second is the same tree on `origin/dev`. The third is the canon, the registry under `mock/registry/` that `mockspace.toml:14` declares as `canon_paths`, read with `cargo mock canon subjects`, `subject`, `find` and `cargo mock already-said`. The fourth is the three sketches the step names. The fifth is the design round of the revision, `mock/design_rounds/202611130617-h6mb/`. The question entries under `mock/registry/question/` sit outside `canon_paths`; they are cited as the maintainer's recorded answers and never as canon. Where a tenet is cited, its standing is given beside it, and a `held` tenet binds nothing (`mockspace.toml:526`). Claims about source are marked as claims about the code. Nothing was built, tested or benched.

Between the revision and dev, `252c5190` moved the engine-only items out of the api crate and renamed `Provider` to `BuilderInput` and `LinkedBin` to `ExtensionSurface`. None of the passages bearing on the nine questions changed in substance between the two trees. Line numbers are given for both trees where they differ, as `rev:` (the design file at `8236de3c`) and `dev:` (the same file on `origin/dev`). Two source files, `store.rs` and `member.rs`, carry the same line numbers in both.

## What the audit's own sentence about the revision refers to

`DOWNSTREAM.md:13` says "The api revision adds a record domain as a column parameter with a count per domain". The audit is dated 13 November, before the revision was committed on 14 November at 06:10. What it describes is the first writing of the doc changelist, which `202611130617-h6mb_changelist.doc.lock.md:11` records as "as posted at `288811a`". That commit cannot be reached from any ref in this clone. The locked second writing withdrew the domain form (`:129-133`, and the table at `:329`: "C3, `Domain`, `Records`, `Grid`, `Extent<D>` as a store, `Column<T, D>`, `build(storage)` | nothing; one record count per scheduler as on the trunk, several record sets wait on a bench"). So the committed revision has no record domain. The same holds for the audit's sentence at `:17` about `Reach<C, const N: Cap>`: the committed form keeps it, in one dimension only.

## The bench B2 waits on

No bench of either record-set shape was run before the api revision, and the revision took no record-domain form at all. The record-domain form existed only in three places: the first writing of the changelist, the round topic `202611130617-h6mb_topic.value-form-domains-and-the-buffered-kind.md:53-55`, and the type sketch `202611130617_value-form-and-domains`, which says "Any cost. No timing was taken and none may be read out of this directory" (`FINDINGS.md:104`). The locked changelist made the choice on the maintainer's answer and names the bench as owed (`:127-133`).

His answer is recorded in the question `how_record_sets_of_different_counts_live_together` (`mock/registry/question/records.toml:148`). The options were [several sets in one schedule, each contiguous with its own count as a plan parameter | one schedule per set, crossings through resources or accumulators | bench both first], and he answered verbatim: "Bench both first". The intent is that the api takes neither shape until both are measured.

The design says the same on both trees: `rev:327-334`, `dev:300-307`, ending with `FIXME(placeholders_tagged_8bcd16133d14): several record sets in one schedule wait on a bench of both forms`.

These searches found nothing:

1. `ls mock/benches` has no bench named for record sets, domains or counts.
2. `grep -rli "record set|record_set|record-set|record domain|record_domain|several sets|one schedule per|schedule_per|per_set|bench both" mock/benches` returned no file. The same pattern hits 14 files in `mock/research`, which is the positive control on the pattern.
3. `git log --all -i --grep` on those phrases returned two commits, `d77e7447` (May) and `f78cba5a`, and neither is about record sets.
4. `git log --all -S "Extent<D>"` finds the form only in round, research and sketch files.

The bench is still listed as owed in `mock/research/202611131215_stack-audit/BENCH-CATALOGUE.md:41-43`. Its agenda ticket, `sketch_and_bench_several_record_sets_in_one_schedule_against` (clause-dev `.shared/scripts/life/tickets-hilavitkutin.toml:605`), is open.

Holds for: commits reachable from this clone's refs on 2026-11-19, `mock/benches` and `mock/research` on `origin/dev` `3177527a`. It does not cover `288811a` or any unfetched branch.

## The nine questions

### Several record sets

The audit found one record count for every column in a schedule (`DOWNSTREAM.md:13`).

The revision and dev provide no new form. A scheduler holds the one count `build` takes, `Column<T>` has no second parameter, an accumulator's capacity is that same count, and "A consumer whose record sets differ in count builds a scheduler for each set" (`rev:327-334`, `dev:300-307`). In the code, `RunCfg::record_count` is the one figure (`src/run_cfg.rs:31,136-160`). The locked changelist says the one-scheduler-per-set route "is his option b as it works today and is not a choice of it" (`doc.lock.md:129`).

The canon binds this in four tenets: `the_engine_does_not_load_data_and_does_not_know_record_counts_ahead` (ratified, `tenet/records.toml:82`); `storage_is_contiguous_and_a_fiber_s_columns_share_one_arena` (ratified, `:22`); `at_ecs_scale_the_consumer_holds_separate_schedules_per_component_set` (provisional, stamped, `:215`), which says "the design must not assume a single all-encompassing run"; and `record_counts_are_known_when_loaders_complete_and_cascade_downstream` (provisional, stamped, `:238`).

The maintainer's answers bear on it too. On `where_the_record_count_of_a_column_lives` (`question/records.toml:4`) he answered "Isn't the intent a contiguous per prototype and indices to process needed windows to it?". On `whether_the_extent_of_a_record_domain_is_fixed` (`:45`) he answered "Both, declared per domain", which the changelist reads as waiting on the later answer (`doc.lock.md:127`). On `how_record_sets_of_different_counts_live_together` (`:148`) he answered "Bench both first".

The sketch `202611130617_value-form-and-domains` showed domains on the column marker with their own extents as types: one domain per unit folded from the lists, a unit over two domains refused, and two extents of different counts in one world (`FINDINGS.md:3,41,43`, fixtures 02 and 03 at `:76-77`). Its outcome is WORKS. It took no timing and has no planner (`:100,104`). The other two sketches do not bear on this question.

Status: unanswered.

Holds for: `hilavitkutin-api` at `8236de3c` and at `3177527a`, any consumer, any worker count.

### Growth

The audit found columns sized at plan time, re-reserving undefined, and no grow operation on the memory provider (`DOWNSTREAM.md:15`).

The revision and dev provide no growth operation. The `ColumnStorage::reserve` contract says "Re-reserving an existing `id` resizes it" (`src/storage.rs:46-47` on both trees; `rev:794`, `dev:772`). In the code, the shipped `ArenaColumnStorage::reserve` frees the old block and allocates afresh (`mock/crates/hilavitkutin-providers/src/storage.rs:68-80`), and the spike `202611131951_load-time-units-seat02/s4-growth/README.md:8` records the effect: four written records read back `[0, 0, 0, 0]` at the same base. `MemoryProviderApi` has `allocate`, `deallocate` and `protect` and no grow (`src/platform.rs:49` on dev). The OS provider's `protect` is an empty body under a FIXME (`mock/crates/hilavitkutin/src/platform/os.rs:93-96`, code). `Accum<T>` grows within a capacity fixed at build (`dev:217-220`), which covers emission and does not grow a column.

The canon has `container_capacity_lives_at_type_level_and_a_consumer_needing_more_picks_the_bigger_type` (provisional, stamped, `tenet/principles-3.toml:49`), "There is no runtime resize and no runtime growth"; `a_need_for_a_growable_container_means_the_shape_is_wrong_and_is_reshaped_toward_an_emitter_iterator_or_bounded_store` (provisional, stamped, `principles-2.toml:200`); `morsel_boundaries_are_stable_unless_the_record_count_changes` (ratified, `records.toml:101`); and `the_consumer_provides_the_backing_memory_of_a_column` (ratified, `stores.toml:149`).

The question `whether_column_buffers_are_allocated_at_build_to_capacity_or_per_run_once_the_record_count_is_known` (`question/memory.toml:35`) is open. The maintainer's "Both, declared per domain" on the extent question names a capacity fixed at build with an extent that can be replaced, and it waits with the record sets.

The three sketches do not bear on growth. The s4 spike is std-backed and is not among the three; its reserve-and-commit arm is green (`s4-growth/README.md:10`).

Status: unanswered. The contract sentence on resizing is stated in design and contradicted by the shipped provider, which is a claim about the code.

Holds for: both trees, any target. The s4 spike holds for x86_64 linux under overcommit 0 only.

### Reading a neighbour

The audit found no notion of reading another record of the same column, and noted that the revision's `Reach` had "no example and no sketch yet" (`DOWNSTREAM.md:17`).

The revision and dev provide `Reach<C, const N: Cap>` as a member of a read list (`rev:527`, `dev:505`; `src/member.rs:98-122`). Its value is a `Window` with `here`, `before::<K>` and `after::<K>`, where `K > N` does not compile and the read past either end is nothing (`dev:528-536`; `src/member.rs:192`). The reach counts records along the column's one sequence. In the design's words, "Axes, the shape of a grid, a neighbour in two dimensions and the value read past an edge are geometry, which belongs to an opt-in crate over this read and is not in this crate" (`dev:536-541`, `rev:558-563`). A unit reading a plain `Column` with a reach may not write that column; this is refused when the scheduler is built, and writers finish their range before the reader starts (`dev:543-550`). `Reach<Buffered<T, M>, N>` reads the frozen side, and the reader may also be the writer (`dev:552-558`). The positional accessors have no reach (`dev:560`).

In the code, the engine holds the rule at build (`mock/crates/hilavitkutin/src/scheduler/reach_rule.rs:1-25`) and cuts the accumulator walk ahead of a reach reader that sits behind a writer (`scheduler/worker.rs:179-181`). The frozen units `Spread` (`Reach<Field, WIDE>`) and `Smooth` (`Reach<Pool, ONE>`) use it (`mock/crates/hilavitkutin-frozen-authors/src/units.rs:83,216`); a grid's north and south neighbours are reached as records a row width away.

The canon gives `records_within_a_fiber_are_independent` (ratified, `tenet/records.toml:63`) and `the_only_cross_record_surface_is_the_accumulator_append_path` (provisional, stamped, `records.toml:266`), whose text says "the only surface that crosses records is the accumulator append path". The design now has a second cross-record surface in `Reach`, so that tenet and the design disagree. Under the ladder the design is wrong where it disagrees with a binding tenet. The tenet is provisional and was taken from the engine design of June (`source = mock/crates/hilavitkutin/DESIGN.md.tmpl:325-332`), so one of the two has to move, and that call is not this read's to make. The question `whether_hilavitkutin_owns_geometry` (`question/records.toml:86`) is answered "Opt-in crate over the api". No such crate exists among the fourteen.

The sketch `202611130617_value-form-and-domains` showed neighbour reads as `Near<C, O, E>` with offsets on several axes: an offset past the domain's reach is refused (fixture 01), and a neighbour read of a column written in place is refused (fixture 07, `FINDINGS.md:45,75,81`). Its outcome is WORKS. The revision kept the one-axis reach and the refusal and moved the axes to the opt-in crate. The other two sketches do not bear on this question.

Status: partly answered. A one-dimensional reach is answered in the api. Missing are the opt-in geometry crate (axes, offsets on several axes, the value past an edge) and the agreement between the design and the provisional tenet above.

Holds for: the value form of a unit only (`dev:517-520`, "reached through the value function alone"), not the positional accessors.

### Reading last frame's value

The audit found no buffered store kind, disagreement about its form, no sketch, and the maintainer asking for a schedule for two writers (`DOWNSTREAM.md:19`).

The revision and dev provide `Buffered<T, M = Snapshot>` with the sealed modes `Snapshot` and `Exchange` (`rev:152`; `dev:125-129,134-148`; `src/store.rs:213,220,229,243`). Every read sees the frozen side, which is the store as the pass began; every write lands on the live side; after the last unit of a pass, the live side of every changed buffered store becomes its frozen side (`dev:134-139`). Two writers of one buffered column are put in sequence as two writers of any store are (`dev:681-685`; the general rule is at `dev:909-913`), which is the maintainer's "Schedule should exist for this specifically" (`question/ordering.toml:45`). A buffered column is registered from a start value through `Started<S>`, with `start()` resolving the trivial constructor before the default (`dev:150-208`; `src/store.rs:339,363,385`).

Two FIXMEs stand on both trees: `ecb940c91d26`, "which mode pays at which size waits on its bench" (`dev:720`, `rev:742`), and `24fa3df63a9d`, "a read and a write of a buffered column by position are not designed" (`dev:723`, `rev:745`). The design also names the one-copy alternative, appending changes and applying them at the close (`dev:725-735`). A single value held in two sides is out of scope (`dev:737-740`).

The canon binds little here. `an_expert_memo_says_hilavitkutin_s_columns_are_double_buffered_for_the_pass_boundary` is held (`tenet/stores-2.toml:283`) and binds nothing. `a_sketch_of_november_2026_holds_every_lane_of_a_frame_twice_committed_and_staged` is provisional and stamped (`stores-3.toml:216`). The loimu tenets `loimu_double_buffers_record_data_and_its_readers_get_the_frozen_snapshot` and `loimu_s_swap_buffer_is_rated_directly_portable_as_a_double_buffering_primitive` are provisional, stamped, at `downstream.toml:42,61`. `a_storage_heritage_note_lists_the_per_morsel_snapshot_buffer_s_double_buffering_as_superseded_by_generation_counters` is provisional and stamped (`change-2.toml:424`). Two questions are open, `whether_hilavitkutin_s_own_columns_are_double_buffered` (`question/stores.toml:114`) and `who_rotates_a_buffered_store` (`question/change.toml:167`). No binding tenet says that hilavitkutin buffers a column across the pass boundary.

All three sketches bear on this question. `202611130617_value-form-and-domains` showed a frame-buffered kind with a front, a back and a swap at the close; two runners visiting records in opposite directions leave the same stores, and the mutant that writes the front at once turns all three tests red (`FINDINGS.md:47,55-69`). Its outcome is WORKS, and it did not show the refusal of a second writer, which the design later replaced with sequencing (`:96`). `202611132225_buffered-start-by-type` showed `start()` following the element type through the constructor tier and then the default tier, with no specialisation (`FINDINGS.md:4,50`); WORKS, leaving open the null-bit tier and the choice made at a generic call site (`:54-58`). `202611132325_column-start-one-carrier` showed one carrier `Started<S>` for both the plain and the buffered column (`FINDINGS.md:4`); WORKS with one refusal, `into_inner` cannot be a `const fn` without `const_destruct` (`:15-21`).

Status: answered in the api.

Holds for: the value form, `M` in `{Snapshot, Exchange}`, any worker count as the design states it. Positional access to a buffered column is not designed, and the cost of each mode has not been measured.

### Emission at scale

The audit found the accumulator's admissible input depending on the worker count (`DOWNSTREAM.md:21`).

The revision and dev provide `Accum<T>`, which reserves a capacity at build, resets at the top of a frame and grows by append (`dev:126,217-220`; `src/store.rs:175`). The capacity is the record count (`dev:302-304`). A close unit reads the appended values through `AccumReaderApi::each` (`dev:631-656`; `src/context.rs:243-251`). The run's verdict on overflow is `Refuse` or `Assert` on the run configuration (`src/accum_overflow.rs`).

The engine design on dev regions the buffer per core (`mock/crates/hilavitkutin/DESIGN.md.tmpl:615-627`). It says `wanted` gives the same `AccumRefused` at every worker count when an emitter asks more than the reserved capacity (`:686-691`), and that the slice is scoped "to the at-most-one-append-per-record convergence pattern", a frame asking more than one append per record on one core's slice being "refused on more than one worker and accepted on one" (`:694-700`).

The canon has `an_accumulator_is_a_column_written_by_append_and_a_fourth_store_marker` (held, `tenet/stores-2.toml:176`), `an_accumulator_append_saturates_at_the_reserved_capacity_which_equals_the_record_count_at_build` (held, `:221`), `an_append_past_an_accumulator_s_capacity_is_counted_and_never_written_and_the_refusal_is_sticky_for_the_frame` (provisional, stamped, `stores-3.toml:115`), `each_core_appends_into_its_own_region_of_an_accumulator_and_the_main_thread_merges_the_regions_in_order` (provisional, stamped, `:89`), `the_refusal_of_an_accumulator_is_read_at_the_close_and_is_the_same_value_at_every_worker_count` (provisional, stamped, `:134`), and `under_the_parallel_run_each_core_s_accumulator_region_starts_after_the_leading_records_and_keeps_its_own_refusal` (provisional, stamped, `:198`). Both tenets that state the store and its capacity are held, so they bind nothing.

The sketch `202611130617_value-form-and-domains` showed an `Accum` member whose value is `Option<T>` and which a value unit cannot read (fixture 10, `FINDINGS.md:37,84`); its appends were compared sorted (`:55`). The other two sketches do not bear on this question.

Status: partly answered. The store, the refusal and the read at the close are in the api. Missing are a capacity independent of the record count, and admission that does not depend on the worker count for more than one append per record, which the engine design names as a follow-up.

Holds for: the engine design and api on `3177527a`. The worker-count independence of the refusal holds only for at most one append per record.

### Determinism

The audit found accumulator results depending on the core count, an ungated N-way split, and no wasm arm or cross-target digest (`DOWNSTREAM.md:23`).

`8236de3c` added `hilavitkutin-conformance`, a one-thread reference runner with an FNV-1a `Fold` over 64-bit words and the laws `model`, `split` and `isolation`; `split` holds when "Every stream equals the first", across entry, worker count, morsel size and visiting (`mock/crates/hilavitkutin-conformance/DESIGN.md.tmpl:146-177`). The same commit added `hilavitkutin-frozen-authors`, whose tests hold `run_parallel` and the phase loop to the reference at one, two and four workers, "the appends are compared sorted" (`tests/authors.rs:250-252`, code), and the tool `mock/tools/cross-target`, which compares the frozen units' digest stream on `wasm32-unknown-unknown` under node with the host's, run through the reference runner and not the engine (`src/lib.rs:1-27`, code).

In the code, the reach rule gates the split ahead of a reach reader (see the neighbour read). The api states that the sequence of appended values after a parallel run "depends on the worker count", and that a close unit whose result must not depend on it applies commuting changes or "carries in each change what decides between two changes to one record" (`dev:653-656`, `rev:675-678`). The engine design says the merged prefix is byte-identical to the one-core run when nothing was refused, within the one-append-per-record scope (`hilavitkutin/DESIGN.md.tmpl:682-694`); the api contract does not promise that.

The canon has, all ratified, `work_stealing_is_not_the_default_and_the_consumer_may_provide_a_stealing_executor` (`tenet/execution-3.toml:231`), `ordering_is_guaranteed_by_construction` (`ordering.toml:114`), `a_unit_declares_whether_it_is_commutative` (`ordering.toml:93`), `two_writers_of_one_store_are_ordered_by_hint_priority` (`ordering.toml:43`) and `records_within_a_fiber_are_independent` (`records.toml:63`). The canon is silent on determinism across targets: `cargo mock canon find determinis` and `find "worker count"` return no tenet on it.

The sketch `202611130617_value-form-and-domains` held two runners that visit records in opposite directions to the same stores (`FINDINGS.md:49,55`). The sketch `202611130039_wasm-digest-under-node`, not among the three, gives the native digests under node with WORKS (`FINDINGS.md:5`). The other two of the three do not bear on this question.

Status: partly answered. The instruments and the split law exist, held at one, two and four workers. Missing are runs at three workers and at the core maximum, an append sequence the contract fixes, and the engine itself, as opposed to the reference runner, on a second target. No committed run of `cross-target` was found under `mock/research` or `mock/benches`; five research notes name the tool.

Holds for: the frozen units on 48 records (`values.rs:18` sizes them), worker counts 1, 2 and 4, with appends compared sorted.

### Fixed point

The audit found that kaski's `Fixed<FRAC>(i64)` with an `i128` multiply does not vectorise, and that the performance story has not accounted for it (`DOWNSTREAM.md:25`).

The revision and dev provide nothing specific. Any `Copy + 'static` type is a column value (`src/column_value.rs:34`, code), so kaski's carrier is storable. The api's backlog says the choice between float and `UFixed` is one of "PRECISION/determinism/FPU-availability" and not of speed (`mock/crates/hilavitkutin-api/BACKLOG.md.tmpl:257-264`). The bench `mock/benches/fxpmul_strategy` measures an `i128`-intermediate multiply against an `i64` truncation at 64 to 16384 points, with results under `mock/benches/results/fxpmul_strategy/`; its numbers were not read here, and it times the multiply alone and not a lane through the engine.

The canon has `each_column_has_its_own_stride_and_a_value_is_at_most_sixteen_bytes` (ratified, `tenet/records.toml:42`), `any_copy_static_type_of_at_most_sixteen_bytes_is_column_storable_through_a_blanket_implementation` (held, `stores.toml:215`), and `the_design_is_the_simd_contract_and_llvm_auto_vectorises_when_the_eight_killers_are_absent` (ratified, `principles.toml:493`). `cargo mock canon find "fixed point"` returns no tenet. The maintainer answered `whether_a_lattice_lane_may_be_narrower_than_64_bits` (`question/downstream.toml:44`) with "Upstream on hila it should be entirely generic and configurable by user".

The three sketches use bare numbers in place of arvo's types (`202611130617/FINDINGS.md:108`) and do not bear on this question.

Status: unanswered.

Holds for: both trees. The fixed-point lane bench the catalogue asks for is not under `mock/benches`.

### Content as data

The audit found that an extension's work units cannot join a schedule, and that nothing in the record addresses it (`DOWNSTREAM.md:27`).

The revision and dev provide a typestate slot for extension instances: `LinkedBin<T: ?Sized>` (`rev:336-358`), renamed to `ExtensionSurface<T: ?Sized>` (`dev:309-335`). Units declare reads and writes against it, the typestate proves coverage at `.build()`, and "Runtime instance population happens in `hilavitkutin-extensions` and `hilavitkutin`" (`dev:326-330`).

The canon has `static_composition_means_every_unit_is_registered_through_the_builder_and_no_schedule_is_dynamic` (ratified, `tenet/principles.toml:165`), `plugin_type_access_to_columns_comes_from_has_shape_and_erased_column_from_loimu` (ratified, `extensions.toml:4`), and `a_pipeline_is_extended_by_new_records_in_existing_columns_or_by_a_facade_unit_over_a_plugin` (provisional, stamped, `extensions.toml:20`), which says that new code "is reached through a statically registered facade unit that calls it across the capability interface, off the per-record path".

None of the three sketches bears on this question. The std-only spikes of round `202611131951` do: `s2-extension-surface-unit` is green on admission and refusal at load, with the deliberate red that admission checks the declaration and not the code behind it, and `s3-content-closed-forms` is green (`seat02/s2-extension-surface-unit/README.md:8-14`, `s3-content-closed-forms/README.md:8-13`).

Status: partly answered. The static slot is in the api, and the facade route is a provisional tenet. Missing are the engine-side population of an `ExtensionSurface`, a facade unit over a loaded extension under the engine, and any sketch of either against the shipped crates.

Holds for: both trees. The spikes hold for std on x86_64 and use no hilavitkutin crate.

### Persistence

The audit found that the cold store opens without reading a manifest and flushes without writing (`DOWNSTREAM.md:29`).

The revision and dev leave the code unchanged: `ColdStore::open` reads no manifest (`mock/crates/hilavitkutin-persistence/src/cold_store.rs:31-33`), `flush` "writes nothing and reports success" (`:59-61`), `load` always returns `Missing` (`:68-70`), and `snapshot` copies nothing (`:77-79`). All four are claims about the code. `8236de3c` did not touch the crate; `62453a65` and `266650fb` did after it. The api design says "The persistence boundary (`evict` / `inject`, R2) folds onto `ColumnStorage` [...] it is not part of the core contract here" (`dev:820-822`, `rev:842-844`).

The canon has `column_data_moves_out_of_and_into_the_engine_through_evict_dump_and_inject_import` (ratified, `tenet/persistence.toml:4`), `the_cold_store_provides_the_flush_mechanism_and_the_consumer_decides_when_to_flush` (provisional, stamped, `:360`), `the_cold_store_keeps_a_manifest_a_table_file_per_table_and_a_runtime_string_file_in_one_data_directory_per_consumer_and_the_consumer_adds_its_own_files` (provisional, stamped, `:295`), `dirty_data_is_flushed_before_it_is_evicted_so_the_hot_store_is_reconstructable_from_the_cold_store_files` (provisional, stamped, `:405`), and `the_engine_reaches_persistence_only_through_a_resource_the_consumer_wires_and_never_names_the_persistence_crate_s_types` (held, `persistence-2.toml:93`).

The three sketches do not bear on persistence.

Status: unanswered.

Holds for: `hilavitkutin-persistence` on `3177527a`, any target.

## The ordered mutation queue

No store kind for an ordered mutation queue exists in the revision or on dev. The five store markers are `Resource`, `Column`, `Buffered`, `Accum` and `Virtual` (`dev:122-127`), and none of them carries a sequence key or a drain in a fixed sequence.

The nearest form is `Accum<T>` written by ordinary units and read by a unit scheduled on `ScheduleEnd` through `AccumReaderApi::each`, as `ApplyHeat` in the design does (`dev:595-669`). The api says that after a parallel run the values "come region by region, one region for each core, so their sequence depends on the worker count", and leaves the deciding key to the consumer, who "carries in each change what decides between two changes to one record" (`dev:653-656`, `rev:675-678`). The frozen-authors tests compare appends sorted (`tests/authors.rs:250-252`). The engine design's byte-identity claim covers only one append per record (`hilavitkutin/DESIGN.md.tmpl:682-694`).

The maintainer's own words on mutation are in clause-dev `.data/op-responses/202611131254_op-on-your-guys-are-sitting-waiting-for-you-again.md:100`, quoted verbatim:

> This is for the kaski impl and use of hilavitkutin. Don't thin natively need that. Unless we do an abstraction and contract for the mutation behaviour of kaski, then I suppose we'll need the closers. But even then I think it's all about the data: if the data is written in place, then that work unit is special case. Intended case in the mutation paradigm is writing through queues. Now that can be either a second write buffer OR it can be queued up ops that drain verctoeised and optimized at the close or whatever. All depends what part of that is in hila and what is a kaski impl of hila or work on top of its work unkts

The intent, in this read's words: writing through queues is the intended mutation case, either as a second write buffer or as queued operations drained at the close. Where the line between hilavitkutin and kaski falls is left open. The rest is kept for provenance. No tenet or question entry carries this text: `cargo mock already-said queue` finds six tenets, none on a mutation queue.

The canon is silent on a sequenced mutation queue. `cargo mock canon find queue` returns four tenets, about thread start, ikiuni's frame close, the topological sort's fixed queue and thread spawn. The tenet `each_core_appends_into_its_own_region_of_an_accumulator_and_the_main_thread_merges_the_regions_in_order` (provisional, stamped, `tenet/stores-3.toml:89`) says the merge preserves append sequence within regions, and nothing more.

The three sketches do not test a sequence key. The first one's appends were compared sorted "since the sequence of appends is not something the value form promises" (`202611130617/FINDINGS.md:55`). The prior work that does exist is the std spike `202611131951_load-time-units-seat01/c7-packed-queue-drain/README.md:18-32`: one drainer loses nothing, two drainers split by queue position lose writes unless they use `fetch_or`, and an overwrite field "would need a compare-and-swap loop and an order between ops on the same record, which this spike does not test". The hilavitkutin ticket lane holds no ticket for a keyed queue (searched `tickets-hilavitkutin.toml` for "order key", "mutation queue", "keyed").

Status: partly answered. The queue and the drain at the close exist as `Accum<T>` with a `ScheduleEnd` unit. Missing are a store kind or member that fixes the drain's sequence independent of the worker count by a key, a parallel or vectorised drain, and any tenet on either.

Holds for: both trees, any worker count above one. At one worker the sequence is the append sequence (`dev:652-653`).

## Summary

| Question | Status | Tree read |
|---|---|---|
| Several record sets | unanswered | both; identical |
| Growth | unanswered | both; identical |
| Reading a neighbour | partly answered: geometry crate, tenet conflict | both; identical in substance |
| Reading last frame's value | answered in the api | both; identical in substance |
| Emission at scale | partly answered: capacity tied to count, multi-append by worker count | both; engine design read on dev |
| Determinism | partly answered: 3 and max workers, append sequence, engine off host | revision added the instruments; dev read |
| Fixed point | unanswered | both |
| Content as data | partly answered: engine-side population, facade unit | both; `LinkedBin` renamed `ExtensionSurface` |
| Persistence | unanswered | both; code read on dev |
| Ordered mutation queue | partly answered: no sequenced kind, no tenet | both; identical |

## What is still owed

Several record sets: the sketch and bench of both shapes under agenda ticket `sketch_and_bench_several_record_sets_in_one_schedule_against`, on the data described at `BENCH-CATALOGUE.md:41-43`.

Growth: a sketch of an extent that can be replaced up to a capacity fixed at build, against the real `ColumnStorage`. A ticket for the contract sentence at `src/storage.rs:46-47` against the provider's free-and-reallocate. A question for the maintainer on how `container_capacity_lives_at_type_level_and_a_consumer_needing_more_picks_the_bigger_type` stands beside his "Both, declared per domain". The open question `whether_column_buffers_are_allocated_at_build_to_capacity_or_per_run_once_the_record_count_is_known`.

Reading a neighbour: a design round for the opt-in geometry crate. An amendment or a question on `the_only_cross_record_surface_is_the_accumulator_append_path` against `Reach`. The bench of a neighbour read's generated code that `202611130617/FINDINGS.md:104` names.

Reading last frame's value: the mode bench behind FIXME `ecb940c91d26`. A design for positional access to a buffered column (FIXME `24fa3df63a9d`). Answers to the open questions `who_rotates_a_buffered_store` and `whether_hilavitkutin_s_own_columns_are_double_buffered`. A binding tenet for the buffered kind, since the expert-memo tenet is held.

Emission at scale: the engine follow-up for regions sized to the worst case per region. A decision on whether a capacity independent of the record count enters the api. Tenets for the store itself, since its two capacity tenets are held.

Determinism: frozen-authors runs at three workers and at the core maximum. A committed run of `cross-target`. The engine, not only the reference runner, on a second target.

Fixed point: the fixed-point lane bench through the engine that the catalogue asks for. A tenet, since the canon is silent.

Content as data: a sketch of a facade unit over a loaded extension under the engine, and of how an `ExtensionSurface` is populated.

Persistence: the mmap-backed open, flush and load behind `hilavitkutin_persistence_5_unmarked_placeholders`, and a test that a flushed store is what a later load finds.

Ordered mutation queue: a question for the maintainer on which part of the mutation paradigm is hilavitkutin's and which is kaski's. Then a sketch of a keyed drain held equal at 1, 2, 3, 4 and the core maximum of workers, compared unsorted, with a twin keyed by arrival that must fail (H-proof's premise). A tenet once either is settled.
