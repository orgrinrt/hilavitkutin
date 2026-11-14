# Seat Two: Five Open Points on the Engine Path

Persona: Frank McSherry. Opus tier. Read-only. No file written, no source, design or registry row touched. Everything I read is cited below. Following the brief, I opened nothing else under the scratchpad.

## Gates, before the points

Test gate: not run, and that is my own call, so treat it as suspect. The brief forbids building or running suites, and `df` shows 3.3 GB free on `/`. I did not run the whole suite, so the green or red state of `dev` is unmeasured by this seat. I did read the test bodies in the surface these points touch:

- `tests/frame_protocol.rs:61-130` makes real assertions across 1, 2 and 3 cores and two spin budgets.
- `tests/waist_detect.rs:82-184` has two fixtures, a bowtie and a twin chain. Neither has a terminal fan-in, which is point 4's case, so that law is sampled.
- `tests/replace_swap.rs` holds five tests (`:149, :177, :251, :315, :384`), none of them ignored.

The deviation 5 audit says the aliasing pattern "is catalogued as an ignored test ... (`tests/replace_swap.rs`)" (`mock/research/202607201600_gate2-deviation-5-audit.md:48-52`). The deprecated changelist names it `parallel_swap_pattern_is_aliasing_clean` (`mock/design_rounds/202611131112/202611131117_changelist.src.deprecated.md:50`). `git grep` finds that name on neither `dev` nor `origin/work/api-revision`. Two documents point at a test that does not exist, so the only catalogue of a confirmed soundness hole is fiction.

Three fundamentals have no test on `dev` at all:

- a unit panic under `run_parallel`;
- Miri over the swap pattern (nothing in the repo mentions miri);
- incremental-skip agreement between `run` and `run_parallel`.

Under my standing rules, missing fundamentals mean I refuse. I did not refuse, because points 2, 3 and 5 ask exactly which missing test closes each hole. Refusing would withhold the very thing the refusal demands. That is my decision, and the caller should weigh it as one.

Canon gate: mixed. I checked against these:

- the amendment `mock/research/202606061000_canon-amendment-unified-engine.md`;
- the founding spec `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`;
- the ratified registry rows named below (registry at `/home/user/.worktrees/hv-canon`);
- op's quoted words.

Arms refused as misaligned: 1b, 2a, 2b, 4b, 4c. Calls handed back: 1 (whether the caller is always core 0), 3 (relocation, pending an open question) and 5 (accumulator under skip). I am the first of two seats on every "fits the canon" call here, so a second independent read is owed on each.

## 1. The shape of the one path in source

Arms: (a) one frame body that every entry reaches; (b) three walks held to one meaning by a differential; (c) none offered. I add (a′): one frame body, with the calling thread always running core 0's body at every core count.

Canon. Row `the_engine_is_one_and_the_core_count_is_configuration` (ratified) is the amendment `:7` and `:9`. It says: "The same primitives and the same code paths run at 1 core, 2 cores, or 7 cores", "Single-core mode is therefore never special-cased against multi-core mode in code", and its `rejects:` field is "a separate single-core engine or dispatch path". At one core, "the pool has one worker" and "phase sync points degenerate to one arriver" (`:7`). The founding spec's runtime is per-core programs, "spawn N threads, each runs its compiled program, join when complete" (`:1596-1617`). On fusion, row `the_flattener_emits_one_monomorphised_function_per_fiber_in_the_rust_pipe_pattern` (ratified, founding `:1564-1586`) says the flattener, per fiber, "is where the execution strategy for the units in a fiber is decided". The vocabulary agrees: "Execution strategy (fused vs sequential passes) is decided by the codegen, not the fiber definition" (`:83`).

The code has three walks that compute different things:

- `run` gates windowed fibers on the dirty mask (`src/scheduler/run.rs:130, :249-253`).
- `run_parallel`'s workers never consult it: the record-less path dispatches all-ones (`src/scheduler/worker.rs:307-328`) and the windowed path passes only the fiber mask (`:340-388`). The accumulator carrier skips the phase loop and waist barriers entirely (`:107-124`).
- `run_fused` folds the whole carrier into one chain chosen by the consumer (`src/scheduler/run_fused.rs:70-97`).

On `dev`, the core count picks which semantics you get. That is exactly the fork the ratified tenet forbids, and no ledger row records it.

Arm (b) is refused as misaligned. Keeping three walks and policing them with a differential is building the forbidden separate paths and then buying insurance on them. The api revision writes this in as design: "`run`, `run_parallel` and `run_fused` are three ways through one frame ... It is stated outside this crate, by the reference runner" (`origin/work/api-revision:mock/crates/hilavitkutin/DESIGN.md.tmpl:1627-1650`). That is agent work locked with no human in the round. It reads the ratified tenet as "agree on outputs", when the tenet says "same code paths". It is drift, and it should not land as written. The reference runner is a good test oracle. It is not an architecture.

