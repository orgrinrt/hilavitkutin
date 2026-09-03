# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Key findings

- **Fastest: wp_park** at 10045.8 ns median (-11.7% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.75x (fastest 10045.8 ns, slowest 17585.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wp_isb8 | 14647ns | 14381ns | 10697ns | 14088ns | 17459ns | base |
| wp_lp0 | 15412ns | 12804ns | 10936ns | 12568ns | 21915ns | +5.23% |
| wp_nh128 | 15278ns | 13538ns | 12172ns | 13260ns | 19859ns | +4.31% |
| wp_nh2k | 17381ns | 15140ns | 11403ns | 14897ns | 24098ns | +18.67% |
| wp_nh8k | 20017ns | 20385ns | 15724ns | 19305ns | 23233ns | +36.67% |
| wp_park | 13290ns | 12781ns | 11218ns | 12511ns | 15495ns | -9.26% |
| wp_spin | 18921ns | 17071ns | 14147ns | 16381ns | 25118ns | +29.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 11805ns | 8019ns | 14597ns | base | 0.087 |
| wp_lp0 | 12592ns | 8446ns | 18760ns | +6.66% | 0.081 |
| wp_nh128 | 12455ns | 9389ns | 17053ns | +5.51% | 0.082 |
| wp_nh2k | 14603ns | 8704ns | 21106ns | +23.70% | 0.070 |
| wp_nh8k | 17228ns | 13138ns | 20286ns | +45.94% | 0.059 |
| wp_park | 10618ns | 8860ns | 12694ns | -10.06% | 0.096 |
| wp_spin | 15949ns | 11654ns | 21824ns | +35.10% | 0.064 |

## Performance model

- Peak throughput: **0.128 Gops/s** (wp_isb8; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.090 | 70.5% |
| wp_lp0 | 0.101 | 79.3% |
| wp_nh128 | 0.097 | 75.8% |
| wp_nh2k | 0.082 | 64.3% |
| wp_nh8k | 0.058 | 45.6% |
| wp_park | 0.102 | 79.8% |
| wp_spin | 0.073 | 57.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 14647ns | 14647ns | base |
| wp_lp0 | 15412ns | 15412ns | +5.23% |
| wp_nh128 | 15278ns | 15278ns | +4.31% |
| wp_nh2k | 17381ns | 17381ns | +18.67% |
| wp_nh8k | 20017ns | 20017ns | +36.67% |
| wp_park | 13290ns | 13290ns | -9.26% |
| wp_spin | 18921ns | 18921ns | +29.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 11371ns | base | --- | [10721, 12469] | --- | --- | --- | --- |
| wp_lp0 | 10119ns | no significant difference | [-2044, +2185]ns | [9479, 15540] | no | 0.5768 | 0.4807 | 0 |
| wp_nh128 | 10585ns | no significant difference | [-1094, +1610]ns | [10117, 11771] | no | 0.8145 | 0.8145 | 0 |
| wp_nh2k | 12465ns | no significant difference | [-148, +3546]ns | [11829, 14267] | no | 0.2888 | 0.0963 | 0 |
| wp_nh8k | 17585ns | +5443.8ns (+47.9%) | [+3308, +7200]ns | [15625, 18665] | YES | 0.0000 | 0.0000 | 0 |
| wp_park | 10046ns | no significant difference | [-3396, +800]ns | [9500, 10656] | no | 0.4758 | 0.2379 | 0 |
| wp_spin | 13954ns | no significant difference | [-388, +6419]ns | [12696, 17181] | no | 0.5768 | 0.4807 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|
| 1 | 8025ns | +29.5% | +18.8% | +29.1% | +93.4% | +21.0% | +176.1% |
| 2 | 8025ns | +23.2% | +37.3% | +1.2% | +59.2% | +10.2% | +203.9% |
| 3 | 8008ns | +25.0% | +25.7% | -4.7% | +72.5% | +9.8% | +206.6% |
| 4 | 10779ns | -16.4% | -0.9% | +39.1% | +46.5% | -4.0% | +136.5% |
| 5 | 10862ns | -22.6% | -1.3% | +93.9% | +102.3% | -5.3% | +60.6% |
| 6 | 10662ns | -22.0% | -1.7% | +27.0% | +72.8% | -0.7% | +58.6% |
| 7 | 10562ns | -18.5% | -4.2% | +15.5% | +48.1% | +55.5% | +15.3% |
| 8 | 12021ns | -21.1% | +1.9% | -3.3% | +18.2% | +30.0% | -3.1% |
| 9 | 10862ns | -14.3% | +6.1% | +7.1% | +18.0% | +9.2% | +8.7% |
| 10 | 11650ns | -8.3% | -14.6% | +6.4% | +58.0% | -6.2% | +22.2% |
| 11 | 16271ns | +33.5% | -43.3% | -26.1% | +16.1% | -41.3% | -17.0% |
| 12 | 11092ns | +91.7% | -15.3% | +8.5% | +63.5% | -9.1% | +26.3% |
| 13 | 12512ns | +75.2% | -3.9% | +0.3% | +26.4% | -14.3% | +11.1% |
| 14 | 16638ns | -0.3% | -37.5% | -21.2% | +2.4% | -39.8% | -13.7% |
| 15 | 16142ns | -4.1% | -36.4% | -22.4% | +14.3% | -38.7% | -15.7% |
| 16 | 12425ns | +25.5% | +72.6% | +98.3% | +77.5% | -28.0% | -1.8% |
| 17 | 12358ns | -17.2% | +75.4% | +122.7% | +60.5% | -26.8% | -6.9% |
| 18 | 13596ns | -28.2% | +72.1% | +83.0% | +51.0% | -30.5% | -2.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | 0.516 | HIGH+ (drift/warm-up) |
| wp_lp0 | 0.752 | HIGH+ (drift/warm-up) |
| wp_nh128 | 0.607 | HIGH+ (drift/warm-up) |
| wp_nh2k | 0.617 | HIGH+ (drift/warm-up) |
| wp_nh8k | 0.474 | moderate+ |
| wp_park | 0.613 | HIGH+ (drift/warm-up) |
| wp_spin | 0.807 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wp_lp0**: won 11/18, lost 7/18
- **wp_nh128**: won 10/18, lost 8/18
- **wp_nh2k**: won 5/18, lost 13/18
- **wp_nh8k**: won 0/18, lost 18/18
- **wp_park**: won 12/18, lost 6/18
- **wp_spin**: won 7/18, lost 11/18

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 1545.8ns | 11805.1ns | 13.1% | HIGH |
| wp_lp0 | 1563.0ns | 12591.9ns | 12.4% | HIGH |
| wp_nh128 | 1558.8ns | 12455.3ns | 12.5% | HIGH |
| wp_nh2k | 1465.0ns | 14603.2ns | 10.0% | HIGH |
| wp_nh8k | 1493.1ns | 17228.2ns | 8.7% | HIGH |
| wp_park | 1468.5ns | 10618.1ns | 13.8% | HIGH |
| wp_spin | 1521.8ns | 15948.8ns | 9.5% | HIGH |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 8019.4-14597.2 ns)
   8019.4 |####################
   8348.3 |
   8677.2 |
   9006.1 |
   9335.0 |
   9663.9 |
   9992.8 |
  10321.7 |##########
  10650.5 |########################################
  10979.4 |##########
  11308.3 |
  11637.2 |##########
  11966.1 |##########
  12295.0 |##############################
  12623.9 |
  12952.8 |
  13281.7 |##########
  13610.5 |
  13939.4 |
  14268.3 |
  (1 below, 3 above range)

wp_lp0 (n=18, range 8445.8-18760.4 ns)
   8445.8 |#############
   8961.6 |##########################
   9477.3 |########################################
   9993.0 |########################################
  10508.7 |#############
  11024.5 |
  11540.2 |
  12055.9 |
  12571.7 |
  13087.4 |
  13603.1 |
  14118.8 |
  14634.6 |
  15150.3 |##########################
  15666.0 |
  16181.8 |#############
  16697.5 |
  17213.2 |
  17728.9 |
  18244.7 |
  (2 below, 3 above range)

wp_nh128 (n=18, range 9388.9-17052.8 ns)
   9388.9 |##########################
   9772.1 |########################################
  10155.3 |########################################
  10538.5 |##########################
  10921.6 |#############
  11304.8 |#############
  11688.0 |#############
  12071.2 |#############
  12454.4 |
  12837.6 |
  13220.8 |
  13604.0 |
  13987.2 |
  14370.4 |
  14753.6 |
  15136.8 |
  15520.0 |
  15903.2 |
  16286.4 |
  16669.6 |
  (1 below, 3 above range)

wp_nh2k (n=18, range 8704.2-21105.5 ns)
   8704.2 |
   9324.2 |
   9944.3 |##########
  10564.4 |
  11184.4 |####################
  11804.5 |########################################
  12424.6 |####################
  13044.6 |####################
  13664.7 |
  14284.8 |
  14904.9 |##########
  15524.9 |
  16145.0 |
  16765.1 |
  17385.1 |
  18005.2 |
  18625.3 |
  19245.3 |
  19865.4 |
  20485.5 |##########
  (2 below, 3 above range)

wp_nh8k (n=18, range 13137.5-20286.1 ns)
  13137.5 |
  13494.9 |#############
  13852.4 |
  14209.8 |#############
  14567.2 |
  14924.7 |
  15282.1 |#############
  15639.5 |########################################
  15996.9 |
  16354.4 |
  16711.8 |#############
  17069.2 |
  17426.7 |
  17784.1 |#############
  18141.5 |########################################
  18499.0 |
  18856.4 |#############
  19213.8 |
  19571.3 |#############
  19928.7 |
  (2 below, 3 above range)

wp_park (n=18, range 8859.7-12694.4 ns)
   8859.7 |########################################
   9051.5 |
   9243.2 |
   9434.9 |########################################
   9626.7 |####################
   9818.4 |########################################
  10010.1 |####################
  10201.9 |########################################
  10393.6 |####################
  10585.4 |####################
  10777.1 |####################
  10968.8 |
  11160.6 |
  11352.3 |
  11544.0 |
  11735.8 |####################
  11927.5 |
  12119.2 |
  12311.0 |
  12502.7 |
  (2 below, 2 above range)

wp_spin (n=18, range 11654.2-21824.3 ns)
  11654.2 |#############
  12162.7 |##########################
  12671.2 |
  13179.7 |########################################
  13688.2 |##########################
  14196.7 |##########################
  14705.2 |
  15213.7 |
  15722.2 |
  16230.7 |
  16739.2 |#############
  17247.7 |#############
  17756.3 |
  18264.8 |
  18773.3 |
  19281.8 |
  19790.3 |
  20298.8 |
  20807.3 |
  21315.8 |
  (2 below, 4 above range)

```

## Diagnostics

- **wp_isb8**: CV=21.5% (high variance, measurements may be unstable)
- **wp_isb8**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **wp_isb8**: bridge=13.8% of algo (FFI overhead may distort results)
- **wp_lp0**: CV=37.4% (high variance, measurements may be unstable)
- **wp_lp0**: autocorrelation=0.75 (measurement drift or warm-up artifact)
- **wp_lp0**: bridge=14.9% of algo (FFI overhead may distort results)
- **wp_nh128**: CV=35.6% (high variance, measurements may be unstable)
- **wp_nh128**: autocorrelation=0.61 (measurement drift or warm-up artifact)
- **wp_nh128**: bridge=14.0% of algo (FFI overhead may distort results)
- **wp_nh2k**: CV=38.8% (high variance, measurements may be unstable)
- **wp_nh2k**: autocorrelation=0.62 (measurement drift or warm-up artifact)
- **wp_nh2k**: bridge=11.4% of algo (FFI overhead may distort results)
- **wp_park**: autocorrelation=0.61 (measurement drift or warm-up artifact)
- **wp_park**: bridge=14.8% of algo (FFI overhead may distort results)
- **wp_spin**: CV=29.3% (high variance, measurements may be unstable)
- **wp_spin**: autocorrelation=0.81 (measurement drift or warm-up artifact)
- **wp_spin**: bridge=11.3% of algo (FFI overhead may distort results)
