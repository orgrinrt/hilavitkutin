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

### hand dominates: 111% faster than the next best (rust_std)

hand (52.93 us) leads rust_std (111.65 us) by 111%, a clear separation rather than a photo finish. CV 0.8%.


### engine_par is an outlier: 16.8x slower than the field

engine_par (890.28 us) is 16.8x the fastest (52.93 us), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (hand)

The baseline hand is the fastest (52.93 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 16.8x the fastest

Fastest hand (52.93 us) to slowest engine_par (890.28 us): 16.8x. The strategy choice matters a lot for this workload.


### engine_par is inconsistent: worst-20% is 2.5x its best-20%

engine_par's best 20% of batches run at 471.59 us but its worst 20% at 1.17 ms (2.5x) - a bimodal or bursty profile the median hides.


## Key findings

- Baseline (hand) is the fastest at 52928.6 ns median
- 6 variants significantly slower than baseline
- Spread: 16.82x (fastest 52928.6 ns, slowest 890275.1 ns)

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
| Counter frequency | 2112550401 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 122502ns | 122213ns | 118792ns | 119139ns | 125952ns | +130.96% |
| engine_par | 884700ns | 891606ns | 472849ns | 634824ns | 1175069ns | +1568.00% |
| hand | 53040ns | 53077ns | 52285ns | 52801ns | 53407ns | base |
| hand_par | 311818ns | 309104ns | 232659ns | 267188ns | 365081ns | +487.90% |
| rust_nostd | 114695ns | 115466ns | 109717ns | 111876ns | 118054ns | +116.24% |
| rust_std | 111621ns | 111884ns | 110328ns | 111441ns | 112080ns | +110.45% |
| rust_std_par | 507413ns | 487655ns | 403697ns | 447968ns | 577926ns | +856.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 122302ns | 118637ns | 125743ns | +131.37% | 6.430 |
| engine_par | 882980ns | 471590ns | 1172794ns | +1570.44% | 0.891 |
| hand | 52859ns | 52119ns | 53191ns | base | 14.878 |
| hand_par | 310393ns | 232138ns | 363244ns | +487.21% | 2.534 |
| rust_nostd | 114501ns | 109536ns | 117859ns | +116.61% | 6.868 |
| rust_std | 111418ns | 110107ns | 111860ns | +110.78% | 7.058 |
| rust_std_par | 505265ns | 402371ns | 575248ns | +855.87% | 1.556 |

## Performance model

- Peak throughput: 15.089 Gops/s (hand; best 20% batches)
- Ops per call: 786432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 6.448 | 42.7% |
| engine_par | 0.883 | 5.9% |
| hand | 14.858 | 98.5% |
| hand_par | 2.558 | 17.0% |
| rust_nostd | 6.824 | 45.2% |
| rust_std | 7.044 | 46.7% |
| rust_std_par | 1.619 | 10.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 122502ns | 122502ns | +130.96% |
| engine_par | 884700ns | 884700ns | +1568.00% |
| hand | 53040ns | 53040ns | base |
| hand_par | 311818ns | 311818ns | +487.90% |
| rust_nostd | 114695ns | 114695ns | +116.24% |
| rust_std | 111621ns | 111621ns | +110.45% |
| rust_std_par | 507413ns | 507413ns | +856.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 52929ns | base | --- | [52467, 53197] | --- | --- | --- | --- |
| engine | 121969ns | +69380.6ns (+131.1%) | [+65440, +73457]ns | [118641, 126129] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 890275ns | +837617.1ns (+1582.5%) | [+449082, +1128422]ns | [502586, 1181428] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 307419ns | +254606.9ns (+481.0%) | [+185185, +293507]ns | [238190, 346359] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 115250ns | +62662.0ns (+118.4%) | [+57798, +65023]ns | [110470, 118028] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 111652ns | +58602.1ns (+110.7%) | [+57932, +59149]ns | [110937, 111860] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 485708ns | +433312.8ns (+818.7%) | [+368655, +520494]ns | [422159, 572961] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 52762ns | +138.8% | +1501.4% | +537.1% | +109.4% | +111.2% | +896.8% |
| 1 | 2 | 53101ns | +123.6% | +1576.6% | +403.3% | +122.1% | +110.6% | +775.9% |

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
| engine | 726668.5ns | 122301.6ns | 594.2% | HIGH |
| engine_par | 3669899.3ns | 882979.8ns | 415.6% | HIGH |
| hand | 725320.4ns | 52859.2ns | 1372.2% | HIGH |
| hand_par | 2611605.1ns | 310392.6ns | 841.4% | HIGH |
| rust_nostd | 727727.9ns | 114500.8ns | 635.6% | HIGH |
| rust_std | 726024.7ns | 111418.2ns | 651.6% | HIGH |
| rust_std_par | 4337072.8ns | 505264.9ns | 858.4% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 118636.6-125743.1 ns)
  118636.6 |########################################
  118991.9 |
  119347.2 |#############
  119702.6 |
  120057.9 |
  120413.2 |
  120768.5 |
  121123.9 |
  121479.2 |
  121834.5 |
  122189.8 |
  122545.2 |
  122900.5 |
  123255.8 |
  123611.1 |
  123966.5 |
  124321.8 |#############
  124677.1 |
  125032.4 |
  125387.8 |
  (0 below, 3 above range)

