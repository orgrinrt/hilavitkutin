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
with their options and are quoted here from `OP-ANSWERS.md` and
`OP-ANSWERS-13.md` beside this file.

Paths are relative to `mock/`. The founding spec is
`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`
and is written SPEC below. Rows are cited as `tenet::<id>` and
`question::<id>` and live under `mock/registry/`.

## How far the read had come when this was written

This is the fourth form of the answers, of 13 November 2026, about 11:40 UTC.
The first form went out at 10:15 UTC, the second at about 11:10 UTC and the
third at about 11:30 UTC. Op
answered four questions on the first form at about 10:35 UTC; they are in
`OP-ANSWERS-13.md` beside this file and are `question` rows. On his words
the read turned to the unlanded work and to the round of May 2026 before
anything else, and questions 1, 2, 3 and 6 below are revised from what those
hold. At about 10:55 UTC he said that loimu is an old reference and that the
buffering he recalls may have been discussed in ikiuni; his words are in
`OP-ANSWERS-14.md`, and what ikiuni holds is in `IKIUNI.md`.

After this form was written, at about 12:00 UTC, he said he would not walk
hundreds of rows, and that the rows are filled as ratified by default from
the written history, with only the unbacked and the conflicting ones brought
to him. His words are in `OP-ANSWERS-18.md`. Readers then classed every row
against the record. A tenet whose content a passage shows to be his carries
`standing = "ratified"` with that passage in `ratified_from`. A row with no
word of his either way stands on a `stamp`. The rows the record does not
back, or that stand against a word of his, are listed in `ROWS-FOR-OP.md`
beside this file and are neither ratified nor stamped. The answers below
were written before that pass and cite rows by id, whatever their standing
is now. His two remarks on the benches, of about 12:20 and 12:25 UTC, are in
`OP-ANSWERS-20.md` and `OP-ANSWERS-21.md` and are rows of their own.

Read whole and first hand: the founding spec, amendments A1 and A2, the
unified-engine amendment, the resource storage addendum, the governance
seed, the swap specification, its ratification evidence and the topic of the
round that implemented it, the adapt and resource-resolution topics and
section S7 of the plan topic of round `202605101036`, the rounds of 13
November 2026 on the accumulator's overflow, both headers of the standalone
spec and the passages quoted from its body, and op's answers. Read whole by
the readers, with every quote checked against the file: the seven
founding-round topics, the founding research notes, the research notes and
roadmaps from May to July 2026 with the `pre_seed` set, the seed, the design
and readme templates of the workspace and of every crate, every design round
from the founding one to 13 November 2026, the findings of every sketch, the
bench findings on `dev`, and every document that only the branch
`origin/recover/unlanded-rounds-and-benches` carries. Ikiuni was not read
whole. It was searched for buffering, swaps and maintenance units, and the
passages found were read in their files.

That branch holds less unread design than its size suggests. Its design
rounds, research notes, seed and sketches are on `dev` already, byte for
byte. What only it carries is the bench findings under
`mock/benches/results/`, engine source and tests, and changes to a few
design templates. The template passages that say something about the design
are copied in `RECOVERED.md` beside this file.

Not yet read when this was written: the research imported from polka-dots and
saalis. `LEDGER.md` lists every file with its state. Where an answer says
the record has no passage for something, that is a statement about the part
read, and the unread part can still hold one.

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

The founding round took up exactly this question for loimu, the earlier
project whose data was buffered two and three deep, and resolved it. The
spec cites the resolution at line 678 for the rule that a write implies a
read. Loimu is no live user of hilavitkutin. Op said on 13 November that it
is an old reference which hilavitkutin primarily replaces
(`tenet::loimu_is_an_old_reference_that_hilavitkutin_primarily_replaces`),
so where the founding texts below say loimu, they describe the reference the
design was worked out against.

`research/202611130850_the-record-read-whole/OP-ANSWERS-14.md:4-4`

> And loimu is an old reference. I don't remember revitalizing it. Hilavitkutin is primarily the replacement for it, with a pending consideration to later down the line restart loimu based on hilavitkutin, I can't remember loimu being important as anything other than reference, so so weird you mention it at all

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

So the founding round keeps hilavitkutin at two modes and puts the frozen
read in a layer built over it, by whoever consumes it. The loimu research
the founding round read shows how that reference did it: each field is
several columns, and the reader of the frozen one and the writer of the live
one touch different stores.

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

The later record adds three things that are a swap or a buffer by name. Each
is narrower than a store that is read at its frame-start value while its
next value is written.

