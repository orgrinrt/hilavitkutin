# The Bench Record: What Each Number Is, And Why

Sibling of `AUDIT.md`. This file goes through the measurements the project has on record, says for each what it actually measured and why the number came out the way it did, and then names the design calls that were drawn from numbers op has since told us are a baseline.

Op's instruction governs the whole file. The engine-versus-hand, versus-std, versus-no_std and threaded numbers "don't yet include any of the actual headline features like fusing and the adaptive trunk/fiber/morsel crunching ... no calls should be made based on those yet, they are a baseline" (channel `op-answers-21`). So this audit draws no verdict on the engine from those numbers. What it does is ask of each bench whether it measures what it claims, whether it covers the region op named (millions of records), and whether anybody already drew a call from it.

## The shape of the evidence

There are five bodies of measurement and they have very different standing.

The microkernel harness, `mock/benches/`, holds 29 families run through the mockspace bench framework with 160 warm samples per variant, medians, bootstrap confidence intervals and a sign test. The machinery is good. Not one of the 29 families runs the engine.

The resource-storage round, `mock/benches/resource_storage/`, is five families over ten variants, run twice. It is the most careful piece of measurement in the repository and it decided the resource model.

`mock/benches/engine_vs_std/` is the only thing that times the engine, and it is a timing test rather than a harness bench: no `bench.toml` entry, no committed csv, no meta.json, no result file at all. Its own `FINDINGS.md` marks itself premature and superseded at `:3-14`.

`mock/benches/results/` holds six families (fiber_theory, wake_policy, rcm_order, rcm_rivals, swap_cost, swap_band) whose variant sources are not in the working tree on `dev`. They carry 18 samples per variant against the main set's 160, and their meta.json commits are marked dirty. The sources exist on `work/recover-landing-source`, so this is an unlanded-branch problem rather than a missing-evidence problem.

The sketches, about 107 of them, carry six timer loops that the repository's own rule says make them benches (`.claude/rules/bench-and-sketch-discipline.md`: a sketch that needs a timer is a bench). Their numbers are quoted in rounds as though they were measurements.

## Why the engine-versus-std numbers are what they are

The headline is 2.20x at 4,096 records, 3.36x at 65,536 and 4.57x at 1,048,576, engine against an optimal std implementation of a four-stage chain, single core, on an Apple M1 (`mock/research/202606052000_single-core-engine-ideal-vs-actual-audit.md:83-87`). The gap widens with size.

That widening is the whole explanation and it is not mysterious. The audit note itself says the engine on `dev` at that date had neither devirtualised mega-dispatch nor fiber fusion (`:11`), so what ran was a unit-outer walk inside each morsel with every intermediate written to a column and read back (`crates/hilavitkutin/src/scheduler/run.rs:265-273`). The std arm fused the four stages into one loop. A four-stage chain over `u32` moves 16 bytes per record fused and 40 bytes per record with the three intermediates materialised. At 4,096 records everything is in L1 and the ratio is dominated by per-unit-per-morsel overhead; at 1,048,576 the arrays are out of cache and the ratio approaches the bandwidth ratio. A gap that starts at 2.2x and grows to 4.6x as the data leaves cache is the signature of materialisation, not of dispatch.

The ad-hoc spike written for this audit supports that reading on one kernel on one guest: `pass_per_stage` and the morsel-wise arm that keeps intermediate columns run 1.5x to 3.0x of a fused loop at 2^24 and 2^26 records, and the same morsel-wise structure with the intermediates in a reused scratch buffer runs 0.64x to 1.27x (`mock/research/sketches/202611131400_chain-shapes-adhoc/FINDINGS.md`). Those are ad-hoc spike numbers for a different kernel on a different machine and they measure no hilavitkutin code. They say the mechanism the design names, the co-located per-fiber arena, is the mechanism that would close this gap.

Two other things about this bench are worth stating. Its startup axis is a fixed 64 to 67 microseconds of plan build, which is 118.8x of the whole run at 4,096 records and 0.48x at a million (`:85-89`), so any ratio quoted without saying whether plan build is inside it means two different things at the two ends. And at a million records it is unstable by a factor of 5 to 7 between runs by its own account (`mock/benches/engine_vs_std/FINDINGS.md:65-67`), which is where the largest quoted ratio comes from.

## Why the parallel numbers are what they are

The first parallel claim was a 3.5x win. Op caught that it compared an N-core engine against single-threaded std, which is not a bar (`mock/research/202606080900_gate2-n-perf-finding.md:7`). Rerun against `std::thread::scope` across machine parallelism, the engine came out at parity: ratios 0.765 and 0.733 at 4,096, 0.800 and 1.084 at 65,536, 1.541 and 1.043 at 1,048,576, 0.861 and 0.775 at 4,194,304 over two runs (`mock/research/202606081100_gate2-parallel-bench-fairness.md:19-24`). The note records the remaining unfairness in the other direction: the std baseline respawns scope threads every frame while the engine has a persistent pool (`:42`).

