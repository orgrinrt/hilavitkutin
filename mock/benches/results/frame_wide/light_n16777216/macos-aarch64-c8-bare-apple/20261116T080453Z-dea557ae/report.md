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

### rust_std_par is an outlier: 5.5x slower than the field

rust_std_par (44.74 ms) is 5.5x the fastest (8.09 ms), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (hand)

The baseline hand is the fastest (8.09 ms median); no rival improves on it (all deltas are >= 0).


### Two tiers: {hand, hand_par} vs {rust_std, rust_nostd, engine_par, engine, rust_std_par} (262% apart)

The field splits into a fast tier {hand, hand_par} and a slow tier {rust_std, rust_nostd, engine_par, engine, rust_std_par} with a 262% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 5.5x the fastest

Fastest hand (8.09 ms) to slowest rust_std_par (44.74 ms): 5.5x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (hand) is the fastest at 8088945.8 ns median
- 6 variants significantly slower than baseline
- Spread: 5.53x (fastest 8088945.8 ns, slowest 44736154.1 ns)

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
| engine | 33865299ns | 33715883ns | 33575025ns | 33655383ns | 34095304ns | +317.47% |
| engine_par | 32711936ns | 32552058ns | 32312017ns | 32510983ns | 32962631ns | +303.25% |
| hand | 8111981ns | 8094108ns | 7957992ns | 8037267ns | 8206515ns | base |
| hand_par | 8387233ns | 8393333ns | 8322492ns | 8367942ns | 8417888ns | +3.39% |
| rust_nostd | 30505436ns | 30438412ns | 30064283ns | 30258603ns | 30800850ns | +276.05% |
| rust_std | 30678034ns | 30393138ns | 29977825ns | 30330250ns | 31113925ns | +278.18% |
| rust_std_par | 44835127ns | 44739988ns | 44665867ns | 44712200ns | 44969638ns | +452.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 33860548ns | 33568408ns | 34090556ns | +317.68% | 5.946 |
| engine_par | 32707623ns | 32309100ns | 32958429ns | +303.46% | 6.155 |
| hand | 8106777ns | 7952908ns | 8200754ns | base | 24.834 |
| hand_par | 8382765ns | 8319000ns | 8413440ns | +3.40% | 24.017 |
| rust_nostd | 30500291ns | 30060708ns | 30795429ns | +276.23% | 6.601 |
| rust_std | 30672486ns | 29975850ns | 31108160ns | +278.36% | 6.564 |
| rust_std_par | 44831051ns | 44661842ns | 44965744ns | +453.01% | 4.491 |

## Performance model

- Peak throughput: 25.315 Gops/s (hand; best 20% batches)
- Ops per call: 201326592

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 5.972 | 23.6% |
| engine_par | 6.186 | 24.4% |
| hand | 24.889 | 98.3% |
| hand_par | 23.998 | 94.8% |
| rust_nostd | 6.615 | 26.1% |
| rust_std | 6.625 | 26.2% |
| rust_std_par | 4.500 | 17.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 33865299ns | 33865299ns | +317.47% |
| engine_par | 32711936ns | 32711936ns | +303.25% |
| hand | 8111981ns | 8111981ns | base |
| hand_par | 8387233ns | 8387233ns | +3.39% |
| rust_nostd | 30505436ns | 30505436ns | +276.05% |
| rust_std | 30678034ns | 30678034ns | +278.18% |
| rust_std_par | 44835127ns | 44835127ns | +452.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 8088946ns | base | --- | [8007667, 8226283] | --- | --- | --- | --- |
| engine | 33712425ns | +25668829.1ns (+317.3%) | [+25417417, +25836150]ns | [33641075, 33957133] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 32547871ns | +24469675.0ns (+302.5%) | [+24324558, +24695583]ns | [32444250, 32816567] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 8389379ns | +288795.9ns (+3.6%) | [+134258, +388450]ns | [8337983, 8422167] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 30433625ns | +22389841.6ns (+276.8%) | [+22050963, +22788317]ns | [30073692, 30845225] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 30386683ns | +22270641.6ns (+275.3%) | [+22052942, +23783367]ns | [30279225, 31748042] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 44736154ns | +36684183.3ns (+453.5%) | [+36592858, +36910925]ns | [44692342, 45109292] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 8045312ns | +319.8% | +304.7% | +4.5% | +278.3% | +278.3% | +455.7% |
| 1 | 2 | 8159675ns | +312.3% | +298.9% | +2.3% | +274.4% | +272.1% | +451.3% |

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
| engine | 271425290.6ns | 33860547.9ns | 801.6% | HIGH |
| engine_par | 271169404.2ns | 32707622.9ns | 829.1% | HIGH |
| hand | 271105490.6ns | 8106777.1ns | 3344.2% | HIGH |
| hand_par | 271276888.6ns | 8382764.6ns | 3236.1% | HIGH |
| rust_nostd | 271355289.6ns | 30500290.6ns | 889.7% | HIGH |
| rust_std | 271420167.7ns | 30672486.5ns | 884.9% | HIGH |
| rust_std_par | 271141093.8ns | 44831051.1ns | 604.8% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 33568408.3-34090556.2 ns)
  33568408.3 |####################
  33594515.7 |
  33620623.1 |########################################
  33646730.5 |####################
  33672837.9 |
  33698945.3 |
  33725052.7 |
  33751160.1 |####################
  33777267.5 |####################
  33803374.9 |
  33829482.3 |
  33855589.7 |
  33881697.1 |
  33907804.5 |
  33933911.8 |####################
  33960019.2 |
  33986126.6 |
  34012234.0 |
  34038341.4 |
  34064448.8 |
  (0 below, 1 above range)

