Source: kaski channel (coord/kaski of clause-dev), att/20261113T095724595749243-hv-canon-registry-b7b081/ANSWERS.md

# The eight questions, answered from the record

This note answers the eight questions of the brief from hilavitkutin's own
record. Every answer says what the record decides, quotes the passages it
rests on with their path and lines, names the registry rows that carry them,
and then says what the record leaves undecided. Nothing here is a ruling. A
sentence that is mine and not the record's is marked as a reading.

The precedence the answers use is the record's own, from amendment A2-1: the
founding consolidation spec first, then the amendments op ruled or a bench
decided, in date order with the later one winning, then the standalone spec
as a reading aid, and everything else as intermediate work that is never
canon. Rows from the first tier and from op's rulings carry `origin =
"stated"`. Rows from founding-round topics, research notes, agent-written
rounds, the reading aid and the design templates carry `origin = "proposed"`
and name who wrote them. Op's answers of 13 November 2026 are `question` rows
with their options and are quoted here from `OP-ANSWERS.md` beside this file.

Paths are relative to `mock/`. The founding spec is
`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`
and is written SPEC below. Rows are cited as `tenet::<id>` and
`question::<id>` and live under `mock/registry/`.

## How far the read had come when this was written

The answers rest on what had been read by 13 November 2026, 10:15 UTC. Read
whole and first hand: the founding spec, amendments A1 and A2, the
unified-engine amendment, the resource storage addendum, the governance
seed, both headers of the standalone spec and the passages quoted from its
body, and op's answers. Read whole by the readers, with every quote checked
against the file: the seven founding-round topics, the founding research
notes, the research notes and roadmaps from May to July 2026 with the
`pre_seed` set, the seed, and the design templates of the workspace and of
every crate. Not yet read when this was written: the design rounds after
the founding one, apart from the three quoted below, the sketches' findings
files, the research imported from polka-dots and saalis, the readme
templates, the bench findings, and the rounds recovered from
`origin/recover/unlanded-rounds-and-benches`. `LEDGER.md` lists every file
with its state. Where an answer says the record has no passage for
something, that is a statement about the part read, and the unread part can
still hold one. The answers are revised if it does.

## 1. Reading the value a store held at frame start, and what the counters are for

### What the record decides

Hilavitkutin has two access modes, read and write, and a write implies a
read. Ordering is the data flow: a unit that reads a store runs after the
unit that writes it, so inside one pass a reader sees what the writer made in
that pass. The founding spec has no mode that reads the value from before the
write, and no buffered store.

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:589-595`

> **Ordering IS the data flow (T1 §10.6):**
> - DAG edges from Write→Read column overlap
> - Write-write ambiguity resolved by scheduling hint priority (Urgency,
>   Divisibility, Significance). Tie-break: most deps first, then
>   deterministic fallback. Supersedes T6 L2637 `order` field.
> - No triggers, no emits — data access patterns fully capture ordering
>   (T1 decisions #17, #25)

The founding round took up exactly this question for loimu, the downstream
user that double and triple buffers, and resolved it. The spec cites the
resolution at line 678 for the rule that a write implies a read.

`design_rounds/202604200055/202603141800_topic.hilavitkutin-core-design.md:643-661`

> **Q6. Access mode granularity — RESOLVED**
>
> Resolved by Q2: two modes only. Read and Write. Write implies Read.
>
> loimu's seven access modes (Pinned, Immediate, MutImmediate, Sample,
> Post, Approx, Defer) control ordering, freshness, and determinism
> within the double-buffered view system. These are loimu-specific
> concerns tied to its temporal prediction, rotation groups, and
> frame generation. polka and saalis need none of this.
>
> hilavitkutin exposes Read and Write. The DAG uses these for edge
> construction. The scheduler uses these for conflict detection. loimu
> maps its seven modes to Read/Write at the hilavitkutin boundary and
> implements the richer semantics in its own layer.
>
> If hilavitkutin later needs a third mode (e.g. ReadExclusive for
> consumers that need guaranteed exclusive read access), it can be
> added without changing the WorkUnit trait — it would be a new store
> wrapper type like `Exclusive<Resource<T>>` in the AccessSet.

So the frozen read is loimu's, built in loimu's layer over a two-mode
hilavitkutin. The loimu research the founding round read shows how: each
field is several columns, and the reader of the frozen one and the writer of
the live one touch different stores.

`design_rounds/202604200055/202603141800_research.loimu-dag.md:299-301`

> | `Pinned<T>` | Read | Frozen snapshot | No | Yes | Default for reads |
> | `Immediate<T>` | Read | Current frame | Yes | Yes | Opt-in |
> | `MutImmediate` / `&mut T` | Write | Current frame | Yes | Yes | Default for writes |

`design_rounds/202604200055/202603141800_research.loimu-storage.md:190-193`

