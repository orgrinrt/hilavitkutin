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

### rust_std_par dominates: 139% faster than the next best (hand_par)

rust_std_par (727.95 us) leads hand_par (1.74 ms) by 139%, a clear separation rather than a photo finish. CV 64.1%.


### rust_std_par beats baseline by 70% (significant)

rust_std_par is -1.70 ms (70%) faster than baseline hand, with a CI that excludes zero.


### engine_par is an outlier: 7.0x slower than the field

engine_par (5.10 ms) is 7.0x the fastest (727.95 us), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Wide spread: slowest is 7.0x the fastest

Fastest rust_std_par (727.95 us) to slowest engine_par (5.10 ms): 7.0x. The strategy choice matters a lot for this workload.


### engine_par is inconsistent: worst-20% is 4.9x its best-20%

engine_par's best 20% of batches run at 2.00 ms but its worst 20% at 9.70 ms (4.9x) - a bimodal or bursty profile the median hides.


## Key findings

- Fastest: rust_std_par at 727945.8 ns median (-70.1% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 7.01x (fastest 727945.8 ns, slowest 5101352.8 ns)

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
| engine | 2005425ns | 2006070ns | 1995627ns | 1997789ns | 2013602ns | -17.53% |
| engine_par | 6385185ns | 5101905ns | 1996931ns | 3433824ns | 9695769ns | +162.59% |
| hand | 2431616ns | 2431243ns | 2427812ns | 2429837ns | 2433902ns | base |
| hand_par | 1616452ns | 1740874ns | 898719ns | 1516040ns | 1871195ns | -33.52% |
| rust_nostd | 2007179ns | 2007013ns | 1983242ns | 1985209ns | 2029640ns | -17.45% |
| rust_std | 2026106ns | 2029959ns | 1989164ns | 2029658ns | 2032677ns | -16.68% |
| rust_std_par | 981053ns | 728618ns | 647220ns | 693308ns | 1280320ns | -59.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 2005203ns | 1995448ns | 2013381ns | -17.53% | 0.392 |
| engine_par | 6384561ns | 1996444ns | 9695084ns | +162.59% | 0.123 |
| hand | 2431399ns | 2427627ns | 2433632ns | base | 0.323 |
| hand_par | 1614957ns | 898106ns | 1869518ns | -33.58% | 0.487 |
| rust_nostd | 2006983ns | 1983083ns | 2029441ns | -17.46% | 0.392 |
| rust_std | 2025925ns | 1989016ns | 2032479ns | -16.68% | 0.388 |
| rust_std_par | 980051ns | 646673ns | 1279127ns | -59.69% | 0.802 |

## Performance model

- Peak throughput: 1.216 Gops/s (rust_std_par; best 20% batches)
- Ops per call: 786432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.392 | 32.2% |
| engine_par | 0.154 | 12.7% |
| hand | 0.323 | 26.6% |
| hand_par | 0.452 | 37.2% |
| rust_nostd | 0.392 | 32.2% |
| rust_std | 0.387 | 31.9% |
| rust_std_par | 1.080 | 88.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 2005425ns | 2005425ns | -17.53% |
| engine_par | 6385185ns | 6385185ns | +162.59% |
| hand | 2431616ns | 2431616ns | base |
| hand_par | 1616452ns | 1616452ns | -33.52% |
| rust_nostd | 2007179ns | 2007179ns | -17.45% |
| rust_std | 2026106ns | 2026106ns | -16.68% |
| rust_std_par | 981053ns | 981053ns | -59.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 2431042ns | base | --- | [2427791, 2433282] | --- | --- | --- | --- |
| engine | 2005806ns | -426626.4ns (-17.5%) | [-437242, -416516]ns | [1995809, 2014552] | YES | 0.0094 | 0.0078 | 0 |
| engine_par | 5101353ns | no significant difference | [-397362, +8762672]ns | [2033811, 11195954] | no | 0.2891 | 0.2891 | 0 |
| hand_par | 1739222ns | -691820.4ns (-28.5%) | [-1349296, -519074]ns | [1083986, 1908553] | YES | 0.0094 | 0.0078 | 0 |
| rust_nostd | 2006803ns | -424311.8ns (-17.5%) | [-448371, -402521]ns | [1983665, 2030762] | YES | 0.0094 | 0.0078 | 0 |
| rust_std | 2029748ns | -401539.2ns (-16.5%) | [-406998, -398210]ns | [2029331, 2031785] | YES | 0.0094 | 0.0078 | 0 |
| rust_std_par | 727946ns | -1704151.5ns (-70.1%) | [-1759441, -1118256]ns | [670888, 1754394] | YES | 0.0094 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2431745ns | -17.9% | +109.8% | -27.1% | -18.4% | -16.5% | -69.2% |
| 1 | 2 | 2431042ns | -17.2% | +126.3% | -28.5% | -16.5% | -16.5% | -70.1% |

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
- engine_par: won 0/2, lost 2/2
- hand_par: won 2/2, lost 0/2
- rust_nostd: won 2/2, lost 0/2
- rust_std: won 2/2, lost 0/2
- rust_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 725837.9ns | 2005202.9ns | 36.2% | HIGH |
| engine_par | 859054.1ns | 6384560.8ns | 13.5% | HIGH |
| hand | 723503.9ns | 2431398.6ns | 29.8% | HIGH |
| hand_par | 2194415.1ns | 1614957.1ns | 135.9% | HIGH |
| rust_nostd | 727666.5ns | 2006983.4ns | 36.3% | HIGH |
| rust_std | 725382.0ns | 2025925.1ns | 35.8% | HIGH |
| rust_std_par | 1295192.1ns | 980051.5ns | 132.2% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 1995447.5-2013380.6 ns)
  1995447.5 |########################################
  1996344.2 |####################
  1997240.8 |
  1998137.5 |
  1999034.1 |####################
  1999930.8 |
  2000827.4 |
  2001724.1 |
  2002620.7 |
  2003517.4 |
  2004414.0 |
  2005310.7 |
  2006207.3 |
  2007104.0 |
  2008000.7 |
  2008897.3 |
  2009794.0 |
  2010690.6 |
  2011587.3 |########################################
  2012483.9 |
  (0 below, 2 above range)

