# Consumer surface seat: the four loru requests of 19 November

**Date:** 2026-11-19
**Seat:** one expert seat (sebastian-aaltonen persona, opus), fresh and neutral brief, for the job `hila-consumer-surface` (stack-coord)
**Against:** hilavitkutin `origin/dev` of 19 November (the requests were filed against 252c5190)
**Probe:** `mock/research/sketches/202611191955_consumer-surface-seat/`, its outcome in `FINDINGS.md` there

The seat read four requests the consumer loru made of the engine (stack-coord's tickets 7c222795cf39, 288f4a33a359, ee112837c779 and bf100a60942a, loru-coord's asks 1, 2, 4 and 5), against hilavitkutin's canon (`canon_paths` in `mockspace.toml`), its designs and its code, and proposed for each the shape that serves its intent, or said why the request is wrong as filed. The seat could not write files itself; this report is its hand-back, transcribed by the job's helper without change of substance. The seat is the first expert on every canon-fit call here, and a second, independent read is owed on each before it is acted on.

Request numbering below is the seat's: request 1 is ask 1 (host column read and write), request 2 is ask 2 (several record sets), request 3 is ask 4 (byte arena) and request 4 is ask 5 (interner deduplication).

## 0. Gates

### Canon gate, per request

1. Host column read and write. Misaligned as filed.
   - Ratified rows rule out an engine-held column, a typed single-column accessor and slices: `the_consumer_provides_the_backing_memory_of_a_column`, `the_pipeline_run_returns_a_per_fiber_status_and_extracts_no_result_and_the_consumer_decides_on_partial_failure` ("consumer reads ColumnStorage directly"), `a_single_column_typed_accessor_is_eliminated_because_the_consumer_reads_storage_directly`, `the_engine_owns_dag_scheduling_dispatch_morsels_stages_stealing_and_progress_and_not_storage_schema_queries_ui_or_io`.
   - The intent behind it is ratified and unbuilt: `column_data_moves_out_of_and_into_the_engine_through_evict_dump_and_inject_import` (R2), `change_detection_is_per_morsel_and_skips_what_is_unchanged`, `the_engine_owns_the_generation_counters_and_the_skip_logic`.
   - Who marks a host write as changed is an open question (`who_says_that_a_store_changed`). That part is ambiguous and goes to the maintainer.
2. Several record sets in one schedule. Aligned as an arm, misaligned as a change to land now. `question::how_record_sets_of_different_counts_live_together` was answered "Bench both first". The reading recorded with it: "The api revision takes neither shape until a sketch and a bench of both exist."
3. Byte arena store. Misaligned as filed. A new store kind is against `there_are_three_store_types_and_one_scheduling_mechanism` (ratified) and `the_standalone_spec_lists_four_store_shapes_with_the_accumulator_among_them` (held). `no_subsystem_has_bespoke_storage_and_a_subsystem_needing_to_hold_n_things_uses_a_column` (provisional) names "a new arena when a subsystem needs to hold N things" among what it rejects. The intent is served by a byte record set plus a span column, which ties request 3 to request 2's bench.
4. Interner deduplication and reach. Partly misaligned, partly ambiguous. Deduplication is the design tier's law (`mock/crates/hilavitkutin-sym/DESIGN.md.tmpl:16-18`, "equal values give equal handles"), and the shipped arena breaks it. The canon is silent on where it lives. How a unit reaches the interner touches provisional rows whose mechanism text the seat proposes to change, which is the maintainer's call.

### Suite gate

The whole suite does not build under `cargo +nightly` at 1.102.0-nightly of 2026-11-15 (`arvo-strategy`: 119 "expected a trait, found type", exit 101). Under the pinned `nightly-2026-05-28` (`mock/rust-toolchain.toml`), `cargo test --workspace --no-fail-fast` gave 728 passed, 0 failed, 41 ignored; the ignored ones are catalogued reds. These totals are from the seat's run and were not re-run by the helper.

Test bodies read in the surfaces the requests touch, and what is wrong with them:

- `mock/crates/hilavitkutin-str/tests/interner.rs:54-61` (`runtime_intern_dedups`) is tautological about `StringInterner`. The deduplication it asserts is done by the test's own `VecInterner::arena_intern` (lines 22-33), which scans a `Vec<String>` and returns the old index. `StringInterner::intern` forwards to the arena (`src/interner.rs:83`), so the test proves that the fixture deduplicates. The shipped `MemoryArena` does not deduplicate, and no test asks it to. The same deduplicating fixture is copied into five test files (str `interner.rs` and `ergonomics.rs`, persistence `archive_str.rs`, `context.rs` and `cold_store.rs`), so every interning test runs over a stand-in that holds the law the shipped type breaks.
- `mock/crates/hilavitkutin-providers/tests/smoke.rs:88-91` (`internerkit_installs_via_with`) builds a builder and drops it, and asserts nothing; the interner cannot be built into a scheduler (probe result 7). Lines 79-82 only assert that `InternerKit: Kit` holds. Lines 23-39 check overflow at the arena only; the sentinel is never passed through `Str`, so the out-of-bounds resolve (probe result 5) is never reached.
- `mock/crates/hilavitkutin-api/tests/column_value.rs:45,54,63` (`sub_byte_*_reports_container_width`) are tautological: each asserts `BIT_WIDTH == USize(size_of::<T>() * 8)`, the trait-body default at `src/column_value.rs:30`. No test asserts the ratified sixteen-byte refusal, and the shipped blanket does not enforce it (probe result 11).
- `mock/crates/hilavitkutin/tests/incremental_skip.rs` is real, with a divergence twin at lines 183-199, but the law is sampled: `mark_dirty` is exercised on `Resource<_>` only (lines 105, 147), never on a column. Host inputs are written through the doc-hidden `__bindings().__tail()...__ptr()` (`tests/skip_chain/mod.rs:333-355`), so the host-populated column case the `mark_dirty` doc names (`src/scheduler/dirty.rs:154-157`) has no supported route and no test.

