# fold_light, far above cache, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail_dyn is an outlier: 2.4x slower than the field

head_tail_dyn (6.99 ms) is 2.4x the fastest (2.88 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail is fastest but the noisiest (CV 114.6%)

head_tail wins on median (2.88 ms) yet has the highest variance (CV 114.6%), while std_parallel is the steadiest (CV 1.4%, 2.89 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (head_tail, std_parallel) are a dead heat (<1%)

head_tail (2.88 ms) and std_parallel (2.89 ms) differ by 0.18%, inside the noise, even though the wider field spreads 142.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.67)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (2.88 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader head_tail vs stability leader std_parallel (+0% speed for 81.1x steadier)

head_tail is fastest (2.88 ms, CV 114.6%); std_parallel gives up 0.2% median for 81.1x lower variance (CV 1.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### head_tail is inconsistent: worst-20% is 3.0x its best-20%

head_tail's best 20% of batches run at 2.61 ms but its worst 20% at 7.74 ms (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 2880667.6 ns median
- Spread: 2.43x (fastest 2880667.6 ns, slowest 6988298.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 2 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 4441244ns | 2890445ns | 2614983ns | 2844354ns | 7749710ns | base |
| head_tail_dyn | 6469719ns | 6991647ns | 3645177ns | 6926618ns | 7196642ns | +45.67% |
| kernel_nway | 3367564ns | 3025549ns | 2455004ns | 2842033ns | 4612142ns | -24.18% |
| kernel_seq | 4133592ns | 4196875ns | 3794970ns | 4177453ns | 4237113ns | -6.93% |
| nway | 5109159ns | 4820258ns | 2583421ns | 4408770ns | 7422612ns | +15.04% |
| sequential | 3885979ns | 4158770ns | 3280582ns | 3884104ns | 4191490ns | -12.50% |
| std_parallel | 2882951ns | 2889085ns | 2821845ns | 2873544ns | 2927616ns | -35.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4431912ns | 2612395ns | 7740606ns | base | 3.786 |
| head_tail_dyn | 6464222ns | 3640443ns | 7187055ns | +45.86% | 2.595 |
| kernel_nway | 3360000ns | 2452744ns | 4602728ns | -24.19% | 4.993 |
| kernel_seq | 4127381ns | 3788102ns | 4229685ns | -6.87% | 4.065 |
| nway | 5105297ns | 2580560ns | 7416896ns | +15.19% | 3.286 |
| sequential | 3884848ns | 3279432ns | 4190237ns | -12.34% | 4.319 |
| std_parallel | 2879599ns | 2819125ns | 2924284ns | -35.03% | 5.826 |

## Performance model

- Peak throughput: **6.840 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 5.824 | 85.1% |
| head_tail_dyn | 2.401 | 35.1% |
| kernel_nway | 5.565 | 81.4% |
| kernel_seq | 4.004 | 58.5% |
| nway | 3.483 | 50.9% |
| sequential | 4.035 | 59.0% |
| std_parallel | 5.814 | 85.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4441244ns | 4441244ns | base |
| head_tail_dyn | 6469719ns | 6469719ns | +45.67% |
| kernel_nway | 3367564ns | 3367564ns | -24.18% |
| kernel_seq | 4133592ns | 4133592ns | -6.93% |
| nway | 5109159ns | 5109159ns | +15.04% |
| sequential | 3885979ns | 3885979ns | -12.50% |
| std_parallel | 2882951ns | 2882951ns | -35.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2880668ns | base | --- | [2674461, 7740606] | --- | --- | --- | --- |
| head_tail_dyn | 6988298ns | no significant difference | [-1853060, +4372721]ns | [5217314, 7187055] | no | 0.4375 | 0.2188 | 0 |
| kernel_nway | 3014821ns | no significant difference | [-4355488, +1333894]ns | [2462450, 4602728] | no | 0.8250 | 0.6875 | 0 |
| kernel_seq | 4190067ns | no significant difference | [-3565662, +1421332]ns | [3962390, 4229685] | no | 0.4375 | 0.2188 | 0 |
| nway | 4816918ns | no significant difference | [-4229005, +4272002]ns | [3082077, 7416896] | no | 0.4375 | 0.2188 | 0 |
| sequential | 4157712ns | no significant difference | [-4434011, +1499344]ns | [3306595, 4190237] | no | 0.8250 | 0.6875 | 0 |
| std_parallel | 2885818ns | no significant difference | [-4882383, +249822]ns | [2828696, 2924284] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2674461ns | +163.5% | +10.2% | +49.1% | +80.1% | +56.1% | +9.3% |
| 1 | 2 | 2880668ns | +81.1% | -12.2% | +46.5% | +120.1% | +44.9% | -0.8% |
| 1 | 3 | 7740606ns | -7.9% | -40.5% | -46.1% | -46.3% | -57.3% | -63.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.146 | ok |
| head_tail_dyn | -0.666 | HIGH- (thermal bounce) |
| kernel_nway | -0.287 | moderate- |
| kernel_seq | -0.281 | moderate- |
| nway | -0.609 | HIGH- (thermal bounce) |
| sequential | -0.167 | ok |
| std_parallel | -0.181 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 4528372.2ns | 4431911.5ns | 102.2% | HIGH |
| head_tail_dyn | 6060522.2ns | 6464222.4ns | 93.8% | HIGH |
| kernel_nway | 3721274.5ns | 3359999.9ns | 110.8% | HIGH |
| kernel_seq | 4141041.7ns | 4127380.7ns | 100.3% | HIGH |
| nway | 5311640.5ns | 5105297.0ns | 104.0% | HIGH |
| sequential | 3890079.2ns | 3884847.9ns | 100.1% | HIGH |
| std_parallel | 2890883.1ns | 2879599.4ns | 100.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 2612395.4-7740605.6 ns)
  2612395.4 |########################################
  2868805.9 |#############
  3125216.4 |
  3381626.9 |
  3638037.4 |#############
  3894447.9 |
  4150858.4 |
  4407269.0 |
  4663679.5 |
  4920090.0 |
  5176500.5 |
  5432911.0 |
  5689321.5 |
  5945732.0 |
  6202142.5 |
  6458553.0 |
  6714963.5 |
  6971374.0 |
  7227784.5 |
  7484195.0 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 3640442.7-7187055.2 ns)
  3640442.7 |####################
  3817773.3 |
  3995104.0 |
  4172434.6 |
  4349765.2 |
  4527095.8 |
  4704426.5 |
  4881757.1 |
  5059087.7 |
  5236418.3 |
  5413749.0 |
  5591079.6 |
  5768410.2 |
  5945740.9 |
  6123071.5 |
  6300402.1 |
  6477732.7 |
  6655063.4 |####################
  6832394.0 |########################################
  7009724.6 |####################
  (0 below, 1 above range)

kernel_nway (n=6, range 2452743.7-4602728.2 ns)
  2452743.7 |########################################
  2560242.9 |####################
  2667742.2 |
  2775241.4 |
  2882740.6 |
  2990239.8 |
  3097739.1 |
  3205238.3 |
  3312737.5 |
  3420236.7 |########################################
  3527736.0 |
  3635235.2 |
  3742734.4 |
  3850233.6 |
  3957732.9 |
  4065232.1 |
  4172731.3 |
  4280230.5 |
  4387729.8 |
  4495229.0 |
  (0 below, 1 above range)

kernel_seq (n=6, range 3788102.2-4229685.2 ns)
  3788102.2 |####################
  3810181.3 |
  3832260.5 |
  3854339.6 |
  3876418.8 |
  3898497.9 |
  3920577.1 |
  3942656.2 |
  3964735.4 |
  3986814.5 |
  4008893.7 |
  4030972.8 |
  4053052.0 |
  4075131.1 |
  4097210.3 |
  4119289.4 |####################
  4141368.6 |
  4163447.7 |
  4185526.9 |########################################
  4207606.0 |####################
  (0 below, 1 above range)

nway (n=6, range 2580560.3-7416895.8 ns)
  2580560.3 |########################################
  2822377.1 |
  3064193.9 |
  3306010.6 |
  3547827.4 |########################################
  3789644.2 |
  4031461.0 |
  4273277.7 |
  4515094.5 |########################################
  4756911.3 |
  4998728.1 |########################################
  5240544.8 |
  5482361.6 |
  5724178.4 |########################################
  5965995.2 |
  6207811.9 |
  6449628.7 |
  6691445.5 |
  6933262.3 |
  7175079.0 |
  (0 below, 1 above range)

sequential (n=6, range 3279431.8-4190237.2 ns)
  3279431.8 |#############
  3324972.1 |#############
  3370512.3 |
  3416052.6 |
  3461592.9 |
  3507133.2 |
  3552673.4 |
  3598213.7 |
  3643754.0 |
  3689294.3 |
  3734834.5 |
  3780374.8 |
  3825915.1 |
  3871455.3 |
  3916995.6 |
  3962535.9 |
  4008076.2 |
  4053616.4 |
  4099156.7 |
  4144697.0 |########################################
  (0 below, 1 above range)

std_parallel (n=6, range 2819125.1-2924283.6 ns)
  2819125.1 |########################################
  2824383.0 |
  2829641.0 |
  2834898.9 |########################################
  2840156.8 |
  2845414.7 |
  2850672.7 |
  2855930.6 |
  2861188.5 |
  2866446.4 |
  2871704.4 |
  2876962.3 |########################################
  2882220.2 |
  2887478.2 |
  2892736.1 |########################################
  2897994.0 |
  2903251.9 |
  2908509.9 |########################################
  2913767.8 |
  2919025.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=74.5% (high variance, measurements may be unstable)
- **head_tail**: bridge=102.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=77.7% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=33.6% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=125.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.3% of algo (FFI overhead may distort results)
- **nway**: CV=40.3% (high variance, measurements may be unstable)
- **nway**: bridge=109.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.0% of algo (FFI overhead may distort results)
