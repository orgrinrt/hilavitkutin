# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 61% faster than the next best (head_tail_dyn)

kernel_seq (2.56 us) leads head_tail_dyn (4.11 us) by 61%, a clear separation rather than a photo finish. CV 6.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 38% (significant)

kernel_seq is -1.63 us (38%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 32.2x slower than the field

std_parallel (82.42 us) is 32.2x the fastest (2.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.65)

sequential's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 32.2x the fastest

Fastest kernel_seq (2.56 us) to slowest std_parallel (82.42 us): 32.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.8x its best-20%

kernel_nway's best 20% of batches run at 14.63 us but its worst 20% at 26.06 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 2556.7 ns median (-40.6% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 32.24x (fastest 2556.7 ns, slowest 82416.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 8 |
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
| head_tail | 4970ns | 4935ns | 4618ns | 4873ns | 5293ns | base |
| head_tail_dyn | 4845ns | 4722ns | 4584ns | 4699ns | 5193ns | -2.53% |
| kernel_nway | 19847ns | 16101ns | 15248ns | 16817ns | 26693ns | +299.32% |
| kernel_seq | 2955ns | 2855ns | 2714ns | 2827ns | 3266ns | -40.55% |
| nway | 51906ns | 51619ns | 48637ns | 51159ns | 54661ns | +944.33% |
| sequential | 10326ns | 10221ns | 9905ns | 10196ns | 10732ns | +107.76% |
| std_parallel | 84012ns | 82786ns | 75626ns | 82286ns | 90793ns | +1590.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4346ns | 4009ns | 4645ns | base | 0.059 |
| head_tail_dyn | 4241ns | 3980ns | 4609ns | -2.42% | 0.060 |
| kernel_nway | 19224ns | 14629ns | 26058ns | +342.37% | 0.013 |
| kernel_seq | 2617ns | 2477ns | 2796ns | -39.77% | 0.098 |
| nway | 51346ns | 48076ns | 54093ns | +1081.56% | 0.005 |
| sequential | 10178ns | 9766ns | 10565ns | +134.20% | 0.025 |
| std_parallel | 83632ns | 75247ns | 90413ns | +1824.50% | 0.003 |

## Performance model

- Peak throughput: **0.103 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.059 | 57.5% |
| head_tail_dyn | 0.062 | 60.3% |
| kernel_nway | 0.017 | 16.0% |
| kernel_seq | 0.100 | 96.9% |
| nway | 0.005 | 4.9% |
| sequential | 0.025 | 24.5% |
| std_parallel | 0.003 | 3.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4970ns | 4970ns | base |
| head_tail_dyn | 4845ns | 4845ns | -2.53% |
| kernel_nway | 19847ns | 19847ns | +299.32% |
| kernel_seq | 2955ns | 2955ns | -40.55% |
| nway | 51906ns | 51906ns | +944.33% |
| sequential | 10326ns | 10326ns | +107.76% |
| std_parallel | 84012ns | 84012ns | +1590.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4305ns | base | --- | [4176, 4390] | --- | --- | --- | --- |
| head_tail_dyn | 4108ns | no significant difference | [-252, +57]ns | [4042, 4313] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 15481ns | +11288.4ns (+262.2%) | [+10553, +17218]ns | [14862, 21318] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2557ns | -1629.1ns (-37.8%) | [-1807, -1522]ns | [2525, 2634] | YES | 0.0000 | 0.0000 | 0 |
| nway | 51060ns | +46635.8ns (+1083.3%) | [+45326, +48125]ns | [49783, 52472] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10094ns | +5761.6ns (+133.8%) | [+5617, +6057]ns | [9997, 10300] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 82417ns | +77916.7ns (+1809.9%) | [+76258, +81104]ns | [80432, 85132] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4351ns | +0.9% | +430.1% | -36.5% | +1061.6% | +130.7% | +1892.8% |
| 1 | 2 | 4630ns | -12.3% | +225.4% | -45.4% | +974.9% | +122.9% | +1628.8% |
| 1 | 3 | 4085ns | +0.0% | +265.6% | -38.1% | +1203.9% | +144.1% | +1941.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.508 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.209 | moderate- |
| kernel_nway | -0.159 | ok |
| kernel_seq | -0.172 | ok |
| nway | -0.300 | moderate- |
| sequential | -0.646 | HIGH- (thermal bounce) |
| std_parallel | -0.501 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 99389.9ns | 4345.6ns | 2287.1% | HIGH |
| head_tail_dyn | 96268.7ns | 4240.6ns | 2270.1% | HIGH |
| kernel_nway | 114988.1ns | 19224.0ns | 598.1% | HIGH |
| kernel_seq | 90566.7ns | 2617.4ns | 3460.2% | HIGH |
| nway | 109854.5ns | 51346.2ns | 213.9% | HIGH |
| sequential | 90024.2ns | 10177.5ns | 884.5% | HIGH |
| std_parallel | 135374.5ns | 83632.0ns | 161.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 4008.9-4645.3 ns)
   4008.9 |
   4040.7 |#############
   4072.5 |
   4104.3 |#############
   4136.2 |#############
   4168.0 |#############
   4199.8 |
   4231.6 |
   4263.4 |########################################
   4295.3 |##########################
   4327.1 |
   4358.9 |########################################
   4390.7 |#############
   4422.5 |
   4454.4 |
   4486.2 |
   4518.0 |
   4549.8 |
   4581.6 |
   4613.5 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 3980.0-4608.9 ns)
   3980.0 |##########################
   4011.4 |##########################
   4042.9 |
   4074.3 |########################################
   4105.8 |##########################
   4137.2 |#############
   4168.6 |
   4200.1 |
   4231.5 |
   4263.0 |#############
   4294.4 |
   4325.9 |##########################
   4357.3 |
   4388.8 |
   4420.2 |#############
   4451.7 |
   4483.1 |
   4514.6 |
   4546.0 |
   4577.5 |
  (2 below, 2 above range)

