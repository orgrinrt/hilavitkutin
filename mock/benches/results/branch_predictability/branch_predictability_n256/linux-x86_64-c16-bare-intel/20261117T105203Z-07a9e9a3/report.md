# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (233 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bp_skew_low's comparison is tie-heavy (11% tied pairs)

11% of paired samples for bp_skew_low are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader bp_skew_low vs stability leader bp_skew_high (+0% speed for 1.0x steadier)

bp_skew_low is fastest (292 ns, CV 79.8%); bp_skew_high gives up 0.1% median for 1.0x lower variance (CV 77.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bp_balanced is inconsistent: worst-20% is 3.5x its best-20%

bp_balanced's best 20% of batches run at 239 ns but its worst 20% at 844 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.1% of the fastest

All 3 variants sit between 292 ns and 293 ns - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bp_skew_low's edge over baseline is significant but tiny (-0 ns, 0.07%)

bp_skew_low differs from baseline bp_balanced by -0 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bp_skew_low** at 292.3 ns median (-0.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 292.3 ns, slowest 292.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x1b41e7c35fa28d06 |
| Counter frequency | 2112755873 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bp_balanced | 2890ns | 2178ns | 1779ns | 2115ns | 6323ns | base |
| bp_skew_high | 2798ns | 2178ns | 1780ns | 2119ns | 5852ns | -3.17% |
| bp_skew_low | 2895ns | 2177ns | 1779ns | 2177ns | 6166ns | +0.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 387ns | 239ns | 844ns | base | 0.661 |
| bp_skew_high | 374ns | 239ns | 778ns | -3.31% | 0.684 |
| bp_skew_low | 389ns | 239ns | 820ns | +0.54% | 0.658 |

## Performance model

- Peak throughput: **1.070 Gops/s** (bp_skew_low; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.875 | 81.8% |
| bp_skew_high | 0.875 | 81.8% |
| bp_skew_low | 0.876 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 2890ns | 2890ns | base |
| bp_skew_high | 2798ns | 2798ns | -3.17% |
| bp_skew_low | 2895ns | 2895ns | +0.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 293ns | base | --- | [293, 293] | --- | --- | --- | --- |
| bp_skew_high | 293ns | no significant difference | [-0, +0]ns | [292, 293] | no | 0.2576 | 0.2576 | 7 |
| bp_skew_low | 292ns | -0.2ns (-0.1%) | [-0, -0]ns | [292, 292] | YES | 0.0000 | 0.0000 | **17** (11%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 293ns | -0.1% | -0.1% |
| 1 | 2 | 293ns | -18.2% | -18.2% |
| 1 | 3 | 293ns | -18.2% | -0.1% |
| 1 | 4 | 240ns | +22.2% | -0.0% |
| 1 | 5 | 293ns | -18.2% | -18.2% |
| 1 | 6 | 239ns | +22.2% | +143.6% |
| 1 | 7 | 239ns | +22.2% | -0.0% |
| 1 | 8 | 293ns | +0.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.140 | ok |
| bp_skew_high | -0.142 | ok |
| bp_skew_low | -0.304 | moderate- |

**Consistency summary:**

- **bp_skew_high**: won 3/8, lost 3/8
- **bp_skew_low**: won 3/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 31.5ns | 387.1ns | 8.1% | HIGH |
| bp_skew_high | 30.6ns | 374.2ns | 8.2% | HIGH |
| bp_skew_low | 31.7ns | 389.1ns | 8.1% | HIGH |

## Distribution (algo ns)

```
bp_balanced (n=160, range 239.4-844.3 ns)
    239.4 |############
    269.6 |########################################
    299.9 |
    330.1 |
    360.4 |
    390.6 |
    420.9 |
    451.1 |
    481.4 |
    511.6 |
    541.8 |
    572.1 |
    602.3 |
    632.6 |
    662.8 |
    693.1 |
    723.3 |
    753.6 |
    783.8 |#
    814.0 |
  (24 below, 22 above range)

bp_skew_high (n=160, range 239.4-778.2 ns)
    239.4 |#################
    266.3 |########################################
    293.2 |#
    320.2 |
    347.1 |
    374.1 |
    401.0 |
    428.0 |
    454.9 |
    481.9 |
    508.8 |
    535.7 |
    562.7 |
    589.6 |
    616.6 |
    643.5 |
    670.5 |
    697.4 |#
    724.3 |
    751.3 |
  (13 below, 20 above range)

bp_skew_low (n=160, range 239.2-819.7 ns)
    239.2 |########################################
    268.3 |####################################
    297.3 |#
    326.3 |
    355.3 |
    384.4 |
    413.4 |
    442.4 |
    471.4 |
    500.5 |#
    529.5 |##
    558.5 |
    587.5 |##
    616.5 |##
    645.6 |#
    674.6 |
    703.6 |
    732.6 |##
    761.7 |#
    790.7 |#
  (13 below, 16 above range)

```

## Diagnostics

- **bp_balanced**: CV=62.9% (high variance, measurements may be unstable)
- **bp_balanced**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **bp_skew_high**: CV=60.6% (high variance, measurements may be unstable)
- **bp_skew_high**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **bp_skew_low**: CV=60.0% (high variance, measurements may be unstable)
- **bp_skew_low**: worst_20/best_20 = 3.4x (possible bimodal distribution)
