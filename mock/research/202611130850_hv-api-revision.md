Source: kaski channel (coord/kaski of clause-dev), att/20261113T085042186009579-hv-api-revision-c5b61a/report-hv-api-revision.md

# Report: hv-api-revision

State: hilavitkutin `work/api-revision` at `288811a`, pushed, tree clean. Round `202611130617-h6mb` is in the doc window, changelist unlocked. Nothing was edited for this audit. No source window, no `ready-doc`, held on the coordinator's word. `mock/registry/` untouched.

## Audit of the doc changelist against the founding record

Short names for the sources, all under `mock/` in hilavitkutin:

- C is `design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`, the founding consolidation spec, read whole (2480 lines).
- T5 is `design_rounds/202604200055/202603141800_topic.hilavitkutin-core-design.md`; T3 is `…/202603121800_topic.dispatch-and-optimisation.md`.
- LS, LI, LD, LY, LG are `design_rounds/202604200055/202603141800_research.loimu-storage.md`, `…loimu-incremental.md`, `…loimu-dag.md`, `…loimu-synthesis.md`, `…loimu-signals.md`.
- PAC is `design_rounds/202605101036/202605101036_topic.plan-algorithm-chain.md`. E7 is `design_rounds/202606111200/202606111200_topic.e7-dirty-skip-incremental.md`.
- HER is `research/202606210600_expert-loimu-heritage-storage-model.md`; ARC is `research/202605302130_unified-storage-design-archaeology.md`.
- A is `crates/hilavitkutin-api/DESIGN.md.tmpl` and E is `crates/hilavitkutin/DESIGN.md.tmpl`, both as they stand at `288811a`, so they carry the changelist's text. D is `DESIGN.md.tmpl` at the root.

Read whole: C, D, A, E, `PRINCIPLES.md.tmpl`, the twelve seed files, HER, ARC, E7, the workunit-thinking skill, the review of the changelist, the page `what-hilavitkutin-is.md`, `op-answers-9` and `op-answers-10`. Read in the parts that bear on stores, buffering, counts and order: T5, T3, LS, LI, LD, LY, LG, PAC. Not read whole: the other files of round `202604200055` and the later rounds. So "the spec" below is C read whole plus those parts, and a statement that something is absent is a statement about C and a grep over `mock/`, not about every line of every round.

The recovery branch `origin/recover/unlanded-rounds-and-benches` adds one round file beyond this branch, `design_rounds/202608090700/202608090700_changelist.doc.md`, on `hilavitkutin-sym` widths and nothing on stores. Its bench material (`swap_band`, `swap_cost`, `rcm_order`, `rcm_rivals`, `fiber_theory`, `wake_policy`) shows the engine already installs a value between frames (`replace_value`) that seeds a dirty cone, and runs a plan unit on the meta band (`benches/variants/swap_common/src/lib.rs`, header).

### Find (a): generations, counters, increments, buffering, downstream users

The engine's own counters, in the spec:

- C:861-865: "Per-morsel generation counter — bump on write, skip if unchanged", "Propagates through DAG — unchanged root skips all transitive dependents", "evict/inject APIs (R2) handle persistence to consumer storage".
- T3:909-915: "Each morsel has a generation number — a monotonic counter that increments whenever any row in the morsel's range is modified. The ColumnStorage (or a separate change-tracking layer) bumps the generation when a write occurs. Each chain records the generation it last processed for each of its input morsels."
- C:2139-2145: "Parallel arrays (SoA): separate contiguous `[u64; MAX_STORES]` for "before" and "current" stamps. Capture before-versions before WU runs, compare after. Changed stores = bitmask of actually-modified columns. Increment only on completion".
- C:615-619: "WUs buffer internally, write on return", "Downstream doesn't run until completion", "Version stamps increment only on completion".
- C:709-713, the epoch: "Global epoch u8, incremented per pass", "Reset = one u8 increment (not memset)". E:815-819 has the same today.
- C:854-859, the morsel's third axis: "X (records) × Y (columns) × T (temporal across frames)". T3:942-952: "T (temporal): how many frames this morsel's cached result remains valid".
- C:2406-2414 (R2): "hilavitkutin owns per-morsel generation counters and skip-unchanged DAG propagation logic".

One pass behind, as the record already does it for the engine's own data:

- T5:595-598: "The scheduler writes `Resource<SchedulerMetrics>` at the START of each pass, containing the PREVIOUS pass's metrics. This is always one-pass-behind data, which is consistent and non-paradoxical". T5:639-641: "The write happens at pass start → it's the first thing in the DAG. All readers naturally come after."
- PAC:1176 and :1181-1208, "Double-buffer pattern for AdaptWu snapshot publish": a resource `MetricsResource<T> { buffer_a: T, buffer_b: T, current: AtomicUsize }`, the meta unit at `ScheduleEnd` fills the other buffer and stores `current`. That is a double buffer written as a resource with a flipped index, by a unit on the meta band, in the ordinary DAG.
- C:781-786: "Approach 1 (snapshot-to-local): copy shared columns per trunk at phase transition. 1.6% overhead." D:90 and A:1440-1448: `ResourceSnapshot<'phase, R>`, a view "captured at a phase boundary; lifetime bounded to the current phase".

Double and triple buffering, as the record has it. It is loimu's, the named downstream user that does both:

- LS:187-197: "Non-temporal fields get 2 SlabColumns (frozen + live). Temporal fields get 3 (old + current + projected)", "Each column's SnapshotBuffer performs a dirty-region copy from live to frozen at pass boundary. Idle passes with no writes pay zero memcpy."
- LS:244-264: `SwapBuffer<T, Mode = Exchange>` with two modes, `Exchange` (pointer swap and reset) and `Snapshot` (copy the dirty regions).
- LS:700-711: "Buffer rotation at pass boundary: pointer swap (zero-copy), old becomes previous-old, current becomes old, projected becomes current."
- LS:824-850: readers "get the frozen snapshot (parallel)", writers "get the live buffer (exclusive)", and the edge table: "`read nodes M` | No DAG edge to writers (double-buffered) | Normal DAG edge (single-buffered)", "write vs write | Conflict (sequential)".
- LG:360-369: signal double buffering, record double buffering and temporal triple buffering are the "Same `SwapBuffer` primitive".

What the founding decision did with it:

- T5:50-61: "The goal is NOT to copy loimu's full architecture into a shared crate", loimu keeps "a thin layer that adds loimu-specific features (signal bus, rotation groups, temporal prediction) on top".
- T5:643-661 (Q6): "two modes only. Read and Write. Write implies Read.", loimu "maps its seven modes to Read/Write at the hilavitkutin boundary and implements the richer semantics in its own layer.", and the extension point: "If hilavitkutin later needs a third mode (e.g. ReadExclusive …), it can be added without changing the WorkUnit trait — it would be a new store wrapper type like `Exclusive<Resource<T>>` in the AccessSet."
- The research under it recommended carrying the double buffer and leaving the triple: LS:1068-1069 "SnapshotBuffer for read/write separation. The dirty-tracked sync with zero cost for idle passes is directly applicable."; LY:294 "SwapBuffer (Exchange + Snapshot modes) | Directly portable"; LS:1079-1081 "Temporal storage. Not needed in hilavitkutin … loimu should layer temporal columns on top of hilavitkutin's base storage."; LY:363 "exactly 2 versions … no write skew (DAG ordering prevents conflicting writes)".

Said plainly: C itself has no sentence on double or triple buffering (grep over C for double-buffer, triple, frozen, staged, `SwapBuffer`, `SnapshotBuffer`: one hit, C:2237, on topic files being frozen). The intent is in the research and T5 of the same founding round. Two later notes disagree with each other: HER:289-293 "Hilavitkutin's columns are double-buffered (frozen/live) for the pass boundary … There is no temporal prediction or triple buffering in hilavitkutin's design", and ARC:67 lists as stale "per-morsel `SnapshotBuffer` double-buffering (-> generation counters)". I cannot settle that from the text; both are agent notes of 2026.

Downstream users named: C:137-138 "cross-project (polka, saalis, loimu, hilavitkutin)"; C:1964-1970 "polka-dots | ~200 | deep", "saalis | 10K-100K | mixed", "loimu | 100K-10M | wide | phased with convergence"; C:2282-2283 "polka=full pipeline, saalis=full pipeline, loimu=scheduler abstractions only."; C:851-852 "Single-level sufficient for polka/saalis. Micro-morsel activates at ECS scale".

