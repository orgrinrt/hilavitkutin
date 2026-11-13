# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (engine) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline engine has the worst median (28.80 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest std at 22.19 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### std dominates: 30% faster than the next best (engine)

std (22.19 us) leads engine (28.80 us) by 30%, a clear separation rather than a photo finish. CV 37.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std shows alternating (throttle bounce) (autocorr -0.50)

std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6.60 us) is smaller than the fastest variant's own run-to-run std-dev (8.31 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### engine is inconsistent: worst-20% is 2.4x its best-20%

engine's best 20% of batches run at 25.30 us but its worst 20% at 60.65 us (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std** at 22192.7 ns median (-22.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.30x (fastest 22192.7 ns, slowest 28797.3 ns)

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
| engine | 44355ns | 29708ns | 26215ns | 28019ns | 61142ns | base |
| std | 27774ns | 22577ns | 22062ns | 22255ns | 33341ns | -37.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 43585ns | 25305ns | 60649ns | base | 1.504 |
| std | 26751ns | 21852ns | 31574ns | -38.62% | 2.450 |

## Performance model

- Peak throughput: **2.999 Gops/s** (std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 2.276 | 75.9% |
| std | 2.953 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 44355ns | 44355ns | base |
| std | 27774ns | 27774ns | -37.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 28797ns | base | --- | [25912, 30820] | --- | --- | --- | --- |
| std | 22193ns | -5576.1ns (-19.4%) | [-8429, -3152]ns | [21852, 37588] | YES (adj: no) | 0.0703 | 0.0703 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | std |
|---|---|---|---|
| 1 | 1 | 26535ns | -17.6% |
| 1 | 2 | 30741ns | -2.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| std | -0.500 | moderate- |

**Consistency summary:**

- **std**: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 298909.7ns | 43584.9ns | 685.8% | HIGH |
| std | 294198.4ns | 26751.4ns | 1099.7% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 25304.7-60649.4 ns)
  25304.7 |########################################
  27071.9 |########################################
  28839.2 |####################
  30606.4 |########################################
  32373.7 |
  34140.9 |
  35908.1 |
  37675.4 |
  39442.6 |
  41209.8 |
  42977.1 |
  44744.3 |
  46511.6 |
  48278.8 |
  50046.0 |
  51813.3 |
  53580.5 |
  55347.7 |
  57115.0 |
  58882.2 |
  (0 below, 1 above range)

std (n=8, range 21851.5-31573.8 ns)
  21851.5 |########################################
  22337.6 |########
  22823.7 |
  23309.8 |
  23796.0 |
  24282.1 |
  24768.2 |
  25254.3 |
  25740.4 |
  26226.5 |
  26712.7 |
  27198.8 |
  27684.9 |
  28171.0 |
  28657.1 |
  29143.2 |
  29629.3 |
  30115.5 |
  30601.6 |
  31087.7 |
  (0 below, 2 above range)

```

## Diagnostics

- **engine**: CV=93.5% (high variance, measurements may be unstable)
- **engine**: bridge=1036.9% of algo (FFI overhead may distort results)
- **std**: CV=31.1% (high variance, measurements may be unstable)
- **std**: bridge=1278.9% of algo (FFI overhead may distort results)
