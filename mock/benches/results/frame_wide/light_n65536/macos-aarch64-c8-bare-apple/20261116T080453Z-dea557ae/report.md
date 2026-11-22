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

### hand dominates: 208% faster than the next best (hand_par)

hand (15.24 us) leads hand_par (46.88 us) by 208%, a clear separation rather than a photo finish. CV 2.6%.


### engine_par is an outlier: 7.0x slower than the field

engine_par (106.45 us) is 7.0x the fastest (15.24 us), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (hand)

The baseline hand is the fastest (15.24 us median); no rival improves on it (all deltas are >= 0).


### Two tiers: {hand} vs {hand_par, rust_nostd, rust_std, rust_std_par, engine, engine_par} (208% apart)

The field splits into a fast tier {hand} and a slow tier {hand_par, rust_nostd, rust_std, rust_std_par, engine, engine_par} with a 208% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 7.0x the fastest

Fastest hand (15.24 us) to slowest engine_par (106.45 us): 7.0x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (hand) is the fastest at 15237.5 ns median
- 6 variants significantly slower than baseline
- Spread: 6.99x (fastest 15237.5 ns, slowest 106454.1 ns)

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
| engine | 81102ns | 80912ns | 80350ns | 80822ns | 81500ns | +393.65% |
| engine_par | 109175ns | 107617ns | 104417ns | 106922ns | 112054ns | +564.52% |
| hand | 16429ns | 16396ns | 15400ns | 16211ns | 16850ns | base |
| hand_par | 47983ns | 47908ns | 40583ns | 45917ns | 51383ns | +192.06% |
| rust_nostd | 73001ns | 73067ns | 72525ns | 72861ns | 73225ns | +344.34% |
| rust_std | 73627ns | 73688ns | 72683ns | 73342ns | 74077ns | +348.15% |
| rust_std_par | 80404ns | 80442ns | 73425ns | 77172ns | 84573ns | +389.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 80042ns | 79275ns | 80523ns | +419.43% | 9.825 |
| engine_par | 107818ns | 103017ns | 110519ns | +599.69% | 7.294 |
| hand | 15409ns | 15142ns | 15623ns | base | 51.036 |
| hand_par | 46743ns | 40183ns | 50292ns | +203.34% | 16.825 |
| rust_nostd | 71660ns | 71325ns | 71898ns | +365.04% | 10.974 |
| rust_std | 72692ns | 72517ns | 72800ns | +371.74% | 10.819 |
| rust_std_par | 79423ns | 73025ns | 83288ns | +415.42% | 9.902 |

## Performance model

- Peak throughput: 51.938 Gops/s (hand; best 20% batches)
- Ops per call: 786432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 9.867 | 19.0% |
| engine_par | 7.388 | 14.2% |
| hand | 51.612 | 99.4% |
| hand_par | 16.774 | 32.3% |
| rust_nostd | 10.973 | 21.1% |
| rust_std | 10.827 | 20.8% |
| rust_std_par | 10.014 | 19.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 81102ns | 81102ns | +393.65% |
| engine_par | 109175ns | 109175ns | +564.52% |
| hand | 16429ns | 16429ns | base |
| hand_par | 47983ns | 47983ns | +192.06% |
| rust_nostd | 73001ns | 73001ns | +344.34% |
| rust_std | 73627ns | 73627ns | +348.15% |
| rust_std_par | 80404ns | 80404ns | +389.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 15238ns | base | --- | [15200, 15533] | --- | --- | --- | --- |
| engine | 79700ns | +64475.0ns (+423.1%) | [+64025, +65142]ns | [79600, 80367] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 106454ns | +91220.8ns (+598.7%) | [+88367, +96233]ns | [104792, 111483] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 46883ns | +31629.2ns (+207.6%) | [+26517, +35842]ns | [41733, 50983] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 71667ns | +56383.3ns (+370.0%) | [+56183, +56583]ns | [71400, 71975] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 72638ns | +57445.8ns (+377.0%) | [+56983, +57625]ns | [72592, 72908] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 78533ns | +63308.3ns (+415.5%) | [+59383, +68942]ns | [75117, 84158] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 15221ns | +423.5% | +631.3% | +217.5% | +370.8% | +377.4% | +435.3% |
| 1 | 2 | 15408ns | +419.0% | +590.1% | +194.8% | +365.3% | +371.2% | +389.8% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| hand | -0.500 | moderate- |
| hand_par | -0.500 | moderate- |
| rust_nostd | -0.500 | moderate- |
| rust_std | -0.500 | moderate- |
| rust_std_par | -0.500 | HIGH- (thermal bounce) |

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
| engine | 1070274.0ns | 80041.7ns | 1337.1% | HIGH |
| engine_par | 1074718.8ns | 107817.7ns | 996.8% | HIGH |
| hand | 1071892.7ns | 15409.4ns | 6956.1% | HIGH |
| hand_par | 1085072.9ns | 46742.7ns | 2321.4% | HIGH |
| rust_nostd | 1070524.0ns | 71660.4ns | 1493.9% | HIGH |
| rust_std | 1070511.5ns | 72691.7ns | 1472.7% | HIGH |
| rust_std_par | 1075058.3ns | 79422.9ns | 1353.6% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 79275.0-80522.9 ns)
  79275.0 |#############
  79337.4 |
  79399.8 |
  79462.2 |
  79524.6 |
  79587.0 |#############
  79649.4 |########################################
  79711.8 |
  79774.2 |
  79836.6 |
  79899.0 |
  79961.4 |
  80023.8 |
  80086.2 |
  80148.5 |
  80210.9 |#############
  80273.3 |
  80335.7 |#############
  80398.1 |
  80460.5 |
  (0 below, 1 above range)