engine_par (n=8, range 471589.7-1172793.9 ns)
  471589.7 |########################################
  506649.9 |####################
  541710.1 |
  576770.3 |
  611830.5 |
  646890.7 |
  681950.9 |
  717011.2 |
  752071.4 |
  787131.6 |
  822191.8 |
  857252.0 |####################
  892312.2 |####################
  927372.4 |
  962432.6 |
  997492.8 |
  1032553.0 |
  1067613.2 |
  1102673.4 |
  1137733.6 |
  (0 below, 3 above range)

hand (n=8, range 52119.4-53190.8 ns)
  52119.4 |########################################
  52173.0 |
  52226.5 |
  52280.1 |
  52333.7 |
  52387.2 |
  52440.8 |########################################
  52494.4 |
  52548.0 |
  52601.5 |
  52655.1 |########################################
  52708.7 |
  52762.2 |
  52815.8 |########################################
  52869.4 |
  52922.9 |
  52976.5 |########################################
  53030.1 |########################################
  53083.6 |
  53137.2 |
  (0 below, 2 above range)

hand_par (n=8, range 232137.6-363243.5 ns)
  232137.6 |########################################
  238692.9 |
  245248.2 |
  251803.5 |
  258358.8 |####################
  264914.1 |
  271469.4 |
  278024.7 |
  284580.0 |
  291135.3 |####################
  297690.6 |
  304245.9 |
  310801.2 |
  317356.5 |####################
  323911.7 |####################
  330467.0 |
  337022.3 |
  343577.6 |####################
  350132.9 |
  356688.2 |
  (0 below, 1 above range)

rust_nostd (n=8, range 109536.5-117858.7 ns)
  109536.5 |####################
  109952.6 |
  110368.7 |########################################
  110784.8 |
  111200.9 |
  111617.0 |
  112033.2 |
  112449.3 |
  112865.4 |
  113281.5 |
  113697.6 |####################
  114113.7 |
  114529.8 |
  114945.9 |
  115362.0 |
  115778.1 |
  116194.3 |####################
  116610.4 |
  117026.5 |
  117442.6 |####################
  (0 below, 2 above range)

rust_std (n=8, range 110107.3-111859.9 ns)
  110107.3 |####################
  110194.9 |
  110282.6 |
  110370.2 |
  110457.8 |
  110545.4 |
  110633.1 |
  110720.7 |
  110808.3 |
  110895.9 |####################
  110983.6 |
  111071.2 |
  111158.8 |####################
  111246.5 |
  111334.1 |
  111421.7 |
  111509.3 |
  111597.0 |####################
  111684.6 |####################
  111772.2 |########################################
  (0 below, 1 above range)

rust_std_par (n=8, range 402371.2-575248.3 ns)
  402371.2 |########################################
  411015.1 |
  419658.9 |########################################
  428302.8 |
  436946.6 |########################################
  445590.5 |
  454234.3 |
  462878.2 |
  471522.0 |########################################
  480165.9 |
  488809.7 |########################################
  497453.6 |
  506097.4 |
  514741.3 |
  523385.2 |########################################
  532029.0 |
  540672.9 |
  549316.7 |
  557960.6 |
  566604.4 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- engine: bridge=596.1% of algo (FFI overhead may distort results)
- engine_par: CV=38.6% (high variance, measurements may be unstable)
- engine_par: bridge=345.8% of algo (FFI overhead may distort results)
- hand: bridge=1370.2% of algo (FFI overhead may distort results)
- hand_par: CV=22.3% (high variance, measurements may be unstable)
- hand_par: bridge=748.4% of algo (FFI overhead may distort results)
- rust_nostd: bridge=630.6% of algo (FFI overhead may distort results)
- rust_std: bridge=650.1% of algo (FFI overhead may distort results)
- rust_std_par: bridge=844.0% of algo (FFI overhead may distort results)
