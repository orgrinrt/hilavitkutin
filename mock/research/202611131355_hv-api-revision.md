Source: kaski channel (coord/kaski of clause-dev), att/20261113T135555942989316-hv-api-revision-6db012/report-hv-api-revision.md

# hv-api-revision: round one's doc changelist, fourth writing, on the frozen and live split

Branch `work/api-revision` of `orgrinrt/hilavitkutin`, head `e2becf26e75155dd1498a6fa511e6dff1c5a17b3`, pushed. Round `202611130617-h6mb`, phase DOC, changelist unlocked at `mock/design_rounds/202611130617-h6mb_changelist.doc.md`. No source is touched. `cargo mock` passes with every lint. Two commits since `94abd16`: the sketch (`5a2c531`), then the designs, the changelist and the topic (`e2becf2`).

The search found the double buffering. Change 1 is built on it and nothing about it is asked of anybody. The changelist carries every passage by path and line, his answers verbatim with their options, and both readings of the record where they differ.

## Step 1: what the search found

Searched by concept (frozen, live, committed, snapshot, generation, swap, flip, double buffer): `origin/dev`, `origin/recover/unlanded-rounds-and-benches`, `origin/work/recover-landing-swap`, `origin/work/loop-core`, `origin/work/bench-wide`, `origin/work/bench-convergence`. Read whole: the swap bench arms, the swap spec and its evidence, the founding research on loimu's storage, loimu's own buffer design at its repository. Also the research and sketches on the trunk, `.data/op-responses`, ikiuni, and `cargo mock unearth "double buffer"` in kaski (hilavitkutin has no such tool yet).

Found, for resources, in the record. The spec has a unit's loop read a snapshot taken before it, with the canonical value elsewhere: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1714-1723` ("Snapshots all accessed resources to a stack-local array [...] After the loop, mutable resources are written back to canonical storage"). The storage addendum keeps that for a scalar and stops it for a collection (`mock/research/202606210600_resource-storage-model-canonical-addendum.md:16-45`).

Found, the swap that has benches. `mock/research/202607200200_replaceable-swap-semantics-spec.md`: S1 (`:24-38`) installs the new value as one write, marks the store dirty, "legal only between frames"; S3 (`:60-66`) "a swap never changes the base pointer"; S5 (`:91-98`) `replace_value` as the every-frame path. Its arms, on `origin/work/recover-landing-swap`: `mock/benches/variants/swap_common/src/lib.rs`, `sb_none`, `sb_plan`, `sb_value`, `mock/benches/bench.toml:846-887`. The mechanism they exercise: a context carries a snapshot list of the resources it reads, the swap installs into the canonical blob between two frames, and the timed frame after it pays for the cone the dirty mark seeds. Results and evidence on the trunk: `mock/benches/results/swap_band/`, `swap_cost/`, `mock/research/202607201100_swap-semantics-ratification-evidence.md:30-58`. They measure a resource value's swap and no column's.

Found, the frozen and live split of columns with its mechanism, in the founding research (agent readings of loimu, with a recommendation). `mock/design_rounds/202604200055/202603141800_research.loimu-storage.md`: `:191-197` two columns per field, "a dirty-region copy from live to frozen at pass boundary. Idle passes with no writes pay zero memcpy"; `:244-251` `SwapBuffer<T, Mode = Exchange>` with `Exchange` (pointer swap) and `Snapshot` (copy); `:337-338` the boundary needs no lock since nothing writes there; `:826-828` readers get the frozen side, writers the live one; `:502-634` deferred writes with fusion; `:1059-1069` "What hilavitkutin should adopt", items 2 and 4; `:1153` "hilavitkutin should own the buffer primitives". `202603141800_research.loimu-synthesis.md:294` "Directly portable | Double-buffering primitive for any column data". loimu's own design has the same pair (`mock/crates/loimu-buffers/DESIGN.md.tmpl:82-108`, `:166-181`) and no bench.

Found, later agent work: a double buffer in one resource value flipped by a meta unit at `ScheduleEnd` (`mock/design_rounds/202605101036/202605101036_topic.plan-algorithm-chain.md:1181-1208`), which cannot be written against the designs.

