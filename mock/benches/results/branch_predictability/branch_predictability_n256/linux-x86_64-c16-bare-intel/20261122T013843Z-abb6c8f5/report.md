# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bp_balanced is fastest but the noisiest (CV 79.8%)

bp_balanced wins on median (292 ns) yet has the highest variance (CV 79.8%), while bp_skew_high is the steadiest (CV 75.8%, 292 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bp_skew_high shows warm-up / thermal drift (autocorr +0.59)

bp_skew_high's per-pass series has lag-1 autocorrelation +0.59, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (233 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bp_balanced)

The baseline bp_balanced is the fastest (292 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### bp_skew_high's comparison is tie-heavy (13% tied pairs)

13% of paired samples for bp_skew_high are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader bp_balanced vs stability leader bp_skew_high (+0% speed for 1.1x steadier)

bp_balanced is fastest (292 ns, CV 79.8%); bp_skew_high gives up 0.0% median for 1.1x lower variance (CV 75.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bp_balanced is inconsistent: worst-20% is 3.4x its best-20%

bp_balanced's best 20% of batches run at 239 ns but its worst 20% at 824 ns (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.0% of the fastest

All 3 variants sit between 292 ns and 292 ns - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bp_skew_low's edge over baseline is significant but tiny (0 ns, 0.09%)

bp_skew_low differs from baseline bp_balanced by 0 ns (0.09%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (bp_balanced) is the fastest** at 292.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 292.4 ns, slowest 292.5 ns)

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
| Counter frequency | 2112577410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bp_balanced | 2809ns | 2165ns | 1775ns | 2025ns | 6194ns | base |
| bp_skew_high | 2779ns | 2171ns | 1777ns | 2116ns | 5768ns | -1.06% |
| bp_skew_low | 2852ns | 2175ns | 1949ns | 2177ns | 5783ns | +1.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 376ns | 239ns | 824ns | base | 0.681 |
| bp_skew_high | 372ns | 239ns | 770ns | -0.97% | 0.687 |
| bp_skew_low | 382ns | 262ns | 771ns | +1.64% | 0.670 |

## Performance model

- Peak throughput: **1.070 Gops/s** (bp_balanced; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.876 | 81.8% |
| bp_skew_high | 0.875 | 81.8% |
| bp_skew_low | 0.875 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 2809ns | 2809ns | base |
| bp_skew_high | 2779ns | 2779ns | -1.06% |
| bp_skew_low | 2852ns | 2852ns | +1.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 292ns | base | --- | [239, 292] | --- | --- | --- | --- |
| bp_skew_high | 292ns | no significant difference | [+0, +0]ns | [292, 292] | no | 0.0616 | 0.0616 | **21** (13%, HIGH) |
| bp_skew_low | 292ns | +0.3ns (+0.1%) | [+0, +53]ns | [292, 293] | YES | 0.0004 | 0.0002 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 239ns | +0.0% | -0.0% |
| 1 | 2 | 239ns | -0.0% | +22.2% |
| 1 | 3 | 292ns | -18.1% | +0.0% |
| 1 | 4 | 239ns | +22.2% | +22.2% |
| 1 | 5 | 239ns | +22.2% | +22.2% |
| 1 | 6 | 293ns | -0.0% | +0.0% |
| 1 | 7 | 239ns | +22.3% | +22.2% |
| 1 | 8 | 293ns | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.475 | moderate- |
| bp_skew_high | 0.592 | HIGH+ (drift/warm-up) |
| bp_skew_low | -0.018 | ok |

**Consistency summary:**

- **bp_skew_high**: won 1/8, lost 3/8
- **bp_skew_low**: won 0/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 30.7ns | 376.0ns | 8.2% | HIGH |
| bp_skew_high | 30.6ns | 372.4ns | 8.2% | HIGH |
| bp_skew_low | 31.2ns | 382.2ns | 8.2% | HIGH |

## Distribution (algo ns)

```
bp_balanced (n=160, range 239.3-823.8 ns)
    239.3 |########################################
    268.5 |###########################
    297.7 |#
    327.0 |
    356.2 |
    385.4 |
    414.6 |
    443.9 |
    473.1 |
    502.3 |
    531.5 |
    560.8 |
    590.0 |
    619.2 |
    648.5 |
    677.7 |#
    706.9 |#
    736.1 |##
    765.4 |
    794.6 |##
  (7 below, 13 above range)

bp_skew_high (n=160, range 239.3-769.6 ns)
    239.3 |###################
    265.8 |
    292.3 |########################################
    318.8 |
    345.4 |
    371.9 |
    398.4 |
    424.9 |
    451.4 |
    477.9 |
    504.4 |
    530.9 |
    557.5 |
    584.0 |
    610.5 |
    637.0 |
    663.5 |
    690.0 |
    716.5 |
    743.1 |
  (7 below, 20 above range)

bp_skew_low (n=160, range 262.4-771.1 ns)
    262.4 |
    287.9 |########################################
    313.3 |
    338.7 |
    364.2 |
    389.6 |
    415.0 |
    440.5 |
    465.9 |
    491.3 |
    516.8 |
    542.2 |
    567.6 |
    593.1 |
    618.5 |
    643.9 |
    669.3 |
    694.8 |
    720.2 |
    745.6 |
  (18 below, 21 above range)

```

## Diagnostics

- **bp_balanced**: CV=62.0% (high variance, measurements may be unstable)
- **bp_balanced**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **bp_skew_high**: CV=59.5% (high variance, measurements may be unstable)
- **bp_skew_high**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **bp_skew_high**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **bp_skew_low**: CV=59.2% (high variance, measurements may be unstable)
