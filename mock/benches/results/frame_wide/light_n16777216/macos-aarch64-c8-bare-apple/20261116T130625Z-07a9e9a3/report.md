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

### rust_std_par is an outlier: 5.6x slower than the field

rust_std_par (44.04 ms) is 5.6x the fastest (7.81 ms), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (hand)

The baseline hand is the fastest (7.81 ms median); no rival improves on it (all deltas are >= 0).


### Two tiers: {hand, hand_par} vs {rust_std, rust_nostd, engine_par, engine, rust_std_par} (264% apart)

The field splits into a fast tier {hand, hand_par} and a slow tier {rust_std, rust_nostd, engine_par, engine, rust_std_par} with a 264% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 5.6x the fastest

Fastest hand (7.81 ms) to slowest rust_std_par (44.04 ms): 5.6x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (hand) is the fastest at 7814491.7 ns median
- 6 variants significantly slower than baseline
- Spread: 5.64x (fastest 7814491.7 ns, slowest 44037766.6 ns)

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
| engine | 33869025ns | 33589854ns | 33012975ns | 33301786ns | 34508467ns | +333.34% |
| engine_par | 33188909ns | 31834925ns | 31687375ns | 31778247ns | 34622290ns | +324.64% |
| hand | 7815719ns | 7821917ns | 7753533ns | 7800039ns | 7843025ns | base |
| hand_par | 8153262ns | 8146604ns | 8099558ns | 8128939ns | 8184931ns | +4.32% |
| rust_nostd | 30131029ns | 29738433ns | 29342492ns | 29499964ns | 30801462ns | +285.52% |
| rust_std | 29990418ns | 29618754ns | 29454575ns | 29556950ns | 30449479ns | +283.72% |
| rust_std_par | 44070849ns | 44042100ns | 43822508ns | 44010294ns | 44178350ns | +463.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 33864792ns | 33006242ns | 34505573ns | +333.65% | 5.945 |
| engine_par | 33182447ns | 31681700ns | 34614271ns | +324.91% | 6.067 |
| hand | 7809227ns | 7747967ns | 7836281ns | base | 25.781 |
| hand_par | 8146426ns | 8091692ns | 8178290ns | +4.32% | 24.713 |
| rust_nostd | 30126145ns | 29336317ns | 30798123ns | +285.78% | 6.683 |
| rust_std | 29985724ns | 29449267ns | 30445010ns | +283.98% | 6.714 |
| rust_std_par | 44065900ns | 43816967ns | 44173706ns | +464.28% | 4.569 |

## Performance model

- Peak throughput: 25.984 Gops/s (hand; best 20% batches)
- Ops per call: 201326592

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 5.994 | 23.1% |
| engine_par | 6.326 | 24.3% |
| hand | 25.763 | 99.1% |
| hand_par | 24.733 | 95.2% |
| rust_nostd | 6.771 | 26.1% |
| rust_std | 6.798 | 26.2% |
| rust_std_par | 4.572 | 17.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 33869025ns | 33869025ns | +333.34% |
| engine_par | 33188909ns | 33188909ns | +324.64% |
| hand | 7815719ns | 7815719ns | base |
| hand_par | 8153262ns | 8153262ns | +4.32% |
| rust_nostd | 30131029ns | 30131029ns | +285.52% |
| rust_std | 29990418ns | 29990418ns | +283.72% |
| rust_std_par | 44070849ns | 44070849ns | +463.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 7814492ns | base | --- | [7774942, 7831692] | --- | --- | --- | --- |
| engine | 33587671ns | +25795595.9ns (+330.1%) | [+25230850, +27093808]ns | [33094783, 34923533] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 31824896ns | +24032820.8ns (+307.5%) | [+23887783, +24971092]ns | [31751517, 32790867] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 8139942ns | +321550.0ns (+4.1%) | [+295117, +362892]ns | [8108050, 8192617] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 29734750ns | +21896991.6ns (+280.2%) | [+21649383, +23754000]ns | [29457667, 31528942] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 29614875ns | +21812541.6ns (+279.1%) | [+21701300, +22748258]ns | [29486100, 30612192] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 44037767ns | +36240408.3ns (+463.8%) | [+36125817, +36408950]ns | [43989750, 44196225] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 7824750ns | +323.9% | +307.9% | +4.5% | +276.5% | +277.7% | +462.5% |
| 1 | 2 | 7802892ns | +336.5% | +307.9% | +4.0% | +294.3% | +281.3% | +465.3% |

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
| engine | 270574841.7ns | 33864791.7ns | 799.0% | HIGH |
| engine_par | 271667891.6ns | 33182446.9ns | 818.7% | HIGH |
| hand | 270445538.6ns | 7809227.1ns | 3463.2% | HIGH |
| hand_par | 270610313.5ns | 8146426.1ns | 3321.8% | HIGH |
| rust_nostd | 270307589.6ns | 30126144.8ns | 897.3% | HIGH |
| rust_std | 270471209.4ns | 29985724.0ns | 902.0% | HIGH |
| rust_std_par | 270618362.5ns | 44065900.0ns | 614.1% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 33006241.7-34505572.9 ns)
  33006241.7 |########################################
  33081208.3 |########################################
  33156174.8 |
  33231141.4 |########################################
  33306107.9 |
  33381074.5 |
  33456041.1 |
  33531007.6 |########################################
  33605974.2 |########################################
  33680940.8 |
  33755907.3 |
  33830873.9 |
  33905840.4 |
  33980807.0 |
  34055773.6 |
  34130740.1 |
  34205706.7 |
  34280673.2 |
  34355639.8 |
  34430606.4 |########################################
  (0 below, 2 above range)

