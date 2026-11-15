# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wp_spin shows alternating (throttle bounce) (autocorr -0.61)

wp_spin's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (22.09 us) is smaller than the fastest variant's own run-to-run std-dev (31.56 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader wp_spin vs stability leader wp_isb8 (+5% speed for 1.4x steadier)

wp_spin is fastest (56.73 us, CV 55.6%); wp_isb8 gives up 5.1% median for 1.4x lower variance (CV 41.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### wp_nh8k is inconsistent: worst-20% is 2.7x its best-20%

wp_nh8k's best 20% of batches run at 49.79 us but its worst 20% at 131.96 us (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wp_spin** at 56732.8 ns median (-4.9% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.39x (fastest 56732.8 ns, slowest 78817.9 ns)

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
| wp_isb8 | 75938ns | 67424ns | 51847ns | 63566ns | 106543ns | base |
| wp_lp0 | 98026ns | 79425ns | 59371ns | 79743ns | 144780ns | +29.09% |
| wp_nh128 | 89053ns | 75221ns | 59515ns | 70630ns | 131457ns | +17.27% |
| wp_nh2k | 94259ns | 88601ns | 54899ns | 77564ns | 138982ns | +24.13% |
| wp_nh8k | 93144ns | 76857ns | 56666ns | 71706ns | 143540ns | +22.66% |
| wp_park | 87838ns | 72008ns | 53228ns | 70323ns | 131414ns | +15.67% |
| wp_spin | 80209ns | 63470ns | 52881ns | 61629ns | 121742ns | +5.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 68342ns | 45625ns | 97645ns | base | 0.120 |
| wp_lp0 | 88767ns | 52895ns | 132643ns | +29.89% | 0.092 |
| wp_nh128 | 80327ns | 51700ns | 121151ns | +17.54% | 0.102 |
| wp_nh2k | 85618ns | 48783ns | 127964ns | +25.28% | 0.096 |
| wp_nh8k | 84338ns | 49788ns | 131964ns | +23.41% | 0.097 |
| wp_park | 80154ns | 46637ns | 121850ns | +17.28% | 0.102 |
| wp_spin | 71972ns | 46490ns | 110902ns | +5.31% | 0.114 |

## Performance model

- Peak throughput: **0.180 Gops/s** (wp_isb8; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.137 | 76.5% |
| wp_lp0 | 0.117 | 65.0% |
| wp_nh128 | 0.121 | 67.2% |
| wp_nh2k | 0.104 | 57.9% |
| wp_nh8k | 0.117 | 65.0% |
| wp_park | 0.126 | 70.2% |
| wp_spin | 0.144 | 80.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 75938ns | 75938ns | base |
| wp_lp0 | 98026ns | 98026ns | +29.09% |
| wp_nh128 | 89053ns | 89053ns | +17.27% |
| wp_nh2k | 94259ns | 94259ns | +24.13% |
| wp_nh8k | 93144ns | 93144ns | +22.66% |
| wp_park | 87838ns | 87838ns | +15.67% |
| wp_spin | 80209ns | 80209ns | +5.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 59646ns | base | --- | [49383, 77180] | --- | --- | --- | --- |
| wp_lp0 | 70139ns | +11406.9ns (+19.1%) | [+6822, +51947]ns | [61163, 107660] | YES (adj: no) | 0.0927 | 0.0309 | 0 |
| wp_nh128 | 67897ns | no significant difference | [-147, +30091]ns | [54850, 91473] | no | 0.1155 | 0.0963 | 0 |
| wp_nh2k | 78818ns | +4422.1ns (+7.4%) | [+1590, +26615]ns | [52988, 111697] | YES | 0.0452 | 0.0075 | 0 |
| wp_nh8k | 70172ns | no significant difference | [-664, +19553]ns | [52764, 104071] | no | 0.1155 | 0.0963 | 0 |
| wp_park | 65031ns | no significant difference | [-7156, +23500]ns | [53637, 95807] | no | 0.4807 | 0.4807 | 0 |
| wp_spin | 56733ns | no significant difference | [-441, +6559]ns | [50289, 84818] | no | 0.1155 | 0.0963 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 50994ns | +16.2% | +17.9% | +86.3% | +29.0% | +11.5% | +15.9% |
| 1 | 2 | 45944ns | +141.4% | +13.8% | +9.2% | +12.6% | +13.7% | +11.0% |
| 1 | 3 | 52277ns | +23.0% | +106.3% | +5.6% | +42.6% | +57.5% | +4.9% |
| 2 | 1 | 49435ns | +47.1% | +24.4% | +2.4% | +8.8% | +113.4% | +4.0% |
| 2 | 2 | 71508ns | -5.5% | +5.0% | +86.2% | +77.7% | -11.3% | -9.5% |
| 2 | 3 | 67352ns | -21.4% | -21.4% | +13.7% | -19.0% | -24.5% | -26.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | 0.340 | moderate+ |
| wp_lp0 | -0.330 | moderate- |
| wp_nh128 | -0.416 | moderate- |
| wp_nh2k | -0.155 | ok |
| wp_nh8k | -0.466 | moderate- |
| wp_park | 0.164 | ok |
| wp_spin | -0.608 | HIGH- (thermal bounce) |

**Consistency summary:**

- **wp_lp0**: won 2/6, lost 4/6
- **wp_nh128**: won 1/6, lost 5/6
- **wp_nh2k**: won 0/6, lost 6/6
- **wp_nh8k**: won 1/6, lost 5/6
- **wp_park**: won 2/6, lost 4/6
- **wp_spin**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 33322.1ns | 68342.0ns | 48.8% | HIGH |
| wp_lp0 | 38041.0ns | 88767.1ns | 42.9% | HIGH |
| wp_nh128 | 38189.8ns | 80326.9ns | 47.5% | HIGH |
| wp_nh2k | 35391.4ns | 85617.7ns | 41.3% | HIGH |
| wp_nh8k | 37522.3ns | 84338.1ns | 44.5% | HIGH |
| wp_park | 32468.7ns | 80153.9ns | 40.5% | HIGH |
| wp_spin | 34359.0ns | 71971.8ns | 47.7% | HIGH |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 45625.0-97645.3 ns)
  45625.0 |########################################
  48226.0 |##########################
  50827.0 |##########################
  53428.1 |
  56029.1 |
  58630.1 |##########################
  61231.1 |
  63832.1 |
  66433.1 |#############
  69034.2 |#############
  71635.2 |#############
  74236.2 |
  76837.2 |
  79438.2 |
  82039.2 |#############
  84640.3 |
  87241.3 |
  89842.3 |
  92443.3 |#############
  95044.3 |
  (1 below, 3 above range)

wp_lp0 (n=18, range 52894.8-132642.6 ns)
  52894.8 |#############
  56882.2 |########################################
  60869.6 |##########################
  64856.9 |##########################
  68844.3 |#############
  72831.7 |
  76819.1 |
  80806.5 |
  84793.9 |
  88781.3 |
  92768.7 |#############
  96756.1 |#############
  100743.5 |#############
  104730.9 |
  108718.3 |#############
  112705.7 |
  116693.0 |
  120680.4 |
  124667.8 |#############
  128655.2 |#############
  (1 below, 2 above range)

wp_nh128 (n=18, range 51699.6-121150.8 ns)
  51699.6 |########################################
  55172.2 |#############
  58644.8 |##########################
  62117.3 |
  65589.9 |#############
  69062.4 |##########################
  72535.0 |##########################
  76007.6 |
  79480.1 |
  82952.7 |
  86425.2 |
  89897.8 |
  93370.4 |
  96842.9 |
  100315.5 |
  103788.0 |
  107260.6 |#############
  110733.2 |#############
  114205.7 |
  117678.3 |
  (2 below, 3 above range)

wp_nh2k (n=18, range 48782.6-127964.0 ns)
  48782.6 |########################################
  52741.7 |##########
  56700.8 |##########
  60659.8 |##########
  64618.9 |
  68578.0 |
  72537.1 |
  76496.1 |##########
  80455.2 |##########
  84414.3 |
  88373.3 |
  92332.4 |##########
  96291.5 |##########
  100250.5 |
  104209.6 |##########
  108168.7 |
  112127.8 |
  116086.8 |##########
  120045.9 |##########
  124005.0 |
  (1 below, 3 above range)

wp_nh8k (n=18, range 49788.1-131964.4 ns)
  49788.1 |########################################
  53896.9 |########
  58005.8 |########
  62114.6 |########
  66223.4 |
  70332.2 |
  74441.0 |################
  78549.8 |
  82658.7 |
  86767.5 |########
  90876.3 |
  94985.1 |
  99093.9 |########
  103202.7 |
  107311.6 |
  111420.4 |########
  115529.2 |################
  119638.0 |
  123746.8 |########
  127855.6 |
  (1 below, 1 above range)

wp_park (n=18, range 46637.2-121849.5 ns)
  46637.2 |########################################
  50397.8 |########################################
  54158.5 |########################################
  57919.1 |####################
  61679.7 |####################
  65440.3 |####################
  69200.9 |
  72961.5 |
  76722.1 |
  80482.8 |####################
  84243.4 |####################
  88004.0 |
  91764.6 |####################
  95525.2 |
  99285.8 |
  103046.4 |########################################
  106807.1 |
  110567.7 |####################
  114328.3 |
  118088.9 |
  (1 below, 2 above range)

wp_spin (n=18, range 46490.3-110901.9 ns)
  46490.3 |########################################
  49710.9 |########################################
  52931.5 |#############
  56152.1 |##########################
  59372.7 |
  62593.2 |#############
  65813.8 |
  69034.4 |
  72255.0 |
  75475.6 |
  78696.1 |#############
  81916.7 |
  85137.3 |
  88357.9 |##########################
  91578.5 |
  94799.0 |
  98019.6 |
  101240.2 |
  104460.8 |
  107681.4 |
  (2 below, 3 above range)

```

## Diagnostics

- **wp_isb8**: CV=35.9% (high variance, measurements may be unstable)
- **wp_isb8**: bridge=56.0% of algo (FFI overhead may distort results)
- **wp_lp0**: CV=40.6% (high variance, measurements may be unstable)
- **wp_lp0**: bridge=56.4% of algo (FFI overhead may distort results)
- **wp_nh128**: CV=42.4% (high variance, measurements may be unstable)
- **wp_nh128**: bridge=56.3% of algo (FFI overhead may distort results)
- **wp_nh2k**: CV=39.9% (high variance, measurements may be unstable)
- **wp_nh2k**: bridge=43.5% of algo (FFI overhead may distort results)
- **wp_nh8k**: CV=48.8% (high variance, measurements may be unstable)
- **wp_nh8k**: bridge=47.8% of algo (FFI overhead may distort results)
- **wp_park**: CV=44.8% (high variance, measurements may be unstable)
- **wp_park**: bridge=50.6% of algo (FFI overhead may distort results)
- **wp_spin**: CV=43.9% (high variance, measurements may be unstable)
- **wp_spin**: bridge=58.4% of algo (FFI overhead may distort results)