No argument below rests on any of these tests being green; each rests on canon rows, design text and the probe results.

## 1. Request 1: host-side column read and write, with a per-record dirty mark

### What the canon and design say

Ratified: `the_consumer_provides_the_backing_memory_of_a_column` (the consumer provides backing memory; storage is reached by raw pointers, never slices), `column_storage_returns_raw_pointers_and_the_stride_type_is_gone`, `a_single_column_typed_accessor_is_eliminated_because_the_consumer_reads_storage_directly`, `the_pipeline_run_returns_a_per_fiber_status_and_extracts_no_result_and_the_consumer_decides_on_partial_failure`, `the_engine_owns_dag_scheduling_dispatch_morsels_stages_stealing_and_progress_and_not_storage_schema_queries_ui_or_io`, `the_consumer_materialises_external_data_into_columns_through_its_own_loader_units_and_manages_storage_lifecycle`, `column_data_moves_out_of_and_into_the_engine_through_evict_dump_and_inject_import` (the names the canon gives the host path), and `the_engine_owns_the_generation_counters_and_the_skip_logic` with `change_detection_is_per_morsel_and_skips_what_is_unchanged`.

Provisional: `the_dirty_seed_is_set_by_the_consumer_marking_a_store_dirty_or_by_a_resource_replacement`, and `the_per_morsel_generation_counters_are_not_built_and_the_engine_has_a_coarse_per_store_dirty_only`, which asks for a design sketch before any code: where the counters live with no allocation, and how they compose with `store_dirty`. Open: `who_says_that_a_store_changed`. Answered: `which_stores_declare_what_changed_means` ("the writing unit declares"), which covers unit writes and says nothing about a host write. Two held, agent-written rows disagree: `columns_reserve_at_build_to_capacity_with_a_stable_pointer_and_the_per_frame_record_count_is_a_metadata_field` and `column_buffers_are_sized_by_the_per_run_record_count_and_belong_to_the_run_loop_and_not_to_build`.

### What the code does instead

`SchedulerBuilder::build(self, storage: CS, record_count: USize)` (`mock/crates/hilavitkutin/src/scheduler/builder.rs:173-176`) takes the consumer's `ColumnStorage` by value, keeps it as a private field (`scheduler/mod.rs:257`) and never gives it back. That is the engine owning column storage, against a ratified row, and it is why the only route today is `__bindings()` (`run_fused.rs:166`). The record count is fixed at build (`mod.rs:182-186`, `builder.rs:229`). StoreIds come from drain order (`resource/bindings/column.rs:31-40`), and a buffered column takes two of them, so a consumer cannot work out the id of `Column<T>` from type position even if it had the storage back.

### Verdict

`Scheduler::column::<T>() -> &[T]` and `column_mut::<T>() -> &mut [T]` should not exist: it is the eliminated single-column typed accessor, it returns slices where the ratified contract is raw pointers, and it puts the engine in the position of owner of the column. The intent is served by the ratified R2 surface (`dump`, `import`, `evict`, `inject`) in the engine crate, plus giving the storage back to the consumer. It belongs in `hilavitkutin` and not in the api: the api "carries no contract surface for items received or emitted outside unit execution" (`the_api_crate_carries_the_unit_vocabulary_the_store_types_the_context_and_the_platform_contracts_and_nothing_only_the_engine_reads`, held), and the data plane lives in the engine (`op_ruled_that_the_dispatch_codegen_surface_and_the_runtime_data_plane_live_in_the_engine_and_adaptation_is_a_candidate_opt_in_crate`, ratified). The sink position for copy-out is notko's (`op_ruled_that_capability_sink_and_codec_leave_hilavitkutin_and_a_consumer_takes_notko_s_sink`, ratified; `trait_signatures_return_iterators_and_take_notko_s_sink_and_never_name_a_vector_or_boxed_slice`, held, "bytes included").

### Proposed shape (hilavitkutin, methods on `Scheduler`, between frames, no public lifetime)

