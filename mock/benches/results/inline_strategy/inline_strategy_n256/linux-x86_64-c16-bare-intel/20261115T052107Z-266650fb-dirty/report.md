# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (211 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (inline_always)

The baseline inline_always is the fastest (292 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### inline_default's comparison is tie-heavy (16% tied pairs)

16% of paired samples for inline_default are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### inline_never is inconsistent: worst-20% is 3.5x its best-20%

inline_never's best 20% of batches run at 240 ns but its worst 20% at 848 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.3% of the fastest

All 3 variants sit between 292 ns and 293 ns - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### inline_never's edge over baseline is significant but tiny (2 ns, 0.65%)

inline_never differs from baseline inline_always by 2 ns (0.65%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (inline_always) is the fastest** at 292.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 292.2 ns, slowest 293.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x39f847b2c651de80 |
| Counter frequency | 2112795995 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 2574ns | 2167ns | 1773ns | 2029ns | 5012ns | base |
| inline_default | 2762ns | 2171ns | 1773ns | 2102ns | 5730ns | +7.28% |
| inline_never | 2992ns | 2165ns | 1773ns | 2231ns | 6497ns | +16.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 346ns | 239ns | 670ns | base | 0.741 |
| inline_default | 370ns | 239ns | 762ns | +7.00% | 0.692 |
| inline_never | 399ns | 240ns | 848ns | +15.39% | 0.642 |

## Performance model

- Peak throughput: **1.071 Gops/s** (inline_always; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.876 | 81.8% |
| inline_default | 0.876 | 81.8% |
| inline_never | 0.873 | 81.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 2574ns | 2574ns | base |
| inline_default | 2762ns | 2762ns | +7.28% |
| inline_never | 2992ns | 2992ns | +16.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 292ns | base | --- | [292, 292] | --- | --- | --- | --- |
| inline_default | 292ns | no significant difference | [+0, +0]ns | [292, 292] | no (adj: YES) | 0.0304 | 0.0304 | **26** (16%, HIGH) |
| inline_never | 293ns | +1.9ns (+0.7%) | [+1, +3]ns | [242, 294] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 239ns | +22.2% | +0.6% |
| 1 | 2 | 239ns | +0.0% | +0.9% |
| 1 | 3 | 239ns | +0.0% | +23.3% |
| 1 | 4 | 292ns | +0.0% | -17.8% |
| 1 | 5 | 292ns | +0.0% | +107.0% |
| 1 | 6 | 239ns | +22.2% | +0.8% |
| 1 | 7 | 292ns | +0.0% | +0.7% |
| 1 | 8 | 292ns | -18.2% | -17.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | 0.126 | ok |
| inline_default | 0.060 | ok |
| inline_never | -0.284 | moderate- |

**Consistency summary:**

- **inline_default**: won 1/8, lost 2/8
- **inline_never**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 28.2ns | 345.6ns | 8.2% | HIGH |
| inline_default | 30.2ns | 369.8ns | 8.2% | HIGH |
| inline_never | 32.5ns | 398.8ns | 8.2% | HIGH |

## Distribution (algo ns)

```
inline_always (n=160, range 239.1-670.0 ns)
    239.1 |#################################
    260.6 |
    282.2 |########################################
    303.7 |##
    325.2 |
    346.8 |
    368.3 |
    389.9 |
    411.4 |
    433.0 |
    454.5 |
    476.1 |
    497.6 |
    519.2 |
    540.7 |
    562.3 |
    583.8 |
    605.4 |
    626.9 |
    648.4 |
  (12 below, 17 above range)

inline_default (n=160, range 239.1-762.2 ns)
    239.1 |#################
    265.2 |
    291.4 |########################################
    317.5 |
    343.7 |
    369.9 |
    396.0 |
    422.2 |
    448.3 |
    474.5 |
    500.6 |
    526.8 |
    553.0 |
    579.1 |
    605.3 |
    631.4 |
    657.6 |
    683.8 |
    709.9 |
    736.1 |
  (13 below, 21 above range)

inline_never (n=160, range 239.9-847.6 ns)
    239.9 |########################################
    270.3 |######################
    300.7 |#
    331.1 |#
    361.4 |
    391.8 |
    422.2 |##
    452.6 |
    483.0 |
    513.4 |
    543.7 |
    574.1 |##
    604.5 |
    634.9 |##
    665.3 |
    695.6 |####
    726.0 |
    756.4 |##
    786.8 |
    817.2 |#
  (23 below, 17 above range)

```

## Diagnostics

- **inline_always**: CV=61.0% (high variance, measurements may be unstable)
- **inline_default**: CV=63.4% (high variance, measurements may be unstable)
- **inline_default**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **inline_never**: CV=61.1% (high variance, measurements may be unstable)
- **inline_never**: worst_20/best_20 = 3.5x (possible bimodal distribution)
