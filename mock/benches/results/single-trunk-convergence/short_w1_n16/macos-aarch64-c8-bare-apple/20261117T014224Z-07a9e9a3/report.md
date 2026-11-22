# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 7.5x slower than the field

std_parallel (18.64 us) is 7.5x the fastest (2.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway is fastest but the noisiest (CV 8.6%)

kernel_nway wins on median (2.50 us) yet has the highest variance (CV 8.6%), while sequential is the steadiest (CV 2.7%, 9.97 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, kernel_seq, head_tail, head_tail_dyn} vs {sequential, nway, std_parallel} (290% apart)

The field splits into a fast tier {kernel_nway, kernel_seq, head_tail, head_tail_dyn} and a slow tier {sequential, nway, std_parallel} with a 290% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.5x the fastest

Fastest kernel_nway (2.50 us) to slowest std_parallel (18.64 us): 7.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 2500.0 ns median (-1.9% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 7.46x (fastest 2500.0 ns, slowest 18642.5 ns)

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
| head_tail | 3017ns | 2888ns | 2684ns | 2866ns | 3411ns | base |
| head_tail_dyn | 2937ns | 2921ns | 2690ns | 2891ns | 3128ns | -2.68% |
| kernel_nway | 2923ns | 2858ns | 2675ns | 2830ns | 3186ns | -3.13% |
| kernel_seq | 2973ns | 2828ns | 2707ns | 2840ns | 3306ns | -1.47% |
| nway | 12628ns | 12485ns | 12228ns | 12473ns | 13060ns | +318.51% |
| sequential | 10135ns | 10115ns | 9771ns | 10071ns | 10412ns | +235.88% |
| std_parallel | 19199ns | 18978ns | 18303ns | 18950ns | 20020ns | +536.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2642ns | 2459ns | 2889ns | base | 0.006 |
| head_tail_dyn | 2553ns | 2443ns | 2648ns | -3.38% | 0.006 |
| kernel_nway | 2557ns | 2437ns | 2716ns | -3.20% | 0.006 |
| kernel_seq | 2605ns | 2473ns | 2780ns | -1.38% | 0.006 |
| nway | 12305ns | 11978ns | 12676ns | +365.79% | 0.001 |
| sequential | 9991ns | 9636ns | 10268ns | +278.20% | 0.002 |
| std_parallel | 18846ns | 17940ns | 19662ns | +613.36% | 0.001 |

## Performance model

- Peak throughput: **0.007 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.006 | 95.6% |
| head_tail_dyn | 0.006 | 95.2% |
| kernel_nway | 0.006 | 97.5% |
| kernel_seq | 0.006 | 96.0% |
| nway | 0.001 | 20.0% |
| sequential | 0.002 | 24.4% |
| std_parallel | 0.001 | 13.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3017ns | 3017ns | base |
| head_tail_dyn | 2937ns | 2937ns | -2.68% |
| kernel_nway | 2923ns | 2923ns | -3.13% |
| kernel_seq | 2973ns | 2973ns | -1.47% |
| nway | 12628ns | 12628ns | +318.51% |
| sequential | 10135ns | 10135ns | +235.88% |
| std_parallel | 19199ns | 19199ns | +536.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2548ns | base | --- | [2522, 2684] | --- | --- | --- | --- |
| head_tail_dyn | 2559ns | no significant difference | [-170, +66]ns | [2475, 2625] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 2500ns | no significant difference | [-226, +38]ns | [2473, 2537] | no | 0.9774 | 0.8145 | 0 |
| kernel_seq | 2538ns | no significant difference | [-162, +61]ns | [2509, 2606] | no | 0.7210 | 0.4807 | 0 |
| nway | 12189ns | +9650.0ns (+378.7%) | [+9435, +9701]ns | [12136, 12248] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 9974ns | +7334.2ns (+287.8%) | [+7265, +7418]ns | [9867, 10110] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 18642ns | +16108.4ns (+632.1%) | [+15867, +16412]ns | [18479, 18996] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2531ns | +0.1% | +0.0% | -1.3% | +380.2% | +286.2% | +638.9% |
| 1 | 2 | 2903ns | -14.8% | -15.0% | -12.6% | +324.0% | +249.5% | +548.8% |
| 1 | 3 | 2518ns | +5.0% | -0.3% | +4.6% | +383.1% | +295.2% | +634.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.666 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.395 | moderate- |
| kernel_nway | -0.589 | HIGH- (thermal bounce) |
| kernel_seq | -0.034 | ok |
| nway | -0.665 | HIGH- (thermal bounce) |
| sequential | -0.522 | HIGH- (thermal bounce) |
| std_parallel | -0.432 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 1/3
- **kernel_nway**: won 2/3, lost 0/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 91481.6ns | 2641.8ns | 3462.8% | HIGH |
| head_tail_dyn | 91814.3ns | 2552.5ns | 3597.0% | HIGH |
| kernel_nway | 90745.2ns | 2557.3ns | 3548.4% | HIGH |
| kernel_seq | 91251.9ns | 2605.3ns | 3502.6% | HIGH |
| nway | 92769.2ns | 12305.5ns | 753.9% | HIGH |
| sequential | 89701.4ns | 9991.5ns | 897.8% | HIGH |
| std_parallel | 94440.9ns | 18845.9ns | 501.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2459.4-2888.6 ns)
   2459.4 |##########################
   2480.9 |
   2502.4 |##########################
   2523.8 |########################################
   2545.3 |########################################
   2566.7 |#############
   2588.2 |
   2609.6 |
   2631.1 |#############
   2652.6 |
   2674.0 |
   2695.5 |
   2716.9 |#############
   2738.4 |
   2759.9 |
   2781.3 |
   2802.8 |
   2824.2 |
   2845.7 |#############
   2867.2 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 2443.3-2648.3 ns)
   2443.3 |
   2453.6 |##########
   2463.8 |####################
   2474.1 |##########
   2484.3 |##########
   2494.6 |
   2504.8 |##########
   2515.1 |
   2525.3 |
   2535.6 |
   2545.8 |##########
   2556.1 |
   2566.3 |####################
   2576.6 |
   2586.8 |
   2597.1 |
   2607.3 |
   2617.6 |########################################
   2627.8 |
   2638.1 |
  (2 below, 3 above range)

