# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot is fastest but the noisiest (CV 29.0%)

fxpmul_hot wins on median (5 ns) yet has the highest variance (CV 29.0%), while fxpmul_precise is the steadiest (CV 25.7%, 5 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (1 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (5 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_precise's comparison is tie-heavy (10% tied pairs)

10% of paired samples for fxpmul_precise are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader fxpmul_hot vs stability leader fxpmul_precise (+4% speed for 1.1x steadier)

fxpmul_hot is fastest (5 ns, CV 29.0%); fxpmul_precise gives up 4.3% median for 1.1x lower variance (CV 25.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### fxpmul_hot is inconsistent: worst-20% is 2.6x its best-20%

fxpmul_hot's best 20% of batches run at 2 ns but its worst 20% at 6 ns (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 4.6 ns median
- Spread: 1.04x (fastest 4.6 ns, slowest 4.8 ns)

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
| fxpmul_hot | 2143ns | 2126ns | 2116ns | 2129ns | 2212ns | base |
| fxpmul_precise | 2131ns | 2123ns | 2116ns | 2124ns | 2170ns | -0.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 4ns | 2ns | 6ns | base | 14.862 |
| fxpmul_precise | 5ns | 3ns | 6ns | +8.98% | 13.637 |

## Performance model

- Peak throughput: **26.947 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 13.913 | 51.6% |
| fxpmul_precise | 13.333 | 49.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2143ns | 2143ns | base |
| fxpmul_precise | 2131ns | 2131ns | -0.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 5ns | base | --- | [4, 5] | --- | --- | --- | --- |
| fxpmul_precise | 5ns | no significant difference | [+0, +1]ns | [5, 5] | no (adj: YES) | 0.0369 | 0.0369 | 16 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 5ns | +0.0% |
| 1 | 2 | 2ns | +100.0% |
| 1 | 3 | 4ns | +23.8% |
| 1 | 4 | 5ns | -8.0% |
| 1 | 5 | 5ns | -8.3% |
| 1 | 6 | 5ns | +0.0% |
| 1 | 7 | 5ns | +0.0% |
| 1 | 8 | 5ns | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.072 | ok |
| fxpmul_precise | 0.300 | moderate+ |

**Consistency summary:**

- **fxpmul_precise**: won 2/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 3.6ns | 4.3ns | 84.2% | HIGH |
| fxpmul_precise | 2.7ns | 4.7ns | 56.5% | HIGH |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 2.4-6.1 ns)
      2.4 |################
      2.6 |
      2.7 |####################
      2.9 |
      3.1 |
      3.3 |##########################
      3.5 |
      3.7 |##################
      3.9 |
      4.0 |####################
      4.2 |
      4.4 |
      4.6 |#################################
      4.8 |
      5.0 |########################################
      5.1 |
      5.3 |#######################
      5.5 |
      5.7 |##################
      5.9 |
  (13 below, 17 above range)

fxpmul_precise (n=160, range 2.9-6.3 ns)
      2.9 |
      3.1 |
      3.3 |########################################
      3.4 |
      3.6 |###########################
      3.8 |
      3.9 |
      4.1 |###########################
      4.3 |
      4.5 |#####################################
      4.6 |
      4.8 |
      5.0 |########################################
      5.1 |
      5.3 |###################################
      5.5 |
      5.6 |########################################
      5.8 |
      6.0 |
      6.1 |#############################
  (17 below, 11 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=31.0% (high variance, measurements may be unstable)
- **fxpmul_hot**: bridge=71.7% of algo (FFI overhead may distort results)
- **fxpmul_precise**: CV=26.3% (high variance, measurements may be unstable)
- **fxpmul_precise**: bridge=52.1% of algo (FFI overhead may distort results)