Found, in the consumer's canon: kaski's rows `a_field_over_space_steps_whole_and_its_step_is_one_accounted_change` and `change_is_accounted_and_lands_together_at_the_frames_close`, and ikiuni's `a_mutation_applies_at_the_next_frame_barrier_by_a_double_buffer_swap`. The triple buffering he remembered is ikiuni's renderer mailbox (`mailbox_is_triple_buffered`), across a thread boundary, and no column store.

Not there, with the searches above run: any design, sketch or bench in hilavitkutin of a `Column` in two buffers, on any of the six branches; any bench anywhere of a column's swap. The trunk holds the recovered record bar one changelist that is not about this.

Two readings of who holds the buffer are quoted in the changelist. The core design topic (`:643-661`) leaves loimu's richer modes to loimu's layer. The founding research says hilavitkutin should own the buffer. His "loimu is an old reference [...] Hilavitkutin is primarily the replacement for it" takes away the layer the first leaves it to, and the same passage of the core design topic names the way in: "a new store wrapper type [...] in the AccessSet".

## Change 1, restated

A fifth store marker, `Buffered<T, M = Snapshot>`: a column held in a frozen and a live side. Every read of it, by a unit of any rank, sees the frozen side. Every write lands on the live side. A reader and a writer of it take no read-after-write edge and may run at once, one unit may be both, and what a unit writes in one pass every unit reads in the next. Two writers are ordered as for any store.

The mode is a sealed type parameter with the research's two names. `Snapshot` copies the live side into the frozen one and is right under any writer, which is why it is the default. `Exchange` trades the sides with no copy and is right only where every writer writes every record, so a buffered column is reached through the value function alone, which is applied at every record of a unit's range.