> are stored in the record backend. Views group columns; each column is a
> SlabColumn. Non-temporal fields get 2 SlabColumns (frozen + live). Temporal
> fields get 3 (old + current + projected). Spring temporal fields get 4 (old +
> current + projected + velocity).

`design_rounds/202604200055/202603141800_research.loimu-storage.md:826-828`

> Record data in views is double-buffered (frozen + live) by default. Behaviors
> reading `nodes M` get the frozen snapshot (parallel); behaviors writing `nodes M`
> get the live buffer (exclusive).

`design_rounds/202604200055/202603141800_research.loimu-storage.md:1135-1137`

> loimu layers temporal columns, spring dynamics, reel backends, cadence policies,
> signal arenas, and seven access modes on top of these primitives. Those layers
> stay in loimu. hilavitkutin provides the foundation they build on.

Read in hilavitkutin's own terms, a unit that needs the frame-start value of
a store while the next value is produced declares two stores: it reads the
one holding the previous value and writes the one holding the next. No
write-then-read edge joins them, so the scheduler puts no order between the
reader of the old and the writer of the new, and the consumer rotates the
two between passes. This sentence is a reading; the passages above are what
it rests on.

The counters are for knowing whether something changed, never for holding an
older value. The generation counter is per morsel and drives the skip. The
version stamp is per store and tells which stores a unit really modified.
The temporal extent of a morsel is how many frames its cached result stays
valid.

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:861-865`

> **Change detection (T3 decisions #57-58, R2 evict/inject):**
> - Per-morsel generation counter — bump on write, skip if unchanged
> - Propagates through DAG — unchanged root skips all transitive dependents
> - Transforms pipeline from batch processor to incremental processor
> - evict/inject APIs (R2) handle persistence to consumer storage

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:2139-2145`

> **Version stamps (T5 §Q10, T6 §Q10):**
>
> Parallel arrays (SoA): separate contiguous `[u64; MAX_STORES]` for
> "before" and "current" stamps. Capture before-versions before WU runs,
> compare after. Changed stores = bitmask of actually-modified columns.
> Increment only on completion — no partial writes (domain 08). 2-5 u64
> per unit stamp view, packed in one cache line.

`design_rounds/202604200055/202603121800_topic.dispatch-and-optimisation.md:907-915`

> **Mechanism: per-morsel generation counter.**
>
> Each morsel has a generation number — a monotonic counter that
> increments whenever any row in the morsel's range is modified. The
> ColumnStorage (or a separate change-tracking layer) bumps the
> generation when a write occurs.
>
> Each chain records the generation it last processed for each of its
> input morsels. On the next frame:

The one old value the counters do give is a consequence of the skip: a
morsel that is skipped keeps the output it had, so its previous frame's
result is still there to read.

