# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wp_isb8) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wp_isb8 has the worst median (30.45 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wp_spin at 14.53 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wp_spin dominates: 13% faster than the next best (wp_park)

wp_spin (14.53 us) leads wp_park (16.44 us) by 13%, a clear separation rather than a photo finish. CV 98.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wp_nh8k beats baseline by 37% (significant)

wp_nh8k is -11.30 us (37%) faster than baseline wp_isb8, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### wp_isb8 is an outlier: 2.1x slower than the field

wp_isb8 (30.45 us) is 2.1x the fastest (14.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### wp_nh8k shows alternating (throttle bounce) (autocorr -0.72)

wp_nh8k's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### wp_nh8k is inconsistent: worst-20% is 9.1x its best-20%

wp_nh8k's best 20% of batches run at 3.31 us but its worst 20% at 30.01 us (9.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wp_spin** at 14532.5 ns median (-52.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.10x (fastest 14532.5 ns, slowest 30448.2 ns)

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
| wp_isb8 | 44949ns | 38254ns | 19753ns | 32418ns | 76343ns | base |
| wp_lp0 | 42480ns | 35885ns | 18763ns | 29600ns | 73659ns | -5.49% |
| wp_nh128 | 40170ns | 28510ns | 20063ns | 28397ns | 67882ns | -10.63% |
| wp_nh2k | 43889ns | 32367ns | 18672ns | 30750ns | 76207ns | -2.36% |
| wp_nh8k | 25530ns | 25371ns | 9968ns | 21385ns | 39529ns | -43.20% |
| wp_park | 39659ns | 25198ns | 18329ns | 26788ns | 69630ns | -11.77% |
| wp_spin | 28688ns | 22743ns | 14429ns | 21331ns | 46853ns | -36.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 36340ns | 13382ns | 66191ns | base | 0.002 |
| wp_lp0 | 32402ns | 12486ns | 60002ns | -10.84% | 0.002 |
| wp_nh128 | 31721ns | 13019ns | 57892ns | -12.71% | 0.002 |
| wp_nh2k | 35374ns | 12203ns | 66031ns | -2.66% | 0.002 |
| wp_nh8k | 17519ns | 3307ns | 30012ns | -51.79% | 0.004 |
| wp_park | 31642ns | 12301ns | 60244ns | -12.93% | 0.002 |
| wp_spin | 19587ns | 7178ns | 36932ns | -46.10% | 0.003 |

## Performance model

- Peak throughput: **0.019 Gops/s** (wp_nh8k; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.002 | 10.9% |
| wp_lp0 | 0.003 | 13.8% |
| wp_nh128 | 0.003 | 16.8% |
| wp_nh2k | 0.003 | 13.6% |
| wp_nh8k | 0.004 | 19.2% |
| wp_park | 0.004 | 20.1% |
| wp_spin | 0.004 | 22.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 44949ns | 44949ns | base |
| wp_lp0 | 42480ns | 42480ns | -5.49% |
| wp_nh128 | 40170ns | 40170ns | -10.63% |
| wp_nh2k | 43889ns | 43889ns | -2.36% |
| wp_nh8k | 25530ns | 25530ns | -43.20% |
| wp_park | 39659ns | 39659ns | -11.77% |
| wp_spin | 28688ns | 28688ns | -36.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 30448ns | base | --- | [14734, 48989] | --- | --- | --- | --- |
| wp_lp0 | 23887ns | no significant difference | [-15663, +8005]ns | [14024, 42328] | no | 1.0000 | 0.8145 | 0 |
| wp_nh128 | 19721ns | no significant difference | [-6956, +2417]ns | [15063, 46169] | no | 1.0000 | 1.0000 | 0 |
| wp_nh2k | 24341ns | no significant difference | [-8034, +6508]ns | [15598, 49229] | no | 1.0000 | 1.0000 | 0 |
| wp_nh8k | 17193ns | -11299.2ns (-37.1%) | [-31169, -5617]ns | [8139, 22298] | YES (adj: no) | 0.0927 | 0.0309 | 0 |
| wp_park | 16444ns | no significant difference | [-11948, +6541]ns | [13696, 47195] | no | 0.4758 | 0.2379 | 0 |
| wp_spin | 14533ns | -6757.2ns (-22.2%) | [-26762, -3786]ns | [8519, 24672] | YES (adj: no) | 0.0927 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 26572ns | -7.5% | -25.4% | +28.0% | -34.3% | -46.8% | -56.4% |
| 1 | 2 | 34325ns | -51.1% | -58.1% | -58.8% | -59.8% | -65.1% | -55.7% |
| 1 | 3 | 13638ns | -4.6% | +9.7% | +78.3% | +62.0% | +174.7% | +1.5% |
| 2 | 1 | 16657ns | +345.2% | -2.4% | +95.5% | -73.8% | -11.6% | -31.5% |
| 2 | 2 | 14871ns | -4.3% | +18.6% | -18.6% | +51.3% | -7.2% | +4.2% |
| 2 | 3 | 48305ns | -70.4% | -18.6% | -55.6% | -65.5% | -64.8% | -52.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | -0.144 | ok |
| wp_lp0 | -0.325 | moderate- |
| wp_nh128 | 0.040 | ok |
| wp_nh2k | -0.442 | moderate- |
| wp_nh8k | -0.715 | HIGH- (thermal bounce) |
| wp_park | -0.307 | moderate- |
| wp_spin | 0.063 | ok |

**Consistency summary:**

- **wp_lp0**: won 5/6, lost 1/6
- **wp_nh128**: won 4/6, lost 2/6
- **wp_nh2k**: won 3/6, lost 3/6
- **wp_nh8k**: won 4/6, lost 2/6
- **wp_park**: won 5/6, lost 1/6
- **wp_spin**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 252.4ns | 36340.1ns | 0.7% |  |
| wp_lp0 | 300.4ns | 32401.9ns | 0.9% |  |
| wp_nh128 | 230.1ns | 31720.8ns | 0.7% |  |
| wp_nh2k | 272.2ns | 35373.8ns | 0.8% |  |
| wp_nh8k | 196.9ns | 17518.8ns | 1.1% |  |
| wp_park | 203.1ns | 31641.8ns | 0.6% |  |
| wp_spin | 203.7ns | 19586.7ns | 1.0% |  |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 13381.9-66191.3 ns)
  13381.9 |########################################
  16022.4 |######
  18662.8 |
  21303.3 |
  23943.8 |######
  26584.3 |
  29224.7 |
  31865.2 |######
  34505.7 |######
  37146.1 |
  39786.6 |
  42427.1 |######
  45067.6 |
  47708.0 |#############
  50348.5 |######
  52989.0 |######
  55629.4 |
  58269.9 |
  60910.4 |
  63550.9 |
  (1 below, 2 above range)

wp_lp0 (n=18, range 12486.2-60001.5 ns)
  12486.2 |########################################
  14862.0 |######
  17237.8 |
  19613.5 |
  21989.3 |######
  24365.1 |######
  26740.8 |
  29116.6 |######
  31492.4 |
  33868.1 |
  36243.9 |######
  38619.6 |######
  40995.4 |
  43371.2 |######
  45746.9 |
  48122.7 |
  50498.5 |######
  52874.2 |######
  55250.0 |
  57625.8 |
  (1 below, 2 above range)

wp_nh128 (n=18, range 13019.0-57892.2 ns)
  13019.0 |########################################
  15262.7 |########
  17506.3 |################
  19750.0 |########
  21993.6 |
  24237.3 |
  26481.0 |########
  28724.6 |
  30968.3 |
  33211.9 |
  35455.6 |
  37699.3 |########
  39942.9 |
  42186.6 |########
  44430.2 |
  46673.9 |
  48917.5 |########
  51161.2 |
  53404.9 |########
  55648.5 |
  (1 below, 3 above range)

wp_nh2k (n=18, range 12203.2-66031.3 ns)
  12203.2 |########################################
  14894.6 |########################################
  17586.0 |
  20277.5 |########################################
  22968.9 |########################################
  25660.3 |
  28351.7 |
  31043.1 |####################
  33734.5 |####################
  36425.9 |
  39117.3 |
  41808.7 |
  44500.1 |
  47191.5 |####################
  49882.9 |####################
  52574.3 |
  55265.7 |
  57957.1 |####################
  60648.5 |####################
  63339.9 |
  (2 below, 2 above range)

wp_nh8k (n=18, range 3307.3-30011.5 ns)
   3307.3 |########################################
   4642.5 |#############
   5977.8 |
   7313.0 |
   8648.2 |
   9983.4 |#############
  11318.6 |
  12653.8 |#############
  13989.0 |
  15324.2 |#############
  16659.4 |##########################
  17994.7 |#############
  19329.9 |
  20665.1 |#############
  22000.3 |##########################
  23335.5 |
  24670.7 |#############
  26005.9 |
  27341.1 |
  28676.3 |
  (1 below, 3 above range)

wp_park (n=18, range 12300.9-60244.5 ns)
  12300.9 |########################################
  14698.1 |########################
  17095.3 |
  19492.5 |
  21889.6 |
  24286.8 |
  26684.0 |
  29081.2 |########
  31478.3 |
  33875.5 |
  36272.7 |########
  38669.9 |########
  41067.1 |
  43464.2 |
  45861.4 |
  48258.6 |
  50655.8 |
  53052.9 |
  55450.1 |################
  57847.3 |
  (2 below, 3 above range)

wp_spin (n=18, range 7177.6-36931.9 ns)
   7177.6 |########################################
   8665.3 |##########
  10153.0 |####################
  11640.8 |
  13128.5 |##########
  14616.2 |####################
  16103.9 |##########
  17591.6 |
  19079.3 |
  20567.0 |
  22054.8 |##########
  23542.5 |
  25030.2 |##########
  26517.9 |
  28005.6 |##########
  29493.3 |
  30981.1 |
  32468.8 |
  33956.5 |
  35444.2 |
  (1 below, 3 above range)

```

## Diagnostics

- **wp_isb8**: CV=74.7% (high variance, measurements may be unstable)
- **wp_isb8**: worst_20/best_20 = 4.9x (possible bimodal distribution)
- **wp_lp0**: CV=71.5% (high variance, measurements may be unstable)
- **wp_lp0**: worst_20/best_20 = 4.8x (possible bimodal distribution)
- **wp_nh128**: CV=63.9% (high variance, measurements may be unstable)
- **wp_nh128**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **wp_nh2k**: CV=70.1% (high variance, measurements may be unstable)
- **wp_nh2k**: worst_20/best_20 = 5.4x (possible bimodal distribution)
- **wp_nh8k**: CV=63.5% (high variance, measurements may be unstable)
- **wp_nh8k**: worst_20/best_20 = 9.1x (possible bimodal distribution)
- **wp_park**: CV=70.1% (high variance, measurements may be unstable)
- **wp_park**: worst_20/best_20 = 4.9x (possible bimodal distribution)
- **wp_spin**: CV=73.3% (high variance, measurements may be unstable)
- **wp_spin**: worst_20/best_20 = 5.1x (possible bimodal distribution)
