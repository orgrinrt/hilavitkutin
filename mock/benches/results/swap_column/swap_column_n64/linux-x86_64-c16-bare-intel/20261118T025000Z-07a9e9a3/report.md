# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cs_plain is fastest but the noisiest (CV 113.6%)

cs_plain wins on median (16.42 us) yet has the highest variance (CV 113.6%), while cs_exch is the steadiest (CV 2.2%, 17.74 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### cs_snap_twin shows alternating (throttle bounce) (autocorr -0.54)

cs_snap_twin's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6.26 us) is smaller than the fastest variant's own run-to-run std-dev (18.65 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (cs_plain)

The baseline cs_plain is the fastest (16.42 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {cs_plain, cs_exch} vs {cs_snap_twin, cs_snap} (26% apart)

The field splits into a fast tier {cs_plain, cs_exch} and a slow tier {cs_snap_twin, cs_snap} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader cs_plain vs stability leader cs_exch (+8% speed for 52.6x steadier)

cs_plain is fastest (16.42 us, CV 113.6%); cs_exch gives up 8.0% median for 52.6x lower variance (CV 2.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### cs_plain is inconsistent: worst-20% is 1.9x its best-20%

cs_plain's best 20% of batches run at 16.17 us but its worst 20% at 30.39 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (cs_plain) is the fastest** at 16419.1 ns median
- 3 variants significantly slower than baseline
- Spread: 1.38x (fastest 16419.1 ns, slowest 22682.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | a buffered column's swap: plain floor vs Snapshot copy vs Exchange |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000004 |
| Counter frequency | 2112802069 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cs_exch | 19257ns | 19373ns | 18712ns | 19163ns | 19670ns | -16.35% |
| cs_plain | 23022ns | 18047ns | 17821ns | 18005ns | 33147ns | base |
| cs_snap | 24504ns | 24317ns | 23710ns | 24222ns | 25322ns | +6.44% |
| cs_snap_twin | 24140ns | 24024ns | 23394ns | 23965ns | 24776ns | +4.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 17619ns | 17064ns | 18033ns | -16.16% | 0.004 |
| cs_plain | 21016ns | 16175ns | 30386ns | base | 0.003 |
| cs_snap | 22852ns | 22075ns | 23680ns | +8.73% | 0.003 |
| cs_snap_twin | 22483ns | 21753ns | 23100ns | +6.98% | 0.003 |

## Performance model

- Peak throughput: **0.004 Gops/s** (cs_plain; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.004 | 91.2% |
| cs_plain | 0.004 | 98.5% |
| cs_snap | 0.003 | 71.3% |
| cs_snap_twin | 0.003 | 72.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 19257ns | 19257ns | -16.35% |
| cs_plain | 23022ns | 23022ns | base |
| cs_snap | 24504ns | 24504ns | +6.44% |
| cs_snap_twin | 24140ns | 24140ns | +4.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 16419ns | base | --- | [16315, 16696] | --- | --- | --- | --- |
| cs_exch | 17739ns | +1112.1ns (+6.8%) | [+849, +1506]ns | [17328, 17876] | YES | 0.0001 | 0.0001 | 0 |
| cs_snap | 22682ns | +6211.7ns (+37.8%) | [+5909, +6666]ns | [22328, 23418] | YES | 0.0001 | 0.0001 | 0 |
| cs_snap_twin | 22379ns | +5944.3ns (+36.2%) | [+5461, +6065]ns | [22183, 22548] | YES | 0.0001 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 16356ns | +10.6% | +44.2% | +35.7% |
| 1 | 2 | 16407ns | +5.6% | +35.2% | +36.2% |
| 1 | 3 | 16457ns | +7.9% | +38.0% | +37.5% |
| 2 | 1 | 16343ns | +9.5% | +45.4% | +33.0% |
| 2 | 2 | 16664ns | +2.2% | +32.8% | +41.6% |
| 2 | 3 | 16431ns | +5.8% | +36.4% | +36.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | -0.233 | moderate- |
| cs_plain | -0.346 | moderate- |
| cs_snap | -0.357 | moderate- |
| cs_snap_twin | -0.536 | HIGH- (thermal bounce) |

**Consistency summary:**

- **cs_exch**: won 0/6, lost 6/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 239515.1ns | 17619.0ns | 1359.4% | HIGH |
| cs_plain | 295291.8ns | 21016.1ns | 1405.1% | HIGH |
| cs_snap | 240214.7ns | 22851.5ns | 1051.2% | HIGH |
| cs_snap_twin | 240422.6ns | 22482.6ns | 1069.4% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 17064.0-18033.0 ns)
  17064.0 |
  17112.5 |
  17160.9 |
  17209.4 |####################
  17257.8 |####################
  17306.3 |########################################
  17354.7 |########################################
  17403.2 |
  17451.6 |
  17500.1 |
  17548.5 |
  17597.0 |
  17645.4 |
  17693.9 |####################
  17742.3 |########################################
  17790.8 |
  17839.2 |########################################
  17887.7 |####################
  17936.1 |####################
  17984.6 |
  (2 below, 3 above range)

cs_plain (n=18, range 16174.6-30385.6 ns)
  16174.6 |########################################
  16885.1 |#####
  17595.7 |
  18306.2 |
  19016.8 |
  19727.3 |
  20437.9 |
  21148.4 |
  21859.0 |
  22569.5 |
  23280.1 |
  23990.6 |
  24701.2 |
  25411.7 |
  26122.3 |
  26832.8 |
  27543.4 |
  28253.9 |
  28964.5 |
  29675.0 |
  (1 below, 1 above range)

cs_snap (n=18, range 22075.1-23679.7 ns)
  22075.1 |####################
  22155.3 |########################################
  22235.6 |
  22315.8 |########################################
  22396.0 |####################
  22476.3 |####################
  22556.5 |
  22636.7 |########################################
  22717.0 |
  22797.2 |
  22877.4 |########################################
  22957.6 |
  23037.9 |
  23118.1 |
  23198.3 |
  23278.6 |
  23358.8 |####################
  23439.0 |
  23519.3 |####################
  23599.5 |
  (1 below, 4 above range)

cs_snap_twin (n=18, range 21752.6-23099.6 ns)
  21752.6 |
  21820.0 |##########
  21887.3 |
  21954.7 |
  22022.0 |
  22089.4 |##########
  22156.7 |####################
  22224.1 |
  22291.4 |##########
  22358.8 |########################################
  22426.1 |####################
  22493.4 |
  22560.8 |
  22628.1 |####################
  22695.5 |
  22762.8 |
  22830.2 |
  22897.5 |
  22964.9 |
  23032.2 |
  (2 below, 3 above range)

```

## Diagnostics

- **cs_exch**: bridge=1348.6% of algo (FFI overhead may distort results)
- **cs_plain**: CV=88.8% (high variance, measurements may be unstable)
- **cs_plain**: bridge=1456.7% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=1057.8% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=1071.2% of algo (FFI overhead may distort results)
