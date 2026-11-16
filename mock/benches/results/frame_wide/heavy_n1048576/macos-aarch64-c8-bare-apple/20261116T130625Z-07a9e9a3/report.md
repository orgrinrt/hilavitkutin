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

# Wide frame, heavy per-record compute

7 variants, 8 samples per variant.
Baseline: hand

## Highlights

Baseline for all deltas below: hand. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hand) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hand has the worst median (13.39 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest rust_std_par at 2.56 ms).


### rust_std_par dominates: 22% faster than the next best (hand_par)

rust_std_par (2.56 ms) leads hand_par (3.11 ms) by 22%, a clear separation rather than a photo finish. CV 1.8%.


### rust_std_par beats baseline by 81% (significant)

rust_std_par is -10.84 ms (81%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 5.2x slower than the field

hand (13.39 ms) is 5.2x the fastest (2.56 ms), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {rust_std_par, hand_par} vs {engine_par, rust_nostd, engine, rust_std, hand} (156% apart)

The field splits into a fast tier {rust_std_par, hand_par} and a slow tier {engine_par, rust_nostd, engine, rust_std, hand} with a 156% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 5.2x the fastest

Fastest rust_std_par (2.56 ms) to slowest hand (13.39 ms): 5.2x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: rust_std_par at 2556179.2 ns median (-80.9% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 5.24x (fastest 2556179.2 ns, slowest 13394641.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide frame, heavy per-record compute |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xf4a3e00000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 9432223ns | 9370471ns | 9343633ns | 9360308ns | 9508306ns | -29.85% |
| engine_par | 7973042ns | 7960742ns | 7947942ns | 7955569ns | 7992421ns | -40.71% |
| hand | 13446445ns | 13398012ns | 13348033ns | 13370544ns | 13527973ns | base |
| hand_par | 3129950ns | 3116288ns | 3089658ns | 3099336ns | 3162983ns | -76.72% |
| rust_nostd | 9300592ns | 9304212ns | 9281775ns | 9296833ns | 9308115ns | -30.83% |
| rust_std | 9400313ns | 9401575ns | 9382550ns | 9397047ns | 9407202ns | -30.09% |
| rust_std_par | 2577661ns | 2560279ns | 2545558ns | 2557244ns | 2601000ns | -80.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 9430422ns | 9341650ns | 9506696ns | -29.85% | 1.334 |
| engine_par | 7969175ns | 7945417ns | 7988338ns | -40.72% | 1.579 |
| hand | 13443456ns | 13345033ns | 13526019ns | base | 0.936 |
| hand_par | 3124733ns | 3086650ns | 3158079ns | -76.76% | 4.027 |
| rust_nostd | 9298275ns | 9279308ns | 9305727ns | -30.83% | 1.353 |
| rust_std | 9398300ns | 9380308ns | 9405319ns | -30.09% | 1.339 |
| rust_std_par | 2573519ns | 2541642ns | 2597006ns | -80.86% | 4.889 |

## Performance model

- Peak throughput: 4.951 Gops/s (rust_std_par; best 20% batches)
- Ops per call: 12582912

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 1.343 | 27.1% |
| engine_par | 1.581 | 31.9% |
| hand | 0.939 | 19.0% |
| hand_par | 4.046 | 81.7% |
| rust_nostd | 1.353 | 27.3% |
| rust_std | 1.339 | 27.0% |
| rust_std_par | 4.923 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 9432223ns | 9432223ns | -29.85% |
| engine_par | 7973042ns | 7973042ns | -40.71% |
| hand | 13446445ns | 13446445ns | base |
| hand_par | 3129950ns | 3129950ns | -76.72% |
| rust_nostd | 9300592ns | 9300592ns | -30.83% |
| rust_std | 9400313ns | 9400313ns | -30.09% |
| rust_std_par | 2577661ns | 2577661ns | -80.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 13394642ns | base | --- | [13355275, 13493725] | --- | --- | --- | --- |
| engine | 9368754ns | -4034570.8ns (-30.1%) | [-4098875, -3756692]ns | [9354258, 9600983] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 7957721ns | -5425712.5ns (-40.5%) | [-5514558, -5402775]ns | [7946058, 7960392] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 3109838ns | -10273162.5ns (-76.7%) | [-10397442, -10224442]ns | [3087400, 3132725] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 9301788ns | -4097070.8ns (-30.6%) | [-4192142, -4050400]ns | [9288467, 9306675] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 9399838ns | -3999370.8ns (-29.9%) | [-4094867, -3947858]ns | [9388358, 9407417] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 2556179ns | -10842258.3ns (-80.9%) | [-10940008, -10786392]ns | [2552208, 2577808] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 13371633ns | -29.1% | -40.5% | -76.6% | -30.4% | -29.7% | -80.9% |
| 1 | 2 | 13484338ns | -30.5% | -41.0% | -77.0% | -31.0% | -30.3% | -81.0% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| hand | -0.500 | moderate- |
| hand_par | -0.500 | moderate- |
| rust_nostd | -0.500 | moderate- |
| rust_std | -0.500 | moderate- |
| rust_std_par | -0.500 | moderate- |

Consistency summary:

- engine: won 2/2, lost 0/2
- engine_par: won 2/2, lost 0/2
- hand_par: won 2/2, lost 0/2
- rust_nostd: won 2/2, lost 0/2
- rust_std: won 2/2, lost 0/2
- rust_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 16901827.1ns | 9430421.9ns | 179.2% | HIGH |
| engine_par | 16867207.3ns | 7969175.0ns | 211.7% | HIGH |
| hand | 16965093.8ns | 13443456.2ns | 126.2% | HIGH |
| hand_par | 16923364.6ns | 3124733.3ns | 541.6% | HIGH |
| rust_nostd | 16818235.4ns | 9298275.0ns | 180.9% | HIGH |
| rust_std | 16819433.3ns | 9398300.0ns | 179.0% | HIGH |
| rust_std_par | 16845353.1ns | 2573518.8ns | 654.6% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 9341650.0-9506695.8 ns)
  9341650.0 |####################
  9349902.3 |####################
  9358154.6 |########################################
  9366406.9 |
  9374659.2 |####################
  9382911.5 |
  9391163.7 |
  9399416.0 |
  9407668.3 |
  9415920.6 |
  9424172.9 |####################
  9432425.2 |
  9440677.5 |
  9448929.8 |
  9457182.1 |
  9465434.4 |
  9473686.7 |
  9481939.0 |
  9490191.2 |
  9498443.5 |
  (0 below, 2 above range)

engine_par (n=8, range 7945416.7-7988337.5 ns)
  7945416.7 |##########################
  7947562.7 |
  7949708.8 |
  7951854.8 |#############
  7954000.9 |#############
  7956146.9 |
  7958292.9 |########################################
  7960439.0 |
  7962585.0 |
  7964731.1 |
  7966877.1 |
  7969023.2 |
  7971169.2 |
  7973315.2 |
  7975461.3 |
  7977607.3 |
  7979753.4 |
  7981899.4 |
  7984045.4 |
  7986191.5 |
  (0 below, 1 above range)

hand (n=8, range 13345033.3-13526018.8 ns)
  13345033.3 |########################################
  13354082.6 |########################################
  13363131.8 |########################################
  13372181.1 |########################################
  13381230.4 |
  13390279.7 |
  13399328.9 |
  13408378.2 |########################################
  13417427.5 |
  13426476.8 |
  13435526.0 |
  13444575.3 |
  13453624.6 |
  13462673.8 |
  13471723.1 |########################################
  13480772.4 |
  13489821.7 |########################################
  13498870.9 |
  13507920.2 |
  13516969.5 |
  (0 below, 1 above range)

hand_par (n=8, range 3086650.0-3158079.2 ns)
  3086650.0 |########################################
  3090221.5 |
  3093792.9 |########################################
  3097364.4 |
  3100935.8 |
  3104507.3 |
  3108078.8 |
  3111650.2 |
  3115221.7 |
  3118793.1 |
  3122364.6 |########################################
  3125936.0 |
  3129507.5 |####################
  3133079.0 |
  3136650.4 |
  3140221.9 |
  3143793.3 |
  3147364.8 |
  3150936.3 |
  3154507.7 |
  (0 below, 1 above range)

rust_nostd (n=8, range 9279308.3-9305727.1 ns)
  9279308.3 |########################################
  9280629.2 |
  9281950.2 |
  9283271.1 |
  9284592.1 |
  9285913.0 |
  9287233.9 |########################################
  9288554.9 |
  9289875.8 |
  9291196.8 |
  9292517.7 |
  9293838.6 |########################################
  9295159.6 |
  9296480.5 |
  9297801.5 |
  9299122.4 |
  9300443.3 |########################################
  9301764.3 |########################################
  9303085.2 |
  9304406.2 |########################################
  (0 below, 2 above range)

rust_std (n=8, range 9380308.3-9405318.8 ns)
  9380308.3 |########################################
  9381558.8 |
  9382809.3 |
  9384059.9 |
  9385310.4 |
  9386560.9 |
  9387811.4 |########################################
  9389062.0 |
  9390312.5 |
  9391563.0 |
  9392813.5 |
  9394064.0 |
  9395314.6 |
  9396565.1 |########################################
  9397815.6 |########################################
  9399066.1 |
  9400316.7 |########################################
  9401567.2 |########################################
  9402817.7 |
  9404068.2 |
  (0 below, 2 above range)

rust_std_par (n=8, range 2541641.7-2597006.2 ns)
  2541641.7 |####################
  2544409.9 |
  2547178.2 |
  2549946.4 |########################################
  2552714.6 |####################
  2555482.8 |
  2558251.1 |########################################
  2561019.3 |
  2563787.5 |
  2566555.7 |
  2569324.0 |
  2572092.2 |
  2574860.4 |
  2577628.7 |####################
  2580396.9 |
  2583165.1 |
  2585933.3 |
  2588701.6 |
  2591469.8 |
  2594238.0 |
  (0 below, 1 above range)

```

## Diagnostics

- engine: bridge=179.8% of algo (FFI overhead may distort results)
- engine_par: bridge=211.6% of algo (FFI overhead may distort results)
- hand: bridge=126.5% of algo (FFI overhead may distort results)
- hand_par: bridge=544.3% of algo (FFI overhead may distort results)
- rust_nostd: bridge=180.8% of algo (FFI overhead may distort results)
- rust_std: bridge=179.0% of algo (FFI overhead may distort results)
- rust_std_par: bridge=658.8% of algo (FFI overhead may distort results)
