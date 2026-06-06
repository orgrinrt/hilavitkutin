# E1/E7 dirty-gated RunFiber walk — findings

Hypothesis: the real type-level `RunFiber` cons-walk can be gated per-WU by a
runtime dirty mask threaded by carrier position (`if dirty.bit(pos) { project +
invoke }`) without losing devirtualisation. This is the one step neither the E4
meta-WU sketch nor the E7 dirty-skip sketch covered: gating the real walk by a
per-WU dirty bit while keeping the monomorphised straight-line body.

Shape: a `RunFiberDirty`-style variant copying the real `RunFiber` bound block
(`hilavitkutin::dispatch::fiber_run`), adding a `dirty & (1<<pos)` gate around
the `EngineCtx::project` + `invoke_wu_in_fiber` pair, recursing with `pos + 1`.
A 3-WU column DAG: P0 (In -> A), P1 (A -> B), P2 (In -> C). Four dirty-mask
cases: all-dirty, P2-only, AB-cone, clean-frame. A `#[inline(never)] gated_run`
isolates the walk for objdump.

## Outcome: WORKS

Runtime: the gated walk ran exactly the right WUs each case. All-dirty wrote
A, B, C. P2-only wrote C and left A, B untouched (P0, P1 skipped). AB-cone wrote
A, B and skipped C. The clean frame ran nothing. Skip == output column
untouched, matching canonical spec Step 9's "skips execution entirely".

Devirt: `objdump -d` of the `gated_run` mono (2840 instructions on
aarch64-release) shows **zero `bl` and zero `blr`**: no direct or indirect
calls. The dirty gate lowers to a predicated branch around the inlined
project + invoke; the type-level walk still collapses into one straight-line
body. The runtime skip costs only the bit test.

This unblocked the real E1/E7 round (`mock/design_rounds/202606111200_*`): the
shipped `RunFiber::run_gated` (`dispatch/fiber_run.rs`) and the
`Scheduler::run` / `run_fused` seed-propagate-gate use exactly this shape, and
the shipped engine's `incremental_skip` integration test plus the perf gate
(element_wise green, branching/accumulator red) confirm it end to end.
