## Before the numbers

Nothing in this file is a verdict on any arm, and no delta below is a reason to
use one arm over another.

The engine arms measure the engine without its headline features. Op, on 13
November: the engine arms measure something that "doesn't yet include any of
the actual headline features like fusing and the adaptive trunk/fiber/morsel
crunching on the datadlow-ish schedule. It's bare bones, barely lowers to
anything interesting yet. So no calls should be made based on those yet, they
are a baseline to compare against then." An engine number here is a baseline
for a later comparison and decides nothing.

Every delta below is taken against `hand`, the hand-written fused path, rather
than against an engine arm. The bench matrix names `hand` as the bar the engine
is measured against, so a row against it says how far an arm is from what a
person writes for this workload. Where the report says the baseline is the
slowest variant and reads as picked by accident, it was not: it is set in the
bench tree's hooks library, on purpose, and is meant to be the bar whether it
wins or loses. Do not re-baseline on whichever arm happens to be fastest.

The harness writes a `Why it matters` line under each highlight, calling a large
delta a reason to switch and a dominant winner a safe default pick. Those lines
are its stock phrasing for any dataset, they are judgements this file has no
standing to make, and they are stripped before it is written. What is left under
each highlight is the shape of the numbers.

A median here is wall-clock time and carries nothing about how many cores an
arm used to reach it. Two arms with the same median are not the same cost if
one of them ran on one core and the other on two. The occupancy of every
parallel arm is measured separately and stated beside these medians in
`mock/research/202611131400_frame-wide-first-run.md`. Read the two together or
neither.

The generated report follows.

# Wide frame, one multiply per stage

7 variants, 8 samples per variant.
Baseline: hand

## Highlights

Baseline for all deltas below: hand. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rust_std_par is an outlier: 5.0x slower than the field

rust_std_par (10.59 ms) is 5.0x the fastest (2.12 ms), well off the pack.


### hand shows alternating (throttle bounce) (autocorr -0.50)

hand's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (hand)

The baseline hand is the fastest (2.12 ms median); no rival improves on it (all deltas are >= 0).


### Two tiers: {hand, hand_par} vs {rust_std, engine_par, rust_nostd, engine, rust_std_par} (250% apart)

The field splits into a fast tier {hand, hand_par} and a slow tier {rust_std, engine_par, rust_nostd, engine, rust_std_par} with a 250% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 5.0x the fastest

