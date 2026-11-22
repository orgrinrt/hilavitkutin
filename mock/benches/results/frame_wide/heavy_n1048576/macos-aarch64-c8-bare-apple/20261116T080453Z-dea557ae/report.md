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

The declared/defaulted baseline hand has the worst median (13.69 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest rust_std_par at 3.06 ms).


### rust_std_par dominates: 15% faster than the next best (hand_par)

rust_std_par (3.06 ms) leads hand_par (3.51 ms) by 15%, a clear separation rather than a photo finish. CV 4.3%.


### rust_std_par beats baseline by 78% (significant)

rust_std_par is -10.70 ms (78%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 4.5x slower than the field

hand (13.69 ms) is 4.5x the fastest (3.06 ms), well off the pack.


### hand_par shows alternating (throttle bounce) (autocorr -0.50)

hand_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {rust_std_par, hand_par} vs {engine_par, rust_nostd, rust_std, engine, hand} (137% apart)

The field splits into a fast tier {rust_std_par, hand_par} and a slow tier {engine_par, rust_nostd, rust_std, engine, hand} with a 137% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 4.5x the fastest

Fastest rust_std_par (3.06 ms) to slowest hand (13.69 ms): 4.5x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: rust_std_par at 3061800.0 ns median (-77.6% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 4.47x (fastest 3061800.0 ns, slowest 13689929.2 ns)

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
| engine | 9753603ns | 9735725ns | 9628917ns | 9681839ns | 9838598ns | -29.23% |
| engine_par | 8323957ns | 8326688ns | 8220683ns | 8276978ns | 8385010ns | -39.60% |
| hand | 13781772ns | 13692808ns | 13663567ns | 13675519ns | 13891012ns | base |
| hand_par | 3501221ns | 3509392ns | 3390775ns | 3447764ns | 3568925ns | -74.60% |
| rust_nostd | 9604183ns | 9568371ns | 9536825ns | 9546475ns | 9664304ns | -30.31% |
| rust_std | 9676301ns | 9679367ns | 9600550ns | 9646828ns | 9717344ns | -29.79% |
| rust_std_par | 3031604ns | 3064746ns | 2836333ns | 2943397ns | 3146577ns | -78.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 9751379ns | 9626333ns | 9836444ns | -29.23% | 1.290 |
| engine_par | 8320593ns | 8217433ns | 8381610ns | -39.61% | 1.512 |
| hand | 13778770ns | 13660442ns | 13887763ns | base | 0.913 |
| hand_par | 3498017ns | 3387808ns | 3566000ns | -74.61% | 3.597 |
| rust_nostd | 9601666ns | 9534308ns | 9662065ns | -30.32% | 1.310 |
| rust_std | 9673744ns | 9598742ns | 9714660ns | -29.79% | 1.301 |
| rust_std_par | 3028817ns | 2833767ns | 3143965ns | -78.02% | 4.154 |

## Performance model

- Peak throughput: 4.440 Gops/s (rust_std_par; best 20% batches)
- Ops per call: 12582912

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 1.293 | 29.1% |
| engine_par | 1.512 | 34.0% |
| hand | 0.919 | 20.7% |
| hand_par | 3.588 | 80.8% |
| rust_nostd | 1.315 | 29.6% |
| rust_std | 1.300 | 29.3% |
| rust_std_par | 4.110 | 92.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 9753603ns | 9753603ns | -29.23% |
| engine_par | 8323957ns | 8323957ns | -39.60% |
| hand | 13781772ns | 13781772ns | base |
| hand_par | 3501221ns | 3501221ns | -74.60% |
| rust_nostd | 9604183ns | 9604183ns | -30.31% |
| rust_std | 9676301ns | 9676301ns | -29.79% |
| rust_std_par | 3031604ns | 3031604ns | -78.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 13689929ns | base | --- | [13668858, 13709942] | --- | --- | --- | --- |
| engine | 9733362ns | -3954525.0ns (-28.9%) | [-4075842, -3850825]ns | [9645625, 9826858] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 8322788ns | -5405433.3ns (-39.5%) | [-5460250, -5332567]ns | [8225758, 8374350] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 3506479ns | -10174304.2ns (-74.3%) | [-10290883, -10116783]ns | [3419058, 3557867] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 9566117ns | -4108045.8ns (-30.0%) | [-4165492, -4026567]ns | [9536683, 9651117] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 9676654ns | -4014545.8ns (-29.3%) | [-4078942, -3988067]ns | [9629092, 9720950] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 3061800ns | -10696841.7ns (-78.1%) | [-10873150, -10562083]ns | [2835792, 3167675] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 13687150ns | -28.9% | -39.3% | -74.3% | -30.1% | -29.2% | -78.6% |
| 1 | 2 | 13693812ns | -28.9% | -39.2% | -74.6% | -30.1% | -29.4% | -77.4% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| hand | -0.500 | moderate- |
| hand_par | -0.500 | HIGH- (thermal bounce) |
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
| engine | 17316204.1ns | 9751379.1ns | 177.6% | HIGH |
| engine_par | 17343409.4ns | 8320592.7ns | 208.4% | HIGH |
| hand | 17521865.6ns | 13778769.8ns | 127.2% | HIGH |
| hand_par | 17326266.7ns | 3498016.7ns | 495.3% | HIGH |
| rust_nostd | 17338099.0ns | 9601665.6ns | 180.6% | HIGH |
| rust_std | 17351918.7ns | 9673743.8ns | 179.4% | HIGH |
| rust_std_par | 17343251.1ns | 3028816.7ns | 572.6% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 9626333.3-9836443.7 ns)
  9626333.3 |########################################
  9636838.8 |########################################
  9647344.3 |
  9657849.9 |
  9668355.4 |########################################
  9678860.9 |
  9689366.4 |
  9699871.9 |
  9710377.5 |########################################
  9720883.0 |
  9731388.5 |
  9741894.0 |########################################
  9752399.6 |
  9762905.1 |
  9773410.6 |
  9783916.1 |########################################
  9794421.6 |
  9804927.2 |
  9815432.7 |
  9825938.2 |########################################
  (0 below, 1 above range)

engine_par (n=8, range 8217433.3-8381610.4 ns)
  8217433.3 |########################################
  8225642.2 |########################################
  8233851.0 |
  8242059.9 |
  8250268.7 |
  8258477.6 |
  8266686.4 |
  8274895.3 |########################################
  8283104.1 |
  8291313.0 |
  8299521.8 |
  8307730.7 |
  8315939.6 |########################################
  8324148.4 |########################################
  8332357.3 |
  8340566.1 |
  8348775.0 |########################################
  8356983.8 |
  8365192.7 |
  8373401.5 |########################################
  (0 below, 1 above range)

hand (n=8, range 13660441.7-13887762.5 ns)
  13660441.7 |########################################
  13671807.7 |########################################
  13683173.8 |
  13694539.8 |####################
  13705905.9 |########################################
  13717271.9 |
  13728637.9 |
  13740004.0 |
  13751370.0 |
  13762736.1 |
  13774102.1 |
  13785468.2 |
  13796834.2 |
  13808200.2 |
  13819566.3 |
  13830932.3 |
  13842298.4 |
  13853664.4 |
  13865030.4 |
  13876396.5 |
  (0 below, 1 above range)

hand_par (n=8, range 3387808.3-3566000.0 ns)
  3387808.3 |########################################
  3396717.9 |
  3405627.5 |
  3414537.1 |########################################
  3423446.6 |
  3432356.2 |
  3441265.8 |########################################
  3450175.4 |
  3459085.0 |
  3467994.6 |########################################
  3476904.1 |
  3485813.7 |
  3494723.3 |
  3503632.9 |
  3512542.5 |
  3521452.1 |
  3530361.7 |
  3539271.2 |########################################
  3548180.8 |########################################
  3557090.4 |########################################
  (0 below, 1 above range)

rust_nostd (n=8, range 9534308.3-9662064.6 ns)
  9534308.3 |########################################
  9540696.1 |####################
  9547083.9 |####################
  9553471.7 |
  9559859.6 |
  9566247.4 |
  9572635.2 |
  9579023.0 |####################
  9585410.8 |
  9591798.6 |
  9598186.4 |####################
  9604574.3 |
  9610962.1 |
  9617349.9 |
  9623737.7 |
  9630125.5 |
  9636513.3 |
  9642901.2 |
  9649289.0 |####################
  9655676.8 |
  (0 below, 1 above range)

rust_std (n=8, range 9598741.7-9714660.4 ns)
  9598741.7 |########################################
  9604537.6 |
  9610333.6 |
  9616129.5 |
  9621925.4 |
  9627721.4 |########################################
  9633517.3 |
  9639313.2 |
  9645109.2 |########################################
  9650905.1 |
  9656701.1 |########################################
  9662497.0 |
  9668292.9 |
  9674088.9 |
  9679884.8 |
  9685680.7 |
  9691476.7 |########################################
  9697272.6 |
  9703068.5 |
  9708864.5 |########################################
  (0 below, 2 above range)

rust_std_par (n=8, range 2833766.7-3143964.6 ns)
  2833766.7 |########################################
  2849276.6 |
  2864786.5 |
  2880296.4 |
  2895806.3 |
  2911316.2 |
  2926826.1 |
  2942336.0 |
  2957845.9 |####################
  2973355.7 |
  2988865.6 |
  3004375.5 |
  3019885.4 |####################
  3035395.3 |
  3050905.2 |
  3066415.1 |
  3081925.0 |
  3097434.9 |########################################
  3112944.8 |
  3128454.7 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=177.6% of algo (FFI overhead may distort results)
- engine_par: bridge=208.5% of algo (FFI overhead may distort results)
- hand: bridge=126.9% of algo (FFI overhead may distort results)
- hand_par: bridge=493.8% of algo (FFI overhead may distort results)
- rust_nostd: bridge=181.3% of algo (FFI overhead may distort results)
- rust_std: bridge=179.2% of algo (FFI overhead may distort results)
- rust_std_par: bridge=566.3% of algo (FFI overhead may distort results)