```rust
/// A record range of one store, checked against its live count when made.
#[derive(Copy, Clone)]
pub struct RecordSpan { start: USize, len: USize }

/// Stores a host may import into: Column<T>, Buffered<T, M> (the side the next
/// pass reads). Sealed; Accum, Virtual and Resource are not.
pub trait HostImport: Member + sealed::Sealed { type Value: ColumnValue; }
/// Stores a host may dump: Column<T>, Buffered<T, M> (frozen side), Accum<T> (live prefix).
pub trait HostDump: Member + sealed::Sealed { type Value: ColumnValue; }

pub enum HostRefused { PastCount { count: USize }, }

impl<Cfg: RunCfg, WuVals, Vals, CS: ColumnStorage, D: PlanDims, Stores, Clk>
    Scheduler<Cfg, WuVals, Vals, CS, D, Stores, Clk>
where Vals: StoreValues + BindingsFor
{
    /// R2 import: copy records in at `at`, and mark exactly what was written changed.
    pub fn import<S, I, It>(&mut self, at: USize, records: It) -> Outcome<RecordSpan, HostRefused>
    where S: HostImport, Stores: Locate<S, I>, I: WitnessIndex,
          <Vals as BindingsFor>::Bindings: HostSelect<S, I>,
          It: IntoIterator<Item = S::Value>;

    /// R2 dump: copy a span out into the consumer's sink.
    pub fn dump<S, I, P>(&self, span: RecordSpan, sink: &mut P) -> Outcome<(), HostRefused>
    where S: HostDump, Stores: Locate<S, I>, I: WitnessIndex,
          <Vals as BindingsFor>::Bindings: HostSelect<S, I>,
          P: notko::sink::BulkPush<S::Value>;

    /// The consumer's own storage, read directly as the ratified row says.
    pub fn storage(&self) -> &CS;
    /// The id of a plain column in that storage, resolved by the same witness
    /// the context projection walks (one lookup path, not two).
    pub fn store_id<T: ColumnValue, I>(&self) -> StoreId
    where Stores: Locate<Column<T>, I>, <Vals as BindingsFor>::Bindings: HostSelect<Column<T>, I>;
    /// After a raw write through the storage: say which records changed.
    pub fn mark_changed<S, I>(&mut self, span: RecordSpan)
    where S: HostImport, Stores: Locate<S, I>, I: WitnessIndex;
    /// Take the storage back when the scheduler is done.
    pub fn into_storage(self) -> CS;
}
```

`HostSelect` is the existing column selector witness (`ColSelector`) under a host-facing name, not a second lookup structure, so the host and the context resolve a store through one transform.

`mark_changed` with a span means per-record change, and that needs the per-morsel generation counters, which are ratified and not built. Until the design sketch the provisional row demands exists, the only honest implementation is the coarse store bit, shipped with a FIXME naming a ticket and the red test T1.4 below; a version that takes a span and quietly marks the whole store would fake it green. As the sketch's starting hypothesis: a fixed power-of-two granule G per store, independent of fiber windows (windows vary per fiber, which is the open question the row names), with G = 64 records matching the micro-morsel boundary of `micro_morsels_end_every_64_records_and_workers_sync_by_a_spin_budget_within_a_drift_of_32_records_with_no_futex_inside_the_loop`; one `Column<Gen>` of ceil(capacity / G) per tracked store, reserved through the consumer's `ColumnStorage`, so no bespoke storage; and a morsel clean when every granule it covers, for every column it reads, is at or below the fiber's last processed generation. Who may call `mark_changed` and `import` with a span, and what changed means for a host write, are not settled (Q1).

### Cost

`import` and `dump` move len times size_of(T) bytes, bounded by memory bandwidth; on top of that, `import` writes ceil(len / G) + 1 generation counters once they exist, or one bit of `store_dirty` today. The witness walk is compile time, and plan time and frame time are otherwise unchanged. `storage()` with `store_id()` is zero-copy and leaves the pointer arithmetic to the consumer, `unsafe` as `ColumnStorage::column_ptr` already is.

### What it forbids or makes harder

No borrow of column memory outlives the call, so a host cannot hold a view across `run`. A host write into an `Accum`, a `Virtual` or a `Resource` does not compile; `replace_value` stays the resource path. Writing past the live count is refused, not clipped. The parked-worker aliasing that FIXME `placeholders_tagged_74e2b9918149` names for `replace_value` (`dirty.rs:113-114`) applies equally to `import` after `run_parallel`, and the same fix has to cover both. `into_storage` ends the scheduler, deliberately, since the consumer owns the storage.

### Tests, each with the case it must fail on

- T1.1: import `[10, 20, 30, 40]` into `Column<A>`, run a unit `B = 2A`, dump `Column<B>`, expect `[20, 40, 60, 80]`. Fails if import resolves the wrong StoreId, for example by drain order instead of the selector witness.
- T1.2: the same round trip over `Column`, `Buffered<Snapshot>`, and `Buffered<Exchange>` at an even and an odd frame. Fails on the odd `Exchange` frame if import writes the live side.
- T1.3: import, then the cone runs and an unrelated unit stays skipped (the counts pattern of `incremental_skip.rs`); a twin imports without the mark and must diverge. Fails if import does not seed the change.
- T1.4 (red until the counters land, catalogued with a ticket): 1024 records, window 256, import one record into `[0, 1)`; only the first morsel of the reading fiber re-runs. Fails against coarse store dirty.
- T1.5: import at `count - 1` with two records returns `PastCount` and leaves a canary record past the count untouched. Fails if import clips or writes out of bounds.
- T1.6: a zero-length import returns `Ok` and marks nothing; the next frame skips everything.
- T1.7 (compile-fail, trybuild): `import::<Accum<T>>`, `import::<Virtual<V>>`, `import` of an unregistered column, and `dump::<Virtual<V>>`.
- T1.8: the T1.1 to T1.3 histories under `run`, `run_fused` and `run_parallel`, held to the conformance `Oracle`. Fails if any runner reads a different side.
- T1.9: `into_storage()` then a read through `ColumnStorage::column_ptr` at `store_id` equals the last `dump`.

## 2. Request 2: several record sets in one schedule, with a gather across sets

