# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### inline_default dominates: 22% faster than the next best (inline_always)

inline_default (240 ns) leads inline_always (292 ns) by 22%, a clear separation rather than a photo finish. CV 97.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inline_default is fastest but the noisiest (CV 97.5%)

inline_default wins on median (240 ns) yet has the highest variance (CV 97.5%), while inline_always is the steadiest (CV 75.0%, 292 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (54 ns) is smaller than the fastest variant's own run-to-run std-dev (233 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### inline_default is inconsistent: worst-20% is 3.4x its best-20%

inline_default's best 20% of batches run at 239 ns but its worst 20% at 819 ns (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### inline_default's edge over baseline is significant but tiny (-0 ns, 0.09%)

inline_default differs from baseline inline_always by -0 ns (0.09%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: inline_default** at 239.5 ns median (-18.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.22x (fastest 239.5 ns, slowest 293.3 ns)

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
| Counter frequency | 2112795803 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 2821ns | 2178ns | 1781ns | 2201ns | 5723ns | base |
| inline_default | 2758ns | 1792ns | 1779ns | 1963ns | 6122ns | -2.25% |
| inline_never | 2828ns | 2179ns | 1782ns | 2120ns | 5998ns | +0.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 374ns | 239ns | 748ns | base | 0.684 |
| inline_default | 370ns | 239ns | 819ns | -1.23% | 0.692 |
| inline_never | 379ns | 240ns | 799ns | +1.20% | 0.676 |

## Performance model

- Peak throughput: **1.071 Gops/s** (inline_default; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.876 | 81.8% |
| inline_default | 1.069 | 99.8% |
| inline_never | 0.873 | 81.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 2821ns | 2821ns | base |
| inline_default | 2758ns | 2758ns | -2.25% |
| inline_never | 2828ns | 2828ns | +0.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 292ns | base | --- | [292, 292] | --- | --- | --- | --- |
| inline_default | 240ns | -0.2ns (-0.1%) | [-53, -0]ns | [239, 279] | YES | 0.0003 | 0.0003 | 14 |
| inline_never | 293ns | +1.0ns (+0.3%) | [+1, +1]ns | [293, 293] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 239ns | +0.2% | +22.7% |
| 1 | 2 | 292ns | -18.2% | +0.4% |
| 1 | 3 | 239ns | +0.0% | +0.4% |
| 1 | 4 | 292ns | +0.0% | -17.7% |
| 1 | 5 | 335ns | -28.6% | -28.4% |
| 1 | 6 | 292ns | -18.2% | +0.4% |
| 1 | 7 | 292ns | -0.1% | +0.4% |
| 1 | 8 | 292ns | -18.2% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.021 | ok |
| inline_default | -0.378 | moderate- |
| inline_never | 0.389 | moderate+ |

**Consistency summary:**

- **inline_default**: won 4/8, lost 1/8
- **inline_never**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 30.5ns | 374.4ns | 8.1% | HIGH |
| inline_default | 30.1ns | 369.8ns | 8.1% | HIGH |
| inline_never | 31.3ns | 378.9ns | 8.3% | HIGH |

## Distribution (algo ns)

```
inline_always (n=160, range 239.1-748.1 ns)
    239.1 |##########
    264.5 |
    290.0 |########################################
    315.4 |#
    340.9 |#
    366.3 |
    391.8 |
    417.2 |#
    442.7 |
    468.1 |
    493.6 |
    519.0 |
    544.5 |
    569.9 |
    595.4 |
    620.8 |
    646.3 |
    671.7 |
    697.2 |
    722.6 |
  (10 below, 19 above range)

inline_default (n=160, range 239.0-818.7 ns)
    239.0 |########################################
    268.0 |#################
    297.0 |
    326.0 |
    355.0 |
    384.0 |
    412.9 |
    441.9 |
    470.9 |
    499.9 |
    528.9 |
    557.9 |
    586.8 |
    615.8 |#
    644.8 |#
    673.8 |#
    702.8 |#
    731.8 |##
    760.8 |
    789.7 |#
  (17 below, 15 above range)

inline_never (n=160, range 239.9-798.6 ns)
    239.9 |#############
    267.9 |########################################
    295.8 |#
    323.7 |
    351.7 |
    379.6 |
    407.5 |
    435.5 |
    463.4 |#
    491.3 |
    519.3 |
    547.2 |
    575.1 |
    603.0 |
    631.0 |
    658.9 |
    686.8 |
    714.8 |
    742.7 |
    770.6 |
  (19 below, 18 above range)

```

## Diagnostics

- **inline_always**: CV=58.6% (high variance, measurements may be unstable)
- **inline_always**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **inline_default**: CV=63.1% (high variance, measurements may be unstable)
- **inline_default**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **inline_never**: CV=63.4% (high variance, measurements may be unstable)
- **inline_never**: worst_20/best_20 = 3.3x (possible bimodal distribution)
