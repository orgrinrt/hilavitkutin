# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 1.15 us to slowest 1.15 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (561 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader bp_skew_high vs stability leader bp_balanced (+0% speed for 1.1x steadier)

bp_skew_high is fastest (1.15 us, CV 48.9%); bp_balanced gives up 0.0% median for 1.1x lower variance (CV 43.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bp_skew_low is inconsistent: worst-20% is 2.6x its best-20%

bp_skew_low's best 20% of batches run at 939 ns but its worst 20% at 2.41 us (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bp_skew_high** at 1147.4 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 1147.4 ns, slowest 1147.5 ns)

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
| bp_balanced | 3167ns | 3023ns | 2474ns | 2896ns | 4675ns | base |
| bp_skew_high | 3228ns | 3019ns | 2472ns | 2809ns | 5241ns | +1.93% |
| bp_skew_low | 3375ns | 3021ns | 2475ns | 2796ns | 6011ns | +6.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 1198ns | 939ns | 1755ns | base | 0.855 |
| bp_skew_high | 1221ns | 939ns | 1967ns | +1.95% | 0.839 |
| bp_skew_low | 1307ns | 939ns | 2415ns | +9.12% | 0.784 |

## Performance model

- Peak throughput: **1.091 Gops/s** (bp_skew_low; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.892 | 81.8% |
| bp_skew_high | 0.892 | 81.8% |
| bp_skew_low | 0.892 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 3167ns | 3167ns | base |
| bp_skew_high | 3228ns | 3228ns | +1.93% |
| bp_skew_low | 3375ns | 3375ns | +6.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 1148ns | base | --- | [1148, 1148] | --- | --- | --- | --- |
| bp_skew_high | 1147ns | no significant difference | [-0, +0]ns | [950, 1148] | no | 0.3855 | 0.1927 | 9 |
| bp_skew_low | 1148ns | no significant difference | [-0, +0]ns | [949, 1148] | no | 0.7433 | 0.7433 | 11 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 939ns | +0.0% | -0.0% |
| 1 | 2 | 1148ns | -18.2% | +0.0% |
| 1 | 3 | 1148ns | -18.2% | -18.2% |
| 1 | 4 | 939ns | +22.2% | +133.3% |
| 1 | 5 | 1148ns | -0.0% | -18.2% |
| 1 | 6 | 1148ns | -0.0% | +0.0% |
| 1 | 7 | 939ns | +22.2% | +22.2% |
| 1 | 8 | 1148ns | -18.2% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.475 | moderate- |
| bp_skew_high | 0.375 | moderate+ |
| bp_skew_low | -0.360 | moderate- |

**Consistency summary:**

- **bp_skew_high**: won 3/8, lost 2/8
- **bp_skew_low**: won 3/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 24.8ns | 1197.7ns | 2.1% |  |
| bp_skew_high | 25.5ns | 1221.0ns | 2.1% |  |
| bp_skew_low | 25.9ns | 1306.9ns | 2.0% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 938.9-1755.0 ns)
    938.9 |###############
    979.7 |
   1020.5 |
   1061.3 |
   1102.1 |
   1142.9 |########################################
   1183.7 |
   1224.5 |
   1265.3 |
   1306.1 |
   1346.9 |
   1387.7 |
   1428.5 |
   1469.3 |
   1510.1 |
   1550.9 |
   1591.7 |
   1632.6 |
   1673.4 |
   1714.2 |
  (19 below, 10 above range)

bp_skew_high (n=160, range 938.9-1967.4 ns)
    938.9 |############################
    990.3 |
   1041.8 |
   1093.2 |
   1144.6 |########################################
   1196.0 |
   1247.5 |
   1298.9 |
   1350.3 |
   1401.7 |
   1453.2 |
   1504.6 |
   1556.0 |
   1607.4 |
   1658.9 |
   1710.3 |
   1761.7 |
   1813.1 |#
   1864.6 |
   1916.0 |
  (19 below, 12 above range)

bp_skew_low (n=160, range 938.9-2414.6 ns)
    938.9 |##################################
   1012.7 |
   1086.5 |########################################
   1160.3 |
   1234.0 |
   1307.8 |
   1381.6 |
   1455.4 |
   1529.2 |
   1603.0 |
   1676.7 |
   1750.5 |
   1824.3 |
   1898.1 |
   1971.9 |
   2045.7 |
   2119.4 |#############
   2193.2 |#
   2267.0 |
   2340.8 |
  (26 below, 8 above range)

```

## Diagnostics

- **bp_balanced**: CV=41.3% (high variance, measurements may be unstable)
- **bp_skew_high**: CV=46.0% (high variance, measurements may be unstable)
- **bp_skew_low**: CV=46.6% (high variance, measurements may be unstable)
