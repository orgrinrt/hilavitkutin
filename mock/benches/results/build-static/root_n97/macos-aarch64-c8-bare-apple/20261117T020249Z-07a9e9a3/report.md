# build-static: rebuilt after an edit to one root contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (78005.12 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 7421.37 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 951% faster than the next best (llvm_non_incremental)

llvm_incremental (7421.37 ms) leads llvm_non_incremental (78005.12 ms) by 951%, a clear separation rather than a photo finish. CV 8.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 90% (significant)

llvm_incremental is -69969.01 ms (90%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### llvm_incremental is fastest but the noisiest (CV 8.0%)

llvm_incremental wins on median (7421.37 ms) yet has the highest variance (CV 8.0%), while llvm_non_incremental is the steadiest (CV 1.9%, 78005.12 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 10.5x the fastest

Fastest llvm_incremental (7421.37 ms) to slowest llvm_non_incremental (78005.12 ms): 10.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 7421371229.2 ns median (-90.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 10.51x (fastest 7421371229.2 ns, slowest 78005116791.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | build-static: rebuilt after an edit to one root contract crate |
| Passes | 6 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6276626c73740001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 7474244729ns | 7421378667ns | 6782916167ns | 7247302889ns | 8160321771ns | -90.45% |
| llvm_non_incremental | 78225322472ns | 78005124375ns | 76430699833ns | 77581507958ns | 80088355562ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 7474236243ns | 6782907375ns | 8160312542ns | -90.45% |
| llvm_non_incremental | 78225314986ns | 76430692458ns | 80088347604ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 7474244729ns | 7474244729ns | -90.45% |
| llvm_non_incremental | 78225322472ns | 78225322472ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 78005116792ns | base | --- | [76582480562, 80088347604] | --- | --- | --- | --- |
| llvm_incremental | 7421371229ns | -69969012750.0ns (-89.7%) | [-72666976375, -69617247104]ns | [6841024958, 8160312542] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 76430692458ns | -91.1% |
| 1 | 2 | 80352086667ns | -90.6% |
| 1 | 3 | 77847961667ns | -90.1% |
| 1 | 4 | 79824608542ns | -90.8% |
| 1 | 5 | 78162271917ns | -89.0% |
| 1 | 6 | 76734268667ns | -91.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.410 | moderate- |
| llvm_non_incremental | -0.413 | moderate- |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 423953048.6ns | 7474236243.1ns | 5.7% | HIGH |
| llvm_non_incremental | 422838104.2ns | 78225314986.1ns | 0.5% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 6782907375.0-8160312541.7 ns)
  6782907375.0 |########################################
  6851777633.3 |########################################
  6920647891.7 |
  6989518150.0 |
  7058388408.3 |
  7127258666.7 |
  7196128925.0 |
  7264999183.3 |########################################
  7333869441.7 |
  7402739700.0 |
  7471609958.4 |########################################
  7540480216.7 |
  7609350475.0 |
  7678220733.4 |########################################
  7747090991.7 |
  7815961250.0 |
  7884831508.4 |
  7953701766.7 |
  8022572025.0 |
  8091442283.4 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 76430692458.3-80088347604.2 ns)
  76430692458.3 |########################################
  76613575215.6 |########################################
  76796457972.9 |
  76979340730.2 |
  77162223487.5 |
  77345106244.8 |
  77527989002.1 |
  77710871759.4 |########################################
  77893754516.7 |
  78076637274.0 |########################################
  78259520031.2 |
  78442402788.5 |
  78625285545.8 |
  78808168303.1 |
  78991051060.4 |
  79173933817.7 |
  79356816575.0 |
  79539699332.3 |
  79722582089.6 |########################################
  79905464846.9 |
  (0 below, 1 above range)

```
