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

### hand_par dominates: 11% faster than the next best (hand)

hand_par (991.69 us) leads hand (1.10 ms) by 11%, a clear separation rather than a photo finish. CV 11.2%.


### engine_par is an outlier: 8.4x slower than the field

engine_par (8.34 ms) is 8.4x the fastest (991.69 us), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {hand_par, hand} vs {engine, rust_std_par, rust_nostd, rust_std, engine_par} (326% apart)

The field splits into a fast tier {hand_par, hand} and a slow tier {engine, rust_std_par, rust_nostd, rust_std, engine_par} with a 326% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 8.4x the fastest

Fastest hand_par (991.69 us) to slowest engine_par (8.34 ms): 8.4x. The strategy choice matters a lot for this workload.


### engine_par is inconsistent: worst-20% is 2.3x its best-20%

engine_par's best 20% of batches run at 4.93 ms but its worst 20% at 11.24 ms (2.3x) - a bimodal or bursty profile the median hides.


## Key findings

- Fastest: hand_par at 991685.9 ns median (-9.7% vs baseline)
- 5 variants significantly slower than baseline
- Spread: 8.41x (fastest 991685.9 ns, slowest 8342662.0 ns)

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
| engine | 4857257ns | 4679564ns | 4594071ns | 4632585ns | 5091558ns | +345.85% |
| engine_par | 8715240ns | 8345282ns | 4930465ns | 6612794ns | 11238268ns | +699.97% |
| hand | 1089446ns | 1098952ns | 1034344ns | 1063246ns | 1122872ns | base |
| hand_par | 999324ns | 994619ns | 836701ns | 939382ns | 1084937ns | -8.27% |
| rust_nostd | 6150547ns | 6044181ns | 5804210ns | 5905583ns | 6420855ns | +464.56% |
| rust_std | 6390988ns | 6305500ns | 5922823ns | 6172069ns | 6672218ns | +486.63% |
| rust_std_par | 4720497ns | 4693655ns | 4398866ns | 4507916ns | 4960340ns | +333.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 4856077ns | 4593277ns | 5090167ns | +346.33% | 2.591 |
| engine_par | 8712742ns | 4928219ns | 11235865ns | +700.80% | 1.444 |
| hand | 1088003ns | 1033202ns | 1121183ns | base | 11.565 |
| hand_par | 996283ns | 833870ns | 1081924ns | -8.43% | 12.630 |
| rust_nostd | 6149123ns | 5803088ns | 6419268ns | +465.18% | 2.046 |
| rust_std | 6389552ns | 5921665ns | 6670588ns | +487.27% | 1.969 |
| rust_std_par | 4718007ns | 4396874ns | 4957721ns | +333.64% | 2.667 |

## Performance model

- Peak throughput: 15.090 Gops/s (hand_par; best 20% batches)
- Ops per call: 12582912

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 2.690 | 17.8% |
| engine_par | 1.508 | 10.0% |
| hand | 11.463 | 76.0% |
| hand_par | 12.688 | 84.1% |
| rust_nostd | 2.082 | 13.8% |
| rust_std | 1.996 | 13.2% |
| rust_std_par | 2.682 | 17.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 4857257ns | 4857257ns | +345.85% |
| engine_par | 8715240ns | 8715240ns | +699.97% |
| hand | 1089446ns | 1089446ns | base |
| hand_par | 999324ns | 999324ns | -8.27% |
| rust_nostd | 6150547ns | 6150547ns | +464.56% |
| rust_std | 6390988ns | 6390988ns | +486.63% |
| rust_std_par | 4720497ns | 4720497ns | +333.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 1097681ns | base | --- | [1033280, 1127378] | --- | --- | --- | --- |
| engine | 4678402ns | +3628292.9ns (+330.5%) | [+3525221, +4241333]ns | [4622284, 5368711] | YES | 0.0094 | 0.0078 | 0 |
| engine_par | 8342662ns | +7272764.7ns (+662.6%) | [+4218778, +10766727]ns | [5355824, 11800007] | YES | 0.0094 | 0.0078 | 0 |
| hand_par | 991686ns | no significant difference | [-178662, +61088]ns | [875543, 1094290] | no | 0.2891 | 0.2891 | 0 |
| rust_nostd | 6042973ns | +5009732.4ns (+456.4%) | [+4736188, +5552512]ns | [5805319, 6679890] | YES | 0.0094 | 0.0078 | 0 |
| rust_std | 6303999ns | +5253889.3ns (+478.6%) | [+4933631, +5806838]ns | [6019419, 6934215] | YES | 0.0094 | 0.0078 | 0 |
| rust_std_par | 4690933ns | +3614530.1ns (+329.3%) | [+3363672, +3892653]ns | [4430485, 5029563] | YES | 0.0094 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1110154ns | +324.7% | +649.9% | -8.8% | +425.7% | +452.2% | +330.0% |
| 1 | 2 | 1050149ns | +345.5% | +694.4% | -12.7% | +481.2% | +505.4% | +340.1% |

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
- hand_par: won 2/2, lost 0/2
- rust_nostd: won 0/2, lost 2/2
- rust_std: won 0/2, lost 2/2
- rust_std_par: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 11894994.6ns | 4856077.2ns | 245.0% | HIGH |
| engine_par | 12349239.0ns | 8712742.0ns | 141.7% | HIGH |
| hand | 12459613.1ns | 1088002.5ns | 1145.2% | HIGH |
| hand_par | 13170422.4ns | 996282.6ns | 1322.0% | HIGH |
| rust_nostd | 11978500.2ns | 6149122.5ns | 194.8% | HIGH |
| rust_std | 11946062.5ns | 6389551.8ns | 187.0% | HIGH |
| rust_std_par | 12254876.7ns | 4718006.6ns | 259.7% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 4593277.1-5090167.3 ns)
  4593277.1 |#############
  4618121.6 |########################################
  4642966.1 |
  4667810.6 |
  4692655.1 |
  4717499.6 |#############
  4742344.2 |
  4767188.7 |
  4792033.2 |#############
  4816877.7 |
  4841722.2 |
  4866566.7 |
  4891411.2 |
  4916255.7 |
  4941100.2 |
  4965944.8 |
  4990789.3 |
  5015633.8 |
  5040478.3 |
  5065322.8 |
  (0 below, 2 above range)

