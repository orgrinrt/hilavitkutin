# GATE-2 Deviation 2: the Runtime Ownership Mask's Standing Oracles

**Date:** 2026-07-19
**Status:** the bench-gate evidence for deviation 2 under
evidence-then-bless (A2-4); seed governance item 5. This deviation is
already op-blessed; the ledger's obligation is the bench gate that keeps
the bless honest, and this record names and re-runs it.

## The deviation

Canon's fullest form bakes per-core programs at compile time; the
shipped engine dispatches per-trunk monomorphisations under a runtime
core-ownership check (the const-grouping `gate2_phase` / `gate2_trunk`
arrays read at dispatch), with build-script codegen named as the
escalation if the runtime check ever costs.

## The standing oracles, re-run

The disassembly gate (`asm_gate`, the D6 five-check over every emitted
`Scheduler::run` / `run_fused` dispatch mono under fat LTO) passed
fresh on today's tree: zero indirect calls in every dispatch body (the
hard check), indexed addressing on column loads, the morsel immediate
baked, zero helper calls. The ownership check therefore costs a
predictable L1 array read per unit visit inside a fully devirtualised
body, and cannot silently regress into indirect dispatch without the
gate failing the run. The known check-3 reports (stack-relative
accesses in two accumulator-bearing shapes) are the documented
shape-dependent non-gating class, unchanged. The second oracle is the
engine-versus-std perf gate (#664, green at single-core parity): a
mask-read cost large enough to matter shows up there as a parity
regression.

## What this record adds

The bless stands exactly as long as both oracles stay green, and the
escalation (build-script per-core program codegen, the mechanism the
GATE-2 sketches already de-risked) remains the named response to either
going red. No new bench is owed: a bespoke mask-versus-baked
micro-bench would measure the same L1 read the five-check already
bounds and the perf gate already prices in context. Registry shape at
freeze: the deviation registers as blessed-with-armed-escalation,
provenance this record plus the two oracles' artifacts.
