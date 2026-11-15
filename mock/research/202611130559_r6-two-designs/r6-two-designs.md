Source: kaski channel (coord/kaski of clause-dev), briefs/fixup/r6-two-designs.md, first posted 2026-11-13 05:59 UTC, read 2026-11-15.

# Head-and-tail convergence and the N-way record split, side by side

Read from `origin/dev` of hilavitkutin on 13 November 2026 (the checkout equals it). Nothing was built or run. Paths are relative to `mock/`. Three kinds of statement are kept apart: "design says" is a quote from a `DESIGN.md.tmpl`, "code does" is a file and line, "judgement" is mine. Research notes, roadmaps and design rounds are quoted as claims, not as design.

One thing first, since it changes how the rest reads. The supersession r6 recorded on 18 July was reversed on paper by the next two roadmaps a day later, and that reversal never reached the code on `dev`. So the records, the designs and the source currently say three different things about this corner.

## 1. Where the supersession is recorded

`research/202607181300_engine-roadmap-r6.md:18-19`: "The canonical 2-way mechanism turns out to be superseded by an N-way generalisation rather than missing".

`research/202607181300_engine-roadmap-r6.md:177-179`: "This supersession is the agent's call rather than op's: it is already shipped, the evidence is mechanism-level rather than aesthetic, and the deviation ledger plus this roadmap are the audit trail. Flag it for op's asynchronous review rather than blocking on it."

The reason it gives, in two parts. For fibers that accumulate, `research/202607181200_engine-arc-recomprehension.md:90-91`: "a 2-way opposite-ends converge cannot preserve record order but a forward N-way partition can". For fibers that do not, r6 `:135-136`: "Applying canon's 2-way shape there would cap parallelism at 2x to protect an invariant that case does not have."

What followed, all claims in research and rounds: `research/pre_seed/202607193000_engine-roadmap-r8.md:234-236` lists "the N-way ceil-slice in `run_core_phase` (deleted, round `202607192410`; ground is the spec's explicit \"never as an N-way record or morsel partition\"", and calls the head-and-tail types substrate whose deletion would be "the exact mistake the audit caught r6 preparing". `research/seed/execution.md:22-23` (unfrozen seed) says "Never an N-way record or morsel partition."

## 2. Head-and-tail convergence

In plain words: when a phase has only one trunk, there is nothing to hand out per core, so two threads take the same fiber from opposite ends and meet in the middle. Precisely: two walkers, one ascending from record 0 and one descending from the last record, on a fiber declared commutative; each has its own accumulator; the two are merged at the meeting point; a fiber that accumulates non-commutatively is not split at all.

Design says:
- `DESIGN.md.tmpl:82`: "Forward iter by default; Topic 3 axis E adds head+tail convergence variant for commutative single-trunk-phase fibers."
- `crates/hilavitkutin/DESIGN.md.tmpl:627-629`: "Head+tail convergence: two threads process same commutative fiber from opposite ends. Every commutative fiber gets ~2x parallelism (Topic 3 axis E)."
- `crates/hilavitkutin/DESIGN.md.tmpl:532-534`: "Convergence accumulators: each head+tail thread gets its own stack-local accumulator. Merged after convergence (addition for additive, sequential fallback for non-commutative)."
- The founding round it descends from, `design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:770-771`: "Single-trunk phases: head+tail convergence (2 threads, opposite ends, ~2x parallelism). Skip for non-commutative resource accumulation."

