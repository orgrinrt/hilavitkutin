# W = 2, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (grouped) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline grouped has the worst median (3.23 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest per_core at 2.18 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### per_core dominates: 48% faster than the next best (sequential)

per_core (2.18 ms) leads sequential (3.23 ms) by 48%, a clear separation rather than a photo finish. CV 31.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### grouped is inconsistent: worst-20% is 2.3x its best-20%

grouped's best 20% of batches run at 1.86 ms but its worst 20% at 4.25 ms (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: per_core** at 2177468.6 ns median (-32.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.48x (fastest 2177468.6 ns, slowest 3230246.5 ns)

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
| grouped | 2983181ns | 3231231ns | 1864069ns | 2513921ns | 4246628ns | base |
| per_core | 2438820ns | 2178802ns | 1720471ns | 2138265ns | 3248828ns | -18.25% |
| sequential | 3308239ns | 3230994ns | 3209330ns | 3226021ns | 3481021ns | +10.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 2982314ns | 1863120ns | 4245702ns | base | 0.352 |
| per_core | 2435960ns | 1718191ns | 3245309ns | -18.32% | 0.430 |
| sequential | 3306730ns | 3208168ns | 3479089ns | +10.88% | 0.317 |

## Performance model

- Peak throughput: **0.610 Gops/s** (per_core; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.325 | 53.2% |
| per_core | 0.482 | 78.9% |
| sequential | 0.325 | 53.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 2983181ns | 2983181ns | base |
| per_core | 2438820ns | 2438820ns | -18.25% |
| sequential | 3308239ns | 3308239ns | +10.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 3230247ns | base | --- | [1895092, 3356075] | --- | --- | --- | --- |
| per_core | 2177469ns | -192023.1ns (-5.9%) | [-703449, -51797]ns | [1767571, 3074115] | YES (adj: no) | 0.1925 | 0.0963 | 0 |
| sequential | 3229300ns | no significant difference | [-124807, +1367297]ns | [3217510, 3241734] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 2629953ns | -26.4% | +22.7% |
| 1 | 2 | 3286568ns | -27.7% | -2.0% |
| 1 | 3 | 1891124ns | +40.3% | +71.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.480 | moderate- |
| per_core | -0.011 | ok |
| sequential | -0.413 | moderate- |

**Consistency summary:**

- **per_core**: won 2/3, lost 1/3
- **sequential**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 2501003.6ns | 2982314.4ns | 83.9% | HIGH |
| per_core | 2519230.5ns | 2435960.2ns | 103.4% | HIGH |
| sequential | 3229340.2ns | 3306730.0ns | 97.7% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 1863120.3-4245701.8 ns)
  1863120.3 |########################################
  1982249.4 |
  2101378.5 |
  2220507.6 |
  2339636.6 |
  2458765.7 |
  2577894.8 |
  2697023.9 |
  2816152.9 |
  2935282.0 |
  3054411.1 |
  3173540.1 |###########
  3292669.2 |############################
  3411798.3 |
  3530927.4 |
  3650056.4 |
  3769185.5 |
  3888314.6 |
  4007443.7 |
  4126572.7 |
  (1 below, 3 above range)

per_core (n=18, range 1718191.4-3245309.2 ns)
  1718191.4 |########################################
  1794547.3 |
  1870903.2 |
  1947259.1 |
  2023615.0 |#####
  2099970.9 |
  2176326.8 |
  2252682.7 |#####
  2329038.6 |
  2405394.5 |
  2481750.3 |
  2558106.2 |
  2634462.1 |
  2710818.0 |
  2787173.9 |
  2863529.8 |
  2939885.7 |#####
  3016241.6 |#################
  3092597.5 |
  3168953.4 |###########
  (1 below, 2 above range)

sequential (n=18, range 3208167.8-3479088.6 ns)
  3208167.8 |########################################
  3221713.8 |#################################
  3235259.9 |#############
  3248805.9 |######
  3262352.0 |
  3275898.0 |#############
  3289444.0 |
  3302990.1 |
  3316536.1 |
  3330082.1 |
  3343628.2 |
  3357174.2 |
  3370720.3 |
  3384266.3 |
  3397812.3 |
  3411358.4 |
  3424904.4 |
  3438450.5 |
  3451996.5 |
  3465542.5 |
  (1 below, 1 above range)

```

## Diagnostics

- **grouped**: CV=40.2% (high variance, measurements may be unstable)
- **grouped**: bridge=63.7% of algo (FFI overhead may distort results)
- **per_core**: CV=28.5% (high variance, measurements may be unstable)
- **per_core**: bridge=100.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.9% of algo (FFI overhead may distort results)