### What the canon and design say

`question::how_record_sets_of_different_counts_live_together`, answered "Bench both first", with the options as asked: (a) several sets in one schedule, each contiguous with its own count as a plan parameter, a fiber belonging to one set, a cross-set reach a declared read the plan orders; (b) one schedule per set, crossings through resources or accumulators the consumer carries. `question::where_the_record_count_of_a_column_lives`, answered: "Isn't the intent a contiguous per prototype and indices to process needed windows to it?", a third form the bench has to carry (form C below). `at_ecs_scale_the_consumer_holds_separate_schedules_per_component_set` (provisional, DF4): "the design must not assume a single all-encompassing run". Ratified: `the_scheduler_builds_edges_from_store_identities_and_ignores_cardinality`, `records_within_a_fiber_are_independent`, `an_op_that_needs_all_its_input_records_before_output_is_a_pipeline_breaker_and_a_materialisation_boundary`, `storage_is_contiguous_and_a_fiber_s_columns_share_one_arena`. `the_only_cross_record_surface_is_the_accumulator_append_path` (provisional) is already stale against the shipped `Reach<C, N>` window (`hilavitkutin-api/src/member.rs:93-99`, 192-269); a gather would be a second cross-record surface, and an unbounded one.

Design: `mock/crates/hilavitkutin-api/DESIGN.md.tmpl:300-307`, one record count per scheduler and no second parameter on `Column<T>`, FIXME `placeholders_tagged_8bcd16133d14`. This design waits on the bench and should stay as it is until the bench reports.

### Verdict

The api must not take `Column<T, Set>` or `Gather<..>` before the bench. Below are the forms that bench has to compare and what each costs; the seat does not pick one.

### Form A: several sets in one schedule

Type-level identity of a set, in one of three spellings the bench should also separate:

```rust
pub trait RecordSet: 'static {}                    // consumer ZST markers
pub struct Whole; impl RecordSet for Whole {}
// A1: a parameter on the marker, defaulted so existing code is unchanged
pub struct Column<T, S: RecordSet = Whole>(PhantomData<(T, S)>);
// A2: the set as an associated type of the value type (forces a newtype per set)
pub trait InSet: ColumnValue { type Set: RecordSet; }
// A3: no type identity; the engine sees per-column counts and refuses a unit
//     over columns of unequal count at build (option 3 of the count question)

#[repr(transparent)] #[derive(Copy, Clone, PartialEq)]
pub struct RecordIx<S: RecordSet>(U32, PhantomData<S>);  // 4 bytes, a ColumnValue

/// Read-list wrapper: the whole of `C`, read at indices from another set.
pub struct Gather<C>(PhantomData<C>);                     // Member::Store = C
impl<T: ColumnValue, S: RecordSet> Readable for Gather<Column<T, S>> {
    type Val<'a> = Table<'a, T, S>;
}
pub struct Table<'a, T: ColumnValue, S: RecordSet> { base: *const T, count: USize, _s: PhantomData<(&'a [T], S)> }
impl<'a, T: ColumnValue, S: RecordSet> Table<'a, T, S> {
    pub fn at(&self, ix: RecordIx<S>) -> Maybe<T>;        // one compare per gather
}
// builder: the count of each set, a plan parameter
// .records::<Beings>(USize(n_beings)).records::<Tiles>(USize(n_tiles))
```

The index column `Column<RecordIx<Tiles>, Beings>` is a separate member of the read list, since `Member` names one store and a gather reads two.

Plan-time cost: one count per set in the plan; a fiber cannot span sets, because a morsel window ranges over one count, so fiber grouping gains a split by set; a `Gather` read depends on the whole target column, which is a pipeline breaker under the ratified row, so every writer of the target lands in an earlier phase than the gatherer, which forces a waist unless that ordering already held. The edge itself is a store-identity edge like any other.

Frame-time cost: per gathered record, one dependent load of the index and one random load of the target. The morsel formula (`the_morsel_size_is_the_l1_budget_divided_by_the_written_bytes_per_record_within_fixed_bounds`) is unaffected because it counts written bytes, but `the_written_data_of_a_fiber_must_fit_the_l1_write_budget_and_all_accessed_data_must_fit_l1_and_l2_together` fails for any target larger than L2. Auto-vectorisation of the gathering loop is not to be assumed, and whether a hardware gather pays is one of the bench's measurements, not a premise.

Change detection: a gathering unit is dirty when any record of the target is dirty. Per-morsel skip through a gather needs an inverse index from target granules to reader morsels, which costs N reader records of memory per gathered pair.

Forbids or makes harder: `Reach` across sets is meaningless; an accumulator's capacity has to become per set (`accum.rs:112`, `cap: record_count`); A2 stops one value type from living in two sets; A1 doubles monomorphisation per set.

### Form B: one schedule per set, crossings carried by the consumer

No api change. A crossing is either a consumer copy between schedulers, which needs request 1's `dump` and `import`, or a small fixed-count set held as a resource collection: `Resource<Lattice>` with a `Seq<Tile, N>` member, live-streamed by pointer and length (`a_seq_or_map_value_is_a_pointer_and_a_length_and_its_elements_are_live_streamed_from_their_column_and_never_copied`, ratified). That second route is blocked today by the same collection wiring gap as request 4 (`projection.rs:156`, `T: ColumnValue`). Costs: one copy of N_reader times size_of(T) per crossing per frame; the schedules run in sequence unless the consumer runs them on disjoint pools; no cross-schedule skip, because the consumer must mark imports changed. Forbids any in-engine cross-set read, and makes request 1 mandatory.

