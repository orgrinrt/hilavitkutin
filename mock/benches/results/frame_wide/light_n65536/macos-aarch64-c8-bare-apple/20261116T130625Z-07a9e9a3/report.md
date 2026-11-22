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

### hand dominates: 190% faster than the next best (hand_par)

hand (15.00 us) leads hand_par (43.54 us) by 190%, a clear separation rather than a photo finish. CV 0.7%.


### engine_par is an outlier: 7.3x slower than the field

engine_par (109.97 us) is 7.3x the fastest (15.00 us), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (hand)

The baseline hand is the fastest (15.00 us median); no rival improves on it (all deltas are >= 0).


### Two tiers: {hand} vs {hand_par, rust_nostd, rust_std, engine, rust_std_par, engine_par} (190% apart)

The field splits into a fast tier {hand} and a slow tier {hand_par, rust_nostd, rust_std, engine, rust_std_par, engine_par} with a 190% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 7.3x the fastest

Fastest hand (15.00 us) to slowest engine_par (109.97 us): 7.3x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (hand) is the fastest at 15000.0 ns median
- 6 variants significantly slower than baseline
- Spread: 7.33x (fastest 15000.0 ns, slowest 109975.0 ns)

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
| engine | 79076ns | 79125ns | 78633ns | 78897ns | 79321ns | +381.80% |
| engine_par | 111170ns | 112150ns | 103042ns | 111014ns | 113319ns | +577.35% |
| hand | 16413ns | 16225ns | 15492ns | 16108ns | 16871ns | base |
| hand_par | 45960ns | 45554ns | 44342ns | 44669ns | 47333ns | +180.03% |
| rust_nostd | 71129ns | 71208ns | 70683ns | 71031ns | 71315ns | +333.38% |
| rust_std | 72049ns | 72062ns | 71475ns | 71664ns | 72481ns | +338.99% |
| rust_std_par | 86822ns | 85488ns | 83375ns | 84783ns | 89212ns | +429.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 78017ns | 77875ns | 78083ns | +421.31% | 10.080 |
| engine_par | 109771ns | 102142ns | 112106ns | +633.49% | 7.164 |
| hand | 14966ns | 14767ns | 15040ns | base | 52.549 |
| hand_par | 44141ns | 42650ns | 45267ns | +194.95% | 17.817 |
| rust_nostd | 70071ns | 69975ns | 70133ns | +368.21% | 11.223 |
| rust_std | 71441ns | 71267ns | 71581ns | +377.36% | 11.008 |
| rust_std_par | 85067ns | 81958ns | 87421ns | +468.41% | 9.245 |

## Performance model

- Peak throughput: 53.257 Gops/s (hand; best 20% batches)
- Ops per call: 786432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 10.082 | 18.9% |
| engine_par | 7.151 | 13.4% |
| hand | 52.429 | 98.4% |
| hand_par | 18.063 | 33.9% |
| rust_nostd | 11.222 | 21.1% |
| rust_std | 11.027 | 20.7% |
| rust_std_par | 9.435 | 17.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 79076ns | 79076ns | +381.80% |
| engine_par | 111170ns | 111170ns | +577.35% |
| hand | 16413ns | 16413ns | base |
| hand_par | 45960ns | 45960ns | +180.03% |
| rust_nostd | 71129ns | 71129ns | +333.38% |
| rust_std | 72049ns | 72049ns | +338.99% |
| rust_std_par | 86822ns | 86822ns | +429.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 15000ns | base | --- | [14858, 15058] | --- | --- | --- | --- |
| engine | 78004ns | +63050.0ns (+420.3%) | [+62900, +63208]ns | [77950, 78092] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 109975ns | +94991.7ns (+633.3%) | [+93008, +97867]ns | [108067, 112633] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 43538ns | +28554.2ns (+190.4%) | [+27875, +29983]ns | [42867, 44842] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 70079ns | +55095.8ns (+367.3%) | [+54983, +55208]ns | [69983, 70125] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 71317ns | +56395.8ns (+376.0%) | [+56283, +56550]ns | [71300, 71433] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 83354ns | +68450.0ns (+456.3%) | [+67342, +73612]ns | [82333, 89025] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 15012ns | +419.9% | +632.6% | +191.7% | +366.3% | +375.0% | +464.8% |
| 1 | 2 | 14904ns | +423.3% | +643.9% | +190.3% | +370.4% | +378.7% | +459.3% |

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
| engine | 1049095.8ns | 78016.7ns | 1344.7% | HIGH |
| engine_par | 1057024.0ns | 109770.8ns | 962.9% | HIGH |
| hand | 1048994.8ns | 14965.6ns | 7009.4% | HIGH |
| hand_par | 1068444.8ns | 44140.6ns | 2420.5% | HIGH |
| rust_nostd | 1049258.3ns | 70070.8ns | 1497.4% | HIGH |
| rust_std | 1049661.5ns | 71440.6ns | 1469.3% | HIGH |
| rust_std_par | 1055332.3ns | 85066.6ns | 1240.6% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 77875.0-78083.4 ns)
  77875.0 |########################################
  77885.4 |
  77895.8 |
  77906.3 |
  77916.7 |
  77927.1 |
  77937.5 |
  77947.9 |########################################
  77958.3 |
  77968.8 |########################################
  77979.2 |
  77989.6 |########################################
  78000.0 |########################################
  78010.4 |########################################
  78020.8 |
  78031.3 |
  78041.7 |
  78052.1 |
  78062.5 |
  78072.9 |
  (0 below, 2 above range)