That caveat is worth a number. In the ad-hoc spike, four threads spawned per call through `std::thread::scope` cost 5.9x to 13.1x of one thread at 65,536 records and only became a win at 2^20 and above. Per-call thread spawn dominates a short frame. So a comparison of a persistent pool against per-frame scope threads flatters the pool at small sizes by roughly the spawn cost and says nothing at large ones.

The one harness bench that measures the parallel question properly is `single-trunk-convergence` on `work/bench-convergence`, and it is the best-made measurement in the record: it states what it holds for, flags its spread, says what it did not measure, carries a correctness test with a must-fail case, and says of itself that it is a baseline and no design call rests on it (`FINDINGS.md:3,:95,:105`). Its facts: the engine cannot run head-and-tail at all because dispatch never emits those ranges (`:11`); head-and-tail beats one walker 1.58x to 1.84x (`:19-32`); at four and eight walkers the N-way form leads 1.04x to 1.88x (`:42-56`); and the engine's own sequential frame floor for one unit in one fiber is 7.1 to 11.1 microseconds at 16 to 1,024 records against 1.7 to 3.1 microseconds for a pinned single walker (`:60-71`).

That last pair is the number this audit would most like explained. About 5 microseconds per frame goes somewhere before any work is done. The candidates visible in the source are two clock reads per phase per frame (`crates/hilavitkutin/src/scheduler/run.rs:205,:296`), the accumulator reset (`:105`), the dirty-unit propagation, and the fiber member mask rebuilt per phase and fiber (`:232-242`, with its own FIXME). For kaski's budget, 10 milliseconds per tick, 5 microseconds per frame is nothing; for a daemon running thousands of small frames it is the whole cost. Nobody has attributed it.

## The resource-storage round

This is the one that decided a design, and it holds up. Six variants of how a resource is laid out, run against each other on three workloads at three sizes, twice, with the arms producing a byte-exact identical output so the comparison is real.

The decisive numbers are the wide gather, where the decomposed layout loses 3.06x to the blob (`mock/benches/resource_storage/rsb_bwide_n4194304_findings.md:8-10`), and the erased addressing, which came within about 1.7 percent of native (`mock/research/seed/storage.md:60-67`). Op ruled on the hybrid from those (`mock/design_rounds/202607161000/...hybrid-decision:12-19`). That is how a decision from a bench should look.

Two cautions attach to it. The clean and thrash families produce differences of 0.4 to 1.2 percent, and the two runs of the same matrix on different OS builds name different fastest arms at that level, so everything inside about 2 percent in that round is noise and should not be quoted. And the wide-gather comparison across sizes compares different amounts of work: the pass count falls by 250x from the smallest size to the largest (`kernels-shared/src/lib.rs:610-612`), so the N axis there is not a size sweep.

## The microkernel families, and the four whose numbers do not mean what the label says

The 29 families are well run and several of them are asking the wrong question of the arm.

`bit_count_ops` applies `v | 1` to the input in all three arms before counting trailing zeros, so the count is constant zero and the loop has nothing to do; the medians are 0.0 ns with 29 to 32 percent ties, which is one timer step of 41.67 ns divided by a hundred-call batch. `NIGHT_FINDINGS_2026-05-11.md:38` draws "arvo `BitAccess::trailing_zeros` canonical-intrinsic lowering is empirically optimal" from that, and the committed file says in its own words that the arms are below timer resolution.

`bounds_check` runs a multiply dependency chain in all three arms, latency bound at three to four cycles per step, which can hide a one-cycle bounds check entirely. `NIGHT:33` draws "LLVM elides safe indexing over const-sized arrays completely" from it, and no disassembly is committed anywhere in the folder for that family.

The `dispatch_*` families mark every work function `#[inline(never)]`, including the arm named direct, so what is compared is three kinds of call rather than a call against no call. The differences are 0.9 to 1.3 percent, which is inside the round-to-round spread of the harness.

`dep_graph_csr_9` has its dense arm zero-initialise a 64 KB array inside the timed region, and the hybrid arm is dense by construction below N=16. The CSR-versus-dense conclusion survives that (the gap is 5x to 59x), but `NIGHT:70` says the hybrid measured within noise of pure CSR while the committed files give the hybrid at +407 percent at N=8 and +548 percent at N=16.

None of this makes the harness bad. It makes the hand-written synthesis on top of it (`NIGHT_FINDINGS_2026-05-11.md`) a document whose claims need checking against the committed files one at a time, which is the opposite of what a synthesis is for.

## Where a call was drawn from a baseline number

Op's instruction is that no call may rest on the engine-versus-hand numbers yet. The audit was asked to name the places where one already was. These are the ones the record shows.

