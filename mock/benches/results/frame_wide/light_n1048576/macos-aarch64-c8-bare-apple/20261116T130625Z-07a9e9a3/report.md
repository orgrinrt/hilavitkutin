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

### hand_par dominates: 26% faster than the next best (hand)

hand_par (209.05 us) leads hand (262.52 us) by 26%, a clear separation rather than a photo finish. CV 13.4%.


### hand_par beats baseline by 20% (significant)

hand_par is -53.46 us (20%) faster than baseline hand, with a CI that excludes zero.


### engine_par is an outlier: 8.9x slower than the field

engine_par (1.85 ms) is 8.9x the fastest (209.05 us), well off the pack.


### hand_par is fastest but the noisiest (CV 13.4%)

hand_par wins on median (209.05 us) yet has the highest variance (CV 13.4%), while hand is the steadiest (CV 0.5%, 262.52 us).


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {hand_par, hand} vs {rust_std_par, rust_std, rust_nostd, engine, engine_par} (374% apart)

The field splits into a fast tier {hand_par, hand} and a slow tier {rust_std_par, rust_std, rust_nostd, engine, engine_par} with a 374% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 8.9x the fastest

Fastest hand_par (209.05 us) to slowest engine_par (1.85 ms): 8.9x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: hand_par at 209054.1 ns median (-20.4% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 8.86x (fastest 209054.1 ns, slowest 1851654.2 ns)

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
| engine | 1850299ns | 1837933ns | 1828317ns | 1834278ns | 1867810ns | +588.88% |
| engine_par | 1848928ns | 1856158ns | 1821708ns | 1844378ns | 1859146ns | +588.37% |
| hand | 268596ns | 267696ns | 266208ns | 267014ns | 270379ns | base |
| hand_par | 222169ns | 213375ns | 204258ns | 209786ns | 235933ns | -17.29% |
| rust_nostd | 1710081ns | 1697342ns | 1685717ns | 1688992ns | 1731990ns | +536.67% |
| rust_std | 1471003ns | 1467892ns | 1454558ns | 1463431ns | 1480794ns | +447.66% |
| rust_std_par | 1241656ns | 1247838ns | 1201825ns | 1234361ns | 1257085ns | +362.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 1848089ns | 1825742ns | 1865704ns | +603.03% | 6.809 |
| engine_par | 1844926ns | 1818425ns | 1855333ns | +601.82% | 6.820 |
| hand | 262876ns | 261208ns | 263912ns | base | 47.866 |
| hand_par | 218116ns | 200458ns | 231942ns | -17.03% | 57.689 |
| rust_nostd | 1707901ns | 1683192ns | 1729904ns | +549.70% | 7.367 |
| rust_std | 1469233ns | 1453100ns | 1478908ns | +458.91% | 8.564 |
| rust_std_par | 1238222ns | 1197408ns | 1253356ns | +371.03% | 10.162 |

## Performance model

- Peak throughput: 62.771 Gops/s (hand_par; best 20% batches)
- Ops per call: 12582912

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 6.855 | 10.9% |
| engine_par | 6.795 | 10.8% |
| hand | 47.932 | 76.4% |
| hand_par | 60.190 | 95.9% |
| rust_nostd | 7.423 | 11.8% |
| rust_std | 8.583 | 13.7% |
| rust_std_par | 10.105 | 16.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 1850299ns | 1850299ns | +588.88% |
| engine_par | 1848928ns | 1848928ns | +588.37% |
| hand | 268596ns | 268596ns | base |
| hand_par | 222169ns | 222169ns | -17.29% |
| rust_nostd | 1710081ns | 1710081ns | +536.67% |
| rust_std | 1471003ns | 1471003ns | +447.66% |
| rust_std_par | 1241656ns | 1241656ns | +362.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 262517ns | base | --- | [261667, 264383] | --- | --- | --- | --- |
| engine | 1835642ns | +1572679.1ns (+599.1%) | [+1567058, +1583975]ns | [1828725, 1848125] | YES | 0.0094 | 0.0078 | 0 |
| engine_par | 1851654ns | +1588245.9ns (+605.0%) | [+1567208, +1593575]ns | [1831358, 1856242] | YES | 0.0094 | 0.0078 | 0 |
| hand_par | 209054ns | -53462.6ns (-20.4%) | [-59258, -46550]ns | [201950, 215567] | YES (adj: no) | 0.0703 | 0.0703 | 0 |
| rust_nostd | 1695225ns | +1431700.0ns (+545.4%) | [+1422971, +1446492]ns | [1685625, 1708858] | YES | 0.0094 | 0.0078 | 0 |
| rust_std | 1466004ns | +1203933.4ns (+458.6%) | [+1196417, +1216433]ns | [1459083, 1480817] | YES | 0.0094 | 0.0078 | 0 |
| rust_std_par | 1245183ns | +981625.0ns (+373.9%) | [+962042, +991392]ns | [1224408, 1254050] | YES | 0.0094 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 263258ns | +597.9% | +596.8% | -18.9% | +548.2% | +460.1% | +370.7% |
| 1 | 2 | 262167ns | +599.1% | +607.8% | -22.2% | +543.1% | +456.9% | +375.0% |

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
| engine | 16837319.8ns | 1848088.5ns | 911.1% | HIGH |
| engine_par | 16851855.2ns | 1844926.0ns | 913.4% | HIGH |
| hand | 16882740.6ns | 262876.0ns | 6422.3% | HIGH |
| hand_par | 16984567.7ns | 218115.6ns | 7787.0% | HIGH |
| rust_nostd | 16816991.7ns | 1707901.0ns | 984.7% | HIGH |
| rust_std | 16814983.3ns | 1469233.3ns | 1144.5% | HIGH |
| rust_std_par | 16832692.7ns | 1238221.9ns | 1359.4% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 1825741.7-1865704.2 ns)
  1825741.7 |########################################
  1827739.8 |########################################
  1829737.9 |
  1831736.1 |########################################
  1833734.2 |########################################
  1835732.3 |########################################
  1837730.4 |
  1839728.6 |########################################
  1841726.7 |
  1843724.8 |
  1845722.9 |
  1847721.1 |########################################
  1849719.2 |
  1851717.3 |
  1853715.4 |
  1855713.6 |
  1857711.7 |
  1859709.8 |
  1861707.9 |
  1863706.1 |
  (0 below, 1 above range)