Where the writer says what "changed" means (owed from op's answer d). The passages are these; none uses the words "the writing unit declares it", the nearest is the developer's call on the write side:

- LI:110-121: the detection strategies table ("Developer hint | `.mark_unchanged()` | Zero cost, requires discipline") and "loimu uses bitwise comparison for resources under 4KB … For larger resources, the developer calls `.mark_changed()` explicitly; the default is "assume changed.""
- LI:749-753: "Version stamps as the change detection primitive … Adopt this directly. Phantom-write detection with bitwise comparison for small resources … Adopt this directly."
- LD:556-581 the same; LY:301 "Version stamps and phantom-write detection | Directly portable".
- C:2141-2145, compare after the unit ran; E7:22 "an explicit per-store change signal the consumer calls … When a tracked column writer lands later it can call the same primitive internally"; E:107-110 "an input the consumer marked changed".

### Find (b): prototypes, contiguity, indices and windows

The word "prototype" does not occur in hilavitkutin's record in this sense. Grep over `mock/` finds none; grep over `.data/op-responses/` finds it only for preview pages. The nearest passages, which I read as the same idea under other names:

- HER:138-144, op's earlier correction as quoted there: "Instead of batching by concrete type across all resources, batch by 'stored shape.' This way we can do all operations (including extrapolation) in a single pass.", and the note's gloss "if you batch by stored shape (marker combination), you need a mechanism that puts co-accessed columns in contiguous memory. That is the view system."
- LS:415-420: "A view template defines which columns are together. A view instance is a concrete allocation holding elements matching a particular marker combination". LS:431-446: "SoA-only. Each column within a view is a SlabColumn, dense-packed".
- LS:970-991: "Selection vectors (index arrays) filter elements by marker membership without copying data", "the selection vector contains the dense indices of matching elements within the view", "For views where all elements match the filter (common case), the selection vector is elided and iteration is a straight sequential scan". LY:303 "Selection vectors | Directly portable | Zero-copy filtered iteration via index arrays". T5:187-190: "collections use morsel iteration (column iterators with optional selection vector)".
- ARC:67: "loimu's dynamic view affinity / hysteresis / restructuring (-> static plan-time co-located fiber arenas)".
- C:79: "record | One data point in a column. The global identity. A column has N records. A morsel windows into a range of records." C:959-972: "All columns within a fiber share one arena allocation … Record index `i` advances one cursor covering all columns. Morsel boundary = one base pointer update." C:870 "Pre-partitioned morsel ranges at plan time".
- C:1462-1491: "hilavitkutin does not load data, does not allocate storage, does not know record counts ahead of time", "Recompute morsel splits at plan stage on any significant pipeline change (more records, new WUs, structural DAG change)". C:2047-2048 "If record count changed → full plan recompute." C:2442-2446 (R6): record counts are plan-stage parameters "from consumer data".
- C:2397-2404 (R1): "`Column<T>` as a typed storage slot. The `Column<In<IR>, As<T>>` schema-as-traits pattern is consumer-side … hilavitkutin doesn't know or care about domain schemas — it schedules based on Read/Write column sets only." C:2279-2281: "Does NOT own column storage, data model, schema".

So the record's answer to differing counts, as I read it: a population is contiguous per stored shape, its columns are co-located and share one record cursor (the fiber arena is the static form of loimu's view), a morsel is a window of record indices into it, an index array selects a subset where a subset is wanted, and the count is a fact of the consumer's storage that the plan reads and recomputes on. What the record does not say in words is what happens when one unit names two columns whose counts differ.

### Find (c): hints and write-write order

- C:589-595: "DAG edges from Write→Read column overlap", "Write-write ambiguity resolved by scheduling hint priority (Urgency, Divisibility, Significance). Tie-break: most deps first, then deterministic fallback. Supersedes T6 L2637 `order` field."
- C:1056-1059: "Higher-priority hint wins. Tie-break: most deps first, then deterministic fallback (type name or similar)." C:101: "order (on WU) | scheduling hints | WUs declare hints, scheduler derives ordering".
- T5:391-476, the three axes and their values ("Significance multiplies upward rank"). A:1271-1289 has them today.
- E:43-44 says something narrower: "WAW conflicts serialise deterministically by input index (the lower index first)."
- Code: `crates/hilavitkutin/src/plan/steps/step1_dag.rs:49`, `if s < t && writes[t].overlaps(&writes[s]).0`. The file reads no hint (grep for "hint": nothing). So the code is the last fallback alone.
- LS:847 "write vs write | Conflict (sequential)": even loimu's double-buffered layer serialises two writers, it does not refuse them.
- Op, 13 November [a: hints, most dependents, fixed fallback as founded, registration order a deviation to fix | b: hint priority, then most dependents, with registration position named as the fixed fallback | c: registration position, founding rule retired]: "Hints, registration as fallback". Reading: b.

