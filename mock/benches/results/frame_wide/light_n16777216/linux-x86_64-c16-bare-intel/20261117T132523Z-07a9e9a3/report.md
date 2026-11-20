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

hand (16.04 ms) leads hand_par (19.89 ms) by 24%, a clear separation rather than a photo finish. CV 0.4%.


### rust_std is an outlier: 9.0x slower than the field

rust_std (144.17 ms) is 9.0x the fastest (16.04 ms), well off the pack.


### hand shows alternating (throttle bounce) (autocorr -0.50)

hand's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (hand)

The baseline hand is the fastest (16.04 ms median); no rival improves on it (all deltas are >= 0).


### Two tiers: {hand, hand_par} vs {engine_par, engine, rust_nostd, rust_std_par, rust_std} (257% apart)

The field splits into a fast tier {hand, hand_par} and a slow tier {engine_par, engine, rust_nostd, rust_std_par, rust_std} with a 257% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 9.0x the fastest

Fastest hand (16.04 ms) to slowest rust_std (144.17 ms): 9.0x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (hand) is the fastest at 16044590.4 ns median
- 6 variants significantly slower than baseline
- Spread: 8.99x (fastest 16044590.4 ns, slowest 144173682.3 ns)

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
| engine | 74912454ns | 74585661ns | 74475175ns | 74524806ns | 75312509ns | +366.19% |
| engine_par | 71192812ns | 71049161ns | 70321344ns | 70630950ns | 71832075ns | +343.05% |
| hand | 16068963ns | 16046264ns | 16033035ns | 16042116ns | 16098080ns | base |
| hand_par | 19922988ns | 19897210ns | 19606022ns | 19720022ns | 20154453ns | +23.98% |
| rust_nostd | 90273857ns | 89714958ns | 89513922ns | 89595414ns | 90972673ns | +461.79% |
| rust_std | 144332591ns | 144175479ns | 143974621ns | 144077434ns | 144613450ns | +798.21% |
| rust_std_par | 124065053ns | 125482663ns | 118080934ns | 122349439ns | 126847794ns | +672.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 74911270ns | 74474269ns | 75311143ns | +366.24% | 2.688 |
| engine_par | 71190363ns | 70319150ns | 71829670ns | +343.08% | 2.828 |
| hand | 16067243ns | 16031669ns | 16096496ns | base | 12.530 |
| hand_par | 19920012ns | 19602698ns | 20151653ns | +23.98% | 10.107 |
| rust_nostd | 90272275ns | 89512632ns | 90970945ns | +461.84% | 2.230 |
| rust_std | 144330923ns | 143973128ns | 144611712ns | +798.29% | 1.395 |
| rust_std_par | 124061985ns | 118077713ns | 126844930ns | +672.14% | 1.623 |

## Performance model

- Peak throughput: 12.558 Gops/s (hand; best 20% batches)
- Ops per call: 201326592

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 2.699 | 21.5% |
| engine_par | 2.834 | 22.6% |
| hand | 12.548 | 99.9% |
| hand_par | 10.120 | 80.6% |
| rust_nostd | 2.244 | 17.9% |
| rust_std | 1.396 | 11.1% |
| rust_std_par | 1.604 | 12.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 74912454ns | 74912454ns | +366.19% |
| engine_par | 71192812ns | 71192812ns | +343.05% |
| hand | 16068963ns | 16068963ns | base |
| hand_par | 19922988ns | 19922988ns | +23.98% |
| rust_nostd | 90273857ns | 90273857ns | +461.79% |
| rust_std | 144332591ns | 144332591ns | +798.21% |
| rust_std_par | 124065053ns | 124065053ns | +672.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 16044590ns | base | --- | [16039130, 16055274] | --- | --- | --- | --- |
| engine | 74584579ns | +58539951.8ns (+364.9%) | [+58433953, +58861378]ns | [74487301, 74893047] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 71046610ns | +55011210.2ns (+342.9%) | [+54440247, +55838725]ns | [70480564, 71887058] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 19894242ns | +3849880.3ns (+24.0%) | [+3640084, +4050894]ns | [19677066, 20251839] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 89713085ns | +73672502.2ns (+459.2%) | [+73464300, +74965125]ns | [89540641, 91020399] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 144173682ns | +128138282.6ns (+798.6%) | [+128012171, +128630923]ns | [144060504, 144679329] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 125479347ns | +109423263.0ns (+682.0%) | [+103200166, +110772778]ns | [119248573, 126885754] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 16040582ns | +364.6% | +339.8% | +23.7% | +459.3% | +798.2% | +680.5% |
| 1 | 2 | 16048369ns | +365.1% | +347.5% | +24.2% | +462.6% | +799.0% | +681.9% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| hand | -0.500 | HIGH- (thermal bounce) |
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
| engine | 188638382.1ns | 74911270.1ns | 251.8% | HIGH |
| engine_par | 190361585.7ns | 71190363.3ns | 267.4% | HIGH |
| hand | 188258084.9ns | 16067243.4ns | 1171.7% | HIGH |
| hand_par | 189509622.2ns | 19920012.0ns | 951.4% | HIGH |
| rust_nostd | 189174265.7ns | 90272274.7ns | 209.6% | HIGH |
| rust_std | 188373699.4ns | 144330923.4ns | 130.5% | HIGH |
| rust_std_par | 190387005.4ns | 124061984.5ns | 153.5% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 74474269.2-75311143.2 ns)
  74474269.2 |########################################
  74516112.9 |########################################
  74557956.6 |
  74599800.3 |####################
  74641644.0 |####################
  74683487.7 |
  74725331.4 |
  74767175.1 |
  74809018.8 |
  74850862.5 |
  74892706.2 |####################
  74934549.9 |
  74976393.6 |
  75018237.3 |
  75060081.0 |
  75101924.7 |
  75143768.4 |
  75185612.1 |
  75227455.8 |
  75269299.5 |
  (0 below, 1 above range)

