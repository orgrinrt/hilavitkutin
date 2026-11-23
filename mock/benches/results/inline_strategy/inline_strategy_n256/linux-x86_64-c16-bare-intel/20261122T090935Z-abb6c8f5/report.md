# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### inline_never shows alternating (throttle bounce) (autocorr -0.67)

inline_never's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (208 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### inline_default's comparison is tie-heavy (14% tied pairs)

14% of paired samples for inline_default are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### inline_always is inconsistent: worst-20% is 3.5x its best-20%

inline_always's best 20% of batches run at 242 ns but its worst 20% at 843 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.4% of the fastest

All 3 variants sit between 292 ns and 293 ns - a 0.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### inline_default's edge over baseline is significant but tiny (-0 ns, 0.07%)

inline_default differs from baseline inline_always by -0 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: inline_default** at 292.2 ns median (-0.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 292.2 ns, slowest 293.4 ns)

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
| Counter frequency | 2112764099 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 2942ns | 2174ns | 1803ns | 2197ns | 6318ns | base |
| inline_default | 2769ns | 2171ns | 1775ns | 2118ns | 5715ns | -5.89% |
| inline_never | 2825ns | 2170ns | 1777ns | 2091ns | 6074ns | -4.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 394ns | 242ns | 843ns | base | 0.649 |
| inline_default | 371ns | 239ns | 764ns | -5.83% | 0.689 |
| inline_never | 381ns | 240ns | 813ns | -3.53% | 0.673 |

## Performance model

- Peak throughput: **1.071 Gops/s** (inline_default; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.876 | 81.8% |
| inline_default | 0.876 | 81.8% |
| inline_never | 0.873 | 81.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 2942ns | 2942ns | base |
| inline_default | 2769ns | 2769ns | -5.89% |
| inline_never | 2825ns | 2825ns | -4.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 292ns | base | --- | [292, 292] | --- | --- | --- | --- |
| inline_default | 292ns | -0.2ns (-0.1%) | [-2, -0]ns | [292, 292] | YES | 0.0000 | 0.0000 | **22** (14%, HIGH) |
| inline_never | 293ns | no significant difference | [-1, +1]ns | [251, 294] | no | 0.6343 | 0.6343 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 239ns | -0.0% | +0.8% |
| 1 | 2 | 292ns | -18.2% | +0.7% |
| 1 | 3 | 292ns | -0.0% | -17.4% |
| 1 | 4 | 292ns | -0.0% | +0.7% |
| 1 | 5 | 239ns | -0.0% | +0.6% |
| 1 | 6 | 294ns | -0.6% | -0.1% |
| 1 | 7 | 292ns | -18.1% | -17.7% |
| 1 | 8 | 292ns | +0.0% | -17.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.220 | moderate- |
| inline_default | -0.376 | moderate- |
| inline_never | -0.672 | HIGH- (thermal bounce) |

**Consistency summary:**

- **inline_default**: won 3/8, lost 0/8
- **inline_never**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 32.2ns | 394.5ns | 8.2% | HIGH |
| inline_default | 30.2ns | 371.5ns | 8.1% | HIGH |
| inline_never | 31.6ns | 380.5ns | 8.3% | HIGH |

## Distribution (algo ns)

```
inline_always (n=160, range 242.5-842.9 ns)
    242.5 |
    272.5 |########################################
    302.5 |#
    332.5 |
    362.6 |
    392.6 |
    422.6 |
    452.6 |
    482.6 |
    512.7 |
    542.7 |
    572.7 |
    602.7 |
    632.7 |
    662.8 |
    692.8 |#
    722.8 |
    752.8 |#
    782.9 |#
    812.9 |
  (30 below, 18 above range)

inline_default (n=160, range 239.1-764.3 ns)
    239.1 |######################
    265.3 |
    291.6 |########################################
    317.8 |
    344.1 |
    370.4 |
    396.6 |
    422.9 |
    449.2 |
    475.4 |
    501.7 |
    527.9 |
    554.2 |
    580.5 |
    606.7 |##
    633.0 |#
    659.3 |#
    685.5 |###
    711.8 |
    738.0 |###
  (13 below, 10 above range)

inline_never (n=160, range 240.0-813.1 ns)
    240.0 |########################################
    268.6 |################################
    297.3 |##
    326.0 |#
    354.6 |
    383.3 |
    411.9 |
    440.6 |
    469.2 |
    497.9 |
    526.6 |
    555.2 |
    583.9 |
    612.5 |
    641.2 |
    669.8 |##
    698.5 |##
    727.2 |##
    755.8 |##
    784.5 |#
  (13 below, 15 above range)

```

## Diagnostics

- **inline_always**: CV=58.8% (high variance, measurements may be unstable)
- **inline_always**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **inline_default**: CV=55.9% (high variance, measurements may be unstable)
- **inline_default**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **inline_never**: CV=59.3% (high variance, measurements may be unstable)
- **inline_never**: worst_20/best_20 = 3.4x (possible bimodal distribution)
