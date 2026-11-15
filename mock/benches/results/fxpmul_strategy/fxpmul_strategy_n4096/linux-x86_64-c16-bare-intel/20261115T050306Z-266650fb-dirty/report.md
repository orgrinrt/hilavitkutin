# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 59% faster than the next best (fxpmul_precise)

fxpmul_hot (364 ns) leads fxpmul_precise (578 ns) by 59%, a clear separation rather than a photo finish. CV 74.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fxpmul_hot is fastest but the noisiest (CV 74.5%)

fxpmul_hot wins on median (364 ns) yet has the highest variance (CV 74.5%), while fxpmul_precise is the steadiest (CV 68.6%, 578 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (214 ns) is smaller than the fastest variant's own run-to-run std-dev (271 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (364 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_hot is inconsistent: worst-20% is 2.9x its best-20%

fxpmul_hot's best 20% of batches run at 333 ns but its worst 20% at 971 ns (2.9x) - a bimodal or bursty profile the median hides.

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
| Counter frequency | 2112754895 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fxpmul_hot | 2961ns | 2242ns | 2047ns | 2244ns | 6024ns | base |
| fxpmul_precise | 2995ns | 2454ns | 2006ns | 2383ns | 5822ns | +1.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 480ns | 333ns | 971ns | base | 8.540 |
| fxpmul_precise | 703ns | 473ns | 1361ns | +46.64% | 5.824 |

## Performance model

- Peak throughput: **12.293 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 11.243 | 91.5% |
| fxpmul_precise | 7.087 | 57.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2961ns | 2961ns | base |
| fxpmul_precise | 2995ns | 2995ns | +1.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 364ns | base | --- | [364, 364] | --- | --- | --- | --- |
| fxpmul_precise | 578ns | +213.6ns (+58.6%) | [+214, +214]ns | [578, 578] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 364ns | +29.8% |
| 1 | 2 | 298ns | +93.9% |
| 1 | 3 | 364ns | +58.6% |
| 1 | 4 | 364ns | +58.7% |
| 1 | 5 | 364ns | +29.8% |
| 1 | 6 | 364ns | +29.8% |
| 1 | 7 | 364ns | +58.6% |
| 1 | 8 | 364ns | +58.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.160 | ok |
| fxpmul_precise | 0.058 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 31.3ns | 479.6ns | 6.5% | HIGH |
| fxpmul_precise | 29.0ns | 703.3ns | 4.1% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 333.2-970.7 ns)
    333.2 |########################################
    365.1 |##
    397.0 |
    428.8 |
    460.7 |
    492.6 |
    524.5 |
    556.3 |
    588.2 |
    620.1 |
    651.9 |
    683.8 |
    715.7 |
    747.6 |
    779.4 |
    811.3 |
    843.2 |
    875.1 |
    906.9 |
    938.8 |
  (15 below, 21 above range)

fxpmul_precise (n=160, range 472.9-1360.8 ns)
    472.9 |#########
    517.3 |
    561.7 |########################################
    606.1 |
    650.5 |
    694.9 |
    739.3 |
    783.7 |
    828.1 |
    872.5 |
    916.9 |
    961.2 |
   1005.6 |
   1050.0 |
   1094.4 |
   1138.8 |
   1183.2 |
   1227.6 |
   1272.0 |
   1316.4 |
  (27 below, 19 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=56.6% (high variance, measurements may be unstable)
- **fxpmul_precise**: CV=56.4% (high variance, measurements may be unstable)
