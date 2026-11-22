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

The declared/defaulted baseline hand has the worst median (38.84 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hand_par at 7.92 ms).


### hand_par beats baseline by 80% (significant)

hand_par is -30.89 ms (80%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 4.9x slower than the field

hand (38.84 ms) is 4.9x the fastest (7.92 ms), well off the pack.


### hand_par is fastest but the noisiest (CV 115.5%)

hand_par wins on median (7.92 ms) yet has the highest variance (CV 115.5%), while rust_nostd is the steadiest (CV 0.2%, 31.77 ms).


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {hand_par, rust_std_par} vs {rust_nostd, engine, rust_std, engine_par, hand} (288% apart)

The field splits into a fast tier {hand_par, rust_std_par} and a slow tier {rust_nostd, engine, rust_std, engine_par, hand} with a 288% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 4.9x the fastest

Fastest hand_par (7.92 ms) to slowest hand (38.84 ms): 4.9x. The strategy choice matters a lot for this workload.


### hand_par is inconsistent: worst-20% is 3.0x its best-20%

hand_par's best 20% of batches run at 7.08 ms but its worst 20% at 21.50 ms (3.0x) - a bimodal or bursty profile the median hides.


## Key findings

- Fastest: hand_par at 7922427.2 ns median (-79.6% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 4.90x (fastest 7922427.2 ns, slowest 38838246.4 ns)

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
| Counter frequency | 2112550401 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 32155359ns | 32259614ns | 31803268ns | 32046530ns | 32325003ns | -17.48% |
| engine_par | 36935525ns | 35618248ns | 33059856ns | 34297545ns | 39882927ns | -5.21% |
| hand | 38966298ns | 38839677ns | 38762100ns | 38790597ns | 39149124ns | base |
| hand_par | 14497081ns | 7925736ns | 7085468ns | 7628566ns | 21501370ns | -62.80% |
| rust_nostd | 31785892ns | 31768117ns | 31730782ns | 31745088ns | 31830273ns | -18.43% |
| rust_std | 34610253ns | 34648702ns | 34221714ns | 34500760ns | 34789507ns | -11.18% |
| rust_std_par | 8204055ns | 8196705ns | 7354714ns | 7980024ns | 8584412ns | -78.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 32154231ns | 31802130ns | 32323726ns | -17.48% | 0.391 |
| engine_par | 36933058ns | 33057164ns | 39880428ns | -5.21% | 0.341 |
| hand | 38964889ns | 38760943ns | 39147694ns | base | 0.323 |
| hand_par | 14493892ns | 7082713ns | 21498050ns | -62.80% | 0.868 |
| rust_nostd | 31784154ns | 31728587ns | 31828647ns | -18.43% | 0.396 |
| rust_std | 34608796ns | 34220547ns | 34788150ns | -11.18% | 0.364 |
| rust_std_par | 8201730ns | 7352749ns | 8581895ns | -78.95% | 1.534 |

## Performance model

- Peak throughput: 1.777 Gops/s (hand_par; best 20% batches)
- Ops per call: 12582912

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.390 | 22.0% |
| engine_par | 0.353 | 19.9% |
| hand | 0.324 | 18.2% |
| hand_par | 1.588 | 89.4% |
| rust_nostd | 0.396 | 22.3% |
| rust_std | 0.363 | 20.4% |
| rust_std_par | 1.536 | 86.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 32155359ns | 32155359ns | -17.48% |
| engine_par | 36935525ns | 36935525ns | -5.21% |
| hand | 38966298ns | 38966298ns | base |
| hand_par | 14497081ns | 14497081ns | -62.80% |
| rust_nostd | 31785892ns | 31785892ns | -18.43% |
| rust_std | 34610253ns | 34610253ns | -11.18% |
| rust_std_par | 8204055ns | 8204055ns | -78.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 38838246ns | base | --- | [38761368, 38886835] | --- | --- | --- | --- |
| engine | 32258403ns | -6619980.0ns (-17.0%) | [-6883961, -6551464]ns | [31876983, 32290641] | YES | 0.0094 | 0.0078 | 0 |
| engine_par | 35615632ns | no significant difference | [-5696054, +529388]ns | [33065314, 39779394] | no | 0.2891 | 0.2891 | 0 |
| hand_par | 7922427ns | -30893222.5ns (-79.5%) | [-31678230, -14234620]ns | [7542423, 24599768] | YES | 0.0094 | 0.0078 | 0 |
| rust_nostd | 31766644ns | -7030920.5ns (-18.1%) | [-7141344, -6986631]ns | [31731596, 31855474] | YES | 0.0094 | 0.0078 | 0 |
| rust_std | 34646886ns | -4201474.3ns (-10.8%) | [-4666288, -3943280]ns | [34258603, 34873052] | YES | 0.0094 | 0.0078 | 0 |
| rust_std_par | 8194371ns | -30811226.9ns (-79.3%) | [-31358805, -30270829]ns | [7834738, 8633102] | YES | 0.0094 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 38766502ns | -17.6% | -11.3% | -80.5% | -18.1% | -10.6% | -78.8% |
| 1 | 2 | 38856018ns | -17.0% | -3.6% | -39.4% | -18.1% | -10.8% | -78.9% |

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
| engine | 11962153.8ns | 32154230.5ns | 37.2% | HIGH |
| engine_par | 11999383.3ns | 36933057.9ns | 32.5% | HIGH |
| hand | 12178306.1ns | 38964889.2ns | 31.3% | HIGH |
| hand_par | 13086262.3ns | 14493891.8ns | 90.3% | HIGH |
| rust_nostd | 11870121.5ns | 31784154.0ns | 37.3% | HIGH |
| rust_std | 11859875.6ns | 34608795.9ns | 34.3% | HIGH |
| rust_std_par | 12906025.3ns | 8201729.5ns | 157.4% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 31802129.9-32323726.5 ns)
  31802129.9 |####################
  31828209.7 |
  31854289.6 |####################
  31880369.4 |
  31906449.2 |
  31932529.0 |
  31958608.9 |
  31984688.7 |####################
  32010768.5 |
  32036848.4 |
  32062928.2 |
  32089008.0 |
  32115087.8 |
  32141167.7 |
  32167247.5 |
  32193327.3 |
  32219407.2 |
  32245487.0 |########################################
  32271566.8 |########################################
  32297646.6 |
  (0 below, 1 above range)

engine_par (n=8, range 33057163.8-39880428.2 ns)
  33057163.8 |########################################
  33398327.0 |
  33739490.2 |
  34080653.5 |####################
  34421816.7 |
  34762979.9 |
  35104143.1 |
  35445306.3 |########################################
  35786469.5 |
  36127632.8 |
  36468796.0 |
  36809959.2 |
  37151122.4 |
  37492285.6 |
  37833448.9 |
  38174612.1 |
  38515775.3 |
  38856938.5 |
  39198101.7 |####################
  39539265.0 |####################
  (0 below, 1 above range)

hand (n=8, range 38760943.4-39147694.5 ns)
  38760943.4 |########################################
  38780281.0 |
  38799618.5 |
  38818956.1 |#############
  38838293.6 |#############
  38857631.2 |#############
  38876968.7 |#############
  38896306.3 |
  38915643.8 |
  38934981.4 |
  38954319.0 |
  38973656.5 |
  38992994.1 |
  39012331.6 |
  39031669.2 |
  39051006.7 |
  39070344.3 |
  39089681.8 |
  39109019.4 |
  39128356.9 |
  (0 below, 1 above range)

hand_par (n=8, range 7082712.9-21498050.0 ns)
  7082712.9 |########################################
  7803479.8 |##########
  8524246.6 |
  9245013.5 |
  9965780.3 |
  10686547.2 |
  11407314.0 |
  12128080.9 |
  12848847.8 |
  13569614.6 |
  14290381.5 |
  15011148.3 |
  15731915.2 |
  16452682.0 |
  17173448.9 |
  17894215.7 |
  18614982.6 |
  19335749.5 |
  20056516.3 |
  20777283.2 |
  (0 below, 3 above range)

rust_nostd (n=8, range 31728587.4-31828647.3 ns)
  31728587.4 |########################################
  31733590.4 |
  31738593.4 |####################
  31743596.4 |
  31748599.4 |
  31753602.4 |
  31758605.4 |####################
  31763608.4 |
  31768611.3 |
  31773614.3 |########################################
  31778617.3 |
  31783620.3 |
  31788623.3 |
  31793626.3 |
  31798629.3 |
  31803632.3 |
  31808635.3 |
  31813638.3 |
  31818641.3 |
  31823644.3 |
  (0 below, 2 above range)

rust_std (n=8, range 34220546.8-34788149.8 ns)
  34220546.8 |########################################
  34248927.0 |########################################
  34277307.1 |
  34305687.3 |
  34334067.4 |
  34362447.6 |
  34390827.7 |
  34419207.9 |
  34447588.0 |
  34475968.2 |
  34504348.3 |
  34532728.5 |
  34561108.6 |
  34589488.8 |########################################
  34617868.9 |########################################
  34646249.1 |########################################
  34674629.2 |
  34703009.4 |########################################
  34731389.5 |
  34759769.7 |
  (0 below, 2 above range)

rust_std_par (n=8, range 7352749.4-8581894.8 ns)
  7352749.4 |####################
  7414206.7 |
  7475663.9 |
  7537121.2 |
  7598578.5 |
  7660035.7 |
  7721493.0 |
  7782950.3 |####################
  7844407.6 |
  7905864.8 |
  7967322.1 |
  8028779.4 |########################################
  8090236.6 |
  8151693.9 |
  8213151.2 |
  8274608.4 |####################
  8336065.7 |
  8397523.0 |
  8458980.2 |
  8520437.5 |
  (0 below, 3 above range)

```

## Diagnostics

- engine: bridge=37.2% of algo (FFI overhead may distort results)
- engine_par: bridge=33.7% of algo (FFI overhead may distort results)
- hand: bridge=30.9% of algo (FFI overhead may distort results)
- hand_par: CV=63.1% (high variance, measurements may be unstable)
- hand_par: worst_20/best_20 = 3.0x (possible bimodal distribution)
- hand_par: bridge=160.9% of algo (FFI overhead may distort results)
- rust_nostd: bridge=37.3% of algo (FFI overhead may distort results)
- rust_std: bridge=34.2% of algo (FFI overhead may distort results)
- rust_std_par: bridge=149.9% of algo (FFI overhead may distort results)
