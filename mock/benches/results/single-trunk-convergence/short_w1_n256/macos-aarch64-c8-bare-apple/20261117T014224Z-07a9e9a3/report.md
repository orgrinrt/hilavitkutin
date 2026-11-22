# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 8.1x slower than the field

std_parallel (20.47 us) is 8.1x the fastest (2.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (kernel_seq, kernel_nway) are a dead heat (<1%)

kernel_seq (2.53 us) and kernel_nway (2.55 us) differ by 0.52%, inside the noise, even though the wider field spreads 707.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### nway shows alternating (throttle bounce) (autocorr -0.67)

nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, kernel_nway, head_tail, head_tail_dyn} vs {sequential, nway, std_parallel} (280% apart)

The field splits into a fast tier {kernel_seq, kernel_nway, head_tail, head_tail_dyn} and a slow tier {sequential, nway, std_parallel} with a 280% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.1x the fastest

Fastest kernel_seq (2.53 us) to slowest std_parallel (20.47 us): 8.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2534.2 ns median (-3.0% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 8.08x (fastest 2534.2 ns, slowest 20470.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 1 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 3024ns | 2894ns | 2783ns | 2878ns | 3365ns | base |
| head_tail_dyn | 3094ns | 3134ns | 2723ns | 3009ns | 3407ns | +2.30% |
| kernel_nway | 3023ns | 2963ns | 2738ns | 2934ns | 3301ns | -0.03% |
| kernel_seq | 2920ns | 2889ns | 2681ns | 2860ns | 3130ns | -3.45% |
| nway | 12812ns | 12471ns | 12221ns | 12457ns | 13642ns | +323.62% |
| sequential | 10339ns | 10106ns | 9989ns | 10106ns | 10863ns | +241.84% |
| std_parallel | 21080ns | 20806ns | 18873ns | 20265ns | 23405ns | +596.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2662ns | 2495ns | 2845ns | base | 0.096 |
| head_tail_dyn | 2661ns | 2453ns | 2875ns | -0.05% | 0.096 |
| kernel_nway | 2611ns | 2490ns | 2770ns | -1.92% | 0.098 |
| kernel_seq | 2552ns | 2479ns | 2631ns | -4.12% | 0.100 |
| nway | 12490ns | 11991ns | 13211ns | +369.18% | 0.020 |
| sequential | 10195ns | 9850ns | 10707ns | +282.97% | 0.025 |
| std_parallel | 20686ns | 18532ns | 22924ns | +677.09% | 0.012 |

## Performance model

- Peak throughput: **0.104 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.098 | 93.9% |
| head_tail_dyn | 0.098 | 93.5% |
| kernel_nway | 0.100 | 96.3% |
| kernel_seq | 0.101 | 96.8% |
| nway | 0.021 | 20.2% |
| sequential | 0.026 | 24.6% |
| std_parallel | 0.013 | 12.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3024ns | 3024ns | base |
| head_tail_dyn | 3094ns | 3094ns | +2.30% |
| kernel_nway | 3023ns | 3023ns | -0.03% |
| kernel_seq | 2920ns | 2920ns | -3.45% |
| nway | 12812ns | 12812ns | +323.62% |
| sequential | 10339ns | 10339ns | +241.84% |
| std_parallel | 21080ns | 21080ns | +596.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2612ns | base | --- | [2569, 2717] | --- | --- | --- | --- |
| head_tail_dyn | 2622ns | no significant difference | [-184, +251]ns | [2512, 2802] | no | 0.2379 | 0.2379 | 0 |
| kernel_nway | 2548ns | no significant difference | [-133, +2]ns | [2533, 2592] | no | 0.1155 | 0.0963 | 0 |
| kernel_seq | 2534ns | -40.0ns (-1.5%) | [-197, -2]ns | [2509, 2557] | YES (adj: no) | 0.1155 | 0.0963 | 0 |
| nway | 12171ns | +9590.0ns (+367.1%) | [+9519, +9920]ns | [12077, 12785] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 9969ns | +7345.8ns (+281.2%) | [+7290, +7526]ns | [9961, 10118] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 20471ns | +17771.7ns (+680.3%) | [+16538, +18946]ns | [19098, 21541] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2632ns | -5.5% | -2.9% | -1.9% | +360.9% | +278.6% | +748.8% |
| 1 | 2 | 2795ns | -6.2% | -8.3% | -9.9% | +371.1% | +256.5% | +605.1% |
| 1 | 3 | 2524ns | +11.6% | +0.6% | +0.5% | +378.4% | +308.1% | +660.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.563 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.007 | ok |
| kernel_nway | -0.333 | moderate- |
| kernel_seq | -0.354 | moderate- |
| nway | -0.665 | HIGH- (thermal bounce) |
| sequential | -0.165 | ok |
| std_parallel | -0.087 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 92622.8ns | 2662.0ns | 3479.4% | HIGH |
| head_tail_dyn | 93150.4ns | 2660.6ns | 3501.0% | HIGH |
| kernel_nway | 90752.8ns | 2611.0ns | 3475.8% | HIGH |
| kernel_seq | 91208.3ns | 2552.2ns | 3573.7% | HIGH |
| nway | 93031.5ns | 12489.6ns | 744.9% | HIGH |
| sequential | 90453.8ns | 10194.8ns | 887.3% | HIGH |
| std_parallel | 98795.7ns | 20686.3ns | 477.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2495.0-2845.0 ns)
   2495.0 |##########################
   2512.5 |
   2530.0 |#############
   2547.5 |
   2565.0 |########################################
   2582.5 |#############
   2600.0 |##########################
   2617.5 |
   2635.0 |##########################
   2652.5 |
   2670.0 |
   2687.5 |#############
   2705.0 |
   2722.5 |#############
   2740.0 |#############
   2757.5 |
   2775.0 |
   2792.5 |
   2810.0 |
   2827.5 |#############
  (1 below, 2 above range)

head_tail_dyn (n=18, range 2452.8-2875.3 ns)
   2452.8 |#############
   2473.9 |#############
   2495.0 |#############
   2516.1 |########################################
   2537.3 |
   2558.4 |#############
   2579.5 |
   2600.6 |
   2621.8 |
   2642.9 |
   2664.0 |#############
   2685.2 |
   2706.3 |#############
   2727.4 |
   2748.5 |#############
   2769.7 |
   2790.8 |#############
   2811.9 |##########################
   2833.0 |
   2854.2 |
  (2 below, 3 above range)

kernel_nway (n=18, range 2490.0-2770.0 ns)
   2490.0 |
   2504.0 |##########
   2518.0 |##############################
   2532.0 |########################################
   2546.0 |##########
   2560.0 |####################
   2574.0 |##########
   2588.0 |
   2602.0 |##########
   2616.0 |
   2630.0 |
   2644.0 |##########
   2658.0 |##########
   2672.0 |
   2686.0 |
   2700.0 |
   2714.0 |##########
   2728.0 |
   2742.0 |
   2756.0 |
  (1 below, 1 above range)

kernel_seq (n=18, range 2478.9-2631.1 ns)
   2478.9 |########################################
   2486.5 |
   2494.1 |
   2501.7 |########################################
   2509.3 |########################################
   2517.0 |
   2524.6 |########################################
   2532.2 |########################################
   2539.8 |
   2547.4 |########################################
   2555.0 |####################
   2562.6 |
   2570.2 |
   2577.8 |
   2585.5 |####################
   2593.1 |
   2600.7 |
   2608.3 |
   2615.9 |
   2623.5 |####################
  (1 below, 2 above range)

nway (n=18, range 11990.5-13210.8 ns)
  11990.5 |########################################
  12051.5 |##########
  12112.6 |##############################
  12173.6 |####################
  12234.6 |
  12295.6 |
  12356.6 |
  12417.6 |
  12478.7 |
  12539.7 |##########
  12600.7 |
  12661.7 |
  12722.7 |##########
  12783.7 |##########
  12844.8 |
  12905.8 |
  12966.8 |
  13027.8 |##########
  13088.8 |
  13149.8 |
  (1 below, 3 above range)

sequential (n=18, range 9850.0-10707.2 ns)
   9850.0 |
   9892.9 |#####
   9935.7 |########################################
   9978.6 |##########
  10021.4 |#####
  10064.3 |
  10107.2 |
  10150.0 |
  10192.9 |#####
  10235.7 |
  10278.6 |##########
  10321.5 |
  10364.3 |
  10407.2 |
  10450.0 |
  10492.9 |
  10535.8 |
  10578.6 |
  10621.5 |
  10664.3 |
  (1 below, 2 above range)

std_parallel (n=18, range 18531.7-22924.4 ns)
  18531.7 |
  18751.3 |##########
  18970.9 |########################################
  19190.6 |##########
  19410.2 |
  19629.9 |
  19849.5 |
  20069.1 |
  20288.8 |##############################
  20508.4 |##########
  20728.1 |##########
  20947.7 |
  21167.3 |##########
  21387.0 |
  21606.6 |##########
  21826.2 |
  22045.9 |##########
  22265.5 |##########
  22485.2 |
  22704.8 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=3433.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3560.3% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=3564.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3590.9% of algo (FFI overhead may distort results)
- **nway**: bridge=762.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=904.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=476.3% of algo (FFI overhead may distort results)