engine_par (n=8, range 31681700.0-34614270.8 ns)
  31681700.0 |########################################
  31828328.5 |##########
  31974957.1 |##########
  32121585.6 |
  32268214.2 |
  32414842.7 |
  32561471.2 |
  32708099.8 |##########
  32854728.3 |
  33001356.9 |
  33147985.4 |
  33294614.0 |
  33441242.5 |
  33587871.0 |
  33734499.6 |
  33881128.1 |
  34027756.7 |
  34174385.2 |
  34321013.7 |
  34467642.3 |
  (0 below, 1 above range)

hand (n=8, range 7747966.7-7836281.2 ns)
  7747966.7 |####################
  7752382.4 |
  7756798.2 |
  7761213.9 |
  7765629.6 |
  7770045.3 |
  7774461.1 |####################
  7778876.8 |
  7783292.5 |
  7787708.2 |
  7792124.0 |
  7796539.7 |####################
  7800955.4 |
  7805371.2 |####################
  7809786.9 |
  7814202.6 |
  7818618.3 |####################
  7823034.1 |
  7827449.8 |########################################
  7831865.5 |
  (0 below, 1 above range)

hand_par (n=8, range 8091691.7-8178289.6 ns)
  8091691.7 |########################################
  8096021.6 |
  8100351.5 |
  8104681.4 |########################################
  8109011.3 |
  8113341.2 |
  8117671.1 |########################################
  8122001.0 |
  8126330.9 |
  8130660.8 |
  8134990.7 |########################################
  8139320.5 |########################################
  8143650.4 |
  8147980.3 |
  8152310.2 |
  8156640.1 |
  8160970.0 |########################################
  8165299.9 |
  8169629.8 |
  8173959.7 |
  (0 below, 2 above range)

rust_nostd (n=8, range 29336316.7-30798122.9 ns)
  29336316.7 |####################
  29409407.0 |########################################
  29482497.3 |####################
  29555587.6 |
  29628677.9 |
  29701768.3 |
  29774858.6 |
  29847948.9 |####################
  29921039.2 |
  29994129.5 |####################
  30067219.8 |
  30140310.1 |
  30213400.4 |
  30286490.7 |
  30359581.1 |
  30432671.4 |
  30505761.7 |
  30578852.0 |
  30651942.3 |
  30725032.6 |
  (0 below, 2 above range)

rust_std (n=8, range 29449266.7-30445010.4 ns)
  29449266.7 |########################################
  29499053.9 |
  29548841.1 |####################
  29598628.3 |########################################
  29648415.4 |
  29698202.6 |
  29747989.8 |
  29797777.0 |
  29847564.2 |
  29897351.4 |####################
  29947138.6 |
  29996925.7 |
  30046712.9 |
  30096500.1 |
  30146287.3 |
  30196074.5 |
  30245861.7 |
  30295648.9 |
  30345436.1 |
  30395223.2 |
  (0 below, 2 above range)

rust_std_par (n=8, range 43816966.7-44173706.2 ns)
  43816966.7 |####################
  43834803.7 |
  43852640.7 |
  43870477.6 |
  43888314.6 |
  43906151.6 |
  43923988.6 |
  43941825.5 |
  43959662.5 |
  43977499.5 |########################################
  43995336.5 |
  44013173.5 |
  44031010.4 |########################################
  44048847.4 |
  44066684.4 |
  44084521.4 |
  44102358.3 |
  44120195.3 |
  44138032.3 |
  44155869.3 |
  (0 below, 3 above range)

```

## Diagnostics

- engine: bridge=805.0% of algo (FFI overhead may distort results)
- engine_par: bridge=849.9% of algo (FFI overhead may distort results)
- hand: bridge=3461.1% of algo (FFI overhead may distort results)
- hand_par: bridge=3321.4% of algo (FFI overhead may distort results)
- rust_nostd: bridge=908.9% of algo (FFI overhead may distort results)
- rust_std: bridge=912.8% of algo (FFI overhead may distort results)
- rust_std_par: bridge=614.1% of algo (FFI overhead may distort results)
