# fold_light, far above cache, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel dominates: 29% faster than the next best (head_tail)

std_parallel (2.16 ms) leads head_tail (2.78 ms) by 29%, a clear separation rather than a photo finish. CV 20.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### head_tail_dyn is an outlier: 3.1x slower than the field

head_tail_dyn (6.75 ms) is 3.1x the fastest (2.16 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.67)

std_parallel's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.1x the fastest

Fastest std_parallel (2.16 ms) to slowest head_tail_dyn (6.75 ms): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail_dyn is inconsistent: worst-20% is 2.8x its best-20%

head_tail_dyn's best 20% of batches run at 3.35 ms but its worst 20% at 9.21 ms (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 2158892.9 ns median (-22.4% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 3.13x (fastest 2158892.9 ns, slowest 6747585.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 4 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112786031 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 3351871ns | 2792173ns | 2595330ns | 2737287ns | 4652016ns | base |
| head_tail_dyn | 6759203ns | 6750964ns | 3360152ns | 6253250ns | 9217659ns | +101.65% |
| kernel_nway | 4769370ns | 4352170ns | 3460523ns | 4132179ns | 6379581ns | +42.29% |
| kernel_seq | 4042216ns | 4179513ns | 3407495ns | 3929250ns | 4529025ns | +20.60% |
| nway | 5256564ns | 5202876ns | 4729235ns | 5171490ns | 5647839ns | +56.82% |
| sequential | 3685194ns | 3627236ns | 3245640ns | 3501463ns | 4180568ns | +9.94% |
| std_parallel | 2375301ns | 2162360ns | 2130083ns | 2152230ns | 2832516ns | -29.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 3343960ns | 2588741ns | 4642134ns | base | 5.017 |
| head_tail_dyn | 6753581ns | 3348080ns | 9212746ns | +101.96% | 2.484 |
| kernel_nway | 4759432ns | 3448546ns | 6370426ns | +42.33% | 3.525 |
| kernel_seq | 4033497ns | 3403583ns | 4515181ns | +20.62% | 4.159 |
| nway | 5251267ns | 4724542ns | 5641708ns | +57.04% | 3.195 |
| sequential | 3683963ns | 3244653ns | 4179376ns | +10.17% | 4.554 |
| std_parallel | 2370406ns | 2125991ns | 2825616ns | -29.11% | 7.078 |

## Performance model

- Peak throughput: **7.891 Gops/s** (std_parallel; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.029 | 76.4% |
| head_tail_dyn | 2.486 | 31.5% |
| kernel_nway | 3.864 | 49.0% |
| kernel_seq | 4.020 | 50.9% |
| nway | 3.228 | 40.9% |
| sequential | 4.627 | 58.6% |
| std_parallel | 7.771 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3351871ns | 3351871ns | base |
| head_tail_dyn | 6759203ns | 6759203ns | +101.65% |
| kernel_nway | 4769370ns | 4769370ns | +42.29% |
| kernel_seq | 4042216ns | 4042216ns | +20.60% |
| nway | 5256564ns | 5256564ns | +56.82% |
| sequential | 3685194ns | 3685194ns | +9.94% |
| std_parallel | 2375301ns | 2375301ns | -29.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2782976ns | base | --- | [2606770, 4642134] | --- | --- | --- | --- |
| head_tail_dyn | 6747585ns | +3256334.5ns (+117.0%) | [+1128050, +5844478]ns | [4300411, 9212746] | YES (adj: no) | 0.1875 | 0.0313 | 0 |
| kernel_nway | 4342364ns | no significant difference | [-430962, +3763656]ns | [3565508, 6370426] | no | 0.6875 | 0.6875 | 0 |
| kernel_seq | 4173955ns | no significant difference | [-1230779, +1736449]ns | [3411354, 4515181] | no | 0.6875 | 0.6875 | 0 |
| nway | 5198173ns | +2408713.9ns (+86.6%) | [+311300, +3001909]ns | [4913920, 5641708] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| sequential | 3625806ns | no significant difference | [-462758, +862676]ns | [3246706, 4179376] | no | 0.3281 | 0.2188 | 0 |
| std_parallel | 2158893ns | no significant difference | [-2483241, +75669]ns | [2126710, 2825616] | no | 0.3281 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2639799ns | +85.1% | +96.5% | +59.2% | +113.7% | +23.5% | -19.4% |
| 1 | 2 | 2749946ns | +212.5% | +77.5% | +63.1% | +87.6% | +31.4% | +2.8% |
| 1 | 3 | 4642134ns | +46.0% | -9.3% | -26.5% | +6.7% | -10.0% | -53.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.139 | ok |
| head_tail_dyn | -0.493 | moderate- |
| kernel_nway | -0.029 | ok |
| kernel_seq | -0.329 | moderate- |
| nway | -0.034 | ok |
| sequential | -0.012 | ok |
| std_parallel | -0.666 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 3726527.2ns | 3343959.8ns | 111.4% | HIGH |
| head_tail_dyn | 6417602.9ns | 6753580.7ns | 95.0% | HIGH |
| kernel_nway | 4505386.7ns | 4759432.3ns | 94.7% | HIGH |
| kernel_seq | 4433048.8ns | 4033496.9ns | 109.9% | HIGH |
| nway | 5110156.9ns | 5251267.1ns | 97.3% | HIGH |
| sequential | 3717078.7ns | 3683962.8ns | 100.9% | HIGH |
| std_parallel | 2543495.9ns | 2370406.1ns | 107.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 2588741.1-4642133.6 ns)
  2588741.1 |########################################
  2691410.7 |
  2794080.4 |
  2896750.0 |#############
  2999419.6 |
  3102089.2 |
  3204758.8 |
  3307428.5 |
  3410098.1 |
  3512767.7 |
  3615437.3 |#############
  3718107.0 |
  3820776.6 |
  3923446.2 |
  4026115.8 |
  4128785.5 |
  4231455.1 |
  4334124.7 |
  4436794.3 |
  4539464.0 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 3348079.7-9212746.3 ns)
  3348079.7 |########################################
  3641313.0 |
  3934546.4 |
  4227779.7 |
  4521013.0 |
  4814246.4 |
  5107479.7 |########################################
  5400713.0 |
  5693946.3 |
  5987179.7 |
  6280413.0 |########################################
  6573646.3 |
  6866879.7 |########################################
  7160113.0 |
  7453346.3 |
  7746579.7 |
  8039813.0 |########################################
  8333046.3 |
  8626279.6 |
  8919513.0 |
  (0 below, 1 above range)