kernel_nway (n=18, range 14628.9-26057.8 ns)
  14628.9 |########################################
  15200.3 |########################
  15771.8 |
  16343.2 |
  16914.6 |
  17486.1 |
  18057.5 |
  18629.0 |
  19200.4 |########
  19771.9 |########
  20343.3 |########
  20914.8 |
  21486.2 |
  22057.7 |################
  22629.1 |
  23200.5 |
  23772.0 |########
  24343.4 |
  24914.9 |
  25486.3 |
  (2 below, 2 above range)

kernel_seq (n=18, range 2477.2-2796.1 ns)
   2477.2 |
   2493.2 |##########
   2509.1 |####################
   2525.1 |########################################
   2541.0 |
   2556.9 |
   2572.9 |##############################
   2588.8 |
   2604.8 |
   2620.7 |##########
   2636.7 |##########
   2652.6 |
   2668.6 |
   2684.5 |##########
   2700.4 |
   2716.4 |
   2732.3 |
   2748.3 |
   2764.2 |
   2780.2 |
  (2 below, 3 above range)

nway (n=18, range 48075.5-54092.8 ns)
  48075.5 |
  48376.4 |########################################
  48677.3 |
  48978.1 |########################################
  49279.0 |########################################
  49579.8 |
  49880.7 |########################################
  50181.6 |########################################
  50482.4 |########################################
  50783.3 |########################################
  51084.1 |########################################
  51385.0 |########################################
  51685.9 |########################################
  51986.7 |########################################
  52287.6 |
  52588.5 |########################################
  52889.3 |
  53190.2 |
  53491.0 |########################################
  53791.9 |########################################
  (2 below, 2 above range)

sequential (n=18, range 9766.1-10565.0 ns)
   9766.1 |####################
   9806.0 |
   9846.0 |
   9885.9 |
   9925.9 |##########
   9965.8 |##########
  10005.8 |########################################
  10045.7 |
  10085.7 |
  10125.6 |####################
  10165.6 |##########
  10205.5 |##########
  10245.5 |
  10285.4 |
  10325.3 |
  10365.3 |
  10405.2 |####################
  10445.2 |####################
  10485.1 |
  10525.1 |
  (1 below, 1 above range)

std_parallel (n=18, range 75246.7-90412.8 ns)
  75246.7 |
  76005.0 |
  76763.3 |
  77521.6 |
  78279.9 |##########################
  79038.2 |#############
  79796.5 |
  80554.8 |
  81313.1 |########################################
  82071.4 |##########################
  82829.7 |
  83588.0 |#############
  84346.3 |##########################
  85104.6 |#############
  85862.9 |
  86621.3 |
  87379.6 |#############
  88137.9 |
  88896.2 |#############
  89654.5 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=2285.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2356.5% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=32.3% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=737.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3525.4% of algo (FFI overhead may distort results)
- **nway**: bridge=213.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=894.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=166.7% of algo (FFI overhead may distort results)
