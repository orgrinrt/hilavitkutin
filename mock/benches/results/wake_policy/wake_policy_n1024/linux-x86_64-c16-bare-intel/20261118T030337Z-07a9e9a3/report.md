# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (8.85 us) is smaller than the fastest variant's own run-to-run std-dev (24.84 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### wp_nh8k is inconsistent: worst-20% is 5.2x its best-20%

wp_nh8k's best 20% of batches run at 10.79 us but its worst 20% at 56.43 us (5.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wp_spin** at 17152.8 ns median (-15.3% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.52x (fastest 17152.8 ns, slowest 26006.0 ns)

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
| Counter frequency | 2112580118 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wp_isb8 | 38467ns | 28022ns | 23450ns | 27124ns | 62991ns | base |
| wp_lp0 | 40192ns | 27379ns | 23707ns | 26850ns | 68448ns | +4.48% |
| wp_nh128 | 43427ns | 29802ns | 24334ns | 31075ns | 71502ns | +12.89% |
| wp_nh2k | 49422ns | 32082ns | 26221ns | 38186ns | 77876ns | +28.48% |
| wp_nh8k | 39261ns | 32161ns | 17555ns | 29190ns | 65220ns | +2.06% |
| wp_park | 44674ns | 25360ns | 22693ns | 26725ns | 82587ns | +16.14% |
| wp_spin | 35217ns | 25915ns | 18401ns | 22882ns | 62127ns | -8.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 30527ns | 16969ns | 53756ns | base | 0.034 |
| wp_lp0 | 32335ns | 17459ns | 58838ns | +5.92% | 0.032 |
| wp_nh128 | 34212ns | 17341ns | 60104ns | +12.07% | 0.030 |
| wp_nh2k | 40397ns | 19344ns | 67895ns | +32.33% | 0.025 |
| wp_nh8k | 31721ns | 10792ns | 56431ns | +3.91% | 0.032 |
| wp_park | 37250ns | 16984ns | 74022ns | +22.02% | 0.027 |
| wp_spin | 26852ns | 11488ns | 51992ns | -12.04% | 0.038 |

## Performance model

- Peak throughput: **0.095 Gops/s** (wp_nh8k; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.051 | 53.3% |
| wp_lp0 | 0.052 | 54.5% |
| wp_nh128 | 0.050 | 52.3% |
| wp_nh2k | 0.046 | 48.1% |
| wp_nh8k | 0.039 | 41.5% |
| wp_park | 0.056 | 59.0% |
| wp_spin | 0.060 | 62.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 38467ns | 38467ns | base |
| wp_lp0 | 40192ns | 40192ns | +4.48% |
| wp_nh128 | 43427ns | 43427ns | +12.89% |
| wp_nh2k | 49422ns | 49422ns | +28.48% |
| wp_nh8k | 39261ns | 39261ns | +2.06% |
| wp_park | 44674ns | 44674ns | +16.14% |
| wp_spin | 35217ns | 35217ns | -8.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 20256ns | base | --- | [17574, 44979] | --- | --- | --- | --- |
| wp_lp0 | 19791ns | no significant difference | [-2886, +2108]ns | [18689, 35368] | no | 1.0000 | 1.0000 | 0 |
| wp_nh128 | 20621ns | no significant difference | [-799, +4806]ns | [19464, 40524] | no | 0.7137 | 0.2379 | 0 |
| wp_nh2k | 22415ns | +2538.0ns (+12.5%) | [+1394, +8702]ns | [20971, 61372] | YES (adj: no) | 0.1853 | 0.0309 | 0 |
| wp_nh8k | 26006ns | no significant difference | [-5045, +8477]ns | [15899, 40756] | no | 1.0000 | 1.0000 | 0 |
| wp_park | 18283ns | no significant difference | [-1413, +2913]ns | [17263, 52096] | no | 1.0000 | 0.8145 | 0 |
| wp_spin | 17153ns | no significant difference | [-8708, +1497]ns | [12242, 24758] | no | 0.9614 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 17222ns | +28.8% | +16.7% | +29.8% | -7.0% | -0.6% | +12.8% |
| 1 | 2 | 22439ns | -6.8% | -13.9% | +0.2% | -1.9% | -18.3% | +28.5% |
| 1 | 3 | 22998ns | -20.5% | +58.4% | -5.1% | +14.9% | +253.0% | -47.2% |
| 2 | 1 | 17117ns | +11.5% | +0.7% | +14.0% | +49.5% | +7.1% | -30.0% |
| 2 | 2 | 17926ns | +11.2% | +10.3% | +14.5% | +134.7% | -0.0% | +4.2% |
| 2 | 3 | 19237ns | -6.4% | +9.9% | +235.9% | +27.3% | -5.2% | +7.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | -0.016 | ok |
| wp_lp0 | 0.126 | ok |
| wp_nh128 | -0.371 | moderate- |
| wp_nh2k | -0.049 | ok |
| wp_nh8k | 0.016 | ok |
| wp_park | -0.222 | moderate- |
| wp_spin | -0.079 | ok |

**Consistency summary:**

- **wp_lp0**: won 3/6, lost 3/6
- **wp_nh128**: won 1/6, lost 5/6
- **wp_nh2k**: won 1/6, lost 5/6
- **wp_nh8k**: won 2/6, lost 4/6
- **wp_park**: won 3/6, lost 2/6
- **wp_spin**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 4465.5ns | 30527.1ns | 14.6% | HIGH |
| wp_lp0 | 4378.7ns | 32335.1ns | 13.5% | HIGH |
| wp_nh128 | 4979.1ns | 34211.9ns | 14.6% | HIGH |
| wp_nh2k | 4877.0ns | 40396.8ns | 12.1% | HIGH |
| wp_nh8k | 4404.1ns | 31720.9ns | 13.9% | HIGH |
| wp_park | 4064.4ns | 37249.6ns | 10.9% | HIGH |
| wp_spin | 4742.0ns | 26851.9ns | 17.7% | HIGH |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 16969.5-53756.2 ns)
  16969.5 |########################################
  18808.8 |#############
  20648.1 |#############
  22487.5 |######
  24326.8 |
  26166.1 |
  28005.5 |
  29844.8 |
  31684.1 |
  33523.5 |
  35362.8 |
  37202.2 |
  39041.5 |
  40880.8 |
  42720.2 |######
  44559.5 |######
  46398.8 |
  48238.2 |
  50077.5 |######
  51916.8 |
  (1 below, 3 above range)

