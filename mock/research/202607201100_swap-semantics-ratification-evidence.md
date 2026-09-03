# Swap Semantics: Implementation and Bench Evidence for Ratification

**Date:** 2026-07-19
**Round:** `202607200500_topic.replaceable-swap-semantics.md` (#697,
absorbing the A2-3 `PlanAffecting` ruling, #696)
**Spec under ratification:** S1 to S7 of
`202607200200_replaceable-swap-semantics-spec.md`. Op held ratification
until the spec was implemented and benched; this memo is that evidence.

## What shipped (both changelists locked)

S1: `replace_value` installs the whole value as one blob write through
the same `Selector` witness the drain wrote through, plus the
store-dirty mark; no other pointer derivation exists on the swap path.
S2: `PlanAffecting` unsealed to an open marker on resource value types;
`replace_resource` performs the identical install plus the plan-dirty
bit keyed by the store-position witness; `run` and `run_parallel` enter
the leading plan band on first frame or a consumed bit, publishing the
decision for the worker phase loops. S4: the crate ships
`impl<T: PlanAffecting> !Replaceable for T` under `negative_impls`
(WATCH tier), with an executable `compile_fail,E0751` doctest. S6: five
tests pin blob-address identity, last-swap-wins, ZST swap, install
through `replace_resource`, and the band asymmetry; the previously
catalogued install test unignores. The three A2-5 mis-grounded
artifacts (engine DESIGN swap paragraphs, the `replace_value` FIXME,
the catalogued ignore reason) now state the spec. S3 (collection
element writes) and the Swap-D arm stay gated on #344 by name. All 73
engine test targets green.

## Bench evidence (harness, one clean run, M1)

Install cost (`swap_cost`, timed body is exactly one `replace_value`):

| payload | median |
|---|---|
| 64 B | 2 ns |
| 1 KiB | 85 ns |
| 64 KiB | 1.81 us |

The install is the memcpy it should be, scaling linearly with payload
size, with the witness resolution compiled away at the small end.

Next-frame asymmetry (`swap_band`, one carrier: a producer-to-consumer
cone off the `Replaceable` value, an unrelated chain that skips, an
`OnMeta<PlanStage>` unit carrying a fixed synthetic recompute cost):

| records | clean skip | plan swap | value swap |
|---|---|---|---|
| 64K | 9.95 us | 14.03 us | 22.23 us |
| 1M | 9.38 us | 12.88 us | 172.59 us |
| 8M | 10.90 us | 14.34 us | 2.61 ms |

Each path pays exactly its own channel and nothing else, which is the
S5 asymmetry claim confirmed structurally: the clean-skip baseline is
flat and size-independent; the plan swap pays baseline plus the band
(flat, record-count-independent, band cost only); the value swap pays
baseline plus its dirty cone, scaling linearly with the records the
cone touches, and never pays the band.

One honest caveat on magnitudes: the plan band's content today is the
synthetic stand-in unit, because the adapt-subsystem plan recompute
that will fill the band is sequenced later. The bench therefore proves
the CHANNEL separation (cone cost scales with data, band cost with
plan work, neither leaks into the other), not the eventual absolute
cost of a real recompute. When the recompute lands, the same bench
re-runs unchanged and the plan-swap column absorbs the real number.

## Upstream state (asked and answered)

All settled bench-framework work is merged on mockspace dev: PR #269
(the full feature batch through docgen with graphviz), PR #270 (the
`timed!` setup fix, which these benches' untimed swap setup uses
directly), PR #271 (the builtin bench-and-sketch rule), PR #278
(subprocess validation, bounded orchestrator memory), PR #279
(transactional results, crash-borne trees auto-void).

## The ask

Ratify S1 through S7 as the swap-semantics canon. On ratification, the
seed storage chapter replaces its open-question wording with the spec,
governance item 1 (and the A2-3 remainder in item 6) closes, and #696
plus #697 complete. S3 and Swap-D remain #344-gated inside the ratified
spec by name, not as open questions.
