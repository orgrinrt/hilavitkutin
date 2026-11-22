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

The declared/defaulted baseline hand has the worst median (13.37 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest rust_std_par at 2.57 ms).


### rust_std_par dominates: 21% faster than the next best (hand_par)

rust_std_par (2.57 ms) leads hand_par (3.11 ms) by 21%, a clear separation rather than a photo finish. CV 4.5%.


### rust_std_par beats baseline by 81% (significant)

rust_std_par is -10.79 ms (81%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 5.2x slower than the field

hand (13.37 ms) is 5.2x the fastest (2.57 ms), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {rust_std_par, hand_par} vs {engine_par, rust_nostd, engine, rust_std, hand} (156% apart)

The field splits into a fast tier {rust_std_par, hand_par} and a slow tier {engine_par, rust_nostd, engine, rust_std, hand} with a 156% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 5.2x the fastest

Fastest rust_std_par (2.57 ms) to slowest hand (13.37 ms): 5.2x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: rust_std_par at 2573500.0 ns median (-80.7% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 5.19x (fastest 2573500.0 ns, slowest 13365479.2 ns)

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
| engine | 9366624ns | 9356833ns | 9337367ns | 9353828ns | 9383535ns | -30.00% |
| engine_par | 7994431ns | 7971233ns | 7953125ns | 7964331ns | 8027333ns | -40.25% |
| hand | 13380304ns | 13369575ns | 13355000ns | 13365019ns | 13398094ns | base |
| hand_par | 3182638ns | 3112517ns | 3094842ns | 3109078ns | 3259756ns | -76.21% |
| rust_nostd | 9353412ns | 9315779ns | 9283783ns | 9303747ns | 9408069ns | -30.10% |
| rust_std | 9479472ns | 9487133ns | 9391092ns | 9449069ns | 9524369ns | -29.15% |
| rust_std_par | 2623516ns | 2578108ns | 2550233ns | 2567272ns | 2684019ns | -80.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 9364358ns | 9334750ns | 9381302ns | -29.99% | 1.344 |
| engine_par | 7990698ns | 7950683ns | 8023217ns | -40.26% | 1.575 |
| hand | 13376341ns | 13352267ns | 13394065ns | base | 0.941 |
| hand_par | 3177741ns | 3089542ns | 3254971ns | -76.24% | 3.960 |
| rust_nostd | 9351494ns | 9281383ns | 9406260ns | -30.09% | 1.346 |
| rust_std | 9477206ns | 9388425ns | 9521988ns | -29.15% | 1.328 |
| rust_std_par | 2619885ns | 2546850ns | 2680698ns | -80.41% | 4.803 |

## Performance model

- Peak throughput: 4.941 Gops/s (rust_std_par; best 20% batches)
- Ops per call: 12582912

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 1.345 | 27.2% |
| engine_par | 1.579 | 32.0% |
| hand | 0.941 | 19.1% |
| hand_par | 4.049 | 82.0% |
| rust_nostd | 1.351 | 27.3% |
| rust_std | 1.327 | 26.9% |
| rust_std_par | 4.889 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 9366624ns | 9366624ns | -30.00% |
| engine_par | 7994431ns | 7994431ns | -40.25% |
| hand | 13380304ns | 13380304ns | base |
| hand_par | 3182638ns | 3182638ns | -76.21% |
| rust_nostd | 9353412ns | 9353412ns | -30.10% |
| rust_std | 9479472ns | 9479472ns | -29.15% |
| rust_std_par | 2623516ns | 2623516ns | -80.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 13365479ns | base | --- | [13359125, 13403925] | --- | --- | --- | --- |
| engine | 9354596ns | -4013120.8ns (-30.0%) | [-4038583, -3962300]ns | [9349542, 9396825] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 7966604ns | -5392875.0ns (-40.3%) | [-5416333, -5391350]ns | [7956383, 7977317] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 3107279ns | -10254395.8ns (-76.7%) | [-10294550, -10175092]ns | [3103033, 3191925] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 9314117ns | -4049783.3ns (-30.3%) | [-4101267, -3927492]ns | [9287283, 9436450] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 9484846ns | -3892166.7ns (-29.1%) | [-3945183, -3860450]ns | [9421833, 9535383] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 2573500ns | -10790429.2ns (-80.7%) | [-10817092, -10727625]ns | [2558367, 2631500] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 13363071ns | -29.9% | -40.4% | -76.5% | -30.2% | -29.4% | -80.7% |
| 1 | 2 | 13383933ns | -30.1% | -40.4% | -76.8% | -30.4% | -28.9% | -80.8% |

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
| engine | 16813987.5ns | 9364358.3ns | 179.6% | HIGH |
| engine_par | 16867999.0ns | 7990697.9ns | 211.1% | HIGH |
| hand | 16955135.4ns | 13376340.6ns | 126.8% | HIGH |
| hand_par | 16964271.9ns | 3177740.6ns | 533.8% | HIGH |
| rust_nostd | 16847679.2ns | 9351493.7ns | 180.2% | HIGH |
| rust_std | 16957997.9ns | 9477206.2ns | 178.9% | HIGH |
| rust_std_par | 16870911.5ns | 2619885.4ns | 644.0% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 9334750.0-9381302.1 ns)
  9334750.0 |####################
  9337077.6 |
  9339405.2 |
  9341732.8 |
  9344060.4 |
  9346388.0 |
  9348715.6 |####################
  9351043.2 |####################
  9353370.8 |########################################
  9355698.4 |
  9358026.1 |
  9360353.7 |
  9362681.3 |
  9365008.9 |####################
  9367336.5 |
  9369664.1 |
  9371991.7 |
  9374319.3 |
  9376646.9 |
  9378974.5 |
  (0 below, 2 above range)

engine_par (n=8, range 7950683.3-8023216.7 ns)
  7950683.3 |####################
  7954310.0 |####################
  7957936.6 |####################
  7961563.3 |
  7965190.0 |########################################
  7968816.6 |
  7972443.3 |
  7976070.0 |########################################
  7979696.6 |
  7983323.3 |
  7986950.0 |
  7990576.7 |
  7994203.3 |
  7997830.0 |
  8001456.7 |
  8005083.3 |
  8008710.0 |
  8012336.7 |
  8015963.3 |
  8019590.0 |
  (0 below, 1 above range)

hand (n=8, range 13352266.7-13394064.6 ns)
  13352266.7 |####################
  13354356.6 |
  13356446.5 |
  13358536.4 |########################################
  13360626.3 |
  13362716.2 |####################
  13364806.1 |
  13366896.0 |########################################
  13368985.9 |
  13371075.8 |
  13373165.6 |
  13375255.5 |
  13377345.4 |
  13379435.3 |
  13381525.2 |
  13383615.1 |
  13385705.0 |
  13387794.9 |
  13389884.8 |
  13391974.7 |
  (0 below, 2 above range)

hand_par (n=8, range 3089541.7-3254970.8 ns)
  3089541.7 |#############
  3097813.2 |########################################
  3106084.6 |#############
  3114356.1 |
  3122627.5 |
  3130899.0 |
  3139170.4 |
  3147441.9 |
  3155713.4 |
  3163984.8 |#############
  3172256.3 |
  3180527.7 |
  3188799.2 |#############
  3197070.6 |
  3205342.1 |
  3213613.5 |
  3221885.0 |
  3230156.5 |
  3238427.9 |
  3246699.4 |
  (0 below, 1 above range)

rust_nostd (n=8, range 9281383.3-9406260.4 ns)
  9281383.3 |########################################
  9287627.2 |
  9293871.0 |
  9300114.9 |####################
  9306358.7 |####################
  9312602.6 |####################
  9318846.4 |
  9325090.3 |
  9331334.2 |####################
  9337578.0 |
  9343821.9 |
  9350065.7 |
  9356309.6 |
  9362553.4 |
  9368797.3 |
  9375041.1 |
  9381285.0 |
  9387528.9 |
  9393772.7 |
  9400016.6 |
  (0 below, 2 above range)

rust_std (n=8, range 9388425.0-9521987.5 ns)
  9388425.0 |########################################
  9395103.1 |
  9401781.2 |
  9408459.4 |
  9415137.5 |
  9421815.6 |########################################
  9428493.8 |
  9435171.9 |########################################
  9441850.0 |
  9448528.1 |
  9455206.2 |
  9461884.4 |
  9468562.5 |
  9475240.6 |########################################
  9481918.8 |
  9488596.9 |########################################
  9495275.0 |
  9501953.1 |
  9508631.2 |
  9515309.4 |########################################
  (0 below, 2 above range)

rust_std_par (n=8, range 2546850.0-2680697.9 ns)
  2546850.0 |####################
  2553542.4 |########################################
  2560234.8 |
  2566927.2 |####################
  2573619.6 |####################
  2580312.0 |
  2587004.4 |
  2593696.8 |####################
  2600389.2 |
  2607081.6 |
  2613774.0 |
  2620466.4 |
  2627158.8 |####################
  2633851.2 |
  2640543.5 |
  2647235.9 |
  2653928.3 |
  2660620.7 |
  2667313.1 |
  2674005.5 |
  (0 below, 1 above range)

```

## Diagnostics

- engine: bridge=179.7% of algo (FFI overhead may distort results)
- engine_par: bridge=211.4% of algo (FFI overhead may distort results)
- hand: bridge=126.9% of algo (FFI overhead may distort results)
- hand_par: bridge=545.6% of algo (FFI overhead may distort results)
- rust_nostd: bridge=180.5% of algo (FFI overhead may distort results)
- rust_std: bridge=179.3% of algo (FFI overhead may distort results)
- rust_std_par: bridge=654.4% of algo (FFI overhead may distort results)