kernel_nway (n=6, range 3448546.2-6370425.8 ns)
  3448546.2 |####################
  3594640.2 |########################################
  3740734.2 |
  3886828.1 |
  4032922.1 |
  4179016.1 |
  4325110.1 |
  4471204.1 |
  4617298.0 |
  4763392.0 |
  4909486.0 |####################
  5055580.0 |
  5201674.0 |
  5347767.9 |
  5493861.9 |
  5639955.9 |
  5786049.9 |
  5932143.9 |####################
  6078237.8 |
  6224331.8 |
  (0 below, 1 above range)

kernel_seq (n=6, range 3403582.8-4515181.1 ns)
  3403582.8 |########################################
  3459162.7 |
  3514742.6 |
  3570322.5 |
  3625902.5 |
  3681482.4 |
  3737062.3 |
  3792642.2 |
  3848222.1 |
  3903802.0 |
  3959381.9 |
  4014961.9 |
  4070541.8 |
  4126121.7 |####################
  4181701.6 |########################################
  4237281.5 |
  4292861.4 |
  4348441.4 |
  4404021.3 |
  4459601.2 |
  (0 below, 1 above range)

nway (n=6, range 4724542.2-5641707.8 ns)
  4724542.2 |########################################
  4770400.5 |
  4816258.8 |
  4862117.0 |
  4907975.3 |
  4953833.6 |
  4999691.9 |
  5045550.2 |
  5091408.5 |########################################
  5137266.7 |########################################
  5183125.0 |########################################
  5228983.3 |########################################
  5274841.6 |
  5320699.9 |
  5366558.2 |
  5412416.4 |
  5458274.7 |
  5504133.0 |
  5549991.3 |
  5595849.6 |
  (0 below, 1 above range)

sequential (n=6, range 3244652.8-4179376.1 ns)
  3244652.8 |########################################
  3291389.0 |
  3338125.1 |
  3384861.3 |
  3431597.4 |
  3478333.6 |
  3525069.8 |
  3571805.9 |
  3618542.1 |
  3665278.3 |
  3712014.4 |
  3758750.6 |
  3805486.8 |
  3852222.9 |
  3898959.1 |
  3945695.2 |#############
  3992431.4 |
  4039167.6 |
  4085903.7 |
  4132639.9 |#############
  (0 below, 1 above range)

std_parallel (n=6, range 2125991.3-2825615.9 ns)
  2125991.3 |########################################
  2160972.5 |#############
  2195953.8 |
  2230935.0 |
  2265916.2 |
  2300897.4 |#############
  2335878.7 |
  2370859.9 |
  2405841.1 |
  2440822.3 |
  2475803.6 |
  2510784.8 |
  2545766.0 |
  2580747.3 |
  2615728.5 |
  2650709.7 |
  2685690.9 |
  2720672.2 |
  2755653.4 |
  2790634.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=32.1% (high variance, measurements may be unstable)
- **head_tail**: bridge=100.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=31.9% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=76.6% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=26.3% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=98.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.7% of algo (FFI overhead may distort results)
- **nway**: bridge=95.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=101.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=104.5% of algo (FFI overhead may distort results)
