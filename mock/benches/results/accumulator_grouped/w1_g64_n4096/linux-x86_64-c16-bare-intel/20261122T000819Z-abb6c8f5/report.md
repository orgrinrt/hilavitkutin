# W = 1, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (grouped, per_core) are a dead heat (<1%)

grouped (22.75 us) and per_core (22.89 us) differ by 0.62%, inside the noise, even though the wider field spreads 12.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### grouped shows alternating (throttle bounce) (autocorr -0.62)

grouped's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.84 us) is smaller than the fastest variant's own run-to-run std-dev (4.50 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (grouped)

The baseline grouped is the fastest (22.75 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader grouped vs stability leader per_core (+1% speed for 2.3x steadier)

grouped is fastest (22.75 us, CV 19.8%); per_core gives up 0.6% median for 2.3x lower variance (CV 8.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### sequential is inconsistent: worst-20% is 1.6x its best-20%

sequential's best 20% of batches run at 19.84 us but its worst 20% at 32.64 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (grouped) is the fastest** at 22747.2 ns median
- Spread: 1.13x (fastest 22747.2 ns, slowest 25591.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 1, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112760367 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 25192ns | 23797ns | 20240ns | 23108ns | 30794ns | base |
| per_core | 23875ns | 24109ns | 20818ns | 23100ns | 26566ns | -5.23% |
| sequential | 27356ns | 26567ns | 20339ns | 24992ns | 34411ns | +8.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 24237ns | 19612ns | 29776ns | base | 0.169 |
| per_core | 22896ns | 20226ns | 25147ns | -5.53% | 0.179 |
| sequential | 26236ns | 19837ns | 32635ns | +8.25% | 0.156 |

## Performance model

- Peak throughput: **0.209 Gops/s** (grouped; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.180 | 86.2% |
| per_core | 0.179 | 85.7% |
| sequential | 0.160 | 76.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 25192ns | 25192ns | base |
| per_core | 23875ns | 23875ns | -5.23% |
| sequential | 27356ns | 27356ns | +8.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 22747ns | base | --- | [21079, 26297] | --- | --- | --- | --- |
| per_core | 22888ns | no significant difference | [-2029, +1051]ns | [21208, 24628] | no | 0.8145 | 0.4807 | 0 |
| sequential | 25591ns | no significant difference | [-700, +3171]ns | [21584, 27609] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 22747ns | +0.6% | +13.1% |
| 1 | 2 | 29571ns | -16.4% | -6.2% |
| 1 | 3 | 20058ns | +4.7% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.617 | HIGH- (thermal bounce) |
| per_core | -0.497 | moderate- |
| sequential | -0.330 | moderate- |

**Consistency summary:**

- **per_core**: won 1/3, lost 2/3
- **sequential**: won 1/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 24785.7ns | 24237.4ns | 102.3% | HIGH |
| per_core | 23171.8ns | 22896.1ns | 101.2% | HIGH |
| sequential | 30115.9ns | 26236.3ns | 114.8% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 19611.6-29775.6 ns)
  19611.6 |########################################
  20119.8 |
  20628.0 |#############
  21136.2 |#############
  21644.4 |##########################
  22152.6 |
  22660.8 |##########################
  23169.0 |#############
  23677.2 |#############
  24185.4 |
  24693.6 |
  25201.8 |#############
  25710.0 |
  26218.2 |
  26726.4 |#############
  27234.6 |
  27742.8 |#############
  28251.0 |
  28759.2 |
  29267.4 |
  (1 below, 3 above range)

per_core (n=18, range 20225.6-25147.4 ns)
  20225.6 |
  20471.7 |##########################
  20717.7 |
  20963.8 |##########################
  21209.9 |##########################
  21456.0 |
  21702.1 |
  21948.2 |#############
  22194.3 |#############
  22440.4 |
  22686.5 |
  22932.5 |
  23178.6 |
  23424.7 |#############
  23670.8 |#############
  23916.9 |
  24163.0 |#############
  24409.1 |#############
  24655.2 |########################################
  24901.3 |#############
  (1 below, 1 above range)

sequential (n=18, range 19836.7-32635.2 ns)
  19836.7 |##########################
  20476.6 |
  21116.5 |#############
  21756.4 |#############
  22396.4 |
  23036.3 |
  23676.2 |#############
  24316.2 |#############
  24956.1 |#############
  25596.0 |##########################
  26235.9 |
  26875.9 |#############
  27515.8 |########################################
  28155.7 |#############
  28795.7 |
  29435.6 |
  30075.5 |
  30715.4 |#############
  31355.4 |
  31995.3 |
  (2 below, 1 above range)

```

## Diagnostics

- **grouped**: bridge=103.7% of algo (FFI overhead may distort results)
- **per_core**: bridge=101.5% of algo (FFI overhead may distort results)
- **sequential**: CV=28.2% (high variance, measurements may be unstable)
- **sequential**: bridge=103.4% of algo (FFI overhead may distort results)
