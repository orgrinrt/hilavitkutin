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

### hand dominates: 197% faster than the next best (hand_par)

hand (14.96 us) leads hand_par (44.44 us) by 197%, a clear separation rather than a photo finish. CV 0.9%.


### engine_par is an outlier: 7.4x slower than the field

engine_par (111.08 us) is 7.4x the fastest (14.96 us), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (hand)

The baseline hand is the fastest (14.96 us median); no rival improves on it (all deltas are >= 0).


### Two tiers: {hand} vs {hand_par, rust_nostd, rust_std, engine, rust_std_par, engine_par} (197% apart)

The field splits into a fast tier {hand} and a slow tier {hand_par, rust_nostd, rust_std, engine, rust_std_par, engine_par} with a 197% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 7.4x the fastest

Fastest hand (14.96 us) to slowest engine_par (111.08 us): 7.4x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (hand) is the fastest at 14962.5 ns median
- 6 variants significantly slower than baseline
- Spread: 7.42x (fastest 14962.5 ns, slowest 111079.1 ns)

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
| engine | 79244ns | 79321ns | 78592ns | 79011ns | 79581ns | +409.57% |
| engine_par | 112620ns | 112767ns | 109050ns | 111233ns | 114552ns | +624.19% |
| hand | 15551ns | 15488ns | 14900ns | 15039ns | 16098ns | base |
| hand_par | 48099ns | 45017ns | 43108ns | 44339ns | 52167ns | +209.30% |
| rust_nostd | 71521ns | 71321ns | 71175ns | 71236ns | 71821ns | +359.91% |
| rust_std | 72457ns | 72529ns | 71950ns | 72458ns | 72583ns | +365.93% |
| rust_std_par | 89331ns | 86617ns | 80908ns | 85478ns | 94327ns | +474.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 78051ns | 77917ns | 78156ns | +421.79% | 10.076 |
| engine_par | 110825ns | 107467ns | 112606ns | +640.89% | 7.096 |
| hand | 14958ns | 14792ns | 15077ns | base | 52.575 |
| hand_par | 47033ns | 42450ns | 50962ns | +214.43% | 16.721 |
| rust_nostd | 70055ns | 69975ns | 70102ns | +368.34% | 11.226 |
| rust_std | 71336ns | 71283ns | 71377ns | +376.90% | 11.024 |
| rust_std_par | 87992ns | 79783ns | 93044ns | +488.24% | 8.938 |

## Performance model

- Peak throughput: 53.167 Gops/s (hand; best 20% batches)
- Ops per call: 786432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 10.077 | 19.0% |
| engine_par | 7.080 | 13.3% |
| hand | 52.560 | 98.9% |
| hand_par | 17.696 | 33.3% |
| rust_nostd | 11.221 | 21.1% |
| rust_std | 11.026 | 20.7% |
| rust_std_par | 9.244 | 17.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 79244ns | 79244ns | +409.57% |
| engine_par | 112620ns | 112620ns | +624.19% |
| hand | 15551ns | 15551ns | base |
| hand_par | 48099ns | 48099ns | +209.30% |
| rust_nostd | 71521ns | 71521ns | +359.91% |
| rust_std | 72457ns | 72457ns | +365.93% |
| rust_std_par | 89331ns | 89331ns | +474.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 14962ns | base | --- | [14808, 15100] | --- | --- | --- | --- |
| engine | 78046ns | +63075.0ns (+421.6%) | [+62992, +63167]ns | [77950, 78133] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 111079ns | +96033.3ns (+641.8%) | [+92975, +97033]ns | [107783, 112150] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 44442ns | +29391.7ns (+196.4%) | [+27692, +36117]ns | [42500, 51042] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 70083ns | +55083.3ns (+368.1%) | [+54975, +55250]ns | [69983, 70100] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 71325ns | +56395.8ns (+376.9%) | [+56183, +56483]ns | [71283, 71392] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 85071ns | +70270.9ns (+469.6%) | [+67725, +80483]ns | [82817, 95583] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 15050ns | +419.2% | +626.8% | +188.5% | +365.4% | +373.7% | +485.4% |
| 1 | 2 | 14879ns | +423.9% | +648.6% | +221.7% | +371.0% | +379.6% | +463.4% |

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
| engine | 1049890.6ns | 78051.0ns | 1345.1% | HIGH |
| engine_par | 1057019.8ns | 110825.0ns | 953.8% | HIGH |
| hand | 1050717.7ns | 14958.3ns | 7024.3% | HIGH |
| hand_par | 1072024.0ns | 47033.3ns | 2279.3% | HIGH |
| rust_nostd | 1049342.7ns | 70055.2ns | 1497.9% | HIGH |
| rust_std | 1049370.8ns | 71336.4ns | 1471.0% | HIGH |
| rust_std_par | 1058851.0ns | 87991.7ns | 1203.4% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 77916.7-78156.2 ns)
  77916.7 |#############
  77928.7 |
  77940.7 |#############
  77952.6 |##########################
  77964.6 |
  77976.6 |
  77988.6 |
  78000.5 |
  78012.5 |
  78024.5 |
  78036.5 |
  78048.4 |
  78060.4 |
  78072.4 |
  78084.4 |
  78096.3 |
  78108.3 |
  78120.3 |
  78132.3 |########################################
  78144.2 |
  (0 below, 1 above range)

