# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 3 ns to slowest 3 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### fxpmul_hot is fastest but the noisiest (CV 130.8%)

fxpmul_hot wins on median (3 ns) yet has the highest variance (CV 130.8%), while fxpmul_precise is the steadiest (CV 42.1%, 3 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (3 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader fxpmul_hot vs stability leader fxpmul_precise (+0% speed for 3.1x steadier)

fxpmul_hot is fastest (3 ns, CV 130.8%); fxpmul_precise gives up 0.0% median for 3.1x lower variance (CV 42.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### fxpmul_hot is inconsistent: worst-20% is 3.8x its best-20%

fxpmul_hot's best 20% of batches run at 2 ns but its worst 20% at 7 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 3.3 ns median
- Spread: 1.00x (fastest 3.3 ns, slowest 3.3 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fxpmul_hot | 2213ns | 2178ns | 2084ns | 2191ns | 2408ns | base |
| fxpmul_precise | 2285ns | 2374ns | 2104ns | 2301ns | 2420ns | +3.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 4ns | 2ns | 7ns | base | 17.908 |
| fxpmul_precise | 3ns | 2ns | 6ns | -2.97% | 18.457 |

## Performance model

- Peak throughput: **36.312 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 19.394 | 53.4% |
| fxpmul_precise | 19.394 | 53.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2213ns | 2213ns | base |
| fxpmul_precise | 2285ns | 2285ns | +3.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 3ns | base | --- | [3, 3] | --- | --- | --- | --- |
| fxpmul_precise | 3ns | no significant difference | [+0, +0]ns | [3, 3] | no | 0.0600 | 0.0600 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 3ns | +7.4% |
| 1 | 2 | 3ns | +0.0% |
| 1 | 3 | 3ns | -6.5% |
| 1 | 4 | 3ns | +27.6% |
| 1 | 5 | 5ns | -35.4% |
| 1 | 6 | 3ns | -12.1% |
| 1 | 7 | 3ns | -6.1% |
| 1 | 8 | 3ns | +86.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.056 | ok |
| fxpmul_precise | -0.021 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 4/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 3.0ns | 3.6ns | 82.8% | HIGH |
| fxpmul_precise | 3.3ns | 3.5ns | 95.8% | HIGH |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 1.8-6.7 ns)
      1.8 |
      2.0 |##############################
      2.3 |###################################
      2.5 |
      2.7 |###################################
      3.0 |
      3.2 |########################################
      3.5 |######################
      3.7 |
      4.0 |############################
      4.2 |
      4.5 |############
      4.7 |
      5.0 |######
      5.2 |####
      5.5 |
      5.7 |###
      5.9 |
      6.2 |###
      6.4 |
  (16 below, 5 above range)

fxpmul_precise (n=160, range 1.9-5.6 ns)
      1.9 |
      2.1 |#################
      2.2 |
      2.4 |######################
      2.6 |
      2.8 |########################################
      3.0 |
      3.2 |################################
      3.4 |
      3.6 |########################
      3.8 |
      3.9 |
      4.1 |##########
      4.3 |
      4.5 |#############
      4.7 |
      4.9 |##########
      5.1 |
      5.3 |##########
      5.4 |
  (14 below, 10 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=120.8% (high variance, measurements may be unstable)
- **fxpmul_hot**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **fxpmul_hot**: bridge=87.9% of algo (FFI overhead may distort results)
- **fxpmul_precise**: CV=40.0% (high variance, measurements may be unstable)
- **fxpmul_precise**: worst_20/best_20 = 3.0x (possible bimodal distribution)
- **fxpmul_precise**: bridge=100.0% of algo (FFI overhead may distort results)
