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

### rust_std_par is an outlier: 5.7x slower than the field

rust_std_par (44.23 ms) is 5.7x the fastest (7.81 ms), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (hand)

The baseline hand is the fastest (7.81 ms median); no rival improves on it (all deltas are >= 0).


### Two tiers: {hand, hand_par} vs {rust_nostd, rust_std, engine_par, engine, rust_std_par} (265% apart)

The field splits into a fast tier {hand, hand_par} and a slow tier {rust_nostd, rust_std, engine_par, engine, rust_std_par} with a 265% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 5.7x the fastest

Fastest hand (7.81 ms) to slowest rust_std_par (44.23 ms): 5.7x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (hand) is the fastest at 7805229.2 ns median
- 6 variants significantly slower than baseline
- Spread: 5.67x (fastest 7805229.2 ns, slowest 44228329.1 ns)

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
| engine | 33835659ns | 33633538ns | 32627500ns | 33211664ns | 34605696ns | +328.45% |
| engine_par | 32285776ns | 31975625ns | 31516375ns | 31779153ns | 32858094ns | +308.83% |
| hand | 7897149ns | 7809388ns | 7718467ns | 7753594ns | 8049485ns | base |
| hand_par | 8150334ns | 8138162ns | 8088750ns | 8131006ns | 8180227ns | +3.21% |
| rust_nostd | 29924092ns | 29691025ns | 29287825ns | 29394536ns | 30480325ns | +278.92% |
| rust_std | 30049041ns | 29942242ns | 29221742ns | 29589419ns | 30600581ns | +280.50% |
| rust_std_par | 44315379ns | 44233600ns | 44092933ns | 44196161ns | 44460404ns | +461.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 33829781ns | 32622933ns | 34599579ns | +328.69% | 5.951 |
| engine_par | 32280058ns | 31510108ns | 32852481ns | +309.05% | 6.237 |
| hand | 7891514ns | 7714842ns | 8043608ns | base | 25.512 |
| hand_par | 8143205ns | 8080267ns | 8172896ns | +3.19% | 24.723 |
| rust_nostd | 29917918ns | 29282358ns | 30473365ns | +279.12% | 6.729 |
| rust_std | 30043507ns | 29215750ns | 30595050ns | +280.71% | 6.701 |
| rust_std_par | 44309956ns | 44086842ns | 44455175ns | +461.49% | 4.544 |

## Performance model

- Peak throughput: 26.096 Gops/s (hand; best 20% batches)
- Ops per call: 201326592

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 5.987 | 22.9% |
| engine_par | 6.297 | 24.1% |
| hand | 25.794 | 98.8% |
| hand_par | 24.760 | 94.9% |
| rust_nostd | 6.782 | 26.0% |
| rust_std | 6.725 | 25.8% |
| rust_std_par | 4.552 | 17.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 33835659ns | 33835659ns | +328.45% |
| engine_par | 32285776ns | 32285776ns | +308.83% |
| hand | 7897149ns | 7897149ns | base |
| hand_par | 8150334ns | 8150334ns | +3.21% |
| rust_nostd | 29924092ns | 29924092ns | +278.92% |
| rust_std | 30049041ns | 30049041ns | +280.50% |
| rust_std_par | 44315379ns | 44315379ns | +461.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 7805229ns | base | --- | [7727358, 8124358] | --- | --- | --- | --- |
| engine | 33627454ns | +25771195.9ns (+330.2%) | [+24802275, +27041200]ns | [32651617, 34864342] | YES | 0.0094 | 0.0078 | 0 |
| engine_par | 31969546ns | +24026808.4ns (+307.8%) | [+23839858, +24861442]ns | [31628733, 33110775] | YES | 0.0094 | 0.0078 | 0 |
| hand_par | 8131079ns | +320016.6ns (+4.1%) | [+45292, +375650]ns | [8118342, 8179758] | YES (adj: no) | 0.0703 | 0.0703 | 0 |
| rust_nostd | 29684517ns | +21896166.6ns (+280.5%) | [+21521242, +22755950]ns | [29319325, 30510308] | YES | 0.0094 | 0.0078 | 0 |
| rust_std | 29937388ns | +22206908.4ns (+284.5%) | [+21552067, +22670925]ns | [29401408, 30622325] | YES | 0.0094 | 0.0078 | 0 |
| rust_std_par | 44228329ns | +36484091.6ns (+467.4%) | [+36027050, +36659058]ns | [44151408, 44571125] | YES | 0.0094 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 7900371ns | +319.3% | +310.8% | +2.9% | +280.7% | +276.9% | +460.8% |
| 1 | 2 | 7744238ns | +342.4% | +312.8% | +5.2% | +279.9% | +286.6% | +471.1% |

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
| engine | 270723613.6ns | 33829781.3ns | 800.3% | HIGH |
| engine_par | 270746771.9ns | 32280058.3ns | 838.7% | HIGH |
| hand | 270865516.7ns | 7891513.5ns | 3432.4% | HIGH |
| hand_par | 270631668.7ns | 8143205.2ns | 3323.4% | HIGH |
| rust_nostd | 270748048.9ns | 29917917.7ns | 905.0% | HIGH |
| rust_std | 270901267.7ns | 30043507.3ns | 901.7% | HIGH |
| rust_std_par | 270830918.7ns | 44309956.2ns | 611.2% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 32622933.3-34599579.2 ns)
  32622933.3 |########################################
  32721765.6 |
  32820597.9 |
  32919430.2 |
  33018262.5 |
  33117094.8 |
  33215927.1 |
  33314759.4 |####################
  33413591.6 |
  33512423.9 |####################
  33611256.2 |####################
  33710088.5 |
  33808920.8 |
  33907753.1 |
  34006585.4 |
  34105417.7 |
  34204250.0 |
  34303082.3 |
  34401914.6 |
  34500746.9 |
  (0 below, 3 above range)

