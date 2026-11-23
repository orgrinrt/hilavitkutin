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

### hand_par dominates: 35% faster than the next best (rust_std_par)

hand_par (713.06 us) leads rust_std_par (961.52 us) by 35%, a clear separation rather than a photo finish. CV 8.6%.


### hand_par beats baseline by 71% (significant)

hand_par is -1.73 ms (71%) faster than baseline hand, with a CI that excludes zero.


### engine_par is an outlier: 9.0x slower than the field

engine_par (6.40 ms) is 9.0x the fastest (713.06 us), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Wide spread: slowest is 9.0x the fastest

Fastest hand_par (713.06 us) to slowest engine_par (6.40 ms): 9.0x. The strategy choice matters a lot for this workload.


### engine_par is inconsistent: worst-20% is 4.9x its best-20%

engine_par's best 20% of batches run at 2.16 ms but its worst 20% at 10.52 ms (4.9x) - a bimodal or bursty profile the median hides.


## Key findings

- Fastest: hand_par at 713063.6 ns median (-70.7% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 8.98x (fastest 713063.6 ns, slowest 6400140.2 ns)

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
| engine | 2009074ns | 2013492ns | 1994051ns | 2002116ns | 2018047ns | -18.09% |
| engine_par | 6735851ns | 6400630ns | 2159593ns | 3213784ns | 10521465ns | +174.63% |
| hand | 2452659ns | 2431576ns | 2423781ns | 2428176ns | 2478242ns | base |
| hand_par | 729242ns | 713670ns | 643521ns | 686705ns | 782574ns | -70.27% |
| rust_nostd | 2026632ns | 2026068ns | 2024217ns | 2025183ns | 2028323ns | -17.37% |
| rust_std | 2008100ns | 2007294ns | 1983504ns | 1986762ns | 2030252ns | -18.13% |
| rust_std_par | 1145462ns | 962707ns | 672213ns | 753490ns | 1557753ns | -53.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 2008873ns | 1993885ns | 2017838ns | -18.09% | 0.391 |
| engine_par | 6735303ns | 2159085ns | 10520940ns | +174.64% | 0.117 |
| hand | 2452435ns | 2423641ns | 2477976ns | base | 0.321 |
| hand_par | 728560ns | 642913ns | 781838ns | -70.29% | 1.079 |
| rust_nostd | 2026438ns | 2024030ns | 2028128ns | -17.37% | 0.388 |
| rust_std | 2007898ns | 1983330ns | 2030053ns | -18.13% | 0.392 |
| rust_std_par | 1144346ns | 671464ns | 1556277ns | -53.34% | 0.687 |

## Performance model

- Peak throughput: 1.223 Gops/s (hand_par; best 20% batches)
- Ops per call: 786432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.391 | 31.9% |
| engine_par | 0.123 | 10.0% |
| hand | 0.323 | 26.4% |
| hand_par | 1.103 | 90.2% |
| rust_nostd | 0.388 | 31.7% |
| rust_std | 0.392 | 32.0% |
| rust_std_par | 0.818 | 66.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 2009074ns | 2009074ns | -18.09% |
| engine_par | 6735851ns | 6735851ns | +174.63% |
| hand | 2452659ns | 2452659ns | base |
| hand_par | 729242ns | 729242ns | -70.27% |
| rust_nostd | 2026632ns | 2026632ns | -17.37% |
| rust_std | 2008100ns | 2008100ns | -18.13% |
| rust_std_par | 1145462ns | 1145462ns | -53.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 2431337ns | base | --- | [2426879, 2434929] | --- | --- | --- | --- |
| engine | 2013261ns | -424488.8ns (-17.5%) | [-439121, -416073]ns | [1996743, 2016906] | YES | 0.0094 | 0.0078 | 0 |
| engine_par | 6400140ns | no significant difference | [-251783, +7882387]ns | [2175578, 10315366] | no | 0.2891 | 0.2891 | 0 |
| hand_par | 713064ns | -1734471.1ns (-71.3%) | [-1790093, -1629689]ns | [668589, 800006] | YES | 0.0094 | 0.0078 | 0 |
| rust_nostd | 2025862ns | -405422.3ns (-16.7%) | [-410560, -400256]ns | [2024369, 2028208] | YES | 0.0094 | 0.0078 | 0 |
| rust_std | 2007134ns | -419021.8ns (-17.2%) | [-448246, -399010]ns | [1986468, 2032115] | YES | 0.0094 | 0.0078 | 0 |
| rust_std_par | 961521ns | -1471471.6ns (-60.5%) | [-1758232, -814048]ns | [682478, 1874620] | YES | 0.0094 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2428528ns | -17.8% | -0.4% | -71.3% | -16.6% | -16.4% | -53.3% |
| 1 | 2 | 2433954ns | -17.2% | +322.8% | -69.7% | -16.8% | -18.4% | -67.6% |

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
- engine_par: won 1/2, lost 1/2
- hand_par: won 2/2, lost 0/2
- rust_nostd: won 2/2, lost 0/2
- rust_std: won 2/2, lost 0/2
- rust_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 727502.2ns | 2008873.5ns | 36.2% | HIGH |
| engine_par | 753747.5ns | 6735303.0ns | 11.2% | HIGH |
| hand | 731447.5ns | 2452435.3ns | 29.8% | HIGH |
| hand_par | 1025478.4ns | 728560.2ns | 140.8% | HIGH |
| rust_nostd | 725512.1ns | 2026437.7ns | 35.8% | HIGH |
| rust_std | 725477.6ns | 2007897.7ns | 36.1% | HIGH |
| rust_std_par | 1502149.2ns | 1144346.5ns | 131.3% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 1993885.1-2017838.2 ns)
  1993885.1 |########################################
  1995082.8 |
  1996280.4 |########################################
  1997478.1 |########################################
  1998675.7 |
  1999873.4 |
  2001071.0 |
  2002268.7 |
  2003466.3 |
  2004664.0 |
  2005861.6 |
  2007059.3 |
  2008256.9 |
  2009454.6 |
  2010652.3 |########################################
  2011849.9 |
  2013047.6 |
  2014245.2 |########################################
  2015442.9 |########################################
  2016640.5 |########################################
  (0 below, 1 above range)