Arm (a), as phrased, carries a defect. "The calling thread running the worker body where there is one core" makes the caller's role depend on the core count, which is a code fork at exactly the place the tenet names. Arm (a′) removes that: the caller always runs core 0's frame body, and the executor supplies the other N−1. At one core the executor supplies none, and the done-wait counts one arrival, the caller's own (the amendment's "one arriver"). This also settles the zero-worker question the design leaves open (`DESIGN.md.tmpl:1471-1482`) without clamping zero up to one.

The canon is silent on whether the caller is a worker at all. Arm (a) with the caller only ever coordinating, and the executor always supplying N ≥ 1, also satisfies the tenet. So that choice is handed back. What the canon does decide is that the caller's role must not change with the count.

`run_fused` under my answer: neither an entry nor a strategy the consumer picks. Fusion is the flattener's per-fiber strategy (ratified row above), reached inside the one body when a fiber's shape allows it. The source admits the consumer-facing entry exists only because auto-detection "is not expressible on the toolchain" (`run_fused.rs:54-59`). That is an implementation wall presented as a design. The ratified answer is that the plan selects, per fiber, among bodies it compiled. If that is not yet expressible, the honest state is a catalogued red test and a FIXME, not a public entry.

The check that would show me wrong: a sketch (committed under `mock/research/sketches/`) in which one generic frame body, with the dirty mask, plan band, fused fiber and accumulator merge all present, fails to monomorphise or devirtualise at one core, where `run` today does. That would be a toolchain wall against the tenet, and the call would go to op.

## 2. A worker panic hangs `frame_await_done`

Arms: (a) the executor catches the unwind and reports a poisoned arrival; (b) panic-free units, with the stall documented; (c) a `Clock` timeout on the done barrier; (d) a guard on the worker's frame that reports the arrival. I add (e): the frame body runs behind a boundary that cannot unwind, so a unit panic becomes a process abort.

Canon. Row `execute_returns_nothing_and_a_failing_unit_aborts_because_abort_gives_better_llvm_codegen` (ratified, founding `:2147-2153, :2448-2451`) says: "Failure aborts, and on the no_os and os tiers panic is abort already". Its mechanism note adds that the founding mention of `catch_unwind` on a std tier is something "his later answer removes". Row `there_is_no_std_platform_tier` (ratified): "There is no std tier." Failure is meant to be signalled as data, an error column or virtual that poisons dependents (founding `:2154-2167`). Op: "Abort stays, fix hang only".

So the hang exists only because the test executors and any std consumer build with unwinding, which lets a panic strand the frame instead of aborting it. Fixing the hang therefore means making the panic abort. It does not mean making the engine survive the panic.

How each arm fares:

- (a) is refused as misaligned. It needs `catch_unwind`, which the canon removed. "a poisoned flag the engine turns into its failure" is a new failure policy, the redesign op declined. `dev`'s design already leans this way: "Catching it is the executor's" (`DESIGN.md.tmpl:650-653`). That sentence is drift against the ratified row and op's ruling.
- (b) is refused. Op said to fix the hang, and documenting it does not fix it.
- (c) fails on its merits. At "millions of simulation and crunching work data going constantly" (op), a frame's length is unbounded by design, so a timeout either kills legitimate frames or never fires. It adds a tunable to guess with.
- (d) as phrased does not close the hang. A panic in phase p < nphases−1 strands the other workers in `waist_barrier`'s follower park (`src/thread/barrier.rs:65-70`), and they never reach the done barrier. The dead worker never reaches `frame_exit_arrive`, so `await_exit` in `Drop` hangs next (`src/thread/frame.rs:142-155`). And if it did return, the frame would report done over partial output, which is "continue after failure" again.

Answer: (e). The fix is to put the frame body, wherever it runs, behind a boundary that cannot unwind. Two candidate mechanisms exist:

1. Make the worker entry `extern "C"`. Unwinding out of an `extern "C"` function aborts on stable Rust (since 1.81), so the compiler supplies the abort with no std and no unwind tables on the hot path.
2. A drop guard that panics again while unwinding, which aborts under std's panic runtime.

I have not compiled either against this generic `worker_main` (`worker.rs:48-149`). Per the evidence rules, that claim is a hypothesis until someone commits a sketch. The boundary must also cover the caller's own body under 1(a′). A caller unwinding out of `run_parallel` leaves the peers parked at the waist barrier, and the unwinding `Drop` then hangs in `await_exit`. That is a second reason the one body matters here.

