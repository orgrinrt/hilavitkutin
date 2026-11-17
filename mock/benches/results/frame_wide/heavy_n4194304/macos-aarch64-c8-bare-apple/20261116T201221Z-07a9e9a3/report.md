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

The declared/defaulted baseline hand has the worst median (53.67 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest rust_std_par at 11.09 ms).


### rust_std_par dominates: 11% faster than the next best (hand_par)

rust_std_par (11.09 ms) leads hand_par (12.33 ms) by 11%, a clear separation rather than a photo finish. CV 0.6%.


### rust_std_par beats baseline by 79% (significant)

rust_std_par is -42.59 ms (79%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 4.8x slower than the field

hand (53.67 ms) is 4.8x the fastest (11.09 ms), well off the pack.


### rust_std shows alternating (throttle bounce) (autocorr -0.50)

rust_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {rust_std_par, hand_par} vs {engine_par, rust_nostd, rust_std, engine, hand} (156% apart)

The field splits into a fast tier {rust_std_par, hand_par} and a slow tier {engine_par, rust_nostd, rust_std, engine, hand} with a 156% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 4.8x the fastest

Fastest rust_std_par (11.09 ms) to slowest hand (53.67 ms): 4.8x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: rust_std_par at 11086987.5 ns median (-79.3% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 4.84x (fastest 11086987.5 ns, slowest 53668458.4 ns)

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
| engine | 37494682ns | 37418179ns | 37360942ns | 37382661ns | 37612133ns | -30.15% |
| engine_par | 31578734ns | 31559379ns | 31478708ns | 31528214ns | 31641631ns | -41.17% |
| hand | 53681845ns | 53672425ns | 53401392ns | 53504025ns | 53885323ns | base |
| hand_par | 12747275ns | 12338388ns | 12177092ns | 12288258ns | 13234083ns | -76.25% |
| rust_nostd | 37423219ns | 37355358ns | 37075908ns | 37209481ns | 37670350ns | -30.29% |
| rust_std | 37530713ns | 37382608ns | 37295250ns | 37377808ns | 37704256ns | -30.09% |
| rust_std_par | 11115231ns | 11091050ns | 11034792ns | 11060142ns | 11176658ns | -79.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 37491671ns | 37356933ns | 37609102ns | -30.15% | 1.342 |
| engine_par | 31574640ns | 31473883ns | 31637960ns | -41.18% | 1.594 |
| hand | 53677183ns | 53397083ns | 53880765ns | base | 0.938 |
| hand_par | 12741870ns | 12171383ns | 13229535ns | -76.26% | 3.950 |
| rust_nostd | 37420492ns | 37073892ns | 37667440ns | -30.29% | 1.345 |
| rust_std | 37527741ns | 37292783ns | 37701252ns | -30.09% | 1.341 |
| rust_std_par | 11110992ns | 11030900ns | 11172510ns | -79.30% | 4.530 |

## Performance model

- Peak throughput: 4.563 Gops/s (rust_std_par; best 20% batches)
- Ops per call: 50331648

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 1.345 | 29.5% |
| engine_par | 1.595 | 35.0% |
| hand | 0.938 | 20.6% |
| hand_par | 4.081 | 89.4% |
| rust_nostd | 1.347 | 29.5% |
| rust_std | 1.347 | 29.5% |
| rust_std_par | 4.540 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 37494682ns | 37494682ns | -30.15% |
| engine_par | 31578734ns | 31578734ns | -41.17% |
| hand | 53681845ns | 53681845ns | base |
| hand_par | 12747275ns | 12747275ns | -76.25% |
| rust_nostd | 37423219ns | 37423219ns | -30.29% |
| rust_std | 37530713ns | 37530713ns | -30.09% |
| rust_std_par | 11115231ns | 11115231ns | -79.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 53668458ns | base | --- | [53461067, 53856333] | --- | --- | --- | --- |
| engine | 37415729ns | -16094970.9ns (-30.0%) | [-16474817, -16038758]ns | [37358325, 37568675] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 31556029ns | -22112766.7ns (-41.2%) | [-22336242, -21857900]ns | [31512117, 31594142] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 12332358ns | -41205645.9ns (-76.8%) | [-41541117, -40121675]ns | [12254742, 13364317] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 37353608ns | -16232158.4ns (-30.2%) | [-16538450, -15963250]ns | [37137542, 37739650] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 37379046ns | -16132579.2ns (-30.1%) | [-16493867, -15991083]ns | [37369083, 37769250] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 11086988ns | -42587420.9ns (-79.4%) | [-42808125, -42356592]ns | [11037842, 11187817] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 53821492ns | -30.5% | -41.4% | -77.1% | -30.7% | -30.2% | -79.2% |
| 1 | 2 | 53518129ns | -30.0% | -41.0% | -76.6% | -30.2% | -30.2% | -79.3% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| hand | -0.500 | moderate- |
| hand_par | -0.500 | moderate- |
| rust_nostd | -0.500 | moderate- |
| rust_std | -0.500 | HIGH- (thermal bounce) |
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
| engine | 67500568.8ns | 37491670.8ns | 180.0% | HIGH |
| engine_par | 67556922.9ns | 31574639.6ns | 214.0% | HIGH |
| hand | 67927567.7ns | 53677183.3ns | 126.5% | HIGH |
| hand_par | 68004188.5ns | 12741869.8ns | 533.7% | HIGH |
| rust_nostd | 67719999.0ns | 37420491.7ns | 181.0% | HIGH |
| rust_std | 67651692.7ns | 37527740.6ns | 180.3% | HIGH |
| rust_std_par | 67475214.6ns | 11110991.7ns | 607.3% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 37356933.3-37609102.1 ns)
  37356933.3 |########################################
  37369541.7 |####################
  37382150.2 |
  37394758.6 |####################
  37407367.1 |
  37419975.5 |####################
  37432583.9 |
  37445192.4 |
  37457800.8 |
  37470409.2 |
  37483017.7 |
  37495626.1 |####################
  37508234.6 |
  37520843.0 |
  37533451.4 |
  37546059.9 |
  37558668.3 |####################
  37571276.8 |
  37583885.2 |
  37596493.6 |
  (0 below, 1 above range)

engine_par (n=8, range 31473883.3-31637960.4 ns)
  31473883.3 |####################
  31482087.2 |
  31490291.0 |
  31498494.9 |
  31506698.7 |####################
  31514902.6 |####################
  31523106.4 |
  31531310.3 |####################
  31539514.2 |
  31547718.0 |
  31555921.9 |
  31564125.7 |
  31572329.6 |########################################
  31580533.4 |
  31588737.3 |####################
  31596941.1 |
  31605145.0 |
  31613348.9 |
  31621552.7 |
  31629756.6 |
  (0 below, 1 above range)

hand (n=8, range 53397083.3-53880764.6 ns)
  53397083.3 |####################
  53421267.4 |
  53445451.4 |####################
  53469635.5 |####################
  53493819.6 |
  53518003.6 |
  53542187.7 |####################
  53566371.7 |
  53590555.8 |
  53614739.9 |
  53638923.9 |
  53663108.0 |
  53687292.1 |
  53711476.1 |
  53735660.2 |
  53759844.3 |
  53784028.3 |####################
  53808212.4 |
  53832396.4 |########################################
  53856580.5 |
  (0 below, 1 above range)

hand_par (n=8, range 12171383.3-13229535.4 ns)
  12171383.3 |####################
  12224290.9 |########################################
  12277198.5 |####################
  12330106.1 |####################
  12383013.7 |
  12435921.3 |
  12488828.9 |
  12541736.5 |
  12594644.2 |
  12647551.8 |
  12700459.4 |
  12753367.0 |####################
  12806274.6 |
  12859182.2 |
  12912089.8 |
  12964997.4 |
  13017905.0 |
  13070812.6 |
  13123720.2 |
  13176627.8 |
  (0 below, 2 above range)

rust_nostd (n=8, range 37073891.7-37667439.6 ns)
  37073891.7 |########################################
  37103569.1 |
  37133246.5 |########################################
  37162923.9 |
  37192601.3 |########################################
  37222278.7 |
  37251956.1 |
  37281633.5 |########################################
  37311310.9 |
  37340988.2 |
  37370665.6 |
  37400343.0 |########################################
  37430020.4 |########################################
  37459697.8 |
  37489375.2 |
  37519052.6 |
  37548730.0 |
  37578407.4 |
  37608084.8 |
  37637762.2 |
  (0 below, 2 above range)

rust_std (n=8, range 37292783.3-37701252.1 ns)
  37292783.3 |#############
  37313206.7 |
  37333630.2 |
  37354053.6 |#############
  37374477.1 |########################################
  37394900.5 |
  37415323.9 |
  37435747.4 |
  37456170.8 |
  37476594.2 |#############
  37497017.7 |
  37517441.1 |
  37537864.6 |
  37558288.0 |
  37578711.4 |
  37599134.9 |
  37619558.3 |
  37639981.8 |
  37660405.2 |
  37680828.6 |
  (0 below, 2 above range)

rust_std_par (n=8, range 11030900.0-11172510.4 ns)
  11030900.0 |########################################
  11037980.5 |
  11045061.0 |
  11052141.6 |
  11059222.1 |####################
  11066302.6 |####################
  11073383.1 |
  11080463.6 |
  11087544.2 |
  11094624.7 |
  11101705.2 |####################
  11108785.7 |
  11115866.3 |
  11122946.8 |
  11130027.3 |
  11137107.8 |
  11144188.3 |
  11151268.9 |
  11158349.4 |####################
  11165429.9 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=180.2% of algo (FFI overhead may distort results)
- engine_par: bridge=214.2% of algo (FFI overhead may distort results)
- hand: bridge=126.4% of algo (FFI overhead may distort results)
- hand_par: bridge=550.7% of algo (FFI overhead may distort results)
- rust_nostd: bridge=180.5% of algo (FFI overhead may distort results)
- rust_std: bridge=180.6% of algo (FFI overhead may distort results)
- rust_std_par: bridge=607.6% of algo (FFI overhead may distort results)
