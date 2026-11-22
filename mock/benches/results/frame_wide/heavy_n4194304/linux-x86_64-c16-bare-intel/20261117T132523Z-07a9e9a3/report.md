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

The declared/defaulted baseline hand has the worst median (155.27 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest rust_std_par at 31.44 ms).


### rust_std_par dominates: 18% faster than the next best (hand_par)

rust_std_par (31.44 ms) leads hand_par (36.95 ms) by 18%, a clear separation rather than a photo finish. CV 3.2%.


### rust_std_par beats baseline by 80% (significant)

rust_std_par is -123.95 ms (80%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 4.9x slower than the field

hand (155.27 ms) is 4.9x the fastest (31.44 ms), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {rust_std_par, hand_par} vs {engine_par, rust_nostd, engine, rust_std, hand} (218% apart)

The field splits into a fast tier {rust_std_par, hand_par} and a slow tier {engine_par, rust_nostd, engine, rust_std, hand} with a 218% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 4.9x the fastest

Fastest rust_std_par (31.44 ms) to slowest hand (155.27 ms): 4.9x. The strategy choice matters a lot for this workload.


### hand_par is inconsistent: worst-20% is 1.8x its best-20%

hand_par's best 20% of batches run at 26.40 ms but its worst 20% at 47.33 ms (1.8x) - a bimodal or bursty profile the median hides.


## Key findings

- Fastest: rust_std_par at 31444164.2 ns median (-79.7% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 4.94x (fastest 31444164.2 ns, slowest 155272872.3 ns)

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
| Counter frequency | 2112550401 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 127267013ns | 127271028ns | 127058462ns | 127163770ns | 127396583ns | -18.06% |
| engine_par | 120716174ns | 117676528ns | 115907110ns | 116760558ns | 124885151ns | -22.27% |
| hand | 155309665ns | 155274308ns | 155148160ns | 155228617ns | 155410828ns | base |
| hand_par | 39108987ns | 36955747ns | 26407343ns | 32377889ns | 47332721ns | -74.82% |
| rust_nostd | 127216147ns | 127162193ns | 127063333ns | 127116573ns | 127329031ns | -18.09% |
| rust_std | 149733944ns | 149732834ns | 145051384ns | 149141703ns | 151348765ns | -3.59% |
| rust_std_par | 31579321ns | 31446735ns | 30706733ns | 31000272ns | 32231756ns | -79.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 127265864ns | 127057457ns | 127395463ns | -18.06% | 0.395 |
| engine_par | 120713593ns | 115904308ns | 124882591ns | -22.27% | 0.417 |
| hand | 155308093ns | 155146611ns | 155409387ns | base | 0.324 |
| hand_par | 39106041ns | 26404513ns | 47329792ns | -74.82% | 1.287 |
| rust_nostd | 127214594ns | 127062078ns | 127327471ns | -18.09% | 0.396 |
| rust_std | 149732247ns | 145049625ns | 151347128ns | -3.59% | 0.336 |
| rust_std_par | 31576840ns | 30704276ns | 32229259ns | -79.67% | 1.594 |

## Performance model

- Peak throughput: 1.906 Gops/s (hand_par; best 20% batches)
- Ops per call: 50331648

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.395 | 20.7% |
| engine_par | 0.428 | 22.4% |
| hand | 0.324 | 17.0% |
| hand_par | 1.362 | 71.5% |
| rust_nostd | 0.396 | 20.8% |
| rust_std | 0.336 | 17.6% |
| rust_std_par | 1.601 | 84.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 127267013ns | 127267013ns | -18.06% |
| engine_par | 120716174ns | 120716174ns | -22.27% |
| hand | 155309665ns | 155309665ns | base |
| hand_par | 39108987ns | 39108987ns | -74.82% |
| rust_nostd | 127216147ns | 127216147ns | -18.09% |
| rust_std | 149733944ns | 149733944ns | -3.59% |
| rust_std_par | 31579321ns | 31579321ns | -79.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 155272872ns | base | --- | [155204724, 155385621] | --- | --- | --- | --- |
| engine | 127269508ns | -28039698.7ns (-18.1%) | [-28311700, -27833393]ns | [127115304, 127416599] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 117673984ns | -37635590.7ns (-24.2%) | [-38994655, -27995210]ns | [116390965, 127230664] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 36953108ns | -118342064.5ns (-76.2%) | [-127848407, -108416027]ns | [27738610, 46809847] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 127160259ns | -28117596.4ns (-18.1%) | [-28257920, -28035800]ns | [127092301, 127277957] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 149731371ns | -5541501.0ns (-3.6%) | [-6904726, -4966296]ns | [148241885, 150259578] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 31444164ns | -123954949.2ns (-79.8%) | [-124500448, -123506824]ns | [30776710, 31719050] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 155332455ns | -18.2% | -22.0% | -73.6% | -18.1% | -3.4% | -80.0% |
| 1 | 2 | 155237933ns | -17.9% | -24.7% | -76.8% | -18.1% | -4.1% | -79.6% |

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
| engine | 47712537.9ns | 127265864.0ns | 37.5% | HIGH |
| engine_par | 47755573.3ns | 120713592.9ns | 39.6% | HIGH |
| hand | 47858207.0ns | 155308093.4ns | 30.8% | HIGH |
| hand_par | 48151130.2ns | 39106040.7ns | 123.1% | HIGH |
| rust_nostd | 47345229.6ns | 127214593.7ns | 37.2% | HIGH |
| rust_std | 47565795.6ns | 149732246.9ns | 31.8% | HIGH |
| rust_std_par | 50552423.9ns | 31576840.1ns | 160.1% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 127057456.8-127395462.7 ns)
  127057456.8 |########################################
  127074357.1 |
  127091257.4 |
  127108157.7 |########################################
  127125058.0 |
  127141958.3 |
  127158858.6 |########################################
  127175758.8 |
  127192659.1 |
  127209559.4 |########################################
  127226459.7 |
  127243360.0 |
  127260260.3 |
  127277160.6 |
  127294060.9 |
  127310961.2 |########################################
  127327861.5 |
  127344761.8 |########################################
  127361662.1 |
  127378562.4 |
  (0 below, 2 above range)

engine_par (n=8, range 115904307.7-124882590.6 ns)
  115904307.7 |####################
  116353221.8 |########################################
  116802136.0 |
  117251050.1 |####################
  117699964.3 |####################
  118148878.4 |
  118597792.6 |
  119046706.7 |
  119495620.9 |
  119944535.0 |
  120393449.2 |
  120842363.3 |
  121291277.4 |
  121740191.6 |
  122189105.7 |
  122638019.9 |
  123086934.0 |
  123535848.2 |
  123984762.3 |
  124433676.5 |####################
  (0 below, 2 above range)

hand (n=8, range 155146610.6-155409386.8 ns)
  155146610.6 |########################################
  155159749.4 |
  155172888.2 |
  155186027.0 |
  155199165.8 |########################################
  155212304.6 |
  155225443.4 |########################################
  155238582.3 |########################################
  155251721.1 |
  155264859.9 |
  155277998.7 |
  155291137.5 |########################################
  155304276.3 |
  155317415.1 |
  155330553.9 |
  155343692.7 |
  155356831.5 |########################################
  155369970.3 |
  155383109.1 |########################################
  155396247.9 |
  (0 below, 1 above range)

hand_par (n=8, range 26404512.8-47329792.2 ns)
  26404512.8 |####################
  27450776.8 |####################
  28497040.7 |
  29543304.7 |
  30589568.7 |
  31635832.7 |
  32682096.6 |
  33728360.6 |####################
  34774624.6 |####################
  35820888.5 |
  36867152.5 |
  37913416.5 |####################
  38959680.5 |
  40005944.4 |
  41052208.4 |
  42098472.4 |
  43144736.3 |
  44191000.3 |
  45237264.3 |
  46283528.3 |########################################
  (0 below, 1 above range)

rust_nostd (n=8, range 127062078.2-127327470.8 ns)
  127062078.2 |########################################
  127075347.8 |
  127088617.5 |########################################
  127101887.1 |########################################
  127115156.7 |
  127128426.4 |########################################
  127141696.0 |
  127154965.6 |
  127168235.2 |########################################
  127181504.9 |
  127194774.5 |########################################
  127208044.1 |
  127221313.8 |
  127234583.4 |
  127247853.0 |
  127261122.7 |
  127274392.3 |########################################
  127287661.9 |
  127300931.6 |
  127314201.2 |
  (0 below, 1 above range)

rust_std (n=8, range 145049625.3-151347128.1 ns)
  145049625.3 |####################
  145364500.4 |
  145679375.6 |
  145994250.7 |
  146309125.9 |
  146624001.0 |
  146938876.1 |
  147253751.3 |
  147568626.4 |
  147883501.6 |
  148198376.7 |####################
  148513251.9 |
  148828127.0 |
  149143002.1 |
  149457877.3 |########################################
  149772752.4 |####################
  150087627.6 |########################################
  150402502.7 |
  150717377.8 |
  151032253.0 |
  (0 below, 1 above range)

rust_std_par (n=8, range 30704275.8-32229259.1 ns)
  30704275.8 |########################################
  30780525.0 |####################
  30856774.1 |
  30933023.3 |
  31009272.5 |
  31085521.6 |
  31161770.8 |
  31238020.0 |
  31314269.1 |####################
  31390518.3 |
  31466767.5 |####################
  31543016.6 |####################
  31619265.8 |
  31695515.0 |####################
  31771764.1 |
  31848013.3 |
  31924262.5 |
  32000511.6 |
  32076760.8 |
  32153010.0 |
  (0 below, 1 above range)

```

## Diagnostics

- engine: bridge=37.2% of algo (FFI overhead may distort results)
- engine_par: bridge=40.6% of algo (FFI overhead may distort results)
- hand: bridge=30.7% of algo (FFI overhead may distort results)
- hand_par: CV=25.4% (high variance, measurements may be unstable)
- hand_par: bridge=130.1% of algo (FFI overhead may distort results)
- rust_nostd: bridge=37.2% of algo (FFI overhead may distort results)
- rust_std: bridge=31.7% of algo (FFI overhead may distort results)
- rust_std_par: bridge=156.1% of algo (FFI overhead may distort results)
