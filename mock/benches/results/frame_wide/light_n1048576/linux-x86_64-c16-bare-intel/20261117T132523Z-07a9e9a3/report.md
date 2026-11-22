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

### hand_par dominates: 23% faster than the next best (hand)

hand_par (878.80 us) leads hand (1.08 ms) by 23%, a clear separation rather than a photo finish. CV 3.2%.


### rust_std is an outlier: 7.0x slower than the field

rust_std (6.19 ms) is 7.0x the fastest (878.80 us), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {hand_par, hand} vs {rust_std_par, engine, engine_par, rust_nostd, rust_std} (325% apart)

The field splits into a fast tier {hand_par, hand} and a slow tier {rust_std_par, engine, engine_par, rust_nostd, rust_std} with a 325% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 7.0x the fastest

Fastest hand_par (878.80 us) to slowest rust_std (6.19 ms): 7.0x. The strategy choice matters a lot for this workload.


### engine_par is inconsistent: worst-20% is 1.6x its best-20%

engine_par's best 20% of batches run at 4.68 ms but its worst 20% at 7.30 ms (1.6x) - a bimodal or bursty profile the median hides.


## Key findings

- Fastest: hand_par at 878802.0 ns median (-18.5% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 7.04x (fastest 878802.0 ns, slowest 6190170.3 ns)

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
| engine | 4955223ns | 4857846ns | 4605530ns | 4714222ns | 5223397ns | +356.27% |
| engine_par | 6123994ns | 5259286ns | 4684126ns | 5032728ns | 7302411ns | +463.89% |
| hand | 1086030ns | 1079321ns | 963825ns | 995797ns | 1184256ns | base |
| hand_par | 887692ns | 881714ns | 853493ns | 867754ns | 911196ns | -18.26% |
| rust_nostd | 6162216ns | 6015614ns | 5863213ns | 5950017ns | 6396115ns | +467.41% |
| rust_std | 6314119ns | 6191474ns | 6014545ns | 6060867ns | 6578953ns | +481.39% |
| rust_std_par | 4576135ns | 4580537ns | 4310785ns | 4445495ns | 4740453ns | +321.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 4953995ns | 4604453ns | 5221989ns | +356.79% | 2.540 |
| engine_par | 6121439ns | 4681587ns | 7299761ns | +464.43% | 2.056 |
| hand | 1084532ns | 962612ns | 1182738ns | base | 11.602 |
| hand_par | 884665ns | 850522ns | 908157ns | -18.43% | 14.223 |
| rust_nostd | 6160633ns | 5861779ns | 6394357ns | +468.05% | 2.042 |
| rust_std | 6312730ns | 6013374ns | 6577450ns | +482.07% | 1.993 |
| rust_std_par | 4573522ns | 4308569ns | 4737708ns | +321.70% | 2.751 |

## Performance model

- Peak throughput: 14.794 Gops/s (hand_par; best 20% batches)
- Ops per call: 12582912

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 2.591 | 17.5% |
| engine_par | 2.394 | 16.2% |
| hand | 11.676 | 78.9% |
| hand_par | 14.318 | 96.8% |
| rust_nostd | 2.092 | 14.1% |
| rust_std | 2.033 | 13.7% |
| rust_std_par | 2.749 | 18.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 4955223ns | 4955223ns | +356.27% |
| engine_par | 6123994ns | 6123994ns | +463.89% |
| hand | 1086030ns | 1086030ns | base |
| hand_par | 887692ns | 887692ns | -18.26% |
| rust_nostd | 6162216ns | 6162216ns | +467.41% |
| rust_std | 6314119ns | 6314119ns | +481.39% |
| rust_std_par | 4576135ns | 4576135ns | +321.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 1077684ns | base | --- | [991490, 1197644] | --- | --- | --- | --- |
| engine | 4856729ns | +3772693.2ns (+350.1%) | [+3497764, +4390481]ns | [4640441, 5507518] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 5256796ns | +4056169.5ns (+376.4%) | [+3849678, +6435268]ns | [4887844, 7431467] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 878802ns | -205450.4ns (-19.1%) | [-309185, -73952]ns | [857456, 922248] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 6014219ns | +4927653.2ns (+457.2%) | [+4809269, +5546438]ns | [5922567, 6602396] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 6190170ns | +5110042.5ns (+474.2%) | [+4854205, +5707298]ns | [6016706, 6730821] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 4577900ns | +3491085.1ns (+323.9%) | [+3272056, +3653105]ns | [4378493, 4694683] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 993245ns | +389.0% | +520.2% | -9.2% | +510.1% | +527.8% | +360.9% |
| 1 | 2 | 1184087ns | +304.2% | +344.0% | -27.4% | +407.9% | +419.6% | +285.6% |

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
| engine | 11952528.9ns | 4953995.0ns | 241.3% | HIGH |
| engine_par | 11960311.8ns | 6121438.7ns | 195.4% | HIGH |
| hand | 12126631.7ns | 1084531.8ns | 1118.1% | HIGH |
| hand_par | 12116363.0ns | 884665.4ns | 1369.6% | HIGH |
| rust_nostd | 11961365.3ns | 6160632.7ns | 194.2% | HIGH |
| rust_std | 11855101.8ns | 6312730.1ns | 187.8% | HIGH |
| rust_std_par | 12521434.3ns | 4573521.7ns | 273.8% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 4604453.2-5221988.5 ns)
  4604453.2 |####################
  4635330.0 |####################
  4666206.7 |####################
  4697083.5 |
  4727960.3 |
  4758837.0 |
  4789713.8 |
  4820590.6 |####################
  4851467.3 |
  4882344.1 |########################################
  4913220.9 |
  4944097.6 |
  4974974.4 |
  5005851.2 |
  5036727.9 |
  5067604.7 |
  5098481.5 |
  5129358.2 |
  5160235.0 |
  5191111.8 |
  (0 below, 2 above range)

