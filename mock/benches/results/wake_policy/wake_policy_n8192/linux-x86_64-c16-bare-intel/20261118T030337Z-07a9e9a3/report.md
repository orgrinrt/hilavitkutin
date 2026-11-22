# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wp_spin dominates: 11% faster than the next best (wp_isb8)

wp_spin (54.15 us) leads wp_isb8 (60.26 us) by 11%, a clear separation rather than a photo finish. CV 42.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wp_lp0 shows alternating (throttle bounce) (autocorr -0.83)

wp_lp0's per-pass series has lag-1 autocorrelation -0.83, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### wp_lp0 is inconsistent: worst-20% is 3.0x its best-20%

wp_lp0's best 20% of batches run at 48.66 us but its worst 20% at 146.23 us (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wp_spin** at 54147.6 ns median (-10.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.52x (fastest 54147.6 ns, slowest 82048.1 ns)

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
| wp_isb8 | 90145ns | 68214ns | 54119ns | 67640ns | 141916ns | base |
| wp_lp0 | 104988ns | 86017ns | 55526ns | 86869ns | 156896ns | +16.46% |
| wp_nh128 | 96023ns | 72917ns | 58574ns | 73444ns | 148616ns | +6.52% |
| wp_nh2k | 94895ns | 78315ns | 56386ns | 72820ns | 147261ns | +5.27% |
| wp_nh8k | 96653ns | 72649ns | 59134ns | 72007ns | 152381ns | +7.22% |
| wp_park | 91670ns | 90398ns | 56592ns | 76154ns | 132482ns | +1.69% |
| wp_spin | 74628ns | 60400ns | 53128ns | 60720ns | 106239ns | -17.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 82201ns | 48326ns | 131615ns | base | 0.100 |
| wp_lp0 | 96166ns | 48661ns | 146231ns | +16.99% | 0.085 |
| wp_nh128 | 87265ns | 51595ns | 137342ns | +6.16% | 0.094 |
| wp_nh2k | 86552ns | 50120ns | 136176ns | +5.29% | 0.095 |
| wp_nh8k | 88419ns | 52128ns | 141833ns | +7.56% | 0.093 |
| wp_park | 83193ns | 49704ns | 121732ns | +1.21% | 0.098 |
| wp_spin | 67493ns | 47909ns | 97466ns | -17.89% | 0.121 |

## Performance model

- Peak throughput: **0.171 Gops/s** (wp_spin; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.136 | 79.5% |
| wp_lp0 | 0.104 | 61.0% |
| wp_nh128 | 0.124 | 72.7% |
| wp_nh2k | 0.115 | 67.3% |
| wp_nh8k | 0.124 | 72.7% |
| wp_park | 0.100 | 58.4% |
| wp_spin | 0.151 | 88.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 90145ns | 90145ns | base |
| wp_lp0 | 104988ns | 104988ns | +16.46% |
| wp_nh128 | 96023ns | 96023ns | +6.52% |
| wp_nh2k | 94895ns | 94895ns | +5.27% |
| wp_nh8k | 96653ns | 96653ns | +7.22% |
| wp_park | 91670ns | 91670ns | +1.69% |
| wp_spin | 74628ns | 74628ns | -17.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 60256ns | base | --- | [54868, 91184] | --- | --- | --- | --- |
| wp_lp0 | 78491ns | no significant difference | [-11722, +41390]ns | [65432, 127952] | no | 0.3568 | 0.2379 | 0 |
| wp_nh128 | 65898ns | +5809.5ns (+9.6%) | [+305, +13559]ns | [58839, 102256] | YES (adj: no) | 0.1925 | 0.0963 | 0 |
| wp_nh2k | 71149ns | no significant difference | [-6812, +21240]ns | [54783, 99217] | no | 1.0000 | 1.0000 | 0 |
| wp_nh8k | 65864ns | no significant difference | [-492, +13796]ns | [58972, 119613] | no | 0.1925 | 0.0963 | 0 |
| wp_park | 82048ns | no significant difference | [-5566, +21332]ns | [54470, 104178] | no | 0.5768 | 0.4807 | 0 |
| wp_spin | 54148ns | -5263.6ns (-8.7%) | [-15621, -1370]ns | [49732, 85427] | YES (adj: no) | 0.1925 | 0.0963 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 79541ns | -18.7% | +4.6% | -32.4% | -27.5% | -35.7% | -34.5% |
| 1 | 2 | 65872ns | +79.9% | +25.6% | -1.6% | +1.9% | +78.7% | +0.3% |
| 1 | 3 | 60841ns | -25.6% | +11.0% | +34.9% | +15.4% | -2.3% | -18.1% |
| 2 | 1 | 59670ns | +126.3% | -1.0% | -11.4% | +0.5% | +36.2% | -17.0% |
| 2 | 2 | 55150ns | +22.4% | +16.5% | +97.6% | +17.2% | +83.6% | -1.2% |
| 2 | 3 | 46215ns | +83.2% | +24.6% | +29.1% | +81.4% | +18.7% | +4.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | 0.297 | moderate+ |
| wp_lp0 | -0.827 | HIGH- (thermal bounce) |
| wp_nh128 | 0.452 | moderate+ |
| wp_nh2k | -0.538 | HIGH- (thermal bounce) |
| wp_nh8k | -0.104 | ok |
| wp_park | -0.619 | HIGH- (thermal bounce) |
| wp_spin | -0.265 | moderate- |

**Consistency summary:**

- **wp_lp0**: won 2/6, lost 4/6
- **wp_nh128**: won 1/6, lost 5/6
- **wp_nh2k**: won 3/6, lost 3/6
- **wp_nh8k**: won 1/6, lost 5/6
- **wp_park**: won 2/6, lost 4/6
- **wp_spin**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 33512.1ns | 82200.8ns | 40.8% | HIGH |
| wp_lp0 | 36496.0ns | 96165.5ns | 38.0% | HIGH |
| wp_nh128 | 37601.7ns | 87265.4ns | 43.1% | HIGH |
| wp_nh2k | 34016.6ns | 86551.9ns | 39.3% | HIGH |
| wp_nh8k | 34665.2ns | 88418.6ns | 39.2% | HIGH |
| wp_park | 35623.8ns | 83192.5ns | 42.8% | HIGH |
| wp_spin | 30154.9ns | 67493.4ns | 44.7% | HIGH |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 48325.5-131615.0 ns)
  48325.5 |
  52490.0 |########################################
  56654.5 |################
  60818.9 |########
  64983.4 |########
  69147.9 |
  73312.4 |
  77476.8 |########
  81641.3 |########
  85805.8 |
  89970.3 |
  94134.7 |########
  98299.2 |
  102463.7 |
  106628.2 |
  110792.7 |
  114957.1 |
  119121.6 |########
  123286.1 |
  127450.6 |
  (2 below, 3 above range)