engine_par (n=8, range 2159085.2-10520939.8 ns)
  2159085.2 |########################################
  2577177.9 |####################
  2995270.7 |
  3413363.4 |
  3831456.1 |
  4249548.8 |
  4667641.6 |####################
  5085734.3 |
  5503827.0 |
  5921919.8 |
  6340012.5 |
  6758105.2 |
  7176197.9 |
  7594290.7 |####################
  8012383.4 |
  8430476.1 |
  8848568.9 |
  9266661.6 |
  9684754.3 |
  10102847.0 |########################################
  (0 below, 1 above range)

hand (n=8, range 2423641.3-2477976.4 ns)
  2423641.3 |####################
  2426358.1 |########################################
  2429074.8 |####################
  2431791.6 |########################################
  2434508.3 |####################
  2437225.1 |
  2439941.8 |
  2442658.6 |
  2445375.3 |
  2448092.1 |
  2450808.9 |
  2453525.6 |
  2456242.4 |
  2458959.1 |
  2461675.9 |
  2464392.6 |
  2467109.4 |
  2469826.2 |
  2472542.9 |
  2475259.7 |
  (0 below, 1 above range)

hand_par (n=8, range 642913.1-781838.1 ns)
  642913.1 |########################################
  649859.3 |
  656805.6 |
  663751.8 |########################################
  670698.1 |
  677644.3 |
  684590.6 |########################################
  691536.8 |
  698483.1 |########################################
  705429.3 |
  712375.6 |
  719321.8 |########################################
  726268.1 |
  733214.3 |
  740160.6 |
  747106.8 |
  754053.1 |
  760999.3 |
  767945.6 |
  774891.8 |########################################
  (0 below, 2 above range)

rust_nostd (n=8, range 2024029.5-2028127.9 ns)
  2024029.5 |########################################
  2024234.4 |########################################
  2024439.3 |
  2024644.3 |
  2024849.2 |
  2025054.1 |########################################
  2025259.0 |
  2025464.0 |########################################
  2025668.9 |
  2025873.8 |
  2026078.7 |########################################
  2026283.6 |
  2026488.6 |########################################
  2026693.5 |
  2026898.4 |
  2027103.3 |
  2027308.3 |
  2027513.2 |
  2027718.1 |
  2027923.0 |
  (0 below, 2 above range)

rust_std (n=8, range 1983329.7-2030053.1 ns)
  1983329.7 |#############
  1985665.9 |########################################
  1988002.0 |
  1990338.2 |
  1992674.4 |
  1995010.6 |
  1997346.7 |
  1999682.9 |
  2002019.1 |
  2004355.2 |
  2006691.4 |
  2009027.6 |
  2011363.8 |
  2013699.9 |
  2016036.1 |
  2018372.3 |
  2020708.4 |
  2023044.6 |
  2025380.8 |#############
  2027717.0 |#############
  (0 below, 2 above range)

rust_std_par (n=8, range 671463.6-1556277.0 ns)
  671463.6 |########################################
  715704.3 |
  759944.9 |########################################
  804185.6 |
  848426.3 |
  892666.9 |
  936907.6 |
  981148.3 |
  1025389.0 |
  1069629.6 |
  1113870.3 |########################################
  1158111.0 |
  1202351.6 |
  1246592.3 |
  1290833.0 |
  1335073.6 |
  1379314.3 |
  1423555.0 |
  1467795.7 |
  1512036.3 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=36.0% of algo (FFI overhead may distort results)
- engine_par: CV=60.9% (high variance, measurements may be unstable)
- engine_par: worst_20/best_20 = 4.9x (possible bimodal distribution)
- engine_par: bridge=11.5% of algo (FFI overhead may distort results)
- hand: bridge=29.8% of algo (FFI overhead may distort results)
- hand_par: bridge=150.9% of algo (FFI overhead may distort results)
- rust_nostd: bridge=35.8% of algo (FFI overhead may distort results)
- rust_std: bridge=36.1% of algo (FFI overhead may distort results)
- rust_std_par: CV=44.9% (high variance, measurements may be unstable)
- rust_std_par: bridge=122.1% of algo (FFI overhead may distort results)
