# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 62% faster than the next best (head_tail)

kernel_seq (2.53 us) leads head_tail (4.09 us) by 62%, a clear separation rather than a photo finish. CV 7.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 38% (significant)

kernel_seq is -1.56 us (38%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 12.4x slower than the field

std_parallel (31.23 us) is 12.4x the fastest (2.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq is fastest but the noisiest (CV 7.9%)

kernel_seq wins on median (2.53 us) yet has the highest variance (CV 7.9%), while nway is the steadiest (CV 3.0%, 17.91 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sequential shows alternating (throttle bounce) (autocorr -0.67)

sequential's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 12.4x the fastest

Fastest kernel_seq (2.53 us) to slowest std_parallel (31.23 us): 12.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2527.5 ns median (-38.3% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 12.35x (fastest 2527.5 ns, slowest 31225.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 2 |
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
| head_tail | 4796ns | 4725ns | 4565ns | 4705ns | 5050ns | base |
| head_tail_dyn | 4780ns | 4748ns | 4583ns | 4733ns | 4949ns | -0.34% |
| kernel_nway | 4876ns | 4792ns | 4652ns | 4766ns | 5153ns | +1.65% |
| kernel_seq | 3003ns | 2947ns | 2733ns | 2935ns | 3240ns | -37.39% |
| nway | 18584ns | 18438ns | 18143ns | 18376ns | 19117ns | +287.45% |
| sequential | 10639ns | 10372ns | 10077ns | 10370ns | 11322ns | +121.80% |
| std_parallel | 32761ns | 31604ns | 30677ns | 31515ns | 35672ns | +583.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4175ns | 3944ns | 4403ns | base | 0.004 |
| head_tail_dyn | 4166ns | 3964ns | 4334ns | -0.21% | 0.004 |
| kernel_nway | 4238ns | 4040ns | 4462ns | +1.51% | 0.004 |
| kernel_seq | 2585ns | 2452ns | 2755ns | -38.09% | 0.006 |
| nway | 18063ns | 17642ns | 18588ns | +332.67% | 0.001 |
| sequential | 10500ns | 9941ns | 11186ns | +151.50% | 0.002 |
| std_parallel | 32378ns | 30316ns | 35273ns | +675.56% | 0.000 |

## Performance model

- Peak throughput: **0.007 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.004 | 59.9% |
| head_tail_dyn | 0.004 | 59.2% |
| kernel_nway | 0.004 | 58.7% |
| kernel_seq | 0.006 | 97.0% |
| nway | 0.001 | 13.7% |
| sequential | 0.002 | 24.0% |
| std_parallel | 0.001 | 7.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4796ns | 4796ns | base |
| head_tail_dyn | 4780ns | 4780ns | -0.34% |
| kernel_nway | 4876ns | 4876ns | +1.65% |
| kernel_seq | 3003ns | 3003ns | -37.39% |
| nway | 18584ns | 18584ns | +287.45% |
| sequential | 10639ns | 10639ns | +121.80% |
| std_parallel | 32761ns | 32761ns | +583.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4093ns | base | --- | [4052, 4271] | --- | --- | --- | --- |
| head_tail_dyn | 4139ns | no significant difference | [-148, +182]ns | [4088, 4191] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 4178ns | no significant difference | [-56, +186]ns | [4120, 4235] | no | 0.8145 | 0.8145 | 0 |
| kernel_seq | 2528ns | -1563.3ns (-38.2%) | [-1740, -1458]ns | [2493, 2565] | YES | 0.0000 | 0.0000 | 0 |
| nway | 17915ns | +13813.3ns (+337.5%) | [+13677, +13993]ns | [17742, 18129] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10238ns | +6138.3ns (+150.0%) | [+6079, +6435]ns | [10178, 10710] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 31225ns | +27156.7ns (+663.4%) | [+26740, +29494]ns | [30849, 33725] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4142ns | -1.2% | +1.3% | -38.6% | +330.5% | +146.0% | +660.2% |
| 1 | 2 | 4298ns | -3.4% | -3.7% | -41.0% | +321.8% | +164.6% | +619.9% |
| 1 | 3 | 4068ns | +4.2% | +3.0% | -38.7% | +341.3% | +150.7% | +671.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.601 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.004 | ok |
| kernel_nway | -0.663 | HIGH- (thermal bounce) |
| kernel_seq | -0.097 | ok |
| nway | -0.563 | HIGH- (thermal bounce) |
| sequential | -0.667 | HIGH- (thermal bounce) |
| std_parallel | -0.649 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 98030.4ns | 4174.8ns | 2348.1% | HIGH |
| head_tail_dyn | 96526.5ns | 4165.9ns | 2317.1% | HIGH |
| kernel_nway | 97307.5ns | 4238.1ns | 2296.0% | HIGH |
| kernel_seq | 93109.4ns | 2584.8ns | 3602.2% | HIGH |
| nway | 92595.0ns | 18063.3ns | 512.6% | HIGH |
| sequential | 91330.7ns | 10499.6ns | 869.8% | HIGH |
| std_parallel | 104956.8ns | 32378.0ns | 324.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 3944.5-4403.0 ns)
   3944.5 |
   3967.4 |
   3990.3 |#############
   4013.3 |#############
   4036.2 |##########################
   4059.1 |#############
   4082.0 |########################################
   4105.0 |
   4127.9 |
   4150.8 |
   4173.8 |#############
   4196.7 |
   4219.6 |#############
   4242.5 |#############
   4265.5 |
   4288.4 |##########################
   4311.3 |##########################
   4334.2 |
   4357.2 |
   4380.1 |
  (2 below, 1 above range)

head_tail_dyn (n=18, range 3964.4-4333.6 ns)
   3964.4 |####################
   3982.9 |####################
   4001.3 |
   4019.8 |
   4038.3 |
   4056.7 |########################################
   4075.2 |
   4093.6 |########################################
   4112.1 |####################
   4130.6 |########################################
   4149.0 |####################
   4167.5 |########################################
   4185.9 |####################
   4204.4 |
   4222.8 |####################
   4241.3 |
   4259.8 |####################
   4278.2 |
   4296.7 |
   4315.1 |
  (1 below, 2 above range)

kernel_nway (n=18, range 4040.0-4462.2 ns)
   4040.0 |
   4061.1 |#############
   4082.2 |
   4103.3 |##########################
   4124.4 |########################################
   4145.6 |
   4166.7 |##########################
   4187.8 |#############
   4208.9 |#############
   4230.0 |########################################
   4251.1 |
   4272.2 |
   4293.3 |
   4314.4 |
   4335.6 |
   4356.7 |
   4377.8 |
   4398.9 |
   4420.0 |
   4441.1 |
  (2 below, 3 above range)

kernel_seq (n=18, range 2452.2-2755.0 ns)
   2452.2 |#############
   2467.3 |#############
   2482.5 |########################################
   2497.6 |#############
   2512.8 |##########################
   2527.9 |##########################
   2543.0 |#############
   2558.2 |########################################
   2573.3 |
   2588.5 |
   2603.6 |
   2618.7 |#############
   2633.9 |
   2649.0 |
   2664.2 |
   2679.3 |
   2694.5 |
   2709.6 |
   2724.7 |
   2739.9 |
  (1 below, 2 above range)

nway (n=18, range 17642.2-18587.8 ns)
  17642.2 |##########################
  17689.5 |#############
  17736.8 |#############
  17784.0 |
  17831.3 |#############
  17878.6 |########################################
  17925.9 |#############
  17973.1 |#############
  18020.4 |#############
  18067.7 |
  18115.0 |
  18162.3 |
  18209.5 |#############
  18256.8 |#############
  18304.1 |#############
  18351.4 |
  18398.7 |
  18445.9 |
  18493.2 |
  18540.5 |
  (2 below, 2 above range)

sequential (n=18, range 9940.6-11186.1 ns)
   9940.6 |
  10002.8 |
  10065.1 |
  10127.4 |########################################
  10189.7 |########################################
  10252.0 |##########
  10314.2 |
  10376.5 |##########
  10438.8 |##########
  10501.1 |
  10563.3 |
  10625.6 |
  10687.9 |
  10750.2 |
  10812.5 |
  10874.7 |##########
  10937.0 |
  10999.3 |
  11061.6 |
  11123.8 |
  (2 below, 4 above range)

std_parallel (n=18, range 30316.1-35273.1 ns)
  30316.1 |####################
  30564.0 |##########
  30811.8 |########################################
  31059.7 |####################
  31307.5 |##########
  31555.4 |##########
  31803.2 |
  32051.1 |
  32298.9 |
  32546.8 |
  32794.6 |
  33042.4 |
  33290.3 |##########
  33538.1 |
  33786.0 |##########
  34033.8 |##########
  34281.7 |
  34529.5 |
  34777.4 |
  35025.2 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=2392.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2333.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=2301.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3647.0% of algo (FFI overhead may distort results)
- **nway**: bridge=516.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=895.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=331.2% of algo (FFI overhead may distort results)