wp_lp0 (n=18, range 17459.4-58837.6 ns)
  17459.4 |########################################
  19528.3 |#################
  21597.2 |#####
  23666.1 |
  25735.0 |
  27803.9 |
  29872.9 |#####
  31941.8 |
  34010.7 |
  36079.6 |
  38148.5 |#####
  40217.4 |
  42286.3 |
  44355.2 |
  46424.1 |
  48493.0 |#####
  50561.9 |#####
  52630.9 |
  54699.8 |
  56768.7 |
  (1 below, 2 above range)

wp_nh128 (n=18, range 17340.6-60104.0 ns)
  17340.6 |########################
  19478.7 |########################################
  21616.9 |
  23755.1 |
  25893.3 |
  28031.4 |########
  30169.6 |
  32307.8 |
  34445.9 |########
  36584.1 |########
  38722.3 |
  40860.5 |
  42998.6 |########
  45136.8 |
  47275.0 |
  49413.2 |
  51551.3 |########
  53689.5 |########
  55827.7 |
  57965.8 |
  (2 below, 2 above range)

wp_nh2k (n=18, range 19343.7-67895.4 ns)
  19343.7 |########################################
  21771.3 |########################################
  24198.8 |
  26626.4 |
  29054.0 |
  31481.6 |
  33909.2 |
  36336.8 |
  38764.4 |
  41192.0 |
  43619.5 |
  46047.1 |
  48474.7 |
  50902.3 |
  53329.9 |##########
  55757.5 |####################
  58185.1 |
  60612.7 |
  63040.2 |##########
  65467.8 |####################
  (2 below, 2 above range)

wp_nh8k (n=18, range 10792.3-56431.0 ns)
  10792.3 |##########################
  13074.2 |
  15356.2 |##########################
  17638.1 |
  19920.0 |#############
  22202.0 |
  24483.9 |########################################
  26765.8 |#############
  29047.8 |#############
  31329.7 |
  33611.6 |
  35893.6 |
  38175.5 |#############
  40457.4 |#############
  42739.4 |
  45021.3 |
  47303.2 |
  49585.2 |
  51867.1 |
  54149.0 |#############
  (2 below, 3 above range)

wp_park (n=18, range 16983.5-74022.4 ns)
  16983.5 |########################################
  19835.4 |
  22687.4 |
  25539.3 |
  28391.3 |
  31243.2 |####
  34095.2 |
  36947.1 |
  39799.1 |
  42651.0 |
  45503.0 |
  48354.9 |
  51206.9 |####
  54058.8 |
  56910.8 |
  59762.7 |####
  62614.7 |####
  65466.6 |
  68318.6 |####
  71170.5 |
  (1 below, 2 above range)

wp_spin (n=18, range 11487.9-51991.7 ns)
  11487.9 |########################################
  13513.1 |######
  15538.2 |######
  17563.4 |#############
  19588.6 |#############
  21613.8 |
  23639.0 |
  25664.2 |
  27689.4 |######
  29714.6 |
  31739.8 |
  33765.0 |
  35790.1 |
  37815.3 |
  39840.5 |
  41865.7 |
  43890.9 |
  45916.1 |######
  47941.3 |
  49966.5 |######
  (1 below, 2 above range)

```

## Diagnostics

- **wp_isb8**: CV=56.6% (high variance, measurements may be unstable)
- **wp_isb8**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **wp_isb8**: bridge=21.1% of algo (FFI overhead may distort results)
- **wp_lp0**: CV=72.4% (high variance, measurements may be unstable)
- **wp_lp0**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **wp_lp0**: bridge=22.3% of algo (FFI overhead may distort results)
- **wp_nh128**: CV=66.3% (high variance, measurements may be unstable)
- **wp_nh128**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **wp_nh128**: bridge=22.9% of algo (FFI overhead may distort results)
- **wp_nh2k**: CV=55.8% (high variance, measurements may be unstable)
- **wp_nh2k**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **wp_nh2k**: bridge=21.6% of algo (FFI overhead may distort results)
- **wp_nh8k**: CV=62.4% (high variance, measurements may be unstable)
- **wp_nh8k**: worst_20/best_20 = 5.2x (possible bimodal distribution)
- **wp_nh8k**: bridge=16.2% of algo (FFI overhead may distort results)
- **wp_park**: CV=77.7% (high variance, measurements may be unstable)
- **wp_park**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **wp_park**: bridge=23.3% of algo (FFI overhead may distort results)
- **wp_spin**: CV=92.5% (high variance, measurements may be unstable)
- **wp_spin**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **wp_spin**: bridge=25.4% of algo (FFI overhead may distort results)
