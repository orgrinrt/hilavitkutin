# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wp_nh8k is an outlier: 2.5x slower than the field

wp_nh8k (22.34 us) is 2.5x the fastest (8.84 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {wp_park, wp_nh128, wp_lp0, wp_isb8, wp_nh2k, wp_spin} vs {wp_nh8k} (68% apart)

The field splits into a fast tier {wp_park, wp_nh128, wp_lp0, wp_isb8, wp_nh2k, wp_spin} and a slow tier {wp_nh8k} with a 68% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader wp_park vs stability leader wp_nh128 (+4% speed for 5.5x steadier)

wp_park is fastest (8.84 us, CV 57.3%); wp_nh128 gives up 3.6% median for 5.5x lower variance (CV 10.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### wp_isb8 is inconsistent: worst-20% is 2.7x its best-20%

wp_isb8's best 20% of batches run at 8.78 us but its worst 20% at 24.10 us (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wp_park** at 8839.5 ns median (-25.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.53x (fastest 8839.5 ns, slowest 22335.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | frame wake policy: park-immediately vs bounded spin pre-roll |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000003 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wp_isb8 | 17631ns | 14394ns | 11128ns | 13758ns | 26693ns | base |
| wp_lp0 | 14332ns | 11827ns | 10944ns | 11739ns | 19917ns | -18.71% |
| wp_nh128 | 12037ns | 11746ns | 11072ns | 11627ns | 13135ns | -31.73% |
| wp_nh2k | 17090ns | 15648ns | 14036ns | 15481ns | 21029ns | -3.07% |
| wp_nh8k | 24335ns | 24852ns | 16347ns | 24088ns | 28700ns | +38.02% |
| wp_park | 13416ns | 11346ns | 10658ns | 11191ns | 18131ns | -23.91% |
| wp_spin | 18791ns | 15871ns | 14843ns | 15606ns | 25542ns | +6.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 15106ns | 8778ns | 24105ns | base | 0.068 |
| wp_lp0 | 11883ns | 8544ns | 17496ns | -21.33% | 0.086 |
| wp_nh128 | 9529ns | 8740ns | 10634ns | -36.92% | 0.107 |
| wp_nh2k | 14516ns | 11557ns | 18523ns | -3.91% | 0.071 |
| wp_nh8k | 21722ns | 13975ns | 25914ns | +43.80% | 0.047 |
| wp_park | 10972ns | 8272ns | 15683ns | -27.37% | 0.093 |
| wp_spin | 16281ns | 12261ns | 22982ns | +7.78% | 0.063 |

## Performance model

- Peak throughput: **0.124 Gops/s** (wp_park; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.087 | 70.1% |
| wp_lp0 | 0.110 | 88.5% |
| wp_nh128 | 0.112 | 90.4% |
| wp_nh2k | 0.078 | 63.4% |
| wp_nh8k | 0.046 | 37.0% |
| wp_park | 0.116 | 93.6% |
| wp_spin | 0.077 | 62.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 17631ns | 17631ns | base |
| wp_lp0 | 14332ns | 14332ns | -18.71% |
| wp_nh128 | 12037ns | 12037ns | -31.73% |
| wp_nh2k | 17090ns | 17090ns | -3.07% |
| wp_nh8k | 24335ns | 24335ns | +38.02% |
| wp_park | 13416ns | 13416ns | -23.91% |
| wp_spin | 18791ns | 18791ns | +6.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 11804ns | base | --- | [8992, 20508] | --- | --- | --- | --- |
| wp_lp0 | 9346ns | no significant difference | [-4231, +152]ns | [8888, 10902] | no | 0.4758 | 0.2379 | 0 |
| wp_nh128 | 9154ns | no significant difference | [-11242, +604]ns | [8840, 9571] | no | 0.5768 | 0.4807 | 0 |
| wp_nh2k | 13052ns | no significant difference | [-6144, +4973]ns | [12096, 15329] | no | 0.8145 | 0.8145 | 0 |
| wp_nh8k | 22335ns | +8606.2ns (+72.9%) | [+729, +12904]ns | [20048, 24035] | YES (adj: no) | 0.2888 | 0.0963 | 0 |
| wp_park | 8840ns | -535.4ns (-4.5%) | [-6029, -329]ns | [8483, 9358] | YES | 0.0452 | 0.0075 | 0 |
| wp_spin | 13323ns | no significant difference | [-1373, +4617]ns | [12796, 14544] | no | 0.5768 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 20983ns | -57.8% | -55.1% | -8.3% | +20.2% | +4.0% | -39.7% |
| 1 | 2 | 22275ns | -6.5% | -60.5% | -45.3% | -11.2% | -59.7% | +33.0% |
| 1 | 3 | 8842ns | +10.0% | +19.5% | +52.2% | +201.4% | -2.9% | +60.7% |
| 2 | 1 | 8950ns | -0.7% | +4.7% | +70.7% | +164.3% | -5.3% | +40.3% |
| 2 | 2 | 14942ns | -30.0% | -40.5% | -21.3% | +35.9% | -44.1% | -8.4% |
| 2 | 3 | 9033ns | -4.8% | +23.5% | +32.9% | +118.8% | -2.9% | +47.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | 0.164 | ok |
| wp_lp0 | -0.264 | moderate- |
| wp_nh128 | -0.384 | moderate- |
| wp_nh2k | -0.184 | ok |
| wp_nh8k | -0.228 | moderate- |
| wp_park | -0.000 | ok |
| wp_spin | -0.221 | moderate- |

**Consistency summary:**

- **wp_lp0**: won 5/6, lost 1/6
- **wp_nh128**: won 3/6, lost 3/6
- **wp_nh2k**: won 3/6, lost 3/6
- **wp_nh8k**: won 1/6, lost 5/6
- **wp_park**: won 5/6, lost 1/6
- **wp_spin**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 1357.7ns | 15105.8ns | 9.0% | HIGH |
| wp_lp0 | 1478.0ns | 11883.3ns | 12.4% | HIGH |
| wp_nh128 | 1472.0ns | 9529.4ns | 15.4% | HIGH |
| wp_nh2k | 1354.6ns | 14515.7ns | 9.3% | HIGH |
| wp_nh8k | 1353.5ns | 21722.0ns | 6.2% | HIGH |
| wp_park | 1352.3ns | 10971.5ns | 12.3% | HIGH |
| wp_spin | 1352.6ns | 16280.6ns | 8.3% | HIGH |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 8777.8-24104.9 ns)
   8777.8 |########################################
   9544.1 |
  10310.5 |
  11076.8 |
  11843.2 |
  12609.5 |
  13375.9 |
  14142.2 |#####
  14908.6 |#####
  15675.0 |
  16441.3 |
  17207.7 |#####
  17974.0 |
  18740.4 |
  19506.7 |#####
  20273.1 |#####
  21039.4 |
  21805.8 |##########
  22572.1 |
  23338.5 |
  (1 below, 2 above range)

wp_lp0 (n=18, range 8544.5-17495.8 ns)
   8544.5 |########################################
   8992.0 |########################
   9439.6 |################
   9887.2 |########
  10334.7 |########
  10782.3 |
  11229.9 |########
  11677.4 |
  12125.0 |
  12572.6 |
  13020.1 |
  13467.7 |
  13915.3 |########
  14362.8 |
  14810.4 |
  15258.0 |
  15705.5 |
  16153.1 |
  16600.7 |
  17048.2 |
  (1 below, 3 above range)

wp_nh128 (n=18, range 8740.3-10634.1 ns)
   8740.3 |########################################
   8835.0 |##############################
   8929.7 |##########
   9024.4 |
   9119.1 |
   9213.7 |
   9308.4 |##########
   9403.1 |####################
   9497.8 |####################
   9592.5 |
   9687.2 |
   9781.9 |
   9876.6 |
   9971.2 |
  10065.9 |
  10160.6 |
  10255.3 |
  10350.0 |
  10444.7 |
  10539.4 |##########
  (1 below, 3 above range)

wp_nh2k (n=18, range 11556.9-18522.9 ns)
  11556.9 |########################################
  11905.2 |##########################
  12253.5 |#############
  12601.8 |#############
  12950.1 |##########################
  13298.4 |#############
  13646.7 |
  13995.0 |
  14343.3 |
  14691.6 |#############
  15039.9 |##########################
  15388.2 |
  15736.5 |
  16084.8 |#############
  16433.1 |
  16781.4 |
  17129.7 |
  17478.0 |
  17826.3 |
  18174.6 |
  (1 below, 3 above range)

wp_nh8k (n=18, range 13975.0-25913.9 ns)
  13975.0 |
  14571.9 |
  15168.9 |
  15765.8 |
  16362.8 |
  16959.7 |
  17556.7 |
  18153.6 |
  18750.6 |#############
  19347.5 |##########################
  19944.4 |#############
  20541.4 |##########################
  21138.3 |
  21735.3 |#############
  22332.2 |#############
  22929.2 |
  23526.1 |########################################
  24123.1 |#############
  24720.0 |#############
  25316.9 |
  (2 below, 3 above range)

wp_park (n=18, range 8272.2-15682.6 ns)
   8272.2 |########################################
   8642.8 |################################
   9013.3 |################
   9383.8 |########
   9754.3 |########
  10124.8 |
  10495.4 |
  10865.9 |
  11236.4 |
  11606.9 |
  11977.4 |
  12348.0 |
  12718.5 |
  13089.0 |
  13459.5 |
  13830.0 |
  14200.6 |
  14571.1 |
  14941.6 |
  15312.1 |
  (2 below, 3 above range)

wp_spin (n=18, range 12261.1-22982.0 ns)
  12261.1 |##############################
  12797.2 |########################################
  13333.2 |##############################
  13869.3 |##########
  14405.3 |##########
  14941.3 |##########
  15477.4 |
  16013.4 |
  16549.5 |
  17085.5 |
  17621.5 |
  18157.6 |
  18693.6 |
  19229.7 |
  19765.7 |
  20301.7 |
  20837.8 |
  21373.8 |
  21909.9 |
  22445.9 |
  (2 below, 3 above range)

```

## Diagnostics

- **wp_isb8**: CV=47.7% (high variance, measurements may be unstable)
- **wp_isb8**: bridge=11.5% of algo (FFI overhead may distort results)
- **wp_lp0**: CV=44.4% (high variance, measurements may be unstable)
- **wp_lp0**: bridge=14.4% of algo (FFI overhead may distort results)
- **wp_nh128**: bridge=14.7% of algo (FFI overhead may distort results)
- **wp_nh2k**: CV=24.1% (high variance, measurements may be unstable)
- **wp_nh2k**: bridge=10.4% of algo (FFI overhead may distort results)
- **wp_nh8k**: CV=20.6% (high variance, measurements may be unstable)
- **wp_park**: CV=46.1% (high variance, measurements may be unstable)
- **wp_park**: bridge=15.3% of algo (FFI overhead may distort results)
- **wp_spin**: CV=42.5% (high variance, measurements may be unstable)
- **wp_spin**: bridge=10.1% of algo (FFI overhead may distort results)