engine_par (n=8, range 1996444.3-9695084.1 ns)
  1996444.3 |########################################
  2381376.3 |
  2766308.3 |
  3151240.3 |####################
  3536172.2 |
  3921104.2 |
  4306036.2 |
  4690968.2 |####################
  5075900.2 |####################
  5460832.2 |
  5845764.2 |
  6230696.2 |
  6615628.2 |
  7000560.1 |
  7385492.1 |
  7770424.1 |
  8155356.1 |
  8540288.1 |
  8925220.1 |####################
  9310152.1 |
  (0 below, 2 above range)

hand (n=8, range 2427626.8-2433632.4 ns)
  2427626.8 |########################################
  2427927.1 |
  2428227.4 |
  2428527.6 |
  2428827.9 |
  2429128.2 |
  2429428.5 |
  2429728.8 |
  2430029.0 |####################
  2430329.3 |
  2430629.6 |####################
  2430929.9 |####################
  2431230.2 |
  2431530.4 |
  2431830.7 |
  2432131.0 |
  2432431.3 |
  2432731.6 |
  2433031.8 |########################################
  2433332.1 |
  (0 below, 1 above range)

hand_par (n=8, range 898106.0-1869517.9 ns)
  898106.0 |####################
  946676.6 |
  995247.2 |
  1043817.8 |####################
  1092388.4 |
  1140959.0 |
  1189529.6 |
  1238100.2 |
  1286670.8 |
  1335241.4 |
  1383811.9 |
  1432382.5 |
  1480953.1 |
  1529523.7 |
  1578094.3 |
  1626664.9 |
  1675235.5 |####################
  1723806.1 |########################################
  1772376.7 |####################
  1820947.3 |
  (0 below, 2 above range)

rust_nostd (n=8, range 1983082.9-2029441.4 ns)
  1983082.9 |########################################
  1985400.8 |#############
  1987718.7 |
  1990036.7 |
  1992354.6 |
  1994672.5 |
  1996990.4 |
  1999308.4 |
  2001626.3 |
  2003944.2 |
  2006262.1 |
  2008580.1 |
  2010898.0 |
  2013215.9 |
  2015533.8 |
  2017851.8 |
  2020169.7 |
  2022487.6 |
  2024805.5 |#############
  2027123.5 |#############
  (0 below, 2 above range)

rust_std (n=8, range 1989016.0-2032478.6 ns)
  1989016.0 |########
  1991189.1 |
  1993362.3 |
  1995535.4 |
  1997708.5 |
  1999881.7 |
  2002054.8 |
  2004227.9 |
  2006401.1 |
  2008574.2 |
  2010747.3 |
  2012920.5 |
  2015093.6 |
  2017266.7 |
  2019439.9 |
  2021613.0 |
  2023786.1 |
  2025959.3 |
  2028132.4 |########################################
  2030305.5 |########
  (0 below, 1 above range)

rust_std_par (n=8, range 646673.2-1279127.5 ns)
  646673.2 |########################################
  678295.9 |####################
  709918.6 |########################################
  741541.3 |
  773164.1 |
  804786.8 |####################
  836409.5 |
  868032.2 |
  899654.9 |
  931277.6 |
  962900.3 |
  994523.1 |
  1026145.8 |
  1057768.5 |
  1089391.2 |
  1121013.9 |
  1152636.6 |
  1184259.3 |
  1215882.0 |
  1247504.8 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=36.2% of algo (FFI overhead may distort results)
- engine_par: CV=63.3% (high variance, measurements may be unstable)
- engine_par: worst_20/best_20 = 4.9x (possible bimodal distribution)
- engine_par: bridge=15.7% of algo (FFI overhead may distort results)
- hand: bridge=29.8% of algo (FFI overhead may distort results)
- hand_par: CV=23.2% (high variance, measurements may be unstable)
- hand_par: bridge=139.8% of algo (FFI overhead may distort results)
- rust_nostd: bridge=36.2% of algo (FFI overhead may distort results)
- rust_std: bridge=35.7% of algo (FFI overhead may distort results)
- rust_std_par: CV=47.6% (high variance, measurements may be unstable)
- rust_std_par: bridge=138.4% of algo (FFI overhead may distort results)
