# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### layout_soa shows alternating (throttle bounce) (autocorr -0.55)

layout_soa's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (246 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (layout_aos)

The baseline layout_aos is the fastest (333 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### layout_soa is inconsistent: worst-20% is 3.4x its best-20%

layout_soa's best 20% of batches run at 273 ns but its worst 20% at 920 ns (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### layout_soa's edge over baseline is significant but tiny (0 ns, 0.09%)

layout_soa differs from baseline layout_aos by 0 ns (0.09%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (layout_aos) is the fastest** at 333.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 333.0 ns, slowest 333.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | AoS vs SoA (column-store) layout under partial-field iteration |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eadbeef01234567 |
| Counter frequency | 2112817009 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| layout_aos | 2795ns | 2214ns | 1825ns | 2216ns | 5502ns | base |
| layout_soa | 2870ns | 2208ns | 1807ns | 2124ns | 6172ns | +2.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 418ns | 274ns | 819ns | base | 39.158 |
| layout_soa | 430ns | 273ns | 920ns | +2.83% | 38.081 |

## Performance model

- Peak throughput: **60.074 Gops/s** (layout_soa; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 49.201 | 81.9% |
| layout_soa | 49.157 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2795ns | 2795ns | base |
| layout_soa | 2870ns | 2870ns | +2.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 333ns | base | --- | [333, 333] | --- | --- | --- | --- |
| layout_soa | 333ns | +0.3ns (+0.1%) | [+0, +0]ns | [333, 333] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 272ns | +0.1% |
| 1 | 2 | 333ns | +3.3% |
| 1 | 3 | 272ns | +0.1% |
| 1 | 4 | 333ns | +0.1% |
| 1 | 5 | 333ns | +0.1% |
| 1 | 6 | 333ns | +0.1% |
| 1 | 7 | 333ns | -18.1% |
| 1 | 8 | 333ns | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | -0.208 | moderate- |
| layout_soa | -0.550 | HIGH- (thermal bounce) |

**Consistency summary:**

- **layout_soa**: won 1/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 29.6ns | 418.4ns | 7.1% | HIGH |
| layout_soa | 30.9ns | 430.2ns | 7.2% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 274.3-818.7 ns)
    274.3 |
    301.5 |
    328.7 |########################################
    356.0 |
    383.2 |
    410.4 |
    437.6 |
    464.8 |
    492.1 |
    519.3 |
    546.5 |
    573.7 |
    600.9 |
    628.2 |
    655.4 |
    682.6 |
    709.8 |
    737.0 |
    764.3 |
    791.5 |
  (31 below, 18 above range)

layout_soa (n=160, range 272.7-919.7 ns)
    272.7 |########################
    305.1 |########################################
    337.4 |##
    369.8 |
    402.1 |
    434.5 |
    466.8 |
    499.2 |
    531.5 |
    563.9 |
    596.2 |
    628.6 |
    660.9 |
    693.3 |#
    725.6 |
    758.0 |
    790.3 |#
    822.7 |
    855.0 |
    887.4 |#
  (21 below, 19 above range)

```

## Diagnostics

- **layout_aos**: CV=58.8% (high variance, measurements may be unstable)
- **layout_soa**: CV=60.0% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 3.4x (possible bimodal distribution)
