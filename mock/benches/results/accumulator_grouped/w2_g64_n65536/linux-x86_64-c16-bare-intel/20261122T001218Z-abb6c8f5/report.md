# W = 2, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### per_core dominates: 40% faster than the next best (grouped)

per_core (84.82 us) leads grouped (119.00 us) by 40%, a clear separation rather than a photo finish. CV 75.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential is an outlier: 2.1x slower than the field

sequential (176.85 us) is 2.1x the fastest (84.82 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### grouped is inconsistent: worst-20% is 3.0x its best-20%

grouped's best 20% of batches run at 84.36 us but its worst 20% at 250.54 us (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: per_core** at 84821.8 ns median (-28.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.09x (fastest 84821.8 ns, slowest 176854.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 2, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112789207 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 149823ns | 119351ns | 84560ns | 103489ns | 251955ns | base |
| per_core | 108988ns | 84999ns | 71518ns | 84446ns | 164535ns | -27.26% |
| sequential | 188510ns | 177467ns | 175190ns | 177221ns | 212105ns | +25.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 149188ns | 84362ns | 250545ns | base | 0.439 |
| per_core | 108285ns | 71233ns | 162893ns | -27.42% | 0.605 |
| sequential | 187400ns | 174786ns | 209756ns | +25.61% | 0.350 |

## Performance model

- Peak throughput: **0.920 Gops/s** (per_core; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.551 | 59.9% |
| per_core | 0.773 | 84.0% |
| sequential | 0.371 | 40.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 149823ns | 149823ns | base |
| per_core | 108988ns | 108988ns | -27.26% |
| sequential | 188510ns | 188510ns | +25.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 118997ns | base | --- | [86149, 143336] | --- | --- | --- | --- |
| per_core | 84822ns | -15646.4ns (-13.1%) | [-49829, -5355]ns | [83927, 87588] | YES | 0.0003 | 0.0001 | 0 |
| sequential | 176855ns | +60404.1ns (+50.8%) | [+34656, +89835]ns | [176287, 179401] | YES | 0.0075 | 0.0075 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 85210ns | -11.6% | +108.3% |
| 1 | 2 | 88620ns | -3.7% | +98.9% |
| 1 | 3 | 143336ns | -39.5% | +23.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.137 | ok |
| per_core | -0.107 | ok |
| sequential | -0.466 | moderate- |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 157359.6ns | 149188.5ns | 105.5% | HIGH |
| per_core | 111063.5ns | 108285.1ns | 102.6% | HIGH |
| sequential | 189061.5ns | 187400.4ns | 100.9% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 84361.9-250544.9 ns)
  84361.9 |########################################
  92671.0 |
  100980.2 |
  109289.3 |######
  117598.5 |######
  125907.6 |#############
  134216.8 |
  142525.9 |####################
  150835.1 |
  159144.2 |
  167453.4 |
  175762.5 |
  184071.7 |
  192380.8 |
  200690.0 |
  208999.1 |
  217308.3 |
  225617.4 |
  233926.6 |
  242235.7 |
  (2 below, 3 above range)

per_core (n=18, range 71232.8-162892.8 ns)
  71232.8 |#############
  75815.8 |######
  80398.8 |########################################
  84981.8 |##########################
  89564.8 |
  94147.8 |
  98730.8 |
  103313.8 |
  107896.8 |
  112479.8 |######
  117062.8 |
  121645.8 |######
  126228.8 |
  130811.8 |
  135394.8 |
  139977.8 |
  144560.8 |
  149143.8 |
  153726.8 |
  158309.8 |
  (1 below, 2 above range)

sequential (n=18, range 174785.5-209755.5 ns)
  174785.5 |########################################
  176534.0 |#################################
  178282.5 |######
  180031.0 |#############
  181779.5 |
  183528.0 |
  185276.5 |
  187025.0 |
  188773.5 |
  190522.0 |
  192270.5 |
  194019.0 |
  195767.5 |
  197516.0 |
  199264.5 |
  201013.0 |
  202761.5 |
  204510.0 |
  206258.5 |
  208007.0 |
  (1 below, 3 above range)

```

## Diagnostics

- **grouped**: CV=64.7% (high variance, measurements may be unstable)
- **grouped**: bridge=106.4% of algo (FFI overhead may distort results)
- **per_core**: CV=58.8% (high variance, measurements may be unstable)
- **per_core**: bridge=100.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.9% of algo (FFI overhead may distort results)
