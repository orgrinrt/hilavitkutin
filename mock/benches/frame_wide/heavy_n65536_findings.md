# Wide frame, heavy per-record compute

7 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hand_par dominates: 19% faster than the next best (rust_std_par)

hand_par (1.35 ms) leads rust_std_par (1.61 ms) by 19%, a clear separation rather than a photo finish. CV 14.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hand_par beats baseline by 58% (significant)

hand_par is -1.61 ms (58%) faster than baseline engine, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hand is an outlier: 2.5x slower than the field

hand (3.42 ms) is 2.5x the fastest (1.35 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {hand_par, rust_std_par} vs {rust_nostd, engine_par, engine, rust_std, hand} (70% apart)

The field splits into a fast tier {hand_par, rust_std_par} and a slow tier {rust_nostd, engine_par, engine, rust_std, hand} with a 70% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### rust_std_par is inconsistent: worst-20% is 1.6x its best-20%

rust_std_par's best 20% of batches run at 1.23 ms but its worst 20% at 1.94 ms (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hand_par** at 1351788.0 ns median (-51.6% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.53x (fastest 1351788.0 ns, slowest 3416134.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 2885374ns | 2792300ns | 2625409ns | 2714606ns | 3078441ns | base |
| engine_par | 2901023ns | 2744711ns | 2514764ns | 2670443ns | 3170522ns | +0.54% |
| hand | 3412987ns | 3417988ns | 3361582ns | 3391354ns | 3442062ns | +18.29% |
| hand_par | 1326522ns | 1354166ns | 1076972ns | 1193587ns | 1488610ns | -54.03% |
| rust_nostd | 2725317ns | 2731503ns | 2652557ns | 2692169ns | 2768368ns | -5.55% |
| rust_std | 3099261ns | 3057513ns | 2914260ns | 2975684ns | 3238195ns | +7.41% |
| rust_std_par | 1628988ns | 1612267ns | 1231690ns | 1339867ns | 1945153ns | -43.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 2883793ns | 2624239ns | 3076660ns | base | 0.273 |
| engine_par | 2898747ns | 2512726ns | 3168187ns | +0.52% | 0.271 |
| hand | 3411503ns | 3360883ns | 3440353ns | +18.30% | 0.231 |
| hand_par | 1324417ns | 1075853ns | 1486306ns | -54.07% | 0.594 |
| rust_nostd | 2723274ns | 2650686ns | 2766373ns | -5.57% | 0.289 |
| rust_std | 3097105ns | 2911888ns | 3236096ns | +7.40% | 0.254 |
| rust_std_par | 1626679ns | 1229480ns | 1942892ns | -43.59% | 0.483 |

## Performance model

- Peak throughput: **0.731 Gops/s** (hand_par; best 20% batches)
- Ops per call: 786432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.282 | 38.5% |
| engine_par | 0.287 | 39.2% |
| hand | 0.230 | 31.5% |
| hand_par | 0.582 | 79.6% |
| rust_nostd | 0.288 | 39.4% |
| rust_std | 0.257 | 35.2% |
| rust_std_par | 0.488 | 66.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 2885374ns | 2885374ns | base |
| engine_par | 2901023ns | 2901023ns | +0.54% |
| hand | 3412987ns | 3412987ns | +18.29% |
| hand_par | 1326522ns | 1326522ns | -54.03% |
| rust_nostd | 2725317ns | 2725317ns | -5.55% |
| rust_std | 3099261ns | 3099261ns | +7.41% |
| rust_std_par | 1628988ns | 1628988ns | -43.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 2790864ns | base | --- | [2640272, 3057535] | --- | --- | --- | --- |
| engine_par | 2742097ns | no significant difference | [-544808, +711022]ns | [2555314, 3431291] | no | 0.7266 | 0.7266 | 0 |
| hand | 3416135ns | +615098.5ns (+22.0%) | [+393225, +736645]ns | [3377935, 3445689] | YES (adj: no) | 0.1055 | 0.0703 | 0 |
| hand_par | 1351788ns | -1613642.7ns (-57.8%) | [-1727842, -1197242]ns | [1124411, 1426997] | YES | 0.0234 | 0.0078 | 0 |
| rust_nostd | 2729571ns | no significant difference | [-295410, +147081]ns | [2664033, 2771320] | no | 0.7266 | 0.7266 | 0 |
| rust_std | 3055325ns | +289970.3ns (+10.4%) | [+201162, +480613]ns | [2934683, 3282368] | YES (adj: no) | 0.1055 | 0.0703 | 0 |
| rust_std_par | 1610005ns | -1248085.6ns (-44.7%) | [-1573325, -761699]ns | [1257315, 2008410] | YES | 0.0234 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | engine_par | hand | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2680271ns | +2.3% | +27.8% | -47.3% | +3.2% | +14.0% | -27.8% |
| 1 | 2 | 2803250ns | +0.9% | +21.5% | -56.7% | -4.5% | +11.1% | -53.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| hand | -0.500 | moderate- |
| hand_par | -0.500 | moderate- |
| rust_nostd | -0.500 | moderate- |
| rust_std | -0.500 | moderate- |
| rust_std_par | -0.500 | moderate- |

**Consistency summary:**

- **engine_par**: won 0/2, lost 2/2
- **hand**: won 0/2, lost 2/2
- **hand_par**: won 2/2, lost 0/2
- **rust_nostd**: won 1/2, lost 1/2
- **rust_std**: won 0/2, lost 2/2
- **rust_std_par**: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 991581.9ns | 2883792.7ns | 34.4% | HIGH |
| engine_par | 1116732.2ns | 2898746.9ns | 38.5% | HIGH |
| hand | 1013111.4ns | 3411503.3ns | 29.7% | HIGH |
| hand_par | 995747.0ns | 1324417.2ns | 75.2% | HIGH |
| rust_nostd | 993519.1ns | 2723273.5ns | 36.5% | HIGH |
| rust_std | 1040743.7ns | 3097104.7ns | 33.6% | HIGH |
| rust_std_par | 1031094.4ns | 1626678.5ns | 63.4% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 2624238.7-3076659.5 ns)
  2624238.7 |########################################
  2646859.7 |
  2669480.8 |
  2692101.8 |
  2714722.9 |####################
  2737343.9 |
  2759965.0 |####################
  2782586.0 |########################################
  2805207.0 |
  2827828.1 |
  2850449.1 |
  2873070.2 |
  2895691.2 |
  2918312.3 |
  2940933.3 |
  2963554.3 |
  2986175.4 |
  3008796.4 |
  3031417.5 |
  3054038.5 |####################
  (0 below, 1 above range)

