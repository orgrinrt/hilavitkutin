# The Context Crate As A Dependency Of The Api

This read carries the maintainer's answer of 15 November on where `hilavitkutin-ctx` sits through the canon flow. It finds every tenet and every configured surface that the answer makes wrong, and writes out the correction for each one as a proposal. No row, design or config line was changed for it. Each proposed row change goes to the maintainer as a question ticket before anything is written into the registry, and the ticket ids are at the end.

It rests on the answer as recorded in clause-dev `.shared/research/202611150830_stack-path/42-op-settles-the-open-calls.md:36-46` and `:112`, captured verbatim in clause-dev `.data/op-responses/202611152021_op-settles-the-open-calls-of-the-roadmap-and-orders-the.md:152`. It also rests on the registry on `origin/dev` (`3177527a`), read with `cargo mock canon find`, `canon check` and `already-said`. The `Cargo.toml` of each crate is a claim about the code. mockspace's own source was read to see how the layer list is used.

## The answer

The question was agenda ticket `is_ctx_substrate_every_unit_author_depends_on_or_an_opt_in_e`. The api crate depends on `hilavitkutin-ctx` today, while the held tenet `the_ecosystem_crates_are_standalone_and_consumers_opt_into_them` says the engine does not depend on it. The options were [Substrate, rows corrected: ctx sits below the api like notko-hlist; the ecosystem row and the layer list are corrected | Substrate, and ctx grows: as above, and ctx gains a private field and constructor or a `Deref` impl | Ecosystem, api drops it: ctx stays opt-in; the api stops depending on it and inlines its nine accessor declarations]. His words:

> It actually is yeah so it should be dep of api, good catch

The intent: the context crate is a dependency of the api, so the held tenet and the layer list in `mockspace.toml` are wrong and get corrected. Whether the crate also grows he did not say; the H-ctx sketch decides what the context type has to hold. `cargo mock already-said "dep of api"` finds that no row of the registry carries his sentence (0 of 1016), so the ruling itself is not yet a row.

## What the code holds

As a claim about the code, the dependency he confirmed is already there. `mock/crates/hilavitkutin-api/Cargo.toml:13` names `hilavitkutin-ctx` under `[dependencies]`, and `hilavitkutin-ctx` depends on no hilavitkutin crate but `hilavitkutin-build` as a build dependency. The engine reaches ctx through the api. The `forbidden-imports` rule for `hilavitkutin-ctx` (`mockspace.toml:196`) forbids `hilavitkutin_api::*` in ctx, which is the direction the answer keeps: ctx sits below and never reaches up. That rule stays as it is. Its reason, "a standalone ecosystem crate; keep it independently usable", reads as still true of a crate that depends on nothing above it, and its word "ecosystem" goes with the rows below.

Two edges that looked like contradictions while reading are dev dependencies only and are not findings: `hilavitkutin-providers/Cargo.toml:19` takes the engine under `[dev-dependencies]`, and `hilavitkutin-persistence/Cargo.toml:24` takes the providers crate the same way.

## The tenets the answer makes wrong

The answer forbids two things: that the context crate is an opt-in crate a unit author may leave out, and that the engine does not depend on it. Searching by those two concepts (`canon find hilavitkutin-ctx`, `find ecosystem`, `find standalone`, `find opt-in`) gives two tenets that assert one of them, and one that agrees with the answer.

`the_ecosystem_crates_are_standalone_and_consumers_opt_into_them` (`mock/registry/tenet/identity-2.toml:283`, held) says "The context, persistence, identity core, string, kit and providers crates are standalone ecosystem crates that consumers opt into. The context, persistence, identity core and string crates are pure ecosystem crates that the engine does not depend on." Both forbidden claims are in it. `cargo mock canon check` reports it as held, binding nothing until he backs it. The ticket `correct_the_held_ecosystem_row_and_the_layer_list_the_contex` says it is superseded, not edited.

`the_context_framework_is_a_standalone_ecosystem_crate_gated_by_providers` (`identity-2.toml:248`, provisional, stamped) says "The context framework is a standalone ecosystem crate, and what a context gives access to is gated by providers", from `mock/DESIGN.md.tmpl:18`. Its word "ecosystem" places ctx among the opt-in crates, which the answer forbids. "Standalone" and "gated by providers" are untouched by the answer. This one binds, so it is the one with weight.

`the_kit_crate_is_separate_so_the_kit_shape_lives_below_the_engine_and_above_the_api_and_ctx_layer_and_recurring_kit_helpers_ship_in_it` (provisional, stamped) already speaks of "the api and ctx layer" and agrees with the answer. No change.

Holds for: the registry on `3177527a`, the tenet namespace only. Question entries were searched as well and none asserts either forbidden claim.

## The proposed rows

Three rows, each held back until he answers its question ticket.

A new row records the ruling, ratified from his words, in the shape of the four rulings `252c5190` added for call 3 of file 57:

```toml
[[tenet]]
id = "op_ruled_that_the_context_crate_is_a_dependency_of_the_api_crate_below_it_as_notko_s_list_crate_is"
subject = "identity"
what = "Op ruled on 2026-11-15 that the context crate is a dependency of the api crate and sits below it, as notko's list crate does, so every unit author has it. The held ecosystem row and the layer list are corrected. Whether the context crate grows is left to the context sketch."
mechanism = "`hilavitkutin-api` depends on `hilavitkutin-ctx`; `hilavitkutin-ctx` depends on no hilavitkutin crate above it. Asked [substrate, rows corrected | substrate, and ctx grows | ecosystem, api drops it]."
source = ".shared/research/202611150830_stack-path/42-op-settles-the-open-calls.md:36-46"
quote = '''
It actually is yeah so it should be dep of api, good catch
'''
origin = "stated"
standing = "ratified"
```

The held row is superseded whole, as the ticket says, by a successor that keeps what the answer leaves standing:

```toml
[[tenet]]
id = "the_persistence_identity_string_kit_and_providers_crates_are_opt_in_and_the_context_crate_sits_below_the_api"
subject = "identity"
what = "The persistence, identity core, string, kit and providers crates are crates a consumer opts into. The context crate is not among them: it sits below the api crate, which depends on it. The engine depends on the persistence, identity core and string crates not at all, and on the context crate only through the api."
superseded = "the_ecosystem_crates_are_standalone_and_consumers_opt_into_them"
```

with `superseded_by` on the old row naming it. Whether the successor is written held, as `252c5190` did for its successor, or stamped provisional is the question of the first ticket.

The provisional ctx row is superseded by one with the word "ecosystem" taken out and the place named:

```toml
[[tenet]]
id = "the_context_framework_is_a_standalone_crate_below_the_api_and_what_a_context_gives_access_to_is_gated_by_providers"
subject = "identity"
what = "The context framework is a standalone crate below the api crate, which depends on it, and what a context gives access to is gated by providers."
```

The alternative put to him is to leave the provisional row as it stands, reading "ecosystem" as the crate's family name and not as opt-in.

## The layer list

The list at `mockspace.toml:23-34` is wrong in more than ctx. It reads as a label per crate (`"API", # hilavitkutin-api`, `"Engine"`, `"Build"`, `"Ext:ctx"` and so on), but mockspace uses it as a label per dependency depth: `compute_crate_layers` in mockspace's `src/render_design.rs:449-485` computes each crate's depth from its dependencies and prints `cfg.layer_label(d)` for depth `d`. So the rendered `docs/STRUCTURE.md:19-56` puts `build` under "Layer 0: API", `ctx`, `linking` and `sym` under "Layer 1: Engine", `api`, `extensions` and `str` under "Layer 2: Build", `api-macros`, `extensions-macros` and `kit` under "Layer 3: Ext:ctx", `providers` under "Ext:persist", `persistence` under "Ext:sym", `conformance` under "Ext:str" and `frozen-authors` under "Ext:kit". Every label but none of the depths is wrong, and the context crate already renders below the api, which is where the answer puts it.

The correction is labels for the depths as they are:

```toml
layers = [
    "Build",            # hilavitkutin-build
    "Foundations",      # hilavitkutin-ctx, -linking, -sym
    "Contracts",        # hilavitkutin-api, -extensions, -str
    "Companions",       # hilavitkutin-api-macros, -extensions-macros, -kit
    "Providers",        # hilavitkutin-providers
    "Persistence",      # hilavitkutin-persistence
    "Conformance",      # hilavitkutin-conformance
    "Frozen authors",   # hilavitkutin-frozen-authors
]
```

The engine crate is left out of the render as the project's own crate, so no label names it. A label keyed by depth drifts every time a dependency edge moves a crate, which is how this list came to be wrong; that is an idea for mockspace and is filed as a ticket there, not decided here. `mockspace.toml` is a step for the coordinator, so the list is handed to it and not edited on this branch.

## The design and agent text that say the same

These state the forbidden claims in design and generated agent text, and change in the round that follows the rows, under the mutation order: `mock/DESIGN.md.tmpl:18` ("Provider-gated context framework (ecosystem, standalone)"), `mock/DESIGN.md.tmpl:32` ("ctx, persistence, sym, str are pure ecosystem (the engine does not depend on them). ctx, sym and str are standalone"), and `mock/agent/MAIN.md.tmpl:26`, which renders `.claude/CLAUDE.md` with ctx under "Ecosystem (opt-in sub-libraries)". The agent rule that ctx and str "do not depend on each other and do not depend on the engine" stays true and needs no change.

Holds for: `origin/dev` `3177527a`, the files named. A grep for `ecosystem` over `mock/crates/*/DESIGN.md.tmpl` found no other sentence putting ctx among the opt-in crates.

## What is put to the maintainer

Each proposed row is a question ticket with the row's path, filed on kaski's `dev` by `ask`. Their ids are in the job's report and on the channel.

1. The new ruling row and the successor of the held row, in `mock/registry/tenet/identity-3.toml`, superseding `identity-2.toml:283`.
2. The provisional ctx row at `identity-2.toml:248`: superseded as above, or left as it stands.