The check that would show me wrong: a subprocess test (abort kills the harness) with ≥2 workers and ≥2 phases, where a unit in phase 0 panics under `panic=unwind`. Before the fix the child hangs past a generous deadline. After it, the child dies by SIGABRT. If the `extern "C"` boundary does not abort under a test executor's unwinding, (e)'s first mechanism is refuted.

## 3. Workers hold `&Scheduler` across parks while the swaps take `&mut self`

Arms: (a) relocate what workers share to its own plane; (b) the worker keeps a raw pointer across the park and forms its reference per frame; (c) none offered.

Canon. No ratified row speaks to it. The question row `whether_the_maintainer_blesses_the_plane_relocation_round_for_deviations_one_five_and_six_and_the_bless_of_deviation_four` is `state: open`. The tenet `deviation_five_has_a_confirmed_aliasing_hole_...` has standing `held` and is "agent-written". Op stamped the swap "Land as provisional". So which repair to take is waiting on an open question to op, and I hand it back. On the technique, though, the arms are not even.

Arm (b) cannot close the hole, because the problem is the pointer chain, not the reference's lifetime. Every worker pointer is derived from `me`, taken in the first `run_parallel` call (`run_parallel.rs:139`). Both `WorkerCtx::sched` and the ctx pointer itself are derived this way, and the ctx is stored inside the scheduler (`:188-193`). The worker reaches the frame words through the scheduler too (`worker.rs:102`, `s.pool`). The next `replace_value(&mut self)` (`dirty.rs:96-122`) or the next `Pin<&mut Self>` reborrow retags the whole allocation through a fresh unique reference. Under Stacked Borrows that write-retag pops every tag derived from the old one, whether it was held as `&` or as raw. A worker that "forms its reference per frame after `frame_await` returns" forms it from a dead tag, and it has already read `pool.seq` through that tag to get there (`frame.rs:56`). No discipline on the worker side fixes a pointer that the owner's next exclusive borrow has already invalidated. The SAFETY comment at `worker.rs:77-84` reasons only about which fields are interior-mutable, and that does not help: `&mut` asserts uniqueness over `UnsafeCell` contents too.

Answer: (a). Everything a worker dereferences, meaning the frame words, `worker_ctxs`, the per-core publish rows, and the bindings and meta block it reads, lives in an allocation no `&mut Scheduler` covers: a provider-allocated plane, as the ledger and the audit name it (`202607201600_gate2-deviation-5-audit.md:32-44`). The FIXMEs at `dirty.rs:62-65, :113-114` describe the same thing.

What Miri has to run over, and what must fail first:

- An integration test on a std-thread test executor with ≥1 spawned worker: build, `run_parallel`, `replace_value`, `replace_resource`, `mark_dirty`, `run_parallel`, then drop.
- Run it under Miri twice, once with the default Stacked Borrows and once with `-Zmiri-tree-borrows`.
- The Linux parking path is a `libc::syscall(SYS_futex)` (`src/thread/parking.rs:164-177`), which Miri emulates. Allocation must go through a test provider, not raw mmap.
- Before the repair, this test must fail under Stacked Borrows at the worker's first access after the main thread's `&mut`.
- I cannot predict Tree Borrows with confidence. The only main-thread writes into the scheduler between frames are to `Cell` and atomic fields, which Tree Borrows may tolerate. That is why both models have to run.
- A second case must also fail before the repair: the arm (b) variant. If it passes under Stacked Borrows, my argument against (b) is refuted.

The test this needs is the one two documents claim exists and does not (see the gates section).

## 4. Whether a terminal fan-in closes a phase

Arms: (a) keep interior-only and accept one phase; (b) admit a terminal narrowing in arvo for every consumer; (c) close a phase before terminal convergence in hilavitkutin only. I add (d): keep the frame as one phase and run the join as a bridge.

Canon. Row `a_waist_is_found_by_counting_live_paths_per_topological_level_and_a_local_minimum_is_a_phase_boundary` (ratified, founding `:1306-1317`) defines it. The founding example is this exact shape: "Phase A: roots → integ_pos (3 paths → 1 waist)" (`:1321`). The converging unit is the last unit of the phase it closes, not a phase of its own. Row `a_bridge_is_a_fan_in_node_that_runs_after_its_parent_trunks_reach_the_required_record_range` (ratified, founding `:85, :745-746, :950`) places a join over several trunks as a bridge. Row `phases_overlap_through_progress_counters_so_total_time_is_the_longest_phase_plus_fill_latency` (ratified) says even phase edges overlap. Op on these benches: "no calls should be made based on those yet, they are a baseline to compare against then."

