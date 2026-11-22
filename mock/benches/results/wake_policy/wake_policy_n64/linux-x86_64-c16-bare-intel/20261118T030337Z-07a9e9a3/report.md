# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wp_nh8k dominates: 226% faster than the next best (wp_lp0)

wp_nh8k (5.29 us) leads wp_lp0 (17.24 us) by 226%, a clear separation rather than a photo finish. CV 234.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wp_nh8k beats baseline by 68% (significant)

wp_nh8k is -11.96 us (68%) faster than baseline wp_isb8, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### wp_park is an outlier: 4.7x slower than the field

wp_park (24.98 us) is 4.7x the fastest (5.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### wp_nh8k is fastest but the noisiest (CV 234.7%)

wp_nh8k wins on median (5.29 us) yet has the highest variance (CV 234.7%), while wp_isb8 is the steadiest (CV 96.9%, 17.62 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {wp_nh8k} vs {wp_lp0, wp_nh2k, wp_isb8, wp_spin, wp_nh128, wp_park} (226% apart)

The field splits into a fast tier {wp_nh8k} and a slow tier {wp_lp0, wp_nh2k, wp_isb8, wp_spin, wp_nh128, wp_park} with a 226% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.7x the fastest

Fastest wp_nh8k (5.29 us) to slowest wp_park (24.98 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### wp_spin is inconsistent: worst-20% is 9.5x its best-20%

wp_spin's best 20% of batches run at 5.78 us but its worst 20% at 55.02 us (9.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wp_nh8k** at 5294.4 ns median (-70.0% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 4.72x (fastest 5294.4 ns, slowest 24980.3 ns)

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
| wp_isb8 | 34382ns | 25391ns | 18442ns | 24022ns | 57891ns | base |
| wp_lp0 | 41604ns | 25312ns | 20254ns | 25223ns | 76850ns | +21.01% |
| wp_nh128 | 46190ns | 30368ns | 21993ns | 32041ns | 79513ns | +34.35% |
| wp_nh2k | 38357ns | 25330ns | 18629ns | 26362ns | 66214ns | +11.56% |
| wp_nh8k | 19261ns | 12374ns | 8431ns | 11734ns | 35966ns | -43.98% |
| wp_park | 48267ns | 34986ns | 20888ns | 30345ns | 88839ns | +40.39% |
| wp_spin | 36976ns | 28845ns | 13211ns | 26101ns | 65169ns | +7.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 26396ns | 12359ns | 48877ns | base | 0.002 |
| wp_lp0 | 32918ns | 13782ns | 66421ns | +24.71% | 0.002 |
| wp_nh128 | 37152ns | 14938ns | 68401ns | +40.75% | 0.002 |
| wp_nh2k | 30558ns | 12376ns | 56947ns | +15.77% | 0.002 |
| wp_nh8k | 11826ns | 2947ns | 27099ns | -55.20% | 0.005 |
| wp_park | 38514ns | 13833ns | 76993ns | +45.91% | 0.002 |
| wp_spin | 28563ns | 5775ns | 55019ns | +8.21% | 0.002 |

## Performance model

- Peak throughput: **0.022 Gops/s** (wp_nh8k; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.004 | 16.7% |
| wp_lp0 | 0.004 | 17.1% |
| wp_nh128 | 0.003 | 13.3% |
| wp_nh2k | 0.004 | 16.8% |
| wp_nh8k | 0.012 | 55.7% |
| wp_park | 0.003 | 11.8% |
| wp_spin | 0.003 | 13.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 34382ns | 34382ns | base |
| wp_lp0 | 41604ns | 41604ns | +21.01% |
| wp_nh128 | 46190ns | 46190ns | +34.35% |
| wp_nh2k | 38357ns | 38357ns | +11.56% |
| wp_nh8k | 19261ns | 19261ns | -43.98% |
| wp_park | 48267ns | 48267ns | +40.39% |
| wp_spin | 36976ns | 36976ns | +7.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 17625ns | base | --- | [14381, 36135] | --- | --- | --- | --- |
| wp_lp0 | 17236ns | no significant difference | [-382, +6280]ns | [16124, 52256] | no | 0.2855 | 0.2379 | 0 |
| wp_nh128 | 22132ns | +6185.8ns (+35.1%) | [+2547, +11652]ns | [17774, 62349] | YES | 0.0004 | 0.0001 | 0 |
| wp_nh2k | 17547ns | no significant difference | [-405, +2693]ns | [15121, 42304] | no | 0.4807 | 0.4807 | 0 |
| wp_nh8k | 5294ns | -11958.6ns (-67.8%) | [-14694, -9324]ns | [3255, 17584] | YES | 0.0004 | 0.0001 | 0 |
| wp_park | 24980ns | +5571.1ns (+31.6%) | [+1091, +14968]ns | [16086, 46649] | YES (adj: no) | 0.1925 | 0.0963 | 0 |
| wp_spin | 21767ns | no significant difference | [-6722, +9424]ns | [9523, 42227] | no | 0.2855 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 12285ns | +9.5% | +21.2% | -1.4% | -75.5% | +28.1% | +8.8% |
| 1 | 2 | 36109ns | -55.2% | -37.6% | -54.6% | -80.8% | -54.5% | -68.5% |
| 1 | 3 | 14931ns | +31.9% | +16.7% | +167.2% | -56.4% | +112.8% | -49.1% |
| 2 | 1 | 18185ns | -2.5% | +27.9% | -3.6% | -70.5% | -18.9% | +102.4% |
| 2 | 2 | 14437ns | +4.9% | +50.5% | +9.8% | -78.9% | +86.5% | +166.8% |
| 2 | 3 | 19142ns | -9.4% | +194.5% | +3.7% | -77.0% | +40.2% | +88.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | -0.478 | moderate- |
| wp_lp0 | 0.033 | ok |
| wp_nh128 | -0.022 | ok |
| wp_nh2k | -0.170 | ok |
| wp_nh8k | 0.020 | ok |
| wp_park | -0.385 | moderate- |
| wp_spin | 0.464 | moderate+ |

**Consistency summary:**

- **wp_lp0**: won 3/6, lost 3/6
- **wp_nh128**: won 1/6, lost 5/6
- **wp_nh2k**: won 3/6, lost 3/6
- **wp_nh8k**: won 6/6, lost 0/6
- **wp_park**: won 2/6, lost 4/6
- **wp_spin**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 269.0ns | 26396.2ns | 1.0% |  |
| wp_lp0 | 297.7ns | 32918.2ns | 0.9% |  |
| wp_nh128 | 350.4ns | 37152.3ns | 0.9% |  |
| wp_nh2k | 236.5ns | 30557.9ns | 0.8% |  |
| wp_nh8k | 183.1ns | 11825.8ns | 1.5% |  |
| wp_park | 375.3ns | 38513.8ns | 1.0% |  |
| wp_spin | 235.1ns | 28562.5ns | 0.8% |  |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 12358.8-48876.6 ns)
  12358.8 |####################
  14184.7 |########################################
  16010.6 |####################
  17836.5 |####################
  19662.4 |
  21488.2 |
  23314.1 |
  25140.0 |
  26965.9 |
  28791.8 |
  30617.7 |
  32443.6 |
  34269.5 |
  36095.4 |####################
  37921.2 |
  39747.1 |
  41573.0 |
  43398.9 |
  45224.8 |
  47050.7 |
  (2 below, 4 above range)

wp_lp0 (n=18, range 13782.2-66421.3 ns)
  13782.2 |########################################
  16414.1 |################################
  19046.1 |########
  21678.0 |
  24310.0 |
  26942.0 |
  29573.9 |
  32205.9 |
  34837.8 |
  37469.8 |
  40101.7 |
  42733.7 |
  45365.6 |########
  47997.6 |
  50629.6 |
  53261.5 |
  55893.5 |########
  58525.4 |########
  61157.4 |########
  63789.3 |
  (2 below, 2 above range)

wp_nh128 (n=18, range 14938.3-68401.0 ns)
  14938.3 |########################################
  17611.4 |########################################
  20284.6 |##########################
  22957.7 |#############
  25630.8 |
  28304.0 |
  30977.1 |
  33650.2 |
  36323.4 |
  38996.5 |
  41669.6 |
  44342.8 |
  47015.9 |
  49689.0 |
  52362.2 |
  55035.3 |#############
  57708.4 |#############
  60381.6 |
  63054.7 |
  65727.8 |##########################
  (2 below, 3 above range)

wp_nh2k (n=18, range 12375.5-56946.8 ns)
  12375.5 |########################################
  14604.1 |##########################
  16832.7 |########################################
  19061.2 |#############
  21289.8 |
  23518.3 |
  25746.9 |
  27975.5 |
  30204.0 |
  32432.6 |
  34661.2 |
  36889.7 |#############
  39118.3 |#############
  41346.8 |
  43575.4 |#############
  45804.0 |
  48032.5 |
  50261.1 |#############
  52489.6 |
  54718.2 |
  (2 below, 3 above range)

wp_nh8k (n=18, range 2946.6-27098.7 ns)
   2946.6 |########################################
   4154.2 |################
   5361.8 |################
   6569.4 |########
   7777.0 |
   8984.6 |
  10192.3 |
  11399.9 |
  12607.5 |
  13815.1 |
  15022.7 |
  16230.3 |
  17437.9 |################
  18645.5 |
  19853.1 |
  21060.7 |
  22268.3 |########
  23475.9 |########
  24683.5 |
  25891.1 |
  (2 below, 2 above range)

wp_park (n=18, range 13832.8-76993.3 ns)
  13832.8 |########################################
  16990.9 |######
  20148.9 |######
  23306.9 |
  26464.9 |#############
  29622.9 |######
  32781.0 |
  35939.0 |
  39097.0 |
  42255.0 |######
  45413.1 |
  48571.1 |
  51729.1 |
  54887.1 |
  58045.1 |
  61203.2 |#############
  64361.2 |
  67519.2 |
  70677.2 |
  73835.3 |
  (1 below, 3 above range)

wp_spin (n=18, range 5775.5-55019.2 ns)
   5775.5 |########################################
   8237.7 |##########################
  10699.8 |##########################
  13162.0 |#############
  15624.2 |
  18086.4 |
  20548.6 |
  23010.8 |
  25473.0 |
  27935.1 |#############
  30397.3 |
  32859.5 |
  35321.7 |##########################
  37783.9 |#############
  40246.1 |
  42708.3 |
  45170.5 |#############
  47632.6 |##########################
  50094.8 |
  52557.0 |#############
  (1 below, 1 above range)

```

## Diagnostics

- **wp_isb8**: CV=64.7% (high variance, measurements may be unstable)
- **wp_isb8**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **wp_lp0**: CV=76.4% (high variance, measurements may be unstable)
- **wp_lp0**: worst_20/best_20 = 4.8x (possible bimodal distribution)
- **wp_nh128**: CV=64.4% (high variance, measurements may be unstable)
- **wp_nh128**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **wp_nh2k**: CV=67.8% (high variance, measurements may be unstable)
- **wp_nh2k**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **wp_nh8k**: CV=105.1% (high variance, measurements may be unstable)
- **wp_nh8k**: worst_20/best_20 = 9.2x (possible bimodal distribution)
- **wp_park**: CV=82.9% (high variance, measurements may be unstable)
- **wp_park**: worst_20/best_20 = 5.6x (possible bimodal distribution)
- **wp_spin**: CV=82.0% (high variance, measurements may be unstable)
- **wp_spin**: worst_20/best_20 = 9.5x (possible bimodal distribution)