### The four hypotheses

1. Is a minted index the handle pattern the model forbids, and what is the model's own answer to an unsound `write(i)`? As a hypothesis: `Index<'b>` is not that pattern. The skill's test is whether a thing refers to scheduler-owned data from outside a unit's declared access set; a branded position cannot leave the closure and names no store, the store is still named by the declared set. But it is not the model's own answer either. C:655-660 gives the surface in two layers: "Raw pointer access: `unsafe fn read<T>(&self, i) -> T`, `unsafe fn write<T>(&self, i, val)`" and "`ctx.each()` / `ctx.batch()` / `ctx.reduce()` op registration", with C:662-668 "Op author controls inner loop, framework controls morsel boundaries". The safe layer the model has is the one where the author holds no index: the engine iterates and the author's function sees values. C:2351-2357 removed the one accessor object the early design had ("ColumnHandle as a single-column typed accessor is eliminated"). And the reviewer's notes on the sketch stand against C8 as written: no fixture writes through `&self`, which C:656 and A:886-887 call load-bearing; the brand is not the bounds proof (`engine/src/morsel.rs:63,69` keep checked indexing); the sketch's stand-in types must not enter the api (`review-four-1.md`, section 4).
2. Is "frame-buffered" a store kind in the model's terms? No. T5 says three store types and one mechanism, and names a wrapper type in the access set as the way a new access mode enters (T5:658-661). `ResourceSnapshot<'phase, R>` is of the same family but does not express it: it is a view of a resource across one phase, not of a column across a pass. What the model has for a value across a pass is the pass boundary on the meta band, write on return, the generation counter, the epoch that resets by one increment, the one-pass-behind write at pass start (T5:595-598) and the flipped index of PAC:1181-1208. A generation read is those put together, not a fifth kind with a walk of its own.
3. How is write-write scheduled, and how does a new thing enter it? Find (c). One rule for every store. Nothing in the record gives any store a refusal in place of an order.
4. Does the value form keep `WorkUnit` as the one unit of code the scheduler knows? Not as written. C4 has `RecordOp` beside `WorkUnit`, a fifth router, and `Declares<W>` as a third trait that every engine fold binds on in place of `WorkUnit` (E:1259-1262). C:1181-1188: "What WUs must declare: 1. Read column set 2. Write column set 3. Scheduling hints … Everything else derived". D:86: "Per-morsel vs per-record WU dispatch inferred from AccessSet shape". C:660 has the per-record function as something a unit's body registers through `each`. In the model the value form is a way to write a `WorkUnit`, not a sibling of it.

### The forms, one by one

For each: what it rests on, where it leaves the model, and the restatement on what the spec has. The restatements are proposals for the coordinator, not edits; names in them are placeholders.

C1, five store kinds. Rests on: naming `Accum` closes a real gap, since E:1201-1229 describes an accumulator the api's "three stores" never named. Outside: `Buffered` as a fifth kind and `Extent<D>` as in effect a sixth, against T5's three types and one mechanism. Restated: `Resource<T>`, `Column<T>`, `Virtual<T>`, with `Accum<T>` named; no `Buffered`, no `Extent`.

C2, the shared list from `notko-hlist`. In the model, and it consumes the parked topic. Stands.

C3, record domains (`Column<T, D>`, `Domain { Axes, Reach }`, `Records`, `Extent<D>`, `build(storage)` with no count). Rests on a real need (kaski has tiles and beings in one scheduler) and on nothing in the spec. Outside: a second parameter on `Column` that says which set it belongs to is what R1 keeps consumer-side (C:2397-2404); axes and reach are a data model, which C:2279-2281 puts outside; `Extent<D>` stands beside a count the plan already has as a parameter (C:2442-2446). Restated: `Column<T>` keeps one parameter. A consumer that has two populations has distinct value types for them and names its groups in its own wrapper, as R1 says of polka. The count is read per column from the consumer's storage at the plan stage (`count(id)`, C:1462-1491), so `build(storage)` taking no count is right and survives. Columns of one fiber share a cursor and so share a count; the plan carries a count per fiber. A unit over columns of unequal count: the record has no words for it, and the two candidates are a plan error value or a window over the common selection. Subsets go through a selection vector, an index array, as LS:970-991 has it. Op's "Both, declared per domain" on whether an extent is fixed or replaceable then needs a place that is not a `Column` parameter: capacity is what storage reserved, the live count is the plan parameter, and "fixed" is a marker that lets the plan bake the window. Where that marker sits is the one thing here I could not place from the record.

