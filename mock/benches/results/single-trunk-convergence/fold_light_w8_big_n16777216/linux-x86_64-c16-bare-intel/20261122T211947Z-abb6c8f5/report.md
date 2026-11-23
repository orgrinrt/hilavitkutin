# fold_light, far above cache, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential shows alternating (throttle bounce) (autocorr -0.67)

sequential's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {std_parallel} vs {nway, head_tail, head_tail_dyn, kernel_nway, sequential, kernel_seq} (42% apart)

The field splits into a fast tier {std_parallel} and a slow tier {nway, head_tail, head_tail_dyn, kernel_nway, sequential, kernel_seq} with a 42% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### head_tail is inconsistent: worst-20% is 3.0x its best-20%

head_tail's best 20% of batches run at 2.51 ms but its worst 20% at 7.44 ms (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 2236397.9 ns median (-35.9% vs baseline)
- Spread: 1.87x (fastest 2236397.9 ns, slowest 4183394.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 8 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112617144 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 4502001ns | 3499612ns | 2509386ns | 3199101ns | 7452659ns | base |
| head_tail_dyn | 4658124ns | 3861083ns | 3215141ns | 3709754ns | 6802171ns | +3.47% |
| kernel_nway | 3913059ns | 3951037ns | 2822598ns | 3662661ns | 4833886ns | -13.08% |
| kernel_seq | 4709365ns | 4191259ns | 3984218ns | 4173058ns | 5876399ns | +4.61% |
| nway | 3304066ns | 3189756ns | 1983480ns | 2855535ns | 4637157ns | -26.61% |
| sequential | 3877505ns | 4150488ns | 3256773ns | 3870885ns | 4197799ns | -13.87% |
| std_parallel | 2736416ns | 2243163ns | 1756102ns | 2086583ns | 4201323ns | -39.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4494170ns | 2506881ns | 7444019ns | base | 3.733 |
| head_tail_dyn | 4649748ns | 3211015ns | 6791357ns | +3.46% | 3.608 |
| kernel_nway | 3904706ns | 2815853ns | 4824629ns | -13.12% | 4.297 |
| kernel_seq | 4703185ns | 3979569ns | 5868954ns | +4.65% | 3.567 |
| nway | 3300272ns | 1981230ns | 4631764ns | -26.57% | 5.084 |
| sequential | 3876382ns | 3255603ns | 4196618ns | -13.75% | 4.328 |
| std_parallel | 2730068ns | 1753647ns | 4191348ns | -39.25% | 6.145 |

## Performance model

- Peak throughput: **9.567 Gops/s** (std_parallel; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 4.806 | 50.2% |
| head_tail_dyn | 4.355 | 45.5% |
| kernel_nway | 4.256 | 44.5% |
| kernel_seq | 4.010 | 41.9% |
| nway | 5.266 | 55.0% |
| sequential | 4.043 | 42.3% |
| std_parallel | 7.502 | 78.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4502001ns | 4502001ns | base |
| head_tail_dyn | 4658124ns | 4658124ns | +3.47% |
| kernel_nway | 3913059ns | 3913059ns | -13.08% |
| kernel_seq | 4709365ns | 4709365ns | +4.61% |
| nway | 3304066ns | 3304066ns | -26.61% |
| sequential | 3877505ns | 3877505ns | -13.87% |
| std_parallel | 2736416ns | 2736416ns | -39.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 3491110ns | base | --- | [2547381, 7444019] | --- | --- | --- | --- |
| head_tail_dyn | 3852116ns | no significant difference | [-4138248, +3786428]ns | [3305770, 6791357] | no | 1.0000 | 0.6875 | 0 |
| kernel_nway | 3941898ns | no significant difference | [-3242914, +1491360]ns | [2947593, 4824629] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 4183395ns | no significant difference | [-1577663, +1611406]ns | [4057207, 5868954] | no | 1.0000 | 0.6875 | 0 |
| nway | 3186161ns | no significant difference | [-3347746, +716724]ns | [2082891, 4631764] | no | 1.0000 | 0.6875 | 0 |
| sequential | 4149428ns | no significant difference | [-3258857, +1178833]ns | [3283101, 4196618] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 2236398ns | no significant difference | [-5620896, +1157787]ns | [1762456, 4191348] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 3033562ns | +27.0% | +22.9% | +37.2% | -31.3% | +37.2% | +38.2% |
| 1 | 2 | 3004929ns | +126.0% | +25.9% | +35.8% | +23.9% | +9.3% | -27.6% |
| 1 | 3 | 7444019ns | -55.6% | -43.6% | -21.2% | -45.0% | -43.8% | -75.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.170 | ok |
| head_tail_dyn | -0.653 | HIGH- (thermal bounce) |
| kernel_nway | -0.109 | ok |
| kernel_seq | -0.190 | ok |
| nway | -0.077 | ok |
| sequential | -0.666 | HIGH- (thermal bounce) |
| std_parallel | -0.094 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 4664063.2ns | 4494169.9ns | 103.8% | HIGH |
| head_tail_dyn | 7478514.2ns | 4649747.7ns | 160.8% | HIGH |
| kernel_nway | 3901450.2ns | 3904706.5ns | 99.9% | HIGH |
| kernel_seq | 4199237.9ns | 4703185.5ns | 89.3% | HIGH |
| nway | 3117557.5ns | 3300272.2ns | 94.5% | HIGH |
| sequential | 3889234.6ns | 3876382.2ns | 100.3% | HIGH |
| std_parallel | 2969134.3ns | 2730067.5ns | 108.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 2506880.6-7444018.7 ns)
  2506880.6 |########################################
  2753737.5 |
  3000594.4 |
  3247451.3 |####################
  3494308.2 |####################
  3741165.1 |
  3988022.0 |
  4234878.9 |
  4481735.8 |
  4728592.7 |
  4975449.6 |
  5222306.5 |
  5469163.4 |
  5716020.3 |
  5962877.2 |####################
  6209734.1 |
  6456591.0 |
  6703447.9 |
  6950304.8 |
  7197161.7 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 3211015.4-6791356.8 ns)
  3211015.4 |####################
  3390032.5 |####################
  3569049.5 |
  3748066.6 |########################################
  3927083.7 |
  4106100.7 |
  4285117.8 |
  4464134.9 |
  4643151.9 |
  4822169.0 |####################
  5001186.1 |
  5180203.1 |
  5359220.2 |
  5538237.3 |
  5717254.3 |
  5896271.4 |
  6075288.5 |
  6254305.5 |
  6433322.6 |
  6612339.7 |
  (0 below, 1 above range)