### Form C: contiguous per prototype, with index lists choosing what is processed (the maintainer's own words)

One set per prototype. A unit's morsel walks an index column, a selection vector, instead of a contiguous record range: indirect dispatch. It composes with A or B. Costs: one index load per record processed, a loss of contiguity for the processed records, and a morsel sized by index count rather than record count. It does not need gathers for "which beings this frame".

### What the bench must hold to count

Kaski-like data (beings, tiles and items at the counts kaski expects). Arms: A1, A2, A3, B by copy, B by `Resource<Seq>`, C, and a hand-written plain Rust struct-of-arrays with an index gather as the competitor arm. Swept: target size against L2, index locality (sorted against random), the reader-to-target ratio, and the changed fraction per frame. Reported: frame time, bytes moved per frame, plan time, units and morsels skipped, binary size, and equality of every arm's output across `run`, `run_fused` and `run_parallel`. It lives in `mock/benches/` on the mockspace harness; a comparison anywhere else is a spike and cannot decide this fork.

### Tests that pin any winning form

A unit over columns of two sets fails to build (A1, A2) or is refused at build (A3). `Table::at` past the target count is `Maybe::Isnt`, failing if a stale index reads out of bounds. The writer of a gathered target is planned before the gatherer, failing with a registration that lists the gatherer first. Marking one target record changed re-runs every gatherer and no unrelated unit. Equal outputs at every worker count from 1 to the core count. Each of these at set counts 0, 1 and a count not a multiple of the window.

## 3. Request 3: variable-length per-record output, addressed by a `Copy` span

### What the canon and design say

Ratified: `there_are_three_store_types_and_one_scheduling_mechanism`. Held: `the_standalone_spec_lists_four_store_shapes_with_the_accumulator_among_them` and `an_accumulator_is_a_column_written_by_append_and_a_fourth_store_marker`. Provisional: `no_subsystem_has_bespoke_storage_and_a_subsystem_needing_to_hold_n_things_uses_a_column` rejects a new arena. Ratified: `the_sixteen_byte_limit_is_enforced_at_compile_time_and_forces_interned_identifiers_for_strings_and_paths` and `the_framework_constrains_the_inputs_of_an_op_to_column_values_with_native_stride_contiguous_and_non_aliasing`. Held: `an_accumulator_append_saturates_at_the_reserved_capacity_which_equals_the_record_count_at_build`, which the code follows (`resource/bindings/accum.rs:112`). Provisional: `each_core_appends_into_its_own_region_of_an_accumulator_and_the_main_thread_merges_the_regions_in_order` and `under_the_parallel_run_each_core_s_accumulator_region_starts_after_the_leading_records_and_keeps_its_own_refusal`. Ratified: `no_write_is_partial`.

### The premise needs correcting

The filer says rendered bytes have no home because `ColumnValue` is `Copy + 'static`. The shipped blanket (`hilavitkutin-api/src/column_value.rs:25,35`) has no sixteen-byte bound at all, against the ratified limit, and `Column::<[u8; 4096]>` registers today (probe result 11). So a fixed-maximum output already works, by drift; once the ratified limit is enforced, which it must be, it will stop working. Variable length has no home either way.

### Verdict

A new arena store kind should not exist. The intent is a byte record set, whose record count is the frame's total output bytes, plus a `Copy` span column on the producing set. It belongs in the api (the span and wrapper markers) and the engine (the scan, the per-set capacity and the merge). Whether the byte set can live in the same schedule is request 2's question: if form B wins, every span crosses schedules through a consumer copy. So request 3 is evidence for request 2's bench, not a separate decision.

### Shapes the decision has to choose between

```rust
#[derive(Copy, Clone)]
pub struct Span<S: RecordSet> { start: RecordIx<S>, len: U32 }   // 8 bytes, a ColumnValue

// 3A, sizes first (deterministic at every worker count):
//   pass 1: a unit writes Column<ByteLen> on the producing set;
//   pass 2: an engine-owned exclusive prefix scan writes Column<Span<Out>>
//           and sets the live count of set `Out` (capacity reserved at build);
//   pass 3: a unit writes bytes into its own span, disjoint by construction.
pub struct Scatter<C>(PhantomData<C>);        // write-list wrapper over Column<u8, Out>
pub struct Spans<C>(PhantomData<C>);          // read-list wrapper; Val<'a> = Run<'a, u8>
pub struct Run<'a, T: ColumnValue> { base: *const T, len: USize, _l: PhantomData<&'a [T]> }
impl<'a, T: ColumnValue> Run<'a, T> {
    pub fn len(&self) -> USize;
    pub fn at(&self, k: USize) -> Maybe<T>;
    pub fn copy_to<P: notko::sink::BulkPush<T>>(&self, sink: &mut P);
}

// 3B, append then rebase: Accum<u8, Out> with its own capacity (not the
//   producing set's count); the append returns a region-local offset, and the
//   merge, which already computes the region bases (provisional row
//   the_worker_s_region_bounds_and_the_merge_s_read_offsets_are_one_shared_computation),
//   rewrites each span by its region base.
```

