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

The declared/defaulted baseline hand has the worst median (623.96 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hand_par at 100.11 ms).


### hand_par dominates: 41% faster than the next best (rust_std_par)

hand_par (100.11 ms) leads rust_std_par (141.29 ms) by 41%, a clear separation rather than a photo finish. CV 4.3%.


### hand_par beats baseline by 84% (significant)

hand_par is -524.72 ms (84%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 6.2x slower than the field

hand (623.96 ms) is 6.2x the fastest (100.11 ms), well off the pack.


### rust_std shows alternating (throttle bounce) (autocorr -0.50)

rust_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {hand_par, rust_std_par} vs {engine_par, rust_nostd, engine, rust_std, hand} (211% apart)

The field splits into a fast tier {hand_par, rust_std_par} and a slow tier {engine_par, rust_nostd, engine, rust_std, hand} with a 211% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 6.2x the fastest

Fastest hand_par (100.11 ms) to slowest hand (623.96 ms): 6.2x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: hand_par at 100111158.3 ns median (-84.0% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 6.23x (fastest 100111158.3 ns, slowest 623957751.2 ns)

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
| Counter frequency | 2112801267 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 510420925ns | 510438023ns | 508384103ns | 508922058ns | 512054281ns | -18.16% |
| engine_par | 439154973ns | 438712837ns | 434979652ns | 437747741ns | 441254228ns | -29.59% |
| hand | 623675440ns | 623959035ns | 621031740ns | 622103081ns | 625515635ns | base |
| hand_par | 100801246ns | 100114225ns | 96042845ns | 98376485ns | 103809418ns | -83.84% |
| rust_nostd | 510036644ns | 509770536ns | 508374863ns | 509361091ns | 510958755ns | -18.22% |
| rust_std | 578686063ns | 578186650ns | 577462785ns | 577918315ns | 579567694ns | -7.21% |
| rust_std_par | 140691439ns | 141294397ns | 135819764ns | 139063177ns | 143130555ns | -77.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 510419763ns | 508382966ns | 512053130ns | -18.16% | 0.394 |
| engine_par | 439151650ns | 434976856ns | 441251521ns | -29.59% | 0.458 |
| hand | 623673811ns | 621029709ns | 625513861ns | base | 0.323 |
| hand_par | 100798041ns | 96039344ns | 103806311ns | -83.84% | 1.997 |
| rust_nostd | 510034768ns | 508373348ns | 510956806ns | -18.22% | 0.395 |
| rust_std | 578684197ns | 577461450ns | 579565651ns | -7.21% | 0.348 |
| rust_std_par | 140688384ns | 135816549ns | 143127562ns | -77.44% | 1.431 |

## Performance model

- Peak throughput: 2.096 Gops/s (hand_par; best 20% batches)
- Ops per call: 201326592

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.394 | 18.8% |
| engine_par | 0.459 | 21.9% |
| hand | 0.323 | 15.4% |
| hand_par | 2.011 | 95.9% |
| rust_nostd | 0.395 | 18.8% |
| rust_std | 0.348 | 16.6% |
| rust_std_par | 1.425 | 68.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 510420925ns | 510420925ns | -18.16% |
| engine_par | 439154973ns | 439154973ns | -29.59% |
| hand | 623675440ns | 623675440ns | base |
| hand_par | 100801246ns | 100801246ns | -83.84% |
| rust_nostd | 510036644ns | 510036644ns | -18.22% |
| rust_std | 578686063ns | 578686063ns | -7.21% |
| rust_std_par | 140691439ns | 140691439ns | -77.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 623957751ns | base | --- | [621204101, 625212751] | --- | --- | --- | --- |
| engine | 510437016ns | -112831838.7ns (-18.1%) | [-115514014, -111645168]ns | [508582272, 512214282] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 438710298ns | -184917847.8ns (-29.6%) | [-188083183, -180715944]ns | [436560633, 440313766] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 100111158ns | -524723056.5ns (-84.1%) | [-527021311, -519467274]ns | [97620063, 101774511] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 509768836ns | -113267228.8ns (-18.2%) | [-116269485, -111295106]ns | [508943266, 511672279] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 578184705ns | -45516258.6ns (-7.3%) | [-47533340, -42986113]ns | [577545464, 578595719] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 141291489ns | -482461304.1ns (-77.3%) | [-485839322, -480573652]ns | [136049404, 143431209] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 621222942ns | -18.1% | -29.4% | -83.7% | -17.9% | -6.9% | -77.2% |
| 1 | 2 | 625103770ns | -18.1% | -29.8% | -84.4% | -18.4% | -7.6% | -77.4% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| hand | -0.500 | moderate- |
| hand_par | -0.500 | moderate- |
| rust_nostd | -0.500 | moderate- |
| rust_std | -0.500 | HIGH- (thermal bounce) |
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
| engine | 188637989.1ns | 510419762.8ns | 37.0% | HIGH |
| engine_par | 191534487.9ns | 439151649.9ns | 43.6% | HIGH |
| hand | 188912738.5ns | 623673811.1ns | 30.3% | HIGH |
| hand_par | 190572910.3ns | 100798040.7ns | 189.1% | HIGH |
| rust_nostd | 188545822.2ns | 510034767.8ns | 37.0% | HIGH |
| rust_std | 188818165.6ns | 578684197.1ns | 32.6% | HIGH |
| rust_std_par | 190779675.1ns | 140688384.0ns | 135.6% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 508382965.6-512053129.8 ns)
  508382965.6 |####################
  508566473.8 |########################################
  508749982.0 |
  508933490.2 |
  509116998.4 |
  509300506.7 |####################
  509484014.9 |
  509667523.1 |
  509851031.3 |
  510034539.5 |
  510218047.7 |
  510401555.9 |
  510585064.1 |
  510768572.3 |
  510952080.6 |
  511135588.8 |
  511319097.0 |####################
  511502605.2 |
  511686113.4 |
  511869621.6 |
  (0 below, 3 above range)

engine_par (n=8, range 434976855.9-441251521.3 ns)
  434976855.9 |########################################
  435290589.2 |
  435604322.4 |
  435918055.7 |
  436231789.0 |
  436545522.3 |########################################
  436859255.5 |
  437172988.8 |
  437486722.1 |
  437800455.3 |########################################
  438114188.6 |
  438427921.9 |########################################
  438741655.2 |########################################
  439055388.4 |
  439369121.7 |
  439682855.0 |########################################
  439996588.2 |
  440310321.5 |########################################
  440624054.8 |
  440937788.1 |
  (0 below, 1 above range)

hand (n=8, range 621029709.3-625513861.2 ns)
  621029709.3 |########################################
  621253916.9 |
  621478124.5 |
  621702332.1 |
  621926539.7 |
  622150747.3 |
  622374954.9 |
  622599162.5 |
  622823370.1 |
  623047577.7 |
  623271785.2 |
  623495992.8 |
  623720200.4 |#############
  623944408.0 |#############
  624168615.6 |
  624392823.2 |
  624617030.8 |
  624841238.4 |#############
  625065446.0 |#############
  625289653.6 |
  (0 below, 1 above range)

hand_par (n=8, range 96039343.6-103806310.8 ns)
  96039343.6 |####################
  96427692.0 |
  96816040.3 |
  97204388.7 |
  97592737.0 |########################################
  97981085.4 |
  98369433.8 |
  98757782.1 |
  99146130.5 |####################
  99534478.8 |
  99922827.2 |
  100311175.5 |####################
  100699523.9 |
  101087872.3 |
  101476220.6 |########################################
  101864569.0 |
  102252917.3 |
  102641265.7 |
  103029614.1 |
  103417962.4 |
  (0 below, 1 above range)

rust_nostd (n=8, range 508373347.8-510956806.4 ns)
  508373347.8 |####################
  508502520.7 |
  508631693.7 |
  508760866.6 |
  508890039.5 |####################
  509019212.5 |
  509148385.4 |
  509277558.3 |
  509406731.2 |
  509535904.2 |########################################
  509665077.1 |
  509794250.0 |
  509923423.0 |####################
  510052595.9 |
  510181768.8 |
  510310941.8 |####################
  510440114.7 |
  510569287.6 |
  510698460.5 |
  510827633.5 |
  (0 below, 2 above range)

rust_std (n=8, range 577461449.6-579565651.4 ns)
  577461449.6 |########################################
  577566659.7 |
  577671869.8 |
  577777079.9 |
  577882290.0 |
  577987500.0 |####################
  578092710.1 |####################
  578197920.2 |########################################
  578303130.3 |
  578408340.4 |
  578513550.5 |####################
  578618760.6 |
  578723970.7 |
  578829180.8 |
  578934390.9 |
  579039601.0 |
  579144811.0 |
  579250021.1 |
  579355231.2 |
  579460441.3 |
  (0 below, 1 above range)

rust_std_par (n=8, range 135816549.0-143127561.9 ns)
  135816549.0 |########################################
  136182099.6 |
  136547650.3 |
  136913200.9 |
  137278751.6 |
  137644302.2 |
  138009852.9 |
  138375403.5 |
  138740954.2 |
  139106504.8 |
  139472055.4 |
  139837606.1 |
  140203156.7 |####################
  140568707.4 |####################
  140934258.0 |
  141299808.7 |
  141665359.3 |####################
  142030910.0 |
  142396460.6 |####################
  142762011.3 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=36.9% of algo (FFI overhead may distort results)
- engine_par: bridge=43.5% of algo (FFI overhead may distort results)
- hand: bridge=30.2% of algo (FFI overhead may distort results)
- hand_par: bridge=190.3% of algo (FFI overhead may distort results)
- rust_nostd: bridge=37.0% of algo (FFI overhead may distort results)
- rust_std: bridge=32.7% of algo (FFI overhead may distort results)
- rust_std_par: bridge=135.1% of algo (FFI overhead may distort results)