engine_par (n=8, range 103016.7-110518.8 ns)
  103016.7 |####################
  103391.8 |
  103766.9 |
  104142.0 |
  104517.1 |####################
  104892.2 |
  105267.3 |
  105642.4 |
  106017.5 |########################################
  106392.6 |####################
  106767.7 |
  107142.8 |
  107517.9 |
  107893.0 |
  108268.1 |
  108643.2 |
  109018.3 |
  109393.4 |
  109768.5 |
  110143.6 |
  (0 below, 3 above range)

hand (n=8, range 15141.7-15622.9 ns)
  15141.7 |####################
  15165.8 |
  15189.8 |####################
  15213.9 |########################################
  15237.9 |####################
  15262.0 |####################
  15286.1 |
  15310.1 |
  15334.2 |
  15358.2 |
  15382.3 |
  15406.4 |
  15430.4 |
  15454.5 |
  15478.5 |
  15502.6 |
  15526.7 |####################
  15550.7 |
  15574.8 |
  15598.8 |
  (0 below, 1 above range)

hand_par (n=8, range 40183.3-50291.6 ns)
  40183.3 |########################################
  40688.7 |
  41194.1 |
  41699.6 |########################################
  42205.0 |
  42710.4 |
  43215.8 |
  43721.2 |
  44226.6 |########################################
  44732.1 |
  45237.5 |
  45742.9 |
  46248.3 |########################################
  46753.7 |
  47259.1 |########################################
  47764.6 |
  48270.0 |
  48775.4 |########################################
  49280.8 |
  49786.2 |
  (0 below, 2 above range)

rust_nostd (n=8, range 71325.0-71897.9 ns)
  71325.0 |########################################
  71353.6 |
  71382.3 |########################################
  71410.9 |########################################
  71439.6 |
  71468.2 |
  71496.9 |
  71525.5 |########################################
  71554.2 |
  71582.8 |
  71611.4 |
  71640.1 |
  71668.7 |
  71697.4 |
  71726.0 |
  71754.7 |########################################
  71783.3 |
  71812.0 |########################################
  71840.6 |
  71869.3 |
  (0 below, 2 above range)

rust_std (n=8, range 72516.7-72800.0 ns)
  72516.7 |########################################
  72530.9 |
  72545.0 |
  72559.2 |
  72573.4 |
  72587.5 |########################################
  72601.7 |########################################
  72615.8 |########################################
  72630.0 |
  72644.2 |########################################
  72658.3 |
  72672.5 |########################################
  72686.7 |
  72700.8 |
  72715.0 |
  72729.2 |
  72743.3 |
  72757.5 |
  72771.6 |
  72785.8 |
  (0 below, 2 above range)

rust_std_par (n=8, range 73025.0-83287.5 ns)
  73025.0 |########################################
  73538.1 |
  74051.2 |
  74564.4 |
  75077.5 |########################################
  75590.6 |########################################
  76103.8 |
  76616.9 |
  77130.0 |
  77643.1 |
  78156.2 |########################################
  78669.4 |########################################
  79182.5 |
  79695.6 |
  80208.8 |
  80721.9 |########################################
  81235.0 |
  81748.1 |
  82261.2 |
  82774.4 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=1342.0% of algo (FFI overhead may distort results)
- engine_par: bridge=1007.9% of algo (FFI overhead may distort results)
- hand: bridge=7035.6% of algo (FFI overhead may distort results)
- hand_par: bridge=2315.8% of algo (FFI overhead may distort results)
- rust_nostd: bridge=1493.4% of algo (FFI overhead may distort results)
- rust_std: bridge=1472.3% of algo (FFI overhead may distort results)
- rust_std_par: bridge=1370.0% of algo (FFI overhead may distort results)