The engine does the swap. It runs after the last unit of the close with the workers parked (the swap spec's between-frames point), as plumbing, only for a store the pass changed, and marks it dirty for the next pass. No base pointer moves: `Exchange` is an index flip the dispatch reads when it binds the next frame's contexts.

`Reach<Buffered<T, M>, N>` reads frozen neighbours, so a unit may write the store it reads with a reach and no phase boundary is needed. That is the field step.

What is the record's, what is the research's and what this seat puts together is said in the changelist in so many words. Put together here: the pair as a store marker, the mode as a type parameter, the no-edge rule in plan step 1, the boundary as engine plumbing, `Snapshot` as the default. No bench stands under it.

## Change 4, restated

Both behaviours, configurable per store by the unit that writes it: `Written<C>` counts the store changed wherever the unit wrote, `Differs<C>` only where the value differs (it asks `T: PartialEq`). Each holds for the value function and for the positional `write`, which now follows the member (`(W, Self): WriteAt<T, I>`, the call still `write::<T, _>`). A bare marker takes a default by how it is written: `Differs` under a value function, which is his chosen option, and `Written` under the positional `write`, as on the trunk, until the bench says otherwise. The bench is ticket 29cdc662ca14.

## Maintenance units, restated

No write of their own, on his answer that hilavitkutin does not natively need one. A unit is a special case by the data it writes: a write in place to a plain column is seen later in the same pass. The two ways to keep a change out of sight until the pass is over are the two of his answer. The second write buffer is the buffered column's live side. The queued operations are appends to an accumulator that a close unit reads through `AccumReaderApi` and applies. The contract holds no unit to either; a consumer states its own rule as a bound over its units' write lists.

## The split proposed, with reasons

Hilavitkutin holds the pair of sides, the frozen read and the live write, the no-edge rule, the swap at the boundary with its dirty mark, the queue store with its reader for a close unit, and `replace_value`. Only the scheduler holds a column's memory and base pointers, so only it can keep two reservations under one store and flip them with no pointer moving under a context. The swap has to seed the next pass's dirty set, which is the engine's. That nothing reads or writes during the swap is the engine's parked-workers point. And the plan has to know a reader and a writer of such a store are not ordered, which a consumer's two plain columns and a carrying unit cannot tell it; that form pays a whole copy every pass and an edge it does not need.

kaski holds what a mutation is, the units that drain its queues, the journal and the conservation check, which of its stores are buffered and in which mode, and the rule that its ordinary units never write committed state in place. Those are meanings and policy, and the record keeps a consumer's schema out of the engine (the spec, `:2397-2404`).

A drain that is vectorised and fused before it is applied is the consumer's close unit to write for now; nothing in the engine fuses appends.

## Evidence

`mock/research/sketches/202611131330_buffered-sides-and-write-members/`, outcome WORKS. Ten tests green; three mutants (read the live side, exchange copies, swap when nothing was written) each turn tests red; two functions that must not compile are refused (E0277 at `src/lib.rs:320` and `:333`). It shows both modes giving one result under a total write, the regions never moving, a partial write right under `Snapshot` and stale under `Exchange` (`[8, 7, 7, 7]` against `[8, 0, 0, 0]`), and one `write::<T, _>` reaching a column listed bare, under `Written` and under `Differs`. Not shown: readable refusal messages, the real engine bridge, the planner's rule, the boundary's seed, any cost.

## Breaks counted

Unit authors, two: `append` loses `unsafe`; a one-value `RecordOp` is written over one-member lists. Scheduler builders, none. Runner and context authors, five: `EachApi` gains `map`; folds read `Member::Store`; `ResolveAccumAppend` becomes an `unsafe trait`; `ResolveColumnWrite` gains `resolve_held`; folds over store markers answer for a fifth. Kit authors, one where it applies (`HoldsAll`). Three changes of behaviour with no change of signature, as before. The positional `write` changes its bound and not its call or, for a bare column, its behaviour.

## Unchanged from the third writing

Record counts left out on "Bench both first" (2). Two writers by hints, dependents, registration (3). `Reach` over a plain column and geometry in an opt-in crate (5). `Accum<T>` as a marker (6). Typed registration (7). The value function as `map` on `EachApi` (8). The shared list (10), formats (11). The three review passes are spent and no further one was run.

## Departures

From the brief: items 1, 3 and 5 are not built as written (nothing for domains, a reach in records, the positional `write` stays `unsafe`). Item 4, a frame-buffered kind, is now in. From the flow: the mutation order, on his "Revise the api in place now".

## Tickets

Answered by him and their markers out of the design text: 2143d534c715, 29426931e825, 7f93b9f3e621. With coord: 3df7466ff25a.

Filed for this writing, as tasks: `bench_the_positional_write_s_changed_rule_in_hilavitkutin_co` (29cdc662ca14), `positional_read_and_write_of_a_buffered_store_in_hilavitkuti` (24fa3df63a9d). Bumped: ecb940c91d26 (now the bench of `Snapshot` against `Exchange`), 8e2e2326d588 (gains the copy per morsel under `Snapshot`).

Standing: `sketch_and_bench_several_record_sets_in_one_schedule_against` (8bcd16133d14), `an_opt_in_hilavitkutin_crate_gives_axes_grid_shapes_and_edge` (c7dbcaeb1beb), `bench_the_value_function_under_each_against_the_position_clo` (f8955d2897cc), `hilavitkutin_orders_two_writers_of_one_store_by_registration` (4b4f40f32a11), `hilavitkutin_s_two_local_lints_do_not_build_against_the_mock` (92d416895392), `writing_style_counts_the_exclamation_mark_of_an_html_comment` (261222dc6493), `design_doc_source_mismatch_blocks_a_push_in_the_doc_phase_fo` (f2294fa7f7dc), `arvo_implements_notko_hlist_s_cardinal_for_usize`, `which_names_the_access_list_s_cell_and_end_take_in_hilavitku`, `which_registration_hilavitkutin_s_contract_takes_beside_the`, `a_readable_member_that_gives_a_record_its_own_key`, `a_product_over_a_list_built_by_member_name`, `the_first_sections_of_the_api_design_still_name_provider_pro`, `the_engine_crate_enables_generic_const_exprs_which_the_works`, `the_bench_drivers_in_hilavitkutin_do_not_build_against_the_b`, `work_in_mockspace_stamps_a_bare_minute_where_a_repo_mints_a`, `stand_up_cuts_the_job_branch_in_the_kaski_clone_when_no_repo`. To close: `approach_selection_takes_the_sum_of_the_domain_extents_where`.

## What waits on the coordinator

The word to lock and open the source window. `replace_value` on the trunk still does not install the value; the landing of the recovery branch owes it and this job does not touch it.
