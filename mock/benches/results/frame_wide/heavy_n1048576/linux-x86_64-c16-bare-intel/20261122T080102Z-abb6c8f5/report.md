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

The declared/defaulted baseline hand has the worst median (38.79 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hand_par at 7.66 ms).


### hand_par beats baseline by 81% (significant)

hand_par is -31.24 ms (81%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 5.1x slower than the field

hand (38.79 ms) is 5.1x the fastest (7.66 ms), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {hand_par, rust_std_par} vs {engine_par, rust_nostd, engine, rust_std, hand} (299% apart)

The field splits into a fast tier {hand_par, rust_std_par} and a slow tier {engine_par, rust_nostd, engine, rust_std, hand} with a 299% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 5.1x the fastest

Fastest hand_par (7.66 ms) to slowest hand (38.79 ms): 5.1x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: hand_par at 7657606.5 ns median (-80.3% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 5.07x (fastest 7657606.5 ns, slowest 38790735.5 ns)

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
| Counter frequency | 2112801267 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 31798709ns | 31788061ns | 31764509ns | 31775974ns | 31824310ns | -18.17% |
| engine_par | 31015257ns | 31358504ns | 27542408ns | 29703069ns | 32867610ns | -20.18% |
| hand | 38858632ns | 38792652ns | 38784025ns | 38790134ns | 38928657ns | base |
| hand_par | 7644067ns | 7661009ns | 7208814ns | 7419849ns | 7921044ns | -80.33% |
| rust_nostd | 31767108ns | 31753412ns | 31733195ns | 31746859ns | 31790774ns | -18.25% |
| rust_std | 36099842ns | 35952751ns | 34322531ns | 34868364ns | 37467778ns | -7.10% |
| rust_std_par | 7878436ns | 7866277ns | 7528940ns | 7687536ns | 8108985ns | -79.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 31797643ns | 31763442ns | 31823263ns | -18.17% | 0.396 |
| engine_par | 31012557ns | 27539341ns | 32864900ns | -20.19% | 0.406 |
| hand | 38857053ns | 38782511ns | 38927109ns | base | 0.324 |
| hand_par | 7640878ns | 7205223ns | 7917828ns | -80.34% | 1.647 |
| rust_nostd | 31765525ns | 31731916ns | 31789117ns | -18.25% | 0.396 |
| rust_std | 36098482ns | 34320980ns | 37466666ns | -7.10% | 0.349 |
| rust_std_par | 7875716ns | 7526397ns | 8106230ns | -79.73% | 1.598 |

## Performance model

- Peak throughput: 1.746 Gops/s (hand_par; best 20% batches)
- Ops per call: 12582912

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.396 | 22.7% |
| engine_par | 0.401 | 23.0% |
| hand | 0.324 | 18.6% |
| hand_par | 1.643 | 94.1% |
| rust_nostd | 0.396 | 22.7% |
| rust_std | 0.350 | 20.0% |
| rust_std_par | 1.600 | 91.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 31798709ns | 31798709ns | -18.17% |
| engine_par | 31015257ns | 31015257ns | -20.18% |
| hand | 38858632ns | 38858632ns | base |
| hand_par | 7644067ns | 7644067ns | -80.33% |
| rust_nostd | 31767108ns | 31767108ns | -18.25% |
| rust_std | 36099842ns | 36099842ns | -7.10% |
| rust_std_par | 7878436ns | 7878436ns | -79.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 38790736ns | base | --- | [38787325, 38984659] | --- | --- | --- | --- |
| engine | 31787046ns | -7021878.8ns (-18.1%) | [-7214033, -6978278]ns | [31770626, 31834391] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 31355976ns | -7431363.7ns (-19.2%) | [-11174890, -5224773]ns | [27613961, 33604887] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 7657606ns | -31238016.5ns (-80.5%) | [-31584080, -30855481]ns | [7269146, 7955505] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 31752014ns | -7033666.3ns (-18.1%) | [-7225946, -7005025]ns | [31737772, 31784278] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 35951577ns | -2834103.4ns (-7.3%) | [-4268529, -1182575]ns | [34733278, 37609593] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 7863564ns | -31053936.3ns (-80.1%) | [-31238205, -30805303]ns | [7596240, 8055258] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 38911966ns | -18.3% | -22.9% | -79.7% | -18.4% | -10.6% | -79.6% |
| 1 | 2 | 38788314ns | -18.0% | -19.2% | -81.0% | -18.1% | -3.3% | -80.0% |

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
| engine | 11826885.7ns | 31797642.7ns | 37.2% | HIGH |
| engine_par | 12074185.3ns | 31012557.1ns | 38.9% | HIGH |
| hand | 12020209.5ns | 38857053.2ns | 30.9% | HIGH |
| hand_par | 12251857.5ns | 7640877.6ns | 160.3% | HIGH |
| rust_nostd | 11853033.5ns | 31765525.3ns | 37.3% | HIGH |
| rust_std | 11864525.7ns | 36098482.3ns | 32.9% | HIGH |
| rust_std_par | 12294626.8ns | 7875715.6ns | 156.1% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 31763441.9-31823263.3 ns)
  31763441.9 |########################################
  31766433.0 |
  31769424.0 |########################################
  31772415.1 |########################################
  31775406.2 |
  31778397.2 |########################################
  31781388.3 |
  31784379.4 |
  31787370.5 |
  31790361.5 |
  31793352.6 |########################################
  31796343.7 |
  31799334.7 |
  31802325.8 |
  31805316.9 |
  31808307.9 |########################################
  31811299.0 |
  31814290.1 |
  31817281.2 |
  31820272.2 |
  (0 below, 2 above range)

engine_par (n=8, range 27539341.2-32864899.5 ns)
  27539341.2 |########################################
  27805619.1 |
  28071897.0 |
  28338174.9 |
  28604452.9 |
  28870730.8 |
  29137008.7 |
  29403286.6 |
  29669564.5 |
  29935842.4 |
  30202120.4 |####################
  30468398.3 |
  30734676.2 |
  31000954.1 |####################
  31267232.0 |
  31533509.9 |####################
  31799787.8 |
  32066065.8 |
  32332343.7 |####################
  32598621.6 |
  (0 below, 2 above range)

hand (n=8, range 38782510.8-38927109.0 ns)
  38782510.8 |########################################
  38789740.7 |##########
  38796970.6 |
  38804200.5 |
  38811430.4 |
  38818660.4 |
  38825890.3 |
  38833120.2 |##########
  38840350.1 |
  38847580.0 |
  38854809.9 |
  38862039.8 |
  38869269.8 |
  38876499.7 |
  38883729.6 |
  38890959.5 |
  38898189.4 |
  38905419.3 |
  38912649.2 |
  38919879.1 |
  (0 below, 2 above range)

hand_par (n=8, range 7205223.0-7917828.0 ns)
  7205223.0 |########################################
  7240853.2 |########################################
  7276483.5 |
  7312113.7 |
  7347744.0 |
  7383374.2 |
  7419004.5 |
  7454634.7 |########################################
  7490265.0 |########################################
  7525895.2 |
  7561525.5 |
  7597155.7 |
  7632786.0 |
  7668416.2 |
  7704046.5 |
  7739676.7 |
  7775307.0 |
  7810937.2 |########################################
  7846567.5 |
  7882197.7 |########################################
  (0 below, 2 above range)

rust_nostd (n=8, range 31731915.9-31789116.9 ns)
  31731915.9 |########################################
  31734775.9 |
  31737636.0 |########################################
  31740496.0 |
  31743356.1 |
  31746216.1 |########################################
  31749076.2 |########################################
  31751936.2 |########################################
  31754796.3 |
  31757656.3 |########################################
  31760516.4 |
  31763376.4 |
  31766236.5 |
  31769096.5 |
  31771956.6 |
  31774816.6 |
  31777676.7 |
  31780536.7 |
  31783396.8 |########################################
  31786256.8 |
  (0 below, 1 above range)

rust_std (n=8, range 34320980.0-37466666.1 ns)
  34320980.0 |########################################
  34478264.3 |
  34635548.6 |########################################
  34792832.9 |########################################
  34950117.2 |########################################
  35107401.5 |
  35264685.8 |
  35421970.1 |
  35579254.4 |
  35736538.7 |
  35893823.0 |
  36051107.4 |
  36208391.7 |
  36365676.0 |
  36522960.3 |
  36680244.6 |
  36837528.9 |########################################
  36994813.2 |
  37152097.5 |
  37309381.8 |########################################
  (0 below, 2 above range)

rust_std_par (n=8, range 7526396.6-8106229.7 ns)
  7526396.6 |####################
  7555388.3 |
  7584379.9 |########################################
  7613371.6 |
  7642363.2 |
  7671354.9 |
  7700346.5 |
  7729338.2 |
  7758329.8 |
  7787321.5 |
  7816313.2 |
  7845304.8 |########################################
  7874296.5 |
  7903288.1 |
  7932279.8 |
  7961271.4 |####################
  7990263.1 |
  8019254.8 |
  8048246.4 |####################
  8077238.1 |
  (0 below, 1 above range)

```

## Diagnostics

- engine: bridge=37.2% of algo (FFI overhead may distort results)
- engine_par: bridge=38.5% of algo (FFI overhead may distort results)
- hand: bridge=30.9% of algo (FFI overhead may distort results)
- hand_par: bridge=159.7% of algo (FFI overhead may distort results)
- rust_nostd: bridge=37.3% of algo (FFI overhead may distort results)
- rust_std: bridge=33.0% of algo (FFI overhead may distort results)
- rust_std_par: bridge=156.3% of algo (FFI overhead may distort results)