kernel_nway (n=18, range 2436.7-2716.4 ns)
   2436.7 |
   2450.7 |##########################
   2464.6 |#############
   2478.6 |##########################
   2492.6 |########################################
   2506.6 |#############
   2520.6 |#############
   2534.6 |##########################
   2548.6 |#############
   2562.5 |
   2576.5 |
   2590.5 |#############
   2604.5 |
   2618.5 |
   2632.5 |
   2646.5 |
   2660.5 |#############
   2674.4 |
   2688.4 |
   2702.4 |
  (2 below, 1 above range)

kernel_seq (n=18, range 2473.3-2780.3 ns)
   2473.3 |
   2488.7 |########################################
   2504.0 |##########################
   2519.4 |##########################
   2534.7 |#############
   2550.1 |
   2565.4 |#############
   2580.8 |##########################
   2596.1 |
   2611.5 |#############
   2626.8 |##########################
   2642.1 |
   2657.5 |
   2672.8 |
   2688.2 |
   2703.5 |
   2718.9 |
   2734.2 |
   2749.6 |
   2764.9 |
  (2 below, 2 above range)

nway (n=18, range 11978.3-12675.8 ns)
  11978.3 |
  12013.2 |##########
  12048.1 |
  12083.0 |
  12117.8 |########################################
  12152.7 |##########
  12187.6 |########################################
  12222.5 |####################
  12257.3 |##########
  12292.2 |
  12327.1 |##########
  12362.0 |
  12396.8 |
  12431.7 |
  12466.6 |
  12501.5 |
  12536.3 |
  12571.2 |
  12606.1 |
  12641.0 |
  (2 below, 2 above range)

sequential (n=18, range 9635.6-10267.5 ns)
   9635.6 |
   9667.2 |
   9698.8 |#############
   9730.4 |
   9762.0 |
   9793.6 |
   9825.2 |##########################
   9856.7 |##########################
   9888.3 |#############
   9919.9 |
   9951.5 |#############
   9983.1 |##########################
  10014.7 |#############
  10046.3 |
  10077.9 |
  10109.5 |#############
  10141.1 |########################################
  10172.7 |
  10204.3 |
  10235.9 |#############
  (2 below, 1 above range)

std_parallel (n=18, range 17940.0-19662.0 ns)
  17940.0 |#############
  18026.1 |
  18112.2 |
  18198.3 |
  18284.4 |#############
  18370.5 |#############
  18456.6 |########################################
  18542.7 |#############
  18628.8 |##########################
  18714.9 |#############
  18801.0 |
  18887.1 |##########################
  18973.2 |#############
  19059.3 |
  19145.4 |
  19231.5 |
  19317.6 |
  19403.7 |
  19489.8 |#############
  19575.9 |#############
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=3588.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3583.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=3577.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3579.7% of algo (FFI overhead may distort results)
- **nway**: bridge=761.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=903.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=508.2% of algo (FFI overhead may distort results)
