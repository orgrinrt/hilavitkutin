# Seat Five: Whether the Accumulator Tiling Reads Commutativity

Persona: Frank McSherry. Model: Claude Opus 5.5 (claude-opus-5-5). Read only, worktree `01f44ed7`. I did not run the test suite because the dispatch forbids it (disk). I read the bodies of the tests in the touched surface (`tests/gate2_accumulator.rs`, `tests/plan_inputs_from_bundle.rs`). A second independent read is owed; the other seat is it.

## Canon gate

Passed, with one scoping item handed back. Ratified canon licenses splitting a fiber's records across cores, for commutative fibers only. `a_unit_declares_whether_it_is_commutative` (ratified) says a commutative fiber is "eligible for front-back processing, multi-core morsel dispatch and deterministic segment assignment; a non-commutative one is processed in a single direction only". `a_fiber_is_commutative_only_if_all_its_units_are_and_resource_accumulation_uses_per_thread_accumulators_merged_after` (ratified) gives a commutative fiber "per-thread stack-local accumulator, merged after". So the tiling may exist. The provisional row `the_n_way_record_split_of_a_single_trunk_phase_is_not_head_and_tail_convergence_and_is_drift` ("never as an N-way record or morsel partition") binds only as provisional and loses to the ratified "multi-core morsel dispatch" where they meet.

The scoping item: op's "The code stays as it is meanwhile" (`question::which_convergence_a_single_trunk_phase_uses`, answered) is scoped by its question to single-trunk phases. The accumulator path runs at any trunk count (`src/scheduler/worker.rs:107`, `src/scheduler/run_parallel.rs:467-479`), and `each_core_appends_into_its_own_region_of_an_accumulator_and_the_main_thread_merges_the_regions_in_order` (provisional) calls it "a different mechanism from head and tail convergence". I read the freeze as not covering it. Whether a repair waits on the freeze is op's call, not mine. What the design is does not wait.

## The point

### What the code does

The flag is copied and never read. `COMMUTATIVE` is projected at `src/plan/project.rs:162` and stored at `src/plan/mod.rs:423` into `UnitMeta.commutative` (`src/plan/unit.rs:30`). No engine source reads it; the one reader is a test asserting the copy (`tests/plan_inputs_from_bundle.rs:220-233`). `carrier_unit_outer` sets the tiling for the whole carrier if any fiber is not morsel-local (`run_parallel.rs:467-479`). So every fiber of that carrier is split, including fibers with no accumulator. Each core's accumulator binding starts with a fresh live length of zero (`src/resource/bindings/per_core.rs:135`). The api lets a unit read that length mid-frame (`hilavitkutin-api/src/context.rs:169-173`). The api defaults the flag to false (`hilavitkutin-api/src/work_unit.rs:84`). The engine design on dev already states arm (a): "Non-commutative accumulation keeps the sequential single-core fallback, routed by a commutativity hint that defaults to single-core" (`mock/crates/hilavitkutin/DESIGN.md.tmpl:646-648`). The code does not implement it, and where they disagree the code is wrong.

### Arm (a): a non-commutative unit takes one core

Canon: `ordering_is_guaranteed_by_construction` (ratified): "a non-commutative fiber runs on a single core and is trivially ordered, a commutative one runs in deterministic segments that are always in record order, and no lineariser or other ordering mechanism is needed." `a_unit_declares_whether_it_is_commutative` (ratified) as quoted. `each_convergence_thread_accumulates_into_its_own_stack_local_resource_and_non_commutative_accumulation_runs_sequentially` (ratified): non-commutative accumulation "falls back to sequential processing of that fiber". The canon favours this arm. The governing design text says it too (`DESIGN.md.tmpl:646-648`).

The unit that matters is any unit, not only the one writing the accumulator. The canon puts commutativity on the fiber, as the conjunction over all its units (both ratified rows above). The canon says nothing of carriers. Because the code splits every fiber of the carrier, any non-commutative unit in it forbids the carrier-wide split; a walk splitting only the commutative fibers would satisfy the canon equally.

### Arm (b): the tiling stays, since the merge keeps record sequence

Canon: `records_within_a_fiber_are_independent` (ratified) says record order "does not matter for correctness", except non-commutative resource accumulation. Alone, it leans to (b). But the founding spec scopes the claim: "Within any commutative fiber, records are independent" (`202603181200_topic.hilavitkutin-design-consolidation.md:1840`; no row transcribes this line). And `ordering_is_guaranteed_by_construction` answers this very case. The in-order merge is an ordering mechanism applied to non-commutative work, and that row says the design needs none because such work runs on one core. The canon puts "segments in record order" on the commutative side. Arm (b) moves it to the non-commutative side, which inverts the row. Op's words do not support (b): "ordered appends: N-way forward slices" was option 3 of the convergence question, offered and not chosen.

The code also refutes (b). The merge keeps the order of the appends. It does not keep what each append depended on. A unit that reads `len()` (`context.rs:169-173`) sees a count that restarts per region (`per_core.rs:135`). The same holds for a unit with an atomic counter in `&self`, which `Send + Sync` permits.

