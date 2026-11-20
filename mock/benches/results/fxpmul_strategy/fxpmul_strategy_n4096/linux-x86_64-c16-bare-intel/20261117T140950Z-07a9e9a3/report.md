# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 59% faster than the next best (fxpmul_precise)

fxpmul_hot (364 ns) leads fxpmul_precise (578 ns) by 59%, a clear separation rather than a photo finish. CV 76.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fxpmul_hot is fastest but the noisiest (CV 76.1%)

fxpmul_hot wins on median (364 ns) yet has the highest variance (CV 76.1%), while fxpmul_precise is the steadiest (CV 64.7%, 578 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fxpmul_hot shows alternating (throttle bounce) (autocorr -0.68)

fxpmul_hot's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (214 ns) is smaller than the fastest variant's own run-to-run std-dev (277 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (364 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_hot is inconsistent: worst-20% is 3.2x its best-20%

fxpmul_hot's best 20% of batches run at 298 ns but its worst 20% at 954 ns (3.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 364.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.59x (fastest 364.3 ns, slowest 578.0 ns)

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
| Counter frequency | 2112548743 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fxpmul_hot | 2862ns | 2248ns | 1837ns | 2188ns | 5907ns | base |
| fxpmul_precise | 2909ns | 2459ns | 2013ns | 2324ns | 5557ns | +1.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 463ns | 298ns | 954ns | base | 8.846 |
| fxpmul_precise | 681ns | 473ns | 1296ns | +47.05% | 6.016 |

## Performance model

- Peak throughput: **13.743 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 11.243 | 81.8% |
| fxpmul_precise | 7.087 | 51.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2862ns | 2862ns | base |
| fxpmul_precise | 2909ns | 2909ns | +1.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 364ns | base | --- | [364, 364] | --- | --- | --- | --- |
| fxpmul_precise | 578ns | +213.6ns (+58.6%) | [+175, +214]ns | [578, 578] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 364ns | +58.7% |
| 1 | 2 | 364ns | +58.7% |
| 1 | 3 | 298ns | +58.7% |
| 1 | 4 | 364ns | +29.8% |
| 1 | 5 | 298ns | +58.7% |
| 1 | 6 | 364ns | +58.7% |
| 1 | 7 | 298ns | +93.9% |
| 1 | 8 | 364ns | +29.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.675 | HIGH- (thermal bounce) |
| fxpmul_precise | 0.125 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 30.4ns | 463.0ns | 6.6% | HIGH |
| fxpmul_precise | 28.2ns | 680.9ns | 4.1% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 298.0-953.8 ns)
    298.0 |#############
    330.8 |
    363.6 |########################################
    396.4 |
    429.2 |
    462.0 |
    494.8 |
    527.6 |
    560.4 |
    593.1 |
    625.9 |
    658.7 |
    691.5 |
    724.3 |
    757.1 |
    789.9 |
    822.7 |
    855.5 |
    888.3 |#
    921.0 |
  (19 below, 18 above range)

fxpmul_precise (n=160, range 472.8-1295.5 ns)
    472.8 |#########################
    514.0 |
    555.1 |########################################
    596.2 |#
    637.4 |
    678.5 |
    719.6 |
    760.8 |
    801.9 |
    843.0 |
    884.2 |
    925.3 |
    966.4 |
   1007.6 |
   1048.7 |#
   1089.8 |
   1131.0 |
   1172.1 |
   1213.2 |
   1254.4 |
  (16 below, 19 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=59.8% (high variance, measurements may be unstable)
- **fxpmul_hot**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **fxpmul_precise**: CV=55.0% (high variance, measurements may be unstable)
