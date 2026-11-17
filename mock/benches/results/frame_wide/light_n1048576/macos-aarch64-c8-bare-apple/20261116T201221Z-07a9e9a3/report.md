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

### hand_par dominates: 30% faster than the next best (hand)

hand_par (207.75 us) leads hand (270.26 us) by 30%, a clear separation rather than a photo finish. CV 2.7%.


### hand_par beats baseline by 23% (significant)

hand_par is -63.08 us (23%) faster than baseline hand, with a CI that excludes zero.


### engine is an outlier: 8.9x slower than the field

engine (1.85 ms) is 8.9x the fastest (207.75 us), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {hand_par, hand} vs {rust_std_par, rust_std, rust_nostd, engine_par, engine} (368% apart)

The field splits into a fast tier {hand_par, hand} and a slow tier {rust_std_par, rust_std, rust_nostd, engine_par, engine} with a 368% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 8.9x the fastest

Fastest hand_par (207.75 us) to slowest engine (1.85 ms): 8.9x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: hand_par at 207745.8 ns median (-23.1% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 8.90x (fastest 207745.8 ns, slowest 1848470.8 ns)

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
| engine | 1848495ns | 1850725ns | 1831950ns | 1843525ns | 1856358ns | +533.58% |
| engine_par | 1843783ns | 1841108ns | 1825458ns | 1830714ns | 1858167ns | +531.97% |
| hand | 291753ns | 275621ns | 268292ns | 272158ns | 312315ns | base |
| hand_par | 214254ns | 212108ns | 208867ns | 211083ns | 217979ns | -26.56% |
| rust_nostd | 1699004ns | 1701708ns | 1674317ns | 1688864ns | 1712781ns | +482.34% |
| rust_std | 1514781ns | 1480662ns | 1460600ns | 1464783ns | 1565825ns | +419.20% |
| rust_std_par | 1260910ns | 1269462ns | 1214600ns | 1242800ns | 1286071ns | +332.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 1846232ns | 1829633ns | 1854065ns | +540.74% | 6.815 |
| engine_par | 1839840ns | 1820550ns | 1854352ns | +538.52% | 6.839 |
| hand | 288141ns | 264200ns | 309017ns | base | 43.669 |
| hand_par | 210014ns | 205583ns | 213571ns | -27.11% | 59.915 |
| rust_nostd | 1696997ns | 1672725ns | 1710873ns | +488.95% | 7.415 |
| rust_std | 1513122ns | 1458450ns | 1564348ns | +425.13% | 8.316 |
| rust_std_par | 1257029ns | 1210042ns | 1282044ns | +336.26% | 10.010 |

## Performance model

- Peak throughput: 61.206 Gops/s (hand_par; best 20% batches)
- Ops per call: 12582912

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 6.807 | 11.1% |
| engine_par | 6.849 | 11.2% |
| hand | 46.559 | 76.1% |
| hand_par | 60.569 | 99.0% |
| rust_nostd | 7.402 | 12.1% |
| rust_std | 8.506 | 13.9% |
| rust_std_par | 9.940 | 16.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 1848495ns | 1848495ns | +533.58% |
| engine_par | 1843783ns | 1843783ns | +531.97% |
| hand | 291753ns | 291753ns | base |
| hand_par | 214254ns | 214254ns | -26.56% |
| rust_nostd | 1699004ns | 1699004ns | +482.34% |
| rust_std | 1514781ns | 1514781ns | +419.20% |
| rust_std_par | 1260910ns | 1260910ns | +332.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 270258ns | base | --- | [266883, 307050] | --- | --- | --- | --- |
| engine | 1848471ns | +1563529.1ns (+578.5%) | [+1544333, +1589983]ns | [1835400, 1855958] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 1837092ns | +1563837.5ns (+578.6%) | [+1516725, +1591450]ns | [1822075, 1862492] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 207746ns | -63075.1ns (-23.3%) | [-92017, -55867]ns | [205683, 215033] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 1699921ns | +1429662.5ns (+529.0%) | [+1365675, +1440450]ns | [1673283, 1707133] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 1479250ns | +1208991.6ns (+447.3%) | [+1159167, +1293917]ns | [1458683, 1586417] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 1265862ns | +980987.5ns (+363.0%) | [+910758, +1017375]ns | [1215525, 1285875] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 268988ns | +584.2% | +589.8% | -22.9% | +533.4% | +461.1% | +370.6% |
| 1 | 2 | 303167ns | +510.8% | +501.3% | -31.4% | +454.2% | +381.7% | +310.0% |

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
| engine | 16812469.8ns | 1846232.3ns | 910.6% | HIGH |
| engine_par | 16860092.7ns | 1839839.6ns | 916.4% | HIGH |
| hand | 16994387.5ns | 288140.6ns | 5897.9% | HIGH |
| hand_par | 16856007.3ns | 210013.5ns | 8026.2% | HIGH |
| rust_nostd | 16859512.5ns | 1696996.9ns | 993.5% | HIGH |
| rust_std | 16876814.6ns | 1513121.9ns | 1115.4% | HIGH |
| rust_std_par | 16832720.8ns | 1257029.2ns | 1339.1% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 1829633.3-1854064.6 ns)
  1829633.3 |########################################
  1830854.9 |
  1832076.4 |
  1833298.0 |
  1834519.6 |########################################
  1835741.1 |
  1836962.7 |
  1838184.2 |
  1839405.8 |
  1840627.4 |
  1841848.9 |########################################
  1843070.5 |
  1844292.1 |
  1845513.6 |########################################
  1846735.2 |
  1847956.8 |
  1849178.3 |
  1850399.9 |########################################
  1851621.4 |########################################
  1852843.0 |
  (0 below, 2 above range)

