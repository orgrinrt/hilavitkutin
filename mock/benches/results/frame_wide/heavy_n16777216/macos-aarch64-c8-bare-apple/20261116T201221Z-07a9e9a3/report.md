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

The declared/defaulted baseline hand has the worst median (213.98 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest rust_std_par at 43.63 ms).


### rust_std_par beats baseline by 80% (significant)

rust_std_par is -170.29 ms (80%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 4.9x slower than the field

hand (213.98 ms) is 4.9x the fastest (43.63 ms), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {rust_std_par, hand_par} vs {engine_par, rust_nostd, rust_std, engine, hand} (168% apart)

The field splits into a fast tier {rust_std_par, hand_par} and a slow tier {engine_par, rust_nostd, rust_std, engine, hand} with a 168% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 4.9x the fastest

Fastest rust_std_par (43.63 ms) to slowest hand (213.98 ms): 4.9x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: rust_std_par at 43631304.1 ns median (-79.6% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 4.90x (fastest 43631304.1 ns, slowest 213984354.2 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 150039541ns | 149961183ns | 149231875ns | 149417808ns | 150707756ns | -29.89% |
| engine_par | 126474886ns | 126238667ns | 125913367ns | 126104569ns | 126893004ns | -40.90% |
| hand | 213997435ns | 213988858ns | 213702375ns | 213862592ns | 214172333ns | base |
| hand_par | 47147796ns | 47185908ns | 45063475ns | 46338053ns | 48276183ns | -77.97% |
| rust_nostd | 149129479ns | 148649588ns | 148213925ns | 148563867ns | 149782577ns | -30.31% |
| rust_std | 149643110ns | 149473108ns | 149020567ns | 149310853ns | 150047940ns | -30.07% |
| rust_std_par | 43340951ns | 43636658ns | 41648492ns | 42801172ns | 44168900ns | -79.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 150034073ns | 149225025ns | 150703315ns | -29.89% | 1.342 |
| engine_par | 126469840ns | 125909342ns | 126887673ns | -40.90% | 1.592 |
| hand | 213992308ns | 213695950ns | 214167631ns | base | 0.941 |
| hand_par | 47141795ns | 45057767ns | 48270690ns | -77.97% | 4.271 |
| rust_nostd | 149123423ns | 148209025ns | 149776975ns | -30.31% | 1.350 |
| rust_std | 149636486ns | 149014083ns | 150041458ns | -30.07% | 1.345 |
| rust_std_par | 43335299ns | 41643583ns | 44163104ns | -79.75% | 4.646 |

## Performance model

- Peak throughput: 4.835 Gops/s (rust_std_par; best 20% batches)
- Ops per call: 201326592

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 1.343 | 27.8% |
| engine_par | 1.595 | 33.0% |
| hand | 0.941 | 19.5% |
| hand_par | 4.267 | 88.3% |
| rust_nostd | 1.354 | 28.0% |
| rust_std | 1.347 | 27.9% |
| rust_std_par | 4.614 | 95.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 150039541ns | 150039541ns | -29.89% |
| engine_par | 126474886ns | 126474886ns | -40.90% |
| hand | 213997435ns | 213997435ns | base |
| hand_par | 47147796ns | 47147796ns | -77.97% |
| rust_nostd | 149129479ns | 149129479ns | -30.31% |
| rust_std | 149643110ns | 149643110ns | -30.07% |
| rust_std_par | 43340951ns | 43340951ns | -79.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 213984354ns | base | --- | [213770383, 214236408] | --- | --- | --- | --- |
| engine | 149956567ns | -64153233.3ns (-30.0%) | [-64481008, -63150025]ns | [149289375, 150880200] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 126233696ns | -87814329.2ns (-41.0%) | [-88121267, -86508842]ns | [125958633, 127187108] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 47181008ns | -166952308.3ns (-78.0%) | [-168556308, -165981992]ns | [45429208, 48285917] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 148643075ns | -65160250.0ns (-30.5%) | [-65540471, -64007117]ns | [148524625, 149811300] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 149464954ns | -64566720.8ns (-30.2%) | [-64870517, -63817583]ns | [149244808, 150000833] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 43631304ns | -170290458.3ns (-79.6%) | [-171931167, -170100575]ns | [42099058, 44137458] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 213877950ns | -29.9% | -41.0% | -77.9% | -30.5% | -30.0% | -79.4% |
| 1 | 2 | 214006708ns | -29.9% | -41.0% | -78.0% | -30.3% | -30.2% | -80.1% |

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
| engine | 270923193.8ns | 150034072.9ns | 180.6% | HIGH |
| engine_par | 270942918.8ns | 126469839.6ns | 214.2% | HIGH |
| hand | 271270293.8ns | 213992308.3ns | 126.8% | HIGH |
| hand_par | 271094065.6ns | 47141794.8ns | 575.1% | HIGH |
| rust_nostd | 270747627.1ns | 149123422.9ns | 181.6% | HIGH |
| rust_std | 270594395.9ns | 149636486.4ns | 180.8% | HIGH |
| rust_std_par | 271992737.5ns | 43335298.9ns | 627.6% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 149225025.0-150703314.6 ns)
  149225025.0 |########################################
  149298939.5 |####################
  149372854.0 |
  149446768.4 |
  149520682.9 |####################
  149594597.4 |
  149668511.9 |
  149742426.4 |
  149816340.8 |
  149890255.3 |
  149964169.8 |
  150038084.3 |
  150111998.8 |
  150185913.2 |
  150259827.7 |####################
  150333742.2 |
  150407656.7 |####################
  150481571.2 |
  150555485.6 |
  150629400.1 |
  (0 below, 2 above range)

engine_par (n=8, range 125909341.7-126887672.9 ns)
  125909341.7 |####################
  125958258.3 |####################
  126007174.8 |
  126056091.4 |
  126105007.9 |
  126153924.5 |########################################
  126202841.1 |
  126251757.6 |####################
  126300674.2 |
  126349590.7 |
  126398507.3 |####################
  126447423.9 |
  126496340.4 |
  126545257.0 |
  126594173.5 |
  126643090.1 |
  126692006.7 |
  126740923.2 |
  126789839.8 |
  126838756.3 |
  (0 below, 2 above range)

hand (n=8, range 213695950.0-214167631.2 ns)
  213695950.0 |####################
  213719534.1 |
  213743118.1 |
  213766702.2 |####################
  213790286.2 |
  213813870.3 |####################
  213837454.4 |
  213861038.4 |
  213884622.5 |
  213908206.6 |
  213931790.6 |
  213955374.7 |
  213978958.8 |########################################
  214002542.8 |
  214026126.9 |####################
  214049710.9 |
  214073295.0 |
  214096879.1 |
  214120463.1 |
  214144047.2 |
  (0 below, 2 above range)

hand_par (n=8, range 45057766.7-48270689.6 ns)
  45057766.7 |########################################
  45218412.8 |
  45379059.0 |########################################
  45539705.1 |
  45700351.3 |
  45860997.4 |
  46021643.6 |
  46182289.7 |
  46342935.9 |
  46503582.0 |########################################
  46664228.2 |
  46824874.3 |
  46985520.4 |########################################
  47146166.6 |
  47306812.7 |########################################
  47467458.9 |
  47628105.0 |########################################
  47788751.2 |
  47949397.3 |
  48110043.5 |
  (0 below, 2 above range)

rust_nostd (n=8, range 148209025.0-149776975.0 ns)
  148209025.0 |####################
  148287422.5 |
  148365820.0 |
  148444217.5 |
  148522615.0 |########################################
  148601012.5 |########################################
  148679410.0 |
  148757807.5 |
  148836205.0 |
  148914602.5 |
  148993000.0 |
  149071397.5 |
  149149795.0 |
  149228192.5 |
  149306590.0 |
  149384987.5 |
  149463385.0 |
  149541782.5 |
  149620180.0 |
  149698577.5 |####################
  (0 below, 2 above range)

rust_std (n=8, range 149014083.3-150041458.3 ns)
  149014083.3 |########################################
  149065452.1 |
  149116820.8 |
  149168189.6 |
  149219558.3 |########################################
  149270927.1 |########################################
  149322295.8 |
  149373664.6 |########################################
  149425033.3 |
  149476402.1 |
  149527770.8 |########################################
  149579139.6 |
  149630508.3 |
  149681877.1 |
  149733245.8 |########################################
  149784614.6 |
  149835983.3 |
  149887352.1 |
  149938720.8 |
  149990089.6 |########################################
  (0 below, 1 above range)

rust_std_par (n=8, range 41643583.3-44163104.1 ns)
  41643583.3 |####################
  41769559.3 |
  41895535.4 |
  42021511.4 |####################
  42147487.5 |
  42273463.5 |
  42399439.6 |
  42525415.6 |
  42651391.6 |
  42777367.7 |####################
  42903343.7 |
  43029319.8 |
  43155295.8 |
  43281271.9 |####################
  43407247.9 |
  43533223.9 |
  43659200.0 |
  43785176.0 |########################################
  43911152.1 |
  44037128.1 |####################
  (0 below, 1 above range)

```

## Diagnostics

- engine: bridge=180.6% of algo (FFI overhead may distort results)
- engine_par: bridge=214.5% of algo (FFI overhead may distort results)
- hand: bridge=126.7% of algo (FFI overhead may distort results)
- hand_par: bridge=574.6% of algo (FFI overhead may distort results)
- rust_nostd: bridge=182.0% of algo (FFI overhead may distort results)
- rust_std: bridge=181.0% of algo (FFI overhead may distort results)
- rust_std_par: bridge=621.6% of algo (FFI overhead may distort results)