3A costs either two renders per record or a length the producer can compute cheaply, plus one O(N) scan in the plan's sequence; the byte layout is identical at every worker count, which the conformance laws need. 3B costs one render plus an O(records) span rewrite after the merge; its layout is the sequential one only when no region refuses, and under a refusal the bytes kept depend on the worker count (the provisional row says so), and so would the spans. The host drains the byte set with request 1's `dump` into `&mut impl BulkPush<u8>`, the ratified sink position.

### What it forbids or makes harder

An accumulator's capacity is no longer always the record count, so the held row needs a successor. A span is valid for one frame; `Span<S>` carries no frame identity today, so either the type gains one or a test pins the misuse as a runtime refusal.

### Tests, each with the case it must fail on

The spans tile `[0, total)` with no gap and no overlap, at producing-set counts 0, 1, a window multiple and a non-multiple, failing if the scan is inclusive instead of exclusive. The sum of lengths equals the live count of the byte set. Each record's run equals a reference render. The bytes are equal across `run`, `run_fused` and `run_parallel` at every worker count (3A); for 3B the same with no refusal, plus a separate test pinning what a refusal yields. An output past the byte capacity is reported through the frame's refusal, never silently truncated. Zero-length outputs give `len == 0` spans and no bytes. `dump` of the byte set into a `BulkPush<u8>` equals the concatenation in record sequence. A gather or `Run` read past its span is `Maybe::Isnt`.

## 4. Request 4: the interner's deduplication, its reach from a unit, and the sentinel

### Deduplication: the filer is right, and it is worse than reported

`MemoryArena::arena_intern` (`hilavitkutin-providers/src/interner.rs:154-204`) appends unconditionally: five interns of one string give ids `[0, 1, 2, 3, u32::MAX]` (probe result 3). Through the shipped default interner, two interns of one runtime string give unequal handles, `Bits(2147483648)` and `Bits(2147483649)` (probe result 1), and the same through `Interner<StrDomain>` (probe result 2). That breaks the design law (`mock/crates/hilavitkutin-sym/DESIGN.md.tmpl:16-18`, "equal values give equal handles"), the code doc (`hilavitkutin-sym/src/interner.rs:12`) and the str doc (`hilavitkutin-str/src/interner.rs:15-16`).

It also defeats what interning is for in this engine. Handle identity is integer equality (`the_identity_handle_is_four_bytes_with_a_four_bit_kind_over_a_twenty_eight_bit_id_and_handles_of_different_kind_are_never_equal`, held), so a `Differs<Column<Str>>` over undeduplicated handles marks a rewritten equal string as changed. Persistence inject interns each stored pair (`hilavitkutin-persistence/src/archive_str.rs:76`; row `runtime_strings_persist_as_content_hash_and_bytes_pairs_and_the_content_hash_is_file_identity_while_the_sequential_id_is_session_identity`), so with the shipped arena two injects of one string give two identities. The canon is silent on where deduplication lives; the design tier says what it is.

Where it should live, as a design proposal with a second read owed: the law on the producer contract, and the mechanism once in `hilavitkutin-str`, over a byte store that only appends and resolves. The arena today is exactly request 3's byte arena: `[u8; BYTES]` plus an `Entry { offset, len }` span table. If deduplication is put into each arena, every arena carries its own copy of the hash index; the five deduplicating test fixtures are already that drift, made visible. One index type in str, used by `StringInterner` over any store, keeps one transform over one layout. The const section lookup (`lookup_const_by_value`, `hilavitkutin-str/src/interner.rs:122-130`) is a linear scan of the whole linker section on every runtime intern, after hashing the string; the same FNV-1a value can seed the deduplication index with the const entries at construction, so the const lookup and the runtime lookup become one probe on one hash. A bench arm has to confirm that win.

```rust
// hilavitkutin-str
#[repr(transparent)] #[derive(Copy, Clone, PartialEq)]
pub struct SpanId(Bits<28, Hot>);

/// An append-only byte store. Makes no deduplication claim; that is the interner's.
pub trait ByteArena {
    fn append(&mut self, s: &str) -> Maybe<SpanId>;      // Isnt when full; a write is &mut
    fn get(&self, id: SpanId) -> Maybe<&str>;            // bound-checked against the live count
    fn count(&self) -> USize;
}
/// Fixed-capacity open-addressed index of (hash28, SpanId), SLOTS a separate
/// const parameter (no generic_const_exprs: not computed from ENTRIES).
pub struct DedupIndex<const SLOTS: Cap> { /* [(Bits<28, Hot>, SpanId); SLOTS] */ }

pub struct StringInterner<A: ByteArena, const SLOTS: Cap> { arena: A, index: DedupIndex<SLOTS> }
impl<A: ByteArena, const SLOTS: Cap> StringInterner<A, SLOTS> {
    pub fn intern(&mut self, s: &str) -> Maybe<Str>;     // equal strings give equal handles
    pub fn resolve(&self, h: Str) -> Maybe<&str>;        // Isnt for an id this interner did not issue
}
```

Cost: SLOTS times 8 bytes of index, one FNV-1a pass per intern (which the const lookup already pays), and at load 0.5 or below expected probes under two plus a byte compare on a hit; one compare on resolve; nothing at plan time. The alternative placement is a law on `ArenaInterner`, with an index inside each arena: cheaper to land, and it keeps the five-copies shape.

### The overflow sentinel through the 28-bit handle

