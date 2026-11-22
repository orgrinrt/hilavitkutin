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

The declared/defaulted baseline hand has the worst median (213.92 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest rust_std_par at 42.32 ms).


### rust_std_par dominates: 12% faster than the next best (hand_par)

rust_std_par (42.32 ms) leads hand_par (47.54 ms) by 12%, a clear separation rather than a photo finish. CV 1.0%.


### rust_std_par beats baseline by 80% (significant)

rust_std_par is -171.61 ms (80%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 5.1x slower than the field

hand (213.92 ms) is 5.1x the fastest (42.32 ms), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {rust_std_par, hand_par} vs {engine_par, rust_nostd, engine, rust_std, hand} (168% apart)

The field splits into a fast tier {rust_std_par, hand_par} and a slow tier {engine_par, rust_nostd, engine, rust_std, hand} with a 168% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 5.1x the fastest

Fastest rust_std_par (42.32 ms) to slowest hand (213.92 ms): 5.1x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: rust_std_par at 42315933.3 ns median (-80.2% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 5.06x (fastest 42315933.3 ns, slowest 213921687.5 ns)

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
| engine | 151125507ns | 150961904ns | 150108008ns | 150603922ns | 151771071ns | -29.40% |
| engine_par | 127256653ns | 127284058ns | 126328575ns | 126934775ns | 127730081ns | -40.55% |
| hand | 214060667ns | 213928462ns | 213652433ns | 213824408ns | 214339919ns | base |
| hand_par | 47408292ns | 47548712ns | 45733625ns | 47037231ns | 48105254ns | -77.85% |
| rust_nostd | 151216590ns | 150773362ns | 150378425ns | 150633869ns | 151863171ns | -29.36% |
| rust_std | 151702748ns | 151638683ns | 151132342ns | 151322861ns | 152130265ns | -29.13% |
| rust_std_par | 42471056ns | 42321521ns | 42040892ns | 42191900ns | 42787965ns | -80.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 151119773ns | 150104300ns | 151765360ns | -29.40% | 1.332 |
| engine_par | 127251919ns | 126322042ns | 127725590ns | -40.55% | 1.582 |
| hand | 214055523ns | 213649875ns | 214333869ns | base | 0.941 |
| hand_par | 47402700ns | 45727442ns | 48100017ns | -77.85% | 4.247 |
| rust_nostd | 151210621ns | 150368092ns | 151857510ns | -29.36% | 1.331 |
| rust_std | 151697055ns | 151127475ns | 152124265ns | -29.13% | 1.327 |
| rust_std_par | 42465802ns | 42036083ns | 42782954ns | -80.16% | 4.741 |

## Performance model

- Peak throughput: 4.789 Gops/s (rust_std_par; best 20% batches)
- Ops per call: 201326592

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 1.334 | 27.8% |
| engine_par | 1.582 | 33.0% |
| hand | 0.941 | 19.7% |
| hand_par | 4.235 | 88.4% |
| rust_nostd | 1.335 | 27.9% |
| rust_std | 1.328 | 27.7% |
| rust_std_par | 4.758 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 151125507ns | 151125507ns | -29.40% |
| engine_par | 127256653ns | 127256653ns | -40.55% |
| hand | 214060667ns | 214060667ns | base |
| hand_par | 47408292ns | 47408292ns | -77.85% |
| rust_nostd | 151216590ns | 151216590ns | -29.36% |
| rust_std | 151702748ns | 151702748ns | -29.13% |
| rust_std_par | 42471056ns | 42471056ns | -80.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 213921688ns | base | --- | [213750100, 214491192] | --- | --- | --- | --- |
| engine | 150956425ns | -62743562.5ns (-29.3%) | [-63785858, -62124658]ns | [150200242, 152256058] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 127280083ns | -86946937.5ns (-40.6%) | [-87064275, -86308083]ns | [126771158, 127922683] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 47543450ns | -166432558.3ns (-77.8%) | [-167840075, -165525258]ns | [46712917, 48124617] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 150768029ns | -63069904.2ns (-29.5%) | [-63758317, -62786358]ns | [150515575, 150987708] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 151634179ns | -62381054.2ns (-29.2%) | [-62720442, -61985250]ns | [151175858, 151892208] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 42315933ns | -171605754.2ns (-80.2%) | [-172314508, -171179750]ns | [42171100, 42806350] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 213868100ns | -29.5% | -40.5% | -77.6% | -29.6% | -29.3% | -80.2% |
| 1 | 2 | 213921688ns | -29.0% | -40.5% | -78.0% | -29.5% | -29.0% | -80.2% |

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
| engine | 270974354.2ns | 151119772.9ns | 179.3% | HIGH |
| engine_par | 270543357.3ns | 127251918.7ns | 212.6% | HIGH |
| hand | 271135882.3ns | 214055522.9ns | 126.7% | HIGH |
| hand_par | 271355989.6ns | 47402700.0ns | 572.4% | HIGH |
| rust_nostd | 270966705.2ns | 151210620.8ns | 179.2% | HIGH |
| rust_std | 270575801.0ns | 151697055.2ns | 178.4% | HIGH |
| rust_std_par | 270592520.8ns | 42465802.1ns | 637.2% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 150104300.0-151765360.4 ns)
  150104300.0 |####################
  150187353.0 |####################
  150270406.0 |
  150353459.1 |
  150436512.1 |
  150519565.1 |
  150602618.1 |####################
  150685671.1 |
  150768724.2 |
  150851777.2 |
  150934830.2 |########################################
  151017883.2 |
  151100936.2 |
  151183989.3 |
  151267042.3 |
  151350095.3 |
  151433148.3 |####################
  151516201.3 |
  151599254.4 |
  151682307.4 |
  (0 below, 2 above range)

engine_par (n=8, range 126322041.7-127725589.6 ns)
  126322041.7 |########################################
  126392219.1 |
  126462396.5 |
  126532573.9 |
  126602751.3 |
  126672928.7 |
  126743106.1 |########################################
  126813283.5 |
  126883460.9 |########################################
  126953638.2 |
  127023815.6 |
  127093993.0 |########################################
  127164170.4 |
  127234347.8 |
  127304525.2 |
  127374702.6 |########################################
  127444880.0 |
  127515057.4 |########################################
  127585234.8 |
  127655412.2 |
  (0 below, 2 above range)

hand (n=8, range 213649875.0-214333868.8 ns)
  213649875.0 |####################
  213684074.7 |
  213718274.4 |####################
  213752474.1 |
  213786673.8 |
  213820873.4 |####################
  213855073.1 |####################
  213889272.8 |
  213923472.5 |
  213957672.2 |########################################
  213991871.9 |
  214026071.6 |
  214060271.3 |
  214094471.0 |
  214128670.6 |
  214162870.3 |
  214197070.0 |
  214231269.7 |
  214265469.4 |
  214299669.1 |
  (0 below, 2 above range)

hand_par (n=8, range 45727441.7-48100016.7 ns)
  45727441.7 |####################
  45846070.4 |
  45964699.2 |
  46083327.9 |
  46201956.7 |
  46320585.4 |
  46439214.2 |
  46557842.9 |
  46676471.7 |####################
  46795100.4 |
  46913729.2 |
  47032357.9 |####################
  47150986.7 |
  47269615.4 |####################
  47388244.2 |
  47506872.9 |
  47625501.7 |
  47744130.4 |########################################
  47862759.2 |
  47981387.9 |
  (0 below, 2 above range)

rust_nostd (n=8, range 150368091.7-151857510.4 ns)
  150368091.7 |########################################
  150442562.6 |########################################
  150517033.6 |
  150591504.5 |########################################
  150665975.4 |########################################
  150740446.4 |########################################
  150814917.3 |########################################
  150889388.2 |
  150963859.2 |########################################
  151038330.1 |
  151112801.1 |
  151187272.0 |
  151261742.9 |
  151336213.9 |
  151410684.8 |
  151485155.7 |
  151559626.7 |
  151634097.6 |
  151708568.5 |
  151783039.5 |
  (0 below, 1 above range)

rust_std (n=8, range 151127475.0-152124264.6 ns)
  151127475.0 |########################################
  151177314.5 |
  151227154.0 |####################
  151276993.4 |
  151326832.9 |
  151376672.4 |
  151426511.9 |
  151476351.4 |####################
  151526190.8 |
  151576030.3 |
  151625869.8 |
  151675709.3 |
  151725548.7 |####################
  151775388.2 |####################
  151825227.7 |
  151875067.2 |####################
  151924906.7 |
  151974746.1 |
  152024585.6 |
  152074425.1 |
  (0 below, 1 above range)

rust_std_par (n=8, range 42036083.3-42782954.2 ns)
  42036083.3 |####################
  42073426.8 |
  42110770.4 |
  42148113.9 |########################################
  42185457.5 |####################
  42222801.0 |
  42260144.6 |
  42297488.1 |
  42334831.6 |
  42372175.2 |
  42409518.7 |####################
  42446862.3 |
  42484205.8 |
  42521549.4 |####################
  42558892.9 |
  42596236.5 |
  42633580.0 |
  42670923.5 |
  42708267.1 |
  42745610.6 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=179.3% of algo (FFI overhead may distort results)
- engine_par: bridge=212.5% of algo (FFI overhead may distort results)
- hand: bridge=126.7% of algo (FFI overhead may distort results)
- hand_par: bridge=570.4% of algo (FFI overhead may distort results)
- rust_nostd: bridge=179.5% of algo (FFI overhead may distort results)
- rust_std: bridge=178.4% of algo (FFI overhead may distort results)
- rust_std_par: bridge=639.1% of algo (FFI overhead may distort results)
