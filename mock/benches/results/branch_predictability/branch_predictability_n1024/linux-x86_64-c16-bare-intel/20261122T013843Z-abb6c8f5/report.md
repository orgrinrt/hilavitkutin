# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 1.15 us to slowest 1.15 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### bp_skew_high shows warm-up / thermal drift (autocorr +0.59)

bp_skew_high's per-pass series has lag-1 autocorrelation +0.59, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (497 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bp_balanced)

The baseline bp_balanced is the fastest (1.15 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### bp_skew_high's comparison is tie-heavy (18% tied pairs)

18% of paired samples for bp_skew_high are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader bp_balanced vs stability leader bp_skew_high (+0% speed for 1.1x steadier)

bp_balanced is fastest (1.15 us, CV 43.4%); bp_skew_high gives up 0.0% median for 1.1x lower variance (CV 40.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bp_skew_low is inconsistent: worst-20% is 2.0x its best-20%

bp_skew_low's best 20% of batches run at 939 ns but its worst 20% at 1.91 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bp_balanced) is the fastest** at 1147.5 ns median
- Spread: 1.00x (fastest 1147.5 ns, slowest 1147.6 ns)

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
| bp_balanced | 3215ns | 3025ns | 2474ns | 2909ns | 4877ns | base |
| bp_skew_high | 3202ns | 3025ns | 2474ns | 2920ns | 4778ns | -0.41% |
| bp_skew_low | 3262ns | 3028ns | 2478ns | 2911ns | 5097ns | +1.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 1216ns | 939ns | 1835ns | base | 0.842 |
| bp_skew_high | 1211ns | 939ns | 1798ns | -0.42% | 0.845 |
| bp_skew_low | 1230ns | 939ns | 1906ns | +1.14% | 0.832 |

## Performance model

- Peak throughput: **1.091 Gops/s** (bp_skew_high; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.892 | 81.8% |
| bp_skew_high | 0.892 | 81.8% |
| bp_skew_low | 0.892 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 3215ns | 3215ns | base |
| bp_skew_high | 3202ns | 3202ns | -0.41% |
| bp_skew_low | 3262ns | 3262ns | +1.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 1148ns | base | --- | [1148, 1148] | --- | --- | --- | --- |
| bp_skew_high | 1148ns | no significant difference | [-0, +0]ns | [1148, 1148] | no | 0.2578 | 0.2578 | **28** (18%, HIGH) |
| bp_skew_low | 1148ns | no significant difference | [+0, +0]ns | [1148, 1148] | no | 0.2578 | 0.1982 | **24** (15%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 1148ns | -18.2% | +0.0% |
| 1 | 2 | 939ns | -0.0% | +0.0% |
| 1 | 3 | 939ns | -0.0% | +0.0% |
| 1 | 4 | 1148ns | -0.0% | +0.0% |
| 1 | 5 | 939ns | +22.2% | +22.2% |
| 1 | 6 | 1148ns | +0.0% | -18.2% |
| 1 | 7 | 1148ns | +0.0% | +0.0% |
| 1 | 8 | 1148ns | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.142 | ok |
| bp_skew_high | 0.592 | HIGH+ (drift/warm-up) |
| bp_skew_low | -0.142 | ok |

**Consistency summary:**

- **bp_skew_high**: won 1/8, lost 1/8
- **bp_skew_low**: won 1/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 25.2ns | 1216.4ns | 2.1% |  |
| bp_skew_high | 25.1ns | 1211.3ns | 2.1% |  |
| bp_skew_low | 25.7ns | 1230.3ns | 2.1% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 938.9-1835.1 ns)
    938.9 |############
    983.7 |
   1028.5 |
   1073.3 |
   1118.1 |########################################
   1162.9 |
   1207.8 |
   1252.6 |
   1297.4 |
   1342.2 |
   1387.0 |
   1431.8 |
   1476.6 |
   1521.4 |
   1566.2 |
   1611.0 |
   1655.8 |
   1700.6 |
   1745.5 |
   1790.3 |
  (23 below, 13 above range)

bp_skew_high (n=160, range 938.9-1798.3 ns)
    938.9 |#####################
    981.9 |
   1024.8 |
   1067.8 |
   1110.8 |########################################
   1153.7 |####
   1196.7 |
   1239.7 |
   1282.7 |
   1325.6 |
   1368.6 |
   1411.6 |
   1454.6 |
   1497.5 |
   1540.5 |
   1583.5 |
   1626.5 |
   1669.4 |
   1712.4 |
   1755.4 |
  (6 below, 13 above range)

bp_skew_low (n=160, range 938.9-1905.7 ns)
    938.9 |#############
    987.3 |
   1035.6 |
   1083.9 |
   1132.3 |########################################
   1180.6 |
   1229.0 |
   1277.3 |
   1325.6 |
   1374.0 |
   1422.3 |
   1470.6 |
   1519.0 |
   1567.3 |
   1615.7 |
   1664.0 |
   1712.3 |
   1760.7 |
   1809.0 |
   1857.3 |
  (21 below, 14 above range)

```

## Diagnostics

- **bp_balanced**: CV=40.9% (high variance, measurements may be unstable)
- **bp_skew_high**: CV=38.5% (high variance, measurements may be unstable)
- **bp_skew_high**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **bp_skew_low**: CV=43.7% (high variance, measurements may be unstable)
