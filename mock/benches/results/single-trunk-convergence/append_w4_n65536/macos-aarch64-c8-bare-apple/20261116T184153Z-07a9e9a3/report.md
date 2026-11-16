# append, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway is an outlier: 6.1x slower than the field

nway (58.80 us) is 6.1x the fastest (9.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (kernel_seq, head_tail_dyn) are a dead heat (<1%)

kernel_seq (9.56 us) and head_tail_dyn (9.57 us) differ by 0.07%, inside the noise, even though the wider field spreads 514.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.66)

kernel_seq's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 6.1x the fastest

Fastest kernel_seq (9.56 us) to slowest nway (58.80 us): 6.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.5x its best-20%

kernel_nway's best 20% of batches run at 16.52 us but its worst 20% at 24.85 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 9562.5 ns median (-0.5% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 6.15x (fastest 9562.5 ns, slowest 58795.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 10397ns | 10352ns | 9682ns | 10175ns | 11088ns | base |
| head_tail_dyn | 10359ns | 10308ns | 10000ns | 10241ns | 10715ns | -0.37% |
| kernel_nway | 21930ns | 21648ns | 17342ns | 20952ns | 25691ns | +110.92% |
| kernel_seq | 10238ns | 10246ns | 9789ns | 10166ns | 10572ns | -1.53% |
| nway | 59390ns | 59410ns | 54625ns | 58847ns | 62586ns | +471.21% |
| sequential | 56585ns | 56492ns | 55917ns | 56412ns | 57180ns | +444.23% |
| std_parallel | 52597ns | 52046ns | 49806ns | 51892ns | 55051ns | +405.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9707ns | 9406ns | 10051ns | base | 6.751 |
| head_tail_dyn | 9664ns | 9435ns | 9946ns | -0.44% | 6.781 |
| kernel_nway | 21071ns | 16517ns | 24847ns | +117.07% | 3.110 |
| kernel_seq | 9599ns | 9425ns | 9774ns | -1.12% | 6.828 |
| nway | 58533ns | 53632ns | 61647ns | +503.00% | 1.120 |
| sequential | 56065ns | 55625ns | 56567ns | +477.58% | 1.169 |
| std_parallel | 51508ns | 48782ns | 53678ns | +430.63% | 1.272 |

## Performance model

- Peak throughput: **6.968 Gops/s** (head_tail; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.821 | 97.9% |
| head_tail_dyn | 6.849 | 98.3% |
| kernel_nway | 3.151 | 45.2% |
| kernel_seq | 6.853 | 98.4% |
| nway | 1.115 | 16.0% |
| sequential | 1.171 | 16.8% |
| std_parallel | 1.279 | 18.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 10397ns | 10397ns | base |
| head_tail_dyn | 10359ns | 10359ns | -0.37% |
| kernel_nway | 21930ns | 21930ns | +110.92% |
| kernel_seq | 10238ns | 10238ns | -1.53% |
| nway | 59390ns | 59390ns | +471.21% |
| sequential | 56585ns | 56585ns | +444.23% |
| std_parallel | 52597ns | 52597ns | +405.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 9608ns | base | --- | [9490, 9775] | --- | --- | --- | --- |
| head_tail_dyn | 9569ns | no significant difference | [-192, +138]ns | [9500, 9702] | no | 1.0000 | 1.0000 | 1 |
| kernel_nway | 20796ns | +11133.4ns (+115.9%) | [+9092, +13227]ns | [18712, 22867] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 9562ns | no significant difference | [-258, +52]ns | [9504, 9633] | no | 0.3988 | 0.3323 | 1 |
| nway | 58796ns | +49187.5ns (+511.9%) | [+47327, +50302]ns | [56896, 60177] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 55965ns | +46279.2ns (+481.7%) | [+46004, +46550]ns | [55712, 56140] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 51246ns | +41662.5ns (+433.6%) | [+40781, +43621]ns | [50204, 53256] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9640ns | -1.3% | +137.2% | -1.2% | +496.0% | +480.6% | +427.7% |
| 1 | 2 | 9640ns | +1.1% | +98.5% | +0.5% | +528.1% | +482.5% | +425.0% |
| 1 | 3 | 9608ns | -0.1% | +126.7% | -1.1% | +502.3% | +479.6% | +454.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.166 | ok |
| head_tail_dyn | -0.585 | HIGH- (thermal bounce) |
| kernel_nway | -0.614 | HIGH- (thermal bounce) |
| kernel_seq | -0.659 | HIGH- (thermal bounce) |
| nway | -0.656 | HIGH- (thermal bounce) |
| sequential | -0.429 | moderate- |
| std_parallel | -0.220 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 443886.6ns | 9706.9ns | 4572.9% | HIGH |
| head_tail_dyn | 442468.3ns | 9664.1ns | 4578.5% | HIGH |
| kernel_nway | 453531.0ns | 21070.8ns | 2152.4% | HIGH |
| kernel_seq | 442909.9ns | 9598.6ns | 4614.3% | HIGH |
| nway | 470641.7ns | 58532.6ns | 804.1% | HIGH |
| sequential | 467623.8ns | 56065.3ns | 834.1% | HIGH |
| std_parallel | 472179.4ns | 51507.9ns | 916.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 9405.6-10050.7 ns)
   9405.6 |########################################
   9437.8 |
   9470.1 |##########################
   9502.3 |#############
   9534.6 |#############
   9566.8 |#############
   9599.1 |
   9631.4 |#############
   9663.6 |
   9695.9 |
   9728.1 |##########################
   9760.4 |##########################
   9792.6 |#############
   9824.9 |
   9857.1 |
   9889.4 |
   9921.7 |
   9953.9 |
   9986.2 |#############
  10018.4 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 9434.7-9945.8 ns)
   9434.7 |#############
   9460.3 |########################################
   9485.8 |#############
   9511.4 |##########################
   9537.0 |#############
   9562.5 |
   9588.1 |#############
   9613.6 |
   9639.2 |#############
   9664.7 |#############
   9690.3 |##########################
   9715.8 |
   9741.4 |
   9767.0 |
   9792.5 |
   9818.1 |#############
   9843.6 |#############
   9869.2 |
   9894.7 |
   9920.3 |
  (1 below, 2 above range)

