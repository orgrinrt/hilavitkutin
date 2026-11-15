# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wp_lp0 is fastest but the noisiest (CV 16.1%)

wp_lp0 wins on median (28.48 us) yet has the highest variance (CV 16.1%), while wp_isb8 is the steadiest (CV 5.5%, 30.41 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Speed leader wp_lp0 vs stability leader wp_isb8 (+7% speed for 2.9x steadier)

wp_lp0 is fastest (28.48 us, CV 16.1%); wp_isb8 gives up 6.8% median for 2.9x lower variance (CV 5.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: wp_lp0** at 28483.3 ns median (-6.3% vs baseline)
- 5 variants significantly slower than baseline
- Spread: 1.46x (fastest 28483.3 ns, slowest 41647.9 ns)

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
| wp_isb8 | 32732ns | 33083ns | 30226ns | 32367ns | 34532ns | base |
| wp_lp0 | 33239ns | 31004ns | 28728ns | 30863ns | 39058ns | +1.55% |
| wp_nh128 | 36684ns | 36548ns | 33229ns | 36080ns | 39317ns | +12.07% |
| wp_nh2k | 38272ns | 36840ns | 35429ns | 36351ns | 42574ns | +16.93% |
| wp_nh8k | 44001ns | 44690ns | 39300ns | 42218ns | 49028ns | +34.43% |
| wp_park | 36129ns | 35983ns | 30819ns | 34690ns | 40941ns | +10.38% |
| wp_spin | 38220ns | 37762ns | 34008ns | 36815ns | 42433ns | +16.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 30018ns | 27544ns | 31681ns | base | 0.273 |
| wp_lp0 | 30625ns | 26075ns | 36326ns | +2.02% | 0.267 |
| wp_nh128 | 34077ns | 30726ns | 36572ns | +13.52% | 0.240 |
| wp_nh2k | 35457ns | 32575ns | 39863ns | +18.12% | 0.231 |
| wp_nh8k | 41122ns | 36425ns | 46208ns | +36.99% | 0.199 |
| wp_park | 33445ns | 28321ns | 38264ns | +11.42% | 0.245 |
| wp_spin | 35394ns | 31253ns | 39658ns | +17.91% | 0.231 |

## Performance model

- Peak throughput: **0.314 Gops/s** (wp_lp0; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.269 | 85.7% |
| wp_lp0 | 0.288 | 91.5% |
| wp_nh128 | 0.240 | 76.5% |
| wp_nh2k | 0.242 | 76.9% |
| wp_nh8k | 0.197 | 62.6% |
| wp_park | 0.246 | 78.2% |
| wp_spin | 0.235 | 74.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 32732ns | 32732ns | base |
| wp_lp0 | 33239ns | 33239ns | +1.55% |
| wp_nh128 | 36684ns | 36684ns | +12.07% |
| wp_nh2k | 38272ns | 38272ns | +16.93% |
| wp_nh8k | 44001ns | 44001ns | +34.43% |
| wp_park | 36129ns | 36129ns | +10.38% |
| wp_spin | 38220ns | 38220ns | +16.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 30415ns | base | --- | [28862, 31267] | --- | --- | --- | --- |
| wp_lp0 | 28483ns | no significant difference | [-2144, +2937]ns | [28052, 32810] | no | 0.4807 | 0.4807 | 0 |
| wp_nh128 | 34088ns | +4385.4ns (+14.4%) | [+1979, +5960]ns | [32885, 35362] | YES | 0.0020 | 0.0013 | 0 |
| wp_nh2k | 33896ns | +4389.6ns (+14.4%) | [+3085, +6379]ns | [32708, 35817] | YES | 0.0000 | 0.0000 | 0 |
| wp_nh8k | 41648ns | +10462.5ns (+34.4%) | [+7735, +14048]ns | [37027, 44200] | YES | 0.0000 | 0.0000 | 0 |
| wp_park | 33342ns | +2506.2ns (+8.2%) | [+167, +5256]ns | [29304, 35375] | YES (adj: no) | 0.0589 | 0.0490 | 1 |
| wp_spin | 34883ns | +4570.8ns (+15.0%) | [+2642, +7356]ns | [32912, 37658] | YES | 0.0020 | 0.0013 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 31392ns | -10.1% | +6.3% | +6.7% | +19.1% | +9.9% | +5.0% |
| 1 | 2 | 31125ns | +26.3% | +1.9% | +26.0% | +35.3% | +4.5% | +20.3% |
| 1 | 3 | 28938ns | +11.0% | +18.3% | +25.5% | +27.4% | +47.1% | +23.8% |
| 2 | 1 | 28567ns | -0.9% | +27.3% | +14.1% | +27.3% | -0.3% | +15.0% |
| 2 | 2 | 31575ns | -9.6% | +9.5% | +8.8% | +40.0% | +1.1% | -0.2% |
| 2 | 3 | 29133ns | -4.2% | +22.7% | +11.9% | +63.3% | +18.6% | +38.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | -0.186 | ok |
| wp_lp0 | -0.015 | ok |
| wp_nh128 | 0.243 | moderate+ |
| wp_nh2k | -0.008 | ok |
| wp_nh8k | 0.147 | ok |
| wp_park | -0.453 | moderate- |
| wp_spin | -0.277 | moderate- |

**Consistency summary:**

- **wp_lp0**: won 4/6, lost 2/6
- **wp_nh128**: won 0/6, lost 6/6
- **wp_nh2k**: won 0/6, lost 6/6
- **wp_nh8k**: won 0/6, lost 6/6
- **wp_park**: won 1/6, lost 5/6
- **wp_spin**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 11792.6ns | 30018.1ns | 39.3% | HIGH |
| wp_lp0 | 11634.0ns | 30624.8ns | 38.0% | HIGH |
| wp_nh128 | 11166.0ns | 34077.3ns | 32.8% | HIGH |
| wp_nh2k | 11722.7ns | 35457.4ns | 33.1% | HIGH |
| wp_nh8k | 11894.0ns | 41121.8ns | 28.9% | HIGH |
| wp_park | 11463.9ns | 33444.7ns | 34.3% | HIGH |
| wp_spin | 11699.5ns | 35394.2ns | 33.1% | HIGH |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 27544.5-31680.6 ns)
  27544.5 |
  27751.3 |
  27958.1 |
  28164.9 |#############
  28371.7 |#############
  28578.5 |#############
  28785.3 |##########################
  28992.1 |##########################
  29198.9 |
  29405.7 |
  29612.5 |
  29819.3 |
  30026.1 |#############
  30232.9 |
  30439.7 |
  30646.5 |#############
  30853.3 |#############
  31060.2 |##########################
  31267.0 |#############
  31473.8 |########################################
  (1 below, 1 above range)

wp_lp0 (n=18, range 26075.0-36325.7 ns)
  26075.0 |########
  26587.5 |
  27100.1 |
  27612.6 |################################
  28125.1 |########################################
  28637.7 |########
  29150.2 |
  29662.7 |
  30175.3 |
  30687.8 |
  31200.3 |
  31712.9 |########
  32225.4 |
  32737.9 |
  33250.5 |########
  33763.0 |########
  34275.5 |
  34788.1 |
  35300.6 |
  35813.1 |
  (1 below, 3 above range)

wp_nh128 (n=18, range 30726.4-36572.2 ns)
  30726.4 |
  31018.7 |####################
  31311.0 |
  31603.3 |########################################
  31895.6 |
  32187.9 |####################
  32480.1 |
  32772.4 |
  33064.7 |####################
  33357.0 |########################################
  33649.3 |
  33941.6 |########################################
  34233.9 |####################
  34526.2 |####################
  34818.5 |
  35110.8 |####################
  35403.1 |
  35695.3 |####################
  35987.6 |####################
  36279.9 |########################################
  (1 below, 1 above range)

wp_nh2k (n=18, range 32575.0-39862.5 ns)
  32575.0 |########################################
  32939.4 |########
  33303.8 |################
  33668.1 |
  34032.5 |################
  34396.9 |########
  34761.3 |
  35125.6 |########
  35490.0 |
  35854.4 |
  36218.8 |########
  36583.1 |
  36947.5 |
  37311.9 |
  37676.3 |
  38040.7 |
  38405.0 |########
  38769.4 |########
  39133.8 |########
  39498.2 |
  (1 below, 1 above range)

wp_nh8k (n=18, range 36425.0-46208.3 ns)
  36425.0 |########################################
  36914.2 |##########################
  37403.4 |#############
  37892.5 |
  38381.7 |
  38870.9 |
  39360.0 |
  39849.2 |
  40338.4 |
  40827.5 |
  41316.7 |##########################
  41805.8 |#############
  42295.0 |#############
  42784.2 |
  43273.3 |
  43762.5 |#############
  44251.7 |#############
  44740.8 |
  45230.0 |#############
  45719.2 |
  (2 below, 3 above range)

wp_park (n=18, range 28320.8-38263.9 ns)
  28320.8 |########################################
  28818.0 |########################################
  29315.1 |####################
  29812.3 |
  30309.4 |
  30806.6 |
  31303.8 |
  31800.9 |####################
  32298.1 |########################################
  32795.2 |
  33292.4 |
  33789.5 |####################
  34286.7 |########################################
  34783.8 |####################
  35281.0 |
  35778.1 |########################################
  36275.3 |####################
  36772.4 |
  37269.6 |
  37766.7 |
  (1 below, 2 above range)

wp_spin (n=18, range 31252.8-39658.3 ns)
  31252.8 |##########################
  31673.1 |
  32093.4 |
  32513.6 |##########################
  32933.9 |########################################
  33354.2 |
  33774.5 |
  34194.7 |
  34615.0 |########################################
  35035.3 |
  35455.6 |#############
  35875.8 |
  36296.1 |
  36716.4 |
  37136.7 |#############
  37557.0 |##########################
  37977.2 |
  38397.5 |
  38817.8 |
  39238.1 |
  (1 below, 3 above range)

```

## Diagnostics

- **wp_isb8**: bridge=38.8% of algo (FFI overhead may distort results)
- **wp_lp0**: bridge=41.4% of algo (FFI overhead may distort results)
- **wp_nh128**: bridge=32.3% of algo (FFI overhead may distort results)
- **wp_nh2k**: bridge=34.9% of algo (FFI overhead may distort results)
- **wp_nh8k**: bridge=28.4% of algo (FFI overhead may distort results)
- **wp_park**: bridge=35.3% of algo (FFI overhead may distort results)
- **wp_spin**: bridge=33.9% of algo (FFI overhead may distort results)