engine_par (n=8, range 1818425.0-1855333.4 ns)
  1818425.0 |########################################
  1820270.4 |
  1822115.8 |
  1823961.3 |
  1825806.7 |
  1827652.1 |
  1829497.5 |
  1831342.9 |########################################
  1833188.3 |
  1835033.8 |
  1836879.2 |########################################
  1838724.6 |
  1840570.0 |
  1842415.4 |
  1844260.8 |
  1846106.3 |
  1847951.7 |
  1849797.1 |########################################
  1851642.5 |########################################
  1853487.9 |########################################
  (0 below, 2 above range)

hand (n=8, range 261208.3-263912.5 ns)
  261208.3 |########################################
  261343.5 |
  261478.7 |
  261613.9 |########################################
  261749.1 |
  261884.3 |
  262019.6 |########################################
  262154.8 |
  262290.0 |########################################
  262425.2 |
  262560.4 |########################################
  262695.6 |
  262830.8 |
  262966.0 |
  263101.2 |
  263236.5 |
  263371.7 |
  263506.9 |
  263642.1 |
  263777.3 |
  (0 below, 3 above range)

hand_par (n=8, range 200458.3-231941.7 ns)
  200458.3 |########################################
  202032.5 |
  203606.6 |
  205180.8 |####################
  206755.0 |
  208329.1 |########################################
  209903.3 |####################
  211477.5 |
  213051.6 |
  214625.8 |####################
  216200.0 |
  217774.2 |
  219348.3 |
  220922.5 |
  222496.7 |
  224070.8 |
  225645.0 |
  227219.2 |
  228793.3 |
  230367.5 |
  (0 below, 1 above range)

rust_nostd (n=8, range 1683191.7-1729904.2 ns)
  1683191.7 |####################
  1685527.3 |########################################
  1687862.9 |####################
  1690198.6 |
  1692534.2 |
  1694869.8 |
  1697205.4 |
  1699541.1 |####################
  1701876.7 |####################
  1704212.3 |
  1706547.9 |####################
  1708883.5 |
  1711219.2 |
  1713554.8 |
  1715890.4 |
  1718226.0 |
  1720561.7 |
  1722897.3 |
  1725232.9 |
  1727568.5 |
  (0 below, 1 above range)

rust_std (n=8, range 1453100.0-1478908.3 ns)
  1453100.0 |########################################
  1454390.4 |
  1455680.8 |
  1456971.2 |
  1458261.7 |########################################
  1459552.1 |
  1460842.5 |########################################
  1462132.9 |
  1463423.3 |
  1464713.7 |########################################
  1466004.2 |########################################
  1467294.6 |########################################
  1468585.0 |
  1469875.4 |
  1471165.8 |
  1472456.2 |
  1473746.7 |
  1475037.1 |
  1476327.5 |
  1477617.9 |
  (0 below, 2 above range)

rust_std_par (n=8, range 1197408.3-1253356.2 ns)
  1197408.3 |####################
  1200205.7 |
  1203003.1 |
  1205800.5 |
  1208597.9 |
  1211395.3 |
  1214192.7 |
  1216990.1 |
  1219787.5 |
  1222584.9 |####################
  1225382.3 |
  1228179.7 |
  1230977.1 |####################
  1233774.5 |
  1236571.9 |####################
  1239369.3 |
  1242166.7 |
  1244964.1 |
  1247761.5 |
  1250558.9 |########################################
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=916.2% of algo (FFI overhead may distort results)
- engine_par: bridge=909.8% of algo (FFI overhead may distort results)
- hand: bridge=6429.2% of algo (FFI overhead may distort results)
- hand_par: bridge=8053.0% of algo (FFI overhead may distort results)
- rust_nostd: bridge=991.3% of algo (FFI overhead may distort results)
- rust_std: bridge=1146.2% of algo (FFI overhead may distort results)
- rust_std_par: bridge=1351.8% of algo (FFI overhead may distort results)
