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

### hand dominates: 24% faster than the next best (hand_par)

hand (4.04 ms) leads hand_par (5.00 ms) by 24%, a clear separation rather than a photo finish. CV 2.4%.


### rust_std is an outlier: 6.9x slower than the field

rust_std (27.73 ms) is 6.9x the fastest (4.04 ms), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (hand)

The baseline hand is the fastest (4.04 ms median); no rival improves on it (all deltas are >= 0).


### Two tiers: {hand, hand_par} vs {engine, engine_par, rust_nostd, rust_std_par, rust_std} (270% apart)

The field splits into a fast tier {hand, hand_par} and a slow tier {engine, engine_par, rust_nostd, rust_std_par, rust_std} with a 270% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 6.9x the fastest

Fastest hand (4.04 ms) to slowest rust_std (27.73 ms): 6.9x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (hand) is the fastest at 4036430.4 ns median
- 6 variants significantly slower than baseline
- Spread: 6.87x (fastest 4036430.4 ns, slowest 27727811.1 ns)

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
| engine | 18522341ns | 18493269ns | 18371431ns | 18413557ns | 18641657ns | +355.60% |
| engine_par | 19535216ns | 19621643ns | 17702563ns | 18439671ns | 20815039ns | +380.51% |
| hand | 4065480ns | 4037887ns | 3971025ns | 3974559ns | 4157284ns | base |
| hand_par | 5023188ns | 5006207ns | 4954642ns | 4989104ns | 5065889ns | +23.56% |
| rust_nostd | 22340879ns | 22240644ns | 22067485ns | 22141627ns | 22558666ns | +449.53% |
| rust_std | 27992156ns | 27729077ns | 27046625ns | 27460644ns | 28627173ns | +588.53% |
| rust_std_par | 26584717ns | 26509719ns | 26306494ns | 26402459ns | 26790967ns | +553.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 18521182ns | 18370336ns | 18640484ns | +355.74% | 2.718 |
| engine_par | 19532576ns | 17700145ns | 20812254ns | +380.63% | 2.577 |
| hand | 4063968ns | 3969120ns | 4155783ns | base | 12.385 |
| hand_par | 5020262ns | 4951687ns | 5062834ns | +23.53% | 10.026 |
| rust_nostd | 22339356ns | 22065673ns | 22557132ns | +449.69% | 2.253 |
| rust_std | 27990593ns | 27045274ns | 28625656ns | +588.75% | 1.798 |
| rust_std_par | 26581966ns | 26303714ns | 26788194ns | +554.09% | 1.893 |

## Performance model

- Peak throughput: 12.681 Gops/s (hand; best 20% batches)
- Ops per call: 50331648

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 2.722 | 21.5% |
| engine_par | 2.565 | 20.2% |
| hand | 12.469 | 98.3% |
| hand_par | 10.059 | 79.3% |
| rust_nostd | 2.263 | 17.8% |
| rust_std | 1.815 | 14.3% |
| rust_std_par | 1.899 | 15.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 18522341ns | 18522341ns | +355.60% |
| engine_par | 19535216ns | 19535216ns | +380.51% |
| hand | 4065480ns | 4065480ns | base |
| hand_par | 5023188ns | 5023188ns | +23.56% |
| rust_nostd | 22340879ns | 22340879ns | +449.53% |
| rust_std | 27992156ns | 27992156ns | +588.53% |
| rust_std_par | 26584717ns | 26584717ns | +553.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 4036430ns | base | --- | [3970084, 4181414] | --- | --- | --- | --- |
| engine | 18492000ns | +14448248.9ns (+357.9%) | [+14373477, +14522730]ns | [18401478, 18677141] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 19618922ns | +15648080.2ns (+387.7%) | [+13762941, +15820981]ns | [17733025, 20012269] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 5003436ns | +949107.4ns (+23.5%) | [+854851, +1128444]ns | [4969189, 5098529] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 22239081ns | +18205119.5ns (+451.0%) | [+18096553, +18337410]ns | [22114012, 22523733] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 27727811ns | +23602596.3ns (+584.7%) | [+23174580, +24971005]ns | [27299085, 29162293] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 26507183ns | +22406395.7ns (+555.1%) | [+22229511, +22862320]ns | [26382888, 26906993] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4167912ns | +347.6% | +379.3% | +20.0% | +438.6% | +582.4% | +533.0% |
| 1 | 2 | 3971324ns | +363.5% | +369.1% | +27.1% | +457.2% | +592.9% | +573.8% |

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
| engine | 47321434.2ns | 18521181.7ns | 255.5% | HIGH |
| engine_par | 47637451.7ns | 19532575.9ns | 243.9% | HIGH |
| hand | 47461058.2ns | 4063967.8ns | 1167.9% | HIGH |
| hand_par | 47783140.3ns | 5020261.5ns | 951.8% | HIGH |
| rust_nostd | 47403675.9ns | 22339355.5ns | 212.2% | HIGH |
| rust_std | 47492167.8ns | 27990593.4ns | 169.7% | HIGH |
| rust_std_par | 47724627.8ns | 26581965.9ns | 179.5% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 18370335.5-18640484.4 ns)
  18370335.5 |####################
  18383842.9 |
  18397350.4 |####################
  18410857.8 |########################################
  18424365.3 |
  18437872.7 |
  18451380.2 |
  18464887.6 |
  18478395.1 |
  18491902.5 |
  18505410.0 |
  18518917.4 |
  18532424.9 |
  18545932.3 |
  18559439.7 |####################
  18572947.2 |
  18586454.6 |
  18599962.1 |
  18613469.5 |
  18626977.0 |####################
  (0 below, 2 above range)

