# build-static: rebuilt after an edit to one leaf implementation crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (6408.94 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 4186.59 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 53% faster than the next best (llvm_non_incremental)

llvm_incremental (4186.59 ms) leads llvm_non_incremental (6408.94 ms) by 53%, a clear separation rather than a photo finish. CV 4.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 35% (significant)

llvm_incremental is -2239.85 ms (35%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: llvm_incremental** at 4186590268.8 ns median (-34.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.53x (fastest 4186590268.8 ns, slowest 6408942961.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | build-static: rebuilt after an edit to one leaf implementation crate |
| Passes | 6 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6276626c73740001 |
| Counter frequency | 2100332830 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 4161942070ns | 4186594250ns | 3887519206ns | 4122792206ns | 4357878297ns | -34.60% |
| llvm_non_incremental | 6364280889ns | 6408946795ns | 5839238352ns | 6333191160ns | 6673436753ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 4161937644ns | 3887514419ns | 4357873581ns | -34.60% |
| llvm_non_incremental | 6364275712ns | 5839234086ns | 6673429475ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 4161942070ns | 4161942070ns | -34.60% |
| llvm_non_incremental | 6364280889ns | 6364280889ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 6408942961ns | base | --- | [6010454701, 6673429475] | --- | --- | --- | --- |
| llvm_incremental | 4186590269ns | -2239853249.1ns (-34.9%) | [-2502112967, -1865047988]ns | [3941349083, 4357873581] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 5839234086ns | -31.6% |
| 1 | 2 | 6735142786ns | -36.1% |
| 1 | 3 | 6460310875ns | -31.7% |
| 1 | 4 | 6611716164ns | -38.3% |
| 1 | 5 | 6357575047ns | -38.9% |
| 1 | 6 | 6181675316ns | -30.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.108 | ok |
| llvm_non_incremental | -0.263 | moderate- |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 311714650.9ns | 4161937644.4ns | 7.5% | HIGH |
| llvm_non_incremental | 316833911.4ns | 6364275712.3ns | 5.0% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 3887514418.7-4357873581.1 ns)
  3887514418.7 |####################
  3911032376.8 |
  3934550334.9 |
  3958068293.1 |
  3981586251.2 |####################
  4005104209.3 |
  4028622167.4 |
  4052140125.5 |
  4075658083.6 |####################
  4099176041.8 |
  4122693999.9 |
  4146211958.0 |
  4169729916.1 |
  4193247874.2 |
  4216765832.3 |
  4240283790.5 |
  4263801748.6 |
  4287319706.7 |########################################
  4310837664.8 |
  4334355622.9 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 5839234085.5-6673429474.9 ns)
  5839234085.5 |########################################
  5880943855.0 |
  5922653624.4 |
  5964363393.9 |
  6006073163.4 |
  6047782932.9 |
  6089492702.3 |
  6131202471.8 |
  6172912241.3 |########################################
  6214622010.8 |
  6256331780.2 |
  6298041549.7 |
  6339751319.2 |########################################
  6381461088.6 |
  6423170858.1 |########################################
  6464880627.6 |
  6506590397.1 |
  6548300166.5 |
  6590009936.0 |########################################
  6631719705.5 |
  (0 below, 1 above range)

```
