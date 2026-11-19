# build-static: rebuilt after an edit to one leaf implementation crate

2 variants, 3 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (5451.34 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 3379.88 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 61% faster than the next best (llvm_non_incremental)

llvm_incremental (3379.88 ms) leads llvm_non_incremental (5451.34 ms) by 61%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 38% (significant)

llvm_incremental is -2071.46 ms (38%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: llvm_incremental** at 3379876754.1 ns median (-38.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.61x (fastest 3379876754.1 ns, slowest 5451340308.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | build-static: rebuilt after an edit to one leaf implementation crate |
| Passes | 3 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6276626c73740001 |
| Counter frequency | 2300379024 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 3425541748ns | 3379895050ns | 3366595445ns | -0ns | 3425541748ns | -37.13% |
| llvm_non_incremental | 5448673459ns | 5451350701ns | 5232521422ns | -0ns | 5448673459ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 3425528922ns | 3366584144ns | 3425528922ns | -37.13% |
| llvm_non_incremental | 5448660824ns | 5232509843ns | 5448660824ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 3425541748ns | 3425541748ns | -37.13% |
| llvm_non_incremental | 5448673459ns | 5448673459ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 5451340308ns | base | --- | [5232509843, 5662132322] | --- | --- | --- | --- |
| llvm_incremental | 3379876754ns | -2071463554.0ns (-38.0%) | [-2132006453, -1865925699]ns | [3366584144, 3530125869] | YES (adj: no) | 0.2500 | 0.2500 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 5232509843ns | -35.7% |
| 1 | 2 | 5451340308ns | -38.0% |
| 1 | 3 | 5662132322ns | -37.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.126 | ok |
| llvm_non_incremental | -0.000 | ok |

**Consistency summary:**

- **llvm_incremental**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 63622747.5ns | 3425528922.4ns | 1.9% |  |
| llvm_non_incremental | 67633681.5ns | 5448660824.4ns | 1.2% |  |

## Distribution (algo ns)

```
llvm_incremental (n=3, range 3366584144.0-3425528922.4 ns)
  3366584144.0 |########################################
  3369531382.9 |
  3372478621.8 |
  3375425860.8 |
  3378373099.7 |########################################
  3381320338.6 |
  3384267577.5 |
  3387214816.5 |
  3390162055.4 |
  3393109294.3 |
  3396056533.2 |
  3399003772.1 |
  3401951011.1 |
  3404898250.0 |
  3407845488.9 |
  3410792727.8 |
  3413739966.7 |
  3416687205.7 |
  3419634444.6 |
  3422581683.5 |
  (0 below, 1 above range)

llvm_non_incremental (n=3, range 5232509842.6-5448660824.4 ns)
  5232509842.6 |########################################
  5243317391.7 |
  5254124940.8 |
  5264932489.9 |
  5275740039.0 |
  5286547588.0 |
  5297355137.1 |
  5308162686.2 |
  5318970235.3 |
  5329777784.4 |
  5340585333.5 |
  5351392882.6 |
  5362200431.7 |
  5373007980.7 |
  5383815529.8 |
  5394623078.9 |
  5405430628.0 |
  5416238177.1 |
  5427045726.2 |
  5437853275.3 |
  (0 below, 2 above range)

```
