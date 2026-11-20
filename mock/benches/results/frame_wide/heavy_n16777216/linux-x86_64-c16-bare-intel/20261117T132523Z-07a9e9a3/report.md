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

The declared/defaulted baseline hand has the worst median (620.70 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hand_par at 101.14 ms).


### hand_par dominates: 38% faster than the next best (rust_std_par)

hand_par (101.14 ms) leads rust_std_par (140.04 ms) by 38%, a clear separation rather than a photo finish. CV 3.7%.


### hand_par beats baseline by 84% (significant)

hand_par is -519.74 ms (84%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 6.1x slower than the field

hand (620.70 ms) is 6.1x the fastest (101.14 ms), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {hand_par, rust_std_par} vs {engine_par, rust_nostd, engine, rust_std, hand} (220% apart)

The field splits into a fast tier {hand_par, rust_std_par} and a slow tier {engine_par, rust_nostd, engine, rust_std, hand} with a 220% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 6.1x the fastest

Fastest hand_par (101.14 ms) to slowest hand (620.70 ms): 6.1x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: hand_par at 101141375.7 ns median (-83.7% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 6.14x (fastest 101141375.7 ns, slowest 620700935.5 ns)

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
| engine | 509490474ns | 509321935ns | 508643593ns | 509134195ns | 509969403ns | -17.93% |
| engine_par | 457131773ns | 448187271ns | 435637116ns | 440082833ns | 475292142ns | -26.36% |
| hand | 620782538ns | 620702256ns | 620296766ns | 620411048ns | 621182599ns | base |
| hand_par | 100695943ns | 101144199ns | 95097293ns | 98673782ns | 103612225ns | -83.78% |
| rust_nostd | 509180197ns | 509073424ns | 508675576ns | 508822158ns | 509574881ns | -17.98% |
| rust_std | 577884056ns | 577658709ns | 576571436ns | 576808647ns | 579018769ns | -6.91% |
| rust_std_par | 139052373ns | 140042519ns | 132582801ns | 137989884ns | 141466632ns | -77.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 509489161ns | 508641963ns | 509968131ns | -17.93% | 0.395 |
| engine_par | 457128996ns | 435633702ns | 475289503ns | -26.36% | 0.440 |
| hand | 620781059ns | 620295777ns | 621180925ns | base | 0.324 |
| hand_par | 100692958ns | 95094472ns | 103609186ns | -83.78% | 1.999 |
| rust_nostd | 509178388ns | 508674014ns | 509573221ns | -17.98% | 0.395 |
| rust_std | 577882368ns | 576569819ns | 579017019ns | -6.91% | 0.348 |
| rust_std_par | 139049389ns | 132579846ns | 141463679ns | -77.60% | 1.448 |

## Performance model

- Peak throughput: 2.117 Gops/s (hand_par; best 20% batches)
- Ops per call: 201326592

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.395 | 18.7% |
| engine_par | 0.449 | 21.2% |
| hand | 0.324 | 15.3% |
| hand_par | 1.991 | 94.0% |
| rust_nostd | 0.395 | 18.7% |
| rust_std | 0.349 | 16.5% |
| rust_std_par | 1.438 | 67.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 509490474ns | 509490474ns | -17.93% |
| engine_par | 457131773ns | 457131773ns | -26.36% |
| hand | 620782538ns | 620782538ns | base |
| hand_par | 100695943ns | 100695943ns | -83.78% |
| rust_nostd | 509180197ns | 509180197ns | -17.98% |
| rust_std | 577884056ns | 577884056ns | -6.91% |
| rust_std_par | 139052373ns | 139052373ns | -77.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 620700936ns | base | --- | [620296606, 621330660] | --- | --- | --- | --- |
| engine | 509320893ns | -111229599.4ns (-17.9%) | [-112084679, -110959438]ns | [508962778, 509419941] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 448184736ns | -172729198.2ns (-27.8%) | [-181037165, -141937888]ns | [439258612, 479406150] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 101141376ns | -519735201.4ns (-83.7%) | [-524711018, -516765212]ns | [95584759, 103531394] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 509071441ns | -111729925.4ns (-18.0%) | [-112415191, -110610314]ns | [508713035, 509769064] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 577657248ns | -43043687.3ns (-6.9%) | [-44630323, -40745582]ns | [576575954, 579550195] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 140039537ns | -480708422.3ns (-77.4%) | [-485566410, -478730478]ns | [134730196, 141825574] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 620337992ns | -17.9% | -29.2% | -84.3% | -17.9% | -6.7% | -77.3% |
| 1 | 2 | 621265401ns | -18.1% | -24.8% | -83.6% | -18.1% | -7.2% | -77.5% |

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
| engine | 188333578.2ns | 509489160.8ns | 37.0% | HIGH |
| engine_par | 190464352.4ns | 457128995.8ns | 41.7% | HIGH |
| hand | 188409321.6ns | 620781059.0ns | 30.4% | HIGH |
| hand_par | 189685715.4ns | 100692958.1ns | 188.4% | HIGH |
| rust_nostd | 188313788.9ns | 509178388.0ns | 37.0% | HIGH |
| rust_std | 188649442.7ns | 577882368.0ns | 32.6% | HIGH |
| rust_std_par | 190339264.5ns | 139049388.5ns | 136.9% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 508641962.9-509968130.5 ns)
  508641962.9 |#############
  508708271.3 |
  508774579.7 |
  508840888.0 |
  508907196.4 |#############
  508973504.8 |
  509039813.2 |
  509106121.6 |#############
  509172429.9 |
  509238738.3 |
  509305046.7 |########################################
  509371355.1 |#############
  509437663.5 |
  509503971.9 |
  509570280.2 |
  509636588.6 |
  509702897.0 |
  509769205.4 |
  509835513.8 |
  509901822.1 |
  (0 below, 1 above range)

engine_par (n=8, range 435633702.2-475289502.7 ns)
  435633702.2 |####################
  437616492.2 |####################
  439599282.3 |########################################
  441582072.3 |
  443564862.3 |
  445547652.3 |
  447530442.4 |
  449513232.4 |
  451496022.4 |
  453478812.4 |########################################
  455461602.5 |
  457444392.5 |
  459427182.5 |
  461409972.5 |
  463392762.6 |
  465375552.6 |
  467358342.6 |
  469341132.6 |
  471323922.7 |
  473306712.7 |
  (0 below, 2 above range)

hand (n=8, range 620295777.1-621180924.6 ns)
  620295777.1 |########################################
  620340034.5 |####################
  620384291.9 |
  620428549.2 |
  620472806.6 |
  620517064.0 |####################
  620561321.4 |
  620605578.7 |
  620649836.1 |
  620694093.5 |
  620738350.9 |
  620782608.3 |
  620826865.6 |####################
  620871123.0 |
  620915380.4 |
  620959637.8 |
  621003895.1 |
  621048152.5 |
  621092409.9 |
  621136667.3 |
  (0 below, 3 above range)

hand_par (n=8, range 95094472.5-103609186.4 ns)
  95094472.5 |########################################
  95520208.2 |########################################
  95945943.9 |
  96371679.6 |
  96797415.3 |
  97223151.0 |
  97648886.7 |
  98074622.4 |
  98500358.1 |
  98926093.8 |
  99351829.5 |########################################
  99777565.2 |
  100203300.9 |
  100629036.6 |########################################
  101054772.2 |########################################
  101480507.9 |
  101906243.6 |########################################
  102331979.3 |
  102757715.0 |
  103183450.7 |########################################
  (0 below, 1 above range)

rust_nostd (n=8, range 508674013.5-509573220.9 ns)
  508674013.5 |########################################
  508718973.9 |
  508763934.2 |####################
  508808894.6 |
  508853855.0 |
  508898815.3 |
  508943775.7 |####################
  508988736.1 |
  509033696.4 |
  509078656.8 |
  509123617.2 |
  509168577.5 |####################
  509213537.9 |
  509258498.3 |
  509303458.6 |
  509348419.0 |
  509393379.4 |
  509438339.7 |
  509483300.1 |####################
  509528260.5 |
  (0 below, 2 above range)

rust_std (n=8, range 576569819.1-579017018.5 ns)
  576569819.1 |########################################
  576692179.1 |
  576814539.0 |####################
  576936899.0 |####################
  577059259.0 |
  577181619.0 |
  577303978.9 |
  577426338.9 |
  577548698.9 |
  577671058.8 |
  577793418.8 |
  577915778.8 |
  578038138.8 |
  578160498.7 |
  578282858.7 |####################
  578405218.7 |####################
  578527578.6 |
  578649938.6 |
  578772298.6 |
  578894658.6 |
  (0 below, 2 above range)

rust_std_par (n=8, range 132579846.5-141463679.1 ns)
  132579846.5 |########################################
  133024038.1 |
  133468229.8 |
  133912421.4 |
  134356613.0 |########################################
  134800804.6 |
  135244996.3 |
  135689187.9 |
  136133379.5 |
  136577571.2 |
  137021762.8 |
  137465954.4 |
  137910146.0 |
  138354337.7 |
  138798529.3 |
  139242720.9 |########################################
  139686912.6 |########################################
  140131104.2 |########################################
  140575295.8 |
  141019487.4 |########################################
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=37.0% of algo (FFI overhead may distort results)
- engine_par: bridge=42.5% of algo (FFI overhead may distort results)
- hand: bridge=30.4% of algo (FFI overhead may distort results)
- hand_par: bridge=187.0% of algo (FFI overhead may distort results)
- rust_nostd: bridge=37.0% of algo (FFI overhead may distort results)
- rust_std: bridge=32.7% of algo (FFI overhead may distort results)
- rust_std_par: bridge=135.5% of algo (FFI overhead may distort results)