kernel_nway (n=6, range 2815852.7-4824629.0 ns)
  2815852.7 |########################################
  2916291.5 |
  3016730.3 |########################################
  3117169.1 |
  3217608.0 |
  3318046.8 |########################################
  3418485.6 |
  3518924.4 |
  3619363.2 |
  3719802.0 |
  3820240.9 |
  3920679.7 |
  4021118.5 |
  4121557.3 |
  4221996.1 |
  4322434.9 |
  4422873.7 |########################################
  4523312.6 |
  4623751.4 |########################################
  4724190.2 |
  (0 below, 1 above range)

kernel_seq (n=6, range 3979569.3-5868954.5 ns)
  3979569.3 |#############
  4074038.6 |#############
  4168507.8 |########################################
  4262977.1 |
  4357446.3 |
  4451915.6 |
  4546384.8 |
  4640854.1 |
  4735323.4 |
  4829792.6 |
  4924261.9 |
  5018731.1 |
  5113200.4 |
  5207669.6 |
  5302138.9 |
  5396608.2 |
  5491077.4 |
  5585546.7 |
  5680015.9 |
  5774485.2 |
  (0 below, 1 above range)

nway (n=6, range 1981230.3-4631764.4 ns)
  1981230.3 |########################################
  2113757.0 |########################################
  2246283.7 |
  2378810.4 |
  2511337.1 |
  2643863.8 |
  2776390.5 |########################################
  2908917.3 |
  3041444.0 |
  3173970.7 |
  3306497.4 |
  3439024.1 |########################################
  3571550.8 |
  3704077.5 |
  3836604.2 |
  3969130.9 |
  4101657.6 |
  4234184.3 |
  4366711.0 |
  4499237.7 |########################################
  (0 below, 1 above range)

sequential (n=6, range 3255602.9-4196618.0 ns)
  3255602.9 |####################
  3302653.7 |####################
  3349704.4 |
  3396755.2 |
  3443805.9 |
  3490856.7 |
  3537907.4 |
  3584958.2 |
  3632008.9 |
  3679059.7 |
  3726110.5 |
  3773161.2 |
  3820212.0 |
  3867262.7 |
  3914313.5 |
  3961364.2 |
  4008415.0 |
  4055465.7 |
  4102516.5 |####################
  4149567.2 |########################################
  (0 below, 1 above range)

std_parallel (n=6, range 1753647.2-4191348.4 ns)
  1753647.2 |########################################
  1875532.3 |
  1997417.3 |
  2119302.4 |
  2241187.5 |
  2363072.5 |
  2484957.6 |#############
  2606842.6 |
  2728727.7 |
  2850612.8 |
  2972497.8 |
  3094382.9 |
  3216268.0 |
  3338153.0 |
  3460038.1 |#############
  3581923.1 |
  3703808.2 |
  3825693.3 |
  3947578.3 |
  4069463.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=50.6% (high variance, measurements may be unstable)
- **head_tail**: bridge=149.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=40.1% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=134.5% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=21.5% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=105.4% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=27.1% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=100.3% of algo (FFI overhead may distort results)
- **nway**: CV=32.1% (high variance, measurements may be unstable)
- **nway**: bridge=82.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.1% of algo (FFI overhead may distort results)
- **std_parallel**: CV=41.5% (high variance, measurements may be unstable)
- **std_parallel**: bridge=117.8% of algo (FFI overhead may distort results)