Fastest hand (2.12 ms) to slowest rust_std_par (10.59 ms): 5.0x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (hand) is the fastest at 2118366.6 ns median
- 5 variants significantly slower than baseline
- Spread: 5.00x (fastest 2118366.6 ns, slowest 10587008.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide frame, one multiply per stage |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xf4a3e00000000002 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 8537851ns | 8532075ns | 8452317ns | 8488653ns | 8596133ns | +274.88% |
| engine_par | 8148415ns | 8131971ns | 8065550ns | 8101281ns | 8204481ns | +257.78% |
| hand | 2277491ns | 2121654ns | 1946100ns | 2033644ns | 2543223ns | base |
| hand_par | 2188698ns | 2181138ns | 2027742ns | 2143589ns | 2262769ns | -3.90% |
| rust_nostd | 8869947ns | 8167467ns | 7403733ns | 7730617ns | 10090998ns | +289.46% |
| rust_std | 7658503ns | 7621379ns | 7404542ns | 7579778ns | 7781038ns | +236.27% |
| rust_std_par | 10584180ns | 10589592ns | 10288583ns | 10520889ns | 10705548ns | +364.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 8535599ns | 8450200ns | 8594085ns | +275.29% | 5.897 |
| engine_par | 8145626ns | 8062858ns | 8201752ns | +258.14% | 6.179 |
| hand | 2274397ns | 1944475ns | 2539658ns | base | 22.130 |
| hand_par | 2185578ns | 2024550ns | 2259635ns | -3.91% | 23.029 |
| rust_nostd | 8867617ns | 7402108ns | 10088133ns | +289.89% | 5.676 |
| rust_std | 7656329ns | 7402817ns | 7778781ns | +236.63% | 6.574 |
| rust_std_par | 10581698ns | 10286350ns | 10703269ns | +365.25% | 4.756 |

## Performance model

- Peak throughput: 25.884 Gops/s (hand; best 20% batches)
- Ops per call: 50331648

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 5.901 | 22.8% |
| engine_par | 6.191 | 23.9% |
| hand | 23.760 | 91.8% |
| hand_par | 23.104 | 89.3% |
| rust_nostd | 6.164 | 23.8% |
| rust_std | 6.606 | 25.5% |
| rust_std_par | 4.754 | 18.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 8537851ns | 8537851ns | +274.88% |
| engine_par | 8148415ns | 8148415ns | +257.78% |
| hand | 2277491ns | 2277491ns | base |
| hand_par | 2188698ns | 2188698ns | -3.90% |
| rust_nostd | 8869947ns | 8869947ns | +289.46% |
| rust_std | 7658503ns | 7658503ns | +236.27% |
| rust_std_par | 10584180ns | 10584180ns | +364.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 2118367ns | base | --- | [1991792, 2769300] | --- | --- | --- | --- |
| engine | 8529342ns | +6401170.8ns (+302.2%) | [+5757500, +6593992]ns | [8464467, 8601883] | YES | 0.0094 | 0.0078 | 0 |
| engine_par | 8129246ns | +6002720.8ns (+283.4%) | [+5506550, +6143617]ns | [8092358, 8200333] | YES | 0.0094 | 0.0078 | 0 |
| hand_par | 2178438ns | no significant difference | [-497608, +193400]ns | [2081433, 2264108] | no | 1.0000 | 1.0000 | 0 |
| rust_nostd | 8165604ns | +6009241.7ns (+283.7%) | [+5473650, +7977600]ns | [7534475, 10069983] | YES | 0.0094 | 0.0078 | 0 |
| rust_std | 7619088ns | +5435070.8ns (+256.6%) | [+5122608, +5715592]ns | [7530350, 7807975] | YES | 0.0094 | 0.0078 | 0 |
| rust_std_par | 10587008ns | +8474425.0ns (+400.0%) | [+7762942, +8690425]ns | [10482800, 10675767] | YES | 0.0094 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2568775ns | +230.7% | +218.8% | -14.7% | +290.3% | +200.6% | +309.0% |
| 1 | 2 | 1999842ns | +329.4% | +305.0% | +8.8% | +277.1% | +280.1% | +433.1% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | HIGH- (thermal bounce) |
| engine_par | -0.500 | moderate- |
| hand | -0.500 | HIGH- (thermal bounce) |
| hand_par | -0.500 | moderate- |
| rust_nostd | -0.500 | moderate- |
| rust_std | -0.500 | moderate- |
| rust_std_par | -0.500 | moderate- |

Consistency summary:

- engine: won 0/2, lost 2/2
- engine_par: won 0/2, lost 2/2
- hand_par: won 1/2, lost 1/2
- rust_nostd: won 0/2, lost 2/2
- rust_std: won 0/2, lost 2/2
- rust_std_par: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 67742732.3ns | 8535598.9ns | 793.6% | HIGH |
| engine_par | 67825741.7ns | 8145626.0ns | 832.7% | HIGH |
| hand | 69049086.5ns | 2274396.9ns | 3035.9% | HIGH |
| hand_par | 70343451.0ns | 2185578.1ns | 3218.5% | HIGH |
| rust_nostd | 68487527.1ns | 8867616.7ns | 772.3% | HIGH |
| rust_std | 67839034.4ns | 7656329.2ns | 886.1% | HIGH |
| rust_std_par | 67590914.6ns | 10581697.9ns | 638.8% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 8450200.0-8594085.4 ns)
  8450200.0 |########################################
  8457394.3 |########################################
  8464588.5 |########################################
  8471782.8 |
  8478977.1 |
  8486171.3 |
  8493365.6 |
  8500559.9 |
  8507754.2 |
  8514948.4 |
  8522142.7 |########################################
  8529337.0 |########################################
  8536531.2 |
  8543725.5 |
  8550919.8 |
  8558114.1 |
  8565308.3 |
  8572502.6 |########################################
  8579696.9 |
  8586891.1 |
  (0 below, 2 above range)

