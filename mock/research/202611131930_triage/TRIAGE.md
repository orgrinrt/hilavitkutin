# hilavitkutin triage, 13 November 2026

Read against `origin/dev` at `271909e` (fetched for this triage). Nothing was built: the disk had 4.8 GB free and no classification needed a build. Every red state of dev below is therefore what a ticket measured at the commit it names, plus a source read on `271909e`; where that read could not confirm the state it says so.

Method for part 1: for every file a branch changed since its merge base with dev, `git diff --quiet origin/dev <branch> -- <file>` (same content), and for each file that differs, whether the branch's blob was ever on dev (`git log origin/dev --find-object=<blob>`) and what a three-way `git merge-file` of that one file into dev would change. The scripts are in the scratchpad (`branches.sh`, `diffcheck.sh`, output `branches.out`).

## Part 1: the work branches

Dev itself is a line of single-parent squash commits from `4b327c5` to `271909e`, all of 13 November (`git log --format='%h %p' origin/dev`), so ancestry says nothing and content is the only test.

On dev (every changed file identical on dev):

| branch | files changed | evidence |
| --- | --- | --- |
| work/bench-wide 75a4ae7 | 180 | all 180 SAME. `git diff --stat 1075096 44f7de2` is empty: the merge `1075096` and dev's `44f7de2` are the same tree, so "frame_wide at 44f7de2" and "bench-wide landed at 1075096" are one landing |
| work/land-stray-record 8fcce10 | 16 | all SAME (dev `81c5c58`) |
| work/mockspace-lint-one-pass 8080f61 | 2 | all SAME (dev `ada51bb`) |
| work/recover-landing-record 528a004 | 1 | SAME (dev `6e971d1`) |
| work/sketch-branded-index 7dad131 | 56 | all SAME (dev `27abd8a`) |
| work/sketch-frame-view a4d578f | 87 | all SAME (dev `b373d70`) |
| work/sketch-runner-generic 99f932e | 103 | all SAME (dev `cfe40b4`) |
| work/sketch-wasm-digest b425980 | 22 | all SAME (dev `0928042`) |
| work/stack-audit 02d64fa | 12 | all SAME (dev `c490269`); `AUDIT.md` is on dev |