Two neighbouring mechanisms are sometimes taken for a frame-start read and
are something else. The stack-local resource snapshot before the morsel loop
is there so the compiler keeps resource scalars in registers. The per-trunk
snapshot of shared read columns is a cache measure on ARM.

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:781-786`

> **Shared read columns between trunks (T6 L3635-3670):**
> - Approach 1 (snapshot-to-local): copy shared columns per trunk at
>   phase transition. 1.6% overhead. Simpler, no cross-core contention.
> - Approach 2 (aligned morsel sync): zero copy, both trunks read same
>   data. Works on unified-memory ARM (M1). Complex on NUMA x86.
> - Recommendation: approach 1 for ARM, approach 2 for x86 NUMA.

For a resource, the swap of a replaceable value is not specified yet. Op
commissioned a round for it on 19 July 2026.

`research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md:88-94`

> Op's direction (2026-07-19): the missing swap spec is researched and designed explicitly, not
> picked from options. A design round is commissioned to produce the explicit `Replaceable` swap
> specification under the bench-decided layout (blob record, erased addressing, collection
> elements in collection columns), covering `replace_value`, `replace_resource`, the
> `PlanAffecting` interplay (A2-3), Seq/Map element ownership, dirty propagation, and quiescence.
> The same round corrects the mis-grounded artifacts. Until it lands, `replace_value` stays
> mark-dirty-only and the catalogued test stays red.

Rows: `tenet::ordering_is_the_data_flow`,
`tenet::a_write_declaration_implies_a_read`,
`tenet::a_writer_precedes_every_reader_of_what_it_writes`,
`tenet::loimu_keeps_its_seven_access_modes_and_maps_them_to_read_and_write_at_the_boundary`,
`tenet::loimu_double_buffers_record_data_and_its_readers_get_the_frozen_snapshot`,
`tenet::loimu_s_layers_stay_in_loimu_and_hilavitkutin_provides_the_foundation`,
`tenet::loimu_s_swap_buffer_is_rated_directly_portable_as_a_double_buffering_primitive`,
`tenet::change_detection_is_per_morsel_and_skips_what_is_unchanged`,
`tenet::version_stamps_tell_which_stores_a_unit_actually_modified`,
`tenet::a_morsel_is_reprocessed_only_when_its_generation_is_past_the_one_last_processed`,
`tenet::the_temporal_extent_of_a_morsel_is_how_many_frames_its_cached_result_stays_valid`,
`tenet::the_engine_owns_the_generation_counters_and_the_skip_logic`,
`tenet::the_fiber_dispatcher_snapshots_resources_to_a_stack_local_array_before_the_morsel_loop_and_writes_mutable_ones_back_after`,
`tenet::shared_read_columns_between_trunks_are_snapshotted_per_trunk_on_arm_or_read_in_aligned_sync_on_x86_numa`,
`tenet::the_swap_of_a_replaceable_resource_is_to_be_specified_by_a_commissioned_round`,
`tenet::a_swap_of_a_resource_value_is_legal_only_between_frames_and_is_observed_from_the_next_run`,
`question::how_a_unit_reads_the_value_a_store_held_at_frame_start`.

### What the record does not decide

Op's answer of 13 November says the counters were designed for exactly this,
that known downstream users triple buffer, and that the spec should answer it
at least in intent. The record agrees on the downstream users and on the
intent that a consumer can buffer two or three deep over hilavitkutin. In the
part read it does not give the counters that job: no passage has a counter or
a generation select or hold an earlier value of a store.

One passage in the part read says otherwise. An expert memo of the storage
model round, written by an agent persona and dated 19 June 2026, states that
hilavitkutin's own columns are double-buffered.

`research/202606210600_expert-loimu-heritage-storage-model.md:289-293`

> Hilavitkutin's columns are double-buffered (frozen/live) for the pass boundary. The
> resource handle model adds noalias wins. There is no temporal prediction or triple
> buffering in hilavitkutin's design. The workload does not need it: a pipeline
> execution engine does not extrapolate column values between passes -- it computes
> them.

The memo gives no source for the sentence, and no passage in the founding
spec, the amendments or the founding round carries it. By the precedence it
is intermediate work. It is a row,
`tenet::an_expert_memo_says_hilavitkutin_s_columns_are_double_buffered_for_the_pass_boundary`,
and the disagreement is the open row
`question::whether_hilavitkutin_s_own_columns_are_double_buffered`.

The counters themselves are designed and not built. The roadmap revision r6
records a per-store dirty mask standing in their place
(`tenet::the_per_morsel_generation_counters_are_not_built_and_the_engine_has_a_coarse_per_store_dirty_only`).

`research/202607181300_engine-roadmap-r6.md:315-320`

> **D-gen, per-morsel generation counters (S-6).** Domain 12 (`:861`) specifies a per-morsel
> generation counter bumped on write, propagating through the DAG so an unchanged root skips its
> transitive dependents. The engine currently has coarse per-store dirty only. This needs a **design
> sketch before implementation**, at some-shape leeway: the open question is where the counters live
> given no-alloc and a per-fiber morsel count that varies with the window, and how they compose with
> the existing `store_dirty` mask rather than duplicating it. Do not start this one by writing code.

What the record leaves open is who rotates. The founding round says the
arena is loimu's and names where hilavitkutin would take more if it ever
needed to: a new store wrapper type in the access set. A research note of the
same round rates loimu's swap buffer as directly portable to hilavitkutin as
a double-buffering primitive, and the founding spec did not take it up. So
the record holds both a hilavitkutin that knows nothing of buffering and a
proposal that it carry the rotation. A question for op follows from that and
is in the status this file went out with: whether the rotation stays with
the consumer, as two declared stores it swaps between passes, or whether
hilavitkutin gains a store wrapper that names a store as buffered, so the
schedule knows the frozen and the live side and rotates them itself.

## 2. Columns of different record counts

### What the record decides

Storage is contiguous and the columns of a fiber share one arena. A morsel is
a window into that arena over a range of records. The engine does not know
record counts ahead: they are known when the loaders finish and they cascade
down the graph. The record count is one of the parameters the plan adapts,
and morsel boundaries stay put unless it changes.

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:395-399`

> - Contiguous columnar storage (all topics agree)
> - Co-located arena within fibers (T6 L1770-1777): all columns in a
>   fiber share one arena allocation with known static offsets. One base
>   pointer per morsel, columns at `base + col_offset + i * stride`.
>   Columns across different fibers are separate allocations.

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:2453-2455`

> **R8. "record" is the global data identity.** (vocabulary)
> Not "entity" (ECS connotation), not "row" (tabular connotation).
> A column has N records. A morsel windows into a range of records.

`design_rounds/202604200055/202603121310_topic.memory-layout-and-platform.md:840-845`

> Row counts are not known until loaders complete. Downstream chains
> compute their morsel ranges just-in-time — when their fence
> dependencies are first satisfied, the upstream column lengths are
> known. This is not a special case: the progress counter naturally
> provides the current row count, and morsel ranges are computed from
> (0..current_progress).step_by(morsel_size).

`design_rounds/202604200055/202603121310_topic.memory-layout-and-platform.md:860-863`

> For chains whose inputs come from other transform chains (not
> loaders), the row count cascades: if the upstream chain processed
> N rows, the downstream chain also processes N rows. The count
> propagates through the chain graph.

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:2442-2446`

