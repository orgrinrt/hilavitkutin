# The Supersessions Of The Api Placement Call

This read checks whether the supersessions that the maintainer's answer on the api placement called for went through hilavitkutin's canon flow with `252c5190`, and proposes them where they did not. It changes no row. It finds that both supersessions went through, that the four rulings stand as ratified rows, and that the one gap the flow left, a provisionally stamped sentence now standing held, is already a question in front of him.

It rests on call 3 of clause-dev `.shared/research/202611150830_stack-path/57-op-calls-on-the-night.md:38-52`, on `git show 252c5190 -- mock/registry`, and on the registry on `origin/dev` (`3177527a`) read with `cargo mock canon find` and `cargo mock canon check`. The diff of the registry was read line by line; the source moves of the same commit were not, as they are outside this read.

## The answer

The question put four placements two blind seats had agreed on, with several to be chosen: [WorkUnit stays in the api, with domain crates opt-in above it | Capability, sink, codec leave hilavitkutin, since Push and BulkPush there duplicate notko's sink; this supersedes one sentence of a provisional row | Engine-only items to the engine: dispatch_codegen, PoolFrame, WakeStrategy, Executor, ExecutorError, HybridExecutor and ceiling_div move into the engine, and adaptation becomes a candidate opt-in crate | Persistence builds on str and api, read as a dependency: MemoryProviderApi stays in the api, and the held agent-written persistence row is superseded]. His answer: all four chosen. The intent, as file 57 reads it, is that the placement stands in full and that the supersession of the provisional row's sentence and of the held persistence row goes through the canon flow, his answer being his word for it.

So the flow owed two supersessions and, by the shape the registry uses for a ruling, a row per ruling.

## What `252c5190` did

The four rulings are rows, each `standing = "ratified"`, `origin` from his answer, in `mock/registry/tenet/identity-3.toml:388-454`: `op_ruled_that_work_unit_and_the_unit_vocabulary_stay_in_the_api_crate_with_domain_crates_opt_in_above_it`, `op_ruled_that_capability_sink_and_codec_leave_hilavitkutin_and_a_consumer_takes_notko_s_sink`, `op_ruled_that_the_dispatch_codegen_surface_and_the_runtime_data_plane_live_in_the_engine_and_adaptation_is_a_candidate_opt_in_crate`, and `op_ruled_that_the_memory_provider_contract_stays_in_the_api_and_persistence_builds_on_str_and_api_as_a_dependency`. `cargo mock canon check` over the four reports nothing.

The provisional row whose sentence put capability, sink and codec in the api, `the_api_crate_carries_the_consumer_facing_types_and_traits_and_the_platform_contracts` (`identity-2.toml:196`, mechanism "The consumer contract surface is capability / sink / codec."), lost its stamp and gained `superseded_by` naming `the_api_crate_carries_the_unit_vocabulary_the_store_types_the_context_and_the_platform_contracts_and_nothing_only_the_engine_reads` (`identity-3.toml`). The registry supersedes whole rows, so the successor restates the first sentence and drops the second.

The held persistence row the answer names, `the_persistence_crate_is_the_generic_hot_cold_storage_bridge_as_a_standalone_ecosystem_crate` (`persistence.toml:21`), gained `superseded_by` naming the persistence ruling (`persistence.toml`, one added line). Its sibling `persistence-2.toml` had its `related` list brought up to date.

Two further rows of `principles-2.toml` were superseded in the same commit, on the bare primitive table and on trait signatures, each now naming notko's `Push` and `BulkPush` sink in its successor. They follow from the second ruling and are the class the supersession falsified, so the commit fixed the class and not only the instance named.

The remaining hunks of `plan-3.toml` move `source` and `provenance` paths to the closed round directory `202611191347`, which is bookkeeping and changes no claim.

Holds for: `252c5190` as reachable on `origin/dev` `3177527a`, the tenet namespace.

## The class search

To see whether a standing claim still asserts what the rulings forbid, the canon was asked by concept for each moved item: `canon find codec`, `capability`, `HybridExecutor`, `PoolFrame`, `WakeStrategy`, `dispatch_codegen`, `ceiling_div`. Every hit that places one of those items in the api is the superseded row above. The other hits place them in the engine, in the extensions layer (capability ids of the plugin host, which the ruling does not touch) or nowhere in particular. A grep over the mechanism fields of `mock/registry/tenet/*.toml` for the api crate beside any moved name gives only the superseded row and the ruling itself.

On the design side, `mock/crates/hilavitkutin-api/DESIGN.md.tmpl:1778-1782` now says a contract surface for items received or emitted outside unit execution "is notko's sink". No api design line places a moved item in the api.

One stale design line outside this read's scope was met: `mock/DESIGN.md.tmpl:82` still describes `PoolFrame<MAX_CORES, MAX_PHASES>`, while the provisional tenet `the_scheduler_scratch_is_sized_by_the_plan_dimension_capacities_and_the_three_fixed_constants_are_deleted_outright` says the fixed constants are deleted and the pool frame is sized from the plan dimensions. It is ticketed and not fixed here.

Holds for: the tenet namespace and the root and api design templates on `3177527a`.

## The one gap

The successor of the api row is `standing = "held"`. `cargo mock canon check` reports it: "is held: brought to the maintainer, it binds nothing until he backs it". The sentence it keeps stood on a provisional stamp before, and he did not touch that sentence, so it now stands one step lower than it did. This is already in front of him as agenda question `does_the_api_surface_sentence_op_left_standing_keep_its_prov` (clause-dev `.shared/scripts/life/tickets-hilavitkutin.toml:3045`), filed on 19 November by the helper that made the commit, with the options [stamp the held row provisional, as the sentence stood before | ratify the held row as written | leave it held until the canon walk reaches identity]. Nothing further is proposed for it here.

## What follows

Nothing is left to propose for call 3. The supersessions went through the flow, the class was fixed, and the open standing of the successor row has its question. The one stale root design line found on the way has its ticket.
