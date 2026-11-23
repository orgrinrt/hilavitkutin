# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wp_nh8k dominates: 16% faster than the next best (wp_spin)

wp_nh8k (11.20 us) leads wp_spin (13.01 us) by 16%, a clear separation rather than a photo finish. CV 149.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wp_nh8k beats baseline by 52% (significant)

wp_nh8k is -7.54 us (52%) faster than baseline wp_isb8, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### wp_nh8k is fastest but the noisiest (CV 149.6%)

wp_nh8k wins on median (11.20 us) yet has the highest variance (CV 149.6%), while wp_nh128 is the steadiest (CV 100.2%, 16.44 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wp_nh2k shows alternating (throttle bounce) (autocorr -0.54)

wp_nh2k's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5.68 us) is smaller than the fastest variant's own run-to-run std-dev (16.75 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### wp_nh8k is inconsistent: worst-20% is 9.1x its best-20%

wp_nh8k's best 20% of batches run at 3.93 us but its worst 20% at 35.91 us (9.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wp_nh8k** at 11196.2 ns median (-22.8% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.51x (fastest 11196.2 ns, slowest 16876.6 ns)

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
| wp_isb8 | 34995ns | 22322ns | 18550ns | 21853ns | 62931ns | base |
| wp_lp0 | 43057ns | 26244ns | 22119ns | 25845ns | 79343ns | +23.04% |
| wp_nh128 | 34681ns | 25159ns | 18874ns | 25360ns | 56567ns | -0.90% |
| wp_nh2k | 32147ns | 23625ns | 16230ns | 22428ns | 54685ns | -8.14% |
| wp_nh8k | 25292ns | 17454ns | 10639ns | 16635ns | 45604ns | -27.73% |
| wp_park | 37572ns | 24264ns | 19486ns | 24003ns | 66968ns | +7.36% |
| wp_spin | 28225ns | 22568ns | 13352ns | 19770ns | 48344ns | -19.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 26640ns | 11570ns | 52443ns | base | 0.002 |
| wp_lp0 | 33969ns | 14610ns | 68641ns | +27.51% | 0.002 |
| wp_nh128 | 26561ns | 11905ns | 47343ns | -0.30% | 0.002 |
| wp_nh2k | 24797ns | 10060ns | 45971ns | -6.92% | 0.003 |
| wp_nh8k | 17348ns | 3929ns | 35909ns | -34.88% | 0.004 |
| wp_park | 28850ns | 12720ns | 55932ns | +8.30% | 0.002 |
| wp_spin | 19304ns | 6173ns | 37383ns | -27.54% | 0.003 |

## Performance model

- Peak throughput: **0.016 Gops/s** (wp_nh8k; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.004 | 27.1% |
| wp_lp0 | 0.004 | 23.3% |
| wp_nh128 | 0.004 | 23.9% |
| wp_nh2k | 0.004 | 24.2% |
| wp_nh8k | 0.006 | 35.1% |
| wp_park | 0.004 | 24.9% |
| wp_spin | 0.005 | 30.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 34995ns | 34995ns | base |
| wp_lp0 | 43057ns | 43057ns | +23.04% |
| wp_nh128 | 34681ns | 34681ns | -0.90% |
| wp_nh2k | 32147ns | 32147ns | -8.14% |
| wp_nh8k | 25292ns | 25292ns | -27.73% |
| wp_park | 37572ns | 37572ns | +7.36% |
| wp_spin | 28225ns | 28225ns | -19.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 14503ns | base | --- | [12914, 37960] | --- | --- | --- | --- |
| wp_lp0 | 16877ns | +4648.1ns (+32.0%) | [+1180, +8205]ns | [16016, 64974] | YES | 0.0079 | 0.0013 | 0 |
| wp_nh128 | 16435ns | no significant difference | [-2724, +4378]ns | [12515, 36402] | no | 0.8145 | 0.8145 | 0 |
| wp_nh2k | 16233ns | no significant difference | [-4652, +7082]ns | [11983, 29603] | no | 0.8145 | 0.8145 | 0 |
| wp_nh8k | 11196ns | -7535.6ns (-52.0%) | [-10985, -4229]ns | [4490, 22412] | YES | 0.0463 | 0.0309 | 0 |
| wp_park | 15769ns | +2383.3ns (+16.4%) | [+959, +4918]ns | [14152, 44866] | YES | 0.0463 | 0.0309 | 0 |
| wp_spin | 13013ns | -6621.0ns (-45.7%) | [-10903, -3140]ns | [6932, 30469] | YES | 0.0463 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 11529ns | +47.2% | +38.4% | +3.1% | -60.6% | +29.1% | -43.2% |
| 1 | 2 | 14172ns | +13.2% | -12.7% | +93.1% | +12.3% | +4.4% | +15.0% |
| 1 | 3 | 15484ns | +8.3% | -18.3% | -17.3% | -16.0% | -12.1% | -3.2% |
| 2 | 1 | 12578ns | +56.5% | +33.8% | +124.5% | +102.2% | +24.2% | +133.0% |
| 2 | 2 | 14393ns | +3.0% | -14.6% | -32.6% | -34.8% | +15.4% | -49.4% |
| 2 | 3 | 19309ns | +12.8% | +66.6% | -36.8% | -60.5% | +33.6% | -56.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | -0.040 | ok |
| wp_lp0 | -0.503 | HIGH- (thermal bounce) |
| wp_nh128 | -0.151 | ok |
| wp_nh2k | -0.540 | HIGH- (thermal bounce) |
| wp_nh8k | -0.167 | ok |
| wp_park | 0.131 | ok |
| wp_spin | -0.168 | ok |

**Consistency summary:**

- **wp_lp0**: won 0/6, lost 6/6
- **wp_nh128**: won 3/6, lost 3/6
- **wp_nh2k**: won 3/6, lost 3/6
- **wp_nh8k**: won 4/6, lost 2/6
- **wp_park**: won 1/6, lost 5/6
- **wp_spin**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 253.2ns | 26639.6ns | 1.0% |  |
| wp_lp0 | 299.2ns | 33968.6ns | 0.9% |  |
| wp_nh128 | 285.7ns | 26560.8ns | 1.1% |  |
| wp_nh2k | 312.7ns | 24796.6ns | 1.3% |  |
| wp_nh8k | 198.8ns | 17348.1ns | 1.1% |  |
| wp_park | 297.2ns | 28849.9ns | 1.0% |  |
| wp_spin | 221.1ns | 19304.0ns | 1.1% |  |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 11569.5-52442.7 ns)
  11569.5 |################################
  13613.2 |########################################
  15656.8 |
  17700.5 |########
  19744.1 |
  21787.8 |
  23831.5 |
  25875.1 |
  27918.8 |
  29962.5 |
  32006.1 |########
  34049.8 |
  36093.4 |
  38137.1 |
  40180.8 |
  42224.4 |################
  44268.1 |
  46311.7 |
  48355.4 |
  50399.1 |
  (2 below, 3 above range)

wp_lp0 (n=18, range 14609.8-68641.1 ns)
  14609.8 |########################################
  17311.4 |####
  20013.0 |####
  22714.5 |
  25416.1 |
  28117.6 |
  30819.2 |
  33520.8 |
  36222.3 |
  38923.9 |
  41625.5 |
  44327.0 |
  47028.6 |
  49730.2 |
  52431.7 |
  55133.3 |
  57834.8 |
  60536.4 |
  63238.0 |########
  65939.5 |####
  (1 below, 3 above range)

wp_nh128 (n=18, range 11905.3-47343.4 ns)
  11905.3 |########################################
  13677.2 |##########
  15449.1 |##############################
  17221.0 |
  18992.9 |
  20764.8 |
  22536.7 |
  24308.6 |##########
  26080.6 |
  27852.5 |
  29624.4 |
  31396.3 |##########
  33168.2 |
  34940.1 |##########
  36712.0 |##########
  38483.9 |
  40255.8 |
  42027.7 |
  43799.6 |##########
  45571.5 |##########
  (2 below, 2 above range)

wp_nh2k (n=18, range 10060.2-45971.3 ns)
  10060.2 |################
  11855.8 |########################################
  13651.3 |
  15446.9 |
  17242.4 |
  19038.0 |################
  20833.5 |
  22629.1 |
  24424.6 |
  26220.2 |########
  28015.8 |########
  29811.3 |########
  31606.9 |
  33402.4 |
  35198.0 |
  36993.5 |
  38789.1 |
  40584.6 |
  42380.2 |
  44175.8 |
  (2 below, 4 above range)

wp_nh8k (n=18, range 3928.6-35908.7 ns)
   3928.6 |########################################
   5527.6 |########
   7126.6 |########
   8725.6 |########
  10324.6 |
  11923.6 |########
  13522.6 |
  15121.6 |########
  16720.6 |
  18319.6 |########
  19918.6 |
  21517.7 |########
  23116.7 |########
  24715.7 |########
  26314.7 |
  27913.7 |
  29512.7 |
  31111.7 |########
  32710.7 |
  34309.7 |
  (1 below, 2 above range)

wp_park (n=18, range 12719.8-55932.3 ns)
  12719.8 |########################################
  14880.4 |################################
  17041.0 |
  19201.7 |
  21362.3 |
  23522.9 |
  25683.5 |########
  27844.2 |
  30004.8 |
  32165.4 |
  34326.0 |
  36486.7 |
  38647.3 |
  40807.9 |
  42968.5 |########
  45129.2 |
  47289.8 |
  49450.4 |########
  51611.0 |
  53771.7 |########
  (2 below, 3 above range)

wp_spin (n=18, range 6173.4-37383.3 ns)
   6173.4 |########################################
   7733.9 |######
   9294.4 |
  10854.9 |######
  12415.4 |
  13975.9 |######
  15536.4 |######
  17096.9 |
  18657.4 |
  20217.9 |
  21778.4 |
  23338.9 |
  24899.4 |######
  26459.9 |
  28020.4 |######
  29580.9 |
  31141.4 |#############
  32701.8 |
  34262.3 |######
  35822.8 |
  (1 below, 2 above range)

```

## Diagnostics

- **wp_isb8**: CV=76.8% (high variance, measurements may be unstable)
- **wp_isb8**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **wp_lp0**: CV=72.6% (high variance, measurements may be unstable)
- **wp_lp0**: worst_20/best_20 = 4.7x (possible bimodal distribution)
- **wp_nh128**: CV=62.0% (high variance, measurements may be unstable)
- **wp_nh128**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **wp_nh2k**: CV=69.2% (high variance, measurements may be unstable)
- **wp_nh2k**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **wp_nh8k**: CV=96.6% (high variance, measurements may be unstable)
- **wp_nh8k**: worst_20/best_20 = 9.1x (possible bimodal distribution)
- **wp_park**: CV=68.4% (high variance, measurements may be unstable)
- **wp_park**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **wp_spin**: CV=76.0% (high variance, measurements may be unstable)
- **wp_spin**: worst_20/best_20 = 6.1x (possible bimodal distribution)
