# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wp_nh8k is an outlier: 3.1x slower than the field

wp_nh8k (16.70 us) is 3.1x the fastest (5.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (wp_isb8)

The baseline wp_isb8 is the fastest (5.46 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.1x the fastest

Fastest wp_isb8 (5.46 us) to slowest wp_nh8k (16.70 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### wp_nh8k is inconsistent: worst-20% is 12.2x its best-20%

wp_nh8k's best 20% of batches run at 1.91 us but its worst 20% at 23.26 us (12.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (wp_isb8) is the fastest** at 5462.5 ns median
- 3 variants significantly slower than baseline
- Spread: 3.06x (fastest 5462.5 ns, slowest 16697.9 ns)

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
| wp_isb8 | 8919ns | 7852ns | 7231ns | 7753ns | 11512ns | base |
| wp_lp0 | 12855ns | 11502ns | 7276ns | 10374ns | 19366ns | +44.13% |
| wp_nh128 | 8774ns | 8000ns | 7194ns | 7912ns | 10858ns | -1.62% |
| wp_nh2k | 13051ns | 12152ns | 11474ns | 11907ns | 15554ns | +46.33% |
| wp_nh8k | 17447ns | 19121ns | 4257ns | 15635ns | 26761ns | +95.62% |
| wp_park | 9915ns | 8475ns | 7017ns | 8215ns | 13915ns | +11.17% |
| wp_spin | 16210ns | 13669ns | 11711ns | 13622ns | 22342ns | +81.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 6403ns | 4872ns | 8752ns | base | 0.010 |
| wp_lp0 | 10430ns | 4988ns | 16933ns | +62.90% | 0.006 |
| wp_nh128 | 6348ns | 4835ns | 8360ns | -0.86% | 0.010 |
| wp_nh2k | 10424ns | 9046ns | 12804ns | +62.80% | 0.006 |
| wp_nh8k | 14634ns | 1912ns | 23260ns | +128.55% | 0.004 |
| wp_park | 7460ns | 4739ns | 11293ns | +16.51% | 0.009 |
| wp_spin | 13535ns | 9288ns | 19599ns | +111.38% | 0.005 |

## Performance model

- Peak throughput: **0.033 Gops/s** (wp_nh8k; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.012 | 35.0% |
| wp_lp0 | 0.007 | 21.4% |
| wp_nh128 | 0.012 | 34.4% |
| wp_nh2k | 0.007 | 20.3% |
| wp_nh8k | 0.004 | 11.5% |
| wp_park | 0.011 | 31.5% |
| wp_spin | 0.006 | 17.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 8919ns | 8919ns | base |
| wp_lp0 | 12855ns | 12855ns | +44.13% |
| wp_nh128 | 8774ns | 8774ns | -1.62% |
| wp_nh2k | 13051ns | 13051ns | +46.33% |
| wp_nh8k | 17447ns | 17447ns | +95.62% |
| wp_park | 9915ns | 9915ns | +11.17% |
| wp_spin | 16210ns | 16210ns | +81.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 5462ns | base | --- | [5210, 6392] | --- | --- | --- | --- |
| wp_lp0 | 8933ns | +2620.8ns (+48.0%) | [+119, +7504]ns | [5962, 14110] | YES (adj: no) | 0.1925 | 0.0963 | 0 |
| wp_nh128 | 5558ns | no significant difference | [-754, +883]ns | [5421, 6185] | no | 0.9774 | 0.8145 | 0 |
| wp_nh2k | 9400ns | +3964.6ns (+72.6%) | [+2927, +4273]ns | [9158, 9767] | YES | 0.0004 | 0.0001 | 0 |
| wp_nh8k | 16698ns | no significant difference | [-521, +13029]ns | [7200, 19298] | no | 0.3568 | 0.2379 | 0 |
| wp_park | 6075ns | no significant difference | [-454, +1562]ns | [5023, 8900] | no | 1.0000 | 1.0000 | 0 |
| wp_spin | 10775ns | +5235.4ns (+95.8%) | [+4438, +11423]ns | [10085, 16685] | YES | 0.0004 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 5196ns | +243.7% | +3.9% | +176.3% | +224.3% | +115.6% | +172.2% |
| 1 | 2 | 7738ns | +14.4% | -33.4% | +18.9% | +162.8% | +16.0% | +22.1% |
| 1 | 3 | 5225ns | +174.0% | +4.1% | +75.9% | +216.7% | -5.6% | +99.8% |
| 2 | 1 | 5325ns | -7.0% | +96.6% | +78.7% | +311.1% | -7.6% | +98.7% |
| 2 | 2 | 5592ns | -0.5% | -2.4% | +70.3% | +169.7% | +35.2% | +93.3% |
| 2 | 3 | 5712ns | +57.8% | +2.6% | +60.1% | -23.1% | -3.0% | +282.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | -0.401 | moderate- |
| wp_lp0 | -0.066 | ok |
| wp_nh128 | -0.222 | moderate- |
| wp_nh2k | -0.067 | ok |
| wp_nh8k | 0.084 | ok |
| wp_park | 0.210 | moderate+ |
| wp_spin | -0.038 | ok |

**Consistency summary:**

- **wp_lp0**: won 2/6, lost 4/6
- **wp_nh128**: won 2/6, lost 4/6
- **wp_nh2k**: won 0/6, lost 6/6
- **wp_nh8k**: won 1/6, lost 5/6
- **wp_park**: won 3/6, lost 3/6
- **wp_spin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 11.1ns | 6403.0ns | 0.2% |  |
| wp_lp0 | 15.3ns | 10430.3ns | 0.1% |  |
| wp_nh128 | 13.2ns | 6347.7ns | 0.2% |  |
| wp_nh2k | 290.0ns | 10424.1ns | 2.8% |  |
| wp_nh8k | 24.3ns | 14634.3ns | 0.2% |  |
| wp_park | 18.3ns | 7460.4ns | 0.2% |  |
| wp_spin | 22.2ns | 13535.0ns | 0.2% |  |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 4872.2-8752.1 ns)
   4872.2 |##########################
   5066.2 |##########################
   5260.2 |########################################
   5454.2 |########################################
   5648.2 |#############
   5842.2 |
   6036.2 |
   6230.2 |
   6424.2 |
   6618.2 |
   6812.2 |
   7006.2 |##########################
   7200.1 |
   7394.1 |
   7588.1 |#############
   7782.1 |
   7976.1 |
   8170.1 |#############
   8364.1 |
   8558.1 |
  (2 below, 1 above range)

wp_lp0 (n=18, range 4987.5-16932.6 ns)
   4987.5 |########################################
   5584.8 |#############
   6182.0 |
   6779.3 |
   7376.5 |#############
   7973.8 |
   8571.0 |########################################
   9168.3 |#############
   9765.6 |
  10362.8 |
  10960.1 |#############
  11557.3 |
  12154.6 |
  12751.8 |
  13349.1 |#############
  13946.4 |#############
  14543.6 |
  15140.9 |##########################
  15738.1 |
  16335.4 |
  (2 below, 2 above range)

wp_nh128 (n=18, range 4834.7-8359.7 ns)
   4834.7 |################
   5011.0 |########
   5187.2 |
   5363.5 |########################################
   5539.7 |########
   5716.0 |################
   5892.2 |
   6068.5 |################
   6244.7 |
   6421.0 |
   6597.2 |
   6773.5 |########
   6949.7 |
   7126.0 |
   7302.2 |
   7478.5 |
   7654.7 |
   7831.0 |
   8007.2 |
   8183.5 |########
  (1 below, 2 above range)

wp_nh2k (n=18, range 9045.8-12804.2 ns)
   9045.8 |########################################
   9233.8 |#####
   9421.7 |#################
   9609.6 |#####
   9797.5 |###########
   9985.4 |
  10173.3 |
  10361.2 |
  10549.2 |
  10737.1 |
  10925.0 |
  11112.9 |
  11300.8 |
  11488.8 |
  11676.7 |
  11864.6 |
  12052.5 |
  12240.4 |#####
  12428.3 |
  12616.2 |
  (1 below, 2 above range)

wp_nh8k (n=18, range 1912.5-23260.4 ns)
   1912.5 |#############
   2979.9 |##########################
   4047.3 |#############
   5114.7 |
   6182.1 |
   7249.5 |
   8316.9 |
   9384.3 |#############
  10451.7 |
  11519.1 |
  12586.5 |
  13653.9 |
  14721.3 |#############
  15788.7 |########################################
  16856.1 |#############
  17923.4 |#############
  18990.8 |#############
  20058.2 |##########################
  21125.6 |#############
  22193.0 |
  (1 below, 2 above range)

wp_park (n=18, range 4738.9-11293.1 ns)
   4738.9 |########################################
   5066.6 |
   5394.3 |########################
   5722.0 |
   6049.7 |
   6377.4 |################
   6705.1 |
   7032.8 |
   7360.5 |########
   7688.3 |
   8016.0 |
   8343.7 |
   8671.4 |################
   8999.1 |
   9326.8 |
   9654.5 |
   9982.2 |
  10309.9 |########
  10637.6 |
  10965.4 |########
  (1 below, 2 above range)

wp_spin (n=18, range 9287.5-19599.3 ns)
   9287.5 |##############################
   9803.1 |####################
  10318.7 |########################################
  10834.3 |
  11349.9 |##########
  11865.5 |
  12381.0 |
  12896.6 |
  13412.2 |
  13927.8 |##########
  14443.4 |
  14959.0 |
  15474.6 |
  15990.2 |##########
  16505.8 |
  17021.4 |##########
  17537.0 |
  18052.5 |
  18568.1 |##########
  19083.7 |
  (1 below, 3 above range)

```

## Diagnostics

- **wp_isb8**: CV=41.6% (high variance, measurements may be unstable)
- **wp_lp0**: CV=51.4% (high variance, measurements may be unstable)
- **wp_lp0**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **wp_nh128**: CV=30.5% (high variance, measurements may be unstable)
- **wp_nh2k**: CV=27.0% (high variance, measurements may be unstable)
- **wp_nh8k**: CV=58.5% (high variance, measurements may be unstable)
- **wp_nh8k**: worst_20/best_20 = 12.2x (possible bimodal distribution)
- **wp_park**: CV=41.2% (high variance, measurements may be unstable)
- **wp_spin**: CV=34.2% (high variance, measurements may be unstable)
