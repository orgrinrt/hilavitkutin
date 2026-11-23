# Intra-resource member gather at M=64: contiguous blob vs scattered columns

2 variants, 120 samples per variant.
Baseline: **wide_blob**

## Highlights

Baseline for all deltas below: **wide_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide_blob is fastest but the noisiest (CV 29.4%)

wide_blob wins on median (183 ns) yet has the highest variance (CV 29.4%), while wide_decomposed is the steadiest (CV 21.7%, 192 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (9 ns) is smaller than the fastest variant's own run-to-run std-dev (54 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (wide_blob)

The baseline wide_blob is the fastest (183 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader wide_blob vs stability leader wide_decomposed (+5% speed for 1.4x steadier)

wide_blob is fastest (183 ns, CV 29.4%); wide_decomposed gives up 5.2% median for 1.4x lower variance (CV 21.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (wide_blob) is the fastest** at 182.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.05x (fastest 182.6 ns, slowest 192.1 ns)

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
| Counter frequency | 2112835454 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide_blob | 241ns | 222ns | 221ns | 223ns | 317ns | base |
| wide_decomposed | 242ns | 231ns | 231ns | 231ns | 284ns | +0.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide_blob | 196ns | 182ns | 251ns | base | 21364.357 |
| wide_decomposed | 199ns | 192ns | 226ns | +1.26% | 21099.347 |

## Performance model

- Peak throughput: **22995.613 Gops/s** (wide_blob; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide_blob | 22963.613 | 99.9% |
| wide_decomposed | 21833.961 | 94.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide_blob | 241ns | 241ns | base |
| wide_decomposed | 242ns | 242ns | +0.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide_blob | 183ns | base | --- | [183, 183] | --- | --- | --- | --- |
| wide_decomposed | 192ns | +9.4ns (+5.1%) | [+9, +10]ns | [192, 192] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide_blob | wide_decomposed |
|---|---|---|---|
| 1 | 1 | 316ns | -39.2% |
| 1 | 2 | 183ns | +5.3% |
| 1 | 3 | 182ns | +5.2% |
| 1 | 4 | 183ns | +5.1% |
| 1 | 5 | 183ns | +5.2% |
| 2 | 1 | 183ns | +5.1% |
| 2 | 2 | 183ns | +4.9% |
| 2 | 3 | 187ns | +2.6% |
| 2 | 4 | 183ns | +5.0% |
| 2 | 5 | 183ns | +5.2% |
| 3 | 1 | 183ns | +5.3% |
| 3 | 2 | 183ns | +5.1% |
| 3 | 3 | 183ns | +5.2% |
| 3 | 4 | 183ns | +5.3% |
| 3 | 5 | 183ns | +5.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide_blob | -0.008 | ok |
| wide_decomposed | -0.143 | ok |

**Consistency summary:**

- **wide_decomposed**: won 1/15, lost 14/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide_blob | 92.4ns | 196.3ns | 47.1% | HIGH |
| wide_decomposed | 312.2ns | 198.8ns | 157.0% | HIGH |

## Distribution (algo ns)

```
wide_blob (n=120, range 182.4-251.1 ns)
    182.4 |########################################
    185.8 |##
    189.3 |
    192.7 |
    196.1 |
    199.6 |
    203.0 |
    206.4 |
    209.9 |
    213.3 |
    216.7 |
    220.2 |
    223.6 |
    227.0 |
    230.5 |
    233.9 |
    237.3 |
    240.8 |
    244.2 |
    247.6 |
  (4 below, 6 above range)

wide_decomposed (n=120, range 191.9-225.8 ns)
    191.9 |########################################
    193.6 |
    195.2 |
    196.9 |
    198.6 |
    200.3 |
    202.0 |
    203.7 |
    205.4 |
    207.1 |
    208.8 |
    210.5 |
    212.2 |
    213.9 |
    215.6 |
    217.3 |
    219.0 |
    220.7 |
    222.4 |
    224.1 |
  (5 below, 3 above range)

```

## Diagnostics

- **wide_blob**: CV=27.3% (high variance, measurements may be unstable)
- **wide_blob**: bridge=47.2% of algo (FFI overhead may distort results)
- **wide_decomposed**: CV=21.0% (high variance, measurements may be unstable)
- **wide_decomposed**: bridge=156.8% of algo (FFI overhead may distort results)
