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

### hand_par dominates: 12% faster than the next best (hand)

hand_par (241.71 us) leads hand (271.49 us) by 12%, a clear separation rather than a photo finish. CV 5.5%.


### engine_par is an outlier: 7.8x slower than the field

engine_par (1.88 ms) is 7.8x the fastest (241.71 us), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {hand_par, hand} vs {rust_std_par, rust_std, rust_nostd, engine, engine_par} (361% apart)

The field splits into a fast tier {hand_par, hand} and a slow tier {rust_std_par, rust_std, rust_nostd, engine, engine_par} with a 361% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 7.8x the fastest

Fastest hand_par (241.71 us) to slowest engine_par (1.88 ms): 7.8x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: hand_par at 241708.4 ns median (-11.0% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 7.77x (fastest 241708.4 ns, slowest 1877662.5 ns)

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
| engine | 1865627ns | 1860358ns | 1834017ns | 1854622ns | 1881783ns | +582.39% |
| engine_par | 1926331ns | 1880654ns | 1827117ns | 1871039ns | 1992604ns | +604.59% |
| hand | 273398ns | 272979ns | 264867ns | 270786ns | 277490ns | base |
| hand_par | 245616ns | 244346ns | 220317ns | 239228ns | 256731ns | -10.16% |
| rust_nostd | 1702971ns | 1710062ns | 1683525ns | 1694592ns | 1714117ns | +522.89% |
| rust_std | 1478238ns | 1478179ns | 1469117ns | 1472136ns | 1485094ns | +440.69% |
| rust_std_par | 1240645ns | 1252971ns | 1140492ns | 1220264ns | 1280969ns | +353.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 1863726ns | 1831500ns | 1880100ns | +586.29% | 6.751 |
| engine_par | 1923105ns | 1821658ns | 1989773ns | +608.16% | 6.543 |
| hand | 271564ns | 262167ns | 275656ns | base | 46.335 |
| hand_par | 242933ns | 218258ns | 253919ns | -10.54% | 51.796 |
| rust_nostd | 1701314ns | 1682058ns | 1712502ns | +526.49% | 7.396 |
| rust_std | 1476462ns | 1466967ns | 1483300ns | +443.69% | 8.522 |
| rust_std_par | 1237917ns | 1137983ns | 1278248ns | +355.85% | 10.165 |

## Performance model

- Peak throughput: 57.651 Gops/s (hand_par; best 20% batches)
- Ops per call: 12582912

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 6.771 | 11.7% |
| engine_par | 6.701 | 11.6% |
| hand | 46.348 | 80.4% |
| hand_par | 52.058 | 90.3% |
| rust_nostd | 7.367 | 12.8% |
| rust_std | 8.524 | 14.8% |
| rust_std_par | 10.063 | 17.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 1865627ns | 1865627ns | +582.39% |
| engine_par | 1926331ns | 1926331ns | +604.59% |
| hand | 273398ns | 273398ns | base |
| hand_par | 245616ns | 245616ns | -10.16% |
| rust_nostd | 1702971ns | 1702971ns | +522.89% |
| rust_std | 1478238ns | 1478238ns | +440.69% |
| rust_std_par | 1240645ns | 1240645ns | +353.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 271488ns | base | --- | [266883, 276292] | --- | --- | --- | --- |
| engine | 1858392ns | +1591095.9ns (+586.1%) | [+1571917, +1604683]ns | [1847675, 1874967] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 1877662ns | +1605895.9ns (+591.5%) | [+1590575, +1629858]ns | [1852742, 1906150] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 241708ns | -26874.9ns (-9.9%) | [-43108, -10408]ns | [232100, 257492] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 1708112ns | +1434825.0ns (+528.5%) | [+1409658, +1448558]ns | [1684475, 1714767] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 1476258ns | +1204900.0ns (+443.8%) | [+1196775, +1211592]ns | [1467450, 1485925] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 1250433ns | +976004.2ns (+359.5%) | [+921508, +1013738]ns | [1183675, 1289550] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 271354ns | +586.9% | +591.5% | -13.1% | +530.6% | +442.5% | +368.7% |
| 1 | 2 | 273421ns | +578.8% | +590.1% | -6.4% | +520.8% | +441.2% | +339.9% |

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
| engine | 16972458.3ns | 1863726.1ns | 910.7% | HIGH |
| engine_par | 16987334.4ns | 1923105.2ns | 883.3% | HIGH |
| hand | 16949268.8ns | 271563.5ns | 6241.4% | HIGH |
| hand_par | 16906845.8ns | 242933.3ns | 6959.5% | HIGH |
| rust_nostd | 16891455.2ns | 1701313.5ns | 992.8% | HIGH |
| rust_std | 16882128.1ns | 1476462.5ns | 1143.4% | HIGH |
| rust_std_par | 17000286.5ns | 1237916.6ns | 1373.3% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 1831500.0-1880100.0 ns)
  1831500.0 |########################################
  1833930.0 |
  1836360.0 |
  1838790.0 |
  1841220.0 |
  1843650.0 |
  1846080.0 |########################################
  1848510.0 |
  1850940.0 |########################################
  1853370.0 |
  1855800.0 |########################################
  1858230.0 |########################################
  1860660.0 |
  1863090.0 |
  1865520.0 |
  1867950.0 |########################################
  1870380.0 |
  1872810.0 |########################################
  1875240.0 |
  1877670.0 |
  (0 below, 1 above range)