Superseded (every differing file's branch blob was on dev once and dev has since moved past it; no file is absent from dev):

| branch | same / differing | evidence |
| --- | --- | --- |
| work/bench-convergence 83a3855 | 236 / 9 | the 9 are the bench's Cargo.toml files; branch blob on dev until `44f7de2`, which moved the mockspace bench deps from `branch = "dev"` to `rev = "f42844c7..."` (`git diff origin/work/bench-convergence origin/dev -- mock/benches/single-trunk-convergence/Cargo.toml`) |
| work/dev-gate-green 1818a73 | 15 / 11 | each differing blob last on dev at `c2b392d`, `c42d707`, `49ab344` or `271909e` |
| work/notko-by-https 54dfa0b | 6 / 3 | blobs on dev until later commits |
| work/overflow-is-a-value f392cab | 44 / 22 | same |
| work/recover-landing-source 5e87366 | 177 / 33, 3 deletions dev also made | same |
| work/recover-landing-swap 1ca93ab | 52 / 6 | same; the swap itself is dev `311a2ae` |
| work/suite-can-fail eac8365 | 43 / 26 | same |

`grep -v blob-in-dev` over the `diffcheck.sh` output for these seven branches prints no file, so none of them holds a change dev never had. All 16 branches above can be deleted once the coordinator agrees; none holds work that should land.

Holding work that should land: none outside the live lanes.

Live lanes, read only:

- work/api-revision e2becf2 (23 ahead): only design text and research so far. Changed source is limited to `mock/benches/src/main.rs` and `bench.toml`; under `mock/crates` it changes only the three `DESIGN.md.tmpl` files (`git diff --stat <mb> origin/work/api-revision -- mock/crates`). A per-file merge into dev conflicts on `docs/API.md`, `docs/HILAVITKUTIN.md`, `docs/KIT.md` (generated, regenerate) and `mock/benches/src/main.rs` (dev replaced the hand-written driver with `driver::drive` in `44f7de2`). Tickets 153, 154, 157, 158, 159, 161 cite source on this branch at `5e6887f`, and `git cat-file -t 5e6887f` says no such object on origin: the source window exists only in the helper's container. That is the largest unpushed-work risk found.
- work/canon-registry 791c837 (41 ahead): registry rows, questions, tickets and two rounds; its source edits are FIXME retags (one line each in 15 files under `hilavitkutin`, `-api`, `-persistence`, `-providers`, `-build`) and `mockspace.toml` (+1145). All merge cleanly file by file.
- work/loop-core 5603f85 (30 ahead): the known quarry. 124 files dev never had: a second contract crate `hilavitkutin-contracts` (18 src files), `-frozen-authors`, `-frozen-implementors`, `-laws`, `-reference` (50 test files), four `frozen_*` lints and a `cross-target` tool. Op's "revise the api in place now, no second contract crate" retires the crates as a destination; the reference tests, the frozen lints and the cross-target tool are what is worth quarrying.

## Part 2: the open tickets

`tickets-hilavitkutin.toml` on `origin/main` holds 162 actions; 2 are closed by their `done_when` file (`hilavitkutin_lints_use_lintcontext_fields_that_mockspace_a81`, `offer_zig_authored_extensions_at_the_hilavitkutin_c_abi_seam`), 160 are open. Numbers below are the row's position in the file (scratchpad `notes.txt` lists them with key and note).

### Fixed on dev (should be closed)

- 1 to 4, the 30 lint errors at af823b7: every api seal is now `pub(crate) trait Sealed` (`hilavitkutin-api/src/{adapt,builder,dispatch_codegen,hint,meta,run_cfg,store_values}.rs`, `hilavitkutin-linking/src/symbol.rs:134`), and every `#[ignore]` names a row of `mock/catalogue.md`, the ledger `mockspace.toml` gives the `catalogue-ignore` lint. Fixed by `4b327c5`.
- 6, 47, 69, 70, 111, the bench drivers' 11-argument `run_worker`: `mock/benches/resource_storage/src/main.rs:250-263` passes 12 arguments, the last `threaded`; the root driver is `driver::drive` (`mock/benches/src/main.rs:5`).
- 123, dylib paths shaped twice: the `variant-path` crate and the test `the_shaping_the_driver_once_did_is_refused_on_every_cell` in `resource_storage/src/main.rs` (from `44f7de2`).
- 8, replace discards the value: `scheduler/dirty.rs:48` and `:96` take `new` and install it; `tests/replace_swap.rs`, `tests/replace_swap_plan_band.rs` (from `311a2ae`).
- 40, replace unreachable by a consumer: `PlanAffecting` is no longer sealed (`hilavitkutin-api/src/run_cfg.rs:48`), and `tests/replace_swap.rs:93` implements it from outside the crate.
- 80, 91, 94, the local lints on LintContext fields: `mock/lints/engine_spawn_through_gate.rs:64` calls `ctx.all_sources()`, `vocabulary_discipline.rs:77` calls `ctx.source()` (from `ada51bb`).
- 96, the recovery copy that lost changes: dev carries none of `dispatch/phase_run.rs`, `thread/pool.rs`, `tests/phase_pipeline_dispatch.rs` (`git ls-tree` prints nothing) and no `ProviderKit` under `mock/crates`; the landing followed the locked changelists.
- Answered by op and still open: 60 (worker panic, op-answers-36 "Abort stays, fix hang only", which is 144), 86, 87, 88 (op-answers-25 Q1, Q2, Q3 and the follow-up; 87 lives on as 120), 101 (op-answers-26 "Land as provisional", landed `311a2ae`; ratification waits on 106 and 107).

### Partly fixed

- 9: `plan_dirty` is consumed now (`dirty.rs:133`, `run.rs:122`, `run_parallel.rs:252`); `plan_cache` is still only touched (`run.rs:126`, `let _ = &self.plan_cache;`). Narrow the row to the cache.
- 119: `configure` exists (`hilavitkutin-build/src/bootstrap.rs:83`); the rest of the design (profiles, enable, run) does not. Held by the build lane.

### Void under op's "revise the api in place, no second contract crate"

41, 42, 43, 44, 45, 51, 53, 54, 57, 61, 62 are about `hilavitkutin-contracts` and the frozen crates of work/loop-core, which no longer have a destination. Three carry a real question that has to be re-filed against `hilavitkutin-api` rather than dropped: 55 (positions, no resize, no growth; the audit's "storage: one record count, no growth" mark), 56 (storage word and widest cell; op's answer on kaski's `may_a_lattice_channel_hold_lanes_narrower_than_64_bits`: "Upstream on hila it should be entirely generic and configurable by user"), 63 (a consumer's own value type).

### Duplicates (keep the first named)

- 156, with 5, 35, 46, 99, 126, 147 and the second half of 151: tests that place nothing in the `hilavitkutin_strings` section fail to link on `__start_/__stop_hilavitkutin_strings` (`hilavitkutin-str/src/section.rs`, linux `imp`). 156 measured it at `271909e`. Its cause is op's question 110.
- 66 is 7 (`generic_const_exprs`, `hilavitkutin/src/lib.rs:24` plus `#![feature]` in `hilavitkutin-api/tests/{column_value,engine_id_index,store_types}.rs`, `hilavitkutin/tests/{gate2_const_grouping,gate2_core_phase_masks,synthesise_core_programs}.rs`).
- 38 is 34 (rustfmt moves trybuild fixtures; a mockspace hook).
- 150 is 37 (close sweeps flat topics; `mockspace.toml` tracks `mockspace_branch = "dev"`, so the fix is in the launcher's pin resolution).
- 50 is 36 (run_fused and an appending RecordOp; `tests/fused_append.rs:235` ignored).
- 151 is 145 (the EMA no-op, `hilavitkutin-providers/src/adapt_ema.rs:119`).
- 33 is 160 (the providers examples; `examples/particle_collision.rs:18`).
- 100 is 141 (two notko spellings; `mock/benches/asm_gate_fixtures/Cargo.toml:26` and `engine_vs_std/Cargo.toml:27` still spell `ssh://`).
- 142 is 140 (bench builds in a shared target).
- 125 is 128 (engine_par on one core; 128 names the cause, the waist-bounded phase axis; 135 is the question it raises).
- 29 is part of 16 (the FIXME at `scheduler/run_parallel.rs:166` names no ticket; the gap is 16).
- 85 is the FIXME `2a77ac57ef94` at `scheduler/worker.rs:354` and the bare one at `run.rs:231` (`#340`).
- 112 is answered in substance by 76 (op: "Bench both first") and the api revision's record domains; its options are still waiting on op, so close it into 76 or put it back to him with 76's answer.
- 108 makes 92 unreachable at the default dimensions; one round fixes both.

### Live, by area (crate, size)

Engine scheduler and plan (`hilavitkutin`; overhauled by the api lane's port until it lands):
- 115 three run entries computing differently (L), 116 the ungated N-way split (M), 157 run_parallel never asks `dirty_units` (M, on the unpushed api branch), 49 no skip differential under run_parallel (M), 105 head and tail convergence not built (L; catalogue row has no ticket, below), 128 waist-bounded phase axis (L), 72 write-write order by registration only (`plan/steps/step1_dag.rs:49`, M), 16 closures per worker (S), 17 MIN_MORSEL alignment (S), 18 fiber_plan_idx (S), 85 bake masks (M), 132 trunk ownership written twice (S), 155 prefetch no-ops (`intrinsics/mod.rs:19,28`, S), 9 plan_cache (S), 92 and 108 descriptor sizing (S).
- Correctness: 144 hang on a worker panic (no `catch_unwind` or panic handling in `scheduler/worker.rs` or `thread/`, M), 93 swap between run_parallel frames aliases a parked worker's shared borrow (FIXME `74e2b9918149` at `dirty.rs:62,113`, L), 58 and 59 accumulators under run_parallel (M each; 58 ignored at `tests/overflow_is_a_value.rs:377`), 36 run_fused (M), 106 and 107 the swap's two gaps (`dirty.rs:77`, `run_fused.rs:77`, M), 26 and 48 test providers handing out bases into a moved-from slot (32 test files on dev carry `UnsafeCell<[MaybeUninit`) and the safe `MemoryProviderApi` (`hilavitkutin-api/src/platform.rs:28`), M.
- Adaptation: 10 to 15 (`tests/adapt_perf_contracts.rs:34-120`, L together), 19 Seq and Map members (M).

Api surface (owned by the api lane until it lands): 24 (`dispatch_codegen.rs:400-406`), 25 (`platform.rs:467`), 27 (`access.rs:37,107`, `store.rs:263` name `add_resource`, `add_column`, `add_virtual`; the builder has `with`, `clock`, `build`, `build_with`), 31 (`DESIGN.md.tmpl:1029`), 65 (`DESIGN.md.tmpl:17-209` still Provider, ProviderKind, LinkedBin), 98 (`work_unit_values.rs:58-80`), 122 host-side column write (frame_wide reaches through `__bindings`, `engine.rs:283-296`), and the api lane's own rows 64, 67, 74, 75, 76, 77, 120, 121, 137, 138, 153, 154, 158.

Leaf crates nobody overhauls:
- 114 `RTLD_LOCAL = 4` (`hilavitkutin-linking/src/backend/unix.rs:9,26`), S, a real linux bug.
- 109 and 97, sym (`generator.rs:18` still says 28-bit, `:115` single; the design has no `Maybe<Self>`), S.
- 145 bind-verdict drift: 28-bit content key in persistence, stale arvo pins, the NEON claim in `docs/DESIGN.md`; the EMA half waits on the providers port, M.
- 21 and 22 cold store mmap I/O and snapshot (`hilavitkutin-persistence/src/cold_store.rs:32,60,69,78`), L.
- 20 LTO smoke off unix (`hilavitkutin-extensions-macros/tests/lto_smoke.rs:225`), S.
- 156 str link, S to M (shape waits on 110).

Suite as an instrument: 148 trybuild compile-fail fixtures compile (engine, api, extensions-macros; measured at `81c5c58`, not re-run), 28 thirteen catalogued tests whose bodies are `unimplemented!` or `unreachable!` (counted on dev: adapt_perf_contracts 7, cold_store 2, a1_fiber_morsel_size 1, r6_morsel_window_formula 1, resource_snapshot 1, lto_smoke 1), 39 (`tests/ui/undeclared_read_fail.stderr` still holds 2 errors), 102 (`tests/gate2_run_parallel.rs` is 892 lines), 7 `generic_const_exprs`, 113 dead_code allows (21 files on dev), 159 warnings, 161 process words in rustdoc, 30 dead ids (`plan/mod.rs:95` `#341`, bare FIXME `projection.rs:149`), 160 examples.

Benches as instruments: 23 perf oracles onto the harness, 95 asm gate misses the parallel body, 117 four microkernel families measure nothing (`variants/ctz_intrinsic/src/lib.rs:24` and `ctz_loop/src/lib.rs:32` still `v | 1`), 118 no committed asm gate output, 103 workloads in the intended region, 104, 124, 133, 134, 136, 139 frame_wide and convergence follow-ups, 140 shared-target staging, 141 notko spelling.

Registry and record (held by hv-canon-registry-2): 73 two copies of the standalone spec (both on dev), 83, 127, 129, 130, 131, 143.

Workspace and mockspace tooling (not hilavitkutin code): 32 lint-rules skill template (`mock/agent/skills/lint-rules/SKILL.md.tmpl:10`), 34, 37, 52, 78, 79, 82, 84, 162.

Build (held by the build lane): 119, 146, 149.

### Op's questions still open

68 (names for Access and Deny), 71 (registration beside the typed one), 89 (value-function unit naming its context), 110 (keep linker-section string collection or replace it), 112 (one record count; see duplicates), 135 (should a terminal fan-in close a phase).

### Files for op with no question ticket

- `mock/research/202611130850_the-record-read-whole/ROWS-FOR-OP.md` on work/canon-registry: 131 registry rows the record does not back. No question ticket; job 131 only says "then bring op ROWS-FOR-OP.md".
- `OP-ANSWERS*.md` in the same directory are records of answers, not asks.
- No file named FOR-HIM, for-op or similar on dev or any work branch.
- Catalogue row `hilavitkutin_catalogue_a_single_trunk_phase_is_walked_by_one` (`mock/catalogue.md`, cited by `tests/gate2_convergence.rs:181`) is no ticket key in the lane; 105 tracks the same gap under another key.
- FIXMEs on dev cite kaski workspace ids (`e3cb3f33eb38` at `dispatch_codegen.rs:403`, `fe682c10859c` at `platform.rs:467`, `ef57305fb5f4` in `frame_wide/.../engine.rs`), which is ticket 130's class; the registry lane retags the first set, not the frame_wide one.

## Part 3: a second look at what landed since 2026-11-12

ab63d61, the convergence bench (squashed, unreviewed):
- `support/src/walk.rs:117-130` implements `Regions` for `HeadTailMid`, and nothing uses it: both head-and-tail arms take `sequential_fallback` on the append (`arms.rs:79-84`, `:99-104`), and `git grep region\(` finds only `walk.rs`. Were it used, the tail walker walks backward (`split.rs:158-160`) and appends from `region.start`, so its region would hold its records in reverse and the compaction at `walk.rs:201-210` would keep that reversed order. Dead and wrong if wired; delete it or make it correct.
- `split.rs:174-177` says whichever walker sees the crossing first stores the meeting record; `HeadTailDyn::walk` stores it from both walkers unconditionally after their loops. Same value, but the doc is wrong.
- The W = 8 sections run eight walkers on four vCPUs (FINDINGS.md:58 says so); they measure oversubscription, and 4 of the 16 section tables rest on it.
- Three of the result sets were recorded from dirty trees (`ed2bf53-dirty`, `d11e290-dirty`, `e486b5f-dirty`, FINDINGS.md:123), with what was dirty unrecorded.
- Neither this member nor frame_wide is a member of `mock/Cargo.toml`, so `cargo mock test` never builds their correctness tests (`frame_wide/run-checks.sh:3-8` says it in words). No ticket names that for the convergence bench.
- `engine_vs_std/src/executor.rs`: `BenchExecutor::new` lost `const` (it now calls `std_threads()`), and `Default` became a runtime impl. Correct, but it changes a sibling bench's surface inside a convergence commit.
- 104 already carries the missing spread flag and the missing walker-count tests.

44f7de2 = 1075096, frame_wide and the wide matrix (squashed on dev, reviewed three rounds on its branch per 134):
- `support/.../engine.rs:213-214` declares `unsafe impl Sync for HeapBump` over a `Cell<usize>` bump pointer, resting on the comment that only `build` allocates. No test or type holds that; a provider called from a worker would race.
- `engine.rs:283-296` reaches input columns through `Scheduler::__bindings()`, documented as not part of the supported surface; FIXME `ef57305fb5f4` is a workspace id, not a registry row (ticket 122 is the gap).
- The parallel bars spawn per frame (124) and engine_par runs on about one core (128); both ticketed.
- The dev landing pinned the bench tree's mockspace dependencies by `rev = "f42844c7..."`, which is the fix for the floating harness signature behind 6 and its duplicates, but the bench members `asm_gate_fixtures` and `engine_vs_std` still spell notko by `ssh://` (141).

Others since 2026-11-12 that want a second look:
- `311a2ae` the swap, landed provisional with two known gaps (106, 107) and the parked-worker aliasing FIXME `74e2b9918149` at `scheduler/dirty.rs:62` and `:113`: the swap's run_parallel driver pattern is the audit's aliasing hole.
- `49ab344` the recovered source: `tests/gate2_run_parallel.rs` grew to 892 lines (102); 92's catalogue row has no ignored test left on dev.
- `271909e` build through hilavitkutin-build: every crate's `build.rs` now runs `configure`; `bootstrap.rs:138` carries a FIXME that names no ticket, and the config file it writes is a placeholder nothing reads (`bootstrap.rs:139-146`). The build lane holds it.
- `c42d707` overflow as a value: leaves 58 and 59 standing (58 ignored at `tests/overflow_is_a_value.rs:377`; 59 has no test on dev).

## Part 4: proposed helper tracks

### What kaski needs from hilavitkutin first

From `mock/research/202611131215_stack-audit/DOWNSTREAM.md:13-29` and `:55-57`, in its order: several record sets of different counts, reading last frame's value (a buffered column), reading a neighbour (reach), then the aliasing hole and the worker-panic hang (correctness), then growth, determinism across worker counts and persistence, and only then fusion and windows. The first three are the api revision's (record domains, `Buffered<T, M>`, `Reach`), so the live lane is already kaski's first need.

From kaski's own tickets on `origin/dev`:
- `may_a_lattice_channel_hold_lanes_narrower_than_64_bits`, op: "Upstream on hila it should be entirely generic and configurable by user". The cell and lane format must be the consumer's (rows 56 and 63 re-filed against the api).
- `how_is_the_lattice_held_tile_entities_pointing_at_a_kaski`, op: "I would be interested in reducing the ecs role to even zero if hilavitkutin proves capable". The lattice-floor and thing-table benches decide it, so hilavitkutin has to be bench-ready in the region kaski runs.
- `hilavitkutin_and_kaski_build_through_hilavitkutin_build`, `kaski_the_rest_of_the_crates_build_through_hilavitkutin`: the build lane.
- Op-answers-28: load-time work units, "Panel it first".
- Op-answers-36: abort stays, fix the hang only.

### Tracks

Live lanes planned around: api-revision overhauls `hilavitkutin-api` and ports `hilavitkutin`, `-kit`, `-providers` (and touches `mock/benches/src/main.rs`, `mock/benches/bench.toml`); canon-registry holds `mock/registry` and retags FIXMEs; the build lane holds `hilavitkutin-build`.

Track 1, leaf crates and a green dev (starts now; nothing waits on the api).
- Jobs: 156 with its duplicates (a section that links with no entries, independent of op's 110, which decides the longer shape); 114 RTLD_LOCAL per target with a test that `Library::load` of a non-resident library returns non-null on linux; 109 and 97 (sym); the persistence and docs parts of 145 (content key width with a collision test at 10k strings, the arvo pins, the NEON sentence); 20 (LTO smoke off unix); the extensions-macros part of 148.
- Crates: `hilavitkutin-str`, `-linking`, `-sym`, `-persistence` (not 21 and 22), `-extensions-macros`. Overhauls none; adds beside.
- Check: `cargo test -p hilavitkutin-providers --test smoke` and `-p hilavitkutin-persistence --test archive_str` link and pass; `cargo test -p hilavitkutin-linking`; the trybuild fail fixtures of `-extensions-macros` fail to compile.
- Tier: sonnet high. Well specified and checkable; the linker work has a named fix and a test that proves it.

Track 2, the benches as instruments (starts now, off the two files the api lane edits).
- Jobs: 141 and 100 (notko spelling in every member), 140 and 142 (stage the build per worktree), 95 and 118 (asm gate over the parallel body, output committed by a tool), 117 (the four mislabelled microkernel families, and the NIGHT_FINDINGS claims drawn from them), 23 (perf oracles onto the harness), 104, 124, 133, 134, plus a new ticket: the bench members' own tests are in no gate (`run-checks.sh:3-8`), so a tool or the launcher runs them; and the dead `Regions for HeadTailMid` and the meeting-record doc from part 3.
- Crates: `mock/benches/*` members only, not `mock/benches/src/main.rs` or `mock/benches/bench.toml` until the api lands. Overhauls `mock/benches/asm_gate_fixtures` and the microkernel variants.
- Check: each member's `run-checks.sh` passes; the asm gate output regenerates byte-identical; the microkernel arms give non-constant results with a negative control.
- Tier: opus medium. Bench validity is judgement, but each job has a check; not fable, by op-answers-31's economy.
- The large sweeps (103, 136, 139) wait for a machine run, not a helper: they are hours on a 4-vCPU guest and the disk is at 4.8 GB free.

Track 3, load-time work units, a panel (starts now; writes only `mock/research/`).
- Jobs: op-answers-28 "Panel it first", and with it the growth question (55 re-filed: no resize, no growth) and the consumer-owned lane format (56, 63 re-filed), since all three are what a game's content and spawning need of a compile-time schedule.
- Crates: none; a research directory and question tickets.
- Check: a consolidation with each arm benched or sketched under `mock/research/sketches/`, and question tickets with options for op.
- Tier: two opus high seats in sequence (one expert at a time), fable for the consolidation only if `weekly-usage` prints a `fable:` line. It is the largest open design question for kaski (DOWNSTREAM.md:27).

Track 4, the engine's one path and its correctness (waits on the api revision landing; one helper, since every job is in `hilavitkutin/src/scheduler`, `plan` and `dispatch`).
- First a plan: 115, 116, 157, 49, 105, 128 with 135, 72 decide what the single engine path is, against the 6 June amendment (`mock/research/202606061000_canon-amendment-unified-engine.md`) and the consolidation, which the audit finds contradicting each other. Then correctness on that path: 144 (fix the hang only), 93 (the aliasing hole), 58, 59, 36, 106, 107 (unblocks S1 to S7 ratification), 26 and 48, 9, 92 with 108, 16, 17, 18, 85, 132, 155.
- Crates: overhauls `hilavitkutin` (scheduler, plan, dispatch); touches `hilavitkutin-api/src/platform.rs` for 48, agreed with the api lane once it has landed.
- Check: Miri over the run_parallel and swap tests; a panicking unit test that ends within a timeout; a differential of `run` against `run_parallel` at 1, 2 and 4 workers over the incremental history of `tests/incremental_skip.rs`; the catalogued ignores of 58 and 36 un-ignored and green.
- Tier: the plan on fable medium (the hardest judgement in the repository: three documents disagree on the engine's shape and any answer redoes performance work); the build on opus high (concurrency and unsafe, nobody to ask).

Track 5, the suite and the surface (waits on the api revision landing; files the api lane ports).
- Jobs: 148 (engine and api parts), 28, 39, 102, 7 with 66, 113, 159, 161, 30, 160 with 33, 98, 27, 24, 25, 31, 65, 32, 73 (with the registry lane), and the adaptation catalogue 10 to 15 and 19 only as a later set.
- Crates: `hilavitkutin/tests`, `hilavitkutin-api` (diagnostics, markers, design text), `hilavitkutin-providers/examples`; must not touch `scheduler/` while track 4 runs.
- Check: every trybuild fail fixture fails, `cargo check --all-targets` prints no warning, no `allow(dead_code)`, `git grep -i -w -E 'round|changelist|megaround|topic'` over each crate's src at zero, the examples run.
- Tier: opus medium for 148, 28 and 7 (removing `generic_const_exprs` means another shape for the const arithmetic); sonnet high for the rest.

Order: tracks 1, 2 and 3 start now. Tracks 4 and 5 start when the api revision lands, and track 4's plan can be written before then from the api revision's locked doc changelist. Persistence (21, 22) follows track 1 on the same helper, opus high, once kaski's save picture (op-answers-27) is in kaski's canon. The adaptation work (10 to 15, the EMA of 145) and fusion come after track 4, as the audit orders it (DOWNSTREAM.md:55-57) and op's baseline ruling (op-answers-21) requires.

Before any of this: the api lane's source window at `5e6887f` is not on origin, and should be pushed before anything else is planned on top of it.