`arena_intern` returns `u32::MAX` on overflow (`providers/src/interner.rs:146,163-165`). `StringInterner::intern` wraps it with `Bits::<28, Hot>::from_raw(id)` (`str/src/interner.rs:87`), and `Sym::new` masks it into the id field, so the handle carries `0x0FFF_FFFF` (probe result 4). `arena_resolve` compares the id against `u32::MAX` (`providers/src/interner.rs:211`), which never matches, and then reads entry 268435455 of the table without a check (`:214-221`); running the resolve alone kills the test process with SIGSEGV (probe result 5). `StringInterner::resolve` documents the runtime branch as always `Maybe::Is` (`str/src/interner.rs:101-108`). Two different strings that both overflow come back as one handle (probe result 6), which silently violates identity, exactly what the sym design's `Generator` refuses to do at its ceiling (`hilavitkutin-sym/DESIGN.md.tmpl`, `mint` returning `Maybe::Isnt`).

Fix: delete the sentinel and return `Maybe<Str>` from `intern`; bound-check `resolve` against the live count (a `Str` from another interner instance is indistinguishable by bits, so an unchecked resolve is out of bounds for any foreign handle too); add a const assertion that `ENTRIES` is at most 2^28, above which ids alias silently. The canon row describing the sentinel, `the_memory_arena_is_an_inline_storage_interner_with_const_generic_byte_and_entry_caps_and_overflow_returns_a_sentinel_id` (provisional), is a modification target, and changing it is the maintainer's call (Q4).

### How a unit reaches the interner: the filer's reason checked

"StringInterner is not Copy so cannot be a Resource" is wrong about the contract: `Resource<T>` has no bound (`hilavitkutin-api/src/store.rs:18`), `Resource::new` takes any `T` (`:46`), and `StagedResource<T>: BuilderInput` asks only `T: 'static` (`:69`). It is right about the engine: the drain requires `T: ColumnValue`, that is `Copy` (`hilavitkutin/src/resource/bindings/resource.rs:75`); the footprint fold requires `ResourceFootprint` (`plan/project.rs:340`); the projection requires `T: ColumnValue` and copies the whole value into every context (`dispatch/engine_ctx/projection.rs:156`, behind FIXME `hilavitkutin_7_unmarked_placeholders`: collection members are not wired, and no mutable resource surface exists). Building a unit that reads `Resource<StringInterner<MemoryArena<256, 16>>>` fails with E0599 on `build`, naming the unmet `Copy` and `ResourceFootprint` bounds (probe result 7); a local newtype carrying the footprint still fails on `Copy` (probe result 8). No crate can write `ResourceFootprint for StringInterner<MemoryArena<..>>`: the type is str's, the trait is api's, and neither depends on the other.

