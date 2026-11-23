# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wp_spin beats baseline by 28% (significant)

wp_spin is -6.45 us (28%) faster than baseline wp_isb8, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### wp_lp0 shows alternating (throttle bounce) (autocorr -0.64)

wp_lp0's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5.71 us) is smaller than the fastest variant's own run-to-run std-dev (20.18 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### wp_nh8k is inconsistent: worst-20% is 6.0x its best-20%

wp_nh8k's best 20% of batches run at 10.33 us but its worst 20% at 62.14 us (6.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wp_spin** at 18788.3 ns median (-19.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.30x (fastest 18788.3 ns, slowest 24497.1 ns)

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
| Counter frequency | 2112740287 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wp_isb8 | 45258ns | 32794ns | 23984ns | 32181ns | 75510ns | base |
| wp_lp0 | 36724ns | 27116ns | 23358ns | 27273ns | 57582ns | -18.86% |
| wp_nh128 | 41164ns | 30336ns | 23627ns | 31962ns | 63737ns | -9.05% |
| wp_nh2k | 49381ns | 32694ns | 26157ns | 35310ns | 82102ns | +9.11% |
| wp_nh8k | 40132ns | 27767ns | 17223ns | 26780ns | 71615ns | -11.33% |
| wp_park | 42003ns | 29124ns | 24720ns | 30649ns | 67675ns | -7.19% |
| wp_spin | 35262ns | 26476ns | 18114ns | 24065ns | 60632ns | -22.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 36506ns | 17815ns | 65531ns | base | 0.028 |
| wp_lp0 | 28629ns | 16747ns | 48037ns | -21.58% | 0.036 |
| wp_nh128 | 33043ns | 16626ns | 54844ns | -9.49% | 0.031 |
| wp_nh2k | 40568ns | 18667ns | 71650ns | +11.13% | 0.025 |
| wp_nh8k | 31919ns | 10331ns | 62141ns | -12.57% | 0.032 |
| wp_park | 33430ns | 18112ns | 57788ns | -8.43% | 0.031 |
| wp_spin | 27054ns | 11240ns | 51069ns | -25.89% | 0.038 |

## Performance model

- Peak throughput: **0.099 Gops/s** (wp_nh8k; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.044 | 44.1% |
| wp_lp0 | 0.052 | 52.9% |
| wp_nh128 | 0.047 | 47.2% |
| wp_nh2k | 0.042 | 42.2% |
| wp_nh8k | 0.050 | 50.6% |
| wp_park | 0.046 | 46.8% |
| wp_spin | 0.055 | 55.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 45258ns | 45258ns | base |
| wp_lp0 | 36724ns | 36724ns | -18.86% |
| wp_nh128 | 41164ns | 41164ns | -9.05% |
| wp_nh2k | 49381ns | 49381ns | +9.11% |
| wp_nh8k | 40132ns | 40132ns | -11.33% |
| wp_park | 42003ns | 42003ns | -7.19% |
| wp_spin | 35262ns | 35262ns | -22.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 23420ns | base | --- | [19182, 50802] | --- | --- | --- | --- |
| wp_lp0 | 19529ns | no significant difference | [-10184, +1511]ns | [18780, 33665] | no | 0.5768 | 0.4807 | 0 |
| wp_nh128 | 21899ns | no significant difference | [-10782, +2427]ns | [19153, 47788] | no | 0.5768 | 0.4807 | 0 |
| wp_nh2k | 24497ns | no significant difference | [-3146, +6617]ns | [19793, 61940] | no | 0.5768 | 0.4807 | 0 |
| wp_nh8k | 20413ns | -7420.1ns (-31.7%) | [-10298, -518]ns | [12236, 36554] | YES (adj: no) | 0.2888 | 0.0963 | 0 |
| wp_park | 22080ns | no significant difference | [-7234, +4157]ns | [19121, 41794] | no | 1.0000 | 1.0000 | 0 |
| wp_spin | 18788ns | -6446.5ns (-27.5%) | [-12286, -411]ns | [12908, 37126] | YES (adj: no) | 0.1853 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 17932ns | +8.1% | +14.7% | +25.8% | +124.4% | +25.5% | -2.9% |
| 1 | 2 | 41916ns | -56.0% | -54.0% | -53.2% | -11.5% | -52.5% | -72.7% |
| 1 | 3 | 19770ns | +12.5% | +17.5% | +0.2% | +12.7% | +51.2% | -33.6% |
| 2 | 1 | 20656ns | -17.7% | -14.2% | -4.9% | -35.5% | +1.8% | +6.0% |
| 2 | 2 | 17978ns | +7.8% | +113.6% | +44.3% | -36.7% | +2.0% | +12.2% |
| 2 | 3 | 24962ns | -21.2% | +122.6% | +125.8% | -40.5% | -25.4% | -49.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | -0.367 | moderate- |
| wp_lp0 | -0.643 | HIGH- (thermal bounce) |
| wp_nh128 | 0.312 | moderate+ |
| wp_nh2k | 0.118 | ok |
| wp_nh8k | 0.567 | HIGH+ (drift/warm-up) |
| wp_park | -0.099 | ok |
| wp_spin | 0.002 | ok |

**Consistency summary:**

- **wp_lp0**: won 3/6, lost 3/6
- **wp_nh128**: won 2/6, lost 4/6
- **wp_nh2k**: won 2/6, lost 4/6
- **wp_nh8k**: won 4/6, lost 2/6
- **wp_park**: won 2/6, lost 4/6
- **wp_spin**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 4829.0ns | 36506.3ns | 13.2% | HIGH |
| wp_lp0 | 4571.0ns | 28629.5ns | 16.0% | HIGH |
| wp_nh128 | 4662.8ns | 33042.9ns | 14.1% | HIGH |
| wp_nh2k | 5036.0ns | 40567.9ns | 12.4% | HIGH |
| wp_nh8k | 5491.9ns | 31919.2ns | 17.2% | HIGH |
| wp_park | 5022.2ns | 33429.8ns | 15.0% | HIGH |
| wp_spin | 4681.2ns | 27054.2ns | 17.3% | HIGH |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 17815.0-65530.5 ns)
  17815.0 |########################################
  20200.7 |################
  22586.5 |################
  24972.3 |
  27358.1 |
  29743.9 |
  32129.6 |
  34515.4 |
  36901.2 |
  39287.0 |
  41672.7 |########
  44058.5 |
  46444.3 |########
  48830.1 |
  51215.9 |########
  53601.6 |
  55987.4 |
  58373.2 |
  60759.0 |########
  63144.7 |
  (2 below, 3 above range)

wp_lp0 (n=18, range 16746.8-48036.9 ns)
  16746.8 |#############
  18311.3 |########################################
  19875.8 |
  21440.3 |#############
  23004.8 |
  24569.3 |
  26133.8 |
  27698.3 |
  29262.8 |
  30827.3 |######
  32391.8 |
  33956.3 |######
  35520.9 |
  37085.4 |
  38649.9 |
  40214.4 |
  41778.9 |
  43343.4 |
  44907.9 |
  46472.4 |
  (2 below, 4 above range)

wp_nh128 (n=18, range 16626.5-54844.4 ns)
  16626.5 |####################
  18537.4 |########################################
  20448.3 |##########
  22359.2 |##########
  24270.1 |
  26181.0 |
  28091.8 |
  30002.7 |
  31913.6 |
  33824.5 |
  35735.4 |##########
  37646.3 |##########
  39557.2 |
  41468.1 |
  43379.0 |
  45289.9 |
  47200.8 |##########
  49111.7 |####################
  51022.6 |
  52933.5 |
  (2 below, 3 above range)

wp_nh2k (n=18, range 18667.3-71650.4 ns)
  18667.3 |########################################
  21316.4 |########################
  23965.6 |########
  26614.7 |
  29263.9 |
  31913.1 |
  34562.2 |
  37211.4 |
  39860.5 |
  42509.7 |########
  45158.9 |
  47808.0 |########
  50457.2 |
  53106.3 |
  55755.5 |########
  58404.7 |
  61053.8 |
  63703.0 |
  66352.1 |########
  69001.3 |
  (1 below, 4 above range)

wp_nh8k (n=18, range 10330.7-62141.3 ns)
  10330.7 |########################################
  12921.2 |################
  15511.8 |
  18102.3 |########
  20692.8 |########
  23283.4 |
  25873.9 |
  28464.4 |
  31055.0 |################
  33645.5 |########
  36236.0 |########
  38826.6 |########
  41417.1 |
  44007.6 |
  46598.2 |
  49188.7 |
  51779.2 |
  54369.8 |
  56960.3 |
  59550.8 |
  (1 below, 3 above range)

wp_park (n=18, range 18112.4-57788.3 ns)
  18112.4 |########################################
  20096.2 |#############
  22080.0 |######
  24063.8 |
  26047.6 |
  28031.4 |#############
  30015.2 |
  31999.0 |
  33982.8 |
  35966.6 |
  37950.4 |
  39934.1 |######
  41917.9 |######
  43901.7 |
  45885.5 |
  47869.3 |
  49853.1 |
  51836.9 |######
  53820.7 |
  55804.5 |
  (1 below, 3 above range)

wp_spin (n=18, range 11240.0-51069.0 ns)
  11240.0 |########################################
  13231.4 |################
  15222.9 |
  17214.3 |########
  19205.8 |################
  21197.2 |########
  23188.7 |
  25180.1 |
  27171.6 |
  29163.0 |
  31154.5 |
  33145.9 |
  35137.4 |########
  37128.8 |################
  39120.3 |
  41111.7 |
  43103.2 |########
  45094.6 |
  47086.1 |
  49077.5 |
  (1 below, 2 above range)

```

## Diagnostics

- **wp_isb8**: CV=61.2% (high variance, measurements may be unstable)
- **wp_isb8**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **wp_isb8**: bridge=20.4% of algo (FFI overhead may distort results)
- **wp_lp0**: CV=53.8% (high variance, measurements may be unstable)
- **wp_lp0**: bridge=24.2% of algo (FFI overhead may distort results)
- **wp_nh128**: CV=51.1% (high variance, measurements may be unstable)
- **wp_nh128**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **wp_nh128**: bridge=20.7% of algo (FFI overhead may distort results)
- **wp_nh2k**: CV=58.9% (high variance, measurements may be unstable)
- **wp_nh2k**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **wp_nh2k**: bridge=19.8% of algo (FFI overhead may distort results)
- **wp_nh8k**: CV=85.8% (high variance, measurements may be unstable)
- **wp_nh8k**: worst_20/best_20 = 6.0x (possible bimodal distribution)
- **wp_nh8k**: autocorrelation=0.57 (measurement drift or warm-up artifact)
- **wp_nh8k**: bridge=22.8% of algo (FFI overhead may distort results)
- **wp_park**: CV=56.9% (high variance, measurements may be unstable)
- **wp_park**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **wp_park**: bridge=21.0% of algo (FFI overhead may distort results)
- **wp_spin**: CV=74.6% (high variance, measurements may be unstable)
- **wp_spin**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **wp_spin**: bridge=24.8% of algo (FFI overhead may distort results)