kernel_nway (n=18, range 16516.7-24846.5 ns)
  16516.7 |####################
  16933.2 |
  17349.7 |
  17766.1 |####################
  18182.6 |########################################
  18599.1 |####################
  19015.6 |####################
  19432.1 |
  19848.6 |####################
  20265.1 |####################
  20681.6 |
  21098.1 |####################
  21514.6 |####################
  21931.1 |####################
  22347.6 |####################
  22764.1 |
  23180.5 |####################
  23597.0 |
  24013.5 |####################
  24430.0 |
  (1 below, 3 above range)

kernel_seq (n=18, range 9425.0-9773.6 ns)
   9425.0 |#############
   9442.5 |#############
   9459.9 |
   9477.3 |
   9494.8 |########################################
   9512.2 |##########################
   9529.6 |#############
   9547.0 |
   9564.5 |##########################
   9581.9 |
   9599.3 |
   9616.8 |##########################
   9634.2 |#############
   9651.6 |
   9669.0 |
   9686.5 |
   9703.9 |
   9721.3 |##########################
   9738.8 |
   9756.2 |
  (1 below, 2 above range)

nway (n=18, range 53631.9-61647.2 ns)
  53631.9 |
  54032.7 |
  54433.5 |
  54834.2 |#############
  55235.0 |
  55635.8 |
  56036.5 |#############
  56437.3 |#############
  56838.0 |
  57238.8 |#############
  57639.6 |
  58040.3 |##########################
  58441.1 |#############
  58841.9 |##########################
  59242.6 |#############
  59643.4 |
  60044.2 |#############
  60444.9 |
  60845.7 |########################################
  61246.5 |
  (2 below, 2 above range)

sequential (n=18, range 55625.0-56566.7 ns)
  55625.0 |#############
  55672.1 |########################################
  55719.2 |#############
  55766.3 |
  55813.4 |#############
  55860.4 |
  55907.5 |##########################
  55954.6 |#############
  56001.7 |#############
  56048.8 |#############
  56095.9 |#############
  56142.9 |
  56190.0 |##########################
  56237.1 |
  56284.2 |
  56331.3 |
  56378.4 |
  56425.4 |
  56472.5 |
  56519.6 |
  (1 below, 3 above range)

std_parallel (n=18, range 48781.9-53677.8 ns)
  48781.9 |
  49026.7 |####################
  49271.5 |
  49516.3 |
  49761.1 |########################################
  50005.9 |
  50250.7 |
  50495.5 |########################################
  50740.3 |
  50985.1 |########################################
  51229.9 |########################################
  51474.7 |
  51719.4 |
  51964.2 |
  52209.0 |
  52453.8 |
  52698.6 |####################
  52943.4 |####################
  53188.2 |########################################
  53433.0 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=4617.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=4625.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=2169.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=4635.1% of algo (FFI overhead may distort results)
- **nway**: bridge=799.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=836.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=921.1% of algo (FFI overhead may distort results)
