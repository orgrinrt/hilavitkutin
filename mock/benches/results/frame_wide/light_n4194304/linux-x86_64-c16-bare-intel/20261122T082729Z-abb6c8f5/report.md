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

### hand dominates: 26% faster than the next best (hand_par)

hand (3.99 ms) leads hand_par (5.04 ms) by 26%, a clear separation rather than a photo finish. CV 0.6%.


### rust_std is an outlier: 6.9x slower than the field

rust_std (27.59 ms) is 6.9x the fastest (3.99 ms), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (hand)

The baseline hand is the fastest (3.99 ms median); no rival improves on it (all deltas are >= 0).


### Two tiers: {hand, hand_par} vs {engine, engine_par, rust_nostd, rust_std_par, rust_std} (269% apart)

The field splits into a fast tier {hand, hand_par} and a slow tier {engine, engine_par, rust_nostd, rust_std_par, rust_std} with a 269% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 6.9x the fastest

Fastest hand (3.99 ms) to slowest rust_std (27.59 ms): 6.9x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (hand) is the fastest at 3990574.2 ns median
- 6 variants significantly slower than baseline
- Spread: 6.91x (fastest 3990574.2 ns, slowest 27592988.2 ns)

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
| Counter frequency | 2112754058 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 18665247ns | 18581924ns | 18554312ns | 18557992ns | 18773423ns | +366.47% |
| engine_par | 20803278ns | 18909792ns | 17751219ns | 17790671ns | 23825749ns | +419.91% |
| hand | 4001351ns | 3992053ns | 3982038ns | 3988057ns | 4016149ns | base |
| hand_par | 5128881ns | 5043218ns | 5006850ns | 5017463ns | 5242953ns | +28.18% |
| rust_nostd | 22313461ns | 22256630ns | 22159684ns | 22227240ns | 22416570ns | +457.65% |
| rust_std | 27671801ns | 27594773ns | 27174964ns | 27409323ns | 27992869ns | +591.56% |
| rust_std_par | 27002694ns | 27003994ns | 26740785ns | 26960927ns | 27099497ns | +574.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 18664135ns | 18553080ns | 18772313ns | +366.62% | 2.697 |
| engine_par | 20800511ns | 17748372ns | 23822768ns | +420.03% | 2.420 |
| hand | 3999846ns | 3980325ns | 4014628ns | base | 12.583 |
| hand_par | 5125702ns | 5003676ns | 5239678ns | +28.15% | 9.819 |
| rust_nostd | 22311750ns | 22158420ns | 22414780ns | +457.82% | 2.256 |
| rust_std | 27670139ns | 27173768ns | 27991331ns | +591.78% | 1.819 |
| rust_std_par | 26999747ns | 26738049ns | 27096506ns | +575.02% | 1.864 |

## Performance model

- Peak throughput: 12.645 Gops/s (hand; best 20% batches)
- Ops per call: 50331648

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 2.709 | 21.4% |
| engine_par | 2.662 | 21.1% |
| hand | 12.613 | 99.7% |
| hand_par | 9.986 | 79.0% |
| rust_nostd | 2.262 | 17.9% |
| rust_std | 1.824 | 14.4% |
| rust_std_par | 1.864 | 14.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 18665247ns | 18665247ns | +366.47% |
| engine_par | 20803278ns | 20803278ns | +419.91% |
| hand | 4001351ns | 4001351ns | base |
| hand_par | 5128881ns | 5128881ns | +28.18% |
| rust_nostd | 22313461ns | 22313461ns | +457.65% |
| rust_std | 27671801ns | 27671801ns | +591.56% |
| rust_std_par | 27002694ns | 27002694ns | +574.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 3990574ns | base | --- | [3984855, 4016219] | --- | --- | --- | --- |
| engine | 18581038ns | +14594938.8ns (+365.7%) | [+14557025, +14629416]ns | [18553902, 18667358] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 18907094ns | +14911945.7ns (+373.7%) | [+13784639, +21384550]ns | [17769494, 25436545] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 5040077ns | +1035971.1ns (+26.0%) | [+1016332, +1301109]ns | [5006106, 5317328] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 22254926ns | +18257624.4ns (+457.5%) | [+18211678, +18269731]ns | [22196533, 22311346] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 27592988ns | +23592451.1ns (+591.2%) | [+23327910, +24166290]ns | [27308235, 28163167] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 27000927ns | +22999005.7ns (+576.3%) | [+22915554, +23123791]ns | [26946099, 27129062] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 3986292ns | +366.3% | +346.0% | +26.4% | +457.7% | +589.4% | +576.1% |
| 1 | 2 | 4004819ns | +363.9% | +492.4% | +25.8% | +455.9% | +590.5% | +575.1% |

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