engine_par (n=8, range 17700144.8-20812254.1 ns)
  17700144.8 |########################################
  17855750.3 |
  18011355.7 |####################
  18166961.2 |
  18322566.7 |
  18478172.1 |
  18633777.6 |
  18789383.1 |
  18944988.5 |
  19100594.0 |
  19256199.5 |
  19411804.9 |####################
  19567410.4 |####################
  19723015.9 |
  19878621.3 |########################################
  20034226.8 |
  20189832.3 |
  20345437.7 |
  20501043.2 |
  20656648.7 |
  (0 below, 1 above range)

hand (n=8, range 3969120.0-4155782.9 ns)
  3969120.0 |########################################
  3978453.1 |
  3987786.3 |
  3997119.4 |
  4006452.6 |
  4015785.7 |
  4025118.9 |
  4034452.0 |
  4043785.1 |
  4053118.3 |
  4062451.4 |
  4071784.6 |
  4081117.7 |
  4090450.9 |##########
  4099784.0 |
  4109117.2 |
  4118450.3 |
  4127783.4 |
  4137116.6 |
  4146449.7 |##########
  (0 below, 2 above range)

hand_par (n=8, range 4951687.4-5062834.4 ns)
  4951687.4 |########################################
  4957244.7 |
  4962802.1 |
  4968359.4 |########################################
  4973916.8 |
  4979474.1 |
  4985031.5 |
  4990588.8 |########################################
  4996146.2 |########################################
  5001703.5 |
  5007260.9 |########################################
  5012818.2 |
  5018375.6 |
  5023932.9 |
  5029490.3 |
  5035047.6 |
  5040605.0 |########################################
  5046162.3 |
  5051719.7 |
  5057277.0 |
  (0 below, 2 above range)

rust_nostd (n=8, range 22065673.0-22557131.7 ns)
  22065673.0 |####################
  22090245.9 |####################
  22114818.9 |
  22139391.8 |########################################
  22163964.7 |
  22188537.7 |
  22213110.6 |
  22237683.5 |
  22262256.5 |
  22286829.4 |
  22311402.4 |####################
  22335975.3 |
  22360548.2 |####################
  22385121.2 |
  22409694.1 |
  22434267.0 |
  22458840.0 |
  22483412.9 |
  22507985.8 |####################
  22532558.8 |
  (0 below, 1 above range)

rust_std (n=8, range 27045273.8-28625656.3 ns)
  27045273.8 |####################
  27124292.9 |
  27203312.1 |
  27282331.2 |########################################
  27361350.3 |
  27440369.4 |
  27519388.6 |
  27598407.7 |
  27677426.8 |########################################
  27756445.9 |
  27835465.1 |
  27914484.2 |####################
  27993503.3 |
  28072522.4 |
  28151541.6 |
  28230560.7 |
  28309579.8 |
  28388598.9 |
  28467618.1 |
  28546637.2 |
  (0 below, 2 above range)

rust_std_par (n=8, range 26303713.6-26788194.3 ns)
  26303713.6 |####################
  26327937.6 |
  26352161.7 |
  26376385.7 |########################################
  26400609.7 |
  26424833.8 |####################
  26449057.8 |
  26473281.8 |
  26497505.9 |
  26521729.9 |
  26545953.9 |
  26570178.0 |####################
  26594402.0 |####################
  26618626.1 |
  26642850.1 |
  26667074.1 |
  26691298.2 |
  26715522.2 |
  26739746.2 |
  26763970.3 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=256.0% of algo (FFI overhead may distort results)
- engine_par: bridge=243.1% of algo (FFI overhead may distort results)
- hand: bridge=1173.3% of algo (FFI overhead may distort results)
- hand_par: bridge=953.2% of algo (FFI overhead may distort results)
- rust_nostd: bridge=213.0% of algo (FFI overhead may distort results)
- rust_std: bridge=170.7% of algo (FFI overhead may distort results)
- rust_std_par: bridge=179.8% of algo (FFI overhead may distort results)
