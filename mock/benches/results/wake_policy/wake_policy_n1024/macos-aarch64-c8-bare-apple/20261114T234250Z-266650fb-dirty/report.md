# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wp_nh8k is an outlier: 2.0x slower than the field

wp_nh8k (17.71 us) is 2.0x the fastest (8.74 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### wp_park shows alternating (throttle bounce) (autocorr -0.57)

wp_park's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader wp_park vs stability leader wp_isb8 (+8% speed for 1.9x steadier)

wp_park is fastest (8.74 us, CV 9.1%); wp_isb8 gives up 8.1% median for 1.9x lower variance (CV 4.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### wp_nh8k is inconsistent: worst-20% is 4.4x its best-20%

wp_nh8k's best 20% of batches run at 4.68 us but its worst 20% at 20.71 us (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wp_park** at 8743.8 ns median (-7.5% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.03x (fastest 8743.8 ns, slowest 17708.3 ns)

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
| wp_isb8 | 11788ns | 11790ns | 10993ns | 11670ns | 12363ns | base |
| wp_lp0 | 11645ns | 11460ns | 10888ns | 11341ns | 12481ns | -1.21% |
| wp_nh128 | 11549ns | 11331ns | 10907ns | 11262ns | 12300ns | -2.03% |
| wp_nh2k | 14857ns | 14750ns | 13944ns | 14586ns | 15719ns | +26.03% |
| wp_nh8k | 17677ns | 20119ns | 7086ns | 17523ns | 23204ns | +49.96% |
| wp_park | 11400ns | 11162ns | 10700ns | 11094ns | 12207ns | -3.30% |
| wp_spin | 18792ns | 16144ns | 14971ns | 16338ns | 24382ns | +59.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 9338ns | 8636ns | 9835ns | base | 0.110 |
| wp_lp0 | 9135ns | 8488ns | 9853ns | -2.18% | 0.112 |
| wp_nh128 | 9135ns | 8592ns | 9770ns | -2.18% | 0.112 |
| wp_nh2k | 12395ns | 11549ns | 13209ns | +32.74% | 0.083 |
| wp_nh8k | 15200ns | 4679ns | 20713ns | +62.77% | 0.067 |
| wp_park | 8998ns | 8311ns | 9844ns | -3.64% | 0.114 |
| wp_spin | 16277ns | 12482ns | 21889ns | +74.30% | 0.063 |

## Performance model

- Peak throughput: **0.219 Gops/s** (wp_nh8k; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.108 | 49.5% |
| wp_lp0 | 0.113 | 51.8% |
| wp_nh128 | 0.114 | 52.0% |
| wp_nh2k | 0.084 | 38.5% |
| wp_nh8k | 0.058 | 26.4% |
| wp_park | 0.117 | 53.5% |
| wp_spin | 0.075 | 34.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 11788ns | 11788ns | base |
| wp_lp0 | 11645ns | 11645ns | -1.21% |
| wp_nh128 | 11549ns | 11549ns | -2.03% |
| wp_nh2k | 14857ns | 14857ns | +26.03% |
| wp_nh8k | 17677ns | 17677ns | +49.96% |
| wp_park | 11400ns | 11400ns | -3.30% |
| wp_spin | 18792ns | 18792ns | +59.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 9450ns | base | --- | [8952, 9692] | --- | --- | --- | --- |
| wp_lp0 | 9033ns | no significant difference | [-846, +296]ns | [8608, 9488] | no | 0.4807 | 0.4807 | 0 |
| wp_nh128 | 8992ns | no significant difference | [-798, +271]ns | [8775, 9327] | no | 0.4807 | 0.4807 | 0 |
| wp_nh2k | 12169ns | +2835.4ns (+30.0%) | [+2371, +3696]ns | [11944, 12923] | YES | 0.0000 | 0.0000 | 0 |
| wp_nh8k | 17708ns | +8343.8ns (+88.3%) | [+2900, +10067]ns | [12123, 19054] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| wp_park | 8744ns | -510.4ns (-5.4%) | [-1125, -33]ns | [8475, 9194] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| wp_spin | 13573ns | +4137.5ns (+43.8%) | [+3623, +9798]ns | [13129, 19144] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9779ns | -12.6% | -10.8% | +24.4% | -50.1% | -5.2% | +131.8% |
| 1 | 2 | 8979ns | +8.1% | +11.7% | +35.1% | +102.6% | -7.0% | +46.8% |
| 1 | 3 | 9496ns | -8.4% | -3.2% | +26.8% | +106.1% | -4.9% | +38.9% |
| 2 | 1 | 9492ns | -9.3% | -7.1% | +38.7% | +71.9% | -12.2% | +121.8% |
| 2 | 2 | 8608ns | +12.5% | +0.3% | +51.4% | +128.3% | +12.5% | +48.0% |
| 2 | 3 | 9725ns | -7.6% | -7.8% | +31.7% | +68.1% | -8.4% | +38.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | -0.549 | HIGH- (thermal bounce) |
| wp_lp0 | -0.512 | HIGH- (thermal bounce) |
| wp_nh128 | -0.064 | ok |
| wp_nh2k | 0.384 | moderate+ |
| wp_nh8k | -0.074 | ok |
| wp_park | -0.568 | HIGH- (thermal bounce) |
| wp_spin | -0.322 | moderate- |

**Consistency summary:**

- **wp_lp0**: won 4/6, lost 2/6
- **wp_nh128**: won 4/6, lost 2/6
- **wp_nh2k**: won 0/6, lost 6/6
- **wp_nh8k**: won 1/6, lost 5/6
- **wp_park**: won 5/6, lost 1/6
- **wp_spin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 1349.1ns | 9338.2ns | 14.4% | HIGH |
| wp_lp0 | 1362.0ns | 9134.7ns | 14.9% | HIGH |
| wp_nh128 | 1346.5ns | 9134.7ns | 14.7% | HIGH |
| wp_nh2k | 1355.3ns | 12395.4ns | 10.9% | HIGH |
| wp_nh8k | 1412.7ns | 15199.8ns | 9.3% | HIGH |
| wp_park | 1351.8ns | 8998.2ns | 15.0% | HIGH |
| wp_spin | 1361.6ns | 16276.9ns | 8.4% | HIGH |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 8636.1-9835.4 ns)
   8636.1 |
   8696.1 |
   8756.0 |#############
   8816.0 |
   8876.0 |##########################
   8935.9 |#############
   8995.9 |
   9055.9 |#############
   9115.8 |
   9175.8 |
   9235.8 |
   9295.7 |
   9355.7 |##########################
   9415.7 |
   9475.6 |########################################
   9535.6 |
   9595.6 |
   9655.5 |#############
   9715.5 |#############
   9775.5 |##########################
  (2 below, 2 above range)

wp_lp0 (n=18, range 8487.5-9853.5 ns)
   8487.5 |########################################
   8555.8 |##########
   8624.1 |
   8692.4 |##########
   8760.7 |
   8829.0 |
   8897.3 |
   8965.6 |####################
   9033.9 |##########
   9102.2 |##########
   9170.5 |
   9238.8 |####################
   9307.1 |
   9375.4 |
   9443.7 |
   9512.0 |
   9580.3 |
   9648.6 |##############################
   9716.9 |
   9785.2 |
  (1 below, 2 above range)

wp_nh128 (n=18, range 8591.7-9770.1 ns)
   8591.7 |#############
   8650.6 |
   8709.5 |########################################
   8768.4 |##########################
   8827.4 |#############
   8886.3 |
   8945.2 |#############
   9004.1 |########################################
   9063.1 |
   9122.0 |
   9180.9 |#############
   9239.8 |
   9298.7 |
   9357.7 |
   9416.6 |#############
   9475.5 |
   9534.4 |
   9593.4 |
   9652.3 |
   9711.2 |
  (1 below, 4 above range)

wp_nh2k (n=18, range 11548.6-13209.0 ns)
  11548.6 |########################################
  11631.6 |
  11714.6 |
  11797.7 |########################################
  11880.7 |
  11963.7 |####################
  12046.7 |########################################
  12129.7 |########################################
  12212.8 |
  12295.8 |
  12378.8 |####################
  12461.8 |
  12544.9 |####################
  12627.9 |
  12710.9 |
  12793.9 |####################
  12876.9 |
  12960.0 |####################
  13043.0 |
  13126.0 |########################################
  (1 below, 2 above range)

wp_nh8k (n=18, range 4679.2-20712.5 ns)
   4679.2 |########################################
   5480.9 |####################
   6282.5 |
   7084.2 |
   7885.9 |####################
   8687.5 |
   9489.2 |
  10290.9 |
  11092.5 |
  11894.2 |
  12695.9 |
  13497.5 |
  14299.2 |
  15100.9 |
  15902.5 |########################################
  16704.2 |########################################
  17505.9 |########################################
  18307.5 |########################################
  19109.2 |########################################
  19910.9 |
  (1 below, 3 above range)

wp_park (n=18, range 8311.1-9843.8 ns)
   8311.1 |########################################
   8387.8 |#############
   8464.4 |#############
   8541.0 |#############
   8617.7 |#############
   8694.3 |##########################
   8770.9 |
   8847.5 |#############
   8924.2 |
   9000.8 |#############
   9077.4 |#############
   9154.1 |
   9230.7 |#############
   9307.3 |#############
   9384.0 |
   9460.6 |
   9537.2 |
   9613.9 |#############
   9690.5 |
   9767.1 |
  (1 below, 2 above range)

wp_spin (n=18, range 12481.9-21888.9 ns)
  12481.9 |##############################
  12952.3 |########################################
  13422.6 |####################
  13893.0 |
  14363.3 |##########
  14833.7 |
  15304.0 |
  15774.4 |
  16244.7 |
  16715.1 |##########
  17185.4 |##########
  17655.8 |
  18126.1 |
  18596.5 |
  19066.8 |
  19537.2 |
  20007.5 |
  20477.9 |##########
  20948.2 |##########
  21418.6 |
  (1 below, 3 above range)

```

## Diagnostics

- **wp_isb8**: bridge=14.3% of algo (FFI overhead may distort results)
- **wp_lp0**: bridge=14.9% of algo (FFI overhead may distort results)
- **wp_nh128**: bridge=15.0% of algo (FFI overhead may distort results)
- **wp_nh2k**: bridge=11.1% of algo (FFI overhead may distort results)
- **wp_nh8k**: CV=41.4% (high variance, measurements may be unstable)
- **wp_nh8k**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **wp_park**: bridge=15.5% of algo (FFI overhead may distort results)
- **wp_spin**: CV=26.7% (high variance, measurements may be unstable)
- **wp_spin**: bridge=10.0% of algo (FFI overhead may distort results)