- engine: won 0/2, lost 2/2
- engine_par: won 0/2, lost 2/2
- hand_par: won 0/2, lost 2/2
- rust_nostd: won 0/2, lost 2/2
- rust_std: won 0/2, lost 2/2
- rust_std_par: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 47324206.0ns | 18664134.8ns | 253.6% | HIGH |
| engine_par | 48597532.3ns | 20800510.7ns | 233.6% | HIGH |
| hand | 47717589.3ns | 3999845.6ns | 1193.0% | HIGH |
| hand_par | 49424500.6ns | 5125702.1ns | 964.2% | HIGH |
| rust_nostd | 47342325.2ns | 22311749.6ns | 212.2% | HIGH |
| rust_std | 47328656.8ns | 27670138.9ns | 171.0% | HIGH |
| rust_std_par | 49673804.9ns | 26999746.5ns | 184.0% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 18553080.2-18772312.8 ns)
  18553080.2 |########################################
  18564041.8 |
  18575003.5 |
  18585965.1 |
  18596926.7 |##########
  18607888.3 |##########
  18618850.0 |
  18629811.6 |
  18640773.2 |
  18651734.8 |
  18662696.5 |##########
  18673658.1 |
  18684619.7 |
  18695581.4 |
  18706543.0 |
  18717504.6 |
  18728466.2 |
  18739427.9 |
  18750389.5 |
  18761351.1 |
  (0 below, 1 above range)

engine_par (n=8, range 17748371.8-23822767.8 ns)
  17748371.8 |########################################
  18052091.6 |
  18355811.4 |
  18659531.2 |
  18963251.0 |
  19266970.8 |
  19570690.6 |
  19874410.4 |##########
  20178130.2 |
  20481850.0 |
  20785569.8 |
  21089289.6 |
  21393009.4 |
  21696729.2 |
  22000449.0 |##########
  22304168.8 |
  22607888.6 |
  22911608.4 |
  23215328.2 |
  23519048.0 |
  (0 below, 2 above range)

hand (n=8, range 3980325.2-4014627.8 ns)
  3980325.2 |####################
  3982040.3 |
  3983755.5 |####################
  3985470.6 |
  3987185.7 |########################################
  3988900.8 |
  3990616.0 |
  3992331.1 |####################
  3994046.2 |
  3995761.4 |####################
  3997476.5 |
  3999191.6 |
  4000906.7 |
  4002621.9 |
  4004337.0 |
  4006052.1 |
  4007767.3 |
  4009482.4 |
  4011197.5 |
  4012912.6 |
  (0 below, 2 above range)

hand_par (n=8, range 5003676.0-5239678.1 ns)
  5003676.0 |########################################
  5015476.1 |########################################
  5027276.2 |
  5039076.3 |
  5050876.4 |####################
  5062676.5 |####################
  5074476.6 |
  5086276.7 |
  5098076.8 |
  5109876.9 |
  5121677.0 |
  5133477.2 |
  5145277.3 |
  5157077.4 |
  5168877.5 |
  5180677.6 |
  5192477.7 |
  5204277.8 |
  5216077.9 |
  5227878.0 |
  (0 below, 2 above range)

rust_nostd (n=8, range 22158420.0-22414780.3 ns)
  22158420.0 |####################
  22171238.0 |
  22184056.0 |####################
  22196874.1 |
  22209692.1 |
  22222510.1 |
  22235328.1 |########################################
  22248146.1 |
  22260964.1 |####################
  22273782.2 |####################
  22286600.2 |
  22299418.2 |####################
  22312236.2 |
  22325054.2 |
  22337872.2 |
  22350690.3 |
  22363508.3 |
  22376326.3 |
  22389144.3 |
  22401962.3 |
  (0 below, 1 above range)

rust_std (n=8, range 27173768.3-27991330.9 ns)
  27173768.3 |########################################
  27214646.4 |
  27255524.6 |
  27296402.7 |########################################
  27337280.8 |
  27378159.0 |########################################
  27419037.1 |
  27459915.2 |
  27500793.3 |########################################
  27541671.5 |
  27582549.6 |
  27623427.7 |########################################
  27664305.9 |
  27705184.0 |
  27746062.1 |########################################
  27786940.2 |
  27827818.4 |
  27868696.5 |
  27909574.6 |
  27950452.8 |
  (0 below, 2 above range)

rust_std_par (n=8, range 26738048.8-27096505.8 ns)
  26738048.8 |####################
  26755971.6 |
  26773894.5 |
  26791817.3 |
  26809740.2 |
  26827663.0 |
  26845585.9 |
  26863508.7 |
  26881431.6 |
  26899354.4 |
  26917277.3 |
  26935200.1 |####################
  26953123.0 |########################################
  26971045.8 |
  26988968.7 |
  27006891.5 |
  27024814.4 |########################################
  27042737.2 |
  27060660.1 |
  27078582.9 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=254.4% of algo (FFI overhead may distort results)
- engine_par: bridge=254.0% of algo (FFI overhead may distort results)
- hand: bridge=1188.2% of algo (FFI overhead may distort results)
- hand_par: bridge=955.2% of algo (FFI overhead may distort results)
- rust_nostd: bridge=212.7% of algo (FFI overhead may distort results)
- rust_std: bridge=171.5% of algo (FFI overhead may distort results)
- rust_std_par: bridge=179.2% of algo (FFI overhead may distort results)
