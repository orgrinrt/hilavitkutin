# build-static: every crate built into an empty target directory

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (llvm_non_incremental)

The baseline llvm_non_incremental is the fastest (16708.63 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader llvm_non_incremental vs stability leader llvm_incremental (+5% speed for 1.7x steadier)

llvm_non_incremental is fastest (16708.63 ms, CV 3.9%); llvm_incremental gives up 4.8% median for 1.7x lower variance (CV 2.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (llvm_non_incremental) is the fastest** at 16708632225.0 ns median
- Spread: 1.05x (fastest 16708632225.0 ns, slowest 17507869819.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | build-static: every crate built into an empty target directory |
| Passes | 6 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6276626c73740001 |
| Counter frequency | 2100417682 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 17460504227ns | 17507873929ns | 16791132445ns | 17397715499ns | 17889373209ns | +3.32% |
| llvm_non_incremental | 16899191031ns | 16708636526ns | 16165167291ns | 16582765297ns | 17740841501ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 17460500469ns | 16791129098ns | 17889369684ns | +3.32% |
| llvm_non_incremental | 16899186907ns | 16165163367ns | 17740837164ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 17460504227ns | 17460504227ns | +3.32% |
| llvm_non_incremental | 16899191031ns | 16899191031ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 16708632225ns | base | --- | [16248091332, 17740837164] | --- | --- | --- | --- |
| llvm_incremental | 17507869819ns | no significant difference | [-299532802, +1451887963]ns | [16984261903, 17889369684] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 16165163367ns | +6.3% |
| 1 | 2 | 16873320303ns | -0.5% |
| 1 | 3 | 16331019296ns | +10.9% |
| 1 | 4 | 18054876885ns | -2.9% |
| 1 | 5 | 16543944147ns | +6.8% |
| 1 | 6 | 17426797443ns | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.173 | ok |
| llvm_non_incremental | -0.469 | moderate- |

**Consistency summary:**

- **llvm_incremental**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 72439894.2ns | 17460500468.8ns | 0.4% |  |
| llvm_non_incremental | 37861293.2ns | 16899186906.8ns | 0.2% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 16791129098.5-17889369684.5 ns)
  16791129098.5 |########################################
  16846041127.8 |
  16900953157.1 |
  16955865186.4 |
  17010777215.7 |
  17065689245.0 |
  17120601274.3 |
  17175513303.6 |########################################
  17230425332.9 |
  17285337362.2 |
  17340249391.5 |
  17395161420.8 |
  17450073450.1 |########################################
  17504985479.4 |########################################
  17559897508.7 |
  17614809538.0 |
  17669721567.3 |########################################
  17724633596.6 |
  17779545625.9 |
  17834457655.2 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 16165163366.7-17740837163.7 ns)
  16165163366.7 |########################################
  16243947056.6 |
  16322730746.4 |########################################
  16401514436.2 |
  16480298126.1 |########################################
  16559081816.0 |
  16637865505.8 |
  16716649195.6 |
  16795432885.5 |########################################
  16874216575.3 |
  16953000265.2 |
  17031783955.0 |
  17110567644.9 |
  17189351334.8 |
  17268135024.6 |
  17346918714.4 |
  17425702404.3 |########################################
  17504486094.1 |
  17583269784.0 |
  17662053473.8 |
  (0 below, 1 above range)

```
