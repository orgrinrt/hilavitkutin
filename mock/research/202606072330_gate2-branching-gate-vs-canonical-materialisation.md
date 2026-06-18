# Branching gate vs canonical fiber-boundary materialisation

**Date:** 2026-06-07
**Scope:** whether the #664 `branching` perf gate is reachable under the canonical engine design, and whether the planned "diamond fusion" next-build is canonical
**Oracle:** consolidation spec `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`, domain 17 flattener (`:1564-1645`), fiber grouping / spanning-tree decomposition (`:1095-1143`)
**Verdict:** the planned "cross-fiber diamond fusion" is a DEVIATION from the canonical design, not a `D4-linear`-class canonical mechanism. The breadcrumb resolution that called it "not op-gated" was drift (it reasoned from the current non-canonical dispatch + the R4d probe, against the spec). The branching gate, as specced, is most likely NOT reachable by any canonical mechanism, because the canonical design materialises the branch outputs at the fan-in by construction. This is an op-level roadmap call.

## What the spec actually says about fusion

Domain 17 (`:1564-1586`): "For **each fiber**, the flattener emits a monomorphised function. This is where the execution strategy **for ops within a fiber** is decided." The rust-pipe fusion (read inputs, pure pipeline with locals, stores grouped at end, DSE removing fiber-internal column traffic) is **within a single fiber**. And explicitly: "Multi-fiber scenarios already achieve 0.95-1.02x because **fiber boundaries are natural materialisation points, no flattening needed.**"

