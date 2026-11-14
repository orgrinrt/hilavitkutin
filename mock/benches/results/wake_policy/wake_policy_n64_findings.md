# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wp_nh8k is an outlier: 2.1x slower than the field

wp_nh8k (10.89 us) is 2.1x the fastest (5.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### wp_lp0 shows alternating (throttle bounce) (autocorr -0.55)

wp_lp0's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {wp_lp0, wp_park, wp_isb8, wp_nh128} vs {wp_nh2k, wp_spin, wp_nh8k} (57% apart)

The field splits into a fast tier {wp_lp0, wp_park, wp_isb8, wp_nh128} and a slow tier {wp_nh2k, wp_spin, wp_nh8k} with a 57% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### wp_nh8k is inconsistent: worst-20% is 23.0x its best-20%

wp_nh8k's best 20% of batches run at 717 ns but its worst 20% at 16.49 us (23.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wp_lp0** at 5068.8 ns median (-8.6% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.15x (fastest 5068.8 ns, slowest 10889.5 ns)

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
| wp_isb8 | 8737ns | 8081ns | 7447ns | 7980ns | 10517ns | base |
| wp_lp0 | 7837ns | 7471ns | 6788ns | 7440ns | 8957ns | -10.30% |
| wp_nh128 | 9396ns | 8721ns | 7561ns | 8470ns | 11703ns | +7.55% |
| wp_nh2k | 12312ns | 12100ns | 11076ns | 11944ns | 13480ns | +40.92% |
| wp_nh8k | 12517ns | 13469ns | 3019ns | 11317ns | 19066ns | +43.27% |
| wp_park | 8772ns | 7702ns | 7072ns | 7568ns | 11429ns | +0.41% |
| wp_spin | 13305ns | 13265ns | 12051ns | 13061ns | 14299ns | +52.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 6299ns | 5121ns | 8109ns | base | 0.010 |
| wp_lp0 | 5373ns | 4522ns | 6373ns | -14.70% | 0.012 |
| wp_nh128 | 6959ns | 5208ns | 9251ns | +10.48% | 0.009 |
| wp_nh2k | 9764ns | 8574ns | 10863ns | +55.01% | 0.007 |
| wp_nh8k | 10026ns | 717ns | 16493ns | +59.17% | 0.006 |
| wp_park | 6302ns | 4774ns | 8863ns | +0.06% | 0.010 |
| wp_spin | 10681ns | 9699ns | 11553ns | +69.57% | 0.006 |

## Performance model

- Peak throughput: **0.089 Gops/s** (wp_nh8k; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.012 | 12.9% |
| wp_lp0 | 0.013 | 14.1% |
| wp_nh128 | 0.010 | 11.6% |
| wp_nh2k | 0.007 | 7.4% |
| wp_nh8k | 0.006 | 6.6% |
| wp_park | 0.012 | 13.9% |
| wp_spin | 0.006 | 6.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 8737ns | 8737ns | base |
| wp_lp0 | 7837ns | 7837ns | -10.30% |
| wp_nh128 | 9396ns | 9396ns | +7.55% |
| wp_nh2k | 12312ns | 12312ns | +40.92% |
| wp_nh8k | 12517ns | 12517ns | +43.27% |
| wp_park | 8772ns | 8772ns | +0.41% |
| wp_spin | 13305ns | 13305ns | +52.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 5546ns | base | --- | [5325, 6390] | --- | --- | --- | --- |
| wp_lp0 | 5069ns | -587.5ns (-10.6%) | [-1296, -233]ns | [4867, 5727] | YES | 0.0151 | 0.0075 | 0 |
| wp_nh128 | 6171ns | no significant difference | [-842, +1783]ns | [5631, 7069] | no | 0.8145 | 0.8145 | 0 |
| wp_nh2k | 9658ns | +3891.7ns (+70.2%) | [+3317, +4044]ns | [9035, 10344] | YES | 0.0004 | 0.0001 | 0 |
| wp_nh8k | 10890ns | no significant difference | [-1017, +8804]ns | [3904, 14190] | no | 0.2855 | 0.2379 | 0 |
| wp_park | 5144ns | no significant difference | [-812, +35]ns | [4931, 5627] | no | 0.1444 | 0.0963 | 0 |
| wp_spin | 10640ns | +4660.4ns (+84.0%) | [+4090, +5454]ns | [10058, 11131] | YES | 0.0004 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 5588ns | -12.9% | +12.8% | +86.0% | +153.2% | -7.8% | +111.0% |
| 1 | 2 | 5679ns | +17.7% | -6.9% | +72.0% | +78.2% | -1.5% | +88.8% |
| 1 | 3 | 6525ns | -26.1% | -13.5% | +38.0% | +180.5% | -21.3% | +53.4% |
| 2 | 1 | 5175ns | -3.4% | +39.9% | +70.9% | -32.1% | -3.8% | +88.4% |
| 2 | 2 | 9188ns | -34.1% | -33.5% | +24.9% | +40.4% | +36.9% | +19.6% |
| 2 | 3 | 5312ns | -14.3% | +74.8% | +70.7% | -19.1% | -8.1% | +111.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | -0.520 | HIGH- (thermal bounce) |
| wp_lp0 | -0.550 | HIGH- (thermal bounce) |
| wp_nh128 | -0.048 | ok |
| wp_nh2k | -0.396 | moderate- |
| wp_nh8k | -0.542 | HIGH- (thermal bounce) |
| wp_park | -0.309 | moderate- |
| wp_spin | 0.209 | moderate+ |

**Consistency summary:**

- **wp_lp0**: won 5/6, lost 1/6
- **wp_nh128**: won 3/6, lost 3/6
- **wp_nh2k**: won 0/6, lost 6/6
- **wp_nh8k**: won 2/6, lost 4/6
- **wp_park**: won 5/6, lost 1/6
- **wp_spin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 13.2ns | 6298.8ns | 0.2% |  |
| wp_lp0 | 15.3ns | 5372.7ns | 0.3% |  |
| wp_nh128 | 16.7ns | 6958.8ns | 0.2% |  |
| wp_nh2k | 24.5ns | 9764.1ns | 0.3% |  |
| wp_nh8k | 22.5ns | 10025.7ns | 0.2% |  |
| wp_park | 11.1ns | 6302.3ns | 0.2% |  |
| wp_spin | 23.4ns | 10681.2ns | 0.2% |  |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 5120.8-8109.0 ns)
   5120.8 |########################################
   5270.2 |##########################
   5419.7 |########################################
   5569.1 |##########################
   5718.5 |
   5867.9 |#############
   6017.3 |
   6166.7 |#############
   6316.1 |
   6465.5 |#############
   6614.9 |
   6764.3 |#############
   6913.8 |
   7063.2 |
   7212.6 |
   7362.0 |
   7511.4 |
   7660.8 |
   7810.2 |#############
   7959.6 |
  (1 below, 2 above range)

wp_lp0 (n=18, range 4522.2-6372.9 ns)
   4522.2 |##########################
   4614.8 |
   4707.3 |
   4799.8 |########################################
   4892.4 |#############
   4984.9 |##########################
   5077.4 |########################################
   5170.0 |
   5262.5 |
   5355.0 |#############
   5447.6 |
   5540.1 |
   5632.6 |
   5725.2 |
   5817.7 |
   5910.2 |
   6002.8 |#############
   6095.3 |
   6187.8 |
   6280.4 |
  (1 below, 4 above range)

wp_nh128 (n=18, range 5208.3-9250.7 ns)
   5208.3 |####################
   5410.5 |####################
   5612.6 |########################################
   5814.7 |####################
   6016.8 |########################################
   6218.9 |########################################
   6421.0 |
   6623.2 |
   6825.3 |########################################
   7027.4 |
   7229.5 |####################
   7431.6 |
   7633.8 |########################################
   7835.9 |
   8038.0 |
   8240.1 |
   8442.2 |
   8644.3 |
   8846.5 |
   9048.6 |
  (2 below, 2 above range)

wp_nh2k (n=18, range 8573.6-10863.2 ns)
   8573.6 |
   8688.1 |
   8802.6 |########################################
   8917.0 |####################
   9031.5 |########################################
   9146.0 |
   9260.5 |
   9375.0 |####################
   9489.4 |####################
   9603.9 |
   9718.4 |########################################
   9832.9 |
   9947.4 |
  10061.9 |
  10176.3 |####################
  10290.8 |########################################
  10405.3 |
  10519.8 |########################################
  10634.3 |
  10748.7 |
  (2 below, 2 above range)

wp_nh8k (n=18, range 716.7-16493.0 ns)
    716.7 |##########################
   1505.5 |
   2294.3 |#############
   3083.1 |#############
   3871.9 |#############
   4660.8 |
   5449.6 |
   6238.4 |
   7027.2 |
   7816.0 |
   8604.9 |
   9393.7 |########################################
  10182.5 |
  10971.3 |#############
  11760.1 |
  12549.0 |#############
  13337.8 |#############
  14126.6 |##########################
  14915.4 |
  15704.2 |#############
  (1 below, 3 above range)

wp_park (n=18, range 4773.6-8863.2 ns)
   4773.6 |################################
   4978.1 |########################################
   5182.6 |########
   5387.1 |########
   5591.5 |################
   5796.0 |
   6000.5 |########
   6205.0 |
   6409.5 |
   6613.9 |
   6818.4 |
   7022.9 |
   7227.4 |
   7431.9 |
   7636.3 |
   7840.8 |
   8045.3 |
   8249.8 |
   8454.2 |
   8658.7 |
  (1 below, 3 above range)

wp_spin (n=18, range 9698.6-11553.5 ns)
   9698.6 |########################################
   9791.3 |
   9884.1 |
   9976.8 |########################################
  10069.6 |########################################
  10162.3 |
  10255.1 |
  10347.8 |
  10440.5 |####################
  10533.3 |####################
  10626.0 |
  10718.8 |########################################
  10811.5 |
  10904.3 |####################
  10997.0 |
  11089.8 |########################################
  11182.5 |########################################
  11275.2 |
  11368.0 |
  11460.7 |
  (1 below, 2 above range)

```

## Diagnostics

- **wp_isb8**: CV=27.3% (high variance, measurements may be unstable)
- **wp_nh128**: CV=36.8% (high variance, measurements may be unstable)
- **wp_nh8k**: CV=61.6% (high variance, measurements may be unstable)
- **wp_nh8k**: worst_20/best_20 = 23.0x (possible bimodal distribution)
- **wp_park**: CV=42.8% (high variance, measurements may be unstable)