engine_par (n=8, range 32309100.0-32958429.2 ns)
  32309100.0 |#############
  32341566.5 |
  32374032.9 |
  32406499.4 |
  32438965.8 |#############
  32471432.3 |
  32503898.8 |
  32536365.2 |########################################
  32568831.7 |#############
  32601298.1 |
  32633764.6 |
  32666231.0 |
  32698697.5 |
  32731164.0 |
  32763630.4 |
  32796096.9 |#############
  32828563.3 |
  32861029.8 |
  32893496.3 |
  32925962.7 |
  (0 below, 1 above range)

hand (n=8, range 7952908.3-8200754.2 ns)
  7952908.3 |########################################
  7965300.6 |
  7977692.9 |
  7990085.2 |
  8002477.5 |########################################
  8014869.8 |
  8027262.1 |########################################
  8039654.3 |
  8052046.6 |########################################
  8064438.9 |
  8076831.2 |
  8089223.5 |
  8101615.8 |
  8114008.1 |########################################
  8126400.4 |
  8138792.7 |
  8151185.0 |
  8163577.3 |
  8175969.6 |
  8188361.9 |########################################
  (0 below, 2 above range)

hand_par (n=8, range 8319000.0-8413439.6 ns)
  8319000.0 |####################
  8323722.0 |
  8328444.0 |
  8333165.9 |
  8337887.9 |####################
  8342609.9 |
  8347331.9 |
  8352053.9 |
  8356775.8 |
  8361497.8 |####################
  8366219.8 |
  8370941.8 |
  8375663.8 |
  8380385.7 |
  8385107.7 |####################
  8389829.7 |########################################
  8394551.7 |
  8399273.7 |
  8403995.6 |
  8408717.6 |
  (0 below, 2 above range)

rust_nostd (n=8, range 30060708.3-30795429.2 ns)
  30060708.3 |########################################
  30097444.3 |
  30134180.4 |
  30170916.4 |
  30207652.5 |
  30244388.5 |####################
  30281124.6 |
  30317860.6 |
  30354596.7 |
  30391332.7 |####################
  30428068.7 |####################
  30464804.8 |
  30501540.8 |
  30538276.9 |
  30575012.9 |
  30611749.0 |
  30648485.0 |
  30685221.0 |
  30721957.1 |
  30758693.1 |
  (0 below, 3 above range)

rust_std (n=8, range 29975850.0-31108160.4 ns)
  29975850.0 |####################
  30032465.5 |
  30089081.0 |
  30145696.6 |
  30202312.1 |
  30258927.6 |####################
  30315543.1 |####################
  30372158.6 |########################################
  30428774.2 |
  30485389.7 |####################
  30542005.2 |
  30598620.7 |
  30655236.3 |
  30711851.8 |
  30768467.3 |
  30825082.8 |
  30881698.3 |
  30938313.9 |
  30994929.4 |
  31051544.9 |
  (0 below, 2 above range)

rust_std_par (n=8, range 44661841.7-44965743.8 ns)
  44661841.7 |####################
  44677036.8 |
  44692231.9 |####################
  44707427.0 |########################################
  44722622.1 |
  44737817.2 |
  44753012.3 |####################
  44768207.4 |
  44783402.5 |
  44798597.6 |
  44813792.7 |
  44828987.8 |
  44844182.9 |####################
  44859378.0 |
  44874573.1 |
  44889768.2 |
  44904963.3 |
  44920158.4 |
  44935353.5 |
  44950548.6 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=804.4% of algo (FFI overhead may distort results)
- engine_par: bridge=833.0% of algo (FFI overhead may distort results)
- hand: bridge=3351.5% of algo (FFI overhead may distort results)
- hand_par: bridge=3230.5% of algo (FFI overhead may distort results)
- rust_nostd: bridge=890.8% of algo (FFI overhead may distort results)
- rust_std: bridge=892.2% of algo (FFI overhead may distort results)
- rust_std_par: bridge=605.8% of algo (FFI overhead may distort results)
