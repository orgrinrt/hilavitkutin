# W = 4, G = 8

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### per_core dominates: 30% faster than the next best (grouped)

per_core (2.45 ms) leads grouped (3.17 ms) by 30%, a clear separation rather than a photo finish. CV 36.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### per_core beats baseline by 25% (significant)

per_core is -789.88 us (25%) faster than baseline grouped, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### per_core is fastest but the noisiest (CV 36.5%)

per_core wins on median (2.45 ms) yet has the highest variance (CV 36.5%), while sequential is the steadiest (CV 5.5%, 4.76 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sequential shows alternating (throttle bounce) (autocorr -0.54)

sequential's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### per_core is inconsistent: worst-20% is 1.6x its best-20%

per_core's best 20% of batches run at 2.13 ms but its worst 20% at 3.50 ms (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: per_core** at 2445394.0 ns median (-22.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.95x (fastest 2445394.0 ns, slowest 4764676.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100451752 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 3172544ns | 3171349ns | 2857021ns | 3107560ns | 3427780ns | base |
| per_core | 2718733ns | 2448099ns | 2130983ns | 2391525ns | 3503419ns | -14.30% |
| sequential | 4802677ns | 4767276ns | 4440971ns | 4723414ns | 5102425ns | +51.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 3170098ns | 2854965ns | 3425659ns | base | 0.331 |
| per_core | 2716635ns | 2129237ns | 3501351ns | -14.30% | 0.386 |
| sequential | 4800391ns | 4438961ns | 5100115ns | +51.43% | 0.218 |

## Performance model

- Peak throughput: **0.492 Gops/s** (per_core; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.331 | 67.2% |
| per_core | 0.429 | 87.1% |
| sequential | 0.220 | 44.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 3172544ns | 3172544ns | base |
| per_core | 2718733ns | 2718733ns | -14.30% |
| sequential | 4802677ns | 4802677ns | +51.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 3167929ns | base | --- | [2986366, 3234508] | --- | --- | --- | --- |
| per_core | 2445394ns | -789881.3ns (-24.9%) | [-877708, -520128]ns | [2300848, 2564854] | YES | 0.0013 | 0.0013 | 0 |
| sequential | 4764677ns | +1672117.6ns (+52.8%) | [+1375130, +1911025]ns | [4637527, 4913725] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 2956906ns | -17.3% | +62.6% |
| 1 | 2 | 3369450ns | -26.4% | +34.3% |
| 1 | 3 | 3148213ns | -27.5% | +59.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.524 | HIGH- (thermal bounce) |
| per_core | -0.271 | moderate- |
| sequential | -0.543 | HIGH- (thermal bounce) |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 4300231.7ns | 3170098.1ns | 135.6% | HIGH |
| per_core | 2511378.1ns | 2716635.1ns | 92.4% | HIGH |
| sequential | 4831824.5ns | 4800391.5ns | 100.7% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 2854965.1-3425658.7 ns)
  2854965.1 |
  2883499.8 |#############
  2912034.5 |#############
  2940569.1 |
  2969103.8 |##########################
  2997638.5 |
  3026173.2 |
  3054707.9 |
  3083242.5 |
  3111777.2 |#############
  3140311.9 |##########################
  3168846.6 |#############
  3197381.2 |########################################
  3225915.9 |#############
  3254450.6 |
  3282985.3 |
  3311520.0 |
  3340054.6 |
  3368589.3 |
  3397124.0 |
  (2 below, 4 above range)

per_core (n=18, range 2129237.4-3501350.8 ns)
  2129237.4 |
  2197843.0 |##########################
  2266448.7 |########################################
  2335054.4 |##########################
  2403660.1 |########################################
  2472265.7 |##########################
  2540871.4 |
  2609477.1 |##########################
  2678082.7 |
  2746688.4 |
  2815294.1 |
  2883899.8 |
  2952505.4 |
  3021111.1 |
  3089716.8 |
  3158322.5 |#############
  3226928.1 |
  3295533.8 |
  3364139.5 |
  3432745.1 |
  (1 below, 2 above range)

sequential (n=18, range 4438961.0-5100115.2 ns)
  4438961.0 |########################################
  4472018.7 |
  4505076.5 |
  4538134.2 |
  4571191.9 |####################
  4604249.6 |####################
  4637307.3 |####################
  4670365.0 |####################
  4703422.7 |####################
  4736480.4 |########################################
  4769538.1 |
  4802595.8 |####################
  4835653.5 |####################
  4868711.2 |
  4901769.0 |####################
  4934826.7 |####################
  4967884.4 |####################
  5000942.1 |
  5033999.8 |
  5067057.5 |
  (1 below, 3 above range)

```

## Diagnostics

- **grouped**: bridge=134.5% of algo (FFI overhead may distort results)
- **per_core**: CV=32.8% (high variance, measurements may be unstable)
- **per_core**: bridge=101.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.8% of algo (FFI overhead may distort results)