engine_par (n=8, range 4681587.2-7299761.4 ns)
  4681587.2 |########################################
  4812495.9 |########################################
  4943404.6 |########################################
  5074313.3 |########################################
  5205222.0 |########################################
  5336130.7 |
  5467039.5 |
  5597948.2 |
  5728856.9 |
  5859765.6 |
  5990674.3 |
  6121583.0 |
  6252491.7 |
  6383400.4 |
  6514309.1 |
  6645217.8 |
  6776126.5 |
  6907035.2 |########################################
  7037944.0 |
  7168852.7 |
  (0 below, 2 above range)

hand (n=8, range 962612.1-1182738.1 ns)
  962612.1 |####################
  973618.4 |
  984624.7 |########################################
  995631.0 |####################
  1006637.3 |
  1017643.6 |
  1028649.9 |
  1039656.2 |
  1050662.5 |
  1061668.8 |
  1072675.1 |
  1083681.4 |
  1094687.7 |
  1105694.0 |
  1116700.3 |
  1127706.6 |
  1138712.9 |
  1149719.2 |####################
  1160725.5 |####################
  1171731.8 |
  (0 below, 2 above range)

hand_par (n=8, range 850522.3-908156.6 ns)
  850522.3 |########################################
  853404.0 |
  856285.7 |########################################
  859167.4 |########################################
  862049.2 |
  864930.9 |
  867812.6 |
  870694.3 |
  873576.0 |########################################
  876457.7 |
  879339.4 |
  882221.1 |########################################
  885102.9 |
  887984.6 |
  890866.3 |
  893748.0 |########################################
  896629.7 |
  899511.4 |
  902393.1 |
  905274.8 |
  (0 below, 2 above range)

rust_nostd (n=8, range 5861778.9-6394357.5 ns)
  5861778.9 |########################################
  5888407.8 |
  5915036.8 |########################################
  5941665.7 |########################################
  5968294.6 |########################################
  5994923.5 |
  6021552.5 |
  6048181.4 |########################################
  6074810.3 |
  6101439.2 |
  6128068.2 |
  6154697.1 |########################################
  6181326.0 |
  6207955.0 |
  6234583.9 |
  6261212.8 |
  6287841.7 |
  6314470.7 |
  6341099.6 |
  6367728.5 |
  (0 below, 2 above range)

rust_std (n=8, range 6013373.9-6577449.9 ns)
  6013373.9 |########################################
  6041577.7 |
  6069781.5 |########################################
  6097985.3 |
  6126189.1 |
  6154392.9 |
  6182596.7 |
  6210800.5 |
  6239004.3 |
  6267208.1 |####################
  6295411.9 |
  6323615.7 |
  6351819.5 |####################
  6380023.3 |
  6408227.1 |
  6436430.9 |
  6464634.7 |
  6492838.5 |
  6521042.3 |
  6549246.1 |
  (0 below, 2 above range)

rust_std_par (n=8, range 4308568.8-4737707.7 ns)
  4308568.8 |####################
  4330025.7 |
  4351482.7 |
  4372939.6 |####################
  4394396.6 |
  4415853.5 |
  4437310.5 |####################
  4458767.4 |
  4480224.4 |
  4501681.3 |####################
  4523138.2 |
  4544595.2 |
  4566052.1 |
  4587509.1 |
  4608966.0 |
  4630423.0 |####################
  4651879.9 |
  4673336.9 |########################################
  4694793.8 |
  4716250.8 |
  (0 below, 1 above range)

```

## Diagnostics

- engine: bridge=246.8% of algo (FFI overhead may distort results)
- engine_par: CV=25.7% (high variance, measurements may be unstable)
- engine_par: bridge=226.9% of algo (FFI overhead may distort results)
- hand: bridge=1127.5% of algo (FFI overhead may distort results)
- hand_par: bridge=1377.9% of algo (FFI overhead may distort results)
- rust_nostd: bridge=199.0% of algo (FFI overhead may distort results)
- rust_std: bridge=191.5% of algo (FFI overhead may distort results)
- rust_std_par: bridge=270.3% of algo (FFI overhead may distort results)