engine_par (n=8, range 2512726.3-3168186.7 ns)
  2512726.3 |####################
  2545499.3 |####################
  2578272.3 |
  2611045.4 |
  2643818.4 |
  2676591.4 |
  2709364.4 |########################################
  2742137.4 |####################
  2774910.4 |
  2807683.5 |
  2840456.5 |
  2873229.5 |
  2906002.5 |####################
  2938775.5 |
  2971548.6 |
  3004321.6 |
  3037094.6 |
  3069867.6 |
  3102640.6 |
  3135413.7 |
  (0 below, 2 above range)

hand (n=8, range 3360883.4-3440353.4 ns)
  3360883.4 |########################################
  3364856.9 |
  3368830.4 |
  3372803.9 |
  3376777.4 |########################################
  3380750.9 |
  3384724.4 |########################################
  3388697.9 |
  3392671.4 |
  3396644.9 |
  3400618.4 |
  3404591.9 |########################################
  3408565.4 |
  3412538.9 |
  3416512.4 |
  3420485.9 |
  3424459.4 |########################################
  3428432.9 |
  3432406.4 |
  3436379.9 |########################################
  (0 below, 2 above range)

hand_par (n=8, range 1075853.3-1486306.4 ns)
  1075853.3 |########################################
  1096376.0 |
  1116898.6 |########################################
  1137421.3 |########################################
  1157943.9 |
  1178466.6 |
  1198989.2 |
  1219511.9 |
  1240034.6 |
  1260557.2 |
  1281079.9 |########################################
  1301602.5 |
  1322125.2 |
  1342647.8 |
  1363170.5 |
  1383693.2 |########################################
  1404215.8 |########################################
  1424738.5 |########################################
  1445261.1 |
  1465783.8 |
  (0 below, 1 above range)

rust_nostd (n=8, range 2650686.3-2766372.6 ns)
  2650686.3 |########################################
  2656470.6 |
  2662254.9 |########################################
  2668039.3 |
  2673823.6 |
  2679607.9 |
  2685392.2 |########################################
  2691176.5 |
  2696960.8 |
  2702745.2 |
  2708529.5 |
  2714313.8 |########################################
  2720098.1 |
  2725882.4 |
  2731666.7 |
  2737451.1 |
  2743235.4 |########################################
  2749019.7 |
  2754804.0 |
  2760588.3 |########################################
  (0 below, 2 above range)

rust_std (n=8, range 2911888.3-3236095.6 ns)
  2911888.3 |########################################
  2928098.7 |########################################
  2944309.0 |
  2960519.4 |
  2976729.8 |########################################
  2992940.1 |########################################
  3009150.5 |
  3025360.9 |
  3041571.2 |
  3057781.6 |
  3073992.0 |
  3090202.3 |########################################
  3106412.7 |
  3122623.0 |
  3138833.4 |
  3155043.8 |
  3171254.1 |
  3187464.5 |
  3203674.9 |
  3219885.2 |
  (0 below, 3 above range)

rust_std_par (n=8, range 1229480.0-1942892.4 ns)
  1229480.0 |########################################
  1265150.6 |
  1300821.2 |####################
  1336491.9 |
  1372162.5 |
  1407833.1 |####################
  1443503.7 |
  1479174.3 |
  1514844.9 |
  1550515.6 |
  1586186.2 |
  1621856.8 |
  1657527.4 |
  1693198.0 |
  1728868.6 |
  1764539.3 |####################
  1800209.9 |
  1835880.5 |####################
  1871551.1 |
  1907221.7 |
  (0 below, 2 above range)

```

## Diagnostics

- **engine**: bridge=35.6% of algo (FFI overhead may distort results)
- **engine_par**: bridge=37.6% of algo (FFI overhead may distort results)
- **hand**: bridge=29.1% of algo (FFI overhead may distort results)
- **hand_par**: bridge=73.4% of algo (FFI overhead may distort results)
- **rust_nostd**: bridge=36.6% of algo (FFI overhead may distort results)
- **rust_std**: bridge=34.0% of algo (FFI overhead may distort results)
- **rust_std_par**: CV=20.4% (high variance, measurements may be unstable)
- **rust_std_par**: bridge=63.7% of algo (FFI overhead may distort results)