engine_par (n=8, range 107466.7-112606.2 ns)
  107466.7 |########################################
  107723.7 |########################################
  107980.7 |
  108237.6 |
  108494.6 |
  108751.6 |
  109008.6 |
  109265.5 |
  109522.5 |
  109779.5 |########################################
  110036.5 |
  110293.5 |
  110550.4 |
  110807.4 |########################################
  111064.4 |########################################
  111321.4 |
  111578.3 |########################################
  111835.3 |
  112092.3 |########################################
  112349.3 |
  (0 below, 1 above range)

hand (n=8, range 14791.7-15077.1 ns)
  14791.7 |########################################
  14806.0 |########################################
  14820.2 |########################################
  14834.5 |
  14848.8 |
  14863.0 |
  14877.3 |
  14891.6 |
  14905.9 |
  14920.1 |########################################
  14934.4 |
  14948.7 |
  14962.9 |
  14977.2 |
  14991.5 |########################################
  15005.8 |
  15020.0 |
  15034.3 |
  15048.6 |
  15062.8 |
  (0 below, 3 above range)

hand_par (n=8, range 42450.0-50962.5 ns)
  42450.0 |########################################
  42875.6 |####################
  43301.2 |
  43726.9 |
  44152.5 |########################################
  44578.1 |####################
  45003.8 |
  45429.4 |
  45855.0 |
  46280.6 |
  46706.2 |
  47131.9 |
  47557.5 |
  47983.1 |
  48408.8 |
  48834.4 |
  49260.0 |
  49685.6 |
  50111.2 |
  50536.9 |
  (0 below, 2 above range)

rust_nostd (n=8, range 69975.0-70102.1 ns)
  69975.0 |####################
  69981.4 |####################
  69987.7 |####################
  69994.1 |
  70000.4 |
  70006.8 |
  70013.1 |
  70019.5 |
  70025.8 |
  70032.2 |
  70038.5 |
  70044.9 |
  70051.2 |
  70057.6 |
  70064.0 |
  70070.3 |
  70076.7 |
  70083.0 |########################################
  70089.4 |####################
  70095.7 |####################
  (0 below, 1 above range)

rust_std (n=8, range 71283.3-71377.1 ns)
  71283.3 |########################################
  71288.0 |
  71292.7 |
  71297.4 |####################
  71302.1 |
  71306.7 |
  71311.4 |
  71316.1 |####################
  71320.8 |
  71325.5 |
  71330.2 |####################
  71334.9 |
  71339.6 |
  71344.3 |
  71348.9 |
  71353.6 |
  71358.3 |
  71363.0 |
  71367.7 |
  71372.4 |
  (0 below, 3 above range)

rust_std_par (n=8, range 79783.3-93043.8 ns)
  79783.3 |########################################
  80446.3 |
  81109.3 |
  81772.4 |
  82435.4 |########################################
  83098.4 |
  83761.4 |########################################
  84424.5 |########################################
  85087.5 |########################################
  85750.5 |
  86413.5 |
  87076.5 |
  87739.6 |
  88402.6 |
  89065.6 |
  89728.6 |
  90391.7 |########################################
  91054.7 |
  91717.7 |
  92380.7 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=1344.2% of algo (FFI overhead may distort results)
- engine_par: bridge=950.8% of algo (FFI overhead may distort results)
- hand: bridge=7022.7% of algo (FFI overhead may distort results)
- hand_par: bridge=2397.3% of algo (FFI overhead may distort results)
- rust_nostd: bridge=1496.7% of algo (FFI overhead may distort results)
- rust_std: bridge=1470.3% of algo (FFI overhead may distort results)
- rust_std_par: bridge=1241.3% of algo (FFI overhead may distort results)
