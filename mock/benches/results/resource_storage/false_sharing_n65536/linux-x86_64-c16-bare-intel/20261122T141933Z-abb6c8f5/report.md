# Cross-core false sharing: slots packed in one line vs one line each

2 variants, 20 samples per variant.
Baseline: **fs_padded**

## Highlights

Baseline for all deltas below: **fs_padded**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fs_padded dominates: 33% faster than the next best (fs_packed)

fs_padded (18.73 ms) leads fs_packed (24.99 ms) by 33%, a clear separation rather than a photo finish. CV 48.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fs_padded is fastest but the noisiest (CV 48.1%)

fs_padded wins on median (18.73 ms) yet has the highest variance (CV 48.1%), while fs_packed is the steadiest (CV 26.7%, 24.99 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fs_packed shows alternating (throttle bounce) (autocorr -0.50)

fs_packed's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6.26 ms) is smaller than the fastest variant's own run-to-run std-dev (9.02 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fs_padded)

The baseline fs_padded is the fastest (18.73 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fs_padded is inconsistent: worst-20% is 2.0x its best-20%

fs_padded's best 20% of batches run at 17.11 ms but its worst 20% at 34.08 ms (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fs_padded) is the fastest** at 18728423.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.33x (fastest 18728423.1 ns, slowest 24991561.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Cross-core false sharing: slots packed in one line vs one line each |
| Passes | 2 |
| Runs per pass | 10 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef1 |
| Counter frequency | 2112784724 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fs_packed | 27216508ns | 24992258ns | 23807525ns | 25336574ns | 36265291ns | +27.62% |
| fs_padded | 21325398ns | 18729759ns | 17111012ns | 18478974ns | 34079056ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fs_packed | 27214857ns | 23806570ns | 36263418ns | +27.62% | 0.002 |
| fs_padded | 21324100ns | 17110532ns | 34076924ns | base | 0.003 |

## Performance model

- Peak throughput: **0.004 Gops/s** (fs_padded; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fs_packed | 0.003 | 68.5% |
| fs_padded | 0.003 | 91.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fs_packed | 27216508ns | 27216508ns | +27.62% |
| fs_padded | 21325398ns | 21325398ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fs_padded | 18728423ns | base | --- | [17386017, 19267732] | --- | --- | --- | --- |
| fs_packed | 24991561ns | +6226944.5ns (+33.2%) | [+5910930, +8174642]ns | [24337745, 26837145] | YES | 0.0004 | 0.0004 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fs_padded | fs_packed |
|---|---|---|---|
| 1 | 1 | 19234138ns | +29.9% |
| 1 | 2 | 18134077ns | +40.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fs_packed | -0.500 | moderate- |
| fs_padded | -0.500 | moderate- |

**Consistency summary:**

- **fs_packed**: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fs_packed | 307.9ns | 27214857.4ns | 0.0% |  |
| fs_padded | 178.2ns | 21324100.2ns | 0.0% |  |

## Distribution (algo ns)

```
fs_packed (n=20, range 23806569.9-36263418.0 ns)
  23806569.9 |########################################
  24429412.4 |########################################
  25052254.8 |##############################
  25675097.2 |
  26297939.6 |##########
  26920782.0 |####################
  27543624.4 |##########
  28166466.8 |
  28789309.2 |
  29412151.6 |
  30034994.0 |
  30657836.4 |##########
  31280678.8 |##########
  31903521.2 |
  32526363.6 |
  33149206.0 |
  33772048.4 |
  34394890.8 |
  35017733.2 |
  35640575.6 |
  (2 below, 1 above range)

fs_padded (n=20, range 17110531.7-34076924.1 ns)
  17110531.7 |############################
  17958851.3 |###########
  18807170.9 |########################################
  19655490.5 |
  20503810.1 |#####
  21352129.8 |
  22200449.4 |
  23048769.0 |
  23897088.6 |
  24745408.2 |
  25593727.9 |
  26442047.5 |
  27290367.1 |
  28138686.7 |
  28987006.3 |
  29835326.0 |
  30683645.6 |
  31531965.2 |
  32380284.8 |
  33228604.5 |
  (3 below, 2 above range)

```

## Diagnostics

- **fs_packed**: CV=24.5% (high variance, measurements may be unstable)
- **fs_padded**: CV=42.3% (high variance, measurements may be unstable)
