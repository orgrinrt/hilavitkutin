# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.0% (fastest 474.68 us to slowest 479.33 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### std shows alternating (throttle bounce) (autocorr -0.50)

std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4.65 us) is smaller than the fastest variant's own run-to-run std-dev (40.62 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (engine)

The baseline engine is the fastest (474.68 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (engine) is the fastest** at 474679.3 ns median
- Spread: 1.01x (fastest 474679.3 ns, slowest 479330.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | element_wise, four-stage RAW chain |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000001 |
| Counter frequency | 2300408230 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 489640ns | 476114ns | 435218ns | 461599ns | 524276ns | base |
| std | 482478ns | 481529ns | 433972ns | 450017ns | 518951ns | -1.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 487786ns | 433165ns | 522406ns | base | 2.150 |
| std | 480388ns | 432522ns | 516315ns | -1.52% | 2.183 |

## Performance model

- Peak throughput: **2.424 Gops/s** (std; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 2.209 | 91.1% |
| std | 2.188 | 90.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 489640ns | 489640ns | base |
| std | 482478ns | 482478ns | -1.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 474679ns | base | --- | [449936, 524230] | --- | --- | --- | --- |
| std | 479330ns | no significant difference | [-55025, +33392]ns | [439265, 504028] | no | 0.7266 | 0.7266 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | std |
|---|---|---|---|
| 1 | 1 | 495116ns | +1.4% |
| 1 | 2 | 472837ns | -6.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| std | -0.500 | moderate- |

**Consistency summary:**

- **std**: won 1/2, lost 1/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 5283392.5ns | 487786.3ns | 1083.1% | HIGH |
| std | 4875075.3ns | 480388.1ns | 1014.8% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 433165.2-522405.6 ns)
  433165.2 |########################################
  437627.2 |
  442089.2 |
  446551.3 |########################################
  451013.3 |
  455475.3 |
  459937.3 |########################################
  464399.3 |########################################
  468861.4 |
  473323.4 |
  477785.4 |
  482247.4 |########################################
  486709.4 |
  491171.4 |
  495633.5 |
  500095.5 |
  504557.5 |
  509019.5 |
  513481.5 |
  517943.6 |
  (0 below, 3 above range)

std (n=8, range 432521.5-516314.7 ns)
  432521.5 |########################################
  436711.2 |########################################
  440900.8 |########################################
  445090.5 |
  449280.1 |
  453469.8 |
  457659.5 |
  461849.1 |########################################
  466038.8 |
  470228.4 |
  474418.1 |
  478607.7 |
  482797.4 |
  486987.1 |
  491176.7 |
  495366.4 |########################################
  499556.0 |########################################
  503745.7 |########################################
  507935.4 |
  512125.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **engine**: bridge=1023.7% of algo (FFI overhead may distort results)
- **std**: bridge=1003.8% of algo (FFI overhead may distort results)