What the 1.08 actually measures. The frame has three three-stage pipelines, a join and two transforms (`mock/benches/frame_wide/support/frame-wide-support/src/engine.rs:1-8`). Occupancy is `fw_engine_par` at 1.11 heavy and 1.00 light (`frame_wide/occupancy.txt`). The engine's trunk is a "within-phase column-conflict component": two units union when one writes a column the other accesses (`src/plan/grouping.rs:289-319`). The join reads A3, B3 and C3, so it unions all three pipelines into one 10-unit trunk, leaving two 1-unit trunks. With round-robin across 4 cores, one core carries 10 of the 12 units, which is about 1.1 cores. The waist criterion is not what this number measures. The canon's trunk is a "column-disjoint maximal path" with bridges for fan-in (founding `:86, :85`). The grouping instead turns a bridge into a merge. That is the deviation.

Arms (b) and (c) are refused. Both add a phase boundary the ratified example does not draw, and a full barrier where the canon wants a bridge running as soon as its parents reach the record range. Both would be decisions taken from a bench op said decides nothing. Arm (a) is right about the phase and wrong to "accept" 1.08 cores as what that phase is worth. Answer: (d).

One caveat I cannot settle. Founding `:1394-1396` says "For ≤5 fibers: single trunk ... within a small phase, all fibers form one trunk". If this frame forms five fibers or fewer, the canon itself makes it one trunk, and its parallelism comes from convergence, which is "Measure first" (`question::which_convergence_a_single_trunk_phase_uses`, answered). Which branch applies depends on the fiber count the plan forms for this frame. I did not run the plan to get it.

The check that would show me wrong: a plan dump for `frame_wide` showing the join as a bridge with A, B and C in separate trunks, and occupancy still about 1.1. That would put the cause somewhere other than grouping.

## 5. The incremental skip and the plan band across the entries

Canon. Row `the_engine_owns_the_generation_counters_and_the_skip_logic` (ratified, founding `:2406-2408`). Step 9: "A WU whose entire predecessor set is clean ... skips execution entirely" (founding `:1418-1431`). The unified tenet in point 1 says this holds on every path. Swap spec S2 (provisional by op's "Land as provisional"): "The very next `run()` consumes that array" (`mock/research/202607200200_replaceable-swap-semantics-spec.md:42-47`).

Every walk gates on one dirty mask. Compute it once per frame between frames, on the coordinating thread, and publish it with the frame the way `plan_band` already is (`dirty.rs:133-143`, `worker.rs:135`). Workers then compose it into the member mask at `worker.rs:364-384`. On `dev`, `run_parallel` reads and clears `store_dirty` without ever gating on it (`run_parallel.rs:410-416`), so a swap's dirty cone buys nothing in parallel. Coming from an engine sold on doing work proportional to the change, that is the most expensive omission on the path.

Accumulator fiber: the canon is silent on accumulators, so I hand it back with the constraint located. Step 9's general rule says a clean unit skips. The design's per-frame reset (`run.rs:101-105`; `DESIGN.md.tmpl:1249-1250`, agent work) plus a skip would leave the accumulator empty, which differs from recomputation and is therefore wrong. That leaves two options:

1. The accumulator writer always runs, which is today's design (`run.rs:242-253`).
2. A skipped accumulator keeps last frame's appends and the reset applies only to accumulators whose writers run.

Option 2 is closer to step 9. Option 1 is simpler. Both are correct.

Fused walk after a plan-affecting swap: under point 1 there is no fused entry, so the next frame, whatever its fibers' strategy, opens the plan band (S2). Because the plan picks which fibers fuse, a fused frame that ignores the band (`run_fused.rs:77-86`; `DESIGN.md.tmpl:599-603`) skips the very recompute that would decide whether it should be fused.

The check that would show me wrong: a differential over nudged inputs (swap, mark, clean frames) at 1, 2 and 4 workers. The one path's stores must equal the never-skipping reference after every frame, and the work done must match the dirty cone's size.

## Not asked, found

- arvo's `waist_detect` uses level width (`~/.cargo/git/checkouts/arvo-da1db1860201e542/1b2796e/mock/crates/arvo-graph/src/waist.rs:1-12`). The ratified row counts live paths, "a WU that has at least one unsatisfied downstream dependency". Those are different measures, and `tests/waist_detect.rs:3-4` repeats the width wording. For `frame_wide` both give one phase. In general they differ.
- `dev`'s design keeps "Fiber failure → freeze progress → poison dependents" (`DESIGN.md.tmpl:1440-1442`) next to "abort". The founding spec reconciles them: data-flow failure poisons, a panic aborts. The design text does not state that reconciliation.
- The ratified phase overlap through progress counters is not what ships. Phases on `dev` are full barriers (`barrier.rs:47-80`), and no ledger row I read records that.
- `DESIGN.md.tmpl:1471-1482` leaves zero workers undecided, while the code clamps zero to one, which can hang. Point 1(a′) answers it as a side effect.