> Plan-stage parameters are adaptive: morsel sizes (from runtime hardware
> detection), record counts (from consumer data), per-phase strategy
> selection (from EMA metrics), and morsel-to-core affinity (from
> temperature). These adjust at plan time without changing the pipeline
> composition. Eliminates need for fallback dispatch approaches.

Within one schedule the count is one: a downstream fiber processes as many
records as its upstream did. Sets of records with different counts are, in
the founding round, different schedules, held by the consumer.

`design_rounds/202604200055/202603121800_topic.dispatch-and-optimisation.md:153-159`

> **DF4. Consumer-side batching and archetypes.** At ECS scale, the
> consumer manages data granularity: archetypes mean separate Schedules
> per component set, not one giant Schedule for all entities. Pipeline-
> level batching (multiple Pipeline::run() calls on subsets) handles
> saalis-scale data. The library doesn't need to know about archetypes
> or batching — it just runs the Schedule it's given. But the design
> must not assume a single all-encompassing run.

The later work adds one more count inside a schedule, and only one: an
accumulator is sized by a reserved capacity and filled by append, beside the
input columns that are sized by the record count.

`design_rounds/202606041800/202606041800_topic.accumulator-columns.md:8-8`

> The column-data-plane round landed INPUT columns: buffers sized to a build-time record count, populated before the frame, read and written by index. This round lands the second half of op's locked model: ACCUMULATOR columns. An accumulator is written by APPEND during the frame, growing from zero toward a capacity; a later PassEnd drain reads the populated prefix into persistence. op chose this two-kind surface deliberately rather than a single global record count.

Rows: `tenet::storage_is_contiguous_and_a_fiber_s_columns_share_one_arena`,
`tenet::a_morsel_is_a_three_dimensional_l1_sized_window_into_a_fiber_s_arena`,
`tenet::a_record_is_the_global_data_identity`,
`tenet::the_engine_does_not_load_data_and_does_not_know_record_counts_ahead`,
`tenet::record_counts_are_known_when_loaders_complete_and_cascade_downstream`,
`tenet::the_plan_parameters_adapt_at_plan_time_and_the_pipeline_composition_does_not`,
`tenet::morsel_boundaries_are_stable_unless_the_record_count_changes`,
`tenet::morsel_boundaries_are_baked_into_the_compiled_per_core_programs`,
`tenet::at_ecs_scale_the_consumer_holds_separate_schedules_per_component_set`,
`tenet::an_accumulator_is_a_column_written_by_append_and_a_fourth_store_marker`,
`question::where_the_record_count_of_a_column_lives`,
`question::whether_the_extent_of_a_record_domain_is_fixed`.

### What the record does not decide

Op's answer of 13 November reads: "Isn't the intent a contiguous per
prototype and indices to process needed windows to it?" The record carries
both halves under its own names. Contiguous per set of columns that share a
count is the fiber's arena. Indices that process the needed windows are the
morsel ranges, partitioned at plan time and baked into the per-core
programs. The word prototype does not occur in the part read as the name of
a record set; the nearest terms are the fiber, and the archetype the
founding round retired in favour of it.

Not decided in the part read: how several record sets with different counts
live inside one schedule in one frame. The founding round answers with
separate schedules and says the design must not assume one all-encompassing
run. It does not say how a unit in one schedule reaches a store sized by
another set's count, or whether one schedule may hold several counts at
once. Op's answer of 13 November on the extent of a record domain, "Both,
declared per domain", takes a domain as given, and the record has no
passage yet that says what declares one.

## 3. Ordering two writers of one store

### What the record decides

The founding spec decides it, in three places that agree: the scheduling
hints order them, by urgency, then divisibility, then significance. A tie
goes to the unit with the most dependents, and after that to a deterministic
fallback.

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:591-593`

> - Write-write ambiguity resolved by scheduling hint priority (Urgency,
>   Divisibility, Significance). Tie-break: most deps first, then
>   deterministic fallback. Supersedes T6 L2637 `order` field.

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1056-1059`

