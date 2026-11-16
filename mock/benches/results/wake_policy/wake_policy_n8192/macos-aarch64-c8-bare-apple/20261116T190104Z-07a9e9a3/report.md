# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wp_lp0 shows alternating (throttle bounce) (autocorr -0.66)

wp_lp0's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {wp_spin, wp_nh2k, wp_nh128, wp_park, wp_isb8, wp_lp0} vs {wp_nh8k} (49% apart)

The field splits into a fast tier {wp_spin, wp_nh2k, wp_nh128, wp_park, wp_isb8, wp_lp0} and a slow tier {wp_nh8k} with a 49% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: wp_spin** at 33070.8 ns median (-4.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.56x (fastest 33070.8 ns, slowest 51650.0 ns)

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
| wp_isb8 | 37011ns | 36846ns | 34158ns | 36407ns | 39344ns | base |
| wp_lp0 | 36872ns | 36954ns | 34406ns | 36448ns | 38742ns | -0.37% |
| wp_nh128 | 36521ns | 36662ns | 33769ns | 36318ns | 38202ns | -1.32% |
| wp_nh2k | 36377ns | 36523ns | 32938ns | 36062ns | 38571ns | -1.71% |
| wp_nh8k | 54198ns | 54412ns | 52518ns | 54120ns | 55154ns | +46.44% |
| wp_park | 37347ns | 36829ns | 33628ns | 36128ns | 41037ns | +0.91% |
| wp_spin | 35837ns | 35704ns | 32419ns | 35062ns | 38709ns | -3.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 34578ns | 31914ns | 36792ns | base | 0.237 |
| wp_lp0 | 34433ns | 31978ns | 36133ns | -0.42% | 0.238 |
| wp_nh128 | 34108ns | 31324ns | 35733ns | -1.36% | 0.240 |
| wp_nh2k | 33929ns | 30671ns | 36131ns | -1.88% | 0.241 |
| wp_nh8k | 51497ns | 49951ns | 52310ns | +48.93% | 0.159 |
| wp_park | 34813ns | 31218ns | 38239ns | +0.68% | 0.235 |
| wp_spin | 33391ns | 30147ns | 36193ns | -3.43% | 0.245 |

## Performance model

- Peak throughput: **0.272 Gops/s** (wp_spin; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.237 | 87.2% |
| wp_lp0 | 0.237 | 87.1% |
| wp_nh128 | 0.239 | 87.8% |
| wp_nh2k | 0.241 | 88.5% |
| wp_nh8k | 0.159 | 58.4% |
| wp_park | 0.238 | 87.6% |
| wp_spin | 0.248 | 91.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 37011ns | 37011ns | base |
| wp_lp0 | 36872ns | 36872ns | -0.37% |
| wp_nh128 | 36521ns | 36521ns | -1.32% |
| wp_nh2k | 36377ns | 36377ns | -1.71% |
| wp_nh8k | 54198ns | 54198ns | +46.44% |
| wp_park | 37347ns | 37347ns | +0.91% |
| wp_spin | 35837ns | 35837ns | -3.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 34565ns | base | --- | [33196, 36117] | --- | --- | --- | --- |
| wp_lp0 | 34602ns | no significant difference | [-2106, +2021]ns | [33594, 34881] | no | 0.7210 | 0.4807 | 0 |
| wp_nh128 | 34342ns | no significant difference | [-1812, +1067]ns | [33279, 34898] | no | 0.7210 | 0.4807 | 0 |
| wp_nh2k | 34056ns | no significant difference | [-3296, +1454]ns | [32821, 35556] | no | 1.0000 | 1.0000 | 0 |
| wp_nh8k | 51650ns | +16785.4ns (+48.6%) | [+16081, +17856]ns | [51296, 52129] | YES | 0.0000 | 0.0000 | 0 |
| wp_park | 34404ns | no significant difference | [-3412, +2975]ns | [32640, 36131] | no | 1.0000 | 1.0000 | 0 |
| wp_spin | 33071ns | -1179.1ns (-3.4%) | [-2173, -50]ns | [31667, 34140] | YES (adj: no) | 0.2888 | 0.0963 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 33196ns | +4.1% | +2.4% | +0.8% | +57.3% | -2.7% | +4.7% |
| 1 | 2 | 34725ns | -5.1% | -0.8% | -2.0% | +48.1% | -1.0% | -5.8% |
| 1 | 3 | 33196ns | +8.1% | +0.1% | +0.6% | +55.5% | +6.2% | -2.1% |
| 2 | 1 | 33854ns | -6.0% | -0.1% | +5.3% | +54.5% | +14.6% | -3.3% |
| 2 | 2 | 37088ns | -8.2% | -6.9% | -3.5% | +38.9% | -12.7% | -8.2% |
| 2 | 3 | 36021ns | -3.6% | -1.1% | -10.2% | +44.3% | -5.7% | -13.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | 0.184 | ok |
| wp_lp0 | -0.658 | HIGH- (thermal bounce) |
| wp_nh128 | 0.142 | ok |
| wp_nh2k | -0.149 | ok |
| wp_nh8k | -0.520 | HIGH- (thermal bounce) |
| wp_park | -0.161 | ok |
| wp_spin | -0.312 | moderate- |

**Consistency summary:**

- **wp_lp0**: won 4/6, lost 2/6
- **wp_nh128**: won 3/6, lost 2/6
- **wp_nh2k**: won 3/6, lost 3/6
- **wp_nh8k**: won 0/6, lost 6/6
- **wp_park**: won 4/6, lost 2/6
- **wp_spin**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 11082.2ns | 34578.0ns | 32.0% | HIGH |
| wp_lp0 | 11004.6ns | 34432.9ns | 32.0% | HIGH |
| wp_nh128 | 11015.3ns | 34108.1ns | 32.3% | HIGH |
| wp_nh2k | 11059.7ns | 33928.9ns | 32.6% | HIGH |
| wp_nh8k | 10632.6ns | 51496.8ns | 20.6% | HIGH |
| wp_park | 11224.5ns | 34812.7ns | 32.2% | HIGH |
| wp_spin | 11055.6ns | 33390.7ns | 33.1% | HIGH |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 31913.9-36791.7 ns)
  31913.9 |####################
  32157.8 |########################################
  32401.7 |
  32645.6 |
  32889.5 |
  33133.3 |########################################
  33377.2 |
  33621.1 |########################################
  33865.0 |
  34108.9 |
  34352.8 |####################
  34596.7 |########################################
  34840.6 |
  35084.4 |
  35328.3 |
  35572.2 |####################
  35816.1 |####################
  36060.0 |####################
  36303.9 |####################
  36547.8 |
  (1 below, 3 above range)

wp_lp0 (n=18, range 31977.8-36133.3 ns)
  31977.8 |
  32185.6 |
  32393.4 |
  32601.1 |##########
  32808.9 |##########
  33016.7 |
  33224.5 |
  33432.2 |##########
  33640.0 |##########
  33847.8 |##########
  34055.6 |##########
  34263.3 |
  34471.1 |####################
  34678.9 |########################################
  34886.7 |
  35094.5 |
  35302.2 |
  35510.0 |
  35717.8 |##########
  35925.6 |
  (2 below, 3 above range)

wp_nh128 (n=18, range 31323.6-35732.6 ns)
  31323.6 |
  31544.1 |
  31764.5 |
  31985.0 |########################################
  32205.4 |
  32425.9 |
  32646.3 |
  32866.8 |
  33087.2 |########################################
  33307.7 |####################
  33528.1 |
  33748.6 |####################
  33969.0 |####################
  34189.5 |####################
  34409.9 |########################################
  34630.4 |####################
  34850.8 |########################################
  35071.3 |
  35291.7 |
  35512.2 |########################################
  (1 below, 2 above range)

wp_nh2k (n=18, range 30670.8-36131.2 ns)
  30670.8 |
  30943.9 |
  31216.9 |
  31489.9 |##########################
  31762.9 |
  32035.9 |
  32309.0 |#############
  32582.0 |
  32855.0 |
  33128.0 |##########################
  33401.0 |#############
  33674.1 |
  33947.1 |########################################
  34220.1 |
  34493.1 |
  34766.1 |
  35039.2 |
  35312.2 |##########################
  35585.2 |##########################
  35858.2 |#############
  (2 below, 2 above range)

wp_nh8k (n=18, range 49951.4-52310.4 ns)
  49951.4 |####################
  50069.4 |
  50187.3 |
  50305.3 |########################################
  50423.2 |
  50541.2 |
  50659.1 |
  50777.1 |
  50895.0 |
  51013.0 |
  51130.9 |####################
  51248.9 |
  51366.8 |####################
  51484.8 |########################################
  51602.7 |########################################
  51720.7 |
  51838.6 |####################
  51956.6 |####################
  52074.5 |########################################
  52192.5 |########################################
  (1 below, 2 above range)

wp_park (n=18, range 31218.0-38238.9 ns)
  31218.0 |
  31569.1 |#############
  31920.1 |
  32271.2 |##########################
  32622.2 |#############
  32973.2 |
  33324.3 |#############
  33675.3 |#############
  34026.4 |
  34377.4 |########################################
  34728.5 |
  35079.5 |#############
  35430.6 |#############
  35781.6 |
  36132.6 |
  36483.7 |#############
  36834.7 |
  37185.8 |#############
  37536.8 |
  37887.9 |#############
  (2 below, 2 above range)

wp_spin (n=18, range 30147.2-36193.1 ns)
  30147.2 |########################################
  30449.5 |
  30751.8 |
  31054.1 |####################
  31356.4 |####################
  31658.7 |####################
  31961.0 |
  32263.3 |####################
  32565.6 |########################################
  32867.9 |
  33170.2 |####################
  33472.4 |####################
  33774.7 |########################################
  34077.0 |####################
  34379.3 |
  34681.6 |####################
  34983.9 |
  35286.2 |
  35588.5 |
  35890.8 |
  (1 below, 3 above range)

```

## Diagnostics

- **wp_isb8**: bridge=31.8% of algo (FFI overhead may distort results)
- **wp_lp0**: bridge=31.7% of algo (FFI overhead may distort results)
- **wp_nh128**: bridge=31.9% of algo (FFI overhead may distort results)
- **wp_nh2k**: bridge=32.4% of algo (FFI overhead may distort results)
- **wp_nh8k**: bridge=20.6% of algo (FFI overhead may distort results)
- **wp_park**: bridge=31.9% of algo (FFI overhead may distort results)
- **wp_spin**: bridge=33.4% of algo (FFI overhead may distort results)
