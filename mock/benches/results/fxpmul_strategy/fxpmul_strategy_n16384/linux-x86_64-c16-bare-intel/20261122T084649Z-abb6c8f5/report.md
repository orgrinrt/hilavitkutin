# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 60% faster than the next best (fxpmul_precise)

fxpmul_hot (1.43 us) leads fxpmul_precise (2.29 us) by 60%, a clear separation rather than a photo finish. CV 34.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fxpmul_hot is fastest but the noisiest (CV 34.9%)

fxpmul_hot wins on median (1.43 us) yet has the highest variance (CV 34.9%), while fxpmul_precise is the steadiest (CV 28.8%, 2.29 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (1.43 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_hot is inconsistent: worst-20% is 1.7x its best-20%

fxpmul_hot's best 20% of batches run at 1.17 us but its worst 20% at 2.00 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 1432.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.60x (fastest 1432.9 ns, slowest 2288.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate) |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5555666677770007 |
| Counter frequency | 2112817198 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fxpmul_hot | 3313ns | 3312ns | 2713ns | 3064ns | 4661ns | base |
| fxpmul_precise | 4413ns | 4178ns | 3723ns | 4179ns | 5807ns | +33.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 1427ns | 1172ns | 1996ns | base | 11.479 |
| fxpmul_precise | 2417ns | 2042ns | 3172ns | +69.35% | 6.778 |

## Performance model

- Peak throughput: **13.974 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 11.434 | 81.8% |
| fxpmul_precise | 7.160 | 51.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 3313ns | 3313ns | base |
| fxpmul_precise | 4413ns | 4413ns | +33.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 1433ns | base | --- | [1182, 1433] | --- | --- | --- | --- |
| fxpmul_precise | 2288ns | +867.3ns (+60.5%) | [+855, +1115]ns | [2288, 2288] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 1433ns | +30.6% |
| 1 | 2 | 1172ns | +95.2% |
| 1 | 3 | 1173ns | +95.1% |
| 1 | 4 | 1433ns | +59.7% |
| 1 | 5 | 1433ns | +59.7% |
| 1 | 6 | 1173ns | +95.1% |
| 1 | 7 | 1173ns | +173.9% |
| 1 | 8 | 1433ns | +59.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.125 | ok |
| fxpmul_precise | -0.063 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 23.8ns | 1427.3ns | 1.7% |  |
| fxpmul_precise | 25.3ns | 2417.1ns | 1.0% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 1172.4-1996.0 ns)
   1172.4 |###########################
   1213.6 |
   1254.8 |
   1296.0 |
   1337.1 |
   1378.3 |
   1419.5 |########################################
   1460.7 |
   1501.9 |
   1543.0 |
   1584.2 |
   1625.4 |
   1666.6 |
   1707.7 |
   1748.9 |
   1790.1 |
   1831.3 |
   1872.5 |#
   1913.6 |
   1954.8 |
  (22 below, 9 above range)

fxpmul_precise (n=160, range 2042.0-3172.4 ns)
   2042.0 |
   2098.5 |
   2155.0 |
   2211.5 |
   2268.1 |########################################
   2324.6 |
   2381.1 |
   2437.6 |
   2494.1 |
   2550.7 |
   2607.2 |
   2663.7 |
   2720.2 |
   2776.7 |
   2833.2 |
   2889.8 |
   2946.3 |
   3002.8 |
   3059.3 |
   3115.8 |
  (19 below, 11 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=35.1% (high variance, measurements may be unstable)
- **fxpmul_precise**: CV=27.3% (high variance, measurements may be unstable)