> 1. Build DAG: write→read edges (arvo-bitmask adjacency). Write-write
>    ambiguity resolved by scheduling hints (Urgency, Divisibility,
>    Significance — domain 09). Higher-priority hint wins. Tie-break:
>    most deps first, then deterministic fallback (type name or similar).

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1278-1281`

> Write-write to the same store: both WUs have the bit set in their
> write masks. Ordering resolved by scheduling hint priority (domain
> 09): compare Urgency, then Divisibility, then Significance. Tie-break:
> most downstream dependents first, then deterministic fallback.

The founding research had proposed registration order or an explicit
priority, and the spec replaced the explicit order field with the hints.

`design_rounds/202604200055/202603141800_research.scheduler-algorithm-deep-dive.md:77-82`

> The scheduler extracts these declarations at registration time and builds
> edges: for every pair (A, B) where A writes a resource R and B reads R,
> create an edge A -> B. For every pair (A, B) where both write R, create
> an edge between them (ordering is determined by registration order or
> explicit priority). Virtual columns (zero-sized types like
> `Write<SchemaChanged>`) create DAG edges with no storage backing.

Op's answer of 13 November is "Hints, registration as fallback". It agrees
with the spec on the hints and names the deterministic fallback, which the
spec had left as "type name or similar": registration order.

Rows: `tenet::two_writers_of_one_store_are_ordered_by_hint_priority`,
`tenet::scheduling_hints_replace_an_order_field_on_the_unit`,
`tenet::the_founding_research_orders_two_writers_by_registration_order_or_explicit_priority`,
`tenet::producer_before_consumer_registration_is_provisional_and_the_ideal_is_auto_ordering`,
`question::how_two_writers_of_one_store_are_ordered`,
`question::what_happens_when_two_units_write_one_frame_buffered_column`.

### What the record does not decide

Whether the middle step stays. The spec has hints, then most dependents,
then the deterministic fallback. Op's answer names the first and the last
and is silent on the middle one. Read as the later ruling changing only what
it names, the order is hints, most dependents, registration. Read as the
whole rule, it is hints, registration. The record does not say which.

For two units that write one frame-buffered column, op's answer of
13 November is "Schedule should exist for this specifically." The record has
no frame-buffered column in hilavitkutin (question 1), so it has no rule for
this case to quote.

## 4. Who declares what changed means

### What the record decides

Before 13 November the record splits it in two. The engine owns the
counters, the stamps and the skip, and to the engine a store changed when a
unit wrote it. Whether a write that stored the same value counts as a change
is left to the consumer's storage.

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:2406-2408`

> **R2. Incremental execution: counters in hilavitkutin, persistence via evict/inject API.** (item 5)
> hilavitkutin owns per-morsel generation counters and skip-unchanged
> DAG propagation logic (runtime scheduling concern). For persistence,

`design_rounds/202604200055/202603141800_research.loimu-dag.md:1021-1025`

> **Version stamps:** the concept is universal (track whether a resource actually
> changed). hilavitkutin should own the version stamp type and the early cutoff
> algorithm. Phantom-write detection (bitwise comparison for small values, explicit
> marking for large values) is a storage concern that each consumer can implement
> differently.

The reading aid, describing the shipped engine, has the consumer set the
dirty seed.

`research/pre_seed/202606111800_canonical-spec-standalone.md:384-390`

> A pipeline that runs the same graph every frame usually has frames where most
> inputs did not change. The engine turns the batch processor into an
> incremental one: before each frame, a dirty seed (which stores changed since
> last frame, set by the consumer marking a store dirty or by a resource
> replacement) propagates forward over the per-unit predecessor masks. A unit
> whose entire predecessor set is clean is skipped for that frame, leaving its
> output columns untouched. The propagation is a bitmask sweep over the

Op's answers of 13 November then place the declaration: what changed means
is declared per column, and the writing unit is the one that declares it.
The options he chose among are in `question::which_stores_declare_what_changed_means`
and `question::what_changed_means_for_a_computed_column`.

Rows: `tenet::the_engine_owns_the_generation_counters_and_the_skip_logic`,
`tenet::version_stamps_tell_which_stores_a_unit_actually_modified`,
`tenet::hilavitkutin_should_own_the_version_stamp_and_early_cutoff_and_phantom_write_detection_is_the_consumer_s`,
`tenet::the_dirty_seed_is_set_by_the_consumer_marking_a_store_dirty_or_by_a_resource_replacement`,
`question::who_says_that_a_store_changed`,
`question::which_stores_declare_what_changed_means`,
`question::what_changed_means_for_a_computed_column`.

### What the record does not decide

The form of the declaration. The record has the engine comparing versions
before and after a unit runs, and op's answer has the writing unit declare
per column what a change is. No passage in the part read joins the two: how
a unit's declaration reaches the stamp, and what the default is for a column
whose writer declares nothing. That is design work under his answer, and no
question for him.

## 5. A cross-record read with a reach

### What the record decides

Records within a fiber are independent, and the founding spec builds its
parallelism on that. The one dependency across records it admits goes
through a resource, and a unit that has it is detected at plan time and run
in order.

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:609-613`

> **Record independence within fibers:**
> - Records within a fiber are independent (T6 L2910)
> - Execution order of records doesn't matter for correctness
> - Exception: resource accumulation with non-commutative ops — skip
>   head+tail convergence for those (T6 L2938-2961)

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1762-1766`

> The stack-local caching pattern handles this naturally — each thread's
> snapshot is independent. Detection at plan time from WU metadata:
> if any WU in the fiber writes a resource that it also reads, the
> fiber has resource accumulation. If the accumulation is non-commutative
> (domain 08), skip convergence.

