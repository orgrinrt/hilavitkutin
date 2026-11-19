# Op's call on the api placement, 16 November

This is the record of the maintainer's answer on where the items of `hilavitkutin-api` belong, copied verbatim from the workspace's record of that morning (clause-dev `.shared/research/202611150830_stack-path/57-op-calls-on-the-night.md`, section 3) so the rows that carry it can cite a file in this repository. The two blind seats the question rests on are clause-dev `.shared/research/202611150830_stack-path/50a-api-placement-seat.md` and `50b-api-placement-seat.md`, compared in `54-the-api-placement-round-compared.md` of the same directory.

## The question and his answer

Asked: two blind seats agreed on all four of these, and nothing is locked. Which do you accept? Pick every one that stands.

Options shown (several could be chosen):

- WorkUnit stays in the api. WorkUnit and the unit vocabulary stay in hilavitkutin-api, as the ratified rows have it. Domain crates are opt-in depends above it.
- Capability, sink, codec leave. Capability, sink and codec leave hilavitkutin, since Push and BulkPush there duplicate notko's sink. This supersedes one sentence of a provisional row.
- Engine-only items to the engine. dispatch_codegen, PoolFrame, WakeStrategy, Executor, ExecutorError, HybridExecutor and ceiling_div move into the engine, and adaptation becomes a candidate opt-in crate.
- Persistence builds on str and api. "Builds on str and api" is read as a dependency: MemoryProviderApi stays in the api, and the held agent-written persistence row is superseded.

His answer: all four chosen.

## The intent

The placement the two seats agreed on stands in full. The one sentence of the provisional row `the_api_crate_carries_the_consumer_facing_types_and_traits_and_the_platform_contracts` that puts capability, sink and codec in the api is superseded, and so is the held, agent-written row `the_persistence_crate_is_the_generic_hot_cold_storage_bridge_as_a_standalone_ecosystem_crate`, which calls persistence standalone against his ruling that it builds on the str and api crates. Adaptation is a candidate for an opt-in crate, named and not built by this answer.

Two readings are the round's and not his words, and are marked so. First, `CeilingDiv` has no reader anywhere in the tree, so it is deleted rather than moved: moving it would carry a dead item into the engine, and the placement's intent, that the api holds no engine-only item, is met either way. Second, the count-reporting residue of the capability module (`Len`, `Capacity`, `BoundedPush`) is not rebuilt in hilavitkutin: the seats put it with the crate that owns the count type, by notko's or arvo's own round, and nothing in hilavitkutin reads it.

## Correction, 19 November

The first reading above, that `CeilingDiv` is deleted rather than moved, was the round's and went into a ratified row as though it were his. The first review of the round (`REVIEW-1.md` here) named it, and with it the stamped row that still offers `CeilingDiv` as a recommended heuristic. Round `202611191514` follows his words instead: `CeilingDiv` and `Ceil` move into the engine, declared in `hilavitkutin::plan`, and the ratified row says so. The second reading stands.
