# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### layout_aos is fastest but the noisiest (CV 79.3%)

layout_aos wins on median (333 ns) yet has the highest variance (CV 79.3%), while layout_soa is the steadiest (CV 77.4%, 333 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (264 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (layout_aos)

The baseline layout_aos is the fastest (333 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader layout_aos vs stability leader layout_soa (+0% speed for 1.0x steadier)

layout_aos is fastest (333 ns, CV 79.3%); layout_soa gives up 0.2% median for 1.0x lower variance (CV 77.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### layout_aos is inconsistent: worst-20% is 3.4x its best-20%

layout_aos's best 20% of batches run at 272 ns but its worst 20% at 928 ns (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### layout_soa's edge over baseline is significant but tiny (0 ns, 0.15%)

layout_soa differs from baseline layout_aos by 0 ns (0.15%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (layout_aos) is the fastest** at 332.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 332.8 ns, slowest 333.3 ns)

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
| Counter frequency | 2112756649 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| layout_aos | 2866ns | 2208ns | 1807ns | 2109ns | 6196ns | base |
| layout_soa | 2790ns | 2208ns | 1807ns | 2044ns | 6012ns | -2.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 431ns | 272ns | 928ns | base | 38.028 |
| layout_soa | 419ns | 273ns | 902ns | -2.64% | 39.060 |

## Performance model

- Peak throughput: **60.178 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 49.231 | 81.8% |
| layout_soa | 49.157 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2866ns | 2866ns | base |
| layout_soa | 2790ns | 2790ns | -2.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 333ns | base | --- | [333, 333] | --- | --- | --- | --- |
| layout_soa | 333ns | +0.5ns (+0.2%) | [+0, +1]ns | [273, 333] | YES | 0.0042 | 0.0042 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 272ns | +0.2% |
| 1 | 2 | 333ns | -18.1% |
| 1 | 3 | 333ns | -18.1% |
| 1 | 4 | 272ns | +22.4% |
| 1 | 5 | 272ns | +0.1% |
| 1 | 6 | 333ns | +0.2% |
| 1 | 7 | 272ns | +22.5% |
| 1 | 8 | 333ns | -18.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | -0.375 | moderate- |
| layout_soa | -0.141 | ok |

**Consistency summary:**

- **layout_soa**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 30.6ns | 430.8ns | 7.1% | HIGH |
| layout_soa | 29.6ns | 419.5ns | 7.1% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 272.3-928.4 ns)
    272.3 |#####################################
    305.1 |########################################
    337.9 |####
    370.7 |
    403.5 |#
    436.3 |
    469.1 |
    501.9 |
    534.7 |
    567.5 |
    600.3 |##
    633.1 |
    666.0 |
    698.8 |
    731.6 |
    764.4 |
    797.2 |#
    830.0 |###
    862.8 |
    895.6 |##
  (11 below, 16 above range)

layout_soa (n=160, range 272.7-901.6 ns)
    272.7 |####################################
    304.2 |########################################
    335.6 |
    367.0 |
    398.5 |
    429.9 |
    461.4 |
    492.8 |
    524.3 |#
    555.7 |
    587.2 |
    618.6 |
    650.1 |
    681.5 |
    713.0 |
    744.4 |
    775.9 |
    807.3 |
    838.8 |####
    870.2 |
  (28 below, 19 above range)

```

## Diagnostics

- **layout_aos**: CV=61.2% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **layout_soa**: CV=61.5% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 3.3x (possible bimodal distribution)
