# The record, file by file

This is the ledger of the read. It has one line for every file of
hilavitkutin's record: how long the file is, the date it carries, the
standing it gives itself in its own words, a few words on what it holds, and
who read it. The registry under `mock/registry/` and `ANSWERS.md` beside this
file rest on the files listed as read here and on nothing else.

The record is every authored document of the repository that says what
hilavitkutin is or is meant to be: the design rounds, the research notes and
their sketches' findings, the design and readme templates of the workspace
and of every crate, the bench findings, and the research imported from the
projects hilavitkutin grew out of. Source files are not part of it. A file
that is a byte for byte copy of another is listed once as a duplicate, and
the rendered files under `docs/` are listed as generated.

A file is read when every line of it was read in order. The readers were
agents who quoted by copying lines out of the file with a command, and every
quote they returned was compared with the file again by a script before
their batch counted. One file of each batch was checked first hand against
its ledger line. The files marked first hand were read whole without a
reader. The date is the one the file states; where a file states none the
ledger says so and does not take the stamp in the file name for it, since
several stamps in this repository are not calendar dates. The standing is
quoted where the file gives itself one and described where it does not.

The line counts in the tables were compared with the files on the branch
when the ledger was assembled.
A date a reader had taken from a file name was struck by a script that looks
for the date in the file, so every date in the tables is one the file
carries.

The branch `origin/recover/unlanded-rounds-and-benches` was read at its ref
and not merged. Most of its documents are on the working branch already,
byte for byte, and are ledgered once, as read here. The documents that only
it carries have their own table below. Four more documents differ between
the two branches (the findings of the `engine_vs_std` bench, the design and
readme templates of `hilavitkutin-sym`, and `mock/research/README.md`); the
recovery side of each was read by the same reader, and the table lists the
file once, by its form on the working branch. The recovery side's changes to
the engine and api design templates were read first hand as a diff, and the
passages that say something about the design are copied in `RECOVERED.md`.

## Read