The founding execution-model topic drew the whole shape of the engine from a results table on one M1: sequential 40.4 ms down to write-sized 16.8 ms at a million agents, the 0.42x headline, the dispatch penalties of 12.6x for struct-field function pointers and 5.8x for a const-generic array, and the write-only morsel sizing at about 5 percent (`design_rounds/202604200055/202603171900_topic.execution-model.md:157-168,:257-269,:437-466`). No committed result file is named for any of it. The 12.6x in particular is still quoted in the build crate's design as the reason `ExpandedLto` is mandatory (`mock/crates/hilavitkutin-build/DESIGN.md.tmpl:115`).

The flattener figure of 0.95 to 0.96x, described in the seed as beating hand-written assembly (`mock/research/seed/dispatch.md:83-94`), is from the same table and the same machine.

Prefetch and the `likely` hint are banned on the strength of two measurements on Apple Silicon, about 2x slower and 1.40x slower (`design_rounds/202604200055/202603171900_topic.execution-model.md:412-427`). Both are microarchitecture-specific results generalised into a rule for every target.

Row sorting was rejected at 10.7 percent slower from the same source (`:409`), and the RCM question was later re-measured properly and came out as 1.65x at 64 MiB columns and near-neutral below (`mock/research/202607200800...:62-66`), which is a different answer at a different size.

The enum-dispatch rule, at most 8 variants, and "batch by variant" for larger spaces come from `match_arm_count` and `fnptr_table_dispatch` (`NIGHT:20`). The first is a plan-stage rule drawn from a microkernel at N=16,384; no bench of the batching rule exists.

CSR as canonical at every N, with the hybrid retired, comes from `dep_graph_csr_9` (`NIGHT:15,:70-71`). The variant crate and its bench.toml entry are still present.

The wake default of park immediately with a spin budget of zero was set from medians of 1.64x, 1.34x and 1.11x (`mock/research/202607202340_wake-policy-evidence.md:34-66`), and the re-examination found that `spin_loop` lowers to an instruction-synchronisation barrier on aarch64 costing 9 to 20 ns against about 0.4 ns for a plain load, that control arms differed by up to 32 percent, and retired the margins as point estimates (`mock/research/202607210100_wake-policy-reexamination.md:11-19,:62-64`). The default stayed. The direction (park rather than spin) is reproducible across seven policies and three sizes and is probably right; the number behind it is not a number any more.

The strategy thresholds (under 10,000 records sequential, over 100,000 consider threading) come from the founding table (`consolidation:1906-1909`) and are still in the design. `LIGHT_THRESHOLD` is marked bench-pending and `DefaultSelector` has no selection logic behind it (`mock/research/202607202100...:30-34`), so the thresholds are not actually dispatched on.

The spectral-versus-greedy role swap rests on `fiber_theory`, where greedy builds a plan in 38 to 398 ns and spectral in 17.2 to 128.2 microseconds, a 322x to 686x difference (`mock/research/202607201200_spectral-role-bench-evidence.md:30-35`). The note states its own scope honestly: a bare DAG with no access matrix (`:84-89`). Plan-build time is the wrong axis for a choice about execution locality, and the fiber and cut counts in the same table (spectral gives 2 fibers and 3 to 8 cuts where greedy gives 45 to 51 fibers and 110 to 124 cuts at 64 units) point the other way.

The erased-addressing and blob decisions rest on the resource-storage round, which is the one place in this list where the bench measures the thing the call is about.

## The instruments themselves

The gate that five of those claims rest on, the five-check disassembly gate, exists in source (`mock/benches/src/disasm_5check.rs`, `src/bin/asm_gate.rs`) and has no committed output anywhere in the tree. The claim that all 26 dispatch functions pass all five checks (`design_rounds/202604200055/202603171900_topic.execution-model.md:170-179`) and the claim of zero indirect calls on a fresh tree (`mock/research/202607202200_gate2-deviation-2-evidence.md:22-27`) both name no artifact. The gate also only runs for `dispatch_*` benches, so it has never been pointed at the engine.

The perf gate itself (`mock/benches/engine_vs_std/tests/perf_gate.rs`) carries fixed expected-ratio ceilings as source constants, with no result file saying which machine produced them, and the tests are `#[ignore]`.

The engine's test suite is the weakest instrument of all, and `AUDIT.md` marks it back to the drawing board for these reasons: twelve tests discard the build outcome (`crates/hilavitkutin/tests/scheduler_builder.rs:14-16` and the pattern through `:502`), the incremental path asserts counts rather than values (`tests/incremental_skip.rs:4,:111`), three test files have empty bodies (`plan_chain.rs`, `adapt_subsystem.rs`, `loom_atomics.rs`), fourteen tests are ignored, and there is no compile-fail fixture for the headline guarantee that an undeclared access does not compile. The api revision branch is the only place where compile-fail fixtures exist (eleven of them), which is the right pattern and should come back to the trunk.

## What this means for the next measurement

The catalogue in `BENCH-CATALOGUE.md` says what to build. The two things that would change the most: an engine arm in the region op named, sizes centred on 2^20 to 2^27 rather than 4,096 to a million, with the plan build outside the timed region and stated separately; and an oracle for the plan chain, so that a plan can be wrong rather than merely different.