C4, the value form (`RecordOp`, `ReadList`, `WriteList`, `Along`, `Vals<L>`, `RecordDispatch`, `Declares<W>`). Rests on: a pure per-record function is what the flattener wants (C:1097-1102, "`d(c(b(a(input_n))))`") and what `each` already registers. Outside: hypothesis 4; `ReadList` and `WriteList` are a second vocabulary beside `AccessSet`; schedule and hint move to a router where `WorkUnit` has them on the unit; `Declares` doubles `Plannable`. Restated: the value form is an authoring form of `WorkUnit`. A unit written as a function over values is a `WorkUnit` whose body is the engine's `each` over that function, by a blanket impl or a thin wrapper type, whichever a sketch shows coherent. `AccessSet` stays the one list, schedule and hint stay where `WorkUnit` has them, `COMMUTATIVE` stays a declared fact, and the planner, the bundles and the kits keep binding on `WorkUnit` alone. `Declares` and the fifth router go. Op's answer on reading a resource in this form, "Yes. Use typestate and statically branch accordingly" (reading: by value for a `Copy` scalar, by reference for a collection, chosen by the resource's kind in the types), fits this unchanged: the kind is `Field` against `Seq` and `Map`, which R5 already distinguishes.

C5, neighbour reads (`Shift<A, N>`, `Near<C, O, E>`, `Edge<V>`, `Committed`). Rests on: declaring the read is in the model's manner. Outside: it breaks "Records within a fiber are independent" (C:609-611) unless the plan knows; geometry is data model. Op: "Opt-in crate over the api". Restated: the core api gets one thing, a wrapper in the read set that says this read may land up to N records away from the record at hand (a reach in records, a const). The plan reads two facts from it: the read is not record-independent, so it is served either from a generation nobody writes this pass (no edge) or after a materialisation boundary behind the writer (C:1092-1093, and E:335-338 already lands a producer fiber's whole range before its consumer's first read); and the window's halo counts toward the size fold. Axes, offsets and edge values live in an opt-in hilavitkutin crate that turns them into that reach.

C6, the frame-buffered kind (`Buffered<T, D, Ch>`, committed and staged sides, `SwapBuffered` walk, `Changed`/`Differs`/`Written`). Rests on the premise that the record was silent, which op rejects, and on LS:824-850 for the one thing it got right: a read of the earlier generation builds no edge to this pass's writers. Outside: a fifth kind; a walk of its own in three run entries beside the meta band; a change rule as a type parameter of the store, where op says the writing unit declares it; tied to one author form. Restated: the store is a `Column<T>`. The generation is a property of the read: a wrapper in the read set (in the manner of T5:658-661) that says "as the last pass left it", with a const depth so that one back is loimu's frozen side and two back is its old. The engine keeps as many generations of a column as the deepest read any registered unit declares, which static composition lets it know at build (R6), so a column nobody reads back stays one buffer and pays nothing. Which buffer is which comes from the per-pass counter the kernel already advances, so rotation is one increment and not a walk over bindings (C:709-713, PAC:1181-1208). The two modes of LS:244-264 are the configurable part: rotate, where the writer writes every record every pass, or sync the morsels whose generation moved, where it does not; the review's point that a finer skip must keep the staged side whole is exactly the second mode. Across passes the reader's dirtiness needs no new seed: "Each chain records the generation it last processed for each of its input morsels" (T3:914-915). Triple buffering is depth two of the same wrapper. Whether hilavitkutin ships depth above one or leaves it to the consumer as T5:148-152 left loimu's temporal layer is a call I cannot make from the text; the wrapper costs nothing either way.

Changed, within C6. Op [a: every per-record kind, default differs for frame-buffered and written for in-place | b: frame-buffered kind only | c: every per-record kind, default differs everywhere | d: the writing unit declares it, not the store]: "Last option. Pretty sure this is somewhere in the research and proposed canon…". Reading: d. The passages are under find (a). Restated: a completed write bumps the generation (T3:909-912) by default. The writing unit declares otherwise in its own declaration: a wrapper on the member of its write set that asks the engine to compare after and bump only on a difference (phantom-write detection, LI:749-753), and for large values the explicit mark of LI:110-121 and E7:22. No parameter on the store.

C7, a second writer of a `Buffered` store refused at build. Outside, whole. Op [a: refused at build now, later disjoint record sets | b: always refused at build | c: resolved by scheduling priority like every other kind]: "Schedule should exist for this specifically. Are we sure that you and the agents working on this actually know what hilavitkutin is and is intended to be?" Reading: none as framed; the schedule orders the writers. Restated: no refusal. Writers of one store are ordered by hint priority, then most dependents, then registration position, for every store, and the later writer wins where both write a record. E:43-44 is corrected to that rule in this revision; `step1_dag.rs:49` implementing only the fallback is a deviation, filed as `hilavitkutin_orders_two_writers_of_one_store_by_registration`, to fix in the plumbing. The const assertion and its post-monomorphisation twin fall away.

C8, safe accessors (`Index<'b>`, `Block<'b>`, `PerRecord`, `InPlace`, `Index::mint`, safe `append`). Rests on: the typestate pillar, and the sketch's WORKS. Outside: hypothesis 1. Restated: safe exclusive writes are given by the form where the author holds no index, which is the value form and `each`; the engine scatters, and "write on return" is its contract. `batch` stays the form where the author controls the inner loop and keeps the founded `unsafe` pair with its contract stated, until a sketch shows a branded index writing through `&self` with the bounds proof named. `Index`, `Block` and `mint` leave this revision. The safe `append(&self, v)` with `len` stays: it rests on the capacity check the trunk landed, not on an index.

C9, formats generic and configured by the consumer. Holds op's words. Stands.

C10, registration stays typed, with the sketch, the bench and the question. Stands. R6 and C:2411-2412 ("no runtime column type creation") say the same. The question is with the coordinator, not built on.

C11, the engine section. Follows the others: the paragraphs on domains, `Declares`, `SwapBuffered` and the second-writer assertion go; a count per fiber read from storage, the generation read and its counter, the reach in the size fold, and the corrected write-write rule come in.

Kaski's field step under the restated forms, as a check that the need is still met: a unit reads its store one generation back with a reach, writes the same store, and the pass counter makes this pass's writes the next pass's earlier generation. Nothing in that unit names a kind, a domain or a swap.

### Where the changelist went wrong, in one place

It tested each form against op's words of 13 November and against the api as it stood, and took the map's rows for the record. It did not read C, T5 and the loimu research before designing, so it filled three "silences" (buffering, counts, write order) that the founding round had answered in intent, and it reached for new kinds and traits where T5:658-661 gives the one extension mechanism: a wrapper type in the access set.

### What I could not establish

- Whether hilavitkutin itself is meant to hold the double buffer (LS:1068-1069, LY:294, HER:289-293) or only the counters under it (T5:148-152, ARC:67). The restatement above works under both.
- What a unit over columns of unequal count should get. The record has no words for it.
- Where "fixed or replaceable, declared per domain" sits once a domain is not a `Column` parameter.
- Whether the blanket form of the value form is coherent with `WorkUnit<Schedule>` on the pinned toolchain. That is a sketch, not a reading.
- Op's word "prototype": absent from the record; I read it as stored shape or marker combination and say so.

These are for the coordinator. Per op's last answer none of them goes to him from me.

## Departures

- The sketch directory is stamped `202611130728_data-bound-registration`, a bare minute where the repo mints its own stamps.
- Item 8 (sketch, bench, note, question) was done in the doc window.
- `append` took an index in `8af52de` and is a safe method without one from `288811a`.
- `ready-doc 288811a` was posted before four queued coordinator messages were read, and withdrawn.
- The changelist's premise that the record was silent on buffering, counts and write order was wrong; this audit is the correction, and the changelist is not yet rewritten.

## Tickets

`approach_selection_takes_the_sum_of_the_domain_extents_where`, `the_first_sections_of_the_api_design_still_name_provider_pro`, `the_engine_crate_enables_generic_const_exprs_which_the_works`, `work_in_mockspace_stamps_a_bare_minute_where_a_repo_mints_a`, `stand_up_cuts_the_job_branch_in_the_kaski_clone_when_no_repo`, `a_product_over_a_list_built_by_member_name`, `a_readable_member_that_gives_a_record_its_own_key`, `arvo_implements_notko_hlist_s_cardinal_for_usize`, `the_bench_drivers_in_hilavitkutin_do_not_build_against_the_b`; questions `which_names_the_access_list_s_cell_and_end_take_in_hilavitku`, `which_registration_hilavitkutin_s_contract_takes_beside_the`.

Filed with this audit: `hilavitkutin_orders_two_writers_of_one_store_by_registration`, the write-write step reading only registration position where the founding rule is hints first (`step1_dag.rs:49` against C:1056-1059).
