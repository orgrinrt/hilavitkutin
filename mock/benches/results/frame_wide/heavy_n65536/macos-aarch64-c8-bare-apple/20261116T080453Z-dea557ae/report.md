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

The declared/defaulted baseline hand has the worst median (854.90 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest rust_std_par at 218.49 us).


### rust_std_par dominates: 17% faster than the next best (hand_par)

rust_std_par (218.49 us) leads hand_par (255.48 us) by 17%, a clear separation rather than a photo finish. CV 3.4%.


### rust_std_par beats baseline by 74% (significant)

rust_std_par is -636.70 us (74%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 3.9x slower than the field

hand (854.90 us) is 3.9x the fastest (218.49 us), well off the pack.


### engine shows alternating (throttle bounce) (autocorr -0.50)

engine's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {rust_std_par, hand_par} vs {engine_par, rust_nostd, rust_std, engine, hand} (115% apart)

The field splits into a fast tier {rust_std_par, hand_par} and a slow tier {engine_par, rust_nostd, rust_std, engine, hand} with a 115% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 3.9x the fastest

Fastest rust_std_par (218.49 us) to slowest hand (854.90 us): 3.9x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: rust_std_par at 218487.5 ns median (-74.4% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 3.91x (fastest 218487.5 ns, slowest 854900.0 ns)

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
| engine | 607831ns | 604671ns | 601433ns | 603081ns | 612994ns | -29.13% |
| engine_par | 552453ns | 551346ns | 543500ns | 550494ns | 556160ns | -35.58% |
| hand | 857643ns | 855533ns | 850975ns | 854814ns | 861431ns | base |
| hand_par | 261333ns | 256504ns | 247750ns | 253011ns | 270971ns | -69.53% |
| rust_nostd | 592815ns | 592562ns | 589058ns | 590719ns | 595325ns | -30.88% |
| rust_std | 598016ns | 597517ns | 593483ns | 596186ns | 600521ns | -30.27% |
| rust_std_par | 220336ns | 220029ns | 210567ns | 217575ns | 224850ns | -74.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 606886ns | 600725ns | 611835ns | -29.18% | 1.296 |
| engine_par | 551317ns | 542108ns | 555273ns | -35.66% | 1.426 |
| hand | 856915ns | 850742ns | 860383ns | base | 0.918 |
| hand_par | 260058ns | 246150ns | 269594ns | -69.65% | 3.024 |
| rust_nostd | 592440ns | 588875ns | 594967ns | -30.86% | 1.327 |
| rust_std | 597295ns | 593133ns | 599688ns | -30.30% | 1.317 |
| rust_std_par | 219255ns | 208608ns | 224167ns | -74.41% | 3.587 |

## Performance model

- Peak throughput: 3.770 Gops/s (rust_std_par; best 20% batches)
- Ops per call: 786432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 1.302 | 34.5% |
| engine_par | 1.429 | 37.9% |
| hand | 0.920 | 24.4% |
| hand_par | 3.078 | 81.7% |
| rust_nostd | 1.329 | 35.3% |
| rust_std | 1.318 | 35.0% |
| rust_std_par | 3.599 | 95.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 607831ns | 607831ns | -29.13% |
| engine_par | 552453ns | 552453ns | -35.58% |
| hand | 857643ns | 857643ns | base |
| hand_par | 261333ns | 261333ns | -69.53% |
| rust_nostd | 592815ns | 592815ns | -30.88% |
| rust_std | 598016ns | 598016ns | -30.27% |
| rust_std_par | 220336ns | 220336ns | -74.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 854900ns | base | --- | [854050, 858550] | --- | --- | --- | --- |
| engine | 603904ns | -251033.3ns (-29.4%) | [-253908, -246042]ns | [601958, 608983] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 550517ns | -304245.8ns (-35.6%) | [-312525, -302017]ns | [548100, 555467] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 255483ns | -600770.8ns (-70.3%) | [-609017, -581325]ns | [247958, 277225] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 591633ns | -262708.3ns (-30.7%) | [-265400, -261200]ns | [589542, 594942] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 596583ns | -259683.3ns (-30.4%) | [-262033, -255558]ns | [594542, 599075] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 218488ns | -636695.8ns (-74.5%) | [-646558, -631775]ns | [215258, 221783] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 854900ns | -29.2% | -35.7% | -70.5% | -30.9% | -30.2% | -74.4% |
| 1 | 2 | 854762ns | -29.4% | -35.3% | -68.9% | -30.5% | -30.2% | -74.6% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | HIGH- (thermal bounce) |
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
| engine | 1079738.5ns | 606886.4ns | 177.9% | HIGH |
| engine_par | 1078766.7ns | 551316.7ns | 195.7% | HIGH |
| hand | 1077108.3ns | 856914.6ns | 125.7% | HIGH |
| hand_par | 1087150.0ns | 260058.3ns | 418.0% | HIGH |
| rust_nostd | 1077542.7ns | 592439.6ns | 181.9% | HIGH |
| rust_std | 1080504.2ns | 597294.8ns | 180.9% | HIGH |
| rust_std_par | 1082838.6ns | 219255.2ns | 493.9% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 600725.0-611835.4 ns)
  600725.0 |####################
  601280.5 |
  601836.0 |########################################
  602391.6 |####################
  602947.1 |
  603502.6 |
  604058.1 |
  604613.6 |####################
  605169.2 |
  605724.7 |
  606280.2 |
  606835.7 |
  607391.2 |
  607946.8 |####################
  608502.3 |####################
  609057.8 |
  609613.3 |
  610168.8 |
  610724.4 |
  611279.9 |
  (0 below, 1 above range)

engine_par (n=8, range 542108.3-555272.9 ns)
  542108.3 |####################
  542766.5 |
  543424.8 |
  544083.0 |
  544741.2 |
  545399.5 |
  546057.7 |
  546715.9 |
  547374.2 |
  548032.4 |####################
  548690.6 |####################
  549348.8 |
  550007.1 |########################################
  550665.3 |
  551323.5 |
  551981.8 |
  552640.0 |
  553298.2 |
  553956.5 |####################
  554614.7 |
  (0 below, 2 above range)

hand (n=8, range 850741.7-860383.3 ns)
  850741.7 |####################
  851223.8 |
  851705.9 |
  852187.9 |
  852670.0 |
  853152.1 |
  853634.2 |####################
  854116.3 |####################
  854598.4 |####################
  855080.4 |########################################
  855562.5 |
  856044.6 |
  856526.7 |
  857008.8 |
  857490.9 |
  857972.9 |
  858455.0 |####################
  858937.1 |
  859419.2 |
  859901.3 |
  (0 below, 1 above range)

hand_par (n=8, range 246150.0-269593.8 ns)
  246150.0 |########################################
  247322.2 |########################################
  248494.4 |
  249666.6 |
  250838.8 |
  252010.9 |
  253183.1 |########################################
  254355.3 |########################################
  255527.5 |########################################
  256699.7 |
  257871.9 |
  259044.1 |
  260216.2 |
  261388.4 |########################################
  262560.6 |
  263732.8 |
  264905.0 |
  266077.2 |
  267249.4 |
  268421.6 |
  (0 below, 2 above range)

rust_nostd (n=8, range 588875.0-594966.7 ns)
  588875.0 |####################
  589179.6 |
  589484.2 |########################################
  589788.8 |
  590093.3 |
  590397.9 |
  590702.5 |
  591007.1 |
  591311.7 |####################
  591616.3 |####################
  591920.8 |
  592225.4 |
  592530.0 |
  592834.6 |
  593139.2 |
  593443.8 |####################
  593748.3 |
  594052.9 |
  594357.5 |
  594662.1 |####################
  (0 below, 1 above range)

rust_std (n=8, range 593133.3-599687.5 ns)
  593133.3 |########################################
  593461.0 |
  593788.7 |
  594116.4 |
  594444.1 |########################################
  594771.9 |
  595099.6 |
  595427.3 |########################################
  595755.0 |
  596082.7 |########################################
  596410.4 |
  596738.1 |########################################
  597065.8 |
  597393.5 |########################################
  597721.2 |
  598048.9 |
  598376.7 |
  598704.4 |
  599032.1 |########################################
  599359.8 |
  (0 below, 1 above range)

rust_std_par (n=8, range 208608.3-224166.7 ns)
  208608.3 |####################
  209386.2 |
  210164.1 |
  210942.1 |
  211720.0 |
  212497.9 |
  213275.8 |
  214053.7 |
  214831.6 |########################################
  215609.6 |
  216387.5 |
  217165.4 |
  217943.3 |####################
  218721.2 |####################
  219499.2 |####################
  220277.1 |
  221055.0 |####################
  221832.9 |
  222610.8 |
  223388.8 |
  (0 below, 1 above range)

```

## Diagnostics

- engine: bridge=178.0% of algo (FFI overhead may distort results)
- engine_par: bridge=195.9% of algo (FFI overhead may distort results)
- hand: bridge=125.8% of algo (FFI overhead may distort results)
- hand_par: bridge=425.7% of algo (FFI overhead may distort results)
- rust_nostd: bridge=182.1% of algo (FFI overhead may distort results)
- rust_std: bridge=181.3% of algo (FFI overhead may distort results)
- rust_std_par: bridge=495.1% of algo (FFI overhead may distort results)