### Arm (c): nothing changes until the convergence measurement returns

Canon and op: op's "Measure first" decides between head-and-tail and the N-way split for single-trunk phases. The question is speed. This point is whether a non-commutative fiber may be split at all, which is correctness, and the ratified rows above already decide it. A bench prices a split; it cannot permit one. The canon does not favour (c) on the substance. Its only pull is the freeze sentence, and I handed its scope back in the gate.

### Arm (d): something else

The serious candidate infers the gate instead of reading the declaration, tiling any fiber whose units never read what they write. Canon: commutativity is declared by the unit (`a_unit_declares_whether_it_is_commutative`, ratified). Plan-time detection (`a_fiber_has_resource_accumulation_when_a_unit_writes_a_resource_it_also_reads_and_this_is_detected_at_plan_time`, ratified) decides whether per-thread accumulators are needed, not whether the fiber is commutative. Access sets cannot see a `len()` call or a counter in `&self` either. The canon does not support (d).

### Answer

(a), at fiber granularity and at every worker count. With the code's carrier-wide walk, that means a carrier tiles only when every unit of every fiber it splits declares `COMMUTATIVE`. Otherwise it runs on one core. One consequence: given the api default (`work_unit.rs:84`), every accumulator carrier that exists today goes serial under `run_parallel` until its units declare.

### What "commutative" must mean for (a) to be well defined

For a unit: what it does for each record does not depend on what it or any other unit did for other records in the same frame. Formally, for the store's merge ⊕, running the unit over s1 ++ s2 gives run(s1) ⊕ run(s2). For a fiber: every unit is commutative (ratified). For a carrier: the canon says nothing. Operationally it means every fiber the walk splits. For an append: an append is never commutative, because concatenation is ordered. The engine supplies the order by merging in record sequence; the unit does not declare it. If "commutative" meant "the append commutes", no accumulator unit could ever declare it, and the per-core regions would be dead code. For a fold into a resource: ⊕ must be associative and commutative ("an additive accumulation adds the two accumulators"), so segments may merge in any order. So the flag is a claim about records (independence), not about writes. The api doc ("writes commute across record order", `work_unit.rs:80`) should say that.

### The check that would show this wrong, and the case that must fail first

The case is a unit that keeps the default flag and, for each record, appends `ctx.accums().len()` of its own accumulator. Under `run` the values are 0 to N-1. Under `run_parallel` at two, four and the maximum worker count, the regions restart at zero (`per_core.rs:135`), so the merged values differ from `run`. This test must fail at HEAD before any repair, and pass after the gate sends this carrier to one core. If it passes at HEAD, my claim about the hazard is wrong. The reading is wrong if op chooses option 3 when the measurement returns, or if a ratified row is found that licenses splitting non-commutative fibers. In that case (b) holds for units that do not observe frame state.

`KeepWu` (`tests/gate2_accumulator.rs:101-136`) declares no flag, so it is non-commutative by default. It is a pure per-record filter, the one shape the merge handles, and `:182-219` asserts the split output equals single-core. After the repair that test still passes, but on one core, so it no longer exercises the tiling. It needs a twin that declares `COMMUTATIVE`.

## Found, not asked

1. The api revision's engine design (agent work) accepts this: "a close unit whose apply does not commute gets a result that depends on it [the worker count] too" (`git show origin/work/api-revision:mock/crates/hilavitkutin/DESIGN.md.tmpl`, lines 1409-1412). It also rests the conformance comparison on arm (b) (lines 1639-1643). Both contradict `ordering_is_guaranteed_by_construction` (ratified). Meanwhile it keeps the single-core fallback sentence (lines 681-683) that nothing implements.
2. The stack audit cites `crates/hilavitkutin/src/scheduler/worker.rs:323-348` as an ungated N-way split in the phase path (`mock/research/202611131215_stack-audit/DESIGNS.md:103`). At HEAD there is no `crates/` at the root. `mock/.../worker.rs:329-389` is the fiber-outer walk over the whole range. The single-trunk split was removed in `49ab3440` (2026-11-13 15:35), which landed after op's 06:02 "The code stays as it is meanwhile". Whether that honoured the freeze is for op.
3. The ratified trait shape has no default for the flag (`a_unit_declares_its_name_read_set_write_set_hint_commutativity_and_an_execute_function`: `const COMMUTATIVE: bool;`). The api adds `= Bool::FALSE` (`work_unit.rs:84`). No row licenses turning a mandatory declaration into a silent default.
4. Resource bindings are copied unchanged into every core's bindings (`per_core.rs:23`, `:48`), and the api offers resources only as `&T` (`context.rs:181-187`). The canon's per-thread resource accumulator has no write surface to attach to. The ratified "resource accumulation" rows describe a store the code does not have, and the `Accum` column is a different store.
5. `HeadTailConvergence` is never set past its default `Isnt` (`src/plan/fiber.rs:212`).
