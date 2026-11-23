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

### hand dominates: 113% faster than the next best (rust_std)

hand (52.57 us) leads rust_std (112.13 us) by 113%, a clear separation rather than a photo finish. CV 0.8%.


### engine_par is an outlier: 20.8x slower than the field

engine_par (1.09 ms) is 20.8x the fastest (52.57 us), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (hand)

The baseline hand is the fastest (52.57 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 20.8x the fastest

Fastest hand (52.57 us) to slowest engine_par (1.09 ms): 20.8x. The strategy choice matters a lot for this workload.


### engine_par is inconsistent: worst-20% is 2.7x its best-20%

engine_par's best 20% of batches run at 474.07 us but its worst 20% at 1.26 ms (2.7x) - a bimodal or bursty profile the median hides.


## Key findings

- Baseline (hand) is the fastest at 52571.3 ns median
- 6 variants significantly slower than baseline
- Spread: 20.78x (fastest 52571.3 ns, slowest 1092640.5 ns)

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
| engine | 125139ns | 125085ns | 123111ns | 123649ns | 126764ns | +136.98% |
| engine_par | 1058991ns | 1094846ns | 475150ns | 981776ns | 1262862ns | +1905.47% |
| hand | 52805ns | 52799ns | 52316ns | 52455ns | 53190ns | base |
| hand_par | 363202ns | 390240ns | 244156ns | 333943ns | 414908ns | +587.82% |
| rust_nostd | 113929ns | 114069ns | 109030ns | 110025ns | 118082ns | +115.75% |
| rust_std | 161806ns | 112541ns | 111899ns | 112242ns | 211455ns | +206.42% |
| rust_std_par | 574893ns | 632625ns | 391500ns | 528166ns | 655787ns | +988.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 124919ns | 122936ns | 126531ns | +137.39% | 6.296 |
| engine_par | 1056853ns | 474072ns | 1260217ns | +1908.38% | 0.744 |
| hand | 52622ns | 52178ns | 53015ns | base | 14.945 |
| hand_par | 361409ns | 243430ns | 412727ns | +586.80% | 2.176 |
| rust_nostd | 113743ns | 108788ns | 117900ns | +116.15% | 6.914 |
| rust_std | 161160ns | 111650ns | 210380ns | +206.26% | 4.880 |
| rust_std_par | 572411ns | 390942ns | 652704ns | +987.78% | 1.374 |

## Performance model

- Peak throughput: 15.072 Gops/s (hand; best 20% batches)
- Ops per call: 786432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 6.296 | 41.8% |
| engine_par | 0.720 | 4.8% |
| hand | 14.959 | 99.3% |
| hand_par | 2.023 | 13.4% |
| rust_nostd | 6.907 | 45.8% |
| rust_std | 7.013 | 46.5% |
| rust_std_par | 1.250 | 8.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 125139ns | 125139ns | +136.98% |
| engine_par | 1058991ns | 1058991ns | +1905.47% |
| hand | 52805ns | 52805ns | base |
| hand_par | 363202ns | 363202ns | +587.82% |
| rust_nostd | 113929ns | 113929ns | +115.75% |
| rust_std | 161806ns | 161806ns | +206.42% |
| rust_std_par | 574893ns | 574893ns | +988.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 52571ns | base | --- | [52234, 53031] | --- | --- | --- | --- |
| engine | 124909ns | +72249.8ns (+137.4%) | [+70538, +74069]ns | [123330, 126611] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 1092640ns | +1040069.2ns (+1978.4%) | [+911301, +1159768]ns | [963479, 1212958] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 388792ns | +336577.5ns (+640.2%) | [+201237, +374801]ns | [254427, 427690] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 113864ns | +60795.2ns (+115.6%) | [+56956, +65772]ns | [109190, 118066] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 112132ns | +59887.9ns (+113.9%) | [+59021, +63543]ns | [111911, 116492] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 629143ns | +576899.8ns (+1097.4%) | [+413132, +599399]ns | [466322, 651653] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 52571ns | +140.8% | +1978.4% | +683.6% | +124.5% | +113.1% | +956.8% |
| 1 | 2 | 52600ns | +134.5% | +1963.8% | +605.8% | +108.4% | +117.3% | +1096.1% |

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
| engine | 726587.1ns | 124919.4ns | 581.6% | HIGH |
| engine_par | 4510944.8ns | 1056852.6ns | 426.8% | HIGH |
| hand | 724616.4ns | 52622.0ns | 1377.0% | HIGH |
| hand_par | 4305276.6ns | 361409.1ns | 1191.2% | HIGH |
| rust_nostd | 726283.6ns | 113743.2ns | 638.5% | HIGH |
| rust_std | 1040610.4ns | 161159.6ns | 645.7% | HIGH |
| rust_std_par | 5066186.9ns | 572411.5ns | 885.1% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 122936.5-126531.1 ns)
  122936.5 |####################
  123116.2 |
  123296.0 |########################################
  123475.7 |####################
  123655.4 |
  123835.1 |
  124014.9 |
  124194.6 |
  124374.3 |
  124554.1 |
  124733.8 |
  124913.5 |
  125093.2 |
  125273.0 |
  125452.7 |
  125632.4 |
  125812.2 |
  125991.9 |
  126171.6 |####################
  126351.3 |
  (0 below, 3 above range)