| file | lines | date | standing, by its own words | holds | read by |
|---|---|---|---|---|---|
| `MIRI.md` | 46 | undated | unstated (document on running MIRI) | MIRI invocation, scope and limits | batch 45 |
| `README.md` | 211 | undated | unstated (readme); Status section: "Design is mature across the engine, the plugin-host layer, and the standalone ecosystem crates" | repo readme: builder, plan, ctx, extensions, build, status | batch 45 |
| `mock/DESIGN.md.tmpl` | 160 | undated | "Pre-implementation; runtime megaround design locked." | workspace architecture overview, runtime layer summary | batch 12 |
| `mock/PRINCIPLES.md.tmpl` | 287 | no date | states none; authored template rendered to docs/PRINCIPLES.md | the engine's principles and bans with reasons | first hand |
| `mock/WORKFLOW.md.tmpl` | 48 | undated | "pre-1.0 design phase" (text); file unstated (workflow template) | consumer usage, WorkUnit authoring, stability | batch 45 |
| `mock/agent/MAIN.md.tmpl` | 163 | undated | unstated (agent instructions template); names spec as "The canonical spec" | identity, crates, architecture rules, vocabulary, what not to do | batch 45 |
| `mock/agent/POSTAMBLE.md.tmpl` | 1 | undated | unstated (agent postamble template) | one-line WorkUnit and vocabulary check | batch 45 |
| `mock/agent/PREAMBLE.md.tmpl` | 1 | undated | unstated (agent preamble template) | one-line static-composition reminder | batch 45 |
| `mock/agent/hooks/no-alloc-guard.sh.tmpl` | 55 | undated | unstated (agent hook template) | hook blocking alloc types in sources | batch 45 |
| `mock/agent/hooks/no-bare-primitive-guard.sh.tmpl` | 79 | undated | unstated (agent hook template) | hook blocking bare primitives in sources | batch 45 |
| `mock/agent/hooks/no-dyn-guard.sh.tmpl` | 36 | undated | unstated (agent hook template) | hook blocking dyn, TypeId, std::any | batch 45 |
| `mock/agent/hooks/no-runtime-registration-guard.sh.tmpl` | 40 | undated | unstated (agent hook template) | hook blocking inventory, ctor, linkme | batch 45 |
| `mock/agent/hooks/no-runtime-spawn-guard.sh.tmpl` | 34 | undated | unstated (agent hook template) | hook blocking runtime thread spawn | batch 45 |
| `mock/agent/hooks/no-std-guard.sh.tmpl` | 35 | undated | unstated (agent hook template) | hook blocking use std | batch 45 |
| `mock/agent/hooks/vocabulary-reminder.sh.tmpl` | 38 | undated | unstated (agent hook template); "ADVISORY severity" | advisory hook on dead vocabulary terms | batch 45 |
| `mock/agent/rules/cargo.md.tmpl` | 190 | undated | unstated (agent rule template) | cargo workspace rules, crate structure, dependency direction | batch 45 |
| `mock/agent/rules/cookbook.md.tmpl` | 116 | undated | unstated (agent rule template) | intent to trait/type pattern book | batch 45 |
| `mock/agent/rules/implementation.md.tmpl` | 112 | undated | unstated (agent rule template) | implementation rules, static composition, morsels | batch 45 |
| `mock/agent/rules/no-heap.md.tmpl` | 50 | undated | unstated (agent rule template) | no-heap rule, red flags, decision test | batch 45 |
| `mock/agent/rules/type-surface.md.tmpl` | 133 | undated | unstated (agent rule template) | trait-first, arvo-types-only rule | batch 45 |
| `mock/agent/rules/writing-style.md.tmpl` | 140 | undated | unstated (agent rule template); "synced from clause-dev/.shared/writing-style-fragment.md: do not edit in place" | shared writing-style rules | batch 45 |
| `mock/agent/skills/commit-style/SKILL.md.tmpl` | 92 | undated | unstated (skill template) | commit message format and byline policy | batch 45 |
| `mock/agent/skills/lint-rules/SKILL.md.tmpl` | 109 | undated | unstated (skill template) | lint sources, severities, custom lint table | batch 45 |
| `mock/agent/skills/type-surface-review/SKILL.md.tmpl` | 80 | undated | unstated (skill template) | pub signature self-review checklist | batch 45 |
| `mock/catalogue.md` | 21 | undated | unstated (catalogue of ignored tests) | rows of ignored catalogue tests with gaps | batch 45 |
| `mock/crates/hilavitkutin-api/BACKLOG.md.tmpl` | 308 | undated | unstated (backlog) | api crate deferred items and bench follow-ups | batch 45 |
| `mock/crates/hilavitkutin-api/DESIGN.md.tmpl` | 1377 | undated | unstated, design template | consumer-facing types, traits, platform contracts | batch 12 |
| `mock/crates/hilavitkutin-api/README.md.tmpl` | 6 | undated | unstated (crate readme template) | api crate one-paragraph description | batch 45 |
| `mock/crates/hilavitkutin-api/SHAME.md.tmpl` | 30 | undated | unstated (shame file); "Known gaps with explanations." | KitDispatch placement gap | batch 45 |
| `mock/crates/hilavitkutin-build/BACKLOG.md.tmpl` | 69 | undated | unstated (backlog) | build crate deferred items | batch 45 |
| `mock/crates/hilavitkutin-build/DESIGN.md.tmpl` | 312 | undated | unstated, design template | build-time pragmas, LLVM flags, PGO and BOLT | batch 12 |
| `mock/crates/hilavitkutin-build/README.md.tmpl` | 5 | undated | unstated (crate readme template) | build crate description | batch 45 |
| `mock/crates/hilavitkutin-ctx/BACKLOG.md.tmpl` | 37 | undated | unstated (backlog) | ctx crate deferred items | batch 45 |
| `mock/crates/hilavitkutin-ctx/DESIGN.md.tmpl` | 300 | undated | unstated, design template | provider-gated context framework, macros | batch 12 |
| `mock/crates/hilavitkutin-ctx/README.md.tmpl` | 5 | undated | unstated (crate readme template) | ctx crate description | batch 45 |
| `mock/crates/hilavitkutin-extensions-macros/BACKLOG.md.tmpl` | 39 | undated | "Designed, deferred" | extensions-macros deferred enrichments | batch 45 |
| `mock/crates/hilavitkutin-extensions-macros/DESIGN.md.tmpl` | 77 | undated | "The emitted shape is locked at v1." | proc-macro emitting extension descriptors | batch 12 |
| `mock/crates/hilavitkutin-extensions-macros/README.md.tmpl` | 14 | undated | unstated (crate readme template) | extensions-macros crate description | batch 45 |
| `mock/crates/hilavitkutin-extensions/BACKLOG.md.tmpl` | 43 | undated | "Designed, deferred" | extensions crate deferred items | batch 45 |
| `mock/crates/hilavitkutin-extensions/DESIGN.md.tmpl` | 191 | undated | "must preserve the v1 contract unless explicit deprecation is scheduled" | extension host, descriptor, lifecycle, policy | batch 12 |
| `mock/crates/hilavitkutin-extensions/README.md.tmpl` | 11 | undated | unstated (crate readme template) | extensions crate description | batch 45 |
| `mock/crates/hilavitkutin-kit/BACKLOG.md.tmpl` | 55 | undated | unstated (backlog) | kit crate deferred items | batch 45 |
| `mock/crates/hilavitkutin-kit/DESIGN.md.tmpl` | 262 | undated | unstated, design template | declarative Kit trait, composition, Replaceable | batch 12 |
| `mock/crates/hilavitkutin-kit/README.md.tmpl` | 9 | undated | unstated (crate readme template) | kit crate description | batch 45 |
| `mock/crates/hilavitkutin-linking/BACKLOG.md.tmpl` | 33 | undated | "Designed, deferred" | linking crate deferred items | batch 45 |
| `mock/crates/hilavitkutin-linking/DESIGN.md.tmpl` | 117 | undated | "must preserve the v1 contract unless explicit deprecation is scheduled" | dynamic library loading primitive | batch 12 |
| `mock/crates/hilavitkutin-linking/README.md.tmpl` | 12 | undated | unstated (crate readme template) | linking crate description | batch 45 |
| `mock/crates/hilavitkutin-persistence/BACKLOG.md.tmpl` | 88 | undated | unstated (backlog) | persistence deferred items, bench findings | batch 45 |
| `mock/crates/hilavitkutin-persistence/DESIGN.md.tmpl` | 229 | undated | unstated, design template | hot-cold storage bridge, rkyv, eviction | batch 12 |
| `mock/crates/hilavitkutin-persistence/README.md.tmpl` | 7 | undated | unstated (crate readme template) | persistence crate description | batch 45 |
| `mock/crates/hilavitkutin-providers/BACKLOG.md.tmpl` | 111 | undated | "Designed-but-deferred surfaces." | providers deferred items, bench findings | batch 45 |
| `mock/crates/hilavitkutin-providers/DESIGN.md.tmpl` | 203 | undated | unstated, design template | default interner and ColumnStorage providers | batch 12 |
| `mock/crates/hilavitkutin-providers/README.md.tmpl` | 8 | undated | unstated (crate readme template) | providers crate description | batch 45 |
| `mock/crates/hilavitkutin-providers/SHAME.md.tmpl` | 37 | undated | unstated (shame file); "Known gaps" | forward-documented AdaptWu, metrics, kits | batch 45 |
| `mock/crates/hilavitkutin-str/BACKLOG.md.tmpl` | 104 | undated | unstated (backlog) | str crate deferred items, bench findings | batch 45 |
| `mock/crates/hilavitkutin-str/DESIGN.md.tmpl` | 236 | undated | unstated, design template | interned string handle, linker section | batch 12 |
| `mock/crates/hilavitkutin-str/README.md.tmpl` | 8 | undated | unstated (crate readme template) | str crate description | batch 45 |
| `mock/crates/hilavitkutin-sym/DESIGN.md.tmpl` | 157 | undated | unstated, design template | generic interned-identity handle core | batch 12 |
| `mock/crates/hilavitkutin-sym/README.md.tmpl` | 9 | undated | unstated (crate readme template) | sym crate description | batch 45 |
| `mock/crates/hilavitkutin/BACKLOG.md.tmpl` | 539 | undated | unstated (backlog) | engine deferred items, follow-ups, GATE-2 grouping | batch 45 |
| `mock/crates/hilavitkutin/DESIGN.md.tmpl` | 1442 | undated | "This document maps domains to engine modules." | engine design, plan, dispatch, scheduler builder | batch 12 |
| `mock/crates/hilavitkutin/README.md.tmpl` | 5 | undated | unstated (crate readme template) | engine crate description | batch 45 |
| `mock/design_rounds/202603231200-abandoned/202603231200_changelist.doc.deprecated.md` | 157 | 2026-03-23 | "Status: IN PROGRESS — being written crate by crate"; abandoned (directory name) | doc changelist, 9 crates arvo and hilavitkutin | batch 14 |
| `mock/design_rounds/202603241500-abandoned/202603241500_changelist.doc.deprecated.md` | 165 | 2026-03-24 | "Status: IN PROGRESS — being written crate by crate"; abandoned (directory name) | revised doc changelist, 10 crates, strategy markers | batch 14 |
| `mock/design_rounds/202604200054-abandoned/202604200054_changelist.doc.deprecated.md` | 193 | 2026-04-20 | abandoned (directory name) | doc changelist, hilavitkutin-api contracts, first attempt | batch 14 |
| `mock/design_rounds/202604200054-abandoned/202604200054_changelist.src.deprecated.md` | 356 | 2026-04-20 | abandoned (directory name) | src changelist, hilavitkutin-api contracts, Context struct | batch 14 |
| `mock/design_rounds/202604200055/202603111529_topic.paradigm-and-architecture.md` | 1414 | 2026-03-11 | "**Status:** IN PROGRESS — being built interactively, point by point" | paradigm of polka-dots: plugins, columns, WorkUnit, executor, principles | batch 01 |
| `mock/design_rounds/202604200055/202603121310_topic.memory-layout-and-platform.md` | 984 | 2026-03-12 | "**Status:** COMPLETE — continued in 202603121800_topic.dispatch-and-optimisation.md" | morsel sizing, external progress, chains, ColumnStorage | batch 01 |
| `mock/design_rounds/202604200055/202603121800_topic.dispatch-and-optimisation.md` | 4155 | 2026-03-12 | "**Status:** COMPLETE — all 11 questions resolved (Q1–Q11), 110 decisions" | dispatch, tiling, optimisation decisions Q1-Q11, decisions log 30-110 | batch 02 |
| `mock/design_rounds/202604200055/202603131230_topic.build-configuration-and-cfg-gating.md` | 2452 | 2026-03-13 | "**Status:** COMPLETE — all questions resolved (Q1-Q8)" | build pipeline, cfg-gating, pragmas, hilavitkutin crate scope | batch 03 |
| `mock/design_rounds/202604200055/202603131400_research.llvm-plugin-passes.md` | 844 | 2026-03-13 | "**Type:** Research (not a design round topic)" | LLVM plugin passes, Polly, custom pass authoring | batch 03 |
| `mock/design_rounds/202604200055/202603131400_research.rustc-llvm-optimization-flags.md` | 1003 | 2026-03-13 | "**Status:** RESEARCH (reference material, not a design round topic)" | rustc/LLVM optimisation flag catalogue | batch 03 |
| `mock/design_rounds/202604200055/202603131500_research.per-crate-backends-and-domain-optimizers.md` | 1199 | 2026-03-13 | "**Type:** Research (reference material, not a design round topic)" | per-crate backends, MLIR, domain optimizers | batch 03 |
| `mock/design_rounds/202604200055/202603131600_research.polly-llvm-rust-ir.md` | 537 | 2026-03-13 | "**Type:** Research (not a design round topic)" | Polly and Rust LLVM IR research | batch 04 |
| `mock/design_rounds/202604200055/202603131700_research.llvm-passes-beyond-polly.md` | 1187 | 2026-03-13 | "**Status:** RESEARCH (reference material, not a design round topic)" | LLVM passes survey for data engines | batch 04 |
| `mock/design_rounds/202604200055/202603141800_research.columnar-storage-ecs.md` | 933 | 2026-03-15 | "**Type:** Research (academic + industry survey)" | columnar storage, ECS, unified store survey | batch 04 |
| `mock/design_rounds/202604200055/202603141800_research.dag-scheduling-work-stealing.md` | 1066 | 2026-03-15 | "**Type:** Research (academic + industry survey)" | DAG scheduling, work stealing, morsel dispatch | batch 04 |
| `mock/design_rounds/202604200055/202603141800_research.fault-tolerance-backpressure.md` | 816 | 2026-03-15 | "**Type:** Research (academic + industry survey)" | retry, backpressure, fault handling, rollback | batch 04 |
| `mock/design_rounds/202604200055/202603141800_research.io-integration-pipelines.md` | 762 | 2026-03-15 | "**Type:** Research (academic + industry survey)" | I/O integration, yield and waker design | batch 04 |
| `mock/design_rounds/202604200055/202603141800_research.loimu-dag.md` | 1063 | 2026-03-15 | "Type: Research (reference material for hilavitkutin design)" | loimu DAG, plan, dirty tracking, aliasing, hilavitkutin relevance | batch 05 |
| `mock/design_rounds/202604200055/202603141800_research.loimu-incremental.md` | 781 | 2026-03-15 | "Type: Research (reference material for hilavitkutin design)" | loimu dirty tracking, version stamps, aliasing, subgraphs, relevance | batch 05 |
| `mock/design_rounds/202604200055/202603141800_research.loimu-macros.md` | 762 | 2026-03-15 | "Type: Research (reference material for hilavitkutin design)" | loimu define_* macros, access modes, persistence, relevance | batch 05 |
| `mock/design_rounds/202604200055/202603141800_research.loimu-primitives.md` | 846 | 2026-03-15 | "Type: Research (reference material for hilavitkutin design)" | loimu fixed-point primitives, ids, storage packing, arvo scope | batch 05 |
| `mock/design_rounds/202604200055/202603141800_research.loimu-scheduler.md` | 1080 | 2026-03-15 | "Type: Research (reference material for hilavitkutin design)" | loimu scheduler, lanes, version stamps, yielding, adoption map | batch 05 |
| `mock/design_rounds/202604200055/202603141800_research.loimu-signals.md` | 642 | 2026-03-15 | "Type: Research (reference material for hilavitkutin design)" | loimu signal bus, double buffering, virtual columns comparison | batch 05 |
| `mock/design_rounds/202604200055/202603141800_research.loimu-storage.md` | 1157 | 2026-03-15 | "**Type:** Research (reference material for hilavitkutin design)" | loimu storage architecture research, relevance to hilavitkutin | batch 06 |
| `mock/design_rounds/202604200055/202603141800_research.loimu-synthesis.md` | 420 | 2026-03-15 | "**Type:** Research (synthesis of Phase 1 findings)" | what loimu's design gives hilavitkutin and what stays in loimu | first hand |
| `mock/design_rounds/202604200055/202603141800_research.loisto-design-rounds.md` | 499 | 2026-03-15 | "**Type:** Research (reference from sister project)" | loisto design rounds analysis, transferable process points | batch 06 |
| `mock/design_rounds/202604200055/202603141800_research.loisto-scheduler.md` | 466 | 2026-03-15 | "**Type:** Research (reference from sister project)" | loisto scheduler, BPF, pacing; policy relevance | batch 06 |
| `mock/design_rounds/202604200055/202603141800_research.morsel-pipeline-execution.md` | 988 | 2026-03-15 | "**Type:** Research (academic + industry survey)" | morsel and pipeline execution survey, yield model verdict | batch 06 |
| `mock/design_rounds/202604200055/202603141800_research.registration-discovery.md` | 736 | 2026-03-15 | "**Type:** Research (Rust ecosystem + industry survey)" | registration and discovery patterns, Option B recommendation | batch 06 |
| `mock/design_rounds/202604200055/202603141800_research.scheduler-algorithm-deep-dive.md` | 1268 | 2026-03-15 | "**Type:** Research (deep dive, second pass)" | scheduler plan, execution, morsel, yield algorithms | batch 06 |
| `mock/design_rounds/202604200055/202603141800_research.task-plan.md` | 800 | 2026-03-14 | unstated (research task plan; title "# Research Task Plan: hilavitkutin Core Design") | plan of 16 research documents plus topic update | batch 07 |
| `mock/design_rounds/202604200055/202603141800_research.unified-store-deep-dive.md` | 1249 | 2026-03-15 | "**Type:** Research (deep dive, second pass)" | Store trait, cardinality, version stamps, context, DAG edges | batch 07 |
| `mock/design_rounds/202604200055/202603141800_research.workunit-trait-deep-dive.md` | 1542 | 2026-03-15 | "**Type:** Research (deep dive, second pass)" | WorkUnit trait, AccessSet, Context, lifecycle, Progress | batch 07 |
| `mock/design_rounds/202604200055/202603141800_research.yielding-deep-dive.md` | 1666 | 2026-03-15 | "**Type:** Research (deep dive, second pass)" | yielding, wakers, partial writes, yield versus split, API sketch | batch 07 |
| `mock/design_rounds/202604200055/202603141800_topic.hilavitkutin-core-design.md` | 1230 | 2026-03-14 | "COMPLETE — all questions resolved (Q1-Q8)" | core design Q1-Q12 decisions, research table | batch 08 |
| `mock/design_rounds/202604200055/202603151200_topic.hilavitkutin-api-surface.md` | 3928 | 2026-03-15 | "IN PROGRESS — being built interactively" | API surface Q1-Q12, benchmarks, execution theory | batch 08 |
| `mock/design_rounds/202604200055/202603171900_topic.execution-model.md` | 516 | 2026-03-17 | "**Status:** IN PROGRESS" | execution rules distilled from the bench findings | first hand |
| `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md` | 2480 | 2026-03-18 | "**Status:** IN PROGRESS — being built interactively" | founding consolidation spec: 23 domains, nine resolutions | first hand |
| `mock/design_rounds/202604200055/202604200055_changelist.doc.lock.md` | 118 | 2026-04-20 | locked doc changelist, "**Phase:** DOC" | ctx parameterised providers, the doc changes | first hand |
| `mock/design_rounds/202604200055/202604200055_changelist.src.lock.md` | 162 | 2026-04-20 | locked src changelist, "**Phase:** SRC" | ctx parameterised provider macros and tests | first hand |
| `mock/design_rounds/202604200117/202604200117_changelist.doc.lock.md` | 156 | 2026-04-20 | unstated; changelist (.lock in file name) | doc changelist, api contracts on ctx provider tuples | batch 14 |
| `mock/design_rounds/202604200117/202604200117_changelist.src.lock.md` | 283 | 2026-04-20 | unstated; changelist (.lock in file name) | src changelist, api contracts v2 deliverables | batch 14 |
| `mock/design_rounds/202604200126/202604200126_changelist.doc.lock.md` | 61 | 2026-04-20 | unstated; changelist (.lock in file name) | doc changelist, hilavitkutin-str impl | batch 14 |
| `mock/design_rounds/202604200126/202604200126_changelist.src.lock.md` | 255 | 2026-04-20 | unstated; changelist (.lock in file name) | src changelist, str handle, interner, macro | batch 14 |
| `mock/design_rounds/202604200141/202604200141_changelist.doc.lock.md` | 118 | 2026-04-20 | unstated; changelist (.lock in file name) | doc changelist, persistence skeleton | batch 14 |
| `mock/design_rounds/202604200141/202604200141_changelist.src.lock.md` | 292 | 2026-04-20 | unstated; changelist (.lock in file name) | src changelist, persistence skeleton deliverables | batch 14 |
| `mock/design_rounds/202604200153/202604200153_changelist.doc.lock.md` | 104 | 2026-04-20 | unstated; changelist (.lock in file name) | doc changelist, hilavitkutin-build skeleton | batch 14 |
| `mock/design_rounds/202604200153/202604200153_changelist.src.lock.md` | 281 | 2026-04-20 | unstated; changelist (.lock in file name) | src changelist, build pragmas, profiles, axes | batch 14 |
| `mock/design_rounds/202604200210/202604200210_changelist.doc.lock.md` | 96 | 2026-04-20 | unstated; changelist (.lock in file name) | doc changelist, engine 5a1 platform tiers | batch 14 |
| `mock/design_rounds/202604200210/202604200210_changelist.src.lock.md` | 124 | 2026-04-20 | unstated; changelist (.lock in file name) | src changelist, engine platform os and std | batch 14 |
| `mock/design_rounds/202604200217/202604200217_changelist.doc.lock.md` | 64 | 2026-04-20 | unstated; changelist (.lock in file name) | doc changelist, engine 5a5 resource caching | batch 14 |
| `mock/design_rounds/202604200217/202604200217_changelist.src.lock.md` | 92 | 2026-04-20 | unstated; changelist (.lock in file name) | src changelist, resource snapshot, accumulator | batch 14 |
| `mock/design_rounds/202604200227/202604200227_changelist.doc.lock.md` | 80 | 2026-04-20 | unstated; changelist (.lock in file name) | doc changelist, engine 5a6 strategy, scheduler | batch 14 |
| `mock/design_rounds/202604200227/202604200227_changelist.src.lock.md` | 161 | 2026-04-20 | unstated; changelist (.lock in file name) | src changelist, strategy and scheduler skeleton | batch 14 |
| `mock/design_rounds/202604200237/202604200237_changelist.doc.lock.md` | 130 | 2026-04-20 | unstated; changelist (.lock in file name) | doc changelist, engine 5a2 plan-stage skeleton | batch 14 |
| `mock/design_rounds/202604200237/202604200237_changelist.src.lock.md` | 291 | 2026-04-20 | unstated; changelist (.lock in file name) | src changelist, plan types, twelve step stubs | batch 14 |
| `mock/design_rounds/202604200251/202604200251_changelist.doc.lock.md` | 138 | 2026-04-20 | unstated; changelist (.lock in file name) | doc changelist, engine 5a3 dispatch, intrinsics | batch 14 |
| `mock/design_rounds/202604200251/202604200251_changelist.src.lock.md` | 375 | 2026-04-20 | unstated; changelist (.lock in file name) | src changelist, dispatch types, intrinsics shim | batch 14 |
| `mock/design_rounds/202604200308/202604200308_changelist.doc.lock.md` | 213 | 2026-04-20 | unstated; changelist (.lock in file name) | doc changelist, engine 5a4 thread, adapt | batch 14 |
| `mock/design_rounds/202604200308/202604200308_changelist.src.lock.md` | 430 | 2026-04-20 | unstated; changelist (.lock in file name) | src changelist, thread pool, adapt types | batch 14 |
| `mock/design_rounds/202604200337/202604200337_changelist.doc.lock.md` | 146 | 2026-04-20 | unstated; changelist (.lock in file name) | doc changelist, engine post-review polish | batch 14 |
| `mock/design_rounds/202604200337/202604200337_changelist.src.lock.md` | 136 | 2026-04-20 | unstated; changelist (.lock in file name) | src changelist, Send/Sync, fence docs | batch 14 |
| `mock/design_rounds/202604200350/202604200350_changelist.doc.lock.md` | 211 | 2026-04-20 | unstated; changelist (.lock in file name) | doc changelist, non-engine post-review polish | batch 14 |
| `mock/design_rounds/202604200350/202604200350_changelist.src.lock.md` | 223 | 2026-04-20 | unstated; changelist (.lock in file name) | src changelist, rename, metadata, resolve Option | batch 14 |
| `mock/design_rounds/202604200445/202604200445_changelist.doc.lock.md` | 95 | 2026-04-20 | unstated; changelist (.lock in file name) | doc changelist, cross-repo polish, NodeId rename | batch 14 |
| `mock/design_rounds/202604200445/202604200445_changelist.src.lock.md` | 92 | 2026-04-20 | "**Phase:** SRC" | src changelist: UnitId rename, arvo deps, mask notes, lint | batch 15 |
| `mock/design_rounds/202604201048/202604201048_changelist.doc.lock.md` | 142 | 2026-04-20 | "**Phase:** DOC" | doc changelist: sink, collector, codec contracts | batch 15 |
| `mock/design_rounds/202604201048/202604201048_changelist.src.lock.md` | 193 | 2026-04-20 | "**Phase:** SRC" | src changelist: capability, sink, codec modules | batch 15 |
| `mock/design_rounds/202604201048/202604201048_topic.sink-collector-codec-contracts.md` | 625 | 2026-04-20 | "**Status:** IN PROGRESS — awaiting user decisions" | topic: sink, collector, codec contract family decisions | batch 15 |
| `mock/design_rounds/202604201319/202604201319_changelist.doc.lock.md` | 161 | 2026-04-20 | "**Phase:** DOC" | doc changelist: persistence primitive-vocabulary sweep | batch 15 |
| `mock/design_rounds/202604201319/202604201319_changelist.src.lock.md` | 350 | 2026-04-20 | "**Phase:** SRC" | src changelist: persistence newtypes, signature migrations | batch 15 |
| `mock/design_rounds/202604201319/202604201319_topic.hilavitkutin-persistence-sweep.md` | 358 | 2026-04-20 | "**Status:** TOPIC phase; decisions recorded, ready to commit." | topic: persistence primitive-vocabulary sweep, provisional decisions | batch 15 |
| `mock/design_rounds/202604240615/202604240615_changelist.doc.lock.md` | 219 | 2026-04-24 | "**Phase:** DOC" | doc changelist: linking rename, extensions host, macros crate | batch 15 |
| `mock/design_rounds/202604240615/202604240615_changelist.src.lock.md` | 96 | 2026-04-24 | "**Phase:** SRC" | src changelist: loader move, host source, macro source | batch 15 |
| `mock/design_rounds/202604240615/202604240615_topic.hilavitkutin-extensions-host.md` | 417 | 2026-04-24 | "**Phase:** TOPIC" | topic: extensions host layer and macro sibling design | batch 15 |
| `mock/design_rounds/202604240615/202604240615_topic.rename-extensions-to-linking.md` | 206 | 2026-04-24 | "**Phase:** TOPIC" | topic: rename extensions to linking, Ecosystem header | batch 15 |
| `mock/design_rounds/202604240745/202604240745_changelist.doc.lock.md` | 126 | 2026-04-24 | "**Phase:** DOC" | doc changelist: first extensions crate, rule scoping, invariant | batch 15 |
| `mock/design_rounds/202604240745/202604240745_changelist.src.lock.md` | 267 | 2026-04-24 | "**Phase:** SRC" | src changelist: first extensions loader crate source | batch 15 |
| `mock/design_rounds/202604240745/202604240745_topic.hilavitkutin-extensions.md` | 193 | 2026-04-24 | "**Phase:** TOPIC" | topic: dynamic library loading primitive crate | batch 15 |
| `mock/design_rounds/202604240831/202604240831_changelist.doc.lock.md` | 43 | 2026-04-24 | "**Phase:** DOC" | doc changelist: DESIGN backtick expansion | batch 15 |
| `mock/design_rounds/202604240831/202604240831_changelist.src.lock.md` | 27 | 2026-04-24 | "**Phase:** SRC" | empty src changelist, doc-only round | batch 15 |
| `mock/design_rounds/202604240831/202604240831_topic.extensions-design-backticks.md` | 29 | 2026-04-24 | "**Phase:** TOPIC" | topic: extensions DESIGN backtick cleanup | batch 15 |
| `mock/design_rounds/202604240840/202604240840_changelist.doc.lock.md` | 63 | 2026-04-24 | "**Phase:** DOC" | doc changelist: extensions completeness doc updates | batch 15 |
| `mock/design_rounds/202604240840/202604240840_changelist.src.lock.md` | 117 | 2026-04-24 | "**Phase:** SRC" | src changelist: StaticRef, close, arities, CI | batch 15 |
| `mock/design_rounds/202604240840/202604240840_topic.extensions-completeness.md` | 37 | 2026-04-24 | "**Phase:** TOPIC" | topic: extensions feature completeness | batch 15 |
| `mock/design_rounds/202604241201/202604241201_changelist.doc.lock.md` | 42 | 2026-04-24 | "**Phase:** DOC" | doc changelist: BACKLOG housekeeping only | batch 15 |
| `mock/design_rounds/202604241201/202604241201_changelist.src.lock.md` | 100 | 2026-04-24 | "**Phase:** SRC" | src changelist: host conformance, macro emission, tests | batch 15 |
| `mock/design_rounds/202604241201/202604241201_topic.extensions-conformance-and-macros-emission.md` | 88 | 2026-04-24 | "**Phase:** TOPIC" | topic: source-versus-DESIGN mismatch ledger, source follows | batch 15 |
| `mock/design_rounds/202604241222/202604241222_changelist.doc.lock.md` | 17 | 2026-04-24 | "**Phase:** DOC" | doc changelist: two prose tweaks for MaybeNull | batch 15 |
| `mock/design_rounds/202604241222/202604241222_changelist.src.lock.md` | 46 | 2026-04-24 | "**Phase:** SRC-PLAN" | src changelist: Option fn to MaybeNull fn | batch 15 |
| `mock/design_rounds/202604241222/202604241222_topic.maybenull-fn-at-descriptor-boundary.md` | 36 | 2026-04-24 | "**Phase:** TOPIC" | topic: MaybeNull at descriptor boundary | batch 15 |
| `mock/design_rounds/202604241317/202604241317_changelist.doc.lock.md` | 34 | 2026-04-24 | "**Phase:** DOC" | doc changelist: severity ramp-up BACKLOG housekeeping | batch 15 |
| `mock/design_rounds/202604241317/202604241317_changelist.src.lock.md` | 75 | 2026-04-24 | "**Phase:** SRC" | src changelist: lint:allow annotations, has_capability fix | batch 15 |
| `mock/design_rounds/202604241317/202604241317_topic.severity-ramp-up.md` | 91 | 2026-04-24 | "**Phase:** TOPIC" | topic: restore ten lints to error severity | batch 15 |
| `mock/design_rounds/202604290000/202604290000_changelist.doc.lock.md` | 48 | 2026-04-21 | "**Phase:** DOC" | doc changelist: hint aliases, Str as Bits<32> | batch 15 |
| `mock/design_rounds/202604290000/202604290000_changelist.src.lock.md` | 174 | 2026-04-21 | "**Phase:** SRC" | src changelist: hint UFixed aliases, Str Bits<32> | batch 15 |
| `mock/design_rounds/202604290000/202604290000_topic.exact-widths-str-and-hints.md` | 161 | 2026-04-21 | "**Status:** TOPIC phase" | topic: exact widths for scheduling hints and Str | batch 15 |
| `mock/design_rounds/202604291932-abandoned/202604291932_changelist.doc.deprecated.md` | 23 | 2026-04-29 | "**Phase:** DOC"; abandoned (directory name) | deprecated doc changelist: arvo Round D alignment | batch 15 |
| `mock/design_rounds/202604291932-abandoned/202604291932_changelist.src.deprecated.md` | 57 | 2026-04-29 | "**Phase:** SRC (deprecated mid-execution)"; abandoned (directory name) | deprecated src changelist: abandoned scope discovery | batch 15 |
| `mock/design_rounds/202604291932-abandoned/202604291932_topic.arvo-round-d-alignment.md` | 62 | 2026-04-29 | "**Phase:** TOPIC"; abandoned (directory name) | topic: arvo Round D alignment adapter round | batch 15 |
| `mock/design_rounds/202604292037/202604292037_changelist.doc.lock.md` | 37 | 2026-04-29 | "**Phase:** DOC" | doc changelist: Round D adoption, no doc edits | batch 15 |
| `mock/design_rounds/202604292037/202604292037_changelist.src.lock.md` | 266 | 2026-04-29 | "**Phase:** SRC" | src changelist: Round D adoption patterns A to G | batch 15 |
| `mock/design_rounds/202604292037/202604292037_topic.hilavitkutin-arvo-round-d-adoption.md` | 94 | 2026-04-29 | "**Phase:** TOPIC" | topic: workspace-wide arvo Round D adoption | batch 15 |
| `mock/design_rounds/202604292117/202604292117_changelist.doc.lock.md` | 25 | 2026-04-29 | "**Phase:** DOC" | doc changelist: review fixes, no doc edits | batch 15 |
| `mock/design_rounds/202604292117/202604292117_changelist.src.lock.md` | 31 | 2026-04-29 | "**Phase:** SRC" | src changelist: three review-finding source edits | batch 15 |
| `mock/design_rounds/202604292117/202604292117_topic.round-d-adoption-review-fixes.md` | 29 | 2026-04-29 | "**Phase:** TOPIC" | topic: Round D adoption review fixes | batch 15 |
| `mock/design_rounds/202604301630/202604301630_changelist.doc.lock.md` | 37 | 2026-04-30 | "**Phase:** DOC" | doc changelist: arvo-refit adoption, no doc edits | batch 15 |
| `mock/design_rounds/202604301630/202604301630_changelist.src.lock.md` | 181 | 2026-04-30 | "**Phase:** SRC-PLAN" | source changelist: arvo-refit adoption, lint allow sweep | batch 16 |
| `mock/design_rounds/202604301630/202604301630_topic.arvo-refit-adoption-and-lint-allow-sweep.md` | 133 | 2026-04-30 | "**Phase:** TOPIC" | topic: arvo-refit adoption, lint allow sweep | batch 16 |
| `mock/design_rounds/202604310500/202604310500_changelist.doc.lock.md` | 17 | 2026-04-21 | "**Phase:** DOC" | doc changelist: Str mask constants forward to StrLayout | batch 16 |
| `mock/design_rounds/202604310500/202604310500_changelist.src.lock.md` | 48 | 2026-04-21 | "**Phase:** SRC" | source changelist: Str mask constants forward to StrLayout | batch 16 |
| `mock/design_rounds/202604310500/202604310500_topic.str-layout-mask-forward.md` | 35 | 2026-04-21 | "**Status:** TOPIC phase" | topic: Str mask constants forward to StrLayout | batch 16 |
| `mock/design_rounds/202604320000/202604320000_changelist.doc.lock.md` | 13 | 2026-04-21 | "**Phase:** DOC" | doc changelist: lint:allow comment format drift cleanup | batch 16 |
| `mock/design_rounds/202604320000/202604320000_changelist.src.lock.md` | 78 | 2026-04-21 | "**Phase:** SRC" | source changelist: lint:allow comment format drift cleanup | batch 16 |
| `mock/design_rounds/202604320000/202604320000_topic.lint-allow-format-drift.md` | 41 | 2026-04-21 | "**Status:** TOPIC phase" | topic: lint:allow comment format drift cleanup | batch 16 |
| `mock/design_rounds/202604320500/202604320500_changelist.doc.lock.md` | 10 | 2026-04-21 | "**Phase:** DOC" | doc changelist: residual lint:allow format fix, str lib.rs | batch 16 |
| `mock/design_rounds/202604320500/202604320500_changelist.src.lock.md` | 18 | 2026-04-21 | "**Phase:** SRC" | source changelist: residual lint:allow format fix, str lib.rs | batch 16 |
| `mock/design_rounds/202604320500/202604320500_topic.lint-allow-format-drift-residual.md` | 28 | 2026-04-21 | "**Status:** TOPIC phase" | topic: residual lint:allow format fix, str lib.rs | batch 16 |
| `mock/design_rounds/202604330000/202604330000_changelist.doc.lock.md` | 33 | 2026-04-21 | "**Phase:** DOC" | doc changelist: api primitive sweep: USize, Bool, Nanos | batch 16 |
| `mock/design_rounds/202604330000/202604330000_changelist.src.lock.md` | 330 | 2026-04-21 | "**Phase:** SRC-PLAN" | source changelist: api primitive sweep: USize, Bool, Nanos | batch 16 |
| `mock/design_rounds/202604330000/202604330000_topic.api-primitive-sweep.md` | 118 | 2026-04-21 | "**Phase:** TOPIC" | topic: api primitive sweep: USize, Bool, Nanos | batch 16 |
| `mock/design_rounds/202604340000/202604340000_changelist.doc.lock.md` | 9 | 2026-04-21 | "**Phase:** DOC" | doc changelist: engine strategy selector USize sweep | batch 16 |
| `mock/design_rounds/202604340000/202604340000_changelist.src.lock.md` | 75 | 2026-04-21 | "**Phase:** SRC-PLAN" | source changelist: engine strategy selector USize sweep | batch 16 |
| `mock/design_rounds/202604340000/202604340000_topic.engine-strategy-sweep.md` | 48 | 2026-04-21 | "**Phase:** TOPIC" | topic: engine strategy selector USize sweep | batch 16 |
| `mock/design_rounds/202604350000/202604350000_changelist.doc.lock.md` | 8 | 2026-04-21 | "**Phase:** DOC" | doc changelist: engine resource module primitive sweep | batch 16 |
| `mock/design_rounds/202604350000/202604350000_changelist.src.lock.md` | 75 | 2026-04-21 | "**Phase:** SRC-PLAN" | source changelist: engine resource module primitive sweep | batch 16 |
| `mock/design_rounds/202604350000/202604350000_topic.engine-resource-sweep.md` | 68 | 2026-04-21 | "**Phase:** TOPIC" | topic: engine resource module primitive sweep | batch 16 |
| `mock/design_rounds/202604360000/202604360000_changelist.doc.lock.md` | 8 | 2026-04-21 | "**Phase:** DOC" | doc changelist: engine adapt module primitive sweep | batch 16 |
| `mock/design_rounds/202604360000/202604360000_changelist.src.lock.md` | 109 | 2026-04-21 | "**Phase:** SRC-PLAN" | source changelist: engine adapt module primitive sweep | batch 16 |
| `mock/design_rounds/202604360000/202604360000_topic.engine-adapt-sweep.md` | 54 | 2026-04-21 | "**Phase:** TOPIC" | topic: engine adapt module primitive sweep | batch 16 |
| `mock/design_rounds/202604370000/202604370000_changelist.doc.lock.md` | 7 | 2026-04-21 | "**Phase:** DOC" | doc changelist: engine platform module primitive sweep | batch 16 |
| `mock/design_rounds/202604370000/202604370000_changelist.src.lock.md` | 68 | 2026-04-21 | "**Phase:** SRC-PLAN" | source changelist: engine platform module primitive sweep | batch 16 |
| `mock/design_rounds/202604370000/202604370000_topic.engine-platform-sweep.md` | 78 | 2026-04-21 | "**Phase:** TOPIC" | topic: engine platform module primitive sweep | batch 16 |
| `mock/design_rounds/202604380000/202604380000_changelist.doc.lock.md` | 7 | 2026-04-21 | "**Phase:** DOC" | doc changelist: engine thread module primitive sweep | batch 16 |
| `mock/design_rounds/202604380000/202604380000_changelist.src.lock.md` | 88 | 2026-04-21 | "**Phase:** SRC-PLAN" | source changelist: engine thread module primitive sweep | batch 16 |
| `mock/design_rounds/202604380000/202604380000_topic.engine-thread-sweep.md` | 76 | 2026-04-21 | "**Phase:** TOPIC" | topic: engine thread module primitive sweep | batch 16 |
| `mock/design_rounds/202604390000/202604390000_changelist.doc.lock.md` | 7 | 2026-04-21 | "**Phase:** DOC" | doc changelist: engine dispatch module primitive sweep | batch 16 |
| `mock/design_rounds/202604390000/202604390000_changelist.src.lock.md` | 84 | 2026-04-21 | "**Phase:** SRC-PLAN" | source changelist: engine dispatch module primitive sweep | batch 16 |
| `mock/design_rounds/202604390000/202604390000_topic.engine-dispatch-sweep.md` | 38 | 2026-04-21 | "**Phase:** TOPIC" | topic: engine dispatch module primitive sweep | batch 16 |
| `mock/design_rounds/202604400000/202604400000_changelist.doc.lock.md` | 7 | 2026-04-21 | "**Phase:** DOC" | doc changelist: engine scheduler module primitive sweep | batch 16 |
| `mock/design_rounds/202604400000/202604400000_changelist.src.lock.md` | 49 | 2026-04-21 | "**Phase:** SRC-PLAN" | source changelist: engine scheduler module primitive sweep | batch 16 |
| `mock/design_rounds/202604400000/202604400000_topic.engine-scheduler-sweep.md` | 36 | 2026-04-21 | "**Phase:** TOPIC" | topic: engine scheduler module primitive sweep | batch 16 |
| `mock/design_rounds/202604410000/202604410000_changelist.doc.lock.md` | 7 | 2026-04-21 | "**Phase:** DOC" | doc changelist: engine plan module primitive sweep | batch 16 |
| `mock/design_rounds/202604410000/202604410000_changelist.src.lock.md` | 38 | 2026-04-21 | "**Phase:** SRC-PLAN" | source changelist: engine plan module primitive sweep | batch 16 |
| `mock/design_rounds/202604410000/202604410000_topic.engine-plan-sweep.md` | 55 | 2026-04-21 | "**Phase:** TOPIC" | topic: engine plan module primitive sweep | batch 16 |
| `mock/design_rounds/202604420000/202604420000_changelist.doc.lock.md` | 17 | 2026-04-21 | "**Phase:** DOC" | doc changelist: persistence crate primitive sweep | batch 16 |
| `mock/design_rounds/202604420000/202604420000_changelist.src.lock.md` | 77 | 2026-04-21 | "**Phase:** SRC-PLAN" | source changelist: persistence crate primitive sweep | batch 16 |
| `mock/design_rounds/202604420000/202604420000_topic.persistence-sweep.md` | 76 | 2026-04-21 | "**Phase:** TOPIC" | topic: persistence crate primitive sweep | batch 16 |
| `mock/design_rounds/202604430000/202604430000_changelist.doc.lock.md` | 10 | 2026-04-21 | "**Phase:** DOC" | doc changelist: str crate primitive sweep | batch 16 |
| `mock/design_rounds/202604430000/202604430000_changelist.src.lock.md` | 104 | 2026-04-21 | "**Phase:** SRC-PLAN" | source changelist: str crate primitive sweep | batch 16 |
| `mock/design_rounds/202604430000/202604430000_topic.str-sweep.md` | 76 | 2026-04-21 | "**Phase:** TOPIC" | topic: str crate primitive sweep | batch 16 |
| `mock/design_rounds/202604440000/202604440000_changelist.doc.lock.md` | 9 | 2026-04-21 | "**Phase:** DOC" | doc changelist: build crate primitive sweep | batch 16 |
| `mock/design_rounds/202604440000/202604440000_changelist.src.lock.md` | 93 | 2026-04-21 | "**Phase:** SRC-PLAN" | source changelist: build crate primitive sweep | batch 16 |
| `mock/design_rounds/202604440000/202604440000_topic.build-sweep.md` | 73 | 2026-04-21 | "**Phase:** TOPIC" | topic: build crate primitive sweep | batch 16 |
| `mock/design_rounds/202604450000/202604450000_changelist.doc.lock.md` | 36 | 2026-04-21 | "**Phase:** DOC" | doc changelist: writing-style sweep, em-dashes and exclamations | batch 16 |
| `mock/design_rounds/202604450000/202604450000_changelist.src.deprecated.md` | 24 | 2026-04-21 | "**Phase:** SRC-PLAN" | deprecated source changelist: writing-style sweep, em-dashes and exclamations | batch 16 |
| `mock/design_rounds/202604450000/202604450000_changelist.src.lock.md` | 32 | 2026-04-21 | "**Phase:** SRC-PLAN" | source changelist: writing-style sweep, em-dashes and exclamations | batch 16 |
| `mock/design_rounds/202604450000/202604450000_topic.writing-style-sweep.md` | 45 | 2026-04-21 | "**Phase:** TOPIC" | topic: writing-style sweep, em-dashes and exclamations | batch 16 |
| `mock/design_rounds/202604480000/202604480000_changelist.doc.lock.md` | 20 | 2026-04-21 | "**Phase:** DOC" | doc changelist: BACKLOG type-surface reminder block | batch 16 |
| `mock/design_rounds/202604480000/202604480000_changelist.src.lock.md` | 9 | 2026-04-21 | "**Phase:** SRC-PLAN" | source changelist: BACKLOG type-surface reminder block | batch 16 |
| `mock/design_rounds/202604480000/202604480000_topic.backlog-type-surface-reminder.md` | 37 | 2026-04-21 | "**Phase:** TOPIC" | topic: BACKLOG type-surface reminder block | batch 16 |
| `mock/design_rounds/202604500000/202604500000_changelist.doc.lock.md` | 24 | 2026-04-24 | "**Phase:** DOC" | doc changelist: extensions cop-out cleanup, AbiVersion CStr | batch 16 |
| `mock/design_rounds/202604500000/202604500000_changelist.src.lock.md` | 35 | 2026-04-24 | "**Phase:** SRC" | source changelist: extensions cop-out cleanup, AbiVersion CStr | batch 16 |
| `mock/design_rounds/202604500000/202604500000_topic.extensions-cop-out-cleanup.md` | 34 | 2026-04-24 | "**Phase:** TOPIC" | topic: extensions cop-out cleanup, AbiVersion CStr | batch 16 |
| `mock/design_rounds/202604510000/202604510000_changelist.doc.lock.md` | 24 | 2026-04-24 | "**Phase:** DOC" | doc changelist: FFI-boundary primitive policy, extensions | batch 16 |
| `mock/design_rounds/202604510000/202604510000_changelist.src.lock.md` | 33 | 2026-04-24 | "**Phase:** SRC" | source changelist: FFI-boundary primitive policy, extensions | batch 16 |
| `mock/design_rounds/202604510000/202604510000_topic.ffi-boundary-primitive-policy.md` | 66 | 2026-04-24 | "**Phase:** TOPIC" | topic: FFI-boundary primitive policy, extensions | batch 16 |
| `mock/design_rounds/202604520000-abandoned/202604520000_changelist.doc.deprecated.md` | 124 | 2026-04-26 | "**Phase:** DOC (living)" | abandoned (directory name); doc changelist: app-builder formalisation: ProviderKit, Demand/Supply | batch 16 |
| `mock/design_rounds/202604520000-abandoned/202604520000_changelist.src.deprecated.md` | 192 | 2026-04-26 | "**Phase:** SRC (living)" | abandoned (directory name); deprecated source changelist: app-builder formalisation: ProviderKit, Demand/Supply | batch 16 |
| `mock/design_rounds/202604520000-abandoned/202604520000_topic.app-builder-formalisation.md` | 134 | 2026-04-26 | "**Phase:** TOPIC (complete)" | abandoned (directory name); topic: app-builder formalisation: ProviderKit, Demand/Supply | batch 16 |
| `mock/design_rounds/202604520000-abandoned/202604520000_topic.use-existing-access-set.md` | 95 | 2026-04-26 | "**Phase:** TOPIC (frozen on commit)" | abandoned (directory name); topic: correction: validate via existing Has bounds | batch 16 |
| `mock/design_rounds/202605010900/202605010900_changelist.doc.lock.md` | 201 | 2026-05-01 | "**Phase:** DOC" | doc changelist: SchedulerBuilder type-state, Kit crate, Buildable | batch 16 |
| `mock/design_rounds/202605010900/202605010900_changelist.src.lock.md` | 236 | 2026-05-01 | "**Phase:** SRC-PLAN" | source changelist: SchedulerBuilder type-state, Kit crate, Buildable | batch 16 |
| `mock/design_rounds/202605010900/202605010900_topic.app-builder-typestate-and-kit.md` | 132 | 2026-05-01 | "**Phase:** TOPIC" | topic: SchedulerBuilder type-state, Kit crate, Buildable | batch 16 |
| `mock/design_rounds/202605011000/202605011000_changelist.doc.lock.md` | 28 | 2026-05-01 | "**Phase:** DOC" | doc changelist: post-review polish of builder bounds, tests | batch 16 |
| `mock/design_rounds/202605011000/202605011000_changelist.src.lock.md` | 123 | 2026-05-01 | "**Phase:** SRC-PLAN" | source changelist: post-review polish of builder bounds, tests | batch 16 |
| `mock/design_rounds/202605011000/202605011000_topic.post-review-polish.md` | 72 | 2026-05-01 | "**Phase:** TOPIC" | topic: post-review polish of builder bounds, tests | batch 16 |
| `mock/design_rounds/202605011500/202605011500_changelist.doc.lock.md` | 251 | 2026-05-01 | "**Phase:** DOC" | doc changelist: uncap Wus, read!/write!, BuilderExtending, Depth | batch 16 |
| `mock/design_rounds/202605011500/202605011500_changelist.src.lock.md` | 358 | 2026-05-01 | "**Phase:** SRC-PLAN" | source changelist: uncap Wus, read!/write!, BuilderExtending, Depth | batch 16 |
| `mock/design_rounds/202605011500/202605011500_topic.third-reviewer-amendments.md` | 168 | 2026-05-01 | "**Phase:** TOPIC (addendum to `202605011500_topic.uncap-wus-and-polish.md`, which is frozen)" | topic: third-reviewer amendments, cons-list uncap | batch 16 |
| `mock/design_rounds/202605011500/202605011500_topic.uncap-wus-and-polish.md` | 149 | 2026-05-01 | "**Phase:** TOPIC" | topic: uncap Wus, read!/write!, BuilderExtending, Depth | batch 16 |
| `mock/design_rounds/202605041228/202605041228_changelist.doc.lock.md` | 82 | no date line | "status: open" | doc changelist: arvo rounds adoption, typed-const Depth | batch 16 |
| `mock/design_rounds/202605041228/202605041228_changelist.src.lock.md` | 149 | no date line | "status: open" | source CL: arvo pin bump, AccessMask alias deletion, typed-const Depth | batch 17 |
| `mock/design_rounds/202605041228/202605041228_topic_arvo_rounds_1_8_adoption.md` | 87 | no date line | "status: frozen" | topic: mechanical adoption of arvo Rounds 1-8 | batch 17 |
| `mock/design_rounds/202605041228/202605041228_topic_typed_const_completion.md` | 110 | no date line | "status: frozen" | topic: typed-const completion of PR #46 deferred sites | batch 17 |
| `mock/design_rounds/202605041257/202605041257_changelist.doc.lock.md` | 144 | no date line | "status: open" | doc CL: hilavitkutin-providers v0 templates, v0 scoping | batch 17 |
| `mock/design_rounds/202605041257/202605041257_changelist.src.lock.md` | 80 | no date line | "status: open" | source CL: providers crate manifest, interner module, smoke test | batch 17 |
| `mock/design_rounds/202605041257/202605041257_topic_hilavitkutin_providers_v0.md` | 185 | no date line | "status: frozen" | topic: providers crate, InternerApi, MemoryArena, decisions | batch 17 |
| `mock/design_rounds/202605041320/202605041320_changelist.doc.lock.md` | 27 | no date line | "status: open" | doc CL: no template edits for PR #48 fixes | batch 17 |
| `mock/design_rounds/202605041320/202605041320_changelist.src.lock.md` | 86 | no date line | "status: open" | source CL: arena unsafe write fixes, em-dash, soundness test | batch 17 |
| `mock/design_rounds/202605041320/202605041320_topic_pr48_review_fixes.md` | 109 | no date line | "status: frozen" | topic: PR #48 review findings and fixes | batch 17 |
| `mock/design_rounds/202605041345/202605041345_changelist.doc.lock.md` | 168 | 2026-05-04 | unstated (doc changelist, "Phase: DOC") | doc CL: BuilderResource section, InternerKit promotion | batch 17 |
| `mock/design_rounds/202605041345/202605041345_changelist.src.lock.md` | 268 | 2026-05-04 | unstated (source changelist, "Phase: SRC-PLAN") | source CL: BuilderResource trait, engine impl, InternerKit | batch 17 |
| `mock/design_rounds/202605041345/202605041345_topic_providers_v0_1_kit_bridge.md` | 186 | 2026-05-04 | unstated (topic, "Phase: TOPIC") | topic: InternerKit and BuilderResource bridging trait | batch 17 |
| `mock/design_rounds/202605041402/202605041402_changelist.doc.lock.md` | 16 | 2026-05-04 | unstated (doc changelist, "Phase: DOC") | doc CL: no template edits | batch 17 |
| `mock/design_rounds/202605041402/202605041402_changelist.src.lock.md` | 50 | 2026-05-04 | unstated (source changelist, "Phase: SRC-PLAN") | source CL: one add_kit smoke test | batch 17 |
| `mock/design_rounds/202605041402/202605041402_topic_pr49_review_followup.md` | 53 | 2026-05-04 | unstated (topic, "Phase: TOPIC") | topic: PR #49 advisory finding F3 follow-up | batch 17 |
| `mock/design_rounds/202605041547/202605041547_changelist.doc.lock.md` | 39 | 2026-05-04 | unstated (doc changelist, "Phase: DOC") | doc CL: linking error-shapes paragraph, unix errno | batch 17 |
| `mock/design_rounds/202605041547/202605041547_changelist.src.lock.md` | 38 | 2026-05-04 | unstated (source changelist, "Phase: SRC") | source CL: read_errno body, LoadFailed doc comment | batch 17 |
| `mock/design_rounds/202605041547/202605041547_topic_unix_real_errno.md` | 143 | 2026-05-04 | unstated (topic, "Phase: TOPIC") | topic: real errno on Unix dlopen failure | batch 17 |
| `mock/design_rounds/202605041618/202605041618_changelist.doc.lock.md` | 37 | 2026-05-04 | unstated (doc changelist, "Phase: DOC") | doc CL: PathEncodingUnsupported paragraph | batch 17 |
| `mock/design_rounds/202605041618/202605041618_changelist.src.lock.md` | 38 | 2026-05-04 | unstated (source changelist, "Phase: SRC") | source CL: LinkError variant, windows error map | batch 17 |
| `mock/design_rounds/202605041618/202605041618_topic_path_encoding_unsupported.md` | 90 | 2026-05-04 | unstated (topic, "Phase: TOPIC") | topic: PathEncodingUnsupported for Windows paths | batch 17 |
| `mock/design_rounds/202605041624/202605041624_changelist.doc.lock.md` | 15 | 2026-05-04 | unstated (doc changelist, "Phase: DOC") | doc CL: no doc changes, test-only | batch 17 |
| `mock/design_rounds/202605041624/202605041624_changelist.src.lock.md` | 26 | 2026-05-04 | unstated (source changelist, "Phase: SRC") | source CL: one test match arm | batch 17 |
| `mock/design_rounds/202605041624/202605041624_topic_test_accept_path_encoding.md` | 45 | 2026-05-04 | unstated (topic, "Phase: TOPIC") | topic: test accepts PathEncodingUnsupported | batch 17 |
| `mock/design_rounds/202605041629/202605041629_changelist.doc.lock.md` | 71 | 2026-05-04 | unstated (doc changelist, "Phase: DOC") | doc CL: extensions DESIGN shutdown observer edits | batch 17 |
| `mock/design_rounds/202605041629/202605041629_changelist.src.lock.md` | 56 | 2026-05-04 | unstated (source changelist, "Phase: SRC") | source CL: ShutdownObserverFn, host and extension fields | batch 17 |
| `mock/design_rounds/202605041629/202605041629_topic_shutdown_observer.md` | 129 | 2026-05-04 | unstated (topic, "Phase: TOPIC") | topic: shutdown observer hook on ExtensionHost | batch 17 |
| `mock/design_rounds/202605041648/202605041648_changelist.doc.lock.md` | 59 | 2026-05-04 | unstated (doc changelist, "Phase: DOC") | doc CL: trampoline LTO preservation, revised after probe | batch 17 |
| `mock/design_rounds/202605041648/202605041648_changelist.src.deprecated.md` | 96 | 2026-05-04 | abandoned (filename "deprecated"); text says "Replaces the deprecated" | source CL: no_mangle trampolines, LTO smoke fixture | batch 17 |
| `mock/design_rounds/202605041648/202605041648_changelist.src.lock.md` | 94 | 2026-05-04 | unstated (source changelist, "Phase: SRC") | source CL: no_mangle trampolines, LTO smoke fixture | batch 17 |
| `mock/design_rounds/202605041648/202605041648_topic_trampoline_lto_survival.md` | 76 | 2026-05-04 | unstated (topic, "Phase: TOPIC") | topic: trampoline survival under release and LTO | batch 17 |
| `mock/design_rounds/202605041737/202605041737_changelist.doc.lock.md` | 57 | 2026-05-04 | unstated (doc changelist, "Phase: DOC") | doc CL: trampoline prose rewrite, dlopen flags sentence | batch 17 |
| `mock/design_rounds/202605041737/202605041737_changelist.src.lock.md` | 63 | 2026-05-04 | unstated (source changelist, "Phase: SRC") | source CL: discriminator, distinct-address test, RTLD_LOCAL | batch 17 |
| `mock/design_rounds/202605041737/202605041737_topic_lto_review_followup.md` | 47 | 2026-05-04 | unstated (topic, "Phase: TOPIC") | topic: PR #54 review findings F1 to F3 | batch 17 |
| `mock/design_rounds/202605042200/202605042200_changelist.doc.lock.md` | 254 | 2026-05-05 | unstated (doc changelist, "Phase: DOC") | round 4 doc CL: Kit split, Replaceable, bridge deletion | batch 17 |
| `mock/design_rounds/202605042200/202605042200_changelist.src.lock.md` | 752 | 2026-05-05 | unstated (source changelist, "Phase: SRC-PLAN") | round 4 source CL: Cons/Empty, Kit, builder typestate | batch 17 |
| `mock/design_rounds/202605042200/202605042200_topic_builder_register_unification.md` | 263 | 2026-05-04 | unstated (topic, "Phase: TOPIC"); later topic says "supersedes both" | topic 1: BuilderRegister, StoreMarker, three audits | batch 17 |
| `mock/design_rounds/202605042200/202605042200_topic_kit_trait_split.md` | 203 | 2026-05-05 | unstated (topic, "Phase: TOPIC"); "That is the locked shape." | topic 3: Kit Units and Owned split, open items | batch 17 |
| `mock/design_rounds/202605042200/202605042200_topic_markers_as_registrables_final.md` | 443 | 2026-05-04 | unstated (topic, "Phase: TOPIC"); later topic says "REJECTED" | topic 2: Registrable, NotIn, Kit in api | batch 17 |
| `mock/design_rounds/202605042200/202605042200_topic_round_4_audit.md` | 171 | 2026-05-05 | unstated (topic, "Phase: TOPIC") | audit findings C1-C4, M1-M6, remediation order | batch 17 |
| `mock/design_rounds/202605042200/202605042200_topic_round_4_findings.md` | 202 | 2026-05-05 | unstated (topic, "Phase: TOPIC") | synthesis of sketches S1-S5, whole round-4 surface | batch 17 |
| `mock/design_rounds/202605042200/202605042200_topic_round_4_layered_enforcement.md` | 69 | 2026-05-05 | "Frozen on commit" | corrected framing of kit-private state enforcement | batch 17 |
| `mock/design_rounds/202605042200/202605042200_topic_round_4_m6_decision.md` | 42 | 2026-05-05 | unstated (topic, "Phase: TOPIC") | audit M6 decision: drop requirements_doc | batch 17 |
| `mock/design_rounds/202605042200/202605042200_topic_round_4_sketches.md` | 182 | 2026-05-05 | "**Phase:** TOPIC" | Topic: round-4 sketches, S1 to S5 | batch 18 |
| `mock/design_rounds/202605050200/202605050200_changelist.doc.lock.md` | 9 | 2026-05-05 | "**Phase:** DOC" | Doc changelist: USize(0) → USize::ZERO | batch 18 |
| `mock/design_rounds/202605050200/202605050200_changelist.src.lock.md` | 60 | 2026-05-05 | "**Phase:** SRC" | Source changelist: USize(0) → USize::ZERO | batch 18 |
| `mock/design_rounds/202605050200/202605050200_topic_usize_zero_canonical.md` | 26 | 2026-05-05 | "**Phase:** TOPIC" | Topic: USize(0) → USize::ZERO canonicalisation in hilavitkutin-api | batch 18 |
| `mock/design_rounds/202605050300/202605050300_changelist.doc.lock.md` | 9 | 2026-05-05 | "**Phase:** DOC" | Doc changelist: USize(0) -> USize::ZERO sweep | batch 18 |
| `mock/design_rounds/202605050300/202605050300_changelist.src.lock.md` | 58 | 2026-05-05 | "**Phase:** SRC" | Source changelist: USize(0) -> USize::ZERO sweep across engine, persistence, providers | batch 18 |
| `mock/design_rounds/202605050300/202605050300_topic_usize_zero_sweep_remaining.md` | 24 | 2026-05-05 | "**Phase:** TOPIC" | Topic: USize(0) -> USize::ZERO sweep across engine, persistence, providers | batch 18 |
| `mock/design_rounds/202605050400/202605050400_changelist.doc.lock.md` | 9 | 2026-05-05 | "**Phase:** DOC" | Doc changelist: em-dash cleanup | batch 18 |
| `mock/design_rounds/202605050400/202605050400_changelist.src.lock.md` | 36 | 2026-05-05 | "**Phase:** SRC" | Source changelist: em-dash sweep across hilavitkutin source | batch 18 |
| `mock/design_rounds/202605050400/202605050400_topic_em_dash_cleanup.md` | 26 | 2026-05-05 | "**Phase:** TOPIC" | Topic: em-dash sweep across hilavitkutin source | batch 18 |
| `mock/design_rounds/202605050500/202605050500_changelist.doc.lock.md` | 7 | 2026-05-05 | "**Phase:** DOC" | Doc changelist: em-dash polish | batch 18 |
| `mock/design_rounds/202605050500/202605050500_changelist.src.lock.md` | 48 | 2026-05-05 | "**Phase:** SRC" | Source changelist: em-dash polish (round-202605050400 follow-up) | batch 18 |
| `mock/design_rounds/202605050500/202605050500_topic_em_dash_polish.md` | 25 | 2026-05-05 | "**Phase:** TOPIC" | Topic: em-dash polish (round-202605050400 follow-up) | batch 18 |
| `mock/design_rounds/202605050600/202605050600_changelist.doc.lock.md` | 7 | 2026-05-05 | "**Phase:** DOC" | Doc changelist: str test rehab | batch 18 |
| `mock/design_rounds/202605050600/202605050600_changelist.src.lock.md` | 33 | 2026-05-05 | "**Phase:** SRC" | Source changelist: str test rehab | batch 18 |
| `mock/design_rounds/202605050600/202605050600_topic_str_test_rehab.md` | 18 | 2026-05-05 | "**Phase:** TOPIC" | Topic: hilavitkutin-str test rehab (partial #291) | batch 18 |
| `mock/design_rounds/202605051100/202605051100_changelist.doc.lock.md` | 102 | 2026-05-04 | "**Phase:** DOC" | DOC changelist: template leakage scrub | batch 18 |
| `mock/design_rounds/202605051100/202605051100_changelist.src.lock.md` | 25 | 2026-05-04 | "**Phase:** SRC" | SRC changelist: template leakage scrub | batch 18 |
| `mock/design_rounds/202605051100/202605051100_topic_template_leakage_scrub.md` | 67 | 2026-05-04 | "**Phase:** TOPIC" | Template scrub: remove polka-dots / saalis / loimu / clause-dev | batch 18 |
| `mock/design_rounds/202605051400/202605051400_changelist.doc.lock.md` | 81 | 2026-05-05 | "**Phase:** DOC" | Round doc CL: documentation audit consolidation (hilavitkutin) | batch 18 |
| `mock/design_rounds/202605051400/202605051400_changelist.src.lock.md` | 23 | 2026-05-08 | "**Phase:** SRC" | Round src CL: documentation audit consolidation (hilavitkutin): no source | batch 18 |
| `mock/design_rounds/202605051400/202605051400_topic_doc_audit_2026_05_05.md` | 91 | 2026-05-05 | "**Phase:** TOPIC" | Doc audit consolidation round (hilavitkutin) | batch 18 |
| `mock/design_rounds/202605081700/202605081700_changelist.doc.lock.md` | 21 | 2026-05-08 | "**Phase:** DOC" | Round doc CL: PR-review fixes for round 202605051400 | batch 18 |
| `mock/design_rounds/202605081700/202605081700_changelist.src.lock.md` | 14 | 2026-05-08 | "**Phase:** SRC" | Round src CL: PR-review fixes: no source touches | batch 18 |
| `mock/design_rounds/202605081700/202605081700_topic_review_fixes.md` | 41 | 2026-05-08 | "**Phase:** TOPIC" | PR-review follow-up: persistence field-rename slip + extensions-macros truncation | batch 18 |
| `mock/design_rounds/202605082142/202605082142_changelist.doc.lock.md` | 95 | 2026-05-08 | "**Phase:** DOC" | Doc CL: AccessSet legacy-A deletion + HList-only framing | batch 18 |
| `mock/design_rounds/202605082142/202605082142_changelist.src.lock.md` | 58 | 2026-05-08 | "**Phase:** DRAFT" | Source CL: AccessSet legacy-A deletion + LEN fix + macro-form | batch 18 |
| `mock/design_rounds/202605082142/202605082142_topic_access_set_arity_solve.md` | 88 | 2026-05-08 | "**Phase:** TOPIC" | AccessSet arity solve: delete legacy flat-tuple A, retain canonical HList | batch 18 |
| `mock/design_rounds/202605082321/202605082321_changelist.doc.lock.md` | 176 | 2026-05-08 | "**Phase:** DOC" | Doc CL: trait diagnostics + prelude module + recursion-limit guidance | batch 18 |
| `mock/design_rounds/202605082321/202605082321_changelist.src.lock.md` | 432 | 2026-05-08 | "**Phase:** DRAFT" | Source CL: prelude module + diagnostic coverage + Contains note | batch 18 |
| `mock/design_rounds/202605082321/202605082321_topic_trait_diagnostics_prelude.md` | 231 | 2026-05-08 | "**Phase:** TOPIC" | Trait diagnostics across the public API + recursion-limit guidance + | batch 18 |
| `mock/design_rounds/202605082344/202605082344_changelist.doc.lock.md` | 20 | 2026-05-08 | "**Phase:** DOC" | Doc CL: no documentation edits | batch 18 |
| `mock/design_rounds/202605082344/202605082344_changelist.src.lock.md` | 58 | 2026-05-08 | "**Phase:** DRAFT" | Source CL: correct hint diagnostic marker lists | batch 18 |
| `mock/design_rounds/202605082344/202605082344_topic_hint_diagnostic_marker_lists.md` | 64 | 2026-05-08 | "**Phase:** TOPIC" | Hint diagnostic notes: correct the implementor lists | batch 18 |
| `mock/design_rounds/202605082349/202605082349_changelist.doc.lock.md` | 48 | 2026-05-08 | "**Phase:** DOC" | Doc CL: SchedulerBuilder method-name updates in templates | batch 18 |
| `mock/design_rounds/202605082349/202605082349_changelist.src.lock.md` | 139 | 2026-05-08 | "**Phase:** DRAFT" | Source CL: SchedulerBuilder method renames | batch 18 |
| `mock/design_rounds/202605082349/202605082349_topic_builder_api_rename.md` | 104 | 2026-05-08 | "**Phase:** TOPIC" | SchedulerBuilder method renames for naming consistency | batch 18 |
| `mock/design_rounds/202605090011/202605090011_changelist.doc.lock.md` | 136 | no date line | "phase: DOC" | Doc CL: descriptor v1.1 abi hardening | batch 18 |
| `mock/design_rounds/202605090011/202605090011_changelist.src.lock.md` | 142 | no date line | "phase: DRAFT" | Src CL: descriptor v1.1 abi hardening | batch 18 |
| `mock/design_rounds/202605090011/202605090011_topic.descriptor-v1-1-abi-hardening.md` | 267 | no date line | "phase: TOPIC" | ExtensionDescriptor v1.1 ABI hardening | batch 18 |
| `mock/design_rounds/202605090033/202605090033_changelist.doc.lock.md` | 43 | no date line | "phase: DOC" | Doc CL: descriptor v1.1 review-fix | batch 18 |
| `mock/design_rounds/202605090033/202605090033_changelist.src.lock.md` | 36 | no date line | "phase: DRAFT" | Src CL: descriptor v1.1 review-fix | batch 18 |
| `mock/design_rounds/202605090033/202605090033_topic.descriptor-v1-1-review-fix.md` | 99 | no date line | "phase: TOPIC" | Descriptor v1.1 review-fix | batch 18 |
| `mock/design_rounds/202605090047/202605090047_changelist.doc.lock.md` | 25 | no date line | "phase: DOC" | Doc CL: test rehab #291 | batch 18 |
| `mock/design_rounds/202605090047/202605090047_changelist.src.lock.md` | 72 | no date line | "phase: DRAFT" | Src CL: test rehab #291 (phase 1) | batch 18 |
| `mock/design_rounds/202605090047/202605090047_topic.test-rehab-291.md` | 99 | no date line | "phase: TOPIC" | Hilavitkutin test rehab post-arvo-Round-D | batch 18 |
| `mock/design_rounds/202605090144/202605090144_changelist.doc.lock.md` | 39 | no date line | "phase: DOC" | Doc CL: str linker-section rationale | batch 18 |
| `mock/design_rounds/202605090144/202605090144_changelist.src.lock.md` | 23 | no date line | "phase: DRAFT" | Src CL: str linker-section rationale (no source changes) | batch 18 |
| `mock/design_rounds/202605090144/202605090144_topic.str-linker-rationale.md` | 128 | no date line | "phase: TOPIC" | Linker-section rationale for hilavitkutin-str | batch 18 |
| `mock/design_rounds/202605091700/202605091700_changelist.doc.lock.md` | 287 | 2026-05-09 | "**Phase:** DOC" | Doc changelist: round 202605091700 | batch 18 |
| `mock/design_rounds/202605091700/202605091700_changelist.src.lock.md` | 190 | 2026-05-09 | "**Phase:** SRC" | Source changelist: round 202605091700 | batch 18 |
| `mock/design_rounds/202605091700/202605091700_topic.builder-shape.md` | 325 | 2026-05-09 | "**Phase:** TOPIC" | Builder shape: one verb, value-driven, dispatch-via-associated-type | batch 18 |
| `mock/design_rounds/202605091700/202605091700_topic.provider-unification.md` | 235 | 2026-05-09 | "**Phase:** TOPIC" | Provider unification: capability becomes provider; extensions provide bins, not slots | batch 18 |
| `mock/design_rounds/202605101036/202605101036_changelist.doc.lock.md` | 203 | 2026-05-11 | "**Phase:** DOC — drafted autonomously per overnight protocol 2026-05-11. Land time = next user session." | Doc changelist: round 202605101036 (hilavitkutin runtime megaround) | batch 18 |
| `mock/design_rounds/202605101036/202605101036_changelist.src.lock.md` | 653 | 2026-05-11 | "**Phase:** SRC-PLAN. Drafted post-doc-lock" | src changelist for runtime megaround, 8 passes, api and engine shapes | batch 19 |
| `mock/design_rounds/202605101036/202605101036_topic.adapt-subsystem.md` | 315 | 2026-05-10 | "**Phase:** TOPIC (open)" | adapt subsystem axes, metrics resources, locked decisions | batch 19 |
| `mock/design_rounds/202605101036/202605101036_topic.builder-input-rename-and-reshape.md` | 132 | 2026-05-10 | "**Phase:** TOPIC" | Provider renamed BuilderInput; wrapper structs deleted; ExtensionSurface | batch 19 |
| `mock/design_rounds/202605101036/202605101036_topic.dispatch-codegen.md` | 283 | 2026-05-10 | "**Phase:** TOPIC (open)" | dispatch codegen axes A-H and decision log | batch 19 |
| `mock/design_rounds/202605101036/202605101036_topic.morsel-loop.md` | 67 | 2026-05-11 | "**Phase:** TOPIC, closed (autonomous trivial calls + one user-attention flag)." | morsel loop axes, micro-morsel interval, sync points | batch 19 |
| `mock/design_rounds/202605101036/202605101036_topic.per-core-dispatch.md` | 73 | 2026-05-11 | "**Phase:** TOPIC, closed (no fresh axes; consolidated as cross-topic confirmation)." | CoreAssignment and CoreProgram final shape | batch 19 |
| `mock/design_rounds/202605101036/202605101036_topic.plan-algorithm-chain.md` | 1283 | 2026-05-10 | "**Phase:** TOPIC" | 12-step plan chain, ExecutionPlan shape, soundness S1-S7 | batch 19 |
| `mock/design_rounds/202605101036/202605101036_topic.plan-caching.md` | 87 | 2026-05-11 | "**Phase:** TOPIC, closed (autonomous trivial calls + one bench-gated value)." | plan recompute trigger, CSR graph, dirty granularity | batch 19 |
| `mock/design_rounds/202605101036/202605101036_topic.resource-resolution.md` | 90 | 2026-05-11 | "**Phase:** TOPIC, closed (autonomous trivial calls + one user-attention flag)." | ResourceSnapshot semantics, PlanAffecting, persistence wiring | batch 19 |
| `mock/design_rounds/202605101036/202605101036_topic.scheduler-run-signature.md` | 358 | 2026-05-10 | "**Phase:** TOPIC" | Scheduler::run signature, RunCfg, plan-cache, adapt path | batch 19 |
| `mock/design_rounds/202605101036/202605101036_topic.test-and-example-bar.md` | 84 | 2026-05-11 | "**Phase:** TOPIC, closed (autonomous trivial calls + one user-attention flag for example app picks)." | test pyramid, three example apps, test-utils crate | batch 19 |
| `mock/design_rounds/202605101036/202605101036_topic.thread-pool.md` | 131 | 2026-05-10 | "**Phase:** TOPIC, closed-pending-sketch (PoolFrame lifetime)." | Executor, parking tiers, PoolFrame, shutdown, WakeStrategy | batch 19 |
| `mock/design_rounds/202605101200/202605101200_changelist.doc.lock.md` | 14 | 2026-05-10 | "**Phase:** DOC" | doc CL adding KitDispatch SHAME entry | batch 19 |
| `mock/design_rounds/202605101200/202605101200_changelist.src.lock.md` | 7 | 2026-05-10 | "**Phase:** SRC" | empty source CL, doc-only round | batch 19 |
| `mock/design_rounds/202605101200/202605101200_topic.kitdispatch-shame.md` | 13 | 2026-05-10 | "**Phase:** TOPIC" | KitDispatch lives in kit crate, SHAME entry | batch 19 |
| `mock/design_rounds/202605102100/202605102100_changelist.doc.lock.md` | 31 | 2026-05-10 | "**Phase:** DOC" | Capability to Provider doc sweep, BACKLOG entry | batch 19 |
| `mock/design_rounds/202605102100/202605102100_changelist.src.lock.md` | 31 | 2026-05-10 | "**Phase:** SRC" | em-dash removal, macro identifier renames | batch 19 |
| `mock/design_rounds/202605102100/202605102100_topic.review-fixups.md` | 20 | 2026-05-10 | "**Phase:** TOPIC" | PR 73 review fixups, mechanical only | batch 19 |
| `mock/design_rounds/202605110000/202605110000_changelist.doc.lock.md` | 23 | no date line | "phase: DOC" | empty doc CL for engine test rehab | batch 19 |
| `mock/design_rounds/202605110000/202605110000_changelist.src.lock.md` | 87 | no date line | "phase: SRC" | test fixture migration to arvo types | batch 19 |
| `mock/design_rounds/202605110000/202605110000_topic.engine-test-rehab.md` | 87 | 2026-05-10 | "**Phase:** TOPIC" | engine test rehab past arvo API drift | batch 19 |
| `mock/design_rounds/202605232000/202605232000_changelist.doc.lock.md` | 30 | 2026-05-23 | "**Phase:** DOC" | empty doc CL, post-review source fixes | batch 19 |
| `mock/design_rounds/202605232000/202605232000_changelist.src.deprecated.md` | 48 | 2026-05-23 | "**Phase:** SRC" (file name: deprecated) | superseded src CL, plan_dirty lift | batch 19 |
| `mock/design_rounds/202605232000/202605232000_changelist.src.lock.md` | 47 | 2026-05-23 | "**Phase:** SRC" | src CL, plan_dirty lift deferred, stub rename | batch 19 |
| `mock/design_rounds/202605232000/202605232000_topic.runtime-megaround-post-review-fixes.md` | 45 | 2026-05-23 | "**Phase:** TOPIC" | post-review fixes to megaround source drift | batch 19 |
| `mock/design_rounds/202605232020/202605232020_changelist.doc.lock.md` | 30 | 2026-05-23 | "**Phase:** DOC" | empty doc CL for megaround deferrals | batch 19 |
| `mock/design_rounds/202605232020/202605232020_changelist.src.lock.md` | 32 | 2026-05-23 | "**Phase:** SRC" | trybuild fixture, MIRI.md, ema_update Sample type | batch 19 |
| `mock/design_rounds/202605232020/202605232020_topic.runtime-megaround-deferrals.md` | 34 | 2026-05-23 | "**Phase:** TOPIC" | three deferrals from megaround Pass 7 | batch 19 |
| `mock/design_rounds/202605232029/202605232029_changelist.doc.lock.md` | 16 | 2026-05-23 | "**Phase:** DOC" | empty doc CL for PR review polish | batch 19 |
| `mock/design_rounds/202605232029/202605232029_changelist.src.lock.md` | 30 | 2026-05-23 | "**Phase:** SRC" | docstring honesty, assertion, README pointer | batch 19 |
| `mock/design_rounds/202605232029/202605232029_topic.deferrals-pr-review-polish.md` | 32 | 2026-05-23 | "**Phase:** TOPIC" | four review polish items, no design change | batch 19 |
| `mock/design_rounds/202605250545/202605250545_changelist.doc.lock.md` | 22 | 2026-05-25 | "**Phase:** DOC" | assign_cores DESIGN section, BACKLOG entry | batch 19 |
| `mock/design_rounds/202605250545/202605250545_changelist.src.lock.md` | 33 | 2026-05-25 | "**Phase:** SRC" | assign_cores round-robin body and tests | batch 19 |
| `mock/design_rounds/202605250545/202605250545_topic.hila-c4-assign-cores-base.md` | 57 | 2026-05-25 | "**Phase:** TOPIC" | base assign_cores pipeline-wide round-robin | batch 19 |
| `mock/design_rounds/202605250600/202605250600_changelist.doc.lock.md` | 22 | 2026-05-25 | "**Phase:** DOC" | select_approach DESIGN section, BACKLOG entry | batch 19 |
| `mock/design_rounds/202605250600/202605250600_changelist.src.lock.md` | 32 | 2026-05-25 | "**Phase:** SRC" | select_approach three-branch body and tests | batch 19 |
| `mock/design_rounds/202605250600/202605250600_topic.hila-c2-select-approach-base.md` | 56 | 2026-05-25 | "**Phase:** TOPIC" | base select_approach heuristic, 10K threshold | batch 19 |
| `mock/design_rounds/202605250607/202605250607_changelist.doc.lock.md` | 16 | 2026-05-25 | "**Phase:** DOC" | select_approach DESIGN revised with typed constants | batch 19 |
| `mock/design_rounds/202605250607/202605250607_changelist.src.lock.md` | 40 | 2026-05-25 | "**Phase:** SRC" | typed USize thresholds in select_approach | batch 19 |
| `mock/design_rounds/202605250607/202605250607_topic.hila-c2-reviewer-fixes.md` | 33 | 2026-05-25 | "**Phase:** TOPIC" | PR 81 review fixes for select_approach | batch 19 |
| `mock/design_rounds/202605250615/202605250615_changelist.doc.lock.md` | 22 | 2026-05-25 | "**Phase:** DOC" | codegen stubs DESIGN section, BACKLOG entry | batch 19 |
| `mock/design_rounds/202605250615/202605250615_changelist.src.lock.md` | 35 | 2026-05-25 | "**Phase:** SRC" | codegen_fiber and codegen_core skeleton returns | batch 19 |
| `mock/design_rounds/202605250615/202605250615_topic.hila-c2-codegen-stubs.md` | 54 | 2026-05-25 | "**Phase:** TOPIC" | codegen skeleton stubs, LLVM monomorphisation deferred | batch 19 |
| `mock/design_rounds/202605250628/202605250628_changelist.doc.lock.md` | 22 | 2026-05-25 | "**Phase:** DOC" | Scheduler::run no-op body DESIGN section | batch 19 |
| `mock/design_rounds/202605250628/202605250628_changelist.src.lock.md` | 27 | 2026-05-25 | "**Phase:** SRC" | run body returns Cfg::Out::default | batch 19 |
| `mock/design_rounds/202605250628/202605250628_topic.hila-scheduler-run-body.md` | 49 | 2026-05-25 | "**Phase:** TOPIC" | transitional no-op Scheduler::run body | batch 19 |
| `mock/design_rounds/202605250645/202605250645_changelist.doc.lock.md` | 22 | 2026-05-25 | "**Phase:** DOC" | DESIGN corrections for assign_cores and run bound | batch 19 |
| `mock/design_rounds/202605250645/202605250645_changelist.src.lock.md` | 32 | 2026-05-25 | "**Phase:** SRC" | test additions, rustdoc trim | batch 19 |
| `mock/design_rounds/202605250645/202605250645_topic.engine-pr-followups.md` | 41 | 2026-05-25 | "**Phase:** TOPIC" | consolidated engine PR review follow-ups | batch 19 |
| `mock/design_rounds/202605282256/202605282256_changelist.doc.lock.md` | 25 | no date line | "Phase: DOC" | ResourceSnapshot stale signature removed from docs | batch 19 |
| `mock/design_rounds/202605282256/202605282256_changelist.src.lock.md` | 32 | no date line | "Phase: IMPL" | delete placeholder ResourceSnapshot, ResourceCache | batch 19 |
| `mock/design_rounds/202605282256/202605282256_topic.resource-snapshot-disambiguation.md` | 32 | 2026-05-28 | "**Phase:** TOPIC" | one meaning for ResourceSnapshot, placeholder deleted | batch 19 |
| `mock/design_rounds/202605282319/202605282319_changelist.doc.lock.md` | 17 | no date line | "Phase: DOC" | builder four-param shape with Staged | batch 19 |
| `mock/design_rounds/202605282319/202605282319_changelist.src.lock.md` | 43 | no date line | "Phase: IMPL" | StagedResource, Stage list, builder retains values | batch 19 |
| `mock/design_rounds/202605282319/202605282319_topic.builder-value-retention.md` | 29 | 2026-05-28 | "**Phase:** TOPIC" | builder value retention, first data-plane round | batch 19 |
| `mock/design_rounds/202605290018/202605290018_changelist.doc.lock.md` | 13 | no date line | "Phase: DOC" | builder paragraph to StoreValues arena shape | batch 19 |
| `mock/design_rounds/202605290018/202605290018_changelist.src.lock.md` | 38 | no date line | "Phase: IMPL" | RouterKind, StoreValues, ResourceArena, Scheduler Drop | batch 19 |
| `mock/design_rounds/202605290018/202605290018_topic.resource-arena.md` | 34 | 2026-05-29 | "**Phase:** TOPIC" | resource arena first data-plane slice | batch 19 |
| `mock/design_rounds/202605290049/202605290049_changelist.doc.lock.md` | 7 | no date line | "Phase: DOC" | no template changes, B2a fixups | batch 19 |
| `mock/design_rounds/202605290049/202605290049_changelist.src.lock.md` | 12 | no date line | "Phase: IMPL" | remove hollow retention test | batch 19 |
| `mock/design_rounds/202605290049/202605290049_topic.b2a-review-fixups.md` | 19 | 2026-05-29 | "**Phase:** TOPIC" | B2a review fixups, hollow test deleted | batch 19 |
| `mock/design_rounds/202605290111/202605290111_changelist.doc.lock.md` | 11 | no date line | "Phase: DOC" | EngineCtx note in DESIGN dispatch section | batch 19 |
| `mock/design_rounds/202605290111/202605290111_changelist.src.lock.md` | 24 | no date line | "Phase: IMPL" | EngineCtx, Selector, Project, projected Context | batch 19 |
| `mock/design_rounds/202605290111/202605290111_topic.per-wu-context.md` | 29 | 2026-05-29 | "**Phase:** TOPIC" | per-WU projected Context, B3, Selector/Project witnesses | batch 19 |
| `mock/design_rounds/202605290143/202605290143_changelist.doc.lock.md` | 7 | no date line | "Phase: DOC" | no template changes, bundle-tie fixup | batch 19 |
| `mock/design_rounds/202605290143/202605290143_changelist.src.lock.md` | 18 | no date line | "Phase: IMPL" | EngineCtx constructor private, compile-fail check | batch 19 |
| `mock/design_rounds/202605290143/202605290143_topic.b3-bundle-tie-fixup.md` | 22 | 2026-05-29 | "**Phase:** TOPIC" | tie Context bundle types to access sets | batch 19 |
| `mock/design_rounds/202605290203/202605290203_changelist.doc.lock.md` | 39 | 2026-05-29 | "**Phase:** DOC" | no design edits, arvo to_raw/from_raw confirmed | batch 19 |
| `mock/design_rounds/202605290203/202605290203_changelist.src.lock.md` | 37 | 2026-05-29 | "**Phase:** SRC" | index/from_index on engine ids, transmute_copy removed | batch 19 |
| `mock/design_rounds/202605290203/202605290203_topic.plan-id-typed-index.md` | 56 | 2026-05-29 | "**Phase:** TOPIC" | typed id-to-index accessors replace transmute_copy | batch 19 |
| `mock/design_rounds/202605290214/202605290214_changelist.doc.lock.md` | 13 | 2026-05-29 | "**Phase:** DOC" | no design edits, test coverage only | batch 19 |
| `mock/design_rounds/202605290214/202605290214_changelist.src.lock.md` | 10 | 2026-05-29 | "**Phase:** SRC" | engine id index round-trip tests | batch 19 |
| `mock/design_rounds/202605290214/202605290214_topic.engine-id-index-roundtrip-test.md` | 26 | 2026-05-29 | "**Phase:** TOPIC" | round-trip test for id index accessors | batch 19 |
| `mock/design_rounds/202605290312-abandoned/202605290312_changelist.doc.lock.md` | 18 | 2026-05-29 | "**Phase:** DOC" abandoned (directory name) | DependencyGraph to CsrBidirectional adapter, no doc edits | batch 19 |
| `mock/design_rounds/202605290312-abandoned/202605290312_changelist.src.md` | 20 | 2026-05-29 | "**Phase:** SRC" abandoned (directory name) | arvo-tensor dep, to_csr_bidirectional adapter | batch 19 |
| `mock/design_rounds/202606191400/202606191400_changelist.src.lock.md` | 37 | no date line | "**Phase:** IMPL"; name carries lock | src changelist: CtxFor alias, four fold traits | batch 22 |
| `mock/design_rounds/202606191400/202606191400_topic.ctxfor-computed-ctx.md` | 62 | 2026-06-19 | unstated, topic | topic: CtxFor computed per-WorkUnit Context type | batch 22 |
| `mock/design_rounds/202606191500/202606191500_changelist.doc.lock.md` | 11 | no date line | "**Phase:** DOC"; name carries lock | doc changelist: units-cap guard sentence | batch 22 |
| `mock/design_rounds/202606191500/202606191500_changelist.src.lock.md` | 24 | no date line | "**Phase:** IMPL"; name carries lock | src changelist: compile-time Units-cap assert | batch 22 |
| `mock/design_rounds/202606191500/202606191500_topic.units-cap-guard.md` | 31 | 2026-06-19 | unstated, topic | topic: Units-cap guard for run_parallel | batch 22 |
| `mock/design_rounds/202606191900/202606191900_changelist.doc.lock.md` | 9 | no date line | "**Phase:** DOC"; name carries lock | doc changelist: R2a keystone, no template change | batch 22 |
| `mock/design_rounds/202606191900/202606191900_changelist.src.lock.md` | 15 | no date line | "**Phase:** IMPL"; name carries lock | src changelist: nine-resource CtxFor keystone test | batch 22 |
| `mock/design_rounds/202606191900/202606191900_topic.p1-r2a-keystone.md` | 28 | 2026-06-19 | unstated, topic | topic: nine-resource CtxFor through OnMeta ScheduleEnd | batch 22 |
| `mock/design_rounds/202606192000/202606192000_changelist.doc.lock.md` | 10 | no date line | "**Phase:** DOC"; name carries lock | doc changelist: last_record_count note | batch 22 |
| `mock/design_rounds/202606192000/202606192000_changelist.src.lock.md` | 22 | no date line | "**Phase:** IMPL"; name carries lock | src changelist: last_record_count in SchedulerMetrics | batch 22 |
| `mock/design_rounds/202606192000/202606192000_topic.p1-r3c-throughput.md` | 25 | 2026-06-19 | unstated, topic | topic: throughput axis via last_record_count | batch 22 |
| `mock/design_rounds/202606192100/202606192100_changelist.doc.lock.md` | 7 | no date line | "**Phase:** DOC"; name carries lock | doc changelist: unused feature gate, no change | batch 22 |
| `mock/design_rounds/202606192100/202606192100_changelist.src.lock.md` | 9 | no date line | "**Phase:** IMPL"; name carries lock | src changelist: remove unused feature gate | batch 22 |
| `mock/design_rounds/202606192100/202606192100_topic.api-unused-feature-gate.md` | 18 | 2026-06-19 | unstated, topic | topic: api unused feature gate removal | batch 22 |
| `mock/design_rounds/202606192200/202606192200_changelist.doc.lock.md` | 7 | no date line | "**Phase:** DOC"; name carries lock | doc changelist: Executor sealed allow, no change | batch 22 |
| `mock/design_rounds/202606192200/202606192200_changelist.src.lock.md` | 9 | no date line | "**Phase:** IMPL"; name carries lock | src changelist: allow private_bounds on Executor | batch 22 |
| `mock/design_rounds/202606192200/202606192200_topic.api-sealed-executor.md` | 20 | 2026-06-19 | unstated, topic | topic: api Executor sealed private_bounds allow | batch 22 |
| `mock/design_rounds/202606192301/202606192301_changelist.doc.lock.md` | 22 | 2026-06-19 | "**Phase:** DOC"; name carries lock | doc changelist: per-phase EMA note | batch 22 |
| `mock/design_rounds/202606192301/202606192301_changelist.src.lock.md` | 47 | 2026-06-19 | "**Phase:** SRC"; name carries lock | src changelist: phase_ema field and fold | batch 22 |
| `mock/design_rounds/202606192301/202606192301_topic.phase-ema-adapt-axis.md` | 66 | 2026-06-19 | "**Phase:** TOPIC" | topic: per-phase duration EMA adapt axis | batch 22 |
| `mock/design_rounds/202606192330/202606192330_changelist.doc.lock.md` | 21 | 2026-06-19 | "**Phase:** DOC"; name carries lock | doc changelist: adapt non-degradation invariant | batch 22 |
| `mock/design_rounds/202606192330/202606192330_changelist.src.lock.md` | 35 | 2026-06-19 | "**Phase:** SRC"; name carries lock | src changelist: adapt performance contract tests | batch 22 |
| `mock/design_rounds/202606192330/202606192330_topic.adapt-performance-contracts.md` | 80 | 2026-06-19 | "**Phase:** TOPIC" | topic: adapt performance contracts, seven cases | batch 22 |
| `mock/design_rounds/202606192345/202606192345_changelist.doc.lock.md` | 10 | 2026-06-19 | "**Phase:** DOC"; name carries lock | doc changelist: per-phase EMA per-frame, no edit | batch 22 |
| `mock/design_rounds/202606192345/202606192345_changelist.src.lock.md` | 51 | 2026-06-19 | "**Phase:** SRC"; name carries lock | src changelist: phase_accum per-frame fold | batch 22 |
| `mock/design_rounds/202606192345/202606192345_topic.phase-ema-per-frame-fix.md` | 49 | 2026-06-19 | "**Phase:** TOPIC" | topic: per-phase EMA per-frame fold fix | batch 22 |
| `mock/design_rounds/202606200000/202606200000_changelist.doc.lock.md` | 9 | no date line | "**Phase:** DOC"; name carries lock | doc changelist: change_seen_count sentence | batch 22 |
| `mock/design_rounds/202606200000/202606200000_changelist.src.lock.md` | 26 | no date line | "**Phase:** IMPL"; name carries lock | src changelist: change_seen_count in SchedulerMetrics | batch 22 |
| `mock/design_rounds/202606200000/202606200000_topic.p1-r3b-change-class.md` | 29 | 2026-06-20 | unstated, topic | topic: change_class numerator via store_dirty | batch 22 |
| `mock/design_rounds/202606200100/202606200100_changelist.doc.lock.md` | 8 | no date line | "**Phase:** DOC"; name carries lock | doc changelist: change_seen_count qualifier, backlog | batch 22 |
| `mock/design_rounds/202606200100/202606200100_changelist.src.lock.md` | 9 | no date line | "**Phase:** IMPL"; name carries lock | src changelist: R3b post-review test note | batch 22 |
| `mock/design_rounds/202606200100/202606200100_topic.p1-r3b-fixes.md` | 13 | 2026-06-20 | unstated, topic | topic: R3b post-review fixes | batch 22 |
| `mock/design_rounds/202606200200/202606200200_changelist.doc.lock.md` | 9 | no date line | "**Phase:** DOC"; name carries lock | doc changelist: change_seen_count all run paths | batch 22 |
| `mock/design_rounds/202606200200/202606200200_changelist.src.lock.md` | 18 | no date line | "**Phase:** IMPL"; name carries lock | src changelist: change_seen_count parallel and fused | batch 22 |
| `mock/design_rounds/202606200200/202606200200_topic.r3b-parallel-fused.md` | 18 | 2026-06-20 | unstated, topic | topic: change_seen_count in parallel and fused | batch 22 |
| `mock/design_rounds/202606200300/202606200300_changelist.doc.lock.md` | 10 | no date line | "**Phase:** DOC"; name carries lock | doc changelist: accumulator over-capacity panics | batch 22 |
| `mock/design_rounds/202606200300/202606200300_changelist.src.lock.md` | 18 | no date line | "**Phase:** IMPL"; name carries lock | src changelist: resolve_append asserts instead of saturating | batch 22 |
| `mock/design_rounds/202606200300/202606200300_topic.accum-overcap-hard-error.md` | 35 | 2026-06-20 | unstated, topic | topic: accumulator over-capacity is hard error | batch 22 |
| `mock/design_rounds/202606200400/202606200400_changelist.doc.lock.md` | 8 | no date line | "**Phase:** DOC"; name carries lock | doc changelist: PR 145 review fixes, empty | batch 22 |
| `mock/design_rounds/202606200400/202606200400_changelist.src.lock.md` | 14 | no date line | "**Phase:** IMPL"; name carries lock | src changelist: stale saturation comments | batch 22 |
| `mock/design_rounds/202606200400/202606200400_topic.r3b-overcap-review-fixes.md` | 27 | 2026-06-20 | unstated, topic | topic: PR 145 review fixes, audit-trail correction | batch 22 |
| `mock/design_rounds/202606201200/202606201200_changelist.doc.lock.md` | 22 | 2026-06-19 | "**Phase:** DOC"; name carries lock | doc changelist: select_adapt_config trigger note | batch 22 |
| `mock/design_rounds/202606201200/202606201200_changelist.src.lock.md` | 38 | 2026-06-19 | "**Phase:** SRC"; name carries lock | src changelist: select_adapt_config, adapt_reconfigure | batch 22 |
| `mock/design_rounds/202606201200/202606201200_topic.select-adapt-config.md` | 57 | 2026-06-19 | "**Phase:** TOPIC" | topic: select_adapt_config decision logic | batch 22 |
| `mock/design_rounds/202606201500/202606201500_changelist.doc.lock.md` | 12 | 2026-06-19 | "**Phase:** DOC"; name carries lock | doc changelist: morsel slice 1 rename, no template change | batch 22 |
| `mock/design_rounds/202606201500/202606201500_changelist.src.lock.md` | 32 | 2026-06-19 | "**Phase:** SRC"; name carries lock | src changelist: morsel_sizes to morsel_windows rename | batch 22 |
| `mock/design_rounds/202606201500/202606201500_topic.per-fiber-morsel-s1-rename.md` | 46 | 2026-06-19 | "**Phase:** TOPIC" | topic: per-fiber morsel slice 1 rename | batch 22 |
| `mock/design_rounds/202606201600/202606201600_changelist.doc.lock.md` | 9 | 2026-06-19 | "**Phase:** DOC"; name carries lock | doc changelist: slice 1 review fixes, no change | batch 22 |
| `mock/design_rounds/202606201600/202606201600_changelist.src.lock.md` | 20 | 2026-06-19 | "**Phase:** SRC"; name carries lock | src changelist: MorselWindows variant and docs | batch 22 |
| `mock/design_rounds/202606201600/202606201600_topic.s1-rename-review-fixes.md` | 30 | 2026-06-19 | "**Phase:** TOPIC" | topic: slice 1 rename review fixes | batch 22 |
| `mock/design_rounds/202606201800/202606201800_changelist.doc.lock.md` | 8 | 2026-06-19 | "**Phase:** DOC"; name carries lock | doc changelist: slice A1, no template change | batch 22 |
| `mock/design_rounds/202606201800/202606201800_changelist.src.lock.md` | 28 | 2026-06-19 | "**Phase:** SRC"; name carries lock | src changelist: FiberDispatch morsel_size, fiber_plan_idx | batch 22 |
| `mock/design_rounds/202606201800/202606201800_topic.per-fiber-morsel-a2-descriptor.md` | 34 | 2026-06-19 | "**Phase:** TOPIC" | topic: FiberDispatch per-fiber size field | batch 22 |
| `mock/design_rounds/202606201900/202606201900_changelist.doc.lock.md` | 7 | 2026-06-19 | "**Phase:** DOC"; name carries lock | doc changelist: fiber_plan_idx test, no change | batch 22 |
| `mock/design_rounds/202606201900/202606201900_changelist.src.lock.md` | 15 | 2026-06-19 | "**Phase:** SRC"; name carries lock | src changelist: fiber_plan_idx catalogue test | batch 22 |
| `mock/design_rounds/202606201900/202606201900_topic.a1-fiber-plan-idx-test.md` | 25 | 2026-06-19 | "**Phase:** TOPIC" | topic: fiber_plan_idx CSR-vs-dispatch mapping | batch 22 |
| `mock/design_rounds/202606202000/202606202000_changelist.doc.lock.md` | 8 | 2026-06-19 | "**Phase:** DOC"; name carries lock | doc changelist: asm-emission backlog note | batch 22 |
| `mock/design_rounds/202606202000/202606202000_changelist.src.lock.md` | 9 | 2026-06-19 | "**Phase:** SRC"; name carries lock | src changelist: no source change | batch 22 |
| `mock/design_rounds/202606202000/202606202000_topic.asm-emission-contract-tests.md` | 30 | 2026-06-19 | "**Phase:** TOPIC" | topic: ASM-emission contract tests backlog | batch 22 |
| `mock/design_rounds/202606202200/202606202200_changelist.doc.lock.md` | 7 | 2026-06-19 | "**Phase:** DOC"; name carries lock | doc changelist: A2b-1, no template change | batch 22 |
| `mock/design_rounds/202606202200/202606202200_changelist.src.lock.md` | 22 | 2026-06-19 | "**Phase:** SRC"; name carries lock | src changelist: run_one_trunk_windowed | batch 22 |
| `mock/design_rounds/202606202200/202606202200_topic.per-fiber-morsel-a2b1-windowed-trunk.md` | 31 | 2026-06-19 | "**Phase:** TOPIC" | topic: windowed per-trunk dispatch entry | batch 22 |
| `mock/design_rounds/202606202300/202606202300_changelist.doc.lock.md` | 7 | 2026-06-19 | "**Phase:** DOC"; name carries lock | doc changelist: A2b-1 review fixes | batch 22 |
| `mock/design_rounds/202606202300/202606202300_changelist.src.lock.md` | 19 | 2026-06-19 | "**Phase:** SRC"; name carries lock | src changelist: A2b-1 test doc, total==0 note | batch 22 |
| `mock/design_rounds/202606202300/202606202300_topic.a2b1-review-fixes.md` | 30 | 2026-06-19 | "**Phase:** TOPIC" | topic: A2b-1 review fixes | batch 22 |
| `mock/design_rounds/202606202500/202606202500_changelist.doc.lock.md` | 7 | 2026-06-19 | "**Phase:** DOC"; name carries lock | doc changelist: A3a, no template change | batch 22 |
| `mock/design_rounds/202606202500/202606202500_changelist.src.lock.md` | 22 | 2026-06-19 | "**Phase:** SRC"; name carries lock | src changelist: StoreElemBytes and StoreSizes | batch 22 |
| `mock/design_rounds/202606202500/202606202500_topic.a3a-store-size-collection.md` | 34 | 2026-06-19 | "**Phase:** TOPIC" | topic: per-store size collection | batch 22 |
| `mock/design_rounds/202606202600/202606202600_changelist.doc.lock.md` | 7 | 2026-06-19 | "**Phase:** DOC"; name carries lock | doc changelist: A3a import cleanup | batch 22 |
| `mock/design_rounds/202606202600/202606202600_changelist.src.lock.md` | 9 | 2026-06-19 | "**Phase:** SRC"; name carries lock | src changelist: drop unused test imports | batch 22 |
| `mock/design_rounds/202606202600/202606202600_topic.a3a-unused-imports.md` | 20 | 2026-06-19 | "**Phase:** TOPIC" | topic: A3a unused-import cleanup | batch 22 |
| `mock/design_rounds/202606202700/202606202700_changelist.doc.lock.md` | 7 | 2026-06-19 | "**Phase:** DOC"; name carries lock | doc changelist: A3b, no template change | batch 22 |
| `mock/design_rounds/202606202700/202606202700_changelist.src.lock.md` | 46 | 2026-06-19 | "**Phase:** SRC"; name carries lock | src changelist: L1 window formula in compute_plan | batch 22 |
| `mock/design_rounds/202606202700/202606202700_topic.a3b-l1-morsel-formula.md` | 32 | 2026-06-19 | "**Phase:** TOPIC" | topic: L1 window formula wired into plan | batch 22 |
| `mock/design_rounds/202606210200/202606210200_changelist.doc.lock.md` | 7 | 2026-06-20 | "**Phase:** DOC"; name carries lock | doc changelist: footprint traits, no template change | batch 22 |
| `mock/design_rounds/202606210200/202606210200_changelist.src.lock.md` | 25 | 2026-06-20 | "**Phase:** SRC"; name carries lock | src changelist: CollectionBytes, ResourceFootprint | batch 22 |
| `mock/design_rounds/202606210200/202606210200_topic.a3-redo-1-footprint-traits.md` | 35 | 2026-06-20 | "**Phase:** TOPIC" | topic: CollectionBytes and ResourceFootprint traits | batch 22 |
| `mock/design_rounds/202606210300/202606210300_changelist.doc.lock.md` | 21 | 2026-06-20 | "**Phase:** DOC"; name carries lock | doc changelist: api-macros crate, crate-count claims | batch 22 |
| `mock/design_rounds/202606210300/202606210300_changelist.src.lock.md` | 27 | 2026-06-20 | "**Phase:** SRC"; name carries lock | src changelist: api-macros crate, derive | batch 22 |
| `mock/design_rounds/202606210300/202606210300_topic.api-macros-resourcefootprint-derive.md` | 47 | 2026-06-20 | "**Phase:** TOPIC" | topic: api-macros proc-macro crate, ResourceFootprint derive | batch 22 |
| `mock/design_rounds/202607161000/202606210600_topic.bench-decision.md` | 57 | 2026-06-20 | "**Phase:** TOPIC"; "## Status: PROVISIONAL" | topic: bench-driven storage decision, provisional | batch 22 |
| `mock/design_rounds/202607161000/202606210600_topic.bench-run2-resolution.md` | 73 | 2026-07-02 | "**Phase:** TOPIC" | topic: bench run-2 confirms layout | batch 22 |
| `mock/design_rounds/202607161000/202606210600_topic.hybrid-decision.md` | 74 | 2026-07-02 | "**Phase:** TOPIC" | topic: op picks hybrid, value model settled | batch 22 |
| `mock/design_rounds/202607161000/202606210600_topic.r5-underspecification.md` | 46 | 2026-06-19 | "**Phase:** TOPIC" | topic: where R5 is light, amendment | batch 22 |
| `mock/design_rounds/202607161000/202606210600_topic.resource-handle-storage-model.md` | 58 | 2026-06-19 | "**Phase:** TOPIC" | topic: Resource is a handle model | batch 22 |
| `mock/design_rounds/202607161000/202606210600_topic.storage-model-pressure-test.md` | 47 | 2026-06-19 | "**Phase:** TOPIC" | topic: pressure-test the storage model | batch 22 |
| `mock/design_rounds/202607161000/202606210600_topic.v1-v4-tiebreak.md` | 19 | 2026-06-20 | "**Phase:** TOPIC" | topic: V1-vs-V4 tiebreak rule | batch 22 |
| `mock/design_rounds/202607161000/202606210600_topic.v4-plugin-interop-axis.md` | 40 | 2026-06-20 | "**Phase:** TOPIC" | topic: V4 plugin-interop axis | batch 22 |
| `mock/design_rounds/202607161000/202607161000_changelist.doc.lock.md` | 75 | 2026-07-16 | "**Phase:** DOC (this changelist opens it)"; name carries lock | doc changelist: storage model bench-settled, hybrid | batch 22 |
| `mock/design_rounds/202607161000/202607161030_changelist.src.lock.md` | 88 | 2026-07-16 | "**Phase:** IMPL (this changelist opens it)"; name carries lock | src changelist: erased-addressed blob, resource snapshot | batch 22 |
| `mock/design_rounds/202607161200/202607161200_changelist.doc.lock.md` | 16 | 2026-07-16 | "**Phase:** DOC (this changelist opens it; no doc-template edits in scope)"; name carries lock | doc changelist: post-review storage fixes | batch 22 |
| `mock/design_rounds/202607161200/202607161210_changelist.src.lock.md` | 23 | 2026-07-16 | "**Phase:** IMPL (this changelist opens it)"; name carries lock | src changelist: post-review comment clarifications | batch 22 |
| `mock/design_rounds/202607161410/202607161400_topic.a3b-l1-morsel-formula.md` | 63 | 2026-07-16 | "**Phase:** TOPIC" | topic: A3b per-fiber L1 morsel-window formula | batch 22 |
| `mock/design_rounds/202607161410/202607161410_changelist.doc.lock.md` | 17 | 2026-07-16 | "**Phase:** DOC (this changelist opens it)"; name carries lock | doc changelist: A3b budget source | batch 22 |
| `mock/design_rounds/202607161410/202607161420_changelist.src.lock.md` | 72 | 2026-07-16 | "**Phase:** IMPL (this changelist opens it)"; name carries lock | src changelist: A3b window formula, MorselBudget | batch 22 |
| `mock/design_rounds/202607161600/202607161600_changelist.doc.lock.md` | 16 | 2026-07-16 | "**Phase:** DOC (this changelist opens it; no doc-template edits in scope)"; name carries lock | doc changelist: A3b post-review fixes | batch 22 |
| `mock/design_rounds/202607161600/202607161610_changelist.src.lock.md` | 20 | 2026-07-16 | "**Phase:** IMPL (this changelist opens it)"; name carries lock | src changelist: A3b rustdoc, clamp guard | batch 22 |
| `mock/design_rounds/202607161810/202607161800_topic.a2b-dispatch-loop-inversion.md` | 134 | 2026-07-16 | "**Phase:** TOPIC" | topic: A2b dispatch loop inversion | batch 22 |
| `mock/design_rounds/202607161810/202607161810_changelist.doc.lock.md` | 32 | 2026-07-16 | "**Phase:** DOC"; name carries lock | doc changelist: A2b dispatch loop inversion | batch 22 |
| `mock/design_rounds/202607161810/202607161820_changelist.src.lock.md` | 32 | 2026-07-16 | "**Phase:** IMPL"; name carries lock | src changelist: A2b run inversion, asm fixtures | batch 22 |
| `mock/design_rounds/202607161910/202607161900_topic.a2b-post-review-fixes.md` | 18 | 2026-07-16 | "**Phase:** TOPIC" | topic: A2b post-review doc fixes | batch 22 |
| `mock/design_rounds/202607161910/202607161910_changelist.doc.lock.md` | 12 | 2026-07-16 | "**Phase:** DOC"; name carries lock | doc changelist: A2b post-review doc fixes | batch 22 |
| `mock/design_rounds/202607161910/202607161920_changelist.src.lock.md` | 10 | 2026-07-16 | "**Phase:** IMPL"; name carries lock | src changelist: FIXTURE_NAMES comment counts | batch 22 |
| `mock/design_rounds/202607171807/202607171733_topic.plannable-declaration-split.md` | 127 | 2026-07-17 | "**Phase:** TOPIC" | topic: Plannable declaration split from WorkUnit | batch 22 |
| `mock/design_rounds/202607171807/202607171807_changelist.doc.lock.md` | 55 | 2026-07-17 | "**Phase:** DOC"; name carries lock | doc changelist: Plannable and HintExt documentation | batch 22 |
| `mock/design_rounds/202607171807/202607171807_changelist.src.lock.md` | 50 | 2026-07-17 | "**Phase:** IMPL"; name carries lock | src changelist: Plannable trait, HintExt slot | batch 22 |
| `mock/design_rounds/202607171824/202607171824_changelist.doc.lock.md` | 19 | 2026-07-17 | "**Phase:** DOC"; name carries lock | doc changelist: soften engine-reads-fourth-axis claim | batch 22 |
| `mock/design_rounds/202607171824/202607171824_changelist.src.lock.md` | 30 | 2026-07-17 | "**Phase:** IMPL"; name carries lock | src changelist: Plannable blanket test, doc note | batch 22 |
| `mock/design_rounds/202607171824/202607171824_topic.plannable-post-review-fixes.md` | 34 | 2026-07-17 | "**Phase:** TOPIC" | topic: plannable post-review fixes | batch 22 |
| `mock/design_rounds/202607190930/202607190910_topic.a4-parallel-per-fiber-windows.md` | 81 | 2026-07-19 | "**Phase:** TOPIC" | topic: per-fiber windows on parallel path | batch 22 |
| `mock/design_rounds/202607190930/202607190930_changelist.doc.lock.md` | 47 | 2026-07-19 | "**Phase:** DOC"; name carries lock | doc changelist: parallel dispatch windowing paragraph | batch 22 |
| `mock/design_rounds/202607190930/202607190940_changelist.src.lock.md` | 91 | 2026-07-19 | "**Phase:** SRC"; name carries lock | src changelist: run_core_phase fiber-outer branch | batch 22 |
| `mock/design_rounds/202607191110/202607191100_topic.fiber-descriptor-coverage-invariant.md` | 58 | 2026-07-19 | "**Phase:** TOPIC" | topic: fiber-descriptor coverage invariant | batch 22 |
| `mock/design_rounds/202607191110/202607191110_changelist.doc.lock.md` | 36 | 2026-07-19 | "**Phase:** DOC"; name carries lock | doc changelist: state descriptor coverage invariant | batch 22 |
| `mock/design_rounds/202607191110/202607191120_changelist.src.lock.md` | 58 | 2026-07-19 | "**Phase:** SRC"; name carries lock | src changelist: coverage catalogue, parity assert | batch 22 |
| `mock/design_rounds/202607192010/202607192000_topic.reachable-bodies-contradicting-contracts.md` | 78 | 2026-07-19 | "**Phase:** TOPIC" | topic: reachable bodies contradicting contracts | batch 22 |
| `mock/design_rounds/202607192010/202607192010_changelist.doc.lock.md` | 61 | 2026-07-19 | "**Phase:** DOC"; name carries lock | doc changelist: deallocate gains alignment | batch 22 |
| `mock/design_rounds/202607192010/202607192030_changelist.src.lock.md` | 129 | 2026-07-19 | "**Phase:** SRC"; name carries lock | src changelist: spawn spawns, deallocate takes align | batch 22 |
| `mock/design_rounds/202607192110/202607192100_topic.n-way-record-split-is-not-convergence.md` | 71 | 2026-07-19 | "**Phase:** TOPIC" | topic: N-way record split is not convergence | batch 22 |
| `mock/design_rounds/202607192110/202607192110_changelist.doc.lock.md` | 81 | 2026-07-19 | "**Phase:** DOC"; name carries lock | doc changelist: stop calling N-way split convergence | batch 22 |
| `mock/design_rounds/202607192110/202607192120_changelist.src.lock.md` | 51 | 2026-07-19 | "**Phase:** SRC"; name carries lock | src changelist: five-producer test, core surplus | batch 22 |
| `mock/design_rounds/202607192310/202607192300_topic.accumulator-region-sizing-under-trunk-rank.md` | 49 | 2026-07-19 | "**Phase:** TOPIC" | topic: accumulator region sizing under trunk rank | batch 22 |
| `mock/design_rounds/202607192310/202607192310_changelist.doc.lock.md` | 23 | 2026-07-19 | "**Phase:** DOC"; name carries lock | doc changelist: none, waits on experiment | batch 22 |
| `mock/design_rounds/202607192310/202607192320_changelist.src.lock.md` | 61 | 2026-07-19 | "**Phase:** SRC"; name carries lock | src changelist: accumulator in two-trunk phase probe | batch 22 |
| `mock/design_rounds/202607192410/202607192400_topic.delete-the-n-way-record-split.md` | 50 | 2026-07-19 | "**Phase:** TOPIC" | topic: delete the N-way record split | batch 22 |
| `mock/design_rounds/202607192410/202607192410_changelist.doc.lock.md` | 27 | 2026-07-19 | "**Phase:** DOC"; name carries lock | doc changelist: docs already describe result | batch 22 |
| `mock/design_rounds/202607192410/202607192420_changelist.src.lock.md` | 78 | 2026-07-19 | "**Phase:** SRC"; name carries lock | src changelist: remove N-way branch and carve-out | batch 22 |
| `mock/design_rounds/202607192510/202607192500_topic.delete-the-orphaned-thread-pool.md` | 45 | 2026-07-19 | "**Phase:** TOPIC" | topic: delete the orphaned ThreadPool | batch 22 |
| `mock/design_rounds/202607192510/202607192510_changelist.doc.lock.md` | 19 | 2026-07-19 | "**Phase:** DOC"; name carries lock | doc changelist: nothing to change | batch 22 |
| `mock/design_rounds/202607192510/202607192520_changelist.src.lock.md` | 58 | 2026-07-19 | "**Phase:** SRC"; name carries lock | src changelist: remove thread/pool.rs | batch 22 |
| `mock/design_rounds/202607192610/202607192600_topic.one-definition-for-the-accumulator-slice.md` | 48 | 2026-07-19 | "**Phase:** TOPIC" | topic: one definition for accumulator slice | batch 22 |
| `mock/design_rounds/202607192610/202607192610_changelist.doc.lock.md` | 26 | 2026-07-19 | "**Phase:** DOC"; name carries lock | doc changelist: name accumulator region invariant | batch 22 |
| `mock/design_rounds/202607192610/202607192620_changelist.src.lock.md` | 61 | 2026-07-19 | "**Phase:** SRC"; name carries lock | src changelist: accum_slice helper, comment fix | batch 22 |
| `mock/design_rounds/202607192710/202607192700_topic.replace-resource-install-arm.md` | 54 | 2026-07-19 | "**Phase:** TOPIC" | topic: install arm for replace_resource | batch 22 |
| `mock/design_rounds/202607192710/202607192710_changelist.doc.lock.md` | 12 | 2026-07-19 | "**Phase:** DOC"; name carries lock | doc changelist: none until shape known | batch 22 |
| `mock/design_rounds/202607192710/202607192720_changelist.src.lock.md` | 71 | 2026-07-19 | "**Phase:** SRC"; name carries lock | src changelist: replace_value install, value-typed signature | batch 22 |
| `mock/design_rounds/202607192810/202607192800_topic.retire-the-fn-pointer-spawn.md` | 54 | 2026-07-19 | "**Phase:** TOPIC" | topic: retire the function-pointer spawn | batch 22 |
| `mock/design_rounds/202607192810/202607192810_changelist.doc.lock.md` | 31 | 2026-07-19 | "**Phase:** DOC"; name carries lock | doc changelist: close POSIX cast backlog note | batch 22 |
| `mock/design_rounds/202607192810/202607192820_changelist.src.lock.md` | 55 | 2026-07-19 | "**Phase:** SRC"; name carries lock | src changelist: delete spawn_fn and trampoline | batch 22 |
| `mock/design_rounds/202607192910/202607192900_topic.delete-superseded-dispatch-and-barrier.md` | 60 | 2026-07-19 | "**Phase:** TOPIC" | topic: delete superseded dispatch and barrier | batch 22 |
| `mock/design_rounds/202607192910/202607192910_changelist.doc.lock.md` | 32 | 2026-07-19 | "**Phase:** DOC"; name carries lock | doc changelist: remove nested-carrier paragraphs | batch 22 |
| `mock/design_rounds/202607192910/202607192920_changelist.src.lock.md` | 67 | 2026-07-19 | "**Phase:** SRC"; name carries lock | src changelist: delete phase_run, phase_barrier family | batch 22 |
| `mock/design_rounds/202607193110/202607193100_topic.revert-the-blob-install.md` | 55 | 2026-07-19 | "**Phase:** TOPIC" | topic: revert the blob install | batch 22 |
| `mock/design_rounds/202607193110/202607193110_changelist.doc.lock.md` | 32 | 2026-07-19 | "**Phase:** DOC"; name carries lock | doc changelist: state what the swap must be | batch 22 |
| `mock/design_rounds/202607193110/202607193120_changelist.src.lock.md` | 54 | 2026-07-19 | "**Phase:** SRC"; name carries lock | src changelist: remove install, catalogue case | batch 22 |
| `mock/design_rounds/202607200100_topic.b1-cap-lifting-completion.md` | 53 | 2026-07-20 | "**Phase:** TOPIC" | topic: B1 cap-lifting completion onto PlanDims | batch 22 |
| `mock/design_rounds/202607200900/202607200500_topic.replaceable-swap-semantics.md` | 55 | 2026-07-19 | unstated, topic | topic: Replaceable swap semantics implemented | batch 22 |
| `mock/design_rounds/202607200900/202607200900_changelist.doc.lock.md` | 90 | 2026-07-19 | unstated, doc changelist | doc changelist: swap semantics under blob layout | batch 22 |
| `mock/design_rounds/202607200900/202607201000_changelist.src.lock.md` | 90 | 2026-07-19 | unstated, src changelist | src changelist: swap semantics implementation | batch 22 |
| `mock/design_rounds/202607201800/202607201700_topic.pre-freeze-evidence-followups.md` | 19 | 2026-07-19 | unstated, topic | topic: pre-freeze evidence follow-ups | batch 22 |
| `mock/design_rounds/202607201800/202607201800_changelist.doc.lock.md` | 10 | 2026-07-19 | unstated (doc changelist; name ends .lock.md) | no template edits; opens round to src slice | batch 23 |
| `mock/design_rounds/202607201800/202607201900_changelist.src.lock.md` | 20 | 2026-07-19 | unstated (src changelist; name ends .lock.md) | catalogue aliasing finding as ignored test | batch 23 |
| `mock/design_rounds/202607202310/202607202300_topic.wake-spin-preroll.md` | 39 | 2026-07-19 | unstated (topic) | wake spin pre-roll topic, deviation 3 | batch 23 |
| `mock/design_rounds/202607202310/202607202310_changelist.doc.lock.md` | 13 | 2026-07-19 | unstated (doc changelist; name ends .lock.md) | doc CL: no template changes, wake preroll | batch 23 |
| `mock/design_rounds/202607202310/202607202320_changelist.src.lock.md` | 54 | 2026-07-19 | unstated (src changelist; name ends .lock.md) | src CL: bounded spin before frame waits | batch 23 |
| `mock/design_rounds/202607202351/202607202350_topic.wake-budget-default-zero.md` | 16 | 2026-07-19 | unstated (topic) | flip wake spin budget default to measured winner | batch 23 |
| `mock/design_rounds/202607202351/202607202351_changelist.doc.lock.md` | 9 | 2026-07-19 | unstated (doc changelist; name ends .lock.md) | doc CL: no design doc edit owed | batch 23 |
| `mock/design_rounds/202607202351/202607202352_changelist.src.lock.md` | 16 | 2026-07-19 | unstated (src changelist; name ends .lock.md) | src CL: WAKE_SPIN_BUDGET default 128 to 0 | batch 23 |
| `mock/design_rounds/202607210111/202607210110_topic.wake-doc-correction.md` | 15 | 2026-07-19 | unstated (topic) | wake budget doc comment correction topic | batch 23 |
| `mock/design_rounds/202607210111/202607210111_changelist.doc.lock.md` | 8 | 2026-07-19 | unstated (doc changelist; name ends .lock.md) | doc CL: no template surface cites margins | batch 23 |
| `mock/design_rounds/202607210111/202607210112_changelist.src.lock.md` | 16 | 2026-07-19 | unstated (src changelist; name ends .lock.md) | src CL: doc comment follows re-examination | batch 23 |
| `mock/design_rounds/202607241915/202607241915_changelist.doc.lock.md` | 53 | no date line | "**Phase:** DOC" | doc CL: new sym crate, str doc updates | batch 23 |
| `mock/design_rounds/202607241915/202607241915_changelist.src.lock.md` | 87 | no date line | "**Phase:** IMPL" | src CL: create sym crate, refactor str | batch 23 |
| `mock/design_rounds/202607241915/202607241915_topic.sym-generic-interned-identity.md` | 40 | 2026-07-24 | "**Phase:** TOPIC" | topic: generic interned-identity core, strings one domain | batch 23 |
| `mock/design_rounds/202607242000/202607242000_changelist.doc.lock.md` | 14 | no date line | "**Phase:** DOC" | doc CL: list sym in root design overview | batch 23 |
| `mock/design_rounds/202607242000/202607242000_changelist.src.lock.md` | 6 | no date line | "**Phase:** IMPL" | src CL: no source change, doc-only | batch 23 |
| `mock/design_rounds/202607242000/202607242000_topic.sym-in-design-overview.md` | 15 | 2026-07-24 | "**Phase:** TOPIC" | topic: add sym row to design overview | batch 23 |
| `mock/design_rounds/202607281547_topic.adopt-the-shared-hlist.md` | 71 | 2026-07-28 | "**Phase:** TOPIC" | topic: replace local cons-list with shared hlist | batch 23 |
| `mock/design_rounds/202608082215/202608082215_changelist.doc.lock.md` | 116 | no date line | "**Phase:** DOC" | doc CL: SymShape trait, generic kind width | batch 23 |
| `mock/design_rounds/202608082215/202608082215_changelist.src.lock.md` | 121 | no date line | "**Phase:** IMPL" | src CL: SymShape, Generator origin constructor | batch 23 |
| `mock/design_rounds/202608082215/202608082215_topic.sym-peer-collision-and-generic-kind.md` | 124 | no date line | "**Phase:** TOPIC" | topic: sym peer collision, fixed kind width | batch 23 |
| `mock/design_rounds/202608090130/202608090130_changelist.doc.lock.md` | 53 | no date line | "**Phase:** DOC" | doc CL: origin mechanism sharpened, shim removed | batch 23 |
| `mock/design_rounds/202608090130/202608090130_changelist.src.lock.md` | 93 | no date line | "**Phase:** IMPL" | src CL: const SymShape, SixteenMinters, Generator::at | batch 23 |
| `mock/design_rounds/202608090130/202608090130_topic.the-origin-mechanism-round-one-only-named.md` | 114 | no date line | "**Phase:** TOPIC" | topic: origin mechanism described but not built | batch 23 |
| `mock/design_rounds/202608090300/202608090300_changelist.doc.lock.md` | 35 | no date line | "**Phase:** DOC" | doc CL: WideKind shape, widths consumed | batch 23 |
| `mock/design_rounds/202608090300/202608090300_changelist.src.lock.md` | 69 | no date line | "**Phase:** IMPL" | src CL: WideKind layout, remove ORIGINS, laws | batch 23 |
| `mock/design_rounds/202608090300/202608090300_topic.the-kind-width-is-still-fixed.md` | 78 | no date line | "**Phase:** TOPIC" | topic: kind width still fixed, decorative widths | batch 23 |
| `mock/design_rounds/202608090500/202608090500_changelist.doc.lock.md` | 35 | no date line | "**Phase:** DOC" | doc CL: widths load-bearing, drifted signatures | batch 23 |
| `mock/design_rounds/202608090500/202608090500_changelist.src.lock.md` | 88 | no date line | "**Phase:** IMPL" | src CL: layout widths, law tying declarations | batch 23 |
| `mock/design_rounds/202608090500/202608090500_topic.assertions-that-describe-do-not-constrain.md` | 76 | no date line | "**Phase:** TOPIC" | topic: assertions describe rather than constrain | batch 23 |
| `mock/design_rounds/202608090700/202608090700_changelist.doc.lock.md` | 33 | no date line | "**Phase:** DOC" | doc CL: mask-derived width, Interner shape | batch 23 |
| `mock/design_rounds/202608090700/202608090700_changelist.src.lock.md` | 72 | no date line | "**Phase:** IMPL" | src CL: masks, Interner carries shape, rustfmt skip | batch 23 |
| `mock/design_rounds/202608090700/202608090700_topic.the-tautology-moved-down-a-rung-twice.md` | 85 | no date line | "**Phase:** TOPIC" | topic: tautology moved down a rung twice | batch 23 |
| `mock/design_rounds/202608091100/202608091100_changelist.doc.lock.md` | 72 | 2026-08-09 | "**Phase:** DOC" | doc CL: width from associated type, coverage contract | batch 23 |
| `mock/design_rounds/202608091100/202608091100_changelist.src.lock.md` | 165 | 2026-08-09 | "**Phase:** IMPL" | src CL: FieldWidth, span law, accessor suite | batch 23 |
| `mock/design_rounds/202608091100/202608091100_topic.widths-tied-to-the-mechanism-and-a-dead-gate.md` | 130 | 2026-08-09 | "**Phase:** TOPIC" | topic: widths tied to mechanism, dead gate | batch 23 |
| `mock/design_rounds/202608091600/202608091600_changelist.doc.lock.md` | 56 | 2026-08-09 | "**Phase:** DOC" | doc CL: credit round trip not type | batch 23 |
| `mock/design_rounds/202608091600/202608091600_changelist.src.lock.md` | 116 | 2026-08-09 | "**Phase:** IMPL" | src CL: backstop, derived test widths, fmt finding | batch 23 |
| `mock/design_rounds/202608091600/202608091600_topic.the-right-account-of-why-it-holds.md` | 84 | 2026-08-09 | "**Phase:** TOPIC" | topic: right account of why widths hold | batch 23 |
| `mock/design_rounds/202608091900/202608091900_changelist.doc.lock.md` | 12 | 2026-08-09 | "**Phase:** DOC" | doc CL: no document edit needed | batch 23 |
| `mock/design_rounds/202608091900/202608091900_changelist.src.lock.md` | 21 | 2026-08-09 | "**Phase:** IMPL" | src CL: chain renamed route | batch 23 |
| `mock/design_rounds/202608091900/202608091900_topic.a-dead-vocabulary-term-i-introduced.md` | 21 | 2026-08-09 | "**Phase:** TOPIC" | topic: dead vocabulary term introduced | batch 23 |
| `mock/design_rounds/202608092200/202608092200_changelist.doc.lock.md` | 67 | 2026-08-09 | "**Phase:** DOC" | doc CL: laws over behaviour, hardcoded 28 | batch 23 |
| `mock/design_rounds/202608092200/202608092200_changelist.src.lock.md` | 97 | 2026-08-09 | "**Phase:** IMPL" | src CL: disjointness law, origin fit, equality bounds | batch 23 |
| `mock/design_rounds/202608092200/202608092200_topic.the-collision-is-still-reachable.md` | 76 | 2026-08-09 | "**Phase:** TOPIC" | topic: collision reachable through extension point | batch 23 |
| `mock/design_rounds/202608100100/202608100100_changelist.doc.lock.md` | 10 | 2026-08-10 | "**Phase:** DOC" | doc CL: no document edit needed | batch 23 |
| `mock/design_rounds/202608100100/202608100100_changelist.src.lock.md` | 37 | 2026-08-10 | "**Phase:** IMPL" | src CL: allows at definition sites | batch 23 |
| `mock/design_rounds/202608100100/202608100100_topic.the-round-that-fixed-it-added-eight.md` | 27 | 2026-08-10 | "**Phase:** TOPIC" | topic: eight allow violations from prior round | batch 23 |
| `mock/design_rounds/202608100400/202608100400_changelist.doc.lock.md` | 21 | 2026-08-10 | "**Phase:** DOC" | doc CL: refusals are part of the contract | batch 23 |
| `mock/design_rounds/202608100400/202608100400_changelist.src.lock.md` | 73 | 2026-08-10 | "**Phase:** IMPL" | src CL: refusals module, committed negative test | batch 23 |
| `mock/design_rounds/202608100400/202608100400_topic.the-refusals-nothing-pins.md` | 58 | 2026-08-10 | "**Phase:** TOPIC" | topic: refusals nothing pins, hand checks | batch 23 |
| `mock/design_rounds/202608100700/202608100700_changelist.doc.lock.md` | 12 | 2026-08-10 | "**Phase:** DOC" | doc CL: no document edit needed | batch 23 |
| `mock/design_rounds/202608100700/202608100700_changelist.src.lock.md` | 20 | 2026-08-10 | "**Phase:** IMPL" | src CL: doctests moved to module | batch 23 |
| `mock/design_rounds/202608100700/202608100700_topic.two-types-that-exist-only-to-carry-docs.md` | 21 | 2026-08-10 | "**Phase:** TOPIC" | topic: two types existing only to carry docs | batch 23 |
| `mock/design_rounds/202608101000/202608101000_changelist.doc.lock.md` | 52 | 2026-08-10 | "**Phase:** DOC" | doc CL: origin as index, run derived | batch 23 |
| `mock/design_rounds/202608101000/202608101000_changelist.src.lock.md` | 86 | 2026-08-10 | "**Phase:** IMPL" | src CL: origin_index, base and ceiling derived | batch 23 |
| `mock/design_rounds/202608101000/202608101000_topic.disjointness-by-construction-not-by-law.md` | 73 | 2026-08-10 | "**Phase:** TOPIC" | topic: disjointness by construction not by law | batch 23 |
| `mock/design_rounds/202608101300/202608101300_changelist.doc.lock.md` | 9 | 2026-08-10 | "**Phase:** DOC" | doc CL: no document edit needed | batch 23 |
| `mock/design_rounds/202608101300/202608101300_changelist.src.lock.md` | 27 | 2026-08-10 | "**Phase:** IMPL" | src CL: allows on bodies not signatures | batch 23 |
| `mock/design_rounds/202608101300/202608101300_topic.three-allows-on-bodies-not-signatures.md` | 22 | 2026-08-10 | "**Phase:** TOPIC" | topic: allows on bodies not signatures | batch 23 |
| `mock/design_rounds/202608101600/202608101600_changelist.doc.lock.md` | 21 | 2026-08-10 | "**Phase:** DOC" | doc CL: overview header says what file is | batch 23 |
| `mock/design_rounds/202608101600/202608101600_changelist.src.lock.md` | 45 | 2026-08-10 | "**Phase:** IMPL" | src CL: no source change, two corrections | batch 23 |
| `mock/design_rounds/202608101600/202608101600_topic.a-locked-claim-that-was-never-executed.md` | 44 | 2026-08-10 | "**Phase:** TOPIC" | topic: locked claim never executed | batch 23 |
| `mock/design_rounds/202608101900/202608101900_changelist.doc.lock.md` | 48 | 2026-08-10 | "**Phase:** DOC" | doc CL: retract Uint<28> claim, reduction stated | batch 23 |
| `mock/design_rounds/202608101900/202608101900_changelist.src.lock.md` | 76 | 2026-08-10 | "**Phase:** IMPL" | src CL: id_from_counter to layout, fallible at | batch 23 |
| `mock/design_rounds/202608101900/202608101900_topic.the-sixth-route-and-what-a-shape-may-still-decide.md` | 62 | 2026-08-10 | "**Phase:** TOPIC" | topic: sixth route, what shape may decide | batch 23 |
| `mock/design_rounds/202609181402/202609181401_topic.hilavitkutin-on-arvo-dev.md` | 113 | 2026-09-18 | "**Phase:** TOPIC" | topic: arvo dev surface, std tier, op rulings | batch 23 |
| `mock/design_rounds/202609181402/202609181402_changelist.doc.lock.md` | 43 | 2026-09-18 | "**Phase:** DOC" | doc CL: platform tiers, build, persistence settled | batch 23 |
| `mock/design_rounds/202609181402/202609181403_changelist.src.lock.md` | 122 | 2026-09-18 | "**Phase:** IMPL" | src CL: Identity migration, std tier removed | batch 23 |
| `mock/design_rounds/202609190201/202609190200_topic.review-fixes-on-arvo-dev.md` | 56 | 2026-09-19 | "**Phase:** TOPIC" | topic: ten review findings, one-pointer closure | batch 23 |
| `mock/design_rounds/202609190201/202609190201_changelist.doc.lock.md` | 35 | 2026-09-19 | "**Phase:** DOC" | doc CL: closure gate, threads, persistence deps | batch 23 |
| `mock/design_rounds/202609190201/202609190202_changelist.src.lock.md` | 92 | 2026-09-19 | "**Phase:** IMPL" | src CL: OnePointerClosure, worker clamp, tests | batch 23 |
| `mock/design_rounds/202609190620/202607311110_topic.launcher-migration.md` | 71 | 2026-07-31 | "**Phase:** TOPIC" | topic: launcher migration, bootstrap teardown | batch 23 |
| `mock/design_rounds/202609190620/202609190619_topic.remove-the-stale-mockspace-build-dep-claim.md` | 68 | 2026-09-19 | "**Phase:** TOPIC" | topic: stale mockspace build-dep claim removal | batch 23 |
| `mock/design_rounds/202609190620/202609190620_changelist.doc.lock.md` | 40 | no date line | unstated (doc changelist; name ends .lock.md) | doc CL: three design docs drop claim | batch 23 |
| `mock/design_rounds/202609190620/202609190621_changelist.src.lock.md` | 13 | no date line | unstated (src changelist; name ends .lock.md) | src CL: no source change | batch 23 |
| `mock/design_rounds/202609191001/202609191000_topic.second-review-on-arvo-dev.md` | 43 | 2026-09-19 | "**Phase:** TOPIC" | topic: second review, thread-count rule, zero workers | batch 23 |
| `mock/design_rounds/202609191001/202609191001_changelist.doc.lock.md` | 29 | 2026-09-19 | "**Phase:** DOC" | doc CL: thread-count rule, zero-worker provisional | batch 23 |
| `mock/design_rounds/202609191001/202609191002_changelist.src.lock.md` | 57 | 2026-09-19 | "**Phase:** IMPL" | src CL: spawn-site lint, catalogue test, citations | batch 23 |
| `mock/design_rounds/202609191201/202609191200_topic.spawn-gate-lint-forcing.md` | 21 | 2026-09-19 | "**Phase:** TOPIC" | topic: spawn-gate lint forcing gap | batch 23 |
| `mock/design_rounds/202609191201/202609191201_changelist.doc.lock.md` | 17 | 2026-09-19 | "**Phase:** DOC" | doc CL: thread-count paragraph rule alone | batch 23 |
| `mock/design_rounds/202609191201/202609191202_changelist.src.lock.md` | 53 | 2026-09-19 | "**Phase:** IMPL" | src CL: lint matches forcing type, cfg refused | batch 23 |
| `mock/design_rounds/202611122301/202607200100_topic.b1-cap-lifting-completion.md` | 53 | 2026-07-20 | "**Phase:** TOPIC" | topic: B1 cap-lifting onto PlanDims capacities | batch 23 |
| `mock/design_rounds/202611122301/202607281547_topic.adopt-the-shared-hlist.md` | 71 | 2026-07-28 | "**Phase:** TOPIC" | topic: adopt shared hlist, substrate switch | batch 23 |
| `mock/design_rounds/202611122301/202611122258_topic.suite-can-fail.md` | 22 | 2026-11-12 | "**Phase:** TOPIC" | topic: suite-can-fail, replace_value call | batch 23 |
| `mock/design_rounds/202611122301/202611122301_changelist.doc.lock.md` | 25 | 2026-11-12 | "**Phase:** DOC" | doc CL: accumulator append asserts, README status | batch 23 |
| `mock/design_rounds/202611122301/202611130020_changelist.src.lock.md` | 54 | 2026-11-13 | "**Phase:** SRC" | src CL: ui pairs, differential, accumulator text | batch 23 |
| `mock/design_rounds/202611122317/202611122316_topic.the-catalogue-ledger-and-the-sealed-traits.md` | 21 | no date line | unstated (topic) | topic: catalogue ledger, sealed traits | batch 23 |
| `mock/design_rounds/202611122317/202611122317_changelist.doc.lock.md` | 9 | 2026-11-12 | "**Phase:** DOC" | doc CL: no template changes | batch 23 |
| `mock/design_rounds/202611122317/202611122318_changelist.src.lock.md` | 28 | 2026-11-12 | "**Phase:** SRC" | src CL: ignore ledger rows, Sealed pub(crate) | batch 23 |
| `mock/design_rounds/202611130044/202611130034_topic.adopt-the-shared-hlist.md` | 75 | 2026-07-28 | "**Phase:** TOPIC" | refiled hlist topic after round close | batch 23 |
| `mock/design_rounds/202611130044/202611130034_topic.b1-cap-lifting-completion.md` | 57 | 2026-07-20 | "**Phase:** TOPIC" | refiled B1 cap-lifting topic after round close | batch 23 |
| `mock/design_rounds/202611130044/202611130044_changelist.doc.lock.md` | 20 | 2026-11-13 | "**Phase:** DOC" | doc CL: fused entry gap stated | batch 23 |
| `mock/design_rounds/202611130044/202611130044_topic.suite-can-fail-review-fixes.md` | 22 | 2026-11-13 | "**Phase:** TOPIC" | topic: suite-can-fail review fixes | batch 23 |
| `mock/design_rounds/202611130044/202611130045_changelist.src.lock.md` | 53 | 2026-11-13 | "**Phase:** SRC" | src CL: ui fixtures, fused append test, markers | batch 23 |
| `mock/design_rounds/202611130058/202611130049_topic.adopt-the-shared-hlist.md` | 75 | 2026-07-28 | "**Phase:** TOPIC" | refiled hlist topic, second refiling | batch 23 |
| `mock/design_rounds/202611130058/202611130049_topic.b1-cap-lifting-completion.md` | 57 | 2026-07-20 | "**Phase:** TOPIC" | refiled B1 cap-lifting topic, second refiling | batch 23 |
| `mock/design_rounds/202611130058/202611130058_changelist.doc.lock.md` | 24 | 2026-11-13 | "**Phase:** DOC" | doc CL: replace_resource README, fused reset | batch 23 |
| `mock/design_rounds/202611130058/202611130058_topic.suite-can-fail-second-review.md` | 22 | 2026-11-13 | "**Phase:** TOPIC" | topic: suite-can-fail second review | batch 23 |
| `mock/design_rounds/202611130058/202611130102_changelist.src.lock.md` | 42 | 2026-11-13 | "**Phase:** SRC" | src CL: replace docs, fixture isolation, test | batch 23 |
| `mock/design_rounds/202611130216/202611130105_topic.adopt-the-shared-hlist.md` | 77 | 2026-07-28 | "**Phase:** TOPIC" | refiled hlist topic, third refiling | batch 23 |
| `mock/design_rounds/202611130216/202611130105_topic.b1-cap-lifting-completion.md` | 59 | 2026-07-20 | "**Phase:** TOPIC" | refiled B1 cap-lifting topic, third refiling | batch 23 |
| `mock/design_rounds/202611130216/202611130216_changelist.doc.lock.md` | 14 | 2026-11-13 | "**Phase:** DOC" | doc CL: no changes, open topics named | batch 23 |
| `mock/design_rounds/202611130216/202611130216_topic.suite-can-fail-coordinator-review.md` | 22 | 2026-11-13 | "**Phase:** TOPIC" | topic: suite-can-fail coordinator review | batch 23 |
| `mock/design_rounds/202611130216/202611130217_changelist.src.lock.md` | 37 | 2026-11-13 | "**Phase:** SRC" | src CL: pure map unit, shared stable provider | batch 23 |
| `mock/design_rounds/202611130250/202611130249_topic.notko-by-https.md` | 26 | 2026-11-13 | "**Phase:** TOPIC" | topic: notko source spelled by https | batch 23 |
| `mock/design_rounds/202611130250/202611130250_changelist.doc.lock.md` | 9 | 2026-11-13 | "**Phase:** DOC" | doc CL: no template changes, notko spelling | batch 23 |
| `mock/design_rounds/202611130250/202611130250_changelist.src.lock.md` | 37 | 2026-11-13 | "**Phase:** SRC" | src CL: notko https, patch section, lock held | batch 23 |
| `mock/design_rounds/202611130306/202611130224_topic.adopt-the-shared-hlist.md` | 79 | 2026-07-28 | "**Phase:** TOPIC" | refiled hlist topic, fourth refiling | batch 23 |
| `mock/design_rounds/202611130306/202611130225_topic.b1-cap-lifting-completion.md` | 61 | 2026-07-20 | "**Phase:** TOPIC" | refiled B1 cap-lifting topic, fourth refiling | batch 23 |
| `mock/design_rounds/202611130306/202611130306_changelist.doc.lock.md` | 14 | 2026-11-13 | "**Phase:** DOC" | doc CL: intended replace test, open topics | batch 23 |
| `mock/design_rounds/202611130306/202611130306_topic.suite-can-fail-intended-replace-test.md` | 16 | 2026-11-13 | "**Phase:** TOPIC" | the intended replace test as an ignored trybuild pass case | first hand |
| `mock/design_rounds/202611130306/202611130307_changelist.src.lock.md` | 37 | 2026-11-13 | locked src changelist, "**Phase:** SRC" | replace test added, fixtures asserting the defect deleted | first hand |
| `mock/design_rounds/202611130402/202611130400_topic.overflow-is-a-value.md` | 38 | 2026-11-13 | "**Phase:** TOPIC" | over-capacity accumulator append as a sticky refusal value | first hand |
| `mock/design_rounds/202611130402/202611130402_changelist.doc.lock.md` | 44 | 2026-11-13 | locked doc changelist, "**Phase:** DOC" | both designs state the refusal and `AccumRefused` | first hand |
| `mock/design_rounds/202611130402/202611130408_changelist.src.lock.md` | 63 | 2026-11-13 | locked src changelist, "**Phase:** SRC" | append counts `wanted`, run entries return the refusal | first hand |
| `mock/design_rounds/202611130500/202611130500_changelist.doc.lock.md` | 38 | no date line | locked doc changelist, "**Phase:** DOC" | review fixes: refusal per binding, regions after leading records | first hand |
| `mock/design_rounds/202611130500/202611130500_topic.overflow-review-fixes.md` | 27 | 2026-11-13 | "**Phase:** TOPIC" | review of the overflow round, four defects and four smaller | first hand |
| `mock/design_rounds/202611130500/202611130505_changelist.src.lock.md` | 45 | 2026-11-13 | locked src changelist, "**Phase:** SRC" | fused reset, region start, `RunCfg::Err` removed, mutant fenced | first hand |
| `mock/design_rounds/202611130546/202611130546_changelist.doc.lock.md` | 26 | no date line | locked doc changelist, "**Phase:** DOC" | review pass two: fused reset sentence, run signature | first hand |
| `mock/design_rounds/202611130546/202611130546_topic.overflow-review-pass-two.md` | 17 | 2026-11-13 | "**Phase:** TOPIC" | pass two of the overflow review, leftovers named | first hand |
| `mock/design_rounds/202611130546/202611130547_changelist.src.lock.md` | 34 | 2026-11-13 | locked src changelist, "**Phase:** SRC" | ledger row, no `Cfg::Err`, closed changelists restored | first hand |
| `mock/design_rounds/202611130554/202611130554_changelist.doc.lock.md` | 30 | no date line | locked doc changelist, "**Phase:** DOC" | op's ruling "Configurable by the consumer" in both designs | first hand |
| `mock/design_rounds/202611130554/202611130555_changelist.src.lock.md` | 38 | 2026-11-13 | locked src changelist, "**Phase:** SRC" | `AccumOverflow` with `Refuse` and `Assert` on the run config | first hand |
| `mock/design_rounds/202611130554/202611130600_topic.overflow-is-the-consumers-choice.md` | 23 | 2026-11-13 | "**Phase:** TOPIC" | op's ruling on overflow and the sealed choice it becomes | first hand |
| `mock/research/202605232100_workunit-cdylib-boundary.md` | 149 | 2026-05-23 | "Status: recommendation memo, aligned with landed `viola-plugin-abi`" | WorkUnit/cdylib FFI boundary, RunLint slots | batch 10 |
| `mock/research/202605251200_validate-descriptor-cl-drift.md` | 104 | 2026-05-25 | "Status: drift documentation." | locked CL vs shipped validate_descriptor visibility | batch 10 |
| `mock/research/202605282100_engine-dispatch-build-plan.md` | 98 | 2026-05-28 | unstated (research memo, build plan) | engine dispatch arc: state map, phases A-E, risks | batch 10 |
| `mock/research/202605282319_phase-b-data-plane-design.md` | 89 | 2026-05-28 | unstated (research, implementation guide) | Phase B data plane, per-WU Context, rounds B1-B5 | batch 10 |
| `mock/research/202605290230_phase-c-arvo-analysis-wiring.md` | 117 | 2026-05-29 | unstated (research, design note) | Phase C: wire arvo CSR/spectral into plan chain | batch 10 |
| `mock/research/202605301130_numeric-position-convention.md` | 359 | 2026-05-30 | "Status: OPEN design discussion with op. Not a round yet; topics captured live." | bare usize vs arvo types in const-generic position | batch 10 |
| `mock/research/202605302130_unified-storage-design-archaeology.md` | 72 | 2026-05-30 | unstated (research, archaeology; "Purpose: catalogue the storage-relevant design corpus") | storage design corpus chronology and heritage synthesis | batch 10 |
| `mock/research/202606052000_single-core-engine-ideal-vs-actual-audit.md` | 144 | 2026-06-05 | unstated (research, audit memo) | single-core engine vs design thesis, drift, perf gate | batch 10 |
| `mock/research/202606060900_engine-completion-strategic-synthesis.md` | 133 | 2026-06-06 | unstated (research memo, "Step 4 of a `chart-the-path` planning routine") | engine completion shape, state map, proven/unproven | batch 10 |
| `mock/research/202606061000_canon-amendment-unified-engine.md` | 38 | 2026-06-06 | "Status: CANON. Authority: op (O. R. Toimela), 2026-06-06, explicit ruling." | engine is unified; core count is configuration | batch 10 |
| `mock/research/202606061100_engine-completion-roadmap-draft.md` | 231 | 2026-06-06 | "it is now the finalised roadmap whose unproven steps carry a planned sketch (section 9)" | roadmap C/D/E/F steps with sketch status | batch 10 |
| `mock/research/202606070030_dispatch-order-keystone-reconciliation.md` | 118 | 2026-06-07 | "**Phase:** research / reconciliation (resolves the Phase D dispatch-order keystone fork)" | dispatch-order fork, carrier order, corrections, #669 | batch 10 |
| `mock/research/202606070100_gate2-trunk-sectioning-rechart.md` | 76 | 2026-06-07 | "Supersedes the Phase E framing in roadmap r1" | GATE-2 parallelism is trunk-per-core | batch 10 |
| `mock/research/202606070200_engine-roadmap-r3-gate2.md` | 80 | 2026-06-07 | "Supersedes roadmap r1 (`202606061100`) section 5 and r2 (`202606081500`) section 4 wherever they describe *how parallelism is achieved*." | GATE-2 roadmap r3, trunk-sectioning then core-pinning | batch 10 |
| `mock/research/202606070700_engine-roadmap-r4-gate2-mechanism.md` | 78 | 2026-06-07 | "Supersedes roadmap r3 ... **only at stage G2-0c**" | const-eval grouping and const-gated DCE carrier mechanism | batch 10 |
| `mock/research/202606071029_gate2-dispatch-mechanism-options.md` | 99 | 2026-06-07 | unstated (research, options after S3 wall) | N-core dispatch mechanism options, web research | batch 10 |
| `mock/research/202606071200_gate2-carrier-mechanism-fork.md` | 67 | 2026-06-07 | "**Phase:** course-correction (chart-the-path step 11: a step cannot be proven as the roadmap assumed)" | carrier-materialisation fork, options A-D, op's call | batch 10 |
| `mock/research/202606071400_gate2-phase-axis-course-correction.md` | 110 | 2026-06-07 | "**Status:** course-correction finding, pending op decision (chart-the-path step 11)" | depth is not the waist-bounded phase | batch 10 |
| `mock/research/202606071500_gate2-corrected-dispatch-chart.md` | 108 | 2026-06-07 | "**Status:** chart-the-path synthesis + roadmap DRAFT" and later "FINALISED roadmap (steps 8-9) — supersedes all drafts above" | waist-bounded phases, const waist in arvo, R0-R4 | batch 10 |
| `mock/research/202606072100_gate2-canonical-deviation-ledger.md` | 111 | 2026-06-07 | "CORRECTION (2026-06-08): §1, §4, and §9 are STALE." | GATE-2 deviations from canonical design, dispositions | batch 10 |
| `mock/research/202606072130_gate2-r4d-branching-bench-finding.md` | 62 | 2026-06-07 | "Verdict: the threaded executor is correct but does not green `branching`; it regresses it." | R4d bench: trunk parallelism vs branching parity | batch 10 |
| `mock/research/202606072330_gate2-branching-gate-vs-canonical-materialisation.md` | 60 | 2026-06-07 | "Verdict: the planned "cross-fiber diamond fusion" is a DEVIATION" plus later "Correction (op, 2026-06-07)" | branching gate vs fiber-boundary materialisation | batch 10 |
| `mock/research/202606080900_gate2-n-perf-finding.md` | 42 | 2026-06-08 | "CORRECTION (2026-06-08, op caught it)" | G2-N per-trunk dispatch perf, unfair baseline correction | batch 10 |
| `mock/research/202606081100_gate2-parallel-bench-fairness.md` | 42 | 2026-06-08 | "Supersedes the perf claim in" 202606080900 | parallel bench vs multi-threaded std baseline | batch 10 |
| `mock/research/202606081300_full-engine-overview-and-reconciliation.md` | 116 | 2026-06-08 | unstated (research, "Step-2/3/4 deliverable"); "This document is the short-form index into the design, not a replacement for it." | full feature index, static vs adaptive, extensibility | batch 10 |
| `mock/research/202606081500_engine-roadmap-r2.md` | 113 | 2026-06-08 | "Status: FINALISED chart-the-path re-run (op-directed, full-engine scope)" | roadmap r2 after dispatch pivot, op decisions a-c | batch 10 |
| `mock/research/202606081600_engine-state-map-and-roadmap-r5.md` | 81 | 2026-06-08 | "Supersedes for currency: the status columns of `202606081300_full-engine-overview-and-reconciliation.md`" | state map, drift audit, forward chart r5 | batch 10 |
| `mock/research/202606081930_e4-slice2-meta-kernel-shape-resolution.md` | 48 | 2026-06-08 | unstated (research note, "Round: ... at TOPIC") | meta kernel shape A resolution | batch 10 |
| `mock/research/202606082300_run-parallel-meta-parity-findings.md` | 35 | 2026-06-08 | unstated (findings note) | run_parallel meta parity, sequencing | batch 10 |
| `mock/research/202606090100_e4-slice3-resolution.md` | 196 | 2026-06-08 | "Outcome: the topic's two hard de-risk targets ... DISSOLVE on re-grounding" | slice-3 consumer meta hook is OnMeta WU | batch 10 |
| `mock/research/202606111400_canonical-design-addendum-a1.md` | 209 | 2026-06-11 | "**Status:** locked (op rulings recorded live, 2026-06-11)" | Addendum A1: op rulings, constraint notes, amendment chain | batch 10 |
| `mock/research/202606111600_completion-arc-synthesis.md` | 221 | 2026-06-11 | "**Status:** LOCKED (all five pre-chart sketch findings recorded, section 6)" | completion-arc research: what is built, unbuilt, sketch results | batch 10 |
| `mock/research/202606111700_completion-arc-roadmap.md` | 479 | 2026-06-11 | "**Status:** LOCKED (canonical-mirror + granularity expert passes folded in)" | completion-arc roadmap P0-P4 slices | batch 10 |
| `mock/research/202606111800_canonical-spec-standalone.md` | 951 | 2026-06-11 | "**Status:** the consolidated, self-contained design of record" | standalone canonical design spec, appendices, glossary | batch 10 |
| `mock/research/202606191600_p1-adapt-comprehension.md` | 131 | 2026-06-19 | "durable comprehension artifact for the P1 arc" (stated at :8; no status line) | P1 adapt + E8 built-vs-unbuilt comprehension | batch 10 |
| `mock/research/202606191700_p1-adapt-roadmap-draft.md` | 190 | 2026-06-19 | "it is a draft" (:12) | P1 adapt/E8 roadmap draft R1-R10, mirror revisions | batch 10 |
| `mock/research/202606191800_p1-adapt-granularity-sketch-plan.md` | 59 | 2026-06-19 | unstated (chart-the-path phases 8-9 plan) | adapt sub-step decomposition and sketch order | batch 10 |
| `mock/research/202606201300_adapt-actuation-chart.md` | 81 | 2026-06-19 | "**Status:** finding + roadmap for the adapt actuation arc (domain 22, post-#148)" | adapt actuation surfaces dead, wiring fork | batch 10 |
| `mock/research/202606201400_per-fiber-morsel-blueprint.md` | 88 | 2026-06-19 | "design-reviewed blueprint (code-architect)" | per-fiber morsel loop slices, two findings | batch 11 |
| `mock/research/202606201700_arc-audit-and-updated-roadmap.md` | 173 | 2026-06-19 | "critical chart-the-path (op-requested, autonomous)" | arc audit, drift, skips, resequenced roadmap, op resolution | batch 11 |
| `mock/research/202606201900_granular-roadmap-and-sketch-plan.md` | 221 | 2026-06-19 | "completes the critical chart-the-path" | granular leaves, sketch plan, 2026-07-02 status pass | batch 11 |
| `mock/research/202606210000_a3-resource-footprint-rechart.md` | 92 | 2026-06-20 | "limited chart-the-path (op-requested)" | A3 morsel L1 budget against R5 resources | batch 11 |
| `mock/research/202606210600_analysis-self-storage-model.md` | 94 | 2026-06-19 | "one of the pressure-test deliverables" | main agent hybrid lean on resource storage | batch 11 |
| `mock/research/202606210600_expert-alternatives-storage-model.md` | 281 | 2026-06-19 | "research deliverable for round 202606210600" | survey of alternative resource storage models | batch 11 |
| `mock/research/202606210600_expert-architect-storage-model.md` | 55 | 2026-06-19 | "Deliverable for: round 202606210600 pressure-test" | architect analysis, impl path, alternatives | batch 11 |
| `mock/research/202606210600_expert-loimu-heritage-storage-model.md` | 503 | 2026-06-19 | "Durable deliverable for the pressure-test topic" | loimu storage heritage versus resource model | batch 11 |
| `mock/research/202606210600_expert-perf-storage-model.md` | 106 | 2026-06-21 | unstated (expert analysis, "No verdict (op's call)") | perf, soundness, bench gaps of storage model | batch 11 |
| `mock/research/202606210600_resource-storage-model-canonical-addendum.md` | 101 | 2026-06-19, revised 2026-07-02 | "canonical reading-correction of consolidation-spec R5" | resource handle, blob, snapshot, erased addressing | batch 11 |
| `mock/research/202606210600_storage-bench-findings.md` | 398 | 2026-06-20 | unstated (findings) | six-variant storage bench, axes, run-2, hybrid | batch 11 |
| `mock/research/202606210600_synthesis-resource-storage-model.md` | 112 | 2026-06-19 | unstated (synthesis) | convergence, bench plan, op decisions | batch 11 |
| `mock/research/202607181200_engine-arc-recomprehension.md` | 183 | 2026-07-18 | "synthesis of an independent comprehension pass and a source-verification pass" | roadmap re-comprehension, head+tail finding | batch 11 |
| `mock/research/202607181300_engine-roadmap-r6.md` | 541 | 2026-07-18 | "chart-the-path phase 5 draft, pending the canonical-mirror pass and the granularity pass" | corrected ledger, leaves, evidence ledger, sketches | batch 11 |
| `mock/research/202607200200_replaceable-swap-semantics-spec.md` | 128 | 2026-07-19 | "DRAFT pending op ratification" | replace_value and replace_resource swap spec S1-S7 | batch 11 |
| `mock/research/202607200300_pre-freeze-open-question-dossier.md` | 130 | 2026-07-19 | "evidence assembly for the seed pre-freeze resolution batch" | GATE-2 deviations, spectral, mid_slot, LIGHT_THRESHOLD | batch 11 |
| `mock/research/202607200600_a1-1-rcm-order-bench-resolution.md` | 63 | 2026-07-19 | "bench-decided" | RCM row order bench, ruling | batch 11 |
| `mock/research/202607200800_a1-1-ordering-theory-bench-expansion.md` | 102 | 2026-07-19 | "bench-decided; supersedes the evidence base of" | rival ordering theories benched, RCM confirmed | batch 11 |
| `mock/research/202607201100_swap-semantics-ratification-evidence.md` | 83 | 2026-07-19 | unstated (evidence memo, asks ratification) | swap implementation and bench evidence | batch 11 |
| `mock/research/202607201200_spectral-role-bench-evidence.md` | 93 | 2026-07-19 | "evidence delivered under the evidence-then-bless standard" | spectral versus greedy fiber bench #644 | batch 11 |
| `mock/research/202607201400_gate2-deviation-1-6-evidence.md` | 63 | 2026-07-19 | "evidence delivered under evidence-then-bless (A2-4)" | arena plane relocation sketch, Pin receiver | batch 11 |
| `mock/research/202607201500_mid-slot-semantics-call.md` | 55 | 2026-07-19 | "the item-7 design call of the seed pre-freeze batch" | mid_slot is a record boundary | batch 11 |
| `mock/research/202607201600_gate2-deviation-5-audit.md` | 52 | 2026-07-19 | "the #689 audit, delivered as the deviation-5 evidence" | aliasing audit confirms hole | batch 11 |
| `mock/research/202607202000_gate2-deviation-4-memo.md` | 58 | 2026-07-19 | "the design-memo evidence for deviation 4" | pointer-size spawn, exit-counter join | batch 11 |
| `mock/research/202607202100_light-threshold-gate-record.md` | 44 | 2026-07-19 | "the item-8 resolution record of the seed pre-freeze batch" | LIGHT_THRESHOLD oracle and gate | batch 11 |
| `mock/research/202607202200_gate2-deviation-2-evidence.md` | 42 | 2026-07-19 | "the bench-gate evidence for deviation 2" | ownership mask standing oracles | batch 11 |
| `mock/research/202607202340_wake-policy-evidence.md` | 66 | 2026-07-19 | "the deviation-3 evidence under evidence-then-bless (A2-4)" | wake spin pre-roll bench round 1 | batch 11 |
| `mock/research/202607210010_seed-verification-pass-1-deep-audit.md` | 166 | 2026-07-19 (in title) | "FIT AFTER LISTED FIXES" | deep audit of seed chapters | batch 11 |
| `mock/research/202607210010_seed-verification-pass-1.md` | 59 | 2026-07-19 | "the first of the two verification passes" | four audits and fix batch summary | batch 11 |
| `mock/research/202607210100_wake-policy-reexamination.md` | 71 | 2026-07-19 | "supersedes the mechanism story and the margin figures of" | wake policy second bench, corrected ruling | batch 11 |
| `mock/research/202607210300_seed-verification-redo.md` | 90 | 2026-07-20 | "op-ordered redo of the seed fitness verification" | four audits against original sources, fixes | batch 11 |
| `mock/research/2026_05_05_doc_audit.md` | 238 | 2026-05-05 | unstated (audit) | per-crate documentation audit findings | batch 11 |
| `mock/research/2026_05_05_doc_audit_entrypoints.md` | 71 | 2026-05-05 | unstated (audit) | entrypoint documentation audit findings | batch 11 |
| `mock/research/2026_05_11_concurrency_audit_round_202605101036.md` | 79 | 2026-05-11 | "all High-impact findings addressed; deferral roadmap executed in full" | concurrency audit, deferral roadmap | batch 11 |
| `mock/research/EXTENSIONS_RESEARCH.md` | 85 | undated | unstated (research) | extensions and plugins prior art, directives | batch 11 |
| `mock/research/README.md` | 65 | undated | unstated (readme) | research directory standing of seed, pre_seed, imports | batch 11 |
| `mock/research/hilavitkutin-api-history.md` | 13 | undated | "Tier 2 audit-trail file" | source citations migrated from api design | batch 11 |
| `mock/research/hilavitkutin-build-design-audit.md` | 201 | 2026-04-21 | unstated (audit) | hilavitkutin-build designed surface and gaps | batch 11 |
| `mock/research/hilavitkutin-build-history.md` | 13 | undated | "Tier 2 audit-trail file" | source citations migrated from build design | batch 11 |
| `mock/research/hilavitkutin-history.md` | 11 | undated | "Tier 2 audit-trail file" | source citations migrated from engine design | batch 11 |
| `mock/research/marker_trait_attr_bitset_fallback.md` | 103 | 2026-05-05 | "Research note only. Not built." | marker_trait_attr risk, bitset fallback shape | batch 11 |
| `mock/research/next-solver-verification.md` | 81 | 2026-05-04 (Recorded) | unstated (research); "Task #297 stays in a soft-pending state" | next-solver verification of api type-state | batch 11 |
| `mock/research/notes/202605091700_src_apply_decisions.md` | 127 | 2026-05-10 | unstated; notes ("Captures impl-detail decisions made within the locked doc CL's design space.") | src-apply decisions D1-D12, provider round | batch 13 |
| `mock/research/parked/README.md` | 11 | unstated (no date in name) | "It is a record, not a thing to apply" | readme for parked patches directory | batch 13 |
| `mock/research/pre_seed/202606111800_canonical-spec-standalone.md` | 955 | 2026-06-11 | "reading aid, not the tiebreak (A2-1, `202607193200`)" | standalone canonical spec, parts I to III | batch 13 |
| `mock/research/pre_seed/202607191400_engine-three-way-audit.md` | 245 | 2026-07-19 | "chart-the-path take 2, phases 2 to 4" | three-way audit of canon, source, roadmap claims | batch 13 |
| `mock/research/pre_seed/202607191500_engine-roadmap-r7.md` | 637 | 2026-07-19 | "chart-the-path take 2, phase 5 draft" | engine roadmap r7 from the audit ledger | batch 13 |
| `mock/research/pre_seed/202607193000_engine-roadmap-r8.md` | 424 | 2026-07-19 | "chart-the-path conclusions for the engine-completion arc" | engine roadmap r8, supersedes r7 | batch 13 |
| `mock/research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md` | 102 | 2026-07-19 | "locked" | addendum A2: precedence, rulings, citation correction | batch 13 |
| `mock/research/pre_seed/202607193300_swap-semantics-panel/01-arntzen-soundness.md` | 36 | 2026-07-19 | unstated; panel memo 1 | soundness view of swap semantics | batch 13 |
| `mock/research/pre_seed/202607193300_swap-semantics-panel/02-aaltonen-perf.md` | 43 | 2026-07-19 | unstated; panel memo 2 | cost-shape view of swap semantics | batch 13 |
| `mock/research/pre_seed/202607193300_swap-semantics-panel/03-giesen-synthesis.md` | 45 | 2026-07-19 | unstated; panel memo 3 | synthesis and draft swap specification S1-S7 | batch 13 |
| `mock/research/pre_seed/202607193400_canon-registry-derivation.md` | 93 | 2026-07-19 | "derivation for op review; nothing here is adopted until ruled" | canon registry schema derivation | batch 13 |
| `mock/research/pre_seed/sketches/202607192200_r1-participation-without-record-slices/FINDINGS.md` | 218 | 2026-07-19 | "Hypothesis CONFIRMED at the fourth attempt." | sketch on worker participation after N-way split removal | batch 13 |
| `mock/research/pre_seed/sketches/202607192700_d1-install-arm/FINDINGS.md` | 130 | 2026-07-19 | "FAILS, and the failure is larger than the hypothesis." | sketch of replace_value install arm | batch 13 |
| `mock/research/pre_seed/sketches/202607193310_planaffecting-replaceable-exclusivity/FINDINGS.md` | 35 | 2026-07-19 | "Outcome: WORKS" | sketch PlanAffecting/Replaceable negative impl | batch 13 |
| `mock/research/pre_seed/sketches/202607193500_canon-registry-drain/DRAIN-MANIFEST.md` | 54 | no date line | unstated; sketch manifest | drain manifest mapping storage passages to rows | batch 13 |
| `mock/research/pre_seed/sketches/202607193500_canon-registry-drain/FINDINGS.md` | 103 | 2026-07-19 | "Outcome: WORKS" | sketch findings: storage canon drained to registry | batch 13 |
| `mock/research/pre_seed/sketches/202607193500_canon-registry-drain/SCHEMA.md` | 91 | no date line | unstated; sketch schema | registry namespaces and ref roots fragment | batch 13 |
| `mock/research/pre_seed/sketches/202607193500_canon-registry-drain/seed/storage.md` | 100 | no date line | "Sketch demonstration of a seed chapter" | storage and resources seed chapter demo | batch 13 |
| `mock/research/seed/MANIFEST.md` | 247 | no date line | "frozen = false" | which sources drained into which seed chapter | first hand |
| `mock/research/seed/constraints.md` | 108 | as of 2026-07-19 | seed chapter, not frozen | bans, toolchain limits, standing constraints | first hand |
| `mock/research/seed/contracts.md` | 120 | as of 2026-07-19 | seed chapter, not frozen | the api contracts: unit, access sets, builder, virtuals | first hand |
| `mock/research/seed/data-model.md` | 134 | as of 2026-07-19 | seed chapter, not frozen | store kinds, column values, ordering, determinism | first hand |
| `mock/research/seed/dispatch.md` | 161 | as of 2026-07-19 | seed chapter, not frozen | dispatch codegen, fused bodies, intrinsics | first hand |
| `mock/research/seed/execution.md` | 206 | as of 2026-07-19 | seed chapter, not frozen | threading, convergence, strategies, adaptation | first hand |
| `mock/research/seed/foundations.md` | 137 | as of 2026-07-19 | seed chapter, not frozen | arvo primitives, platform tiers, build-time optimisation | first hand |
| `mock/research/seed/governance.md` | 195 | as of 2026-07-19 | seed chapter, "under construction" while frozen = false | precedence ladder, evidence standard, lifecycle, open questions | first hand |
| `mock/research/seed/identity.md` | 132 | as of 2026-07-19 | seed chapter, not frozen | what the engine is, scope, vocabulary | first hand |
| `mock/research/seed/plan.md` | 224 | as of 2026-07-19 | seed chapter, not frozen | analysis steps, fibers, morsels, data loading | first hand |
| `mock/research/seed/scheduler.md` | 126 | as of 2026-07-19 | seed chapter, not frozen | builder, meta pipeline, version stamps, errors | first hand |
| `mock/research/seed/storage.md` | 116 | as of 2026-07-19 | seed chapter, not frozen | resource storage model, snapshot, replace and install | first hand |

## Read from the recovery branch

These files are not on the working branch. They were read from `origin/recover/unlanded-rounds-and-benches`.

| file | lines | date | standing, by its own words | holds | read by |
|---|---|---|---|---|---|
| `mock/benches/engine_vs_std/FINDINGS.md` | 151 | unstated | "STATUS: PREMATURE / SUPERSEDED." | single-core engine versus std bench, gap attributed | batch R5 |
| `mock/benches/results/INDEX.md` | 57 | unstated | unstated, results index | generated index of bench results and findings links | batch R5 |
| `mock/benches/results/fiber_theory/fiber_theory_n16_findings.md` | 288 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/fiber_theory/fiber_theory_n32_findings.md` | 288 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/fiber_theory/fiber_theory_n64_findings.md` | 288 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/fiber_theory/fiber_theory_n8_findings.md` | 287 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/rcm_order/rcm_order_n1024_findings.md` | 209 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/rcm_order/rcm_order_n16384_findings.md` | 214 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/rcm_order/rcm_order_n2048_findings.md` | 209 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/rcm_order/rcm_order_n256_findings.md` | 210 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/rcm_order/rcm_order_n4096_findings.md` | 209 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/rcm_order/rcm_order_n64_findings.md` | 209 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/rcm_order/rcm_order_n8192_findings.md` | 209 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/rcm_rivals/rcm_rivals_n1024_findings.md` | 242 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/rcm_rivals/rcm_rivals_n16384_findings.md` | 247 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/rcm_rivals/rcm_rivals_n2048_findings.md` | 242 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/rcm_rivals/rcm_rivals_n256_findings.md` | 240 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/rcm_rivals/rcm_rivals_n4096_findings.md` | 240 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/rcm_rivals/rcm_rivals_n64_findings.md` | 240 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/rcm_rivals/rcm_rivals_n8192_findings.md` | 242 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/swap_band/swap_band_n1024_findings.md` | 177 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/swap_band/swap_band_n64_findings.md` | 177 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/swap_band/swap_band_n8192_findings.md` | 177 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/swap_cost/swap_cost_n64_findings.md` | 323 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/wake_policy/wake_policy_n1024_findings.md` | 315 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/wake_policy/wake_policy_n64_findings.md` | 309 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/benches/results/wake_policy/wake_policy_n8192_findings.md` | 309 (header; timings tables not shown) | unstated | unstated, bench findings | bench key findings, median and spread | batch R5 |
| `mock/design_rounds/202608090700/202608090700_changelist.doc.md` | 33 | 2026-08-09 | "**Phase:** DOC" | doc changelist, sym shape width and interner | batch R5 |

## Identical duplicates

Each file on the left is byte for byte the file on the right, which is ledgered above or below.

| duplicate | of |
|---|---|
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.io-integration-pipelines.md` | `mock/design_rounds/202604200055/202603141800_research.io-integration-pipelines.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603131400_research.llvm-plugin-passes.md` | `mock/design_rounds/202604200055/202603131400_research.llvm-plugin-passes.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.yielding-deep-dive.md` | `mock/design_rounds/202604200055/202603141800_research.yielding-deep-dive.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.loimu-incremental.md` | `mock/design_rounds/202604200055/202603141800_research.loimu-incremental.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603231200_changelist.doc.deprecated.md` | `mock/design_rounds/202603231200-abandoned/202603231200_changelist.doc.deprecated.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603111529_topic.paradigm-and-architecture.md` | `mock/design_rounds/202604200055/202603111529_topic.paradigm-and-architecture.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.registration-discovery.md` | `mock/design_rounds/202604200055/202603141800_research.registration-discovery.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.morsel-pipeline-execution.md` | `mock/design_rounds/202604200055/202603141800_research.morsel-pipeline-execution.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603131230_topic.build-configuration-and-cfg-gating.md` | `mock/design_rounds/202604200055/202603131230_topic.build-configuration-and-cfg-gating.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.task-plan.md` | `mock/design_rounds/202604200055/202603141800_research.task-plan.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.unified-store-deep-dive.md` | `mock/design_rounds/202604200055/202603141800_research.unified-store-deep-dive.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603151200_topic.hilavitkutin-api-surface.md` | `mock/design_rounds/202604200055/202603151200_topic.hilavitkutin-api-surface.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.loimu-macros.md` | `mock/design_rounds/202604200055/202603141800_research.loimu-macros.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.scheduler-algorithm-deep-dive.md` | `mock/design_rounds/202604200055/202603141800_research.scheduler-algorithm-deep-dive.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.loimu-signals.md` | `mock/design_rounds/202604200055/202603141800_research.loimu-signals.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603171900_topic.execution-model.md` | `mock/design_rounds/202604200055/202603171900_topic.execution-model.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.fault-tolerance-backpressure.md` | `mock/design_rounds/202604200055/202603141800_research.fault-tolerance-backpressure.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.loimu-synthesis.md` | `mock/design_rounds/202604200055/202603141800_research.loimu-synthesis.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.dag-scheduling-work-stealing.md` | `mock/design_rounds/202604200055/202603141800_research.dag-scheduling-work-stealing.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.columnar-storage-ecs.md` | `mock/design_rounds/202604200055/202603141800_research.columnar-storage-ecs.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.loimu-primitives.md` | `mock/design_rounds/202604200055/202603141800_research.loimu-primitives.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603131400_research.rustc-llvm-optimization-flags.md` | `mock/design_rounds/202604200055/202603131400_research.rustc-llvm-optimization-flags.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603131700_research.llvm-passes-beyond-polly.md` | `mock/design_rounds/202604200055/202603131700_research.llvm-passes-beyond-polly.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.loisto-design-rounds.md` | `mock/design_rounds/202604200055/202603141800_research.loisto-design-rounds.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_topic.hilavitkutin-core-design.md` | `mock/design_rounds/202604200055/202603141800_topic.hilavitkutin-core-design.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603131600_research.polly-llvm-rust-ir.md` | `mock/design_rounds/202604200055/202603131600_research.polly-llvm-rust-ir.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603131500_research.per-crate-backends-and-domain-optimizers.md` | `mock/design_rounds/202604200055/202603131500_research.per-crate-backends-and-domain-optimizers.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.loimu-storage.md` | `mock/design_rounds/202604200055/202603141800_research.loimu-storage.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603121800_topic.dispatch-and-optimisation.md` | `mock/design_rounds/202604200055/202603121800_topic.dispatch-and-optimisation.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603181200_topic.hilavitkutin-design-consolidation.md` | `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.loisto-scheduler.md` | `mock/design_rounds/202604200055/202603141800_research.loisto-scheduler.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.loimu-scheduler.md` | `mock/design_rounds/202604200055/202603141800_research.loimu-scheduler.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.workunit-trait-deep-dive.md` | `mock/design_rounds/202604200055/202603141800_research.workunit-trait-deep-dive.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603121310_topic.memory-layout-and-platform.md` | `mock/design_rounds/202604200055/202603121310_topic.memory-layout-and-platform.md` |
| `mock/research/imported-from-polka-dots/design_rounds/202603141800_research.loimu-dag.md` | `mock/design_rounds/202604200055/202603141800_research.loimu-dag.md` |

## Generated

The files under `docs/` are rendered from the templates ledgered above and were not read as sources.

| file | lines |
|---|---|
| `docs/API.md` | 1386 |
| `docs/BUILD.md` | 321 |
| `docs/CTX.md` | 309 |
| `docs/DESIGN.md` | 169 |
| `docs/EXTENSIONS.md` | 200 |
| `docs/EXTENSIONS_MACROS.md` | 86 |
| `docs/HILAVITKUTIN.md` | 1451 |
| `docs/KIT.md` | 271 |
| `docs/LINKING.md` | 126 |
| `docs/PERSISTENCE.md` | 238 |
| `docs/PRINCIPLES.md` | 296 |
| `docs/PROVIDERS.md` | 212 |
| `docs/STR.md` | 245 |
| `docs/STRUCTURE.md` | 218 |
| `docs/SYM.md` | 166 |
| `docs/WORKFLOW.md` | 57 |

## Not yet read

| file | lines | batch |
|---|---|---|
| `mock/benches/NIGHT_FINDINGS_2026-05-11.md` | 884 | 46 |
| `mock/benches/README.md` | 45 | 46 |
| `mock/benches/atomic_inc_strategy_n1024_findings.md excerpt` | 320 | 46 |
| `mock/benches/atomic_inc_strategy_n16384_findings.md excerpt` | 319 | 46 |
| `mock/benches/atomic_inc_strategy_n256_findings.md excerpt` | 318 | 46 |
| `mock/benches/atomic_inc_strategy_n4096_findings.md excerpt` | 320 | 46 |
| `mock/benches/atomic_ordering_cost_n1024_findings.md excerpt` | 318 | 46 |
| `mock/benches/atomic_ordering_cost_n16384_findings.md excerpt` | 318 | 46 |
| `mock/benches/atomic_ordering_cost_n256_findings.md excerpt` | 319 | 46 |
| `mock/benches/atomic_ordering_cost_n4096_findings.md excerpt` | 318 | 46 |
| `mock/benches/atomic_ordering_cost_n64_findings.md excerpt` | 322 | 46 |
| `mock/benches/atomic_vs_plain_n1024_findings.md excerpt` | 317 | 46 |
| `mock/benches/atomic_vs_plain_n16384_findings.md excerpt` | 316 | 46 |
| `mock/benches/atomic_vs_plain_n256_findings.md excerpt` | 319 | 46 |
| `mock/benches/atomic_vs_plain_n4096_findings.md excerpt` | 318 | 46 |
| `mock/benches/bit_count_ops_n1024_findings.md excerpt` | 318 | 46 |
| `mock/benches/bit_count_ops_n16384_findings.md excerpt` | 321 | 46 |
| `mock/benches/bit_count_ops_n256_findings.md excerpt` | 321 | 46 |
| `mock/benches/bit_count_ops_n4096_findings.md excerpt` | 321 | 46 |
| `mock/benches/bounds_check_n1024_findings.md excerpt` | 319 | 46 |
| `mock/benches/bounds_check_n16384_findings.md excerpt` | 318 | 46 |
| `mock/benches/bounds_check_n256_findings.md excerpt` | 320 | 46 |
| `mock/benches/bounds_check_n4096_findings.md excerpt` | 318 | 46 |
| `mock/benches/branch_pattern_n1024_findings.md excerpt` | 286 | 46 |
| `mock/benches/branch_pattern_n16384_findings.md excerpt` | 285 | 46 |
| `mock/benches/branch_pattern_n256_findings.md excerpt` | 287 | 46 |
| `mock/benches/branch_pattern_n4096_findings.md excerpt` | 285 | 46 |
| `mock/benches/branch_pattern_n64_findings.md excerpt` | 285 | 46 |
| `mock/benches/branch_predictability_n1024_findings.md excerpt` | 319 | 46 |
| `mock/benches/branch_predictability_n16384_findings.md excerpt` | 316 | 46 |
| `mock/benches/branch_predictability_n256_findings.md excerpt` | 321 | 46 |
| `mock/benches/branch_predictability_n4096_findings.md excerpt` | 319 | 46 |
| `mock/benches/cache_layout_n1024_findings.md excerpt` | 288 | 46 |
| `mock/benches/cache_layout_n16384_findings.md excerpt` | 285 | 46 |
| `mock/benches/cache_layout_n256_findings.md excerpt` | 290 | 46 |
| `mock/benches/cache_layout_n4096_findings.md excerpt` | 286 | 46 |
| `mock/benches/cache_layout_n64_findings.md excerpt` | 290 | 46 |
| `mock/benches/codegen_dispatch_axis_a_assertions.md` | 124 | 46 |
| `mock/benches/dep_graph_csr_9_n128_findings.md excerpt` | 320 | 46 |
| `mock/benches/dep_graph_csr_9_n16_findings.md excerpt` | 320 | 46 |
| `mock/benches/dep_graph_csr_9_n256_findings.md excerpt` | 319 | 46 |
| `mock/benches/dep_graph_csr_9_n32_findings.md excerpt` | 318 | 46 |
| `mock/benches/dep_graph_csr_9_n64_findings.md excerpt` | 319 | 46 |
| `mock/benches/dep_graph_csr_9_n8_findings.md excerpt` | 318 | 46 |
| `mock/benches/dispatch_dynamic_n1024_findings.md excerpt` | 314 | 46 |
| `mock/benches/dispatch_dynamic_n16384_findings.md excerpt` | 318 | 46 |
| `mock/benches/dispatch_dynamic_n256_findings.md excerpt` | 313 | 46 |
| `mock/benches/dispatch_dynamic_n4096_findings.md excerpt` | 317 | 46 |
| `mock/benches/dispatch_dynamic_n64_findings.md excerpt` | 324 | 46 |
| `mock/benches/dispatch_static_n1024_findings.md excerpt` | 312 | 46 |
| `mock/benches/dispatch_static_n16384_findings.md excerpt` | 316 | 46 |
| `mock/benches/dispatch_static_n256_findings.md excerpt` | 312 | 46 |
| `mock/benches/dispatch_static_n4096_findings.md excerpt` | 313 | 46 |
| `mock/benches/dispatch_static_n64_findings.md excerpt` | 324 | 46 |
| `mock/benches/ema_axis_h_n1024_findings.md excerpt` | 318 | 46 |
| `mock/benches/ema_axis_h_n16384_findings.md excerpt` | 317 | 46 |
| `mock/benches/ema_axis_h_n256_findings.md excerpt` | 319 | 46 |
| `mock/benches/ema_axis_h_n4096_findings.md excerpt` | 318 | 46 |
| `mock/benches/ema_axis_h_n64_findings.md excerpt` | 325 | 46 |
| `mock/benches/ema_formulation_n1024_findings.md excerpt` | 319 | 46 |
| `mock/benches/ema_formulation_n16384_findings.md excerpt` | 317 | 46 |
| `mock/benches/ema_formulation_n256_findings.md excerpt` | 322 | 46 |
| `mock/benches/ema_formulation_n4096_findings.md excerpt` | 319 | 46 |
| `mock/benches/float_width_n1024_findings.md excerpt` | 320 | 46 |
| `mock/benches/float_width_n16384_findings.md excerpt` | 319 | 46 |
| `mock/benches/float_width_n256_findings.md excerpt` | 318 | 46 |
| `mock/benches/float_width_n4096_findings.md excerpt` | 320 | 46 |
| `mock/benches/fnptr_table_dispatch_n1024_findings.md excerpt` | 286 | 46 |
| `mock/benches/fnptr_table_dispatch_n16384_findings.md excerpt` | 286 | 46 |
| `mock/benches/fnptr_table_dispatch_n256_findings.md excerpt` | 286 | 46 |
| `mock/benches/fnptr_table_dispatch_n4096_findings.md excerpt` | 287 | 46 |
| `mock/benches/fold_strategy_n1024_findings.md excerpt` | 319 | 46 |
| `mock/benches/fold_strategy_n16384_findings.md excerpt` | 318 | 46 |
| `mock/benches/fold_strategy_n256_findings.md excerpt` | 322 | 46 |
| `mock/benches/fold_strategy_n4096_findings.md excerpt` | 319 | 46 |
| `mock/benches/fxpmul_strategy_n1024_findings.md excerpt` | 286 | 46 |
| `mock/benches/fxpmul_strategy_n16384_findings.md excerpt` | 282 | 46 |
| `mock/benches/fxpmul_strategy_n256_findings.md excerpt` | 287 | 46 |
| `mock/benches/fxpmul_strategy_n4096_findings.md excerpt` | 282 | 46 |
| `mock/benches/fxpmul_strategy_n64_findings.md excerpt` | 289 | 46 |
| `mock/benches/hash_algos_n1024_findings.md excerpt` | 317 | 46 |
| `mock/benches/hash_algos_n16384_findings.md excerpt` | 317 | 46 |
| `mock/benches/hash_algos_n256_findings.md excerpt` | 319 | 46 |
| `mock/benches/hash_algos_n4096_findings.md excerpt` | 317 | 46 |
| `mock/benches/hash_algos_n64_findings.md excerpt` | 321 | 46 |
| `mock/benches/inline_strategy_n1024_findings.md excerpt` | 320 | 46 |
| `mock/benches/inline_strategy_n16384_findings.md excerpt` | 317 | 46 |
| `mock/benches/inline_strategy_n256_findings.md excerpt` | 319 | 46 |
| `mock/benches/inline_strategy_n4096_findings.md excerpt` | 318 | 46 |
| `mock/benches/iter_patterns_n1024_findings.md excerpt` | 322 | 46 |
| `mock/benches/iter_patterns_n16384_findings.md excerpt` | 319 | 46 |
| `mock/benches/iter_patterns_n256_findings.md excerpt` | 324 | 46 |
| `mock/benches/iter_patterns_n4096_findings.md excerpt` | 318 | 46 |
| `mock/benches/load_alignment_n1024_findings.md excerpt` | 318 | 46 |
| `mock/benches/load_alignment_n16384_findings.md excerpt` | 318 | 46 |
| `mock/benches/load_alignment_n256_findings.md excerpt` | 322 | 46 |
| `mock/benches/load_alignment_n4096_findings.md excerpt` | 318 | 46 |
| `mock/benches/match_arm_count_n1024_findings.md excerpt` | 318 | 46 |
| `mock/benches/match_arm_count_n16384_findings.md excerpt` | 319 | 46 |
| `mock/benches/match_arm_count_n256_findings.md excerpt` | 321 | 46 |
| `mock/benches/match_arm_count_n4096_findings.md excerpt` | 318 | 46 |
| `mock/benches/memcpy_patterns_n1024_findings.md excerpt` | 318 | 46 |
| `mock/benches/memcpy_patterns_n16384_findings.md excerpt` | 316 | 46 |
| `mock/benches/memcpy_patterns_n256_findings.md excerpt` | 317 | 46 |
| `mock/benches/memcpy_patterns_n4096_findings.md excerpt` | 317 | 46 |
| `mock/benches/memcpy_patterns_n64_findings.md excerpt` | 318 | 46 |
| `mock/benches/minmax_strategy_n1024_findings.md excerpt` | 313 | 46 |
| `mock/benches/minmax_strategy_n16384_findings.md excerpt` | 319 | 46 |
| `mock/benches/minmax_strategy_n256_findings.md excerpt` | 321 | 46 |
| `mock/benches/minmax_strategy_n4096_findings.md excerpt` | 313 | 46 |
| `mock/benches/modulo_strategy_n1024_findings.md excerpt` | 320 | 46 |
| `mock/benches/modulo_strategy_n16384_findings.md excerpt` | 320 | 46 |
| `mock/benches/modulo_strategy_n256_findings.md excerpt` | 321 | 46 |
| `mock/benches/modulo_strategy_n4096_findings.md excerpt` | 319 | 46 |
| `mock/benches/popcount_strategy_n1024_findings.md excerpt` | 317 | 46 |
| `mock/benches/popcount_strategy_n16384_findings.md excerpt` | 320 | 46 |
| `mock/benches/popcount_strategy_n256_findings.md excerpt` | 318 | 46 |
| `mock/benches/popcount_strategy_n4096_findings.md excerpt` | 320 | 46 |
| `mock/benches/popcount_strategy_n64_findings.md excerpt` | 323 | 46 |
| `mock/benches/rotate_strategy_n1024_findings.md excerpt` | 317 | 46 |
| `mock/benches/rotate_strategy_n16384_findings.md excerpt` | 319 | 46 |
| `mock/benches/rotate_strategy_n256_findings.md excerpt` | 320 | 46 |
| `mock/benches/rotate_strategy_n4096_findings.md excerpt` | 319 | 46 |
| `mock/benches/rsb_bwide_n16384_findings.md excerpt` | 246 | 46 |
| `mock/benches/rsb_bwide_n256_findings.md excerpt` | 246 | 46 |
| `mock/benches/rsb_bwide_n4194304_findings.md excerpt` | 249 | 46 |
| `mock/benches/rsb_clean_n1024_findings.md excerpt` | 376 | 46 |
| `mock/benches/rsb_clean_n16384_findings.md excerpt` | 377 | 46 |
| `mock/benches/rsb_clean_n262144_findings.md excerpt` | 378 | 46 |
| `mock/benches/rsb_heavy_thrash_n1024_findings.md excerpt` | 376 | 46 |
| `mock/benches/rsb_heavy_thrash_n16384_findings.md excerpt` | 381 | 46 |
| `mock/benches/rsb_heavy_thrash_n262144_findings.md excerpt` | 381 | 46 |
| `mock/benches/rsb_light_thrash_n1024_findings.md excerpt` | 378 | 46 |
| `mock/benches/rsb_light_thrash_n16384_findings.md excerpt` | 378 | 46 |
| `mock/benches/rsb_light_thrash_n262144_findings.md excerpt` | 378 | 46 |
| `mock/benches/rsb_seqd_n1048576_findings.md excerpt` | 247 | 46 |
| `mock/benches/rsb_seqd_n16777216_findings.md excerpt` | 249 | 46 |
| `mock/benches/rsb_seqd_n4194304_findings.md excerpt` | 247 | 46 |
| `mock/benches/rsb_seqd_n65536_findings.md excerpt` | 247 | 46 |
| `mock/benches/runs/run1/rsb_bwide_n16384_findings.md excerpt` | 249 | 46 |
| `mock/benches/runs/run1/rsb_bwide_n256_findings.md excerpt` | 247 | 46 |
| `mock/benches/runs/run1/rsb_bwide_n4194304_findings.md excerpt` | 248 | 46 |
| `mock/benches/runs/run1/rsb_clean_n1024_findings.md excerpt` | 379 | 46 |
| `mock/benches/runs/run1/rsb_clean_n16384_findings.md excerpt` | 376 | 46 |
| `mock/benches/runs/run1/rsb_clean_n262144_findings.md excerpt` | 375 | 46 |
| `mock/benches/runs/run1/rsb_heavy_thrash_n1024_findings.md excerpt` | 381 | 46 |
| `mock/benches/runs/run1/rsb_heavy_thrash_n16384_findings.md excerpt` | 381 | 46 |
| `mock/benches/runs/run1/rsb_heavy_thrash_n262144_findings.md excerpt` | 380 | 46 |
| `mock/benches/runs/run1/rsb_light_thrash_n1024_findings.md excerpt` | 375 | 46 |
| `mock/benches/runs/run1/rsb_light_thrash_n16384_findings.md excerpt` | 376 | 46 |
| `mock/benches/runs/run1/rsb_light_thrash_n262144_findings.md excerpt` | 386 | 46 |
| `mock/benches/runs/run1/rsb_seqd_n1048576_findings.md excerpt` | 247 | 46 |
| `mock/benches/runs/run1/rsb_seqd_n16777216_findings.md excerpt` | 248 | 46 |
| `mock/benches/runs/run1/rsb_seqd_n4194304_findings.md excerpt` | 248 | 46 |
| `mock/benches/runs/run1/rsb_seqd_n65536_findings.md excerpt` | 247 | 46 |
| `mock/benches/runs/run2/rsb_bwide_n16384_findings.md excerpt` | 246 | 46 |
| `mock/benches/runs/run2/rsb_bwide_n256_findings.md excerpt` | 246 | 46 |
| `mock/benches/runs/run2/rsb_bwide_n4194304_findings.md excerpt` | 249 | 46 |
| `mock/benches/runs/run2/rsb_clean_n1024_findings.md excerpt` | 376 | 46 |
| `mock/benches/runs/run2/rsb_clean_n16384_findings.md excerpt` | 377 | 46 |
| `mock/benches/runs/run2/rsb_clean_n262144_findings.md excerpt` | 378 | 46 |
| `mock/benches/runs/run2/rsb_heavy_thrash_n1024_findings.md excerpt` | 376 | 46 |
| `mock/benches/runs/run2/rsb_heavy_thrash_n16384_findings.md excerpt` | 381 | 46 |
| `mock/benches/runs/run2/rsb_heavy_thrash_n262144_findings.md excerpt` | 381 | 46 |
| `mock/benches/runs/run2/rsb_light_thrash_n1024_findings.md excerpt` | 378 | 46 |
| `mock/benches/runs/run2/rsb_light_thrash_n16384_findings.md excerpt` | 378 | 46 |
| `mock/benches/runs/run2/rsb_light_thrash_n262144_findings.md excerpt` | 378 | 46 |
| `mock/benches/runs/run2/rsb_seqd_n1048576_findings.md excerpt` | 247 | 46 |
| `mock/benches/runs/run2/rsb_seqd_n16777216_findings.md excerpt` | 249 | 46 |
| `mock/benches/runs/run2/rsb_seqd_n4194304_findings.md excerpt` | 247 | 46 |
| `mock/benches/runs/run2/rsb_seqd_n65536_findings.md excerpt` | 247 | 46 |
| `mock/benches/sat_arithmetic_n1024_findings.md excerpt` | 318 | 46 |
| `mock/benches/sat_arithmetic_n16384_findings.md excerpt` | 320 | 46 |
| `mock/benches/sat_arithmetic_n256_findings.md excerpt` | 319 | 46 |
| `mock/benches/sat_arithmetic_n4096_findings.md excerpt` | 320 | 46 |
| `mock/benches/shift_amount_origin_n1024_findings.md excerpt` | 320 | 46 |
| `mock/benches/shift_amount_origin_n16384_findings.md excerpt` | 319 | 46 |
| `mock/benches/shift_amount_origin_n256_findings.md excerpt` | 324 | 46 |
| `mock/benches/shift_amount_origin_n4096_findings.md excerpt` | 317 | 46 |
| `mock/design_rounds/202605290312-abandoned/202605290312_topic.plan-csr-adapter.md` | 43 | 20 |
| `mock/design_rounds/202605291450/202605291450_changelist.doc.lock.md` | 5 | 20 |
| `mock/design_rounds/202605291450/202605291450_changelist.src.lock.md` | 15 | 20 |
| `mock/design_rounds/202605291450/202605291450_topic.enable-notko-try-trait-v2.md` | 16 | 20 |
| `mock/design_rounds/202605291700/202605291007_topic.unstable-feature-soundness-sweep.md` | 48 | 20 |
| `mock/design_rounds/202605291700/202605291700_changelist.doc.lock.md` | 22 | 20 |
| `mock/design_rounds/202605291700/202605291701_changelist.src.lock.md` | 90 | 20 |
| `mock/design_rounds/202605291730/202605291730_changelist.doc.lock.md` | 8 | 20 |
| `mock/design_rounds/202605291730/202605291730_topic.engine-ctx-multi-column-test.md` | 16 | 20 |
| `mock/design_rounds/202605291730/202605291731_changelist.src.lock.md` | 16 | 20 |
| `mock/design_rounds/202605291800/202605291800_changelist.doc.lock.md` | 9 | 20 |
| `mock/design_rounds/202605291800/202605291800_topic.remove-associated-type-defaults.md` | 20 | 20 |
| `mock/design_rounds/202605291800/202605291801_changelist.src.lock.md` | 17 | 20 |
| `mock/design_rounds/202605291823/202605291823_changelist.doc.lock.md` | 19 | 20 |
| `mock/design_rounds/202605291823/202605291823_changelist.src.lock.md` | 27 | 20 |
| `mock/design_rounds/202605291823/202605291823_topic.drop-redundant-min-spec-gate.md` | 50 | 20 |
| `mock/design_rounds/202605291830/202605291830_changelist.doc.lock.md` | 8 | 20 |
| `mock/design_rounds/202605291830/202605291830_topic.scope-gce-gates.md` | 20 | 20 |
| `mock/design_rounds/202605291830/202605291831_changelist.src.lock.md` | 25 | 20 |
| `mock/design_rounds/202605291835/202605291835_changelist.doc.lock.md` | 17 | 20 |
| `mock/design_rounds/202605291835/202605291835_changelist.src.lock.md` | 30 | 20 |
| `mock/design_rounds/202605291835/202605291835_topic.fix-trybuild-goldens.md` | 50 | 20 |
| `mock/design_rounds/202605291845/202605291845_changelist.doc.lock.md` | 73 | 20 |
| `mock/design_rounds/202605291845/202605291845_changelist.src.lock.md` | 66 | 20 |
| `mock/design_rounds/202605291845/202605291845_topic.lift-plan-dims-to-cap.md` | 67 | 20 |
| `mock/design_rounds/202605292030/202605292030_changelist.doc.lock.md` | 22 | 20 |
| `mock/design_rounds/202605292030/202605292030_changelist.src.lock.md` | 18 | 20 |
| `mock/design_rounds/202605292030/202605292030_topic.drop-unused-adt-const-params-test-gate.md` | 26 | 20 |
| `mock/design_rounds/202605292215/202605292215_changelist.doc.lock.md` | 28 | 20 |
| `mock/design_rounds/202605292215/202605292215_changelist.src.lock.md` | 69 | 20 |
| `mock/design_rounds/202605292215/202605292215_topic.csr-bidirectional-adapter.md` | 51 | 20 |
| `mock/design_rounds/202605292257/202605292257_changelist.doc.lock.md` | 15 | 20 |
| `mock/design_rounds/202605292257/202605292257_changelist.src.lock.md` | 27 | 20 |
| `mock/design_rounds/202605292257/202605292257_topic.plan-rcm-reorder-wiring.md` | 20 | 20 |
| `mock/design_rounds/202605292352/202605292352_changelist.doc.lock.md` | 15 | 20 |
| `mock/design_rounds/202605292352/202605292352_changelist.src.lock.md` | 31 | 20 |
| `mock/design_rounds/202605292352/202605292352_topic.plan-block-diagonal-trunks.md` | 22 | 20 |
| `mock/design_rounds/202605300006/202605300006_changelist.doc.lock.md` | 13 | 20 |
| `mock/design_rounds/202605300006/202605300006_changelist.src.lock.md` | 15 | 20 |
| `mock/design_rounds/202605300006/202605300006_topic.c1c-post-review-fixes.md` | 21 | 20 |
| `mock/design_rounds/202605300120/202605300120_changelist.doc.lock.md` | 15 | 20 |
| `mock/design_rounds/202605300120/202605300120_changelist.src.lock.md` | 23 | 20 |
| `mock/design_rounds/202605300120/202605300120_topic.plan-spectral-fibers.md` | 18 | 20 |
| `mock/design_rounds/202605300321/202605300321_changelist.doc.lock.md` | 18 | 20 |
| `mock/design_rounds/202605300321/202605300321_changelist.src.lock.md` | 32 | 20 |
| `mock/design_rounds/202605300321/202605300321_topic.fiber-trunk-projection.md` | 24 | 20 |
| `mock/design_rounds/202605300351/202605300351_changelist.doc.lock.md` | 18 | 20 |
| `mock/design_rounds/202605300351/202605300351_changelist.src.lock.md` | 32 | 20 |
| `mock/design_rounds/202605300351/202605300351_topic.spectral-width-gate.md` | 18 | 20 |
| `mock/design_rounds/202605300413/202605300413_changelist.doc.lock.md` | 8 | 20 |
| `mock/design_rounds/202605300413/202605300413_changelist.src.lock.md` | 8 | 20 |
| `mock/design_rounds/202605300413/202605300413_topic.laplacian-doc-fix.md` | 12 | 20 |
| `mock/design_rounds/202605300651/202605300651_changelist.doc.lock.md` | 17 | 20 |
| `mock/design_rounds/202605300651/202605300651_changelist.src.lock.md` | 52 | 20 |
| `mock/design_rounds/202605300651/202605300651_topic.plan-inputs-from-bundle.md` | 30 | 20 |
| `mock/design_rounds/202605300720/202605300720_changelist.doc.lock.md` | 9 | 20 |
| `mock/design_rounds/202605300720/202605300720_changelist.src.lock.md` | 15 | 20 |
| `mock/design_rounds/202605300720/202605300720_topic.plan-inputs-projection-fixes.md` | 14 | 20 |
| `mock/design_rounds/202605300858/202605300858_changelist.doc.lock.md` | 18 | 20 |
| `mock/design_rounds/202605300858/202605300858_changelist.src.lock.md` | 37 | 20 |
| `mock/design_rounds/202605300858/202605300858_topic.c2-run-fiber-walk.md` | 24 | 20 |
| `mock/design_rounds/202605300957/202605300957_changelist.doc.lock.md` | 48 | 20 |
| `mock/design_rounds/202605300957/202605300957_changelist.src.lock.md` | 57 | 20 |
| `mock/design_rounds/202605300957/202605300957_topic.c2-build-captures-wus.md` | 96 | 20 |
| `mock/design_rounds/202605301058-abandoned/202605301058_changelist.doc.lock.md` | 54 | 20 |
| `mock/design_rounds/202605301058-abandoned/202605301058_changelist.src.md` | 117 | 20 |
| `mock/design_rounds/202605301058-abandoned/202605301058_topic.c2-topo-dispatch.md` | 104 | 20 |
| `mock/design_rounds/202605301730/202605301730_changelist.doc.lock.md` | 52 | 20 |
| `mock/design_rounds/202605301730/202605301730_changelist.src.lock.md` | 71 | 20 |
| `mock/design_rounds/202605301730/202605301730_topic.capacity-plandims-adoption.md` | 79 | 20 |
| `mock/design_rounds/202605301800/202605301800_changelist.doc.lock.md` | 7 | 20 |
| `mock/design_rounds/202605301800/202605301800_changelist.src.lock.md` | 17 | 20 |
| `mock/design_rounds/202605301800/202605301800_topic.plandims-review-fixes.md` | 12 | 20 |
| `mock/design_rounds/202605301830-abandoned/202605301830_changelist.doc.lock.md` | 33 | 20 |
| `mock/design_rounds/202605301830-abandoned/202605301830_changelist.src.md` | 64 | 20 |
| `mock/design_rounds/202605301830-abandoned/202605301830_topic.c2-topo-dispatch.md` | 40 | 20 |
| `mock/design_rounds/202605302100/202605302100_changelist.doc.lock.md` | 98 | 20 |
| `mock/design_rounds/202605302100/202605302100_changelist.src.lock.md` | 36 | 20 |
| `mock/design_rounds/202605302100/202605302100_topic.unified-columnar-storage.md` | 79 | 20 |
| `mock/design_rounds/202605302200/202605302200_changelist.doc.lock.md` | 20 | 20 |
| `mock/design_rounds/202605302200/202605302200_changelist.src.lock.md` | 34 | 20 |
| `mock/design_rounds/202605302200/202605302200_topic.naive-arena-columnstorage.md` | 23 | 20 |
| `mock/design_rounds/202605302300/202605302300_changelist.doc.lock.md` | 16 | 20 |
| `mock/design_rounds/202605302300/202605302300_changelist.src.lock.md` | 20 | 20 |
| `mock/design_rounds/202605302300/202605302300_topic.columnstorage-overflow-fix.md` | 15 | 20 |
| `mock/design_rounds/202605302331/202605302330_changelist.doc.deprecated.md` | 25 | 20 |
| `mock/design_rounds/202605302331/202605302330_topic.store-backed-plan.md` | 48 | 20 |
| `mock/design_rounds/202605302331/202605302331_changelist.doc.lock.md` | 38 | 20 |
| `mock/design_rounds/202605302331/202605302332_changelist.src.lock.md` | 60 | 20 |
| `mock/design_rounds/202605310633/202605310633_changelist.doc.lock.md` | 45 | 20 |
| `mock/design_rounds/202605310633/202605310633_changelist.src.lock.md` | 118 | 20 |
| `mock/design_rounds/202605310633/202605310633_topic.resource-arena-columnstorage.md` | 106 | 20 |
| `mock/design_rounds/202605310847/202605310847_changelist.doc.lock.md` | 33 | 20 |
| `mock/design_rounds/202605310847/202605310847_changelist.src.lock.md` | 70 | 20 |
| `mock/design_rounds/202605310847/202605310847_topic.build-dag-order-independent-raw.md` | 103 | 20 |
| `mock/design_rounds/202605311030/202605311030_changelist.doc.lock.md` | 56 | 20 |
| `mock/design_rounds/202605311030/202605311030_changelist.src.lock.md` | 101 | 20 |
| `mock/design_rounds/202605311030/202605311030_topic.topo-dispatch-wiring.md` | 74 | 20 |
| `mock/design_rounds/202605311130/202605311130_changelist.doc.lock.md` | 39 | 20 |
| `mock/design_rounds/202605311130/202605311130_changelist.src.lock.md` | 70 | 20 |
| `mock/design_rounds/202605311130/202605311130_topic.store-back-plan.md` | 74 | 20 |
| `mock/design_rounds/202606041529/202606041529_changelist.doc.lock.md` | 11 | 20 |
| `mock/design_rounds/202606041529/202606041529_changelist.src.lock.md` | 29 | 20 |
| `mock/design_rounds/202606041529/202606041529_topic.accumulator-append-saturates-at-capacity.md` | 27 | 20 |
| `mock/design_rounds/202606041537/202606041537_changelist.doc.lock.md` | 28 | 20 |
| `mock/design_rounds/202606041537/202606041537_changelist.src.lock.md` | 61 | 20 |
| `mock/design_rounds/202606041537/202606041537_topic.rename-resource-arena-to-bindings.md` | 54 | 20 |
| `mock/design_rounds/202606041609/202606041609_changelist.doc.lock.md` | 11 | 20 |
| `mock/design_rounds/202606041609/202606041609_changelist.src.lock.md` | 31 | 20 |
| `mock/design_rounds/202606041609/202606041609_topic.morsel-loop-windowing.md` | 52 | 20 |
| `mock/design_rounds/202606041624/202606041624_changelist.doc.lock.md` | 7 | 20 |
| `mock/design_rounds/202606041624/202606041624_changelist.src.lock.md` | 26 | 20 |
| `mock/design_rounds/202606041624/202606041624_topic.morsel-dispatch-doc-contract.md` | 23 | 20 |
| `mock/design_rounds/202606041631/202606041631_changelist.doc.lock.md` | 60 | 20 |
| `mock/design_rounds/202606041631/202606041631_changelist.src.lock.md` | 100 | 20 |
| `mock/design_rounds/202606041631/202606041631_topic.column-data-plane.md` | 44 | 20 |
| `mock/design_rounds/202606041643/202606041643_changelist.doc.lock.md` | 11 | 20 |
| `mock/design_rounds/202606041643/202606041643_changelist.src.lock.md` | 26 | 20 |
| `mock/design_rounds/202606041643/202606041643_topic.phase-sequential-dispatch.md` | 42 | 20 |
| `mock/design_rounds/202606041720/202606041720_changelist.doc.lock.md` | 14 | 20 |
| `mock/design_rounds/202606041720/202606041720_changelist.src.lock.md` | 44 | 20 |
| `mock/design_rounds/202606041720/202606041720_topic.morsel-relative-iteration.md` | 23 | 20 |
| `mock/design_rounds/202606041800/202606041800_changelist.doc.lock.md` | 24 | 20 |
| `mock/design_rounds/202606041800/202606041800_changelist.src.lock.md` | 50 | 20 |
| `mock/design_rounds/202606041800/202606041800_topic.accumulator-columns.md` | 45 | 20 |
| `mock/design_rounds/202606041802/202606041802_changelist.doc.lock.md` | 7 | 20 |
| `mock/design_rounds/202606041802/202606041802_changelist.src.lock.md` | 25 | 20 |
| `mock/design_rounds/202606041802/202606041802_topic.plan-projection-hardening.md` | 28 | 20 |
| `mock/design_rounds/202606041836/202606041836_changelist.doc.lock.md` | 19 | 20 |
| `mock/design_rounds/202606041836/202606041836_changelist.src.lock.md` | 34 | 20 |
| `mock/design_rounds/202606041836/202606041836_topic.plan-dims-id-width-guard.md` | 33 | 20 |
| `mock/design_rounds/202606041903/202606041903_changelist.doc.lock.md` | 11 | 20 |
| `mock/design_rounds/202606041903/202606041903_changelist.src.lock.md` | 27 | 20 |
| `mock/design_rounds/202606041903/202606041903_topic.resource-arena-partial-failure.md` | 25 | 20 |
| `mock/design_rounds/202606050220/202606050203_topic.single-core-dispatch-completion.md` | 47 | 20 |
| `mock/design_rounds/202606050220/202606050220_changelist.doc.lock.md` | 11 | 20 |
| `mock/design_rounds/202606050220/202606050220_changelist.src.lock.md` | 33 | 20 |
| `mock/design_rounds/202606050259/202606050259_changelist.doc.lock.md` | 7 | 20 |
| `mock/design_rounds/202606050259/202606050259_changelist.src.lock.md` | 17 | 20 |
| `mock/design_rounds/202606050259/202606050259_topic.morsel-outer-postreview.md` | 14 | 20 |
| `mock/design_rounds/202606051215/202606051215_changelist.doc.lock.md` | 7 | 20 |
| `mock/design_rounds/202606051215/202606051215_changelist.src.lock.md` | 16 | 20 |
| `mock/design_rounds/202606051215/202606051215_topic.real-waist-detect.md` | 19 | 20 |
| `mock/design_rounds/202606051500/202606051500_changelist.doc.lock.md` | 23 | 20 |
| `mock/design_rounds/202606051500/202606051500_changelist.src.lock.md` | 37 | 20 |
| `mock/design_rounds/202606051500/202606051500_topic.fiber-structured-morsel-outer-dispatch.md` | 43 | 20 |
| `mock/design_rounds/202606051600/202606051600_changelist.doc.lock.md` | 11 | 20 |
| `mock/design_rounds/202606051600/202606051600_changelist.src.lock.md` | 17 | 20 |
| `mock/design_rounds/202606051600/202606051600_topic.dispatch-post-review-cleanup.md` | 12 | 20 |
| `mock/design_rounds/202606051659/202606051554_topic.dispatch-codegen-fusion.md` | 33 | 20 |
| `mock/design_rounds/202606051659/202606051659_changelist.doc.lock.md` | 60 | 20 |
| `mock/design_rounds/202606051659/202606051733_changelist.src.deprecated.md` | 50 | 20 |
| `mock/design_rounds/202606051659/202606062330_changelist.src.lock.md` | 85 | 20 |
| `mock/design_rounds/202606052250/202606052250_changelist.doc.lock.md` | 21 | 20 |
| `mock/design_rounds/202606052250/202606052250_changelist.src.lock.md` | 20 | 20 |
| `mock/design_rounds/202606052250/202606052250_topic.arvo-bump-waist-detect.md` | 24 | 21 |
| `mock/design_rounds/202606061446/202606061446_changelist.doc.lock.md` | 20 | 21 |
| `mock/design_rounds/202606061446/202606061446_changelist.src.lock.md` | 33 | 21 |
| `mock/design_rounds/202606061446/202606061446_topic.gate1-dispatch-builder-append.md` | 33 | 21 |
| `mock/design_rounds/202606061730/202606061730_changelist.doc.lock.md` | 48 | 21 |
| `mock/design_rounds/202606061730/202606061730_changelist.src.lock.md` | 79 | 21 |
| `mock/design_rounds/202606061730/202606061730_topic.gate1-dispatch-topo-registration-gate.md` | 70 | 21 |
| `mock/design_rounds/202606061830/202606061830_changelist.doc.lock.md` | 34 | 21 |
| `mock/design_rounds/202606061830/202606061830_changelist.src.lock.md` | 54 | 21 |
| `mock/design_rounds/202606061830/202606061830_topic.gate1-dispatch-recommended-order.md` | 54 | 21 |
| `mock/design_rounds/202606062047/202606062047_changelist.doc.lock.md` | 29 | 21 |
| `mock/design_rounds/202606062047/202606062047_changelist.src.lock.md` | 51 | 21 |
| `mock/design_rounds/202606062047/202606062047_topic.d668-engine-adjrow-cap-lift.md` | 79 | 21 |
| `mock/design_rounds/202606070048/202606070048_changelist.doc.lock.md` | 28 | 21 |
| `mock/design_rounds/202606070048/202606070048_changelist.src.lock.md` | 31 | 21 |
| `mock/design_rounds/202606070048/202606070048_topic.gate2-g2-0a-runtrunk-fibercons.md` | 28 | 21 |
| `mock/design_rounds/202606070100/202606070100_changelist.doc.lock.md` | 28 | 21 |
| `mock/design_rounds/202606070100/202606070100_changelist.src.lock.md` | 31 | 21 |
| `mock/design_rounds/202606070100/202606070100_topic.gate2-g2-0b-runphase-waist.md` | 30 | 21 |
| `mock/design_rounds/202606070839/202606070839_changelist.doc.lock.md` | 19 | 21 |
| `mock/design_rounds/202606070839/202606070839_changelist.src.lock.md` | 43 | 21 |
| `mock/design_rounds/202606070839/202606070839_topic.gate2-waist-bounded-phase.md` | 69 | 21 |
| `mock/design_rounds/202606070900/202606070900_changelist.doc.lock.md` | 18 | 21 |
| `mock/design_rounds/202606070900/202606070900_changelist.src.lock.md` | 33 | 21 |
| `mock/design_rounds/202606070900/202606070900_topic.gate2-const-grouping.md` | 34 | 21 |
| `mock/design_rounds/202606071100/202606071100_changelist.doc.lock.md` | 18 | 21 |
| `mock/design_rounds/202606071100/202606071100_changelist.src.lock.md` | 36 | 21 |
| `mock/design_rounds/202606071100/202606071100_topic.gate2-const-gated-dispatch.md` | 45 | 21 |
| `mock/design_rounds/202606071109/202606071109_changelist.doc.lock.md` | 11 | 21 |
| `mock/design_rounds/202606071109/202606071109_changelist.src.lock.md` | 22 | 21 |
| `mock/design_rounds/202606071109/202606071109_topic.gate2-r4a-core-phase-masks.md` | 69 | 21 |
| `mock/design_rounds/202606071139/202606071139_changelist.doc.lock.md` | 30 | 21 |
| `mock/design_rounds/202606071139/202606071139_changelist.src.lock.md` | 35 | 21 |
| `mock/design_rounds/202606071139/202606071139_topic.gate2-r4b-run-parallel-inline.md` | 61 | 21 |
| `mock/design_rounds/202606071300-abandoned/202606071300_changelist.doc.lock.md` | 14 | 21 |
| `mock/design_rounds/202606071300-abandoned/202606071300_changelist.src.md` | 26 | 21 |
| `mock/design_rounds/202606071300-abandoned/202606071300_topic.gate2-phase-sectioned-run.md` | 36 | 21 |
| `mock/design_rounds/202606071730/202606071730_changelist.doc.lock.md` | 20 | 21 |
| `mock/design_rounds/202606071730/202606071730_changelist.src.lock.md` | 38 | 21 |
| `mock/design_rounds/202606071730/202606071730_topic.gate2-r4c-osthreadpool-spawn.md` | 63 | 21 |
| `mock/design_rounds/202606071808/202606071805_topic.gate2-worker-side-waist-barrier.md` | 65 | 21 |
| `mock/design_rounds/202606071808/202606071808_changelist.doc.lock.md` | 17 | 21 |
| `mock/design_rounds/202606071808/202606071810_changelist.src.lock.md` | 35 | 21 |
| `mock/design_rounds/202606071856/202606071834_topic.gate2-head-tail-convergence.md` | 39 | 21 |
| `mock/design_rounds/202606071856/202606071856_changelist.doc.lock.md` | 15 | 21 |
| `mock/design_rounds/202606071856/202606071856_changelist.src.lock.md` | 27 | 21 |
| `mock/design_rounds/202606071900/202606071900_changelist.doc.lock.md` | 19 | 21 |
| `mock/design_rounds/202606071900/202606071900_changelist.src.lock.md` | 34 | 21 |
| `mock/design_rounds/202606071900/202606071900_topic.gate2-r4c-frame-protocol.md` | 64 | 21 |
| `mock/design_rounds/202606072000-2/202606071933_topic.gate2-accumulator-threaded.md` | 38 | 21 |
| `mock/design_rounds/202606072000-2/202606072000_changelist.doc.lock.md` | 60 | 21 |
| `mock/design_rounds/202606072000-2/202606072010_changelist.src.lock.md` | 92 | 21 |
| `mock/design_rounds/202606072000/202606072000_changelist.doc.lock.md` | 16 | 21 |
| `mock/design_rounds/202606072000/202606072000_changelist.src.lock.md` | 23 | 21 |
| `mock/design_rounds/202606072000/202606072000_topic.gate2-r4c-run-core-phase.md` | 53 | 21 |
| `mock/design_rounds/202606072030/202606072030_changelist.doc.lock.md` | 18 | 21 |
| `mock/design_rounds/202606072030/202606072030_changelist.src.lock.md` | 42 | 21 |
| `mock/design_rounds/202606072030/202606072030_topic.gate2-r4c-threaded-run-parallel.md` | 61 | 21 |
| `mock/design_rounds/202606080400/202606080100_topic.gate2-ge-const-gated-trunk-dispatch.md` | 39 | 21 |
| `mock/design_rounds/202606080400/202606080300_topic.gate2-ge-drive-shipped-rungatedtrunk.md` | 41 | 21 |
| `mock/design_rounds/202606080400/202606080400_changelist.doc.lock.md` | 56 | 21 |
| `mock/design_rounds/202606080400/202606080500_changelist.src.lock.md` | 56 | 21 |
| `mock/design_rounds/202606080700/202606080600_topic.gate2-n-core-pinned-trunk-dispatch.md` | 36 | 21 |
| `mock/design_rounds/202606080700/202606080700_changelist.doc.lock.md` | 48 | 21 |
| `mock/design_rounds/202606080700/202606080800_changelist.src.lock.md` | 48 | 21 |
| `mock/design_rounds/202606081300/202606081200_topic.gate2-e4-virtual-firing-and-on-gating.md` | 41 | 21 |
| `mock/design_rounds/202606081300/202606081300_changelist.doc.lock.md` | 30 | 21 |
| `mock/design_rounds/202606081300/202606081300_changelist.src.lock.md` | 71 | 21 |
| `mock/design_rounds/202606082100/202606081900_topic.gate2-e4-slice2-self-hosting-meta-pipeline.md` | 60 | 21 |
| `mock/design_rounds/202606082100/202606082100_changelist.doc.lock.md` | 115 | 21 |
| `mock/design_rounds/202606082100/202606082100_changelist.src.lock.md` | 119 | 21 |
| `mock/design_rounds/202606090400/202606090000_topic.gate2-e4-slice3-consumer-meta-hooks.md` | 142 | 21 |
| `mock/design_rounds/202606090400/202606090200_changelist.doc.deprecated.md` | 73 | 21 |
| `mock/design_rounds/202606090400/202606090200_changelist.src.deprecated.md` | 54 | 21 |
| `mock/design_rounds/202606090400/202606090400_changelist.doc.lock.md` | 75 | 21 |
| `mock/design_rounds/202606090400/202606090400_changelist.src.deprecated.md` | 68 | 21 |
| `mock/design_rounds/202606090400/202606090500_changelist.src.lock.md` | 89 | 21 |
| `mock/design_rounds/202606091000/202606091000_changelist.doc.lock.md` | 26 | 21 |
| `mock/design_rounds/202606091000/202606091000_changelist.src.lock.md` | 42 | 21 |
| `mock/design_rounds/202606091000/202606091000_topic.d1e-runfiber-dispatch.md` | 37 | 21 |
| `mock/design_rounds/202606091600/202606091600_changelist.doc.lock.md` | 20 | 21 |
| `mock/design_rounds/202606091600/202606091600_changelist.src.lock.md` | 29 | 21 |
| `mock/design_rounds/202606091600/202606091600_topic.spec-free-columnvalue-drop-full-specialization.md` | 32 | 21 |
| `mock/design_rounds/202606092000/202606092000_changelist.doc.lock.md` | 19 | 21 |
| `mock/design_rounds/202606092000/202606092000_changelist.src.lock.md` | 38 | 21 |
| `mock/design_rounds/202606092000/202606092000_topic.d4-within-fiber-linear-fusion.md` | 51 | 21 |
| `mock/design_rounds/202606100900/202606100900_changelist.doc.lock.md` | 23 | 21 |
| `mock/design_rounds/202606100900/202606100900_changelist.src.lock.md` | 36 | 21 |
| `mock/design_rounds/202606100900/202606100900_topic.accum-per-frame-reset.md` | 29 | 21 |
| `mock/design_rounds/202606110910/202606110855_topic.gate2-e4-run-parallel-meta-parity.md` | 113 | 21 |
| `mock/design_rounds/202606110910/202606110910_changelist.doc.lock.md` | 56 | 21 |
| `mock/design_rounds/202606110910/202606110920_changelist.src.lock.md` | 122 | 21 |
| `mock/design_rounds/202606110957/202606110948_topic.gate2-e8-adapt-pass-duration-ema.md` | 70 | 21 |
| `mock/design_rounds/202606110957/202606110957_changelist.doc.lock.md` | 46 | 21 |
| `mock/design_rounds/202606110957/202606111010_changelist.src.lock.md` | 132 | 21 |
| `mock/design_rounds/202606111200/202606111200_changelist.doc.lock.md` | 20 | 21 |
| `mock/design_rounds/202606111200/202606111200_changelist.src.lock.md` | 55 | 21 |
| `mock/design_rounds/202606111200/202606111200_topic.e7-dirty-skip-incremental.md` | 28 | 21 |
| `mock/design_rounds/202606111600/202606111600_changelist.doc.lock.md` | 73 | 21 |
| `mock/design_rounds/202606111600/202606111600_changelist.src.lock.md` | 103 | 21 |
| `mock/design_rounds/202606111600/202606111600_topic.d6-asm-dispatch-gate.md` | 121 | 21 |
| `mock/design_rounds/202606111700/202606111700_changelist.doc.lock.md` | 17 | 21 |
| `mock/design_rounds/202606111700/202606111700_changelist.src.lock.md` | 48 | 21 |
| `mock/design_rounds/202606111700/202606111700_topic.gate1-test-audit-remediation.md` | 65 | 21 |
| `mock/design_rounds/202606111800/202606111800_changelist.doc.lock.md` | 12 | 21 |
| `mock/design_rounds/202606111800/202606111800_changelist.src.lock.md` | 31 | 21 |
| `mock/design_rounds/202606111800/202606111800_topic.gate1-readmask-invariant-guard.md` | 32 | 21 |
| `mock/design_rounds/202606190700/202606190700_changelist.doc.lock.md` | 26 | 21 |
| `mock/design_rounds/202606190700/202606190700_changelist.src.lock.md` | 65 | 21 |
| `mock/design_rounds/202606190700/202606190700_topic.core-idle-adapt-axis.md` | 101 | 21 |
| `mock/design_rounds/202606191000/202606191000_changelist.doc.lock.md` | 15 | 21 |
| `mock/design_rounds/202606191000/202606191000_changelist.src.lock.md` | 33 | 21 |
| `mock/design_rounds/202606191000/202606191000_topic.gate2-parallel-aliasing-soundness.md` | 57 | 21 |
| `mock/design_rounds/202606191200/202606191200_changelist.doc.lock.md` | 25 | 21 |
| `mock/design_rounds/202606191200/202606191200_changelist.src.lock.md` | 61 | 21 |
| `mock/design_rounds/202606191200/202606191200_topic.gate2-caps-lifting.md` | 50 | 21 |
| `mock/design_rounds/202606191400/202606191400_changelist.doc.lock.md` | 17 | 21 |
| `mock/research/imported-from-polka-dots/bench/calling-convention-bench-results.md` | 537 | 27 |
| `mock/research/imported-from-polka-dots/build-configuration-research.md` | 282 | 27 |
| `mock/research/imported-from-polka-dots/design_rounds/2026-02-21/2026-02-21_initial-architecture.md` | 78 | 27 |
| `mock/research/imported-from-polka-dots/design_rounds/202603110821/202603110821_changelist.doc.lock.md` | 172 | 27 |
| `mock/research/imported-from-polka-dots/design_rounds/202603110821/202603110821_changelist.src.lock.md` | 93 | 27 |
| `mock/research/imported-from-polka-dots/design_rounds/202603110821/202603110821_topic.architecture-signatures.md` | 191 | 27 |
| `mock/research/imported-from-polka-dots/design_rounds/202603111443/202603111443_topic.core-design-review.md` | 123 | 27 |
| `mock/research/imported-from-polka-dots/design_rounds/202603111443/202603111443_topic.extension-implementation.md` | 125 | 27 |
| `mock/research/imported-from-polka-dots/design_rounds/202603171930_changelist.doc.deprecated.md` | 290 | 27 |
| `mock/research/imported-from-polka-dots/execution-engines-duckdb-blis-polars.md` | 2027 | 27 |
| `mock/research/imported-from-polka-dots/llvm-flags-and-experimental-optimisations.md` | 849 | 27 |
| `mock/research/imported-from-polka-dots/nightly-and-stable-optimisation-config.md` | 1003 | 28 |
| `mock/research/imported-from-polka-dots/rust-nightly-features-for-type-constraints.md` | 1154 | 28 |
| `mock/research/imported-from-saalis/2026-03-14-architecture-review.md` | 1085 | 29 |
| `mock/research/imported-from-saalis/2026-03-14-research.arr-stack-lessons.md` | 667 | 29 |
| `mock/research/imported-from-saalis/2026-03-14-research.data-classification-ux.md` | 500 | 29 |
| `mock/research/imported-from-saalis/2026-03-14-research.filesystem-optimisation.md` | 463 | 29 |
| `mock/research/imported-from-saalis/2026-03-14-research.hot-cold-storage-strategies.md` | 400 | 29 |
| `mock/research/imported-from-saalis/2026-03-14-research.http-batching-and-sourcing.md` | 574 | 29 |
| `mock/research/imported-from-saalis/2026-03-14-research.sqlite-alternatives.md` | 344 | 29 |
| `mock/research/imported-from-saalis/2026-03-14-research.universal-entity-model-gotchas.md` | 422 | 29 |
| `mock/research/imported-from-saalis/2026-03-14-synthesis.appliance-first-design.md` | 406 | 29 |
| `mock/research/imported-from-saalis/2026-03-14-synthesis.columnar-query-engine.md` | 403 | 29 |
| `mock/research/imported-from-saalis/2026-03-14-synthesis.connector-architecture.md` | 346 | 30 |
| `mock/research/imported-from-saalis/2026-03-14-synthesis.content-acquisition-pipeline.md` | 572 | 30 |
| `mock/research/imported-from-saalis/2026-03-14-synthesis.ecosystem-contributions.md` | 332 | 30 |
| `mock/research/imported-from-saalis/2026-03-14-synthesis.extension-ecosystem.md` | 296 | 30 |
| `mock/research/imported-from-saalis/2026-03-14-synthesis.flexible-schema-strategy.md` | 307 | 30 |
| `mock/research/imported-from-saalis/2026-03-14-synthesis.novel-library-intelligence.md` | 413 | 30 |
| `mock/research/imported-from-saalis/2026-03-14-synthesis.persistence-strategy.md` | 431 | 30 |
| `mock/research/imported-from-saalis/2026-03-14-synthesis.progressive-ux-strategy.md` | 368 | 30 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.accessibility.md` | 654 | 30 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.async-sync-bridging.md` | 507 | 30 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.axum-patterns.md` | 928 | 30 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.backup-restore-portability.md` | 646 | 31 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.batocera-internals.md` | 512 | 31 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.concurrency-primitives.md` | 454 | 31 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.content-addressable-storage.md` | 371 | 31 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.cross-compilation-ci.md` | 448 | 31 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.error-handling-patterns.md` | 946 | 31 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.full-text-search.md` | 645 | 31 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.game-frontend-integration.md` | 684 | 31 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.htmx-sse-patterns.md` | 666 | 31 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.i18n-localization.md` | 785 | 32 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.igdb-screenscraper-apis.md` | 690 | 32 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.image-handling.md` | 561 | 32 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.inventory-dlopen-plugins.md` | 537 | 32 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.limbo-sqlite.md` | 415 | 32 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.maud-templates.md` | 632 | 32 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.min-specialization.md` | 393 | 32 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.observability-tracing.md` | 944 | 32 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.plugin-ecosystem-architecture.md` | 323 | 32 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.proc-macro-patterns.md` | 614 | 33 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.retro-gaming-metadata.md` | 634 | 33 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.rom-verification-databases.md` | 320 | 33 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.seaorm-migrations-evaluation.md` | 482 | 33 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.seaorm-vs-rusqlite.md` | 771 | 33 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.security-considerations.md` | 668 | 33 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.serde-patterns.md` | 655 | 33 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.sieve-eviction.md` | 306 | 33 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.testing-strategies.md` | 671 | 33 |
| `mock/research/imported-from-saalis/2026-03-15-deepdive.ureq-http-client.md` | 657 | 34 |
| `mock/research/imported-from-saalis/2026-03-15-synthesis.developer-onboarding.md` | 700 | 34 |
| `mock/research/imported-from-saalis/2026-03-15-synthesis.implementation-stack.md` | 567 | 34 |
| `mock/research/imported-from-saalis/2026-03-19-research.headless-server-client-architecture.md` | 329 | 34 |
| `mock/research/imported-from-saalis/2026-03-19-research.host-crate-apis-from-unfinished-consolidation.md` | 376 | 34 |
| `mock/research/imported-from-saalis/2026-03-28-review.api-designer-decomposition.md` | 408 | 34 |
| `mock/research/imported-from-saalis/2026-03-28-review.protocol-architect-decomposition.md` | 434 | 34 |
| `mock/research/imported-from-saalis/2026-03-28-review.systems-integrator-decomposition.md` | 501 | 34 |
| `mock/research/imported-from-saalis/2026-03-28-revised-taxonomy.md` | 833 | 34 |
| `mock/research/imported-from-saalis/2026-03-28-run10.depanalyst-stage1.md` | 617 | 34 |
| `mock/research/imported-from-saalis/2026-03-28-run10.depanalyst-stage2a.md` | 256 | 35 |
| `mock/research/imported-from-saalis/2026-03-28-run10.depanalyst-stage3a.md` | 248 | 35 |
| `mock/research/imported-from-saalis/2026-03-28-run10.devils-advocate-stage2a.md` | 173 | 35 |
| `mock/research/imported-from-saalis/2026-03-28-run10.gaphunter-stage1.md` | 556 | 35 |
| `mock/research/imported-from-saalis/2026-03-28-run10.gaphunter-stage3a.md` | 271 | 35 |
| `mock/research/imported-from-saalis/2026-03-28-run10.project-lead-synthesis.md` | 431 | 35 |
| `mock/research/imported-from-saalis/2026-03-28-run10.review-history-summary.md` | 116 | 35 |
| `mock/research/imported-from-saalis/2026-03-28-run10.savant-stage1.md` | 373 | 35 |
| `mock/research/imported-from-saalis/2026-03-28-run10.savant-stage2b.md` | 278 | 35 |
| `mock/research/imported-from-saalis/2026-03-28-run10.savant-stage3b.md` | 155 | 35 |
| `mock/research/imported-from-saalis/2026-03-28-run10.specauditor-stage1.md` | 623 | 35 |
| `mock/research/imported-from-saalis/2026-03-28-run10.specauditor-stage2a.md` | 197 | 35 |
| `mock/research/imported-from-saalis/2026-03-28-run10.specauditor-stage3a.md` | 321 | 35 |
| `mock/research/imported-from-saalis/2026-03-28-run10.tensions-stage1.5.md` | 46 | 35 |
| `mock/research/imported-from-saalis/2026-03-28-run11.advocate-stage2a.md` | 832 | 35 |
| `mock/research/imported-from-saalis/2026-03-28-run11.archsurgeon-stage1.md` | 595 | 35 |
| `mock/research/imported-from-saalis/2026-03-28-run11.archsurgeon-stage2a.md` | 1346 | 36 |
| `mock/research/imported-from-saalis/2026-03-28-run11.archsurgeon-stage3a.md` | 663 | 36 |
| `mock/research/imported-from-saalis/2026-03-28-run11.savant-stage1.md` | 410 | 36 |
| `mock/research/imported-from-saalis/2026-03-28-run11.savant-stage2b.md` | 617 | 36 |
| `mock/research/imported-from-saalis/2026-03-28-run11.savant-stage3b.md` | 512 | 36 |
| `mock/research/imported-from-saalis/2026-03-28-run11.specwriter-stage1.md` | 1835 | 36 |
| `mock/research/imported-from-saalis/2026-03-28-run11.specwriter-stage2a.md` | 831 | 37 |
| `mock/research/imported-from-saalis/2026-03-28-run11.specwriter-stage3a.md` | 586 | 37 |
| `mock/research/imported-from-saalis/2026-03-28-run11.tensions-stage1.5.md` | 69 | 37 |
| `mock/research/imported-from-saalis/2026-03-28-run11.wireeng-stage1.md` | 1377 | 37 |
| `mock/research/imported-from-saalis/2026-03-28-run11.wireeng-stage2a.md` | 765 | 37 |
| `mock/research/imported-from-saalis/2026-03-28-run11.wireeng-stage3a.md` | 1186 | 37 |
| `mock/research/imported-from-saalis/2026-03-28-verkko-taxonomy.md` | 389 | 37 |
| `mock/research/imported-from-saalis/2026-03-29-run11.project-lead-synthesis.md` | 2235 | 38 |
| `mock/research/imported-from-saalis/2026-03-29-run12.digest.md` | 83 | 38 |
| `mock/research/imported-from-saalis/2026-03-29-run12.project-lead-synthesis.md` | 2016 | 38 |
| `mock/research/imported-from-saalis/2026-03-29-run12.stage1-expert-a.md` | 1202 | 39 |
| `mock/research/imported-from-saalis/2026-03-29-run12.stage1-expert-b.md` | 1330 | 39 |
| `mock/research/imported-from-saalis/2026-03-29-run12.stage1-expert-c.md` | 1225 | 39 |
| `mock/research/imported-from-saalis/2026-03-29-run12.stage1-savant.md` | 1220 | 39 |
| `mock/research/imported-from-saalis/2026-03-29-run12.stage1.5-tensions.md` | 514 | 39 |
| `mock/research/imported-from-saalis/2026-03-29-run12.stage2a-expert-a.md` | 1025 | 40 |
| `mock/research/imported-from-saalis/2026-03-29-run12.stage2a-expert-b.md` | 1175 | 40 |
| `mock/research/imported-from-saalis/2026-03-29-run12.stage2b-savant.md` | 1120 | 40 |
| `mock/research/imported-from-saalis/2026-03-29-run12.stage2c-devils-advocate.md` | 1163 | 40 |
| `mock/research/imported-from-saalis/2026-03-29-run12.stage3a-expert-a.md` | 914 | 40 |
| `mock/research/imported-from-saalis/2026-03-29-run12.stage3a-expert-b.md` | 971 | 41 |
| `mock/research/imported-from-saalis/2026-03-29-run12.stage3a-expert-c.md` | 962 | 41 |
| `mock/research/imported-from-saalis/2026-03-29-run12.stage3b-savant.md` | 660 | 41 |
| `mock/research/imported-from-saalis/design_rounds/202603161400/202603100000_topic.final-architecture-decisions.md` | 6002 | 42 |
| `mock/research/imported-from-saalis/design_rounds/202603161400/202603140000_topic.architecture-summary.md` | 310 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603161400/202603141200_topic.hilavitkutin-arvo-integration.md` | 371 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603161400/202603150000_topic.architecture-revisions.md` | 554 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603161400/202603161200_topic.crate-taxonomy-revision.md` | 180 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603161400/202603161300_topic.crate-taxonomy-decisions.md` | 63 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603161400/202603161400_changelist.doc.lock.md` | 102 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603161400/202603161500_changelist.src.deprecated.md` | 206 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603161400/202603162100_changelist.src.lock.md` | 33 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603162300/202603162200_topic.implementation-roadmap.md` | 186 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603162300/202603162300_changelist.doc.lock.md` | 135 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603162300/202603162330_changelist.src.lock.md` | 182 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603170810/202603170800_topic.primitives-design.md` | 54 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603170810/202603170810_changelist.doc.lock.md` | 7 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603170810/202603170815_changelist.src.lock.md` | 4 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603170905/202603170900_topic.contracts-crate-split.md` | 54 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603170905/202603170905_changelist.doc.lock.md` | 4 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603170905/202603170910_changelist.src.lock.md` | 4 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603241200_topic.mesh-architecture-and-crates.md` | 76 | 43 |
| `mock/research/imported-from-saalis/design_rounds/202603241200_topic.mesh-data-and-operations.md` | 93 | 43 |
| `mock/research/imported-from-saalis/design_rounds/README.md` | 10 | 43 |
| `mock/research/imported-from-saalis/verkko-crypto.md` | 575 | 43 |
| `mock/research/imported-from-saalis/verkko-matter.md` | 372 | 43 |
| `mock/research/imported-from-saalis/verkko-mesh.md` | 2081 | 44 |
| `mock/research/imported-from-saalis/verkko-ops.md` | 457 | 44 |
| `mock/research/imported-from-saalis/verkko-protocol.md` | 1042 | 44 |
| `mock/research/imported-from-saalis/verkko-relay.md` | 479 | 44 |
| `mock/research/imported-from-saalis/verkko-seat.md` | 470 | 44 |
| `mock/research/sketches/202605050503_accessset_arity/FINDINGS.md` | 174 | 25 |
| `mock/research/sketches/202605050530_deep_stacking/FINDINGS.md` | 103 | 25 |
| `mock/research/sketches/202605050555_replaceable_polarity/FINDINGS.md` | 80 | 25 |
| `mock/research/sketches/202605050615_kit_taxonomy/FINDINGS.md` | 107 | 25 |
| `mock/research/sketches/202605050615_kit_taxonomy/sketch_skeletons.md` | 13 | 25 |
| `mock/research/sketches/202605050700_deep_stacking_d5/FINDINGS.md` | 117 | 25 |
| `mock/research/sketches/202605050800_dedup_concat/FINDINGS.md` | 98 | 25 |
| `mock/research/sketches/202605082230_recursion_limit_macro/FINDINGS.md` | 63 | 25 |
| `mock/research/sketches/202605091700-builder-provider-shape/HYPOTHESIS.md` | 35 | 25 |
| `mock/research/sketches/202605091700-builder-provider-shape/OUTCOME.md` | 120 | 25 |
| `mock/research/sketches/202605101036-builder-input-reshape/HYPOTHESIS.md` | 26 | 25 |
| `mock/research/sketches/202605101036-builder-input-reshape/OUTCOME.md` | 68 | 25 |
| `mock/research/sketches/202605101036-codegen-entrypoint-tait/FINDINGS.md` | 129 | 25 |
| `mock/research/sketches/202605101036-codegen-entrypoint-tait/SKETCH.md` | 48 | 25 |
| `mock/research/sketches/202605101036-codegen-tait-capture/FINDINGS.md` | 67 | 25 |
| `mock/research/sketches/202605101036-codegen-tait-capture/SKETCH.md` | 51 | 25 |
| `mock/research/sketches/202605101036-const-zero-bloat/FINDINGS.md` | 56 | 25 |
| `mock/research/sketches/202605101036-ema-vectorisation/FINDINGS.md` | 80 | 25 |
| `mock/research/sketches/202605101036-ema-vectorisation/SKETCH.md` | 50 | 25 |
| `mock/research/sketches/202605101036-fibershape-typing/FINDINGS.md` | 89 | 25 |
| `mock/research/sketches/202605101036-fibershape-typing/SKETCH.md` | 50 | 25 |
| `mock/research/sketches/202605101036-poolframe-lifetime/FINDINGS.md` | 131 | 25 |
| `mock/research/sketches/202605101036-poolframe-lifetime/SKETCH.md` | 91 | 25 |
| `mock/research/sketches/202605101036-progress-counter-arena/FINDINGS.md` | 45 | 25 |
| `mock/research/sketches/202605101036-resource-column-virtual-as-traits/HYPOTHESIS.md` | 31 | 25 |
| `mock/research/sketches/202605121400-call-site-cap-turbofish/OUTCOME.md` | 231 | 25 |
| `mock/research/sketches/202605121400-call-site-cap-turbofish/SKETCH.md` | 79 | 25 |
| `mock/research/sketches/202605131500-witness-propagation/OUTCOME.md` | 122 | 25 |
| `mock/research/sketches/202605131500-witness-propagation/SKETCH.md` | 75 | 25 |
| `mock/research/sketches/202605131530-csr-to-bitmatrix/SKETCH.md` | 45 | 25 |
| `mock/research/sketches/202605131600-array-reshape-consumed/OUTCOME.md` | 141 | 25 |
| `mock/research/sketches/202605131600-array-reshape-consumed/SKETCH.md` | 144 | 25 |
| `mock/research/sketches/202605131700-plan-forward-arvo-wire-up.md` | 190 | 25 |
| `mock/research/sketches/202605251400_v2-phase-machine-field-notes.md` | 101 | 25 |
| `mock/research/sketches/202605251500_cl-drift-closure-clarification.md` | 73 | 25 |
| `mock/research/sketches/202605290002_builder-kind-dispatch.md` | 81 | 25 |
| `mock/research/sketches/202605290101_ctx-projection-selector.md` | 63 | 25 |
| `mock/research/sketches/202605290245_usize-to-cap-const-bridge.md` | 50 | 25 |
| `mock/research/sketches/202605291007_engine-ctx-spec-free-accessor/findings.md` | 92 | 25 |
| `mock/research/sketches/202605300823_run-fiber-wutuple-walk/FINDINGS.md` | 37 | 25 |
| `mock/research/sketches/202605301145_capacity-as-type/FINDINGS.md` | 151 | 25 |
| `mock/research/sketches/202605301700_plandims-bundle/FINDINGS.md` | 58 | 25 |
| `mock/research/sketches/202605302345_store-backed-flat-plan/FINDINGS.md` | 138 | 25 |
| `mock/research/sketches/202606041631_column-source-dual-selector/FINDINGS.md` | 36 | 25 |
| `mock/research/sketches/202606041800_accumulator-append-selector/FINDINGS.md` | 40 | 25 |
| `mock/research/sketches/202606051601_column-capable-runfiber-walk/FINDINGS.md` | 34 | 25 |
| `mock/research/sketches/202606052130_rcm-order-into-static-body/FINDINGS.md` | 192 | 25 |
| `mock/research/sketches/202606052130_rcm-order-into-static-body/HYPOTHESIS.md` | 39 | 25 |
| `mock/research/sketches/202606060500_approach-e-schedule-mega/FINDINGS.md` | 112 | 25 |
| `mock/research/sketches/202606060500_approach-e-schedule-mega/HYPOTHESIS.md` | 63 | 25 |
| `mock/research/sketches/202606060730_runfibercol-accumulator-inline/FINDINGS.md` | 124 | 25 |
| `mock/research/sketches/202606070829_gate2-waist-phase-from-masks/FINDINGS.md` | 73 | 25 |
| `mock/research/sketches/202606070906_gate2-pairs-enumeration-dispatch/FINDINGS.md` | 96 | 25 |
| `mock/research/sketches/202606071945_gate2-accumulator-percore-region-merge/FINDINGS.md` | 143 | 25 |
| `mock/research/sketches/202606080130_gate2-ge-const-range-dispatcher/FINDINGS.md` | 94 | 25 |
| `mock/research/sketches/202606080200_gate2-ge-outer-drives-rungatedtrunk/FINDINGS.md` | 104 | 25 |
| `mock/research/sketches/202606081400_e4-schedule-gated-walk/FINDINGS.md` | 29 | 25 |
| `mock/research/sketches/202606081700_e4-scheduled-blanket-coherence/FINDINGS.md` | 31 | 25 |
| `mock/research/sketches/202606081800_e4-gate-firer-trait-shapes/FINDINGS.md` | 92 | 25 |
| `mock/research/sketches/202606082000_e4-slice2-lifecycle-classify/FINDINGS.md` | 84 | 25 |
| `mock/research/sketches/202606082200_e4-slice2-rank-phase-renumber/FINDINGS.md` | 90 | 25 |
| `mock/research/sketches/202606090100_facade-accessset-containsall/FINDINGS.md` | 106 | 25 |
| `mock/research/sketches/202606090200_facade-per-morsel-abi-hop/FINDINGS.md` | 92 | 25 |
| `mock/research/sketches/202606090300_e4-slice3-meta-bridge-accessor/FINDINGS.md` | 93 | 26 |
| `mock/research/sketches/202606111000_e1-e7-dirty-gated-walk/FINDINGS.md` | 33 | 26 |
| `mock/research/sketches/202606111430_ctxfor-computed-ctx/FINDINGS.md` | 139 | 26 |
| `mock/research/sketches/202606111440_meta-carrier-shapes/FINDINGS.md` | 160 | 26 |
| `mock/research/sketches/202606111450_cap-lifting-shapes/FINDINGS.md` | 205 | 26 |
| `mock/research/sketches/202606202100_a2a-per-fiber-morsel-loop/FINDINGS.md` | 91 | 26 |
| `mock/research/sketches/202606202400_a3-per-store-size-collection/FINDINGS.md` | 118 | 26 |
| `mock/research/sketches/202606210100_a3-resource-footprint/FINDINGS.md` | 120 | 26 |
| `mock/research/sketches/202607171733_plannable-declaration-split/FINDINGS.md` | 90 | 26 |
| `mock/research/sketches/202607190900_a4-parallel-per-fiber-windows/FINDINGS.md` | 135 | 26 |
| `mock/research/sketches/202607191200_a4-fibercons-nest-wireability/FINDINGS.md` | 92 | 26 |
| `mock/research/sketches/202607201300_arena-poolframe/FINDINGS.md` | 34 | 26 |
| `mock/research/sketches/202611122242_frame-view-typestate/FINDINGS.md` | 145 | 26 |
| `mock/research/sketches/202611122252_runner-generic-workunit/FINDINGS.md` | 121 | 26 |
| `mock/research/sketches/202611130039_wasm-digest-under-node/FINDINGS.md` | 103 | 26 |
| `mock/research/sketches/202611130417_branded-morsel-index/FINDINGS.md` | 93 | 26 |
| `mock/research/sketches/app-builder-typestate-202605010900/FINDINGS.md` | 266 | 26 |
| `mock/research/sketches/registrable-not-in-202605051200/FINDINGS.md` | 73 | 26 |