engine_par (n=8, range 102141.7-112106.2 ns)
  102141.7 |####################
  102639.9 |
  103138.2 |
  103636.4 |
  104134.6 |
  104632.8 |
  105131.1 |
  105629.3 |
  106127.5 |
  106625.7 |
  107124.0 |
  107622.2 |####################
  108120.4 |
  108618.6 |
  109116.9 |
  109615.1 |########################################
  110113.3 |####################
  110611.5 |
  111109.8 |
  111608.0 |####################
  (0 below, 2 above range)

hand (n=8, range 14766.7-15039.6 ns)
  14766.7 |########################################
  14780.3 |
  14794.0 |
  14807.6 |
  14821.3 |
  14834.9 |
  14848.6 |########################################
  14862.2 |
  14875.9 |
  14889.5 |
  14903.1 |
  14916.8 |
  14930.4 |
  14944.1 |########################################
  14957.7 |
  14971.4 |
  14985.0 |########################################
  14998.6 |########################################
  15012.3 |########################################
  15025.9 |
  (0 below, 2 above range)

hand_par (n=8, range 42650.0-45266.7 ns)
  42650.0 |####################
  42780.8 |####################
  42911.7 |####################
  43042.5 |
  43173.3 |
  43304.2 |
  43435.0 |########################################
  43565.8 |
  43696.7 |
  43827.5 |
  43958.3 |####################
  44089.2 |
  44220.0 |
  44350.8 |
  44481.7 |
  44612.5 |
  44743.3 |####################
  44874.2 |
  45005.0 |
  45135.8 |
  (0 below, 1 above range)

rust_nostd (n=8, range 69975.0-70133.4 ns)
  69975.0 |########################################
  69982.9 |########################################
  69990.8 |
  69998.8 |
  70006.7 |########################################
  70014.6 |
  70022.5 |
  70030.4 |
  70038.3 |
  70046.3 |
  70054.2 |
  70062.1 |########################################
  70070.0 |
  70077.9 |
  70085.8 |########################################
  70093.8 |
  70101.7 |
  70109.6 |########################################
  70117.5 |########################################
  70125.4 |
  (0 below, 1 above range)

rust_std (n=8, range 71266.7-71581.2 ns)
  71266.7 |#############
  71282.4 |
  71298.2 |#############
  71313.9 |########################################
  71329.6 |
  71345.3 |
  71361.1 |
  71376.8 |#############
  71392.5 |
  71408.2 |
  71424.0 |#############
  71439.7 |
  71455.4 |
  71471.2 |
  71486.9 |
  71502.6 |
  71518.3 |
  71534.1 |
  71549.8 |
  71565.5 |
  (0 below, 1 above range)

rust_std_par (n=8, range 81958.3-87420.8 ns)
  81958.3 |####################
  82231.4 |####################
  82504.6 |
  82777.7 |
  83050.8 |########################################
  83323.9 |####################
  83597.1 |
  83870.2 |
  84143.3 |
  84416.4 |
  84689.6 |
  84962.7 |
  85235.8 |
  85508.9 |
  85782.1 |
  86055.2 |####################
  86328.3 |
  86601.4 |
  86874.6 |
  87147.7 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=1345.0% of algo (FFI overhead may distort results)
- engine_par: bridge=961.6% of algo (FFI overhead may distort results)
- hand: bridge=6994.3% of algo (FFI overhead may distort results)
- hand_par: bridge=2454.0% of algo (FFI overhead may distort results)
- rust_nostd: bridge=1496.8% of algo (FFI overhead may distort results)
- rust_std: bridge=1471.8% of algo (FFI overhead may distort results)
- rust_std_par: bridge=1264.8% of algo (FFI overhead may distort results)
