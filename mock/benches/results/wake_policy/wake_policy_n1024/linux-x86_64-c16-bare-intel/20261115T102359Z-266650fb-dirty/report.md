# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wp_spin beats baseline by 35% (significant)

wp_spin is -9.71 us (35%) faster than baseline wp_isb8, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### wp_spin is fastest but the noisiest (CV 92.4%)

wp_spin wins on median (21.94 us) yet has the highest variance (CV 92.4%), while wp_nh8k is the steadiest (CV 63.4%, 24.03 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wp_nh8k shows warm-up / thermal drift (autocorr +0.52)

wp_nh8k's per-pass series has lag-1 autocorrelation +0.52, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (19.50 us) is smaller than the fastest variant's own run-to-run std-dev (20.27 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {wp_spin, wp_nh8k, wp_park, wp_nh128, wp_isb8, wp_lp0} vs {wp_nh2k} (36% apart)

The field splits into a fast tier {wp_spin, wp_nh8k, wp_park, wp_nh128, wp_isb8, wp_lp0} and a slow tier {wp_nh2k} with a 36% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader wp_spin vs stability leader wp_nh8k (+10% speed for 1.5x steadier)

wp_spin is fastest (21.94 us, CV 92.4%); wp_nh8k gives up 9.5% median for 1.5x lower variance (CV 63.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### wp_spin is inconsistent: worst-20% is 4.5x its best-20%

wp_spin's best 20% of batches run at 12.08 us but its worst 20% at 54.71 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wp_spin** at 21936.0 ns median (-19.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.89x (fastest 21936.0 ns, slowest 41437.1 ns)

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
| Counter frequency | 2112761518 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wp_isb8 | 51913ns | 38600ns | 28496ns | 36959ns | 86054ns | base |
| wp_lp0 | 50425ns | 42190ns | 24489ns | 36066ns | 84932ns | -2.87% |
| wp_nh128 | 42920ns | 34044ns | 22040ns | 31821ns | 70008ns | -17.32% |
| wp_nh2k | 56801ns | 51184ns | 25141ns | 44309ns | 91369ns | +9.41% |
| wp_nh8k | 38610ns | 31248ns | 20110ns | 30809ns | 59560ns | -25.63% |
| wp_park | 43002ns | 36141ns | 23244ns | 32914ns | 68013ns | -17.17% |
| wp_spin | 39548ns | 30734ns | 19194ns | 29233ns | 65198ns | -23.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 41342ns | 20424ns | 73506ns | base | 0.025 |
| wp_lp0 | 40176ns | 17393ns | 72634ns | -2.82% | 0.025 |
| wp_nh128 | 34027ns | 15467ns | 59948ns | -17.69% | 0.030 |
| wp_nh2k | 47337ns | 18316ns | 80358ns | +14.50% | 0.022 |
| wp_nh8k | 29841ns | 12677ns | 48752ns | -27.82% | 0.034 |
| wp_park | 33883ns | 16254ns | 57773ns | -18.04% | 0.030 |
| wp_spin | 30616ns | 12076ns | 54707ns | -25.94% | 0.033 |

## Performance model

- Peak throughput: **0.085 Gops/s** (wp_spin; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.037 | 44.1% |
| wp_lp0 | 0.034 | 39.6% |
| wp_nh128 | 0.040 | 47.3% |
| wp_nh2k | 0.025 | 29.1% |
| wp_nh8k | 0.043 | 50.3% |
| wp_park | 0.041 | 48.8% |
| wp_spin | 0.047 | 55.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 51913ns | 51913ns | base |
| wp_lp0 | 50425ns | 50425ns | -2.87% |
| wp_nh128 | 42920ns | 42920ns | -17.32% |
| wp_nh2k | 56801ns | 56801ns | +9.41% |
| wp_nh8k | 38610ns | 38610ns | -25.63% |
| wp_park | 43002ns | 43002ns | -17.17% |
| wp_spin | 39548ns | 39548ns | -23.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 27388ns | base | --- | [25613, 58974] | --- | --- | --- | --- |
| wp_lp0 | 30508ns | no significant difference | [-5359, +2113]ns | [21289, 52304] | no | 0.5768 | 0.4807 | 0 |
| wp_nh128 | 25511ns | no significant difference | [-12513, +2010]ns | [18350, 43736] | no | 0.3568 | 0.2379 | 0 |
| wp_nh2k | 41437ns | no significant difference | [-5185, +15533]ns | [22830, 60162] | no | 0.8145 | 0.8145 | 0 |
| wp_nh8k | 24029ns | -9432.8ns (-34.4%) | [-15719, -2139]ns | [17751, 42262] | YES (adj: no) | 0.1925 | 0.0963 | 0 |
| wp_park | 24756ns | -5797.7ns (-21.2%) | [-11116, -2997]ns | [18600, 49463] | YES | 0.0226 | 0.0075 | 0 |
| wp_spin | 21936ns | -9711.8ns (-35.5%) | [-11400, -3111]ns | [17948, 32885] | YES | 0.0226 | 0.0075 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 30980ns | +0.1% | -40.7% | -19.4% | +28.8% | -40.5% | -43.7% |
| 1 | 2 | 28815ns | +6.2% | -11.1% | +84.4% | +47.0% | -3.8% | -19.3% |
| 1 | 3 | 27319ns | -17.5% | +6.3% | +115.0% | -2.0% | -20.0% | -0.3% |
| 2 | 1 | 26870ns | +4.9% | -45.5% | +105.3% | -24.8% | -25.0% | -16.4% |
| 2 | 2 | 20744ns | +1.1% | +22.5% | +12.1% | +2.6% | +149.4% | +3.2% |
| 2 | 3 | 21985ns | -21.8% | -16.7% | -19.8% | -9.7% | -27.2% | -13.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | 0.449 | moderate+ |
| wp_lp0 | 0.189 | ok |
| wp_nh128 | -0.493 | moderate- |
| wp_nh2k | 0.279 | moderate+ |
| wp_nh8k | 0.519 | HIGH+ (drift/warm-up) |
| wp_park | -0.462 | moderate- |
| wp_spin | 0.101 | ok |

**Consistency summary:**

- **wp_lp0**: won 2/6, lost 4/6
- **wp_nh128**: won 4/6, lost 2/6
- **wp_nh2k**: won 2/6, lost 4/6
- **wp_nh8k**: won 3/6, lost 3/6
- **wp_park**: won 5/6, lost 1/6
- **wp_spin**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 6019.9ns | 41341.9ns | 14.6% | HIGH |
| wp_lp0 | 5497.4ns | 40176.2ns | 13.7% | HIGH |
| wp_nh128 | 5061.3ns | 34027.5ns | 14.9% | HIGH |
| wp_nh2k | 6155.2ns | 47337.1ns | 13.0% | HIGH |
| wp_nh8k | 4883.3ns | 29841.2ns | 16.4% | HIGH |
| wp_park | 4951.1ns | 33882.6ns | 14.6% | HIGH |
| wp_spin | 5085.0ns | 30615.8ns | 16.6% | HIGH |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 20423.6-73506.1 ns)
  20423.6 |##############################
  23077.7 |####################
  25731.9 |########################################
  28386.0 |####################
  31040.1 |
  33694.2 |
  36348.4 |
  39002.5 |
  41656.6 |
  44310.7 |
  46964.9 |
  49619.0 |
  52273.1 |
  54927.2 |##########
  57581.4 |
  60235.5 |##########
  62889.6 |
  65543.8 |
  68197.9 |##########
  70852.0 |##########
  (1 below, 2 above range)

wp_lp0 (n=18, range 17392.9-72633.7 ns)
  17392.9 |####################
  20154.9 |##############################
  22916.9 |
  25679.0 |##########
  28441.0 |########################################
  31203.1 |
  33965.1 |
  36727.2 |
  39489.2 |
  42251.2 |##########
  45013.3 |
  47775.3 |
  50537.4 |
  53299.4 |
  56061.4 |
  58823.5 |##########
  61585.5 |
  64347.6 |
  67109.6 |
  69871.7 |##########
  (2 below, 3 above range)

wp_nh128 (n=18, range 15467.5-59948.4 ns)
  15467.5 |#############
  17691.5 |########################################
  19915.6 |#############
  22139.6 |#############
  24363.7 |##########################
  26587.7 |#############
  28811.8 |#############
  31035.8 |
  33259.8 |
  35483.9 |
  37707.9 |
  39932.0 |
  42156.0 |#############
  44380.1 |#############
  46604.1 |
  48828.2 |##########################
  51052.2 |
  53276.3 |
  55500.3 |
  57724.4 |
  (2 below, 2 above range)

wp_nh2k (n=18, range 18315.7-80357.6 ns)
  18315.7 |#############
  21417.8 |########################################
  24519.9 |#############
  27622.0 |
  30724.0 |##########################
  33826.1 |
  36928.2 |
  40030.3 |
  43132.4 |
  46234.5 |
  49336.6 |#############
  52438.7 |##########################
  55540.8 |
  58642.9 |##########################
  61745.0 |
  64847.1 |
  67949.2 |#############
  71051.3 |#############
  74153.4 |
  77255.5 |
  (2 below, 2 above range)

wp_nh8k (n=18, range 12676.8-48752.2 ns)
  12676.8 |#############
  14480.5 |########################################
  16284.3 |
  18088.1 |##########################
  19891.8 |##########################
  21695.6 |
  23499.4 |
  25303.2 |#############
  27106.9 |#############
  28910.7 |
  30714.5 |
  32518.2 |
  34322.0 |
  36125.8 |
  37929.5 |
  39733.3 |#############
  41537.1 |##########################
  43340.9 |
  45144.6 |#############
  46948.4 |
  (1 below, 3 above range)

wp_park (n=18, range 16254.1-57773.0 ns)
  16254.1 |##########################
  18330.0 |########################################
  20406.0 |##########################
  22481.9 |
  24557.8 |
  26633.8 |##########################
  28709.7 |
  30785.7 |
  32861.6 |
  34937.6 |
  37013.5 |
  39089.5 |#############
  41165.4 |
  43241.4 |
  45317.3 |
  47393.2 |#############
  49469.2 |#############
  51545.1 |#############
  53621.1 |
  55697.0 |
  (2 below, 3 above range)

wp_spin (n=18, range 12075.9-54707.0 ns)
  12075.9 |
  14207.4 |##########################
  16339.0 |##########################
  18470.5 |#############
  20602.1 |########################################
  22733.7 |#############
  24865.2 |
  26996.8 |#############
  29128.3 |#############
  31259.9 |
  33391.5 |
  35523.0 |#############
  37654.6 |
  39786.1 |
  41917.7 |
  44049.2 |
  46180.8 |
  48312.4 |
  50443.9 |#############
  52575.5 |
  (2 below, 3 above range)

```

## Diagnostics

- **wp_isb8**: CV=58.6% (high variance, measurements may be unstable)
- **wp_isb8**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **wp_isb8**: bridge=19.6% of algo (FFI overhead may distort results)
- **wp_lp0**: CV=62.8% (high variance, measurements may be unstable)
- **wp_lp0**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **wp_lp0**: bridge=16.6% of algo (FFI overhead may distort results)
- **wp_nh128**: CV=63.2% (high variance, measurements may be unstable)
- **wp_nh128**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **wp_nh128**: bridge=16.8% of algo (FFI overhead may distort results)
- **wp_nh2k**: CV=62.9% (high variance, measurements may be unstable)
- **wp_nh2k**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **wp_nh2k**: bridge=12.7% of algo (FFI overhead may distort results)
- **wp_nh8k**: CV=51.0% (high variance, measurements may be unstable)
- **wp_nh8k**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **wp_nh8k**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **wp_nh8k**: bridge=18.9% of algo (FFI overhead may distort results)
- **wp_park**: CV=54.3% (high variance, measurements may be unstable)
- **wp_park**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **wp_park**: bridge=17.6% of algo (FFI overhead may distort results)
- **wp_spin**: CV=66.2% (high variance, measurements may be unstable)
- **wp_spin**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **wp_spin**: bridge=21.5% of algo (FFI overhead may distort results)