wp_lp0 (n=18, range 48660.8-146230.9 ns)
  48660.8 |
  53539.3 |#############
  58417.8 |#############
  63296.3 |########################################
  68174.8 |##########################
  73053.3 |
  77931.8 |
  82810.4 |#############
  87688.9 |
  92567.4 |
  97445.9 |
  102324.4 |#############
  107202.9 |
  112081.4 |
  116959.9 |#############
  121838.4 |#############
  126716.9 |#############
  131595.4 |########################################
  136473.9 |
  141352.4 |
  (2 below, 1 above range)

wp_nh128 (n=18, range 51595.2-137342.4 ns)
  51595.2 |################
  55882.5 |########################################
  60169.9 |########
  64457.3 |########
  68744.6 |
  73032.0 |
  77319.3 |
  81606.7 |################
  85894.1 |
  90181.4 |
  94468.8 |
  98756.2 |########
  103043.5 |
  107330.9 |
  111618.3 |
  115905.6 |########
  120193.0 |########
  124480.3 |
  128767.7 |
  133055.1 |
  (1 below, 3 above range)

wp_nh2k (n=18, range 50119.7-136175.6 ns)
  50119.7 |########################################
  54422.5 |##########################
  58725.3 |#############
  63028.1 |#############
  67330.9 |
  71633.7 |
  75936.5 |#############
  80239.3 |#############
  84542.1 |#############
  88844.9 |#############
  93147.7 |
  97450.5 |
  101753.3 |
  106056.0 |#############
  110358.8 |#############
  114661.6 |
  118964.4 |
  123267.2 |
  127570.0 |
  131872.8 |#############
  (2 below, 2 above range)

wp_nh8k (n=18, range 52128.3-141833.2 ns)
  52128.3 |
  56613.5 |########################################
  61098.8 |################
  65584.0 |########
  70069.3 |########
  74554.5 |
  79039.8 |
  83525.0 |########
  88010.3 |
  92495.5 |
  96980.8 |
  101466.0 |
  105951.3 |
  110436.5 |
  114921.8 |
  119407.0 |########
  123892.3 |
  128377.5 |########
  132862.8 |########
  137348.0 |########
  (2 below, 2 above range)

wp_park (n=18, range 49704.1-121732.3 ns)
  49704.1 |##########################
  53305.5 |########################################
  56906.9 |##########################
  60508.3 |
  64109.7 |
  67711.2 |
  71312.6 |
  74914.0 |
  78515.4 |#############
  82116.8 |##########################
  85718.2 |
  89319.6 |#############
  92921.0 |
  96522.5 |
  100123.9 |#############
  103725.3 |
  107326.7 |
  110928.1 |
  114529.5 |#############
  118130.9 |#############
  (1 below, 3 above range)

wp_spin (n=18, range 47909.4-97466.1 ns)
  47909.4 |########################################
  50387.2 |######
  52865.0 |#############
  55342.9 |
  57820.7 |
  60298.6 |######
  62776.4 |
  65254.2 |######
  67732.1 |
  70209.9 |
  72687.7 |
  75165.6 |
  77643.4 |######
  80121.2 |
  82599.1 |
  85076.9 |
  87554.8 |
  90032.6 |######
  92510.4 |######
  94988.3 |
  (1 below, 3 above range)

```

## Diagnostics

- **wp_isb8**: CV=49.0% (high variance, measurements may be unstable)
- **wp_isb8**: bridge=55.4% of algo (FFI overhead may distort results)
- **wp_lp0**: CV=45.3% (high variance, measurements may be unstable)
- **wp_lp0**: worst_20/best_20 = 3.0x (possible bimodal distribution)
- **wp_lp0**: bridge=46.1% of algo (FFI overhead may distort results)
- **wp_nh128**: CV=46.9% (high variance, measurements may be unstable)
- **wp_nh128**: bridge=51.8% of algo (FFI overhead may distort results)
- **wp_nh2k**: CV=51.0% (high variance, measurements may be unstable)
- **wp_nh2k**: bridge=47.1% of algo (FFI overhead may distort results)
- **wp_nh8k**: CV=45.2% (high variance, measurements may be unstable)
- **wp_nh8k**: bridge=51.3% of algo (FFI overhead may distort results)
- **wp_park**: CV=36.8% (high variance, measurements may be unstable)
- **wp_park**: bridge=41.2% of algo (FFI overhead may distort results)
- **wp_spin**: CV=34.0% (high variance, measurements may be unstable)
- **wp_spin**: bridge=58.0% of algo (FFI overhead may distort results)