engine_par (n=8, range 8062858.3-8201752.1 ns)
  8062858.3 |####################
  8069803.0 |
  8076747.7 |
  8083692.4 |
  8090637.1 |########################################
  8097581.7 |
  8104526.4 |####################
  8111471.1 |
  8118415.8 |
  8125360.5 |
  8132305.2 |
  8139249.9 |
  8146194.6 |####################
  8153139.3 |
  8160083.9 |
  8167028.6 |
  8173973.3 |####################
  8180918.0 |
  8187862.7 |
  8194807.4 |####################
  (0 below, 1 above range)

hand (n=8, range 1944475.0-2539658.3 ns)
  1944475.0 |########################################
  1974234.2 |########################################
  2003993.3 |########################################
  2033752.5 |
  2063511.7 |########################################
  2093270.8 |
  2123030.0 |########################################
  2152789.2 |
  2182548.3 |
  2212307.5 |
  2242066.7 |
  2271825.8 |
  2301585.0 |
  2331344.2 |
  2361103.3 |########################################
  2390862.5 |
  2420621.7 |
  2450380.8 |
  2480140.0 |
  2509899.2 |
  (0 below, 2 above range)

hand_par (n=8, range 2024550.0-2259635.4 ns)
  2024550.0 |####################
  2036304.3 |
  2048058.5 |
  2059812.8 |
  2071567.1 |####################
  2083321.4 |
  2095075.6 |
  2106829.9 |
  2118584.2 |
  2130338.4 |
  2142092.7 |
  2153847.0 |
  2165601.3 |########################################
  2177355.5 |####################
  2189109.8 |
  2200864.1 |####################
  2212618.3 |
  2224372.6 |
  2236126.9 |
  2247881.2 |
  (0 below, 2 above range)

rust_nostd (n=8, range 7402108.3-10088133.3 ns)
  7402108.3 |########################################
  7536409.6 |####################
  7670710.8 |
  7805012.1 |
  7939313.3 |
  8073614.6 |####################
  8207915.8 |####################
  8342217.1 |
  8476518.3 |
  8610819.6 |
  8745120.8 |
  8879422.1 |
  9013723.3 |
  9148024.6 |
  9282325.8 |
  9416627.1 |
  9550928.3 |
  9685229.6 |
  9819530.8 |
  9953832.1 |########################################
  (0 below, 1 above range)

rust_std (n=8, range 7402816.7-7778781.2 ns)
  7402816.7 |####################
  7421614.9 |
  7440413.2 |
  7459211.4 |
  7478009.6 |
  7496807.8 |
  7515606.1 |####################
  7534404.3 |
  7553202.5 |
  7572000.7 |
  7590799.0 |########################################
  7609597.2 |
  7628395.4 |####################
  7647193.7 |
  7665991.9 |####################
  7684790.1 |
  7703588.3 |
  7722386.6 |
  7741184.8 |
  7759983.0 |
  (0 below, 2 above range)

rust_std_par (n=8, range 10286350.0-10703268.8 ns)
  10286350.0 |########################################
  10307195.9 |
  10328041.9 |
  10348887.8 |
  10369733.8 |
  10390579.7 |
  10411425.6 |
  10432271.6 |
  10453117.5 |
  10473963.4 |########################################
  10494809.4 |
  10515655.3 |########################################
  10536501.3 |########################################
  10557347.2 |
  10578193.1 |
  10599039.1 |
  10619885.0 |########################################
  10640731.0 |########################################
  10661576.9 |########################################
  10682422.8 |
  (0 below, 1 above range)

```

## Diagnostics

- engine: bridge=794.3% of algo (FFI overhead may distort results)
- engine_par: bridge=832.9% of algo (FFI overhead may distort results)
- hand: bridge=3246.1% of algo (FFI overhead may distort results)
- hand_par: bridge=3173.1% of algo (FFI overhead may distort results)
- rust_nostd: bridge=832.4% of algo (FFI overhead may distort results)
- rust_std: bridge=888.8% of algo (FFI overhead may distort results)
- rust_std_par: bridge=638.2% of algo (FFI overhead may distort results)