What it gives (design's own words): about 2x on a single-trunk commutative phase. Judgement: the meeting point can move with the relative speed of the two cores, so no split has to be computed ahead; one join; two regions.
What it costs (judgement, apart from the 2x which is the design's number): a ceiling of two cores whatever the pool size; the tail walker runs descending; the unit author must declare commutativity; an append-style accumulator cannot keep record order across the two halves, which is why the design sends non-commutative accumulation to the sequential fallback.

Code does, today: the types exist and nothing drives them.
- `crates/hilavitkutin-api/src/dispatch_codegen.rs:343-352`: `RecordRange::{Full, Head { mid_slot }, Tail { mid_slot }}`.
- `crates/hilavitkutin/src/plan/fiber.rs:145`: `HeadTailConvergence`; `:212` sets `head_tail: Maybe::Isnt`.
- `crates/hilavitkutin/src/plan/core_program.rs:111-113`: every fiber range is emitted as `RecordRange::Full`.
- `crates/hilavitkutin/src/thread/convergence.rs:15`: `Convergence`, the meeting record. `crates/hilavitkutin/src/resource/accumulator.rs:23`: `ConvergenceBuffer`.
- `crates/hilavitkutin-api/src/work_unit.rs:84`: `const COMMUTATIVE: Bool = Bool::FALSE`, copied into the plan at `crates/hilavitkutin/src/plan/mod.rs:423`. I found no dispatch or scheduler site that reads it.
- A committed sketch proves the two-cursor protocol outside the engine: `research/sketches/202606062200_e4b-head-tail-convergence/src/main.rs:172` "OUTCOME: WORKS (nightly-2026-05-28)", two std threads, every record claimed exactly once. It carries no timing.

## 3. The N-way record-order partition

In plain words: the record range is cut into one contiguous slice per core, every core walks its slice forward, and where units append, the per-core results are concatenated in core order, which is record order. Precisely: `per = ceil(total / ncores)`, `lo = min(c * per, total)`, `hi = min(lo + per, total)`; N regions, N-1 joins; no commutativity condition.

Design says, and here the per-crate design holds both names at once:
- `crates/hilavitkutin/DESIGN.md.tmpl:291-292`: "The head+tail convergence branch (a single-trunk waist-bounded phase split by record range across cores) keeps its record-slice ownership."
- `crates/hilavitkutin/DESIGN.md.tmpl:536-541`: "`run_parallel` realizes this on the no-alloc `Accum` surface as per-core regions of the single reserved buffer. [...] core c appends into the sub-region at element offset `lo_c`, the same head+tail record-slice start the morsel-local path uses (`per = ceil(total/ncores)`, `lo = (c*per).min(total)`, `hi = (lo+per).min(total)`)".
- `:547-555`: "After the phase a forward compaction concatenates the per-core live prefixes in ascending core order [...] The merged prefix is byte-identical to the single-core unit-outer `run()` append. The first slice scopes this to the at-most-one-append-per-record convergence pattern; [...] non-commutative accumulation keeps the sequential single-core fallback, routed by a commutativity hint that defaults to single-core until reachable at dispatch."
So the design text describes an N-way slice under the head-and-tail name at `:291` and `:541`, and the two-thread opposite-ends form at `:627`. Which of the two the author meant at `:291` I cannot tell from the text; git history of those lines would say who wrote them and when.

What it gives (claims from r6 and the recomprehension note, not design): every core is used; append order is preserved, so no commutativity declaration is needed for the append case. What it costs (r6's own list, `:190-202`): the slice boundary is not morsel-aligned, "false sharing on up to `ncores - 1` cache lines for narrow write columns"; there is no minimum record count per core; the branch stays on the runtime-masked walk instead of the const-monomorphised one. Judgement: the accumulator buffer is reserved at one append per record, and more than one append per record needs a per-region worst case (design `:552-554` says the same).

Code does, today, at three sites:
- `crates/hilavitkutin/src/scheduler/worker.rs:317-342`: single-trunk phase, more than one core, records present: every core walks the whole-phase mask over its ceil slice. The comment at `:318` calls it "Head+tail convergence (spec :770)". There is no commutativity test on this branch.
- `crates/hilavitkutin/src/scheduler/worker.rs:108-124` and `:195-204`: a carrier with an accumulator takes `worker_accum_unit_outer`, the same slice, per-core accumulator regions.
- `crates/hilavitkutin/src/scheduler/run_parallel.rs:316-319` recomputes the slice for the merge; `crates/hilavitkutin/src/resource/bindings/per_core.rs:32` (`rebase_accums`) and `:156` (`merge_accums`).
- `crates/hilavitkutin/tests/gate2_convergence.rs:1-8` and `:122` test the N-way slice under the name `run_parallel_single_trunk_phase_converges`.

The round that deleted the first site (`design_rounds/202607192410/`, `.history:4` "d2e45dec refactor: delete the n-way record split from run_core_phase") is on `dev` as a record only. That commit is not an object in this clone, and `2f06ffe` landed the rounds "with no source change". So r8's "deleted" is false of `dev` as it stands.

## 4. Where they differ, coexistence, evidence

Unit author: head-and-tail asks for `COMMUTATIVE` and promises nothing about order between the halves; the N-way slice asks for nothing and, in code, splits any single-trunk phase regardless of the flag. Store author: head-and-tail needs a descending walk over a column and two accumulator regions; N-way needs N regions of one reserved buffer and a forward compaction, with slice edges that may fall inside a morsel. Planner: head-and-tail needs an eligibility predicate (commutative, single trunk, record threshold, accumulation-compatible, per `plan/fiber.rs:139-140`) and a midpoint or a cursor pair; N-way needs only the core count, and today it is decided in the scheduler with no plan input.

Coexistence, judgement: `RecordRange` is `#[non_exhaustive]` and already carries `Head`/`Tail`; an N-way inhabitant would be a fourth variant or a strategy parameter chosen per fiber at plan time, with the accumulator merge keyed to the same choice. The enum's own doc (`dispatch_codegen.rs:338-340`) says a new range shape "triggers a deprecation-replacement round on this enum", so it is a design change either way. The accumulator path and the non-accumulating single-trunk path are separable: r8 itself treats the per-core accumulator regions as canonical and only the dispatch split as drift.

Committed numbers, quoted from committed files only:
- `research/202606081100_gate2-parallel-bench-fairness.md:26`: "`accumulator` parallel: passes at parity vs optimal parallel std" on an "8-core host, nightly-2026-05-28, two runs" (`:15`). This is the N-way accumulator path against std, not against head-and-tail.
- Same file `:21-24`, `wide_parallel` engine over parallel std, run 1 and run 2: 4096 records 0.765x and 0.733x; 65536 0.800x and 1.084x; 1048576 1.541x and 1.043x; 4194304 0.861x and 0.775x. That arm is several trunks, one per thread, so it does not exercise either design. r6 `:199` leans on it for the record-count floor.
- `research/sketches/202606071945_gate2-accumulator-percore-region-merge/FINDINGS.md` and `research/pre_seed/sketches/202607192200_r1-participation-without-record-slices/FINDINGS.md` are about correctness of regions and of the deletion, with no timing.

No committed evidence covers: any head-and-tail timing in the engine; any comparison of 2-way against N-way on one workload; scaling past two cores on a single-trunk phase (r6's proposed G2C-M measurement, `:206-212`, was never run that I can find); the false-sharing cost of unaligned slice edges; behaviour on P and E cores.

## 5. What depends on the choice

Code: the three slice sites and `gate2_convergence.rs` assume N-way; the unused `Head`/`Tail`, `HeadTailConvergence`, `Convergence`, `ConvergenceBuffer` and the `COMMUTATIVE` flag assume head-and-tail. Designs: `crates/hilavitkutin/DESIGN.md.tmpl:291`, `:532-556` and `:627-629`, and `DESIGN.md.tmpl:82`. Roadmap: r6's G2C-1, 1a, 1b and M build on N-way; r8's C1 (`:281-287`) builds head-and-tail and needs B2 first; the seed chapter `execution.md` forbids N-way and is not frozen.

The api revision: I found no committed description of it on `dev`, so this is judgement from the four items as briefed. Per-domain record counts: N-way slices `[0, total)` of one count and reserves the accumulator at that count, so each domain needs its own `total` and its own slice; head-and-tail needs a midpoint or cursor pair per domain. Frame-buffered column kind: either design must say whether a split walker reads the committed frame only; if it does, records are independent and neither needs commutativity for column writes. Value form as primary: `HeadTailConvergence` is written around accumulator slots and a `MergeOp`, the N-way merge around append regions; a value-returning unit would need a combine for either, ordered in the N-way case. Declared neighbour reads: a read across a slice edge, or across the meeting point, touches records another core is writing in the same phase unless the read is of the frame before.

## 6. Options

- Keep head-and-tail as the only split of a single fiber, and delete the N-way dispatch branch from `worker.rs` as round `202607192410` already planned.
- Make the N-way record-order partition the design, rewrite `:627-629` and `DESIGN.md.tmpl:82` to say so, and retire the head-and-tail types.
- Keep both as two named inhabitants chosen per fiber at plan time, with head-and-tail for commutative fibers and N-way where append order must hold.
- Decide after a measurement of both on one single-trunk workload at 2, 4 and 8 cores, and leave code and designs as they are until then.

## 7. What I could not establish

- Whether `:291` and `:541` of the per-crate design were written by you or by an agent round; I did not read the history of those lines.
- Why the source half of round `202607192410` is missing from `dev`, and whether it survives on a branch outside this clone.
- Whether `Topic 3 axis E` holds a reason for the two-thread shape beyond the lines quoted; I read the consolidation spec at `:770-771` only.
- Whether any committed test covers a single-trunk phase whose unit is non-commutative and has no accumulator, which the code splits today.
- What the api revision specifies; nothing on `dev` describes it.
- Whether the bench numbers above still hold on the current toolchain and arvo pin; they date from June.