engine_par (n=8, range 31510108.3-32852481.2 ns)
  31510108.3 |########################################
  31577226.9 |########################################
  31644345.6 |
  31711464.2 |
  31778582.9 |########################################
  31845701.5 |########################################
  31912820.2 |
  31979938.8 |########################################
  32047057.5 |
  32114176.1 |########################################
  32181294.8 |
  32248413.4 |
  32315532.1 |
  32382650.7 |
  32449769.4 |
  32516888.0 |
  32584006.7 |
  32651125.3 |
  32718244.0 |
  32785362.6 |
  (0 below, 2 above range)

hand (n=8, range 7714841.7-8043608.3 ns)
  7714841.7 |########################################
  7731280.0 |
  7747718.4 |########################################
  7764156.7 |
  7780595.0 |
  7797033.4 |
  7813471.7 |
  7829910.0 |
  7846348.4 |####################
  7862786.7 |
  7879225.0 |
  7895663.3 |
  7912101.7 |
  7928540.0 |
  7944978.3 |####################
  7961416.7 |
  7977855.0 |
  7994293.3 |
  8010731.7 |
  8027170.0 |
  (0 below, 2 above range)

hand_par (n=8, range 8080266.7-8172895.8 ns)
  8080266.7 |########################################
  8084898.2 |
  8089529.6 |
  8094161.1 |
  8098792.5 |
  8103424.0 |
  8108055.4 |
  8112686.9 |
  8117318.4 |########################################
  8121949.8 |########################################
  8126581.3 |########################################
  8131212.7 |########################################
  8135844.2 |
  8140475.6 |
  8145107.1 |
  8149738.5 |
  8154370.0 |
  8159001.5 |
  8163632.9 |
  8168264.4 |########################################
  (0 below, 2 above range)

rust_nostd (n=8, range 29282358.3-30473364.6 ns)
  29282358.3 |########################################
  29341908.6 |
  29401458.9 |
  29461009.2 |
  29520559.6 |#############
  29580109.9 |
  29639660.2 |
  29699210.5 |
  29758760.8 |
  29818311.1 |#############
  29877861.4 |
  29937411.8 |
  29996962.1 |
  30056512.4 |
  30116062.7 |
  30175613.0 |
  30235163.3 |
  30294713.6 |#############
  30354263.9 |
  30413814.3 |
  (0 below, 2 above range)

rust_std (n=8, range 29215750.0-30595050.0 ns)
  29215750.0 |####################
  29284715.0 |
  29353680.0 |####################
  29422645.0 |
  29491610.0 |
  29560575.0 |####################
  29629540.0 |
  29698505.0 |####################
  29767470.0 |
  29836435.0 |
  29905400.0 |
  29974365.0 |
  30043330.0 |
  30112295.0 |########################################
  30181260.0 |
  30250225.0 |
  30319190.0 |
  30388155.0 |
  30457120.0 |
  30526085.0 |
  (0 below, 2 above range)

rust_std_par (n=8, range 44086841.7-44455175.0 ns)
  44086841.7 |####################
  44105258.4 |
  44123675.0 |
  44142091.7 |####################
  44160508.4 |
  44178925.0 |####################
  44197341.7 |
  44215758.4 |########################################
  44234175.0 |
  44252591.7 |
  44271008.4 |
  44289425.0 |
  44307841.7 |
  44326258.3 |
  44344675.0 |
  44363091.7 |
  44381508.3 |
  44399925.0 |####################
  44418341.7 |
  44436758.3 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=805.1% of algo (FFI overhead may distort results)
- engine_par: bridge=846.6% of algo (FFI overhead may distort results)
- hand: bridge=3467.6% of algo (FFI overhead may distort results)
- hand_par: bridge=3326.3% of algo (FFI overhead may distort results)
- rust_nostd: bridge=911.3% of algo (FFI overhead may distort results)
- rust_std: bridge=903.7% of algo (FFI overhead may distort results)
- rust_std_par: bridge=612.2% of algo (FFI overhead may distort results)