The canon says an interner is a resource and how its value is shaped: `the_standalone_spec_lists_four_store_shapes_with_the_accumulator_among_them` (held), "Providers (a clock, a configuration, an interner) are Resources"; ratified `a_resource_is_scalars_and_const_sized_collections_only` (Field, `Seq<T, N>`, `Map<K, V, N>`, with elements not held to sixteen bytes), `a_resource_is_a_handle_and_not_an_inline_value_store_which_is_the_corrected_reading_of_r5`, `the_value_of_a_resource_is_one_contiguous_record_blob_and_the_per_member_column_decomposition_was_refuted_by_bench`, and `the_resource_handle_given_to_consumers_stays_a_reference_while_the_internals_use_raw_pointers_through_a_stack_local_cache` (`ctx.resource::<T>() -> &T` and `ctx.resource_mut::<T>() -> &mut T` are the consumer API). An interner is a well-formed R5 resource: Fields for the cursor and count, a `Seq<u8, BYTES>`, a `Seq<Span, ENTRIES>` and a `Map` for the index. The wall is the unbuilt collection wiring (#344) and the unbuilt write-back half, not a law.

Shape: resolve is a read (`Read` lists `Resource<Interner>`, and the unit calls `ctx.resources().resource::<Interner, _>()`); intern is a write (`Write` lists `Resource<Interner>`, through the ratified `resource_mut`). `InternerApi::intern(&self)` (`providers/src/interner.rs:32-40`) mutating through `Cell` behind a shared reference is a write hidden behind a read declaration, and under `run_parallel` two readers on two cores would mutate a `!Sync` arena at once. The reason the provisional row `the_interner_provider_trait_parallels_the_memory_and_clock_provider_traits_and_the_value_lives_in_a_resource_without_a_thread_safety_bound_in_the_first_version` gives, that the engine's resource access is sequential during single-thread morsel dispatch, has been false since `run_parallel` shipped.

The cost is that interning units are writers of one store, so the plan puts them in sequence. The alternative the maintainer should weigh is two-phase: units append candidate strings into the byte set of request 3; a maintenance unit at the band end interns them in merge sequence and writes `Str` into a column; handles for new strings are visible one band later, and interning runs in parallel and stays deterministic.

### `HasInterner` cannot be implemented as shaped

`HasInterner` (`providers/src/interner.rs:47-53`) is implemented nowhere in the workspace. An impl for the engine's context can be written in neither crate, since neither depends on the other. A blanket impl in providers over the api's resource accessor fails with E0207 on `R`, `A` and `I`, and E0210 on `C` (probe result 9). The same reach as a generic method on a blanket extension trait over `ResourceProviderApi` builds in providers (probe result 10); it is a renamed `resource::<StringInterner<A>, I>()` and adds nothing over the generic accessor. The provisional row `a_unit_that_handles_interned_strings_declares_the_interner_accessor_in_its_context_bound_and_the_accessor_lives_in_providers_because_the_interner_is_opt_in` licenses `HasInterner`; deleting it or recasting it is the maintainer's call (Q3).

### Tests, each with the case it must fail on

- The deduplication law over a const string, a runtime string, the empty string, and two strings that collide in 28 bits; through `InternerApi::intern`, `Interner::<StrDomain>::intern` and `intern_static`; at arena shapes including `ENTRIES = 1` and `BYTES = 0`. Fails against today's `MemoryArena`.
- Full arena: `intern` returns `Maybe::Isnt`, and every handle issued before still resolves to its string. Fails against the sentinel.
- Two distinct strings past capacity never compare equal as handles.
- `resolve` of a handle from another interner instance is `Isnt`. Fails against the unchecked read.
- `ENTRIES` above 2^28 does not compile.
- A `StringInterner` over a store that does not deduplicate still deduplicates. This replaces `runtime_intern_dedups`, which proves only its fixture.
- At unit level, under `run`, `run_fused` and `run_parallel` held to the `Oracle`: a unit that interns (`Write`) and a later unit that resolves (`Read`) in the same frame see the string.
- A unit that lists the interner only in `Read` and calls `intern` does not compile.

## 5. How the four relate

They reduce to three mechanisms, and building any of them once, to full spec, serves at least two requests.

An append-only byte store with a span table serves requests 3 and 4. `MemoryArena` is already that store. The interner is that store plus a deduplication index, persistent across frames; rendered output is that store with no index, reset per frame. A 28-bit `Str` id is a `RecordIx` into the interner's span table, and a `Span<Out>` is a range into the byte set. One `ByteArena` contract and one index type cover both; two arenas would be two layouts drifting apart.

A record set with its own count, addressed by an index typed by its set, serves requests 2 and 3, and request 4 as resource collections. The gather index, the span start and the interner id are all `RecordIx<S>`. The byte set of request 3 is a second record set whose count is computed inside the frame, the hard case for form A of request 2, and the bench should carry it as an arm.

The ratified R2 host surface (`dump`, `import`, `evict`, `inject`) and the per-morsel generation counters serve requests 1 and 3 (draining output into `BulkPush<u8>`), form B of request 2 (every crossing), and persistence. Per-record dirty and per-morsel skip through a gather need the same counters.

So the downstream-unblocking piece to take first is the generation-counter design sketch the provisional row demands. It decides the dirty semantics of request 1, the skip semantics of form A, and the change semantics of the byte set. The bench of request 2 decides whether request 3's byte set shares a schedule.

## 6. Questions for the maintainer, with options

- Q1 (request 1, open as `who_says_that_a_store_changed`). For a host import, who says what changed, and at what grain? (a) the import marks exactly the records it wrote, at the generation granule; (b) the import marks the whole store (today's coarse bit) until the counters exist; (c) imports compare against the held value and mark only differences, which needs `PartialEq` on the value.
- Q2 (request 1). Of the two held rows on reservation timing, which stands: build-time capacity with a per-frame live count, or per-run sizing? A pass scheduler whose counts change per frame needs the answer before `import` can grow a column.
- Q3 (request 4). Is `HasInterner` deleted, since it duplicates the ratified resource accessor and is unimplementable as shaped, or recast as the generic-method extension trait?
- Q4 (request 4). Does interning go through the write side, which serialises interning units, or through two phases, append then fold at band end? And does the provisional `InternerApi` mechanism change to `intern(&mut self, &str) -> Maybe<Str>`?
- Q5 (request 3). May a record set's count be set inside a frame by an engine scan, as `record_counts_are_known_when_loaders_complete_and_cascade_downstream` (provisional) suggests, or must the byte capacity and count both be fixed at build?

## 7. Unlicensed mechanisms noticed, including outside scope

- The `ColumnValue` blanket has no sixteen-byte bound (`hilavitkutin-api/src/column_value.rs:25,35`), against two ratified rows; `[u8; 4096]` is a column value (probe result 11), and no test asserts the refusal.
- The scheduler owns the consumer's `ColumnStorage` (`builder.rs:173-176`, `mod.rs:257`), against the ratified rows that the engine does not own storage and the consumer reads it directly.
- `mark_dirty` documents a host-populated column (`dirty.rs:154-157`) with no supported way to populate one; tests and benches reach columns through `#[doc(hidden)]` accessors (`tests/skip_chain/mod.rs:333-355`, `benches/support/evs-support/src/arena.rs:92-110`).
- `InternerApi::intern(&self)` writes through a shared reference (`providers/src/interner.rs:35`); if the resource could be registered, that would be a write outside the declared write set.
- `HasInterner` is implemented nowhere and can be implemented nowhere (probe result 9).
- Every runtime intern walks the whole const section linearly (`str/src/interner.rs:122-130`).
- The api design names `ExtensionSurface<dyn LinterApi>` (`hilavitkutin-api/DESIGN.md.tmpl:46,317`) while the repository's agent instructions ban `dyn` in every crate. Whether a canon row licenses it as a type-level key only was not traced.
- `cargo +nightly` does not build the workspace at today's nightly; the pinned channel does. Either the hook's toolchain rule or the arvo pin needs reconciling.
