# Intra-resource member gather at M=64: contiguous blob vs scattered columns

2 variants, 120 samples per variant.
Baseline: **wide_blob**

## Highlights

Baseline for all deltas below: **wide_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide_blob is fastest but the noisiest (CV 50.1%)

wide_blob wins on median (10.75 us) yet has the highest variance (CV 50.1%), while wide_decomposed is the steadiest (CV 9.8%, 11.63 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (881 ns) is smaller than the fastest variant's own run-to-run std-dev (5.38 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (wide_blob)

The baseline wide_blob is the fastest (10.75 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader wide_blob vs stability leader wide_decomposed (+8% speed for 5.1x steadier)

wide_blob is fastest (10.75 us, CV 50.1%); wide_decomposed gives up 8.2% median for 5.1x lower variance (CV 9.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (wide_blob) is the fastest** at 10749.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.08x (fastest 10749.5 ns, slowest 11630.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Intra-resource member gather at M=64: contiguous blob vs scattered columns |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112727760 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide_blob | 11943ns | 10793ns | 10786ns | 11124ns | 15557ns | base |
| wide_decomposed | 12375ns | 11676ns | 11673ns | 11975ns | 14278ns | +3.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide_blob | 11894ns | 10745ns | 15490ns | base | 1.378 |
| wide_decomposed | 12325ns | 11628ns | 14220ns | +3.62% | 1.329 |

## Performance model

- Peak throughput: **1.525 Gops/s** (wide_blob; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide_blob | 1.524 | 100.0% |
| wide_decomposed | 1.409 | 92.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide_blob | 11943ns | 11943ns | base |
| wide_decomposed | 12375ns | 12375ns | +3.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide_blob | 10749ns | base | --- | [10749, 10750] | --- | --- | --- | --- |
| wide_decomposed | 11631ns | +882.4ns (+8.2%) | [+881, +884]ns | [11630, 11631] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide_blob | wide_decomposed |
|---|---|---|---|
| 1 | 1 | 13137ns | -11.5% |
| 1 | 2 | 10747ns | +8.2% |
| 1 | 3 | 10750ns | +8.2% |
| 1 | 4 | 10749ns | +8.2% |
| 1 | 5 | 10748ns | +8.2% |
| 2 | 1 | 13139ns | +8.2% |
| 2 | 2 | 13139ns | -11.5% |
| 2 | 3 | 10749ns | +8.2% |
| 2 | 4 | 10748ns | +32.3% |
| 2 | 5 | 10748ns | +8.2% |
| 3 | 1 | 10749ns | +32.3% |
| 3 | 2 | 10785ns | +31.8% |
| 3 | 3 | 13138ns | -11.5% |
| 3 | 4 | 10748ns | +8.2% |
| 3 | 5 | 10750ns | +8.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide_blob | 0.047 | ok |
| wide_decomposed | -0.047 | ok |

**Consistency summary:**

- **wide_decomposed**: won 3/15, lost 12/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide_blob | 97.5ns | 11893.8ns | 0.8% |  |
| wide_decomposed | 320.4ns | 12324.7ns | 2.6% |  |

## Distribution (algo ns)

```
wide_blob (n=120, range 10744.8-15490.5 ns)
  10744.8 |########################################
  10982.1 |
  11219.3 |
  11456.6 |
  11693.9 |
  11931.2 |
  12168.5 |
  12405.8 |
  12643.0 |
  12880.3 |
  13117.6 |################
  13354.9 |
  13592.2 |
  13829.5 |
  14066.8 |
  14304.0 |
  14541.3 |
  14778.6 |
  15015.9 |
  15253.2 |
  (8 below, 1 above range)

wide_decomposed (n=120, range 11628.3-14219.6 ns)
  11628.3 |########################################
  11757.8 |
  11887.4 |
  12017.0 |
  12146.5 |
  12276.1 |
  12405.7 |
  12535.2 |
  12664.8 |
  12794.4 |
  12923.9 |
  13053.5 |
  13183.1 |
  13312.6 |
  13442.2 |
  13571.7 |
  13701.3 |
  13830.9 |
  13960.4 |
  14090.0 |#############
  (8 below, 5 above range)

```

## Diagnostics

- **wide_blob**: CV=45.3% (high variance, measurements may be unstable)