engine_par (n=8, range 70319149.6-71829670.4 ns)
  70319149.6 |########################################
  70394675.6 |
  70470201.7 |########################################
  70545727.7 |########################################
  70621253.8 |
  70696779.8 |
  70772305.8 |########################################
  70847831.9 |
  70923357.9 |
  70998884.0 |
  71074410.0 |
  71149936.1 |
  71225462.1 |
  71300988.1 |########################################
  71376514.2 |
  71452040.2 |
  71527566.3 |
  71603092.3 |
  71678618.3 |########################################
  71754144.4 |
  (0 below, 2 above range)

hand (n=8, range 16031669.2-16096495.7 ns)
  16031669.2 |#############
  16034910.5 |
  16038151.9 |########################################
  16041393.2 |
  16044634.5 |
  16047875.8 |##########################
  16051117.2 |
  16054358.5 |#############
  16057599.8 |
  16060841.1 |
  16064082.5 |
  16067323.8 |
  16070565.1 |
  16073806.4 |
  16077047.8 |
  16080289.1 |
  16083530.4 |
  16086771.7 |
  16090013.1 |
  16093254.4 |
  (0 below, 1 above range)

hand_par (n=8, range 19602697.6-20151653.0 ns)
  19602697.6 |####################
  19630145.4 |
  19657593.1 |########################################
  19685040.9 |
  19712488.7 |
  19739936.5 |
  19767384.2 |####################
  19794832.0 |
  19822279.8 |
  19849727.5 |
  19877175.3 |
  19904623.1 |
  19932070.9 |
  19959518.6 |
  19986966.4 |####################
  20014414.2 |
  20041861.9 |
  20069309.7 |####################
  20096757.5 |
  20124205.3 |
  (0 below, 2 above range)

rust_nostd (n=8, range 89512632.4-90970944.9 ns)
  89512632.4 |########################################
  89585548.0 |########################################
  89658463.6 |
  89731379.3 |####################
  89804294.9 |
  89877210.5 |####################
  89950126.1 |
  90023041.8 |
  90095957.4 |
  90168873.0 |
  90241788.6 |
  90314704.3 |
  90387619.9 |
  90460535.5 |
  90533451.1 |
  90606366.8 |
  90679282.4 |
  90752198.0 |
  90825113.6 |
  90898029.3 |
  (0 below, 2 above range)

rust_std (n=8, range 143973128.4-144611711.6 ns)
  143973128.4 |####################
  144005057.6 |
  144036986.7 |####################
  144068915.9 |########################################
  144100845.0 |
  144132774.2 |
  144164703.3 |
  144196632.5 |
  144228561.7 |####################
  144260490.8 |####################
  144292420.0 |
  144324349.1 |
  144356278.3 |
  144388207.4 |
  144420136.6 |
  144452065.8 |
  144483994.9 |
  144515924.1 |
  144547853.2 |
  144579782.4 |
  (0 below, 2 above range)

rust_std_par (n=8, range 118077712.6-126844929.5 ns)
  118077712.6 |####################
  118516073.4 |
  118954434.3 |####################
  119392795.1 |
  119831156.0 |
  120269516.8 |
  120707877.7 |
  121146238.5 |
  121584599.4 |
  122022960.2 |
  122461321.1 |
  122899681.9 |
  123338042.8 |####################
  123776403.6 |
  124214764.5 |####################
  124653125.3 |
  125091486.2 |
  125529847.0 |
  125968207.9 |
  126406568.7 |########################################
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=252.7% of algo (FFI overhead may distort results)
- engine_par: bridge=268.1% of algo (FFI overhead may distort results)
- hand: bridge=1173.0% of algo (FFI overhead may distort results)
- hand_par: bridge=951.3% of algo (FFI overhead may distort results)
- rust_nostd: bridge=210.8% of algo (FFI overhead may distort results)
- rust_std: bridge=130.6% of algo (FFI overhead may distort results)
- rust_std_par: bridge=151.8% of algo (FFI overhead may distort results)
