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
slowest variant and reads as picked by accident, it was not: it is set in this
member's driver, on purpose, and is meant to be the bar whether it wins or
loses. Do not re-baseline on whichever arm happens to be fastest.

The lines under each highlight that call a large delta a reason to switch, or a
dominant winner a safe default pick, are the harness's stock phrasing for any
dataset. They are not a judgement about these arms and should be read as
describing the shape of the numbers, nothing more.

A median here is wall-clock time and carries nothing about how many cores an
arm used to reach it. Two arms with the same median are not the same cost if
one of them ran on one core and the other on two. The occupancy of every
parallel arm is measured separately and stated beside these medians in
`mock/research/202611131400_frame-wide-first-run.md`. Read the two together or
neither.

The generated report follows.

# Wide frame, heavy per-record compute

7 variants, 8 samples per variant.
Baseline: **hand**

## Highlights

Baseline for all deltas below: **hand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hand) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hand has the worst median (3.39 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hand_par at 1.03 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hand_par dominates: 29% faster than the next best (rust_std_par)

hand_par (1.03 ms) leads rust_std_par (1.34 ms) by 29%, a clear separation rather than a photo finish. CV 7.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hand_par beats baseline by 69% (significant)

hand_par is -2.35 ms (69%) faster than baseline hand, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hand is an outlier: 3.3x slower than the field

hand (3.39 ms) is 3.3x the fastest (1.03 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### engine_par shows alternating (throttle bounce) (autocorr -0.50)

engine_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {hand_par, rust_std_par} vs {rust_nostd, rust_std, engine, engine_par, hand} (113% apart)

The field splits into a fast tier {hand_par, rust_std_par} and a slow tier {rust_nostd, rust_std, engine, engine_par, hand} with a 113% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.3x the fastest

Fastest hand_par (1.03 ms) to slowest hand (3.39 ms): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### engine_par is inconsistent: worst-20% is 1.6x its best-20%

engine_par's best 20% of batches run at 2.82 ms but its worst 20% at 4.40 ms (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hand_par** at 1034137.3 ns median (-69.5% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 3.28x (fastest 1034137.3 ns, slowest 3393279.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 2985585ns | 2994843ns | 2628000ns | 2654973ns | 3322940ns | -14.03% |
| engine_par | 3672269ns | 3015144ns | 2823555ns | 2983720ns | 4400859ns | +5.74% |
| hand | 3472982ns | 3396858ns | 2711785ns | 3368627ns | 3741547ns | base |
| hand_par | 1050271ns | 1035695ns | 957076ns | 1016052ns | 1099234ns | -69.76% |
| rust_nostd | 2893281ns | 2842164ns | 2773197ns | 2813028ns | 2983493ns | -16.69% |
| rust_std | 3126236ns | 2929479ns | 2799398ns | 2873169ns | 3397746ns | -9.98% |
| rust_std_par | 1337118ns | 1338001ns | 1108678ns | 1237730ns | 1468769ns | -61.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 2984057ns | 2626385ns | 3321253ns | -14.02% | 0.264 |
| engine_par | 3670100ns | 2821872ns | 4398474ns | +5.75% | 0.214 |
| hand | 3470566ns | 2710163ns | 3739138ns | base | 0.227 |
| hand_par | 1048862ns | 956207ns | 1097838ns | -69.78% | 0.750 |
| rust_nostd | 2890554ns | 2771579ns | 2980052ns | -16.71% | 0.272 |
| rust_std | 3123922ns | 2796895ns | 3395483ns | -9.99% | 0.252 |
| rust_std_par | 1335508ns | 1107281ns | 1467094ns | -61.52% | 0.589 |

## Performance model

- Peak throughput: **0.822 Gops/s** (hand_par; best 20% batches)
- Ops per call: 786432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.263 | 31.9% |
| engine_par | 0.261 | 31.7% |
| hand | 0.232 | 28.2% |
| hand_par | 0.760 | 92.5% |
| rust_nostd | 0.277 | 33.7% |
| rust_std | 0.269 | 32.7% |
| rust_std_par | 0.589 | 71.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 2985585ns | 2985585ns | -14.03% |
| engine_par | 3672269ns | 3672269ns | +5.74% |
| hand | 3472982ns | 3472982ns | base |
| hand_par | 1050271ns | 1050271ns | -69.76% |
| rust_nostd | 2893281ns | 2893281ns | -16.69% |
| rust_std | 3126236ns | 3126236ns | -9.98% |
| rust_std_par | 1337118ns | 1337118ns | -61.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 3393279ns | base | --- | [3350959, 3857149] | --- | --- | --- | --- |
| engine | 2993321ns | -407282.1ns (-12.0%) | [-1230764, -20444]ns | [2633009, 3305006] | YES (adj: no) | 0.1055 | 0.0703 | 0 |
| engine_par | 3012981ns | no significant difference | [-793796, +111709]ns | [2962211, 3229383] | no | 0.2891 | 0.2891 | 0 |
| hand_par | 1034137ns | -2352721.9ns (-69.3%) | [-2853464, -2132100]ns | [1003685, 1100174] | YES | 0.0234 | 0.0078 | 0 |
| rust_nostd | 2840323ns | -525080.0ns (-15.5%) | [-1060285, -112946]ns | [2796864, 2958153] | YES (adj: no) | 0.1055 | 0.0703 | 0 |
| rust_std | 2927264ns | no significant difference | [-546621, +261349]ns | [2860090, 3486453] | no | 0.2891 | 0.2891 | 0 |
| rust_std_par | 1336216ns | -2262908.1ns (-66.7%) | [-2510385, -1960774]ns | [1140621, 1426300] | YES | 0.0234 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 3806579ns | -30.8% | -19.8% | -73.6% | -26.5% | -8.9% | -64.9% |
| 1 | 2 | 3358983ns | -1.7% | -10.9% | -69.2% | -13.4% | -14.4% | -62.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | HIGH- (thermal bounce) |
| hand | -0.500 | moderate- |
| hand_par | -0.500 | moderate- |
| rust_nostd | -0.500 | moderate- |
| rust_std | -0.500 | moderate- |
| rust_std_par | -0.500 | moderate- |

**Consistency summary:**

- **engine**: won 2/2, lost 0/2
- **engine_par**: won 2/2, lost 0/2
- **hand_par**: won 2/2, lost 0/2
- **rust_nostd**: won 2/2, lost 0/2
- **rust_std**: won 2/2, lost 0/2
- **rust_std_par**: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 997703.9ns | 2984057.4ns | 33.4% | HIGH |
| engine_par | 998523.3ns | 3670099.7ns | 27.2% | HIGH |
| hand | 969887.9ns | 3470566.2ns | 27.9% | HIGH |
| hand_par | 1006351.2ns | 1048861.6ns | 95.9% | HIGH |
| rust_nostd | 1041461.6ns | 2890554.2ns | 36.0% | HIGH |
| rust_std | 1055621.3ns | 3123922.4ns | 33.8% | HIGH |
| rust_std_par | 1042807.5ns | 1335507.8ns | 78.1% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 2626385.4-3321253.0 ns)
  2626385.4 |########################################
  2661128.8 |#############
  2695872.2 |
  2730615.5 |
  2765358.9 |
  2800102.3 |
  2834845.7 |
  2869589.1 |
  2904332.4 |
  2939075.8 |
  2973819.2 |
  3008562.6 |
  3043306.0 |
  3078049.3 |
  3112792.7 |
  3147536.1 |
  3182279.5 |
  3217022.9 |
  3251766.2 |
  3286509.6 |########################################
  (0 below, 1 above range)

engine_par (n=8, range 2821872.0-4398474.4 ns)
  2821872.0 |#############
  2900702.1 |#############
  2979532.2 |########################################
  3058362.4 |#############
  3137192.5 |
  3216022.6 |#############
  3294852.7 |
  3373682.8 |
  3452513.0 |
  3531343.1 |
  3610173.2 |
  3689003.3 |
  3767833.4 |
  3846663.5 |
  3925493.7 |
  4004323.8 |
  4083153.9 |
  4161984.0 |
  4240814.1 |
  4319644.3 |
  (0 below, 1 above range)

hand (n=8, range 2710163.4-3739138.2 ns)
  2710163.4 |####################
  2761612.1 |
  2813060.9 |
  2864509.6 |
  2915958.4 |
  2967407.1 |
  3018855.8 |
  3070304.6 |
  3121753.3 |
  3173202.1 |
  3224650.8 |
  3276099.6 |
  3327548.3 |########################################
  3378997.0 |########################################
  3430445.8 |
  3481894.5 |
  3533343.3 |
  3584792.0 |
  3636240.7 |
  3687689.5 |
  (0 below, 3 above range)

hand_par (n=8, range 956207.1-1097838.1 ns)
  956207.1 |####################
  963288.7 |
  970370.2 |
  977451.8 |
  984533.3 |
  991614.8 |
  998696.4 |####################
  1005777.9 |####################
  1012859.5 |
  1019941.0 |
  1027022.6 |####################
  1034104.1 |########################################
  1041185.7 |
  1048267.2 |
  1055348.8 |
  1062430.3 |
  1069511.9 |
  1076593.4 |
  1083675.0 |
  1090756.5 |
  (0 below, 2 above range)

rust_nostd (n=8, range 2771578.7-2980052.2 ns)
  2771578.7 |####################
  2782002.4 |
  2792426.1 |########################################
  2802849.7 |
  2813273.4 |
  2823697.1 |
  2834120.8 |####################
  2844544.4 |####################
  2854968.1 |####################
  2865391.8 |
  2875815.5 |
  2886239.1 |
  2896662.8 |
  2907086.5 |
  2917510.2 |
  2927933.8 |
  2938357.5 |
  2948781.2 |####################
  2959204.9 |
  2969628.5 |
  (0 below, 1 above range)

rust_std (n=8, range 2796895.2-3395483.0 ns)
  2796895.2 |#############
  2826824.6 |
  2856754.0 |########################################
  2886683.4 |
  2916612.8 |
  2946542.1 |#############
  2976471.5 |
  3006400.9 |
  3036330.3 |
  3066259.7 |
  3096189.1 |
  3126118.5 |
  3156047.9 |
  3185977.2 |
  3215906.6 |
  3245836.0 |
  3275765.4 |
  3305694.8 |
  3335624.2 |
  3365553.6 |
  (0 below, 3 above range)

rust_std_par (n=8, range 1107281.2-1467093.5 ns)
  1107281.2 |########################################
  1125271.8 |########################################
  1143262.4 |
  1161253.0 |
  1179243.7 |
  1197234.3 |
  1215224.9 |
  1233215.5 |########################################
  1251206.1 |
  1269196.7 |
  1287187.4 |
  1305178.0 |########################################
  1323168.6 |
  1341159.2 |########################################
  1359149.8 |
  1377140.4 |########################################
  1395131.1 |
  1413121.7 |########################################
  1431112.3 |
  1449102.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **engine**: bridge=33.0% of algo (FFI overhead may distort results)
- **engine_par**: CV=47.3% (high variance, measurements may be unstable)
- **engine_par**: bridge=33.5% of algo (FFI overhead may distort results)
- **hand**: bridge=29.5% of algo (FFI overhead may distort results)
- **hand_par**: bridge=98.9% of algo (FFI overhead may distort results)
- **rust_nostd**: bridge=36.0% of algo (FFI overhead may distort results)
- **rust_std**: bridge=36.0% of algo (FFI overhead may distort results)
- **rust_std_par**: bridge=75.6% of algo (FFI overhead may distort results)