The engine's design template, written later by agents, states the same of
the shipped engine.

`crates/hilavitkutin/DESIGN.md.tmpl:324-331`

> The nesting is per-fiber, chosen by the fiber's `morsel_local` bit. An
> accumulator-free fiber dispatches morsel-outer: one morsel runs the fiber's
> whole unit sequence before the next, so its intermediate columns stay
> cache-resident across the units. This is sound because the per-record `read`
> / `write` accessors are morsel-relative and the only cross-record surface is
> the accumulator append path, so an accumulator-free fiber has every cross-unit
> dependency morsel-local. An accumulator-bearing fiber stays unit-outer (a unit
> completes its record range before the next runs), the cross-record-safe form.

The part read has no declared read of a neighbouring record, and no reach,
in any tier. Op's answer of 13 November on whether hilavitkutin owns
geometry is "Opt-in crate over the api", so a neighbourhood is not the
core's.

Rows: `tenet::records_within_a_fiber_are_independent`,
`tenet::a_unit_declares_whether_it_is_commutative`,
`tenet::a_fiber_has_resource_accumulation_when_a_unit_writes_a_resource_it_also_reads_and_this_is_detected_at_plan_time`,
`tenet::each_convergence_thread_accumulates_into_its_own_stack_local_resource_and_non_commutative_accumulation_runs_sequentially`,
`tenet::the_only_cross_record_surface_is_the_accumulator_append_path`,
`question::whether_hilavitkutin_owns_geometry`.

### What the record does not decide

What an opt-in crate over the api builds a reach from. The api as the record
has it gives a unit the records of its own morsel and nothing past the
morsel's edge, and the record has no passage on a read that crosses that
edge. It decides that the core has no such read and that the crate is where
it goes, and it stops there.

## 6. Whether the accumulator is a fourth store kind

### What the record decides

The founding spec says three.

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:491-496`

> **Three store types, one scheduling mechanism (T5 L198-208):**
> ```rust
> pub struct Resource<T>(PhantomData<T>);  // singleton, one value
> pub struct Column<T>(PhantomData<T>);    // collection, N values, morsel-chunked
> pub struct Virtual<T>(PhantomData<T>);   // zero-data, DAG edge only
> ```

The accumulator arrives with the round of 4 June 2026, which an agent wrote
and which reports op's choice of a second kind of column beside the input
columns. The same round calls its marker the fourth.

`design_rounds/202606041800/202606041800_topic.accumulator-columns.md:16-16`

> **Kind representation (A1): a distinct `Accum<T>` marker.** Sibling of `Resource<T>` / `Column<T>` / `Virtual<T>`, with its own binding node, selector, projection, and accessor. The alternative (a role or mode of `Column<T>`) collapses two semantically distinct operations (indexed random-write vs ordered append) onto one marker, making the `Contains<...>` membership proof ambiguous (read vs write vs append) and forcing a runtime flag or a forbidden specialization. The engine already has three parallel per-kind markers; `Accum<T>` is the fourth on the same pattern.

The reading aid then lists four store shapes, and A1 records the capacity
rule as a constraint note.

`research/pre_seed/202606111800_canonical-spec-standalone.md:162-173`

> State lives in one of four store shapes. There is nothing else; a pipeline's
> entire mutable and immutable state reduces to these.
>
> - **`Resource<T>`**: a singleton store: one value of `T`, with
>   scheduler-managed lifetime. Providers (a clock, a configuration, an
>   interner) are Resources.
> - **`Column<T>`**: a collection store: N records of `T` in columnar layout,
>   accessed through cache-friendly morsel windows.
> - **`Accum<T>`**: an accumulator: an append-only output store a unit pushes
>   records into, with a reserved capacity fixed at build.
> - **`Virtual<V>`**: a fired/event marker carrying no data, used for
>   cross-unit signalling at plan-time-determined boundaries (see 3.3 and 8).

`research/202606111400_canonical-design-addendum-a1.md:158-160`

> 4. Accumulator appends saturate at reserved capacity (a soundness guard);
>    capacity equals record count at build. Fixtures and consumers size with
>    headroom; a silently dropped append means live-versus-capacity first.

Op's answer of 13 November on an append past the capacity is "Configurable
by the consumer", which replaces the fixed saturation of the A1 note.

Rows: `tenet::there_are_three_store_types_and_one_scheduling_mechanism`,
`tenet::the_scheduler_builds_edges_from_store_identities_and_ignores_cardinality`,
`tenet::an_accumulator_is_a_column_written_by_append_and_a_fourth_store_marker`,
`tenet::the_standalone_spec_lists_four_store_shapes_with_the_accumulator_among_them`,
`tenet::an_accumulator_append_saturates_at_the_reserved_capacity_which_equals_the_record_count_at_build`,
`question::whether_the_accumulator_is_a_fourth_store_kind`,
`question::what_an_append_past_the_capacity_of_an_accumulator_does`.

### What the record does not decide

Whether the count of store kinds changed. By the precedence the first tier
says three, and no amendment op ruled, in the part read, says four. What op
is reported to have chosen is that the accumulator exists as a second kind
of column, written by append. That it is a fourth store kind is the wording
of the agent-written round and of the reading aid. The two fit without a
contradiction if the accumulator is a column whose write is an append,
marked by its own wrapper in the access set, which is the extension point
the founding round named for a new access form (question 1). They contradict
if the kinds are counted by marker. The row
`question::whether_the_accumulator_is_a_fourth_store_kind` holds both
passages and is open.

## 7. Whether content may add stores without Rust

### What the record decides

No. A store is a type and its registration is static. What data adds is
records.

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:2411-2412`