engine_par (n=8, range 1820550.0-1854352.1 ns)
  1820550.0 |########################################
  1822240.1 |####################
  1823930.2 |
  1825620.3 |
  1827310.4 |
  1829000.5 |
  1830690.6 |
  1832380.7 |
  1834070.8 |####################
  1835760.9 |
  1837451.1 |
  1839141.2 |####################
  1840831.3 |
  1842521.4 |
  1844211.5 |
  1845901.6 |
  1847591.7 |####################
  1849281.8 |
  1850971.9 |
  1852662.0 |
  (0 below, 2 above range)

hand (n=8, range 264200.0-309016.7 ns)
  264200.0 |####################
  266440.8 |########################################
  268681.7 |####################
  270922.5 |####################
  273163.3 |
  275404.2 |
  277645.0 |
  279885.8 |
  282126.7 |
  284367.5 |
  286608.3 |
  288849.2 |
  291090.0 |
  293330.8 |
  295571.7 |
  297812.5 |####################
  300053.3 |
  302294.2 |
  304535.0 |
  306775.8 |####################
  (0 below, 1 above range)

hand_par (n=8, range 205583.3-213570.8 ns)
  205583.3 |########################################
  205982.7 |
  206382.0 |
  206781.4 |####################
  207180.8 |
  207580.2 |########################################
  207979.5 |####################
  208378.9 |
  208778.3 |
  209177.7 |
  209577.0 |
  209976.4 |
  210375.8 |
  210775.2 |
  211174.5 |
  211573.9 |
  211973.3 |
  212372.7 |
  212772.0 |
  213171.4 |
  (0 below, 2 above range)

rust_nostd (n=8, range 1672725.0-1710872.9 ns)
  1672725.0 |########################################
  1674632.4 |
  1676539.8 |
  1678447.2 |
  1680354.6 |
  1682262.0 |
  1684169.4 |
  1686076.8 |####################
  1687984.2 |
  1689891.6 |
  1691798.9 |
  1693706.3 |
  1695613.7 |
  1697521.1 |
  1699428.5 |########################################
  1701335.9 |
  1703243.3 |####################
  1705150.7 |
  1707058.1 |####################
  1708965.5 |
  (0 below, 1 above range)

rust_std (n=8, range 1458450.0-1564347.9 ns)
  1458450.0 |########################################
  1463744.9 |#############
  1469039.8 |
  1474334.7 |
  1479629.6 |
  1484924.5 |#############
  1490219.4 |
  1495514.3 |
  1500809.2 |
  1506104.1 |
  1511399.0 |
  1516693.9 |
  1521988.8 |
  1527283.7 |#############
  1532578.5 |
  1537873.4 |
  1543168.3 |
  1548463.2 |
  1553758.1 |
  1559053.0 |
  (0 below, 2 above range)

rust_std_par (n=8, range 1210041.7-1282043.8 ns)
  1210041.7 |####################
  1213641.8 |####################
  1217241.9 |
  1220842.0 |
  1224442.1 |
  1228042.2 |
  1231642.3 |
  1235242.4 |
  1238842.5 |####################
  1242442.6 |
  1246042.7 |
  1249642.8 |
  1253242.9 |
  1256843.0 |
  1260443.1 |####################
  1264043.2 |
  1267643.3 |########################################
  1271243.4 |
  1274843.5 |
  1278443.6 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=909.4% of algo (FFI overhead may distort results)
- engine_par: bridge=916.9% of algo (FFI overhead may distort results)
- hand: bridge=6268.4% of algo (FFI overhead may distort results)
- hand_par: bridge=8114.6% of algo (FFI overhead may distort results)
- rust_nostd: bridge=991.2% of algo (FFI overhead may distort results)
- rust_std: bridge=1136.1% of algo (FFI overhead may distort results)
- rust_std_par: bridge=1329.6% of algo (FFI overhead may distort results)