The first is the swap of a resource's value. Amendment A2-5 found it
unspecified on 19 July 2026 and commissioned a round.

`research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md:88-94`

> Op's direction (2026-07-19): the missing swap spec is researched and designed explicitly, not
> picked from options. A design round is commissioned to produce the explicit `Replaceable` swap
> specification under the bench-decided layout (blob record, erased addressing, collection
> elements in collection columns), covering `replace_value`, `replace_resource`, the
> `PlanAffecting` interplay (A2-3), Seq/Map element ownership, dirty propagation, and quiescence.
> The same round corrects the mis-grounded artifacts. Until it lands, `replace_value` stays
> mark-dirty-only and the catalogued test stays red.

The round wrote a specification in seven clauses. The swap replaces one
whole value between frames and is seen from the next run on. It keeps no
earlier value: the old bytes are overwritten.

`research/202607200200_replaceable-swap-semantics-spec.md:24-38`

> `Scheduler::replace_value<V: Replaceable>(&mut self, new: V)` installs `new`
> as the whole one-record blob value: a `ptr::write` of `V` through the same
> `Selector<V, Index>` witness (the binding's backcast pointer) that the drain
> and the projection read use. No pointer derived by any other route (the
> noalias provenance argument depends on it). No drop of the old value
> (`V: ColumnValue` is `Copy`; the bytes are overwritten). The call
> additionally sets the store-dirty bit for `Resource<V>` via `mark_dirty` and
> never touches `plan_dirty`.
>
> The call is legal only between frames. `&mut self` plus the
> parked-between-frames invariant (named as load-bearing, because worker raw
> pointers sit outside the borrow checker's jurisdiction) means no swap can
> race a live-streamed read or an in-flight snapshot. The swap is observable
> starting at the next `run()` and at no earlier point; of multiple swaps
> before a `run()`, only the last is ever observed.

A swap of a plan-affecting resource does the same write and also sets the
bit that sends the next run through the plan band.

`research/202607200200_replaceable-swap-semantics-spec.md:42-47`

> `Scheduler::replace_resource<T: PlanAffecting>(&mut self, new: T)` performs
> the identical S1 write and store-dirty mark, and additionally sets the
> `plan_dirty` bit for `T`'s `PlanAffectingId` in the scheduler's plan-dirty
> bit array. The very next `run()` consumes that array: the band gate is
> `first_frame OR any plan_dirty bit`, a plan-dirty frame enters the leading
> plan band at phase 0, and the consumed bits clear.

The specification is a draft pending op's ratification, and op held the
ratification until it was implemented and benched. The evidence memo reports
both.

`research/202607200200_replaceable-swap-semantics-spec.md:3-6`

> **Date:** 2026-07-19
> **Status:** DRAFT pending op ratification (the seed pre-freeze resolution
> batch). On ratification this document is the ruling source for the swap
> rulings' registry rows and the seed storage chapter's replacement section.

`research/202607201100_swap-semantics-ratification-evidence.md:6-14`

> **Spec under ratification:** S1 to S7 of
> `202607200200_replaceable-swap-semantics-spec.md`. Op held ratification
> until the spec was implemented and benched; this memo is that evidence.
>
> ## What shipped (both changelists locked)
>
> S1: `replace_value` installs the whole value as one blob write through
> the same `Selector` witness the drain wrote through, plus the
> store-dirty mark; no other pointer derivation exists on the swap path.

`research/202607201100_swap-semantics-ratification-evidence.md:53-58`

> Each path pays exactly its own channel and nothing else, which is the
> S5 asymmetry claim confirmed structurally: the clean-skip baseline is
> flat and size-independent; the plan swap pays baseline plus the band
> (flat, record-count-independent, band cost only); the value swap pays
> baseline plus its dirty cone, scaling linearly with the records the
> cone touches, and never pays the band.

What that memo calls shipped is not on `dev`. The engine source, the two
benches `swap_cost` and `swap_band` and the design text that states the
specification are on the recovery branch only. On `dev` the engine design
template has no swap paragraph and the api design template still seals the
plan-affecting marker, which the specification opened. The recovery branch's
paragraph, as copied:

`research/202611130850_the-record-read-whole/RECOVERED.md:74-85`

> A `Replaceable` swap installs the whole value as one blob write through the
> same `Selector` witness the drain and the projection read use, plus a
> store-dirty mark; no other pointer derivation exists on the swap path. Under
> the bench-decided storage layout a resource value is one reserved blob record
> (scalar snapshot plus live-streamed collections), so the whole-blob install is
> the specified mechanism, not an excluded shortcut. The earlier per-leaf-copy
> rule was grounded on the refuted pre-bench layout, and the revert of the
> one-line install that followed it was a mis-citation; the witnessed blob write
> is canonical. Collection-bearing values gain per-record element writes for
> their `Seq` and `Map` halves only when the collection wiring lands (tracked by
> the catalogued swap test); until then the collection halves of a swapped value
> are out of the swap's scope by design, not by omission.

These are the swap benches of the record, and they measure a resource
value's replacement: `swap_cost` times one install across value sizes, and
`swap_band` compares, on the next frame, a clean skip, a value swap and a
plan-affecting swap. Neither measures a store buffered two or three deep.

The second is the adapt unit, which is the maintenance unit the record
specifies. Round `202605101036` makes adaptation a meta unit that observes
the end of the schedule; the consumer registers the default one, ships its
own, or leaves it out.

`design_rounds/202605101036/202605101036_topic.adapt-subsystem.md:15-15`

> - **Topic 1 Axis 5.** Adapt is a meta-WU (`AdaptWu`) observing `ScheduleEnd`. Substrate ships default `AdaptWu` in `hilavitkutin-providers`; consumer registers via `.with(AdaptWu::default())` for substrate behaviour, ships own `AdaptWu` for custom, omits if not wanted. The integration shape is locked. This topic specifies what the WU body does and what metrics structure it reads/writes.

Its work is specified. The metrics are written into a scratch arena on the
hot path; at the end of the schedule the adapt unit refreshes from the arena
the resources a consumer can read, computes the predicted wait per phase for
the parking primitive, and fires one anomaly flag. It signals and does not
re-plan.

`design_rounds/202605101036/202605101036_topic.adapt-subsystem.md:266-266`

> - [x] **Axis B. AdaptMetrics data plane: B3 hybrid.** Scratch-arena hot path + `Resource<SchedulerMetrics>` snapshot refreshed by AdaptWu at `ScheduleEnd`. Snapshot is small (per-phase + per-fiber + per-core counters, on the order of MAX_PHASES + MAX_FIBERS + MAX_LANES u64s). Inner loop accesses arena directly via raw pointer; consumer reads via standard `Has<SchedulerMetricsProvider>` Ctx bound.

`design_rounds/202605101036/202605101036_topic.adapt-subsystem.md:314-314`

> - Topic 5 does not own re-plan execution. AdaptWu signals `Virtual<ReplanRequested>`; Topic 9 (plan caching + dirty-detection) owns the observer that triggers actual re-planning.

The adapt unit publishes that snapshot through a double buffer, and this is
the one double buffer hilavitkutin's own design specifies. The resource
holds two buffers and an index. The adapt unit fills the buffer that is not
live and flips the index with one atomic store; a reader loads the index and
reads the buffer it names.

`design_rounds/202605101036/202605101036_topic.plan-algorithm-chain.md:1181-1198`

> **Double-buffer pattern for AdaptWu snapshot publish (resolves audit M1):**
>
> ```rust
> struct MetricsResource<T> {
>     buffer_a: T,
>     buffer_b: T,
>     current: AtomicUsize,  // 0 = a is live, 1 = b is live
> }
>
> // AdaptWu at ScheduleEnd:
> fn execute(&self, ctx: &Self::Ctx) {
>     let mut r = ctx.writer::<MetricsResource<...>>();
>     let next_idx = 1 - r.current.load(Ordering::Relaxed);
>     let next_buf = if next_idx == 0 { &mut r.buffer_a } else { &mut r.buffer_b };
>     // ... fill next_buf from AdaptArena slots (Relaxed reads paired
>     //     with phase-barrier Acquire fence at WU entry) ...
>     r.current.store(next_idx, Ordering::Release);  // single atomic publish
> }

The unit that writes the store is the one that rotates it, inside its own
write, at the end of the schedule. Nothing in the schedule rotates it for
the unit, and the unit rotates nothing but its own snapshot. The pattern is
there for a reader on another thread, and it was added to resolve an audit
finding on atomic ordering.

The third is the resource snapshot of the same round: a read-only view of a
resource captured at a phase barrier, which cannot outlive the phase.

`design_rounds/202605101036/202605101036_topic.resource-resolution.md:22-25`

> Snapshot captures only at phase barrier (after futex-wake, before next phase's morsels begin). Workers that hold a `ResourceSnapshot<R>` are guaranteed:
> - The underlying `R::Owned` storage is not being written during the snapshot's lifetime.
> - The snapshot's content matches what `R::Owned` held at `captured_at`.
> - Snapshot lifetime is bounded by the current phase (cannot escape across phase barriers).

It holds the current value still for one phase. It is no earlier value kept
beside a newer one. A later round found the name used for two things and
kept this meaning; the concrete signature waits for the data plane.

`design_rounds/202605282256/202605282256_topic.resource-snapshot-disambiguation.md:14-14`

> The locked design's typed view is the surviving meaning of `ResourceSnapshot`, but its DESIGN signature is stale. `hilavitkutin-api/DESIGN.md.tmpl` documents `ResourceSnapshot<'phase, R: BuilderInput<Dispatch = ResourceDispatch>>` with an `inner: Pin<&'phase R::Owned>` field. Neither `ResourceDispatch` nor `BuilderInput::Owned` exists: the round 202605101036 topic 2 builder reshape replaced per-kind dispatch routers with `StoreDispatch` / `UnitDispatch` / `KitDispatch` / `PlatformDispatch` / `RunCfgDispatch`, and `BuilderInput` carries only `Init` and `Dispatch` associated types. The signature predates the reshape. Its concrete form must be re-derived against the real `BuilderInput` contract, and that re-derivation depends on how a registered store exposes its scheduler-owned storage, which is the Phase B data-plane design. So the typed view ships with the data plane (HILA-RUNTIME-C6), not in this round. This round corrects the DESIGN so it stops presenting a stale signature as a shipping contract.

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
`tenet::the_swap_specification_is_a_draft_that_becomes_the_ruling_source_for_the_swap_rows_once_the_maintainer_ratifies_it`,
`tenet::the_maintainer_held_ratification_of_the_swap_spec_until_it_was_implemented_and_benched`,
`tenet::replacing_a_plan_affecting_resource_does_the_replace_value_write_and_sets_the_plan_dirty_bit_the_next_run_consumes`,
`tenet::replace_value_is_the_supported_every_frame_swap_and_costs_one_blob_write_then_the_store_s_dirty_cone`,
`tenet::as_of_2026_07_19_replace_value_ships_as_one_blob_write_through_the_drain_s_selector_witness_plus_the_dirty_mark`,
`tenet::the_swap_band_bench_proves_channel_separation_and_not_the_absolute_cost_of_a_real_plan_recompute`,
`tenet::a_replaceable_swap_installs_the_whole_value_as_one_blob_write_and_marks_the_store_dirty`,
`tenet::the_adapt_unit_is_a_meta_unit_that_observes_the_end_of_the_schedule_and_the_consumer_registers_replaces_or_omits_it`,
`tenet::adapt_metrics_live_in_a_scratch_arena_and_the_adapt_unit_refreshes_a_consumer_readable_snapshot_at_the_end_of_the_schedule`,
`tenet::the_adapt_unit_predicts_the_wait_per_phase_and_the_parking_primitive_only_reads_the_prediction`,
`tenet::the_adapt_unit_fires_one_anomaly_flag_and_the_metrics_resources_carry_which_anomaly_it_was`,
`tenet::the_adapt_unit_signals_and_does_not_re_plan`,
`tenet::the_adapt_unit_publishes_its_metrics_snapshot_through_two_buffers_and_one_index_it_flips`,
`tenet::a_resource_snapshot_is_a_read_only_view_captured_at_a_phase_barrier_and_it_cannot_outlive_the_phase`,
`tenet::the_typed_phase_bounded_view_is_the_one_meaning_of_resource_snapshot_and_its_signature_lands_with_the_data_plane`,
`tenet::four_meta_virtuals_fire_plan_stage_schedule_ready_pass_start_and_schedule_end`,
`question::whether_the_maintainer_ratifies_s1_through_s7_of_the_swap_spec_as_the_swap_semantics_canon`,
`question::whether_the_plan_affecting_marker_is_sealed_or_open`,
`question::who_rotates_a_buffered_store`,
`question::how_a_unit_reads_the_value_a_store_held_at_frame_start`.

### What the record does not decide

Op's first answer of 13 November says the counters were designed for exactly
this, that known downstream users triple buffer, and that the spec should
answer it at least in intent. The record has the intent that a consumer can
buffer two or three deep over hilavitkutin. The only user it names as doing
so is loimu, which by op's later words of that day is a reference and no
live user. In the part read the record does not give the counters that job:
no passage has a counter or a generation select or hold an earlier value of
a store.

His second answer, on who rotates, chose none of the three options. He
recalled research, spikes and perhaps benches on multi-buffering and swaps,
and specifications of maintenance units that do this and adaptation, and he
asked whether that work sat unlanded. The record, read as far as the section
above says, holds the following of what he recalled.

Swaps are specified, implemented and benched, and the implementation is
unlanded: they are swaps of a resource's whole value between frames. A
maintenance unit is specified: the adapt unit, a meta unit at the end of the
schedule. Multi-buffering of hilavitkutin's own stores is specified in one
place, the adapt unit's snapshot with its two buffers and its flipped index.
No passage in the part read specifies a column or a consumer's resource
buffered two or three deep with a rotation the engine or a maintenance unit
performs, and no bench in it measures one.

The passage that comes closest is section S7 of the plan topic of round
`202605101036`, lines 1181 to 1198, quoted above. It is the only place where
hilavitkutin's own design keeps the published value of a store and its next
value side by side and says who flips them: the unit that writes the store,
by one atomic store of an index, at the end of its own write, with the
reader choosing its buffer by loading the index. It is closest and it does
not decide the question, for two reasons that are readings of mine. It is
specified for one resource and one writer, as a publish across threads, and
nothing generalises it. And its reader runs in the pass after the flip; a
reader and a writer of one such resource inside one pass would be ordered by
the data flow, reader after writer, so the reader would see the new value.

The second closest is the founding round's resolution quoted at the top of
this answer: the frozen read is the consumer's layer, and if hilavitkutin
ever took more, it would enter as a new store wrapper type in the access
set. A research note of the same round rates loimu's swap buffer as directly
portable to hilavitkutin as a double-buffering primitive, and the founding
spec did not take it up.

Two later notes say where the earlier buffering designs went. The storage
heritage note of 30 May 2026 lists what of that heritage is stale or
superseded
(`tenet::a_storage_heritage_note_lists_the_per_morsel_snapshot_buffer_s_double_buffering_as_superseded_by_generation_counters`).

`research/202605302130_unified-storage-design-archaeology.md:67-67`

> Stale / superseded: `const N: Cap` / `const N: usize` on storage types (-> `Capacity`); `VersionedStore` threading a const-generic `N` (-> thread `Capacity`); slice-returning `as_slice`/`as_mut_slice` (-> raw pointers, R6); `StoreId` hash identity with `LazyLock` (-> dense plan-time `u16`); `HashMap`-backed `StoreMap` (-> const-array + plan-time index); loimu's dynamic view affinity / hysteresis / restructuring (-> static plan-time co-located fiber arenas); loimu's seven access modes + three fusion tiers (-> Read/Write + commutativity); temporal storage (loimu-specific); per-morsel `SnapshotBuffer` double-buffering (-> generation counters).

This is the one passage found that ties the counters to buffering, and it
agrees with op's first answer as far as it goes: the counters took the place
of the double-buffered snapshot buffer. It says what was replaced. It does
not say how a reader gets a store's earlier value from a counter, and no
other passage read does. The expert survey of alternative storage models,
written for the storage model round, puts the three-deep model outside the
engine
(`tenet::an_expert_survey_places_the_triple_buffer_temporal_model_in_loimu_s_domain_and_not_in_the_engine`).

`research/202606210600_expert-alternatives-storage-model.md:172-175`

> - **what is worth grafting:** the *affinity insight* (§5 option 4): co-locate members that WUs read
>   together. In hilavitkutin terms that is a build-time decision the existing waist/trunk analysis
>   already approximates (fibers that read the same resources run together). The triple-buffer temporal
>   model is loimu-domain (UI extrapolation), not engine substrate.

Both are agent-written and intermediate by the precedence.

One passage in the part read says hilavitkutin's columns are double-buffered
already. It is an expert memo of the storage model round, written by an
agent persona and dated 19 June 2026.

`research/202606210600_expert-loimu-heritage-storage-model.md:289-293`

> Hilavitkutin's columns are double-buffered (frozen/live) for the pass boundary. The
> resource handle model adds noalias wins. There is no temporal prediction or triple
> buffering in hilavitkutin's design. The workload does not need it: a pipeline
> execution engine does not extrapolate column values between passes -- it computes
> them.

The memo gives no source for the sentence, and no passage in the founding
spec, the amendments, the founding round or round `202605101036` carries it.
By the precedence it is intermediate work. It is a row,
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

The sketch findings were read after the first three forms of this answer,
and one sketch of 12 and 13 November 2026 is the nearest thing in
hilavitkutin's own record to a store read at its frame-start value while its
next value is written. It is a feasibility sketch written by a helper for
the roadmap of building kaski's engine on the stack's contracts. It is
agent-written and intermediate, it says itself that it was checked against
no canon, and a sketch shows that a shape can be built, never that the shape
was chosen. Its frame holds every lane twice
(`tenet::a_sketch_of_november_2026_holds_every_lane_of_a_frame_twice_committed_and_staged`).

`research/sketches/202611122242_frame-view-typestate/FINDINGS.md:15-15`

> A lane is a unit struct implementing `Lane` (its item type and initial value) and one of `FrameLane` or `LandedStore`. The registry is a type alias, a `notko_hlist` list of every lane. `Frame<Reg, N>` holds N rows of every lane twice, committed and staged.

A unit of the step reads the committed form through a view that has no write
and writes the staged form through handles that have no read
(`tenet::in_the_frame_view_sketch_a_step_kernel_reads_the_committed_frame_through_a_view_and_writes_through_staged_handles`).

`research/sketches/202611122242_frame-view-typestate/FINDINGS.md:17-19`

> `View<'s, Reg, R>` is the committed frame for the step `'s`, restricted to the read list `R`. It is a list of shared slices and a phantom, so it is `Copy`, and `Sync` exactly when every lane's item is, which `Lane::Item` requires. Its one read is `view.get(Lane, row)`, bounded on `R: Contains<Lane>`; it has no write method. `Row` is an opaque brand built only by the engine, with no order and no integer conversion (stamp 7, F's note).
>
> `StagedLane<'s, X>` is one write-only handle: `set(row, value)` and nothing else, neither `Copy` nor `Clone`. A block kernel receives one per member of its write list as a nested tuple and destructures it. A value kernel returns a nested tuple of values per row and the runner writes them through the handles itself.

The findings do not say in words who exchanges the two forms. They say what
runs after the exchange
(`tenet::in_the_frame_view_sketch_a_close_unit_gets_the_landed_stores_after_the_step_lands_and_never_the_view`).

`research/sketches/202611122242_frame-view-typestate/FINDINGS.md:21-21`

> `Landed<'c, Reg, S>` is what a close unit gets after the step lands: `get`, `set` and `rows` on the stores `S` names, which must be landed stores. There is no path from it to a view, and the view's constructor is crate-private.

The sketch's code does the exchange in the runner. This is a statement about
the code of a sketch and no statement of design:
`Frame::step` in `mock/research/sketches/202611122242_frame-view-typestate/engine/src/frame.rs`
copies the committed form over the staged one at line 41, runs the kernels,
and at line 49 swaps the two with `core::mem::swap`. The close unit runs
after that, at line 90, on the landed stores. So in this one sketch the
runner rotates and no unit does, the depth is two, and the whole frame is
exchanged at once. A second sketch of the same two days names the same
exchange in passing, as one of the whole-store operations its runner writes
in place.

`research/sketches/202611122252_runner-generic-workunit/FINDINGS.md:43-43`

> What the registry did need is a different shape for whole-store operations. A by-value `Clone` over the 64-deep product recurses with a stack frame per level holding the rest of the product, and in the debug profile it overflows a thread with the default 2 MiB stack and runs on 16 MiB; in release it runs on 2 MiB. `stack-probe/tests/deep_by_value.rs` shows the three cases, each in a child process, and `diagnostics/registry-by-value-clone-overflows-the-default-stack.txt` holds the overflow and the release run beside it. Writing the operations in place (`copy_from`, `splat_in` over `MaybeUninit` with field writes, `made` on top of it, and `mem::swap` between the double buffers) runs it on the default stack. Anything deep in notko-hlist that moves a whole product by value has the same exposure.

Neither sketch has a store buffered three deep or a maintenance unit that
rotates.

Op also said on 13 November that the buffering he recalls may have been
discussed in ikiuni, the engine concept repository that weighed hilavitkutin
against bevy. Ikiuni is a reference and not hilavitkutin's record. It does
hold a buffering design, and it does say who rotates. The passages are
copied in `IKIUNI.md` beside this file, from ikiuni's `dev`.

Ikiuni buffers the whole frame two deep. Every computation of a tick reads
the previous committed frame, which is read-only for the whole tick, and
every change is published into the next frame together
(`tenet::ikiuni_s_design_applies_every_change_at_the_frame_barrier_by_a_double_buffer_swap`).

`research/202611130850_the-record-read-whole/IKIUNI.md:34-38`

> > A Mutation never takes effect in place. It applies at the next frame barrier: the frame a tick computes
> > over is read-only for the whole of that computation, and every Mutation the tick produces is published
> > together into the next frame by a double-buffer swap. A tick therefore reads the previous committed frame
> > as an immutable input and writes the next frame as output, and no reader within a tick observes another
> > reader's write. This application-timing law is what makes the order among a tick's own Mutations a pure

The swap is done by the tick loop's closing bracket, which ikiuni says is a
reserved part of the frame and not a system
(`tenet::in_ikiuni_the_frame_close_bracket_of_the_tick_loop_swaps_the_double_buffer_and_it_is_not_a_system`).

`research/202611130850_the-record-read-whole/IKIUNI.md:58-64`

> > - **Frame-open and frame-close are reserved structural brackets, not systems.** They ARE the next-frame
> >   keystone (000) made concrete, not nodes with empty dependency lists. Frame-open ingests the committed
> >   frame, delivers the clock and batch constants (200:47), and faults in any queued deltas. Frame-close
> >   drains the Mutation queue (the queue-during/drain-after seam, 200:23), runs the conservation check
> >   (200:43), swaps the double buffer, and seals the frame. Content systems schedule only in the interior
> >   BETWEEN the brackets and cannot, by construction, order themselves across a bracket. A mis-declared
> >   dependency can only mis-order work within the frame, never move it across the barrier that defines the

That design was written for an engine on bevy. Ikiuni records, as the user's
call of 16 June 2026, that the hilavitkutin stack would have been the ideal
and was not ready, and that the engine stays behind a contract a later
hilavitkutin engine can satisfy
(`tenet::ikiuni_chose_bevy_s_ecs_because_the_hilavitkutin_stack_was_not_ready`,
`tenet::in_ikiuni_bevy_is_one_engine_behind_a_contract_that_a_later_hilavitkutin_engine_can_satisfy`).

`research/202611130850_the-record-read-whole/IKIUNI.md:74-80`

> > ## ECS substrate: DECIDED = bevy_ecs (2026-06-16, user call)
> >
> > `bevy_ecs` used standalone, with rendering, GUI, windowing, and HID input ditched entirely. The ideal
> > would be the project's own `hilavitkutin` + `arvo` + `notko` stack, but that is not ready, so the
> > rebuild works from existing options. `bevy_ecs` is a proven staple for the user's PoC and ideation
> > work; stripped of the render and input layers it is a lean, mature, parallel, archetypal-columnar ECS
> > core, which is the one part needed here and the part bevy does almost perfectly.

Ikiuni's design is two deep and for the whole frame. A search of its
principles, its canonical design and its imported design history finds
double buffering in 20 files and finds triple buffering, multi buffering
and a maintenance unit in none. It has no passage on how the closing
bracket's swap would be carried by hilavitkutin's units, stores or schedule.

So the record holds a hilavitkutin of read and write that leaves buffering
to the layer over it, one unit that double-buffers its own snapshot and
flips it itself, a swap of a resource's whole value between frames, a
proposal that hilavitkutin carry a buffering primitive, and a sketch of
November 2026 whose runner holds every lane twice and exchanges the two
forms itself before the close unit runs. Ikiuni, beside the record, holds a
frame buffered two deep and swapped by the frame's closing bracket. The
sketch and ikiuni agree on who rotates: the runner at the frame's close, and
no unit. Neither is a decision for hilavitkutin. The sketch is agent-written
feasibility work and ikiuni is a reference, so who rotates a store of
hilavitkutin that is read at its frame-start value while its next value is
written in the same pass is still without a stated or ratified passage. The
row `question::who_rotates_a_buffered_store` carries the three options, op's
words and the rows above.

The research imported from polka-dots and saalis was not read when this was
written. Every design round, every sketch's findings and the bench findings
on `dev` were read. The rounds hold the swap round quoted above and no
buffering of a store, and the bench findings hold none either: the one
bench that compares a live read with a copied snapshot is of a sequence
member's payload and not of a store's frame.

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
`question::how_record_sets_of_different_counts_live_together`,
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

Op was asked on 13 November how record sets of different counts live
together: several sets in one schedule, each contiguous with its own count
as a plan parameter; one schedule per set, as the founding round has it,
with crossings carried by the consumer; or a bench of both first. His answer
is "Bench both first". So the api takes neither shape until both are
sketched and benched, and the record stands as above until then. The row is
`question::how_record_sets_of_different_counts_live_together`. The recovery
branch adds nothing here: its bench findings are on fiber theory, the
reverse Cuthill-McKee order and its rivals, the wake policy and the two swap
benches, and none compares the two shapes.

The later rounds keep one count. The round of July 2026 that turns the
per-fiber morsel size into a window has every fiber cover the same record
range
(`tenet::a_fiber_s_morsel_size_is_a_window_and_the_fiber_covers_the_one_record_range_in_count_over_window_morsels`).

`design_rounds/202607161410/202607161400_topic.a3b-l1-morsel-formula.md:29-33`

>    window (nothing constrains its L1 write budget). `plan.morsel_windows[f]`
>    becomes a WINDOW size; a fiber covers the record range in
>    `ceil(record_count / window)` morsels. This is the designed semantics flip the
>    A1 slice documented as pending; tests pinning the partition placeholder are
>    rehabbed to the window contract and `r6_morsel_window_formula` goes green.

The rounds read from 19 June to 12 November 2026 have no passage on a
schedule holding several counts.

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

Op's first answer of 13 November is "Hints, registration as fallback". It
agrees with the spec on the hints and names the deterministic fallback,
which the spec had left as "type name or similar": registration order. Asked
the same day whether the step between them stays, he answered "Hints,
dependents, registration". So the order is settled: the scheduling hints,
then the most downstream dependents, then registration order.

Rows: `tenet::two_writers_of_one_store_are_ordered_by_hint_priority`,
`tenet::scheduling_hints_replace_an_order_field_on_the_unit`,
`tenet::the_founding_research_orders_two_writers_by_registration_order_or_explicit_priority`,
`tenet::producer_before_consumer_registration_is_provisional_and_the_ideal_is_auto_ordering`,
`question::how_two_writers_of_one_store_are_ordered`,
`question::whether_the_most_dependents_step_stays_between_hints_and_registration`,
`question::what_happens_when_two_units_write_one_frame_buffered_column`.

### What the record does not decide

The order itself is decided. What stays undecided is in the code and the
engine design template, which order two writers by registration position
alone and read no hints; by the answers that is a deviation to fix and not a
second design.

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

The rounds hold three states of that rule, in order. The A1 note of 11 June
2026 has the append saturate at the capacity, and the record past it is
dropped. A round of 20 June 2026, agent-written, makes the same append a
violation of the capacity contract that asserts before the write
(`tenet::an_append_past_an_accumulator_s_reserved_capacity_is_a_contract_violation_that_asserts_and_never_drops_silently`).

`design_rounds/202606200300/202606200300_topic.accum-overcap-hard-error.md:19-26`

> Over-appending a fixed-capacity accumulator is a capacity-contract violation (a
> bug, like an out-of-bounds index), not a recoverable runtime condition. The run
> entries return the consumer-defined `Cfg::Out`, so there is no uniform `Outcome`
> channel to carry a recoverable error. The fix is fail-loud: `resolve_append`
> asserts `live < cap` and panics before the write when the contract is violated.
> The assert fires before any write, so the soundness floor (no out-of-bounds write)
> is preserved; the difference is a loud abort instead of a silent drop. A consumer
> sizes the accumulator for its maximum per-frame appends; that is the contract.

The rounds of 13 November 2026 then make the refusal a value that is counted
and read at the close, and record op's answer that the consumer picks
between that and the assert
(`tenet::an_append_past_an_accumulator_s_capacity_is_counted_and_never_written_and_the_refusal_is_sticky_for_the_frame`,
`tenet::the_consumer_picks_on_the_run_configuration_whether_an_over_capacity_append_is_a_refusal_or_an_assert`).

`design_rounds/202611130554/202611130600_topic.overflow-is-the-consumers-choice.md:9-9`

> Asked through the coordinator whether an append past an accumulator's capacity should be a refusal as a value, with the options [a refusal as a value everywhere | the assert stays the default, refusal is a provider's opt-in | configurable by the consumer: a const, generic or trait parameter on the run configuration picks refuse or assert, default refuse], op answered, verbatim: "Configurable by the consumer".

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
if the kinds are counted by marker.

Op was asked this on 13 November with those two shapes as the options, and
answered "Whichever makes most sense typestate and plumbing wise". He
prefers neither as such. The api revision decides, by which shape gives the
cleaner typestate and plumbing, and says why. The rows are
`question::whether_the_accumulator_is_a_fourth_store_kind`, which holds both
passages, and
`question::which_shape_the_accumulator_takes_is_left_to_the_api_revision`,
which holds his answer.

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