> column data in) APIs. Column type registration is static (compile-time)
> — no runtime column type creation. Consumers build hot-cold storage

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:2435-2440`

> **R6. Static composition, adaptive parameters.** (domain 23)
> Pipeline composition is static: all WUs registered via the builder at
> compile time. The WU set, DAG structure, fiber/trunk/phase topology,
> and monomorphised dispatch functions are all fixed at build time. This
> is what enables LLVM devirtualisation — the schedule is statically
> analysable. No runtime WU registration, no dynamic schedules.

The roadmap revision r2, agent-written and listed in A1's amendment chain,
names the two ways a pipeline grows without touching that.

`research/202606081500_engine-roadmap-r2.md:64-65`

> - **Surface 1, new DATA (dominant, native):** records added to existing columns flow through the same static WUs. `record_count` is an R6 adaptive parameter. Unbounded data extensibility with zero new dispatch. No roadmap work needed; it falls out of the static-composition + adaptive-params design.
> - **Surface 2, new CODE (rarer, downstream):** a runtime-loaded cdylib extension's transform does NOT enter the engine's monomorphised dispatch (R6: no runtime WU registration). It integrates via a statically-registered FACADE WU that calls the extension through the `ProviderId` capability ABI (hilavitkutin-extensions). The plugin keeps its own devirt (monomorphised in the cdylib); the host keeps its own; the only indirect hop is the ABI call at the FFI seam, per-invocation or per-morsel-batch, OFF the per-record hot path. Behind the facade the plugin runs its own static sub-engine (sub-graph) or is a per-morsel pure-function capability; downstream + bench-decidable.

A layout known only at load time reaches an existing store through its
shape. The founding spec takes that from loimu, and the resource storage
addendum carries op's pick of 2 July 2026 for resources.

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:577-577`

> - HasShape/ErasedColumn from loimu (T5 L674) — for plugin type access

`research/202606210600_resource-storage-model-canonical-addendum.md:46-50`

