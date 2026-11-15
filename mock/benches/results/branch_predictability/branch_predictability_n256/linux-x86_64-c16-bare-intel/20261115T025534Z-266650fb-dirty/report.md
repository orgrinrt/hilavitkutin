# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (241 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bp_skew_high's comparison is tie-heavy (12% tied pairs)

12% of paired samples for bp_skew_high are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader bp_skew_high vs stability leader bp_skew_low (+0% speed for 1.1x steadier)

bp_skew_high is fastest (292 ns, CV 82.5%); bp_skew_low gives up 0.1% median for 1.1x lower variance (CV 78.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bp_balanced is inconsistent: worst-20% is 3.6x its best-20%

bp_balanced's best 20% of batches run at 239 ns but its worst 20% at 857 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.1% of the fastest

All 3 variants sit between 292 ns and 292 ns - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bp_skew_low's edge over baseline is significant but tiny (0 ns, 0.07%)

bp_skew_low differs from baseline bp_balanced by 0 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bp_skew_high** at 292.2 ns median (-0.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 292.2 ns, slowest 292.5 ns)

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
| Counter frequency | 2112760617 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bp_balanced | 2899ns | 2167ns | 1773ns | 2097ns | 6428ns | base |
| bp_skew_high | 2864ns | 2162ns | 1772ns | 2061ns | 6364ns | -1.19% |
| bp_skew_low | 2859ns | 2172ns | 1972ns | 2174ns | 5800ns | -1.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 389ns | 239ns | 857ns | base | 0.659 |
| bp_skew_high | 384ns | 239ns | 850ns | -1.09% | 0.666 |
| bp_skew_low | 384ns | 266ns | 775ns | -1.28% | 0.667 |

## Performance model

- Peak throughput: **1.070 Gops/s** (bp_skew_high; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.876 | 81.8% |
| bp_skew_high | 0.876 | 81.8% |
| bp_skew_low | 0.875 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 2899ns | 2899ns | base |
| bp_skew_high | 2864ns | 2864ns | -1.19% |
| bp_skew_low | 2859ns | 2859ns | -1.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 292ns | base | --- | [292, 292] | --- | --- | --- | --- |
| bp_skew_high | 292ns | no significant difference | [-0, +0]ns | [239, 292] | no | 0.0918 | 0.0918 | **19** (12%, HIGH) |
| bp_skew_low | 292ns | +0.2ns (+0.1%) | [+0, +0]ns | [292, 292] | YES | 0.0040 | 0.0020 | 7 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 239ns | +0.0% | +22.3% |
| 1 | 2 | 293ns | +0.0% | -18.2% |
| 1 | 3 | 292ns | +0.0% | +0.0% |
| 1 | 4 | 292ns | -18.2% | -0.0% |
| 1 | 5 | 292ns | -18.2% | +0.0% |
| 1 | 6 | 239ns | +22.2% | +22.2% |
| 1 | 7 | 292ns | -18.2% | +0.0% |
| 1 | 8 | 239ns | +0.0% | +22.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.275 | moderate- |
| bp_skew_high | -0.141 | ok |
| bp_skew_low | -0.163 | ok |

**Consistency summary:**

- **bp_skew_high**: won 3/8, lost 1/8
- **bp_skew_low**: won 1/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 31.6ns | 388.6ns | 8.1% | HIGH |
| bp_skew_high | 31.2ns | 384.4ns | 8.1% | HIGH |
| bp_skew_low | 31.2ns | 383.7ns | 8.1% | HIGH |

## Distribution (algo ns)

```
bp_balanced (n=160, range 239.2-856.6 ns)
    239.2 |###################
    270.0 |########################################
    300.9 |
    331.8 |
    362.7 |
    393.5 |
    424.4 |
    455.3 |
    486.1 |
    517.0 |
    547.9 |
    578.7 |#
    609.6 |#
    640.5 |
    671.3 |
    702.2 |
    733.1 |
    764.0 |
    794.8 |#
    825.7 |
  (12 below, 20 above range)

bp_skew_high (n=160, range 239.1-849.7 ns)
    239.1 |########################################
    269.7 |##############################
    300.2 |#
    330.7 |
    361.3 |
    391.8 |
    422.3 |
    452.9 |
    483.4 |
    513.9 |
    544.4 |
    575.0 |
    605.5 |
    636.0 |
    666.6 |
    697.1 |
    727.6 |#
    758.2 |###
    788.7 |###
    819.2 |#
  (15 below, 14 above range)

bp_skew_low (n=160, range 266.0-774.9 ns)
    266.0 |
    291.4 |########################################
    316.9 |
    342.3 |
    367.8 |
    393.2 |
    418.7 |
    444.1 |
    469.5 |
    495.0 |
    520.4 |
    545.9 |
    571.3 |
    596.8 |
    622.2 |
    647.7 |
    673.1 |
    698.6 |
    724.0 |
    749.5 |
  (16 below, 21 above range)

```

## Diagnostics

- **bp_balanced**: CV=63.6% (high variance, measurements may be unstable)
- **bp_balanced**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **bp_skew_high**: CV=62.7% (high variance, measurements may be unstable)
- **bp_skew_high**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **bp_skew_low**: CV=59.6% (high variance, measurements may be unstable)
