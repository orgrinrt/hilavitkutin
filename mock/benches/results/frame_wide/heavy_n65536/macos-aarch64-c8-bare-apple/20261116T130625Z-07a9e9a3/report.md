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

The declared/defaulted baseline hand has the worst median (833.85 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest rust_std_par at 211.72 us).


### rust_std_par dominates: 16% faster than the next best (hand_par)

rust_std_par (211.72 us) leads hand_par (246.15 us) by 16%, a clear separation rather than a photo finish. CV 1.6%.


### rust_std_par beats baseline by 75% (significant)

rust_std_par is -622.23 us (75%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 3.9x slower than the field

hand (833.85 us) is 3.9x the fastest (211.72 us), well off the pack.


### hand shows alternating (throttle bounce) (autocorr -0.50)

hand's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {rust_std_par, hand_par} vs {engine_par, rust_nostd, rust_std, engine, hand} (119% apart)

The field splits into a fast tier {rust_std_par, hand_par} and a slow tier {engine_par, rust_nostd, rust_std, engine, hand} with a 119% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 3.9x the fastest

Fastest rust_std_par (211.72 us) to slowest hand (833.85 us): 3.9x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: rust_std_par at 211725.0 ns median (-74.6% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 3.94x (fastest 211725.0 ns, slowest 833854.2 ns)

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
| engine | 592194ns | 591971ns | 590950ns | 591353ns | 593135ns | -29.08% |
| engine_par | 539633ns | 539517ns | 537567ns | 538892ns | 540706ns | -35.37% |
| hand | 835014ns | 834967ns | 833367ns | 834522ns | 835794ns | base |
| hand_par | 248135ns | 247562ns | 244800ns | 246994ns | 249825ns | -70.28% |
| rust_nostd | 578216ns | 577996ns | 577442ns | 577869ns | 578669ns | -30.75% |
| rust_std | 589421ns | 585650ns | 582433ns | 583800ns | 595383ns | -29.41% |
| rust_std_par | 213032ns | 213633ns | 207400ns | 210672ns | 216210ns | -74.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 590871ns | 589775ns | 591677ns | -29.14% | 1.331 |
| engine_par | 538389ns | 536875ns | 539208ns | -35.43% | 1.461 |
| hand | 833821ns | 832183ns | 834565ns | base | 0.943 |
| hand_par | 246481ns | 243758ns | 248115ns | -70.44% | 3.191 |
| rust_nostd | 576701ns | 576233ns | 577075ns | -30.84% | 1.364 |
| rust_std | 588611ns | 580750ns | 595213ns | -29.41% | 1.336 |
| rust_std_par | 211196ns | 206517ns | 214000ns | -74.67% | 3.724 |

## Performance model

- Peak throughput: 3.808 Gops/s (rust_std_par; best 20% batches)
- Ops per call: 786432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 1.331 | 35.0% |
| engine_par | 1.461 | 38.4% |
| hand | 0.943 | 24.8% |
| hand_par | 3.195 | 83.9% |
| rust_nostd | 1.364 | 35.8% |
| rust_std | 1.344 | 35.3% |
| rust_std_par | 3.714 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 592194ns | 592194ns | -29.08% |
| engine_par | 539633ns | 539633ns | -35.37% |
| hand | 835014ns | 835014ns | base |
| hand_par | 248135ns | 248135ns | -70.28% |
| rust_nostd | 578216ns | 578216ns | -30.75% |
| rust_std | 589421ns | 589421ns | -29.41% |
| rust_std_par | 213032ns | 213032ns | -74.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 833854ns | base | --- | [833058, 834583] | --- | --- | --- | --- |
| engine | 590792ns | -242775.0ns (-29.1%) | [-244117, -242225]ns | [589850, 591417] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 538417ns | -295683.3ns (-35.5%) | [-296450, -293250]ns | [537458, 539267] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 246150ns | -586812.5ns (-70.4%) | [-589167, -586133]ns | [244158, 247417] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 576529ns | -256995.8ns (-30.8%) | [-258233, -255908]ns | [576275, 577333] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 584996ns | -247625.0ns (-29.7%) | [-252992, -237092]ns | [581792, 597142] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 211725ns | -622229.2ns (-74.6%) | [-627575, -618808]ns | [207900, 213375] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 834162ns | -29.1% | -35.4% | -70.5% | -30.9% | -30.2% | -74.5% |
| 1 | 2 | 833646ns | -29.2% | -35.5% | -70.4% | -30.8% | -28.7% | -74.9% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| hand | -0.500 | HIGH- (thermal bounce) |
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
| engine | 1049377.1ns | 590870.8ns | 177.6% | HIGH |
| engine_par | 1054810.4ns | 538388.5ns | 195.9% | HIGH |
| hand | 1049477.1ns | 833820.8ns | 125.9% | HIGH |
| hand_par | 1069984.4ns | 246481.2ns | 434.1% | HIGH |
| rust_nostd | 1049952.1ns | 576701.0ns | 182.1% | HIGH |
| rust_std | 1062651.0ns | 588611.5ns | 180.5% | HIGH |
| rust_std_par | 1053696.9ns | 211195.8ns | 498.9% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 589775.0-591677.1 ns)
  589775.0 |########################################
  589870.1 |####################
  589965.2 |
  590060.3 |
  590155.4 |
  590250.5 |
  590345.6 |
  590440.7 |
  590535.8 |
  590630.9 |####################
  590726.1 |
  590821.2 |####################
  590916.3 |
  591011.4 |
  591106.5 |####################
  591201.6 |
  591296.7 |
  591391.8 |####################
  591486.9 |
  591582.0 |
  (0 below, 1 above range)

engine_par (n=8, range 536875.0-539208.3 ns)
  536875.0 |########################################
  536991.7 |
  537108.3 |
  537225.0 |
  537341.7 |########################################
  537458.3 |########################################
  537575.0 |
  537691.7 |
  537808.3 |
  537925.0 |
  538041.7 |
  538158.3 |
  538275.0 |########################################
  538391.7 |########################################
  538508.3 |########################################
  538625.0 |
  538741.7 |
  538858.3 |
  538975.0 |
  539091.7 |
  (0 below, 2 above range)

hand (n=8, range 832183.3-834564.6 ns)
  832183.3 |########################################
  832302.4 |
  832421.4 |
  832540.5 |
  832659.6 |
  832778.6 |
  832897.7 |
  833016.7 |########################################
  833135.8 |
  833254.9 |########################################
  833373.9 |
  833493.0 |
  833612.1 |
  833731.1 |########################################
  833850.2 |########################################
  833969.3 |
  834088.3 |
  834207.4 |########################################
  834326.4 |
  834445.5 |
  (0 below, 2 above range)

hand_par (n=8, range 243758.3-248114.6 ns)
  243758.3 |####################
  243976.1 |####################
  244193.9 |
  244411.7 |
  244629.6 |
  244847.4 |
  245065.2 |
  245283.0 |####################
  245500.8 |
  245718.6 |
  245936.4 |####################
  246154.3 |####################
  246372.1 |
  246589.9 |
  246807.7 |
  247025.5 |
  247243.3 |########################################
  247461.1 |
  247678.9 |
  247896.8 |
  (0 below, 1 above range)

rust_nostd (n=8, range 576233.3-577075.0 ns)
  576233.3 |########################################
  576275.4 |####################
  576317.5 |
  576359.6 |
  576401.6 |
  576443.7 |
  576485.8 |####################
  576527.9 |####################
  576570.0 |
  576612.1 |
  576654.2 |
  576696.2 |
  576738.3 |
  576780.4 |
  576822.5 |
  576864.6 |####################
  576906.7 |
  576948.7 |
  576990.8 |
  577032.9 |
  (0 below, 2 above range)

rust_std (n=8, range 580750.0-595212.5 ns)
  580750.0 |########################################
  581473.1 |########################################
  582196.3 |########################################
  582919.4 |########################################
  583642.5 |
  584365.6 |
  585088.8 |
  585811.9 |
  586535.0 |########################################
  587258.1 |
  587981.3 |
  588704.4 |
  589427.5 |
  590150.6 |
  590873.8 |
  591596.9 |########################################
  592320.0 |
  593043.1 |
  593766.3 |
  594489.4 |
  (0 below, 2 above range)

rust_std_par (n=8, range 206516.7-214000.0 ns)
  206516.7 |####################
  206890.9 |
  207265.0 |
  207639.2 |########################################
  208013.4 |
  208387.5 |
  208761.7 |
  209135.8 |
  209510.0 |
  209884.2 |
  210258.3 |
  210632.5 |
  211006.7 |####################
  211380.8 |
  211755.0 |
  212129.2 |####################
  212503.3 |
  212877.5 |
  213251.6 |########################################
  213625.8 |
  (0 below, 1 above range)

```

## Diagnostics

- engine: bridge=177.5% of algo (FFI overhead may distort results)
- engine_par: bridge=195.9% of algo (FFI overhead may distort results)
- hand: bridge=125.8% of algo (FFI overhead may distort results)
- hand_par: bridge=435.0% of algo (FFI overhead may distort results)
- rust_nostd: bridge=182.1% of algo (FFI overhead may distort results)
- rust_std: bridge=180.1% of algo (FFI overhead may distort results)
- rust_std_par: bridge=497.7% of algo (FFI overhead may distort results)
