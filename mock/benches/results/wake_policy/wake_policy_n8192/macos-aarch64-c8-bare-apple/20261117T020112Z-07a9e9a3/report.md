# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wp_spin shows alternating (throttle bounce) (autocorr -0.72)

wp_spin's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {wp_spin, wp_park, wp_isb8, wp_nh128, wp_lp0, wp_nh2k} vs {wp_nh8k} (41% apart)

The field splits into a fast tier {wp_spin, wp_park, wp_isb8, wp_nh128, wp_lp0, wp_nh2k} and a slow tier {wp_nh8k} with a 41% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: wp_spin** at 31575.0 ns median (-11.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.73x (fastest 31575.0 ns, slowest 54704.2 ns)

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
| wp_isb8 | 38602ns | 38023ns | 35842ns | 37598ns | 41488ns | base |
| wp_lp0 | 39233ns | 38562ns | 35333ns | 37941ns | 43120ns | +1.63% |
| wp_nh128 | 39453ns | 38081ns | 35228ns | 37552ns | 44418ns | +2.21% |
| wp_nh2k | 40111ns | 41512ns | 34829ns | 39402ns | 43815ns | +3.91% |
| wp_nh8k | 55547ns | 57408ns | 46917ns | 56701ns | 58132ns | +43.90% |
| wp_park | 38216ns | 37012ns | 34490ns | 36748ns | 42281ns | -1.00% |
| wp_spin | 35297ns | 33821ns | 32414ns | 33834ns | 38933ns | -8.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 36041ns | 33261ns | 38799ns | base | 0.227 |
| wp_lp0 | 36734ns | 33056ns | 40393ns | +1.92% | 0.223 |
| wp_nh128 | 36913ns | 32767ns | 41671ns | +2.42% | 0.222 |
| wp_nh2k | 37454ns | 32228ns | 41020ns | +3.92% | 0.219 |
| wp_nh8k | 52784ns | 44097ns | 55245ns | +46.46% | 0.155 |
| wp_park | 35719ns | 32171ns | 39628ns | -0.89% | 0.229 |
| wp_spin | 32892ns | 30115ns | 36344ns | -8.74% | 0.249 |

## Performance model

- Peak throughput: **0.272 Gops/s** (wp_spin; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.229 | 84.4% |
| wp_lp0 | 0.227 | 83.5% |
| wp_nh128 | 0.229 | 84.3% |
| wp_nh2k | 0.210 | 77.4% |
| wp_nh8k | 0.150 | 55.1% |
| wp_park | 0.237 | 87.2% |
| wp_spin | 0.259 | 95.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 38602ns | 38602ns | base |
| wp_lp0 | 39233ns | 39233ns | +1.63% |
| wp_nh128 | 39453ns | 39453ns | +2.21% |
| wp_nh2k | 40111ns | 40111ns | +3.91% |
| wp_nh8k | 55547ns | 55547ns | +43.90% |
| wp_park | 38216ns | 38216ns | -1.00% |
| wp_spin | 35297ns | 35297ns | -8.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 35696ns | base | --- | [34283, 37083] | --- | --- | --- | --- |
| wp_lp0 | 36083ns | no significant difference | [-1917, +2662]ns | [34769, 36760] | no | 0.4807 | 0.4807 | 0 |
| wp_nh128 | 35715ns | no significant difference | [-1298, +3398]ns | [34248, 38617] | no | 0.4807 | 0.4807 | 0 |
| wp_nh2k | 38925ns | no significant difference | [-1044, +3523]ns | [34219, 40156] | no | 0.4758 | 0.2379 | 0 |
| wp_nh8k | 54704ns | +17606.2ns (+49.3%) | [+15023, +20146]ns | [53444, 54969] | YES | 0.0000 | 0.0000 | 0 |
| wp_park | 34527ns | no significant difference | [-3625, +1004]ns | [33854, 36008] | no | 0.4807 | 0.4807 | 0 |
| wp_spin | 31575ns | -3387.5ns (-9.5%) | [-5788, -794]ns | [30625, 34094] | YES (adj: no) | 0.0927 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 35958ns | -7.8% | +10.1% | +9.8% | +52.5% | -1.1% | -6.2% |
| 1 | 2 | 34004ns | +6.8% | +2.2% | -4.4% | +62.3% | -4.6% | -4.1% |
| 1 | 3 | 35850ns | +16.8% | -7.4% | +7.7% | +52.9% | -3.1% | -15.4% |
| 2 | 1 | 35542ns | +1.8% | +1.0% | +10.9% | +22.0% | -3.3% | +4.7% |
| 2 | 2 | 39883ns | -9.8% | -12.9% | +0.6% | +37.7% | -13.8% | -23.9% |
| 2 | 3 | 34867ns | +0.9% | +17.8% | +20.0% | +56.6% | +15.6% | -2.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | -0.279 | moderate- |
| wp_lp0 | -0.022 | ok |
| wp_nh128 | -0.089 | ok |
| wp_nh2k | 0.018 | ok |
| wp_nh8k | -0.226 | moderate- |
| wp_park | -0.066 | ok |
| wp_spin | -0.722 | HIGH- (thermal bounce) |

**Consistency summary:**

- **wp_lp0**: won 2/6, lost 4/6
- **wp_nh128**: won 2/6, lost 4/6
- **wp_nh2k**: won 1/6, lost 5/6
- **wp_nh8k**: won 0/6, lost 6/6
- **wp_park**: won 5/6, lost 1/6
- **wp_spin**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 11077.5ns | 36040.5ns | 30.7% | HIGH |
| wp_lp0 | 10994.2ns | 36733.8ns | 29.9% | HIGH |
| wp_nh128 | 11256.0ns | 36913.0ns | 30.5% | HIGH |
| wp_nh2k | 10956.0ns | 37454.4ns | 29.3% | HIGH |
| wp_nh8k | 10689.4ns | 52783.6ns | 20.3% | HIGH |
| wp_park | 11129.6ns | 35718.8ns | 31.2% | HIGH |
| wp_spin | 11048.4ns | 32891.7ns | 33.6% | HIGH |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 33261.1-38798.6 ns)
  33261.1 |
  33538.0 |
  33814.9 |##########################
  34091.7 |##########################
  34368.6 |
  34645.5 |#############
  34922.4 |
  35199.2 |#############
  35476.1 |#############
  35753.0 |########################################
  36029.9 |#############
  36306.7 |
  36583.6 |
  36860.5 |
  37137.4 |
  37414.2 |
  37691.1 |#############
  37968.0 |
  38244.9 |#############
  38521.7 |
  (2 below, 3 above range)

wp_lp0 (n=18, range 33055.6-40393.0 ns)
  33055.6 |#############
  33422.4 |
  33789.3 |##########################
  34156.2 |#############
  34523.1 |
  34889.9 |##########################
  35256.8 |
  35623.7 |##########################
  35990.6 |########################################
  36357.4 |#############
  36724.3 |#############
  37091.2 |
  37458.1 |
  37824.9 |
  38191.8 |
  38558.7 |
  38925.6 |
  39292.4 |
  39659.3 |#############
  40026.2 |
  (1 below, 3 above range)

wp_nh128 (n=18, range 32766.6-41670.9 ns)
  32766.6 |########################################
  33211.8 |####################
  33657.1 |####################
  34102.3 |
  34547.5 |########################################
  34992.7 |####################
  35437.9 |####################
  35883.1 |########################################
  36328.3 |
  36773.5 |####################
  37218.7 |
  37664.0 |
  38109.2 |
  38554.4 |####################
  38999.6 |
  39444.8 |####################
  39890.0 |####################
  40335.2 |
  40780.4 |####################
  41225.6 |
  (1 below, 2 above range)

wp_nh2k (n=18, range 32227.8-41020.2 ns)
  32227.8 |########################################
  32667.4 |
  33107.0 |####################
  33546.6 |
  33986.2 |########################################
  34425.9 |
  34865.5 |
  35305.1 |####################
  35744.7 |
  36184.3 |
  36624.0 |####################
  37063.6 |
  37503.2 |
  37942.8 |
  38382.4 |####################
  38822.1 |####################
  39261.7 |########################################
  39701.3 |####################
  40140.9 |####################
  40580.5 |########################################
  (1 below, 2 above range)

wp_nh8k (n=18, range 44097.2-55245.1 ns)
  44097.2 |
  44654.6 |
  45212.0 |
  45769.4 |
  46326.8 |######
  46884.2 |
  47441.6 |
  47999.0 |
  48556.4 |
  49113.8 |
  49671.2 |
  50228.6 |
  50786.0 |######
  51343.4 |
  51900.8 |
  52458.2 |
  53015.5 |######
  53572.9 |######
  54130.3 |####################
  54687.7 |########################################
  (2 below, 3 above range)

wp_park (n=18, range 32170.8-39627.8 ns)
  32170.8 |####################
  32543.7 |
  32916.5 |##########
  33289.4 |##########
  33662.2 |
  34035.1 |########################################
  34407.9 |####################
  34780.8 |
  35153.6 |##########
  35526.5 |##########
  35899.3 |
  36272.1 |##########
  36645.0 |
  37017.8 |##########
  37390.7 |
  37763.5 |
  38136.4 |
  38509.2 |
  38882.1 |
  39254.9 |##########
  (1 below, 2 above range)

wp_spin (n=18, range 30115.3-36343.8 ns)
  30115.3 |########################################
  30426.7 |
  30738.1 |#############
  31049.5 |#############
  31361.0 |########################################
  31672.4 |
  31983.8 |
  32295.2 |#############
  32606.7 |
  32918.1 |
  33229.5 |
  33540.9 |#############
  33852.4 |#############
  34163.8 |#############
  34475.2 |
  34786.6 |#############
  35098.1 |
  35409.5 |
  35720.9 |
  36032.3 |
  (2 below, 3 above range)

```

## Diagnostics

- **wp_isb8**: bridge=30.8% of algo (FFI overhead may distort results)
- **wp_lp0**: bridge=30.4% of algo (FFI overhead may distort results)
- **wp_nh128**: bridge=30.8% of algo (FFI overhead may distort results)
- **wp_nh2k**: bridge=28.1% of algo (FFI overhead may distort results)
- **wp_nh8k**: bridge=19.4% of algo (FFI overhead may distort results)
- **wp_park**: bridge=31.8% of algo (FFI overhead may distort results)
- **wp_spin**: bridge=35.0% of algo (FFI overhead may distort results)