Fiber grouping (`:1095-1143`): a column is *fiber-internal* (written by one WU, read by the next WU **in the same fiber**, register-to-register, DSE'd) versus *fiber-input* / *fiber-output* (written/read across a fiber boundary, materialised to memory). The spanning-tree decomposition operates **within each phase**, and "natural barriers (fan-in) = mandatory fiber break points."

So the canonical design fuses inside a fiber and materialises at every fiber boundary, deliberately. It does not fuse across fibers, and a fan-in is a mandatory fiber break.

## Why the branching diamond materialises canonically

The #664 `branching` arm is the diamond `In -> {BranchX, BranchY} -> JoinZ`:
- `BranchX` reads `In`, writes `Xv`; `BranchY` reads `In`, writes `Yv`. They are column-disjoint, so block-diagonalisation places them in distinct fibers (the bench comment states this; the spec's grouper agrees, disjoint columns are not co-located).
- `JoinZ` reads `Xv` and `Yv` (two inputs), writes `Zv`. It is a fan-in.

`JoinZ` reads the outputs of two separate fibers. Therefore at least one of `Xv` / `Yv` is a *fiber-input* to `JoinZ`'s fiber and is materialised to memory. The fan-in is a "mandatory fiber break point" (`:1139`), and the convergence sits at a waist (the DAG width returns to 1 at `JoinZ`), so canonically `JoinZ` is in a later phase than the branches and **both** `Xv` and `Yv` are materialised across the waist. The most favourable legal grouping (co-locate `BranchX` with `JoinZ`) still materialises `Yv`. No canonical grouping eliminates both.

The std baseline fuses the whole diamond per element (`z = join(branch_x(i), branch_y(i))`, `x`/`y` in registers, only `z` written). It materialises neither intermediate.

## Why this makes the gate canonically unreachable for this workload

The R4d probe (finding `202606072130`) measured the diamond three ways at N=1M, checksums equal: engine 3-WU (materialises Xv/Yv) 2.94x, engine 1-hand-fused-WU (no materialisation) 1.33x, std 1.00x. So materialising the two branch outputs is ~1.6x of the gap, and the residual ~1.33x is dispatch overhead. The materialisation cost is pure memory traffic (write N + read N, twice) that scales with N and dominates the tiny per-record arithmetic.

That memory traffic is exactly what the canonical design keeps (materialise at fiber boundaries). No canonical mechanism removes it: not the compiled per-core dispatch (deviation 1's escalation; it improves dispatch quality, not the memory traffic), not trunk parallelism (R4c; it parallelises the materialisation, does not remove it), not within-fiber rust-pipe (the intermediates cross a fiber boundary, so they are not fiber-internal). The spec's "0.95-1.02x multi-fiber" claim was evidently measured on fibers whose per-record work dominates the boundary materialisation; it does not hold for a fan-in diamond of near-zero-arithmetic stages at scale.

The only things that close the branching gap are non-canonical: cross-fiber (cross-waist) fusion of the fan-in into one per-record kernel, which the spec explicitly declines.

## The drift this corrects

Breadcrumb `engine-completion-roadmap-routine` LATEST (and finding `202606072130`'s "fork resolved" update) concluded: "diamond fusion is a single-core mechanism in the same class as the D4-linear fusion, NOT op-gated; next build." That is drift. `D4-linear` fusion is canonical precisely because it is *within-fiber* (element_wise is a deep single fiber `S1->S2->S3->S4`, all fiber-internal, DSE'd; spec `:1583`). The branching diamond's fusion is *across* fibers/a waist, which the spec declines. The probe reasoned from the current dispatch + the bench, not the oracle, and so mislabelled a deviation as canonical. Per `canonical-design-outranks-intermediate-rounds`, the spec wins and the breadcrumb resolution is the thing to fix.

## The call (op-level, roadmap-shaping)

This is not a perf fork (the bench is done) and not an obvious drift-drop (dropping the diamond-fusion plan is the agent's call and is done; what remains is a design-canonicity question op owns). The branching gate, as specced against a fully-fused std baseline, tests an ideal the canonical columnar engine does not target. Three resolutions:

1. **Re-interpret the gate (recommended).** Branching parity means "canonical-dispatch-quality," i.e. the engine matches an optimal std baseline that *also* materialises the two intermediates (an honest columnar-vs-columnar comparison), not a std baseline that hand-fuses across the waist. The materialisation cost is then a documented, honest property of the columnar design (the price of independent reusable fibers + arbitrary fan-in), not a red-by-design failure. The #664 branching arm's std baseline is rewritten to materialise Xv/Yv (matching what the engine canonically does), or the arm is reclassified as a known-divergent benchmark with its gap documented.

2. **Bless cross-fiber/cross-waist fusion as a canonical design extension.** A new optimisation: detect a fan-in where all producers and the consumer are per-record-pure and fuse them across the waist into one kernel. This is a real change to the oracle (the spec currently declines it), and it is the build.rs/macro-codegen-scale work. It would close the gap to ~1.33x (probe), still above 1.10x until the residual dispatch overhead is also closed.

3. **Build the canonical compiled per-core dispatch first, then re-measure.** Spec-faithful empirical test of the "0.95-1.02x multi-fiber" claim. The probe already isolates materialisation as ~1.6x of the gap (memory traffic the compiled dispatch cannot remove), so this is unlikely to reach 1.10x, but it is the only way to falsify the spec's claim with evidence, and the compiled dispatch is canonical work that is needed regardless.

My read, grounded in `design-is-the-oracle`: the canonical engine materialises at the fan-in by construction, so option 1 is the honest resolution and option 2 is a deviation that should only be taken if op wants the engine to beat its own columnar model on fuse-shaped workloads. Option 3 is canonical work worth doing on its own merits (it is the real domain-17 dispatch the engine still owes), but it will not by itself green a gate whose baseline fuses across a waist.

## Correction (op, 2026-06-07): waist vs fiber-break, and red is fine

Op corrected two errors above; preserved here rather than rewritten (audit trail).

1. **Waist vs fiber-break conflation.** This note treated `JoinZ`'s fan-in as sitting at a *waist* forcing a phase boundary. Wrong. A waist (domain 11) detects only the points where *all* paths converge. The diamond fan-in is a within-phase fiber-break ("natural barriers (fan-in) = mandatory fiber break points", `:1139`, is a FIBER break, not a phase/waist break). Within a single phase there can be tens or hundreds of disjoint fibers that sync and start dependent fibers waiting on them; that is the canonical intra-phase structure, not a waist. So the branching diamond is one within-phase fork-join, not a waist-separated two-phase shape.

2. **The gate staying red is not a problem and not an op decision.** The std arm is the optimal performance bar; the comparison is fair as long as input and output are byte-identical and deterministic (they are), regardless of how each side computes. The engine being slower than fully-fused std on this workload is a RED light, and red is the lifeblood: it marks canonical work still owed, not a gate to rewrite or a baseline to handicap. No gate re-interpretation, no design extension. The question this note raised (three resolution options) was a mis-escalation; the answer is "leave it red, keep building the canonical engine."

3. **Materialisation is partly reducible canonically.** The claim "no canonical mechanism removes the materialisation" was too strong. The fiber grouper can co-locate one branch with the join into a single fiber (e.g. `{BranchX, JoinZ}`): then `Xv` is fiber-internal (register-to-register, DSE'd), and only the other branch's output (`Yv`) remains a materialised fiber-input. That is canonical within-fiber fusion and removes half the materialisation. Fully-fused std (zero materialisation) remains the optimal bar the columnar engine does not target; the residual gap is the honest cost of independent reusable fibers with arbitrary fan-in, and the arm stays red until the canonical mechanisms (within-fiber fan-in fusion that keeps one branch register-internal, intra-phase pipeline parallelism with progress counters, the compiled per-core dispatch) narrow it as far as canonical allows.

Disposition: the branching arm stays RED-by-design. No gate change. Resume building the canonical engine mechanisms; the diamond-fusion-that-eliminates-both-intermediates (cross-fiber) remains a non-canonical deviation and is not built, but within-fiber fan-in fusion (one branch co-located with the join) is canonical and is a legitimate narrowing step.