> 6. **Erased static-shape addressing (op picked the hybrid, global-capable, 2026-07-02).** The
>    value bytes stay the one-record contiguous blob (items 1-5), but are ADDRESSED through an
>    erased static-shape descriptor (backcast on access) rather than a monomorphised concrete-type
>    pointer, so a resource value can cross a cdylib/wasm plugin boundary and interoperate with
>    builtin resources near-natively (loimu's model). The bench measured this addressing at parity

Rows: `tenet::column_type_registration_is_static`,
`tenet::static_composition_means_every_unit_is_registered_through_the_builder_and_no_schedule_is_dynamic`,
`tenet::the_plan_parameters_adapt_at_plan_time_and_the_pipeline_composition_does_not`,
`tenet::a_pipeline_is_extended_by_new_records_in_existing_columns_or_by_a_facade_unit_over_a_plugin`,
`tenet::plugin_type_access_to_columns_comes_from_has_shape_and_erased_column_from_loimu`,
`tenet::a_resource_value_is_addressed_through_an_erased_static_shape_so_it_can_cross_a_plugin_boundary`,
`tenet::the_engine_has_no_register_macro_and_consumers_register_their_units_their_own_way_with_the_declarations_as_injection`,
`question::whether_registration_may_be_bound_by_data`.

### What the record does not decide

Whether a registration may be bound by data, so that a statically declared
store or unit gets its instances from content. Op was asked on 13 November
and answered "Decide in the api round". The record therefore decides the
ground (no store type and no unit is created at run time) and leaves the
binding to that round, by his own word.

## 8. What a value function with no context is, relative to the work unit

### What the record decides

The work unit is the one contract, and its one entry is `execute`, which
receives a context. The founding spec has no second kind of unit.

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:640-647`

> pub trait WorkUnit<Schedule = Always>: Send + Sync + 'static {
>     const NAME: &'static str;
>     type Read: AccessSet;
>     type Write: AccessSet;
>     type Hint: SchedulingHint = (Immediate, Atomic, Normal);
>     const COMMUTATIVE: bool;  // T3 decision #45
>     fn execute(&self, ctx: Context<Self::Read, Self::Write>);
> }

`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:660-668`

> - `ctx.each()` / `ctx.batch()` / `ctx.reduce()` op registration (T6 L768-783)
>
> **Inner loop contract (T3 L3418-3560):**
> - Op author controls inner loop, framework controls morsel boundaries
> - `#[inline(always)]` on morsel dispatch loop
> - Framework constrains inputs: ColumnValue (Copy+'static, ≤16 bytes),
>   type-native stride, contiguous, non-aliasing Read/Write
> - Optional `for_each_row` utility for common pattern
> - process_batch = single entry point (T3 decision #82)

The founding round weighed a separate trait for map-shaped units and turned
it down, keeping the single entry.

`design_rounds/202604200055/202603121800_topic.dispatch-and-optimisation.md:3552-3561`

> `for_each_row` get vectorization-friendly loop structure
> automatically. Op authors who write their own loop get the same
> outcome as long as they iterate sequentially — which is the natural
> thing to do with slices.
>
> The key difference from the MapUnit/FuseUnit approach: this doesn't
> change the WorkUnit trait or introduce new traits. It's a utility
> function in the SDK that op authors can optionally use. process_batch
> remains the single entry point. The framework doesn't control the
> inner loop — it provides a helper for the common case.

The same round's api topic then describes a unit inside a fiber as a pure
function of values, and moves the closure a unit registers to that form so
the engine can compose a fiber without going through memory.

`design_rounds/202604200055/202603151200_topic.hilavitkutin-api-surface.md:1767-1771`

> Within a chain, processing row n is: `d(c(b(a(input_n))))`. Each WU is
> a pure function that takes column values and produces column values. The
> chain is a composition of these functions. The intermediate "columns"
> are just named wires between stages — they don't need memory backing
> within the chain.

`design_rounds/202604200055/202603151200_topic.hilavitkutin-api-surface.md:1840-1843`

> The WU's `each` closure signature changes from
> `|col_a: &T, col_b: &mut U| { ... }` to a pure function
> `fn(input_a: T, input_b: T) -> (U, V)` that takes values and returns
> values. The flattener wires the returns to the next stage's inputs.

So in the founding record a value function with no context is the body of a
unit's per-record op, registered through the context inside `execute`. It is
part of a unit and never a unit of its own. The later agent-written round of
9 June 2026 gives it a second carrier, an opt-in contract a unit declares
beside `execute`, from which the engine folds an ordinary work unit.

`crates/hilavitkutin-api/DESIGN.md.tmpl:239-243`

> ## Opt-in per-record transform (within-fiber fusion contract)
>
> A work unit that is a pure per-record map (reads one column, writes one column, value to value, no cross-record state) may additionally declare that map as an opt-in fusion contract: an associated input type, an associated output type, and an apply function from one to the other. This is orthogonal to the execute contract (it does not change execute) and opt-in (a simple work unit never implements it).
>
> Its purpose is within-fiber linear fusion. When a fiber is a straight read-after-write chain of such maps (stage one writes the column stage two reads, and so on), the engine folds the chain at the type level into a single fused work unit whose body keeps every intermediate in a register (dead-store elimination removes the intermediate column traffic), matching a hand-fused loop. The fold is a compile-time composition of concrete monomorphised calls, no function pointers and no dynamic dispatch. A work unit that carries cross-record state (an accumulator) does not implement the contract and stays on the ordinary per-unit dispatch path. The shipped trait name and signature live in the engine-facing rustdoc; this section states the contract shape.

Op's answer of 13 November accepts a unit written as a plain function that
receives the values of its declared reads and returns the values of its
declared writes, and on whether that form reads a resource says "Yes. Use
typestate and statically branch accordingly".

Rows: `tenet::a_unit_declares_its_name_read_set_write_set_hint_commutativity_and_an_execute_function`,
`tenet::a_unit_registers_its_ops_through_each_batch_and_reduce_on_the_context`,
`tenet::process_batch_is_the_single_entry_point`,
`tenet::the_op_author_controls_the_inner_loop_and_the_framework_controls_the_morsel_boundaries`,
`tenet::inside_a_fiber_a_unit_is_a_pure_function_from_column_values_to_column_values`,
`tenet::execute_receives_a_context_and_nothing_else`,
`tenet::a_pure_per_record_map_may_be_declared_as_an_opt_in_fusion_contract_beside_execute`,
`question::how_a_unit_in_the_value_form_reads_a_resource`.

### What the record does not decide

Which carrier the value form has. The founding record carries it as the
closure a unit hands to the context. The round of 9 June carries it as a
contract beside `execute`, limited to one column in and one column out. Op's
answer of 13 November widens it to lists and to resources and does not name
the carrier. In every one of the three the value function stays under the
work unit: the plan is built over units, and a value function is either the
body of one or the thing the engine builds one from. No passage in the part
read makes it a peer of the work unit.
