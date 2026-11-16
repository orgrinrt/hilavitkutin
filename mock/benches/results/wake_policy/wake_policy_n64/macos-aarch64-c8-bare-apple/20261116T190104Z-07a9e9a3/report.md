# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wp_nh8k is an outlier: 2.5x slower than the field

wp_nh8k (12.53 us) is 2.5x the fastest (5.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### wp_park shows alternating (throttle bounce) (autocorr -0.77)

wp_park's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {wp_lp0, wp_nh128, wp_park, wp_isb8} vs {wp_nh2k, wp_spin, wp_nh8k} (62% apart)

The field splits into a fast tier {wp_lp0, wp_nh128, wp_park, wp_isb8} and a slow tier {wp_nh2k, wp_spin, wp_nh8k} with a 62% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### wp_nh8k is inconsistent: worst-20% is 9.5x its best-20%

wp_nh8k's best 20% of batches run at 1.95 us but its worst 20% at 18.46 us (9.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wp_lp0** at 5037.5 ns median (-9.4% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.49x (fastest 5037.5 ns, slowest 12531.2 ns)

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
| wp_isb8 | 8484ns | 8100ns | 7249ns | 7847ns | 10059ns | base |
| wp_lp0 | 7894ns | 7475ns | 7010ns | 7473ns | 8967ns | -6.96% |
| wp_nh128 | 7947ns | 7779ns | 7212ns | 7668ns | 8733ns | -6.34% |
| wp_nh2k | 11735ns | 11544ns | 11026ns | 11465ns | 12495ns | +38.32% |
| wp_nh8k | 14333ns | 14996ns | 4210ns | 13143ns | 21178ns | +68.93% |
| wp_park | 8104ns | 7802ns | 6817ns | 7696ns | 9360ns | -4.48% |
| wp_spin | 12342ns | 12060ns | 11582ns | 12100ns | 13083ns | +45.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 6024ns | 4903ns | 7511ns | base | 0.011 |
| wp_lp0 | 5481ns | 4683ns | 6504ns | -9.00% | 0.012 |
| wp_nh128 | 5522ns | 4922ns | 6190ns | -8.33% | 0.012 |
| wp_nh2k | 9181ns | 8460ns | 9865ns | +52.41% | 0.007 |
| wp_nh8k | 11839ns | 1947ns | 18460ns | +96.53% | 0.005 |
| wp_park | 5724ns | 4560ns | 6925ns | -4.97% | 0.011 |
| wp_spin | 9890ns | 9089ns | 10528ns | +64.17% | 0.006 |

## Performance model

- Peak throughput: **0.033 Gops/s** (wp_nh8k; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.012 | 35.0% |
| wp_lp0 | 0.013 | 38.7% |
| wp_nh128 | 0.012 | 36.3% |
| wp_nh2k | 0.007 | 21.6% |
| wp_nh8k | 0.005 | 15.5% |
| wp_park | 0.012 | 35.9% |
| wp_spin | 0.007 | 20.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 8484ns | 8484ns | base |
| wp_lp0 | 7894ns | 7894ns | -6.96% |
| wp_nh128 | 7947ns | 7947ns | -6.34% |
| wp_nh2k | 11735ns | 11735ns | +38.32% |
| wp_nh8k | 14333ns | 14333ns | +68.93% |
| wp_park | 8104ns | 8104ns | -4.48% |
| wp_spin | 12342ns | 12342ns | +45.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 5560ns | base | --- | [5223, 5756] | --- | --- | --- | --- |
| wp_lp0 | 5038ns | -354.2ns (-6.4%) | [-637, -185]ns | [4933, 5735] | YES (adj: no) | 0.0618 | 0.0309 | 0 |
| wp_nh128 | 5371ns | no significant difference | [-560, +221]ns | [5135, 5617] | no | 0.6291 | 0.6291 | 1 |
| wp_nh2k | 8998ns | +3610.4ns (+64.9%) | [+2719, +3796]ns | [8740, 9494] | YES | 0.0000 | 0.0000 | 0 |
| wp_nh8k | 12531ns | +7387.6ns (+132.9%) | [+1494, +10946]ns | [6846, 17475] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| wp_park | 5429ns | no significant difference | [-821, +369]ns | [5040, 6483] | no | 0.5768 | 0.4807 | 0 |
| wp_spin | 9735ns | +4358.3ns (+78.4%) | [+3979, +4846]ns | [9598, 10135] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 5146ns | -5.3% | +7.7% | +92.6% | +212.7% | -2.9% | +82.6% |
| 1 | 2 | 5762ns | +1.9% | -9.8% | +51.3% | +47.7% | +13.2% | +69.1% |
| 1 | 3 | 5308ns | -6.6% | -4.4% | +65.0% | -17.2% | -14.3% | +90.5% |
| 2 | 1 | 8571ns | -17.7% | -39.3% | +16.3% | +45.5% | -24.4% | +12.3% |
| 2 | 2 | 5550ns | -5.2% | +2.3% | +59.0% | +126.9% | -3.3% | +91.1% |
| 2 | 3 | 5221ns | -6.0% | +4.5% | +71.4% | +250.1% | +24.4% | +83.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | -0.247 | moderate- |
| wp_lp0 | -0.409 | moderate- |
| wp_nh128 | 0.151 | ok |
| wp_nh2k | -0.388 | moderate- |
| wp_nh8k | 0.104 | ok |
| wp_park | -0.770 | HIGH- (thermal bounce) |
| wp_spin | -0.412 | moderate- |

**Consistency summary:**

- **wp_lp0**: won 5/6, lost 1/6
- **wp_nh128**: won 3/6, lost 3/6
- **wp_nh2k**: won 0/6, lost 6/6
- **wp_nh8k**: won 1/6, lost 5/6
- **wp_park**: won 4/6, lost 2/6
- **wp_spin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 15.3ns | 6023.8ns | 0.3% |  |
| wp_lp0 | 13.2ns | 5481.5ns | 0.2% |  |
| wp_nh128 | 11.6ns | 5521.8ns | 0.2% |  |
| wp_nh2k | 20.1ns | 9181.0ns | 0.2% |  |
| wp_nh8k | 23.1ns | 11838.9ns | 0.2% |  |
| wp_park | 13.7ns | 5724.3ns | 0.2% |  |
| wp_spin | 19.2ns | 9889.6ns | 0.2% |  |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 4902.8-7511.1 ns)
   4902.8 |#############
   5033.2 |#############
   5163.6 |########################################
   5294.0 |#############
   5424.4 |#############
   5554.9 |##########################
   5685.3 |########################################
   5815.7 |
   5946.1 |
   6076.5 |
   6206.9 |
   6337.4 |
   6467.8 |
   6598.2 |
   6728.6 |
   6859.0 |
   6989.4 |
   7119.9 |
   7250.3 |
   7380.7 |
  (2 below, 4 above range)

wp_lp0 (n=18, range 4683.3-6504.2 ns)
   4683.3 |##########
   4774.4 |
   4865.4 |####################
   4956.5 |########################################
   5047.5 |##########
   5138.5 |
   5229.6 |##########
   5320.6 |
   5411.7 |
   5502.7 |##########
   5593.8 |##########
   5684.8 |
   5775.8 |
   5866.9 |##########
   5957.9 |
   6049.0 |
   6140.0 |
   6231.0 |
   6322.1 |
   6413.1 |
  (2 below, 4 above range)

wp_nh128 (n=18, range 4922.2-6189.6 ns)
   4922.2 |####################
   4985.6 |####################
   5048.9 |####################
   5112.3 |########################################
   5175.7 |########################################
   5239.0 |####################
   5302.4 |
   5365.8 |
   5429.2 |########################################
   5492.5 |####################
   5555.9 |####################
   5619.3 |####################
   5682.6 |
   5746.0 |
   5809.4 |
   5872.7 |####################
   5936.1 |
   5999.5 |
   6062.8 |
   6126.2 |
  (1 below, 3 above range)

wp_nh2k (n=18, range 8459.7-9864.6 ns)
   8459.7 |
   8529.9 |#############
   8600.2 |
   8670.4 |########################################
   8740.7 |#############
   8810.9 |#############
   8881.2 |##########################
   8951.4 |
   9021.6 |#############
   9091.9 |
   9162.1 |
   9232.4 |
   9302.6 |
   9372.9 |##########################
   9443.1 |##########################
   9513.4 |
   9583.6 |
   9653.8 |
   9724.1 |
   9794.3 |
  (1 below, 4 above range)

wp_nh8k (n=18, range 1947.2-18459.7 ns)
   1947.2 |
   2772.9 |
   3598.5 |##########################
   4424.1 |#############
   5249.7 |
   6075.4 |
   6901.0 |
   7726.6 |#############
   8552.2 |
   9377.9 |#############
  10203.5 |
  11029.1 |
  11854.7 |########################################
  12680.4 |
  13506.0 |
  14331.6 |#############
  15157.2 |
  15982.9 |#############
  16808.5 |##########################
  17634.1 |##########################
  (2 below, 2 above range)

wp_park (n=18, range 4559.7-6925.0 ns)
   4559.7 |##########################
   4678.0 |
   4796.3 |
   4914.5 |#############
   5032.8 |########################################
   5151.1 |
   5269.3 |#############
   5387.6 |#############
   5505.8 |
   5624.1 |
   5742.4 |
   5860.6 |
   5978.9 |##########################
   6097.2 |
   6215.4 |
   6333.7 |
   6451.9 |########################################
   6570.2 |
   6688.5 |
   6806.7 |#############
  (2 below, 2 above range)

wp_spin (n=18, range 9088.9-10527.8 ns)
   9088.9 |
   9160.8 |
   9232.8 |##########
   9304.7 |
   9376.7 |##########
   9448.6 |##########
   9520.6 |
   9592.5 |########################################
   9664.5 |##########
   9736.4 |##########
   9808.4 |
   9880.3 |
   9952.2 |
  10024.2 |##########
  10096.1 |##############################
  10168.1 |
  10240.0 |##########
  10312.0 |
  10383.9 |
  10455.9 |
  (1 below, 3 above range)

```

## Diagnostics

- **wp_isb8**: CV=21.7% (high variance, measurements may be unstable)
- **wp_nh8k**: CV=53.2% (high variance, measurements may be unstable)
- **wp_nh8k**: worst_20/best_20 = 9.5x (possible bimodal distribution)