engine_par (n=8, range 4928219.3-11235865.3 ns)
  4928219.3 |####################
  5243601.6 |####################
  5558983.9 |
  5874366.2 |
  6189748.5 |
  6505130.8 |
  6820513.1 |
  7135895.4 |########################################
  7451277.7 |
  7766660.0 |
  8082042.3 |
  8397424.6 |
  8712806.9 |
  9028189.2 |
  9343571.5 |########################################
  9658953.8 |
  9974336.1 |
  10289718.4 |
  10605100.7 |
  10920483.0 |
  (0 below, 2 above range)

hand (n=8, range 1033201.7-1121183.0 ns)
  1033201.7 |########################################
  1037600.8 |
  1041999.8 |
  1046398.9 |
  1050798.0 |
  1055197.0 |
  1059596.1 |
  1063995.2 |####################
  1068394.2 |
  1072793.3 |
  1077192.4 |
  1081591.4 |####################
  1085990.5 |
  1090389.6 |
  1094788.6 |
  1099187.7 |
  1103586.8 |
  1107985.8 |########################################
  1112384.9 |
  1116784.0 |
  (0 below, 2 above range)

hand_par (n=8, range 833870.4-1081924.4 ns)
  833870.4 |####################
  846273.1 |
  858675.8 |
  871078.5 |####################
  883481.2 |
  895883.9 |
  908286.6 |
  920689.3 |
  933092.0 |
  945494.7 |
  957897.4 |####################
  970300.1 |####################
  982702.8 |
  995105.5 |
  1007508.2 |########################################
  1019910.9 |
  1032313.6 |
  1044716.3 |
  1057119.0 |
  1069521.7 |
  (0 below, 2 above range)

rust_nostd (n=8, range 5803087.5-6419268.1 ns)
  5803087.5 |########################################
  5833896.5 |
  5864705.6 |####################
  5895514.6 |
  5926323.6 |
  5957132.7 |
  5987941.7 |
  6018750.7 |########################################
  6049559.7 |
  6080368.8 |
  6111177.8 |
  6141986.8 |####################
  6172795.9 |
  6203604.9 |
  6234413.9 |
  6265223.0 |
  6296032.0 |
  6326841.0 |
  6357650.0 |
  6388459.1 |
  (0 below, 2 above range)

rust_std (n=8, range 5921664.9-6670588.4 ns)
  5921664.9 |####################
  5959111.1 |
  5996557.2 |####################
  6034003.4 |
  6071449.6 |
  6108895.8 |
  6146342.0 |
  6183788.1 |
  6221234.3 |########################################
  6258680.5 |
  6296126.7 |
  6333572.8 |########################################
  6371019.0 |
  6408465.2 |
  6445911.4 |
  6483357.5 |
  6520803.7 |
  6558249.9 |
  6595696.1 |
  6633142.2 |
  (0 below, 2 above range)

rust_std_par (n=8, range 4396873.9-4957721.4 ns)
  4396873.9 |########################################
  4424916.3 |########################################
  4452958.6 |########################################
  4481001.0 |
  4509043.4 |
  4537085.8 |
  4565128.1 |
  4593170.5 |########################################
  4621212.9 |
  4649255.3 |
  4677297.6 |
  4705340.0 |
  4733382.4 |
  4761424.8 |########################################
  4789467.1 |
  4817509.5 |
  4845551.9 |
  4873594.3 |
  4901636.6 |
  4929679.0 |########################################
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=253.5% of algo (FFI overhead may distort results)
- engine_par: CV=34.3% (high variance, measurements may be unstable)
- engine_par: bridge=144.5% of algo (FFI overhead may distort results)
- hand: bridge=1110.7% of algo (FFI overhead may distort results)
- hand_par: bridge=1230.9% of algo (FFI overhead may distort results)
- rust_nostd: bridge=199.0% of algo (FFI overhead may distort results)
- rust_std: bridge=189.5% of algo (FFI overhead may distort results)
- rust_std_par: bridge=260.8% of algo (FFI overhead may distort results)
