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

The declared/defaulted baseline hand has the worst median (834.58 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest rust_std_par at 213.09 us).


### rust_std_par dominates: 15% faster than the next best (hand_par)

rust_std_par (213.09 us) leads hand_par (246.02 us) by 15%, a clear separation rather than a photo finish. CV 6.7%.


### rust_std_par beats baseline by 74% (significant)

rust_std_par is -619.45 us (74%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 3.9x slower than the field

hand (834.58 us) is 3.9x the fastest (213.09 us), well off the pack.


### rust_std_par is fastest but the noisiest (CV 6.7%)

rust_std_par wins on median (213.09 us) yet has the highest variance (CV 6.7%), while rust_std is the steadiest (CV 0.1%, 581.70 us).


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {rust_std_par, hand_par} vs {engine_par, rust_nostd, rust_std, engine, hand} (120% apart)

The field splits into a fast tier {rust_std_par, hand_par} and a slow tier {engine_par, rust_nostd, rust_std, engine, hand} with a 120% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 3.9x the fastest

Fastest rust_std_par (213.09 us) to slowest hand (834.58 us): 3.9x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: rust_std_par at 213087.5 ns median (-74.5% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 3.92x (fastest 213087.5 ns, slowest 834579.2 ns)

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
| engine | 593728ns | 590850ns | 589417ns | 590156ns | 597485ns | -28.94% |
| engine_par | 544704ns | 542362ns | 537000ns | 541114ns | 549323ns | -34.81% |
| hand | 835564ns | 835767ns | 833758ns | 834906ns | 836508ns | base |
| hand_par | 250960ns | 247342ns | 244508ns | 245853ns | 256404ns | -69.97% |
| rust_nostd | 579064ns | 578450ns | 576625ns | 577922ns | 580529ns | -30.70% |
| rust_std | 582639ns | 582321ns | 581950ns | 582164ns | 583167ns | -30.27% |
| rust_std_par | 221743ns | 215008ns | 204083ns | 213511ns | 232331ns | -73.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 592621ns | 589308ns | 595531ns | -28.96% | 1.327 |
| engine_par | 543714ns | 536542ns | 548267ns | -34.83% | 1.446 |
| hand | 834243ns | 832492ns | 835435ns | base | 0.943 |
| hand_par | 249943ns | 243775ns | 255450ns | -70.04% | 3.146 |
| rust_nostd | 578010ns | 576250ns | 579529ns | -30.71% | 1.361 |
| rust_std | 581803ns | 580775ns | 582554ns | -30.26% | 1.352 |
| rust_std_par | 220250ns | 203608ns | 231056ns | -73.60% | 3.571 |

## Performance model

- Peak throughput: 3.862 Gops/s (rust_std_par; best 20% batches)
- Ops per call: 786432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 1.332 | 34.5% |
| engine_par | 1.452 | 37.6% |
| hand | 0.942 | 24.4% |
| hand_par | 3.197 | 82.8% |
| rust_nostd | 1.363 | 35.3% |
| rust_std | 1.352 | 35.0% |
| rust_std_par | 3.691 | 95.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 593728ns | 593728ns | -28.94% |
| engine_par | 544704ns | 544704ns | -34.81% |
| hand | 835564ns | 835564ns | base |
| hand_par | 250960ns | 250960ns | -69.97% |
| rust_nostd | 579064ns | 579064ns | -30.70% |
| rust_std | 582639ns | 582639ns | -30.27% |
| rust_std_par | 221743ns | 221743ns | -73.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 834579ns | base | --- | [832592, 835675] | --- | --- | --- | --- |
| engine | 590388ns | -243708.3ns (-29.2%) | [-245867, -239917]ns | [589583, 594192] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 541683ns | -293308.3ns (-35.1%) | [-295950, -279200]ns | [537333, 553808] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 246021ns | -587579.2ns (-70.4%) | [-588967, -582233]ns | [244042, 252667] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 577083ns | -256629.2ns (-30.7%) | [-259142, -254758]ns | [576317, 578942] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 581704ns | -252412.5ns (-30.2%) | [-254800, -250408]ns | [580875, 582958] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 213088ns | -619454.2ns (-74.2%) | [-624442, -601917]ns | [209667, 233758] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 832800ns | -28.8% | -34.2% | -70.6% | -30.7% | -30.1% | -74.5% |
| 1 | 2 | 835425ns | -29.4% | -35.2% | -70.1% | -31.0% | -30.5% | -72.4% |

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
| engine | 1054679.2ns | 592620.8ns | 178.0% | HIGH |
| engine_par | 1059146.9ns | 543713.5ns | 194.8% | HIGH |
| hand | 1050336.4ns | 834242.7ns | 125.9% | HIGH |
| hand_par | 1069450.0ns | 249942.7ns | 427.9% | HIGH |
| rust_nostd | 1051150.0ns | 578010.4ns | 181.9% | HIGH |
| rust_std | 1049789.6ns | 581803.1ns | 180.4% | HIGH |
| rust_std_par | 1071185.4ns | 220250.0ns | 486.3% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 589308.3-595531.2 ns)
  589308.3 |########################################
  589619.4 |####################
  589930.6 |####################
  590241.7 |
  590552.9 |####################
  590864.0 |####################
  591175.2 |
  591486.3 |
  591797.5 |
  592108.6 |
  592419.8 |
  592730.9 |
  593042.1 |
  593353.2 |
  593664.4 |
  593975.5 |####################
  594286.7 |
  594597.8 |
  594909.0 |
  595220.1 |
  (0 below, 1 above range)

engine_par (n=8, range 536541.7-548266.7 ns)
  536541.7 |####################
  537127.9 |####################
  537714.2 |
  538300.4 |
  538886.7 |
  539472.9 |
  540059.2 |
  540645.4 |####################
  541231.7 |########################################
  541817.9 |
  542404.2 |####################
  542990.4 |
  543576.7 |
  544162.9 |
  544749.2 |
  545335.4 |
  545921.7 |
  546507.9 |
  547094.2 |
  547680.4 |
  (0 below, 2 above range)

hand (n=8, range 832491.7-835435.4 ns)
  832491.7 |########################################
  832638.9 |
  832786.1 |
  832933.3 |####################
  833080.4 |
  833227.6 |
  833374.8 |
  833522.0 |
  833669.2 |
  833816.4 |
  833963.6 |####################
  834110.7 |
  834257.9 |
  834405.1 |
  834552.3 |
  834699.5 |
  834846.7 |
  834993.9 |####################
  835141.1 |####################
  835288.2 |
  (0 below, 2 above range)

hand_par (n=8, range 243775.0-255450.0 ns)
  243775.0 |########################################
  244358.8 |
  244942.5 |
  245526.3 |#############
  246110.0 |#############
  246693.8 |
  247277.5 |
  247861.3 |
  248445.0 |
  249028.8 |
  249612.5 |
  250196.3 |
  250780.0 |
  251363.8 |#############
  251947.5 |
  252531.3 |#############
  253115.0 |
  253698.8 |
  254282.5 |
  254866.3 |
  (0 below, 1 above range)

rust_nostd (n=8, range 576250.0-579529.2 ns)
  576250.0 |########################################
  576414.0 |####################
  576577.9 |
  576741.9 |
  576905.8 |####################
  577069.8 |####################
  577233.7 |
  577397.7 |
  577561.7 |
  577725.6 |####################
  577889.6 |
  578053.5 |
  578217.5 |
  578381.4 |
  578545.4 |
  578709.4 |
  578873.3 |####################
  579037.3 |
  579201.2 |
  579365.2 |
  (0 below, 1 above range)

rust_std (n=8, range 580775.0-582554.2 ns)
  580775.0 |########################################
  580864.0 |########################################
  580952.9 |
  581041.9 |
  581130.8 |########################################
  581219.8 |
  581308.7 |########################################
  581397.7 |
  581486.7 |
  581575.6 |
  581664.6 |
  581753.5 |
  581842.5 |
  581931.4 |
  582020.4 |########################################
  582109.4 |########################################
  582198.3 |
  582287.3 |
  582376.2 |
  582465.2 |
  (0 below, 2 above range)

rust_std_par (n=8, range 203608.3-231056.2 ns)
  203608.3 |########################################
  204980.7 |
  206353.1 |
  207725.5 |
  209097.9 |########################################
  210470.3 |########################################
  211842.7 |########################################
  213215.1 |########################################
  214587.5 |
  215959.9 |
  217332.3 |
  218704.7 |
  220077.1 |
  221449.5 |
  222821.8 |
  224194.2 |
  225566.6 |
  226939.0 |########################################
  228311.4 |
  229683.8 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=177.8% of algo (FFI overhead may distort results)
- engine_par: bridge=194.8% of algo (FFI overhead may distort results)
- hand: bridge=125.7% of algo (FFI overhead may distort results)
- hand_par: bridge=435.4% of algo (FFI overhead may distort results)
- rust_nostd: bridge=181.8% of algo (FFI overhead may distort results)
- rust_std: bridge=180.2% of algo (FFI overhead may distort results)
- rust_std_par: bridge=497.2% of algo (FFI overhead may distort results)