engine_par (n=8, range 1821658.3-1989772.9 ns)
  1821658.3 |#############
  1830064.0 |
  1838469.8 |
  1846875.5 |#############
  1855281.2 |
  1863687.0 |
  1872092.7 |########################################
  1880498.4 |
  1888904.2 |#############
  1897309.9 |
  1905715.6 |#############
  1914121.3 |
  1922527.1 |
  1930932.8 |
  1939338.5 |
  1947744.3 |
  1956150.0 |
  1964555.7 |
  1972961.5 |
  1981367.2 |
  (0 below, 1 above range)

hand (n=8, range 262166.7-275656.2 ns)
  262166.7 |####################
  262841.2 |
  263515.7 |
  264190.1 |
  264864.6 |
  265539.1 |
  266213.6 |####################
  266888.0 |
  267562.5 |
  268237.0 |
  268911.5 |
  269586.0 |
  270260.4 |########################################
  270934.9 |
  271609.4 |
  272283.9 |####################
  272958.3 |####################
  273632.8 |
  274307.3 |
  274981.8 |
  (0 below, 2 above range)

hand_par (n=8, range 218258.3-253918.8 ns)
  218258.3 |########################################
  220041.3 |
  221824.3 |
  223607.4 |
  225390.4 |
  227173.4 |
  228956.4 |
  230739.5 |########################################
  232522.5 |
  234305.5 |
  236088.5 |########################################
  237871.6 |########################################
  239654.6 |
  241437.6 |
  243220.6 |########################################
  245003.7 |
  246786.7 |
  248569.7 |
  250352.7 |
  252135.8 |
  (0 below, 3 above range)

rust_nostd (n=8, range 1682058.3-1712502.1 ns)
  1682058.3 |########################################
  1683580.5 |########################################
  1685102.7 |########################################
  1686624.9 |
  1688147.1 |
  1689669.2 |
  1691191.4 |
  1692713.6 |
  1694235.8 |
  1695758.0 |
  1697280.2 |
  1698802.4 |
  1700324.6 |
  1701846.8 |
  1703369.0 |
  1704891.2 |
  1706413.3 |########################################
  1707935.5 |########################################
  1709457.7 |
  1710979.9 |########################################
  (0 below, 2 above range)

rust_std (n=8, range 1466966.7-1483300.0 ns)
  1466966.7 |########################################
  1467783.4 |
  1468600.0 |####################
  1469416.7 |
  1470233.4 |
  1471050.0 |
  1471866.7 |
  1472683.4 |
  1473500.0 |
  1474316.7 |####################
  1475133.4 |
  1475950.0 |
  1476766.7 |
  1477583.3 |####################
  1478400.0 |
  1479216.7 |
  1480033.3 |
  1480850.0 |
  1481666.7 |####################
  1482483.3 |
  (0 below, 2 above range)

rust_std_par (n=8, range 1137983.3-1278247.9 ns)
  1137983.3 |########################################
  1144996.5 |
  1152009.8 |
  1159023.0 |
  1166036.2 |
  1173049.4 |
  1180062.7 |########################################
  1187075.9 |
  1194089.1 |
  1201102.4 |
  1208115.6 |
  1215128.8 |########################################
  1222142.1 |
  1229155.3 |
  1236168.5 |
  1243181.8 |########################################
  1250195.0 |########################################
  1257208.2 |########################################
  1264221.4 |
  1271234.7 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=913.1% of algo (FFI overhead may distort results)
- engine_par: bridge=899.6% of algo (FFI overhead may distort results)
- hand: bridge=6244.1% of algo (FFI overhead may distort results)
- hand_par: bridge=6994.9% of algo (FFI overhead may distort results)
- rust_nostd: bridge=988.2% of algo (FFI overhead may distort results)
- rust_std: bridge=1143.2% of algo (FFI overhead may distort results)
- rust_std_par: bridge=1352.5% of algo (FFI overhead may distort results)