engine_par (n=8, range 474071.8-1260217.4 ns)
  474071.8 |####################
  513379.1 |
  552686.4 |
  591993.6 |
  631300.9 |
  670608.2 |
  709915.5 |
  749222.8 |
  788530.0 |
  827837.3 |
  867144.6 |
  906451.9 |
  945759.1 |########################################
  985066.4 |####################
  1024373.7 |
  1063681.0 |
  1102988.3 |
  1142295.5 |####################
  1181602.8 |########################################
  1220910.1 |
  (0 below, 1 above range)

hand (n=8, range 52178.3-53014.6 ns)
  52178.3 |#############
  52220.1 |########################################
  52261.9 |
  52303.7 |
  52345.6 |
  52387.4 |
  52429.2 |
  52471.0 |
  52512.8 |
  52554.6 |
  52596.5 |
  52638.3 |
  52680.1 |
  52721.9 |
  52763.7 |
  52805.5 |
  52847.4 |
  52889.2 |#############
  52931.0 |#############
  52972.8 |
  (0 below, 2 above range)

hand_par (n=8, range 243430.0-412727.3 ns)
  243430.0 |####################
  251894.9 |####################
  260359.7 |
  268824.6 |
  277289.5 |
  285754.3 |
  294219.2 |
  302684.0 |
  311148.9 |
  319613.8 |
  328078.6 |
  336543.5 |
  345008.4 |
  353473.2 |####################
  361938.1 |
  370403.0 |
  378867.8 |####################
  387332.7 |
  395797.5 |########################################
  404262.4 |
  (0 below, 2 above range)

rust_nostd (n=8, range 108788.0-117899.5 ns)
  108788.0 |########################################
  109243.6 |
  109699.2 |####################
  110154.7 |####################
  110610.3 |
  111065.9 |
  111521.5 |
  111977.0 |
  112432.6 |
  112888.2 |
  113343.8 |
  113799.3 |
  114254.9 |
  114710.5 |
  115166.1 |
  115621.6 |
  116077.2 |
  116532.8 |
  116988.4 |####################
  117443.9 |
  (0 below, 3 above range)

rust_std (n=8, range 111650.5-210380.2 ns)
  111650.5 |########################################
  116587.0 |
  121523.5 |
  126460.0 |
  131396.4 |
  136332.9 |
  141269.4 |
  146205.9 |
  151142.4 |
  156078.9 |
  161015.4 |
  165951.8 |
  170888.3 |
  175824.8 |
  180761.3 |
  185697.8 |
  190634.3 |
  195570.8 |
  200507.3 |
  205443.7 |
  (0 below, 1 above range)

rust_std_par (n=8, range 390942.0-652703.8 ns)
  390942.0 |#############
  404030.1 |
  417118.2 |
  430206.3 |
  443294.4 |
  456382.5 |#############
  469470.5 |
  482558.6 |
  495646.7 |#############
  508734.8 |
  521822.9 |
  534911.0 |
  547999.1 |
  561087.2 |
  574175.3 |
  587263.4 |
  600351.5 |
  613439.6 |#############
  626527.6 |
  639615.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- engine: bridge=581.8% of algo (FFI overhead may distort results)
- engine_par: CV=25.3% (high variance, measurements may be unstable)
- engine_par: bridge=393.6% of algo (FFI overhead may distort results)
- hand: bridge=1377.0% of algo (FFI overhead may distort results)
- hand_par: bridge=1122.5% of algo (FFI overhead may distort results)
- rust_nostd: bridge=637.7% of algo (FFI overhead may distort results)
- rust_std: CV=79.5% (high variance, measurements may be unstable)
- rust_std: bridge=648.5% of algo (FFI overhead may distort results)
- rust_std_par: bridge=844.9% of algo (FFI overhead may distort results)
