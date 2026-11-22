# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 62% faster than the next best (head_tail)

kernel_seq (2.56 us) leads head_tail (4.14 us) by 62%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 38% (significant)

kernel_seq is -1.56 us (38%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 31.7x slower than the field

std_parallel (81.08 us) is 31.7x the fastest (2.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.66)

kernel_seq's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 31.7x the fastest

Fastest kernel_seq (2.56 us) to slowest std_parallel (81.08 us): 31.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2561.7 ns median (-38.1% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 31.65x (fastest 2561.7 ns, slowest 81085.0 ns)

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
| head_tail | 4745ns | 4686ns | 4584ns | 4681ns | 4920ns | base |
| head_tail_dyn | 4776ns | 4772ns | 4621ns | 4744ns | 4901ns | +0.66% |
| kernel_nway | 16890ns | 15712ns | 14950ns | 15744ns | 19578ns | +255.98% |
| kernel_seq | 2979ns | 2917ns | 2823ns | 2898ns | 3177ns | -37.22% |
| nway | 50976ns | 51207ns | 47873ns | 50459ns | 53303ns | +974.41% |
| sequential | 10638ns | 10378ns | 10226ns | 10375ns | 11238ns | +124.21% |
| std_parallel | 80485ns | 81451ns | 72841ns | 78770ns | 86880ns | +1596.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4171ns | 4003ns | 4333ns | base | 0.245 |
| head_tail_dyn | 4214ns | 4094ns | 4336ns | +1.03% | 0.243 |
| kernel_nway | 16225ns | 14325ns | 18923ns | +288.94% | 0.063 |
| kernel_seq | 2599ns | 2528ns | 2692ns | -37.70% | 0.394 |
| nway | 50419ns | 47316ns | 52741ns | +1108.65% | 0.020 |
| sequential | 10499ns | 10086ns | 11097ns | +151.68% | 0.098 |
| std_parallel | 80123ns | 72472ns | 86516ns | +1820.73% | 0.013 |

## Performance model

- Peak throughput: **0.405 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.247 | 61.0% |
| head_tail_dyn | 0.244 | 60.3% |
| kernel_nway | 0.068 | 16.8% |
| kernel_seq | 0.400 | 98.7% |
| nway | 0.020 | 5.0% |
| sequential | 0.100 | 24.7% |
| std_parallel | 0.013 | 3.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4745ns | 4745ns | base |
| head_tail_dyn | 4776ns | 4776ns | +0.66% |
| kernel_nway | 16890ns | 16890ns | +255.98% |
| kernel_seq | 2979ns | 2979ns | -37.22% |
| nway | 50976ns | 50976ns | +974.41% |
| sequential | 10638ns | 10638ns | +124.21% |
| std_parallel | 80485ns | 80485ns | +1596.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4141ns | base | --- | [4092, 4211] | --- | --- | --- | --- |
| head_tail_dyn | 4191ns | no significant difference | [-14, +124]ns | [4137, 4272] | no | 0.2379 | 0.2379 | 0 |
| kernel_nway | 15076ns | +10896.6ns (+263.2%) | [+10631, +12044]ns | [14843, 16233] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2562ns | -1560.9ns (-37.7%) | [-1658, -1491]ns | [2549, 2618] | YES | 0.0000 | 0.0000 | 0 |
| nway | 50643ns | +46481.7ns (+1122.5%) | [+44646, +47382]ns | [48852, 51579] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10236ns | +6121.6ns (+147.8%) | [+6008, +6484]ns | [10221, 10510] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 81085ns | +77021.7ns (+1860.0%) | [+70662, +80798]ns | [74766, 85037] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4271ns | -1.9% | +260.3% | -40.1% | +1086.4% | +139.8% | +1826.3% |
| 1 | 2 | 4099ns | +4.2% | +264.9% | -35.5% | +1135.4% | +149.5% | +1858.8% |
| 1 | 3 | 4162ns | -1.1% | +276.3% | -38.7% | +1116.8% | +173.2% | +1794.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.403 | moderate- |
| head_tail_dyn | -0.512 | HIGH- (thermal bounce) |
| kernel_nway | -0.569 | HIGH- (thermal bounce) |
| kernel_seq | -0.662 | HIGH- (thermal bounce) |
| nway | -0.120 | ok |
| sequential | -0.174 | ok |
| std_parallel | -0.005 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 96673.3ns | 4171.5ns | 2317.5% | HIGH |
| head_tail_dyn | 95756.8ns | 4214.4ns | 2272.1% | HIGH |
| kernel_nway | 109590.1ns | 16224.6ns | 675.5% | HIGH |
| kernel_seq | 91683.1ns | 2598.7ns | 3528.0% | HIGH |
| nway | 111854.0ns | 50418.5ns | 221.9% | HIGH |
| sequential | 91882.4ns | 10498.6ns | 875.2% | HIGH |
| std_parallel | 132060.0ns | 80122.9ns | 164.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 4002.8-4332.5 ns)
   4002.8 |
   4019.3 |
   4035.7 |####################
   4052.2 |########################################
   4068.7 |
   4085.2 |########################################
   4101.7 |########################################
   4118.2 |####################
   4134.7 |
   4151.2 |####################
   4167.6 |########################################
   4184.1 |
   4200.6 |########################################
   4217.1 |
   4233.6 |
   4250.1 |
   4266.6 |
   4283.1 |
   4299.5 |
   4316.0 |####################
  (1 below, 3 above range)

head_tail_dyn (n=18, range 4094.4-4335.8 ns)
   4094.4 |########################################
   4106.5 |
   4118.6 |####################
   4130.6 |########################################
   4142.7 |
   4154.8 |####################
   4166.9 |####################
   4178.9 |####################
   4191.0 |####################
   4203.1 |####################
   4215.1 |
   4227.2 |####################
   4239.3 |####################
   4251.3 |
   4263.4 |
   4275.5 |
   4287.6 |
   4299.6 |####################
   4311.7 |
   4323.8 |####################
  (1 below, 3 above range)

kernel_nway (n=18, range 14325.0-18923.3 ns)
  14325.0 |
  14554.9 |##########
  14784.8 |########################################
  15014.8 |########################################
  15244.7 |
  15474.6 |
  15704.5 |##########
  15934.4 |
  16164.3 |####################
  16394.2 |
  16624.2 |
  16854.1 |
  17084.0 |
  17313.9 |
  17543.8 |
  17773.8 |
  18003.7 |
  18233.6 |
  18463.5 |
  18693.4 |##########
  (2 below, 3 above range)

kernel_seq (n=18, range 2527.8-2692.2 ns)
   2527.8 |##########################
   2536.0 |#############
   2544.2 |########################################
   2552.4 |##########################
   2560.7 |#############
   2568.9 |
   2577.1 |#############
   2585.3 |
   2593.5 |#############
   2601.8 |
   2610.0 |#############
   2618.2 |#############
   2626.4 |
   2634.7 |#############
   2642.9 |#############
   2651.1 |
   2659.3 |
   2667.5 |
   2675.8 |
   2684.0 |
  (1 below, 2 above range)

nway (n=18, range 47315.6-52740.5 ns)
  47315.6 |########################################
  47586.8 |
  47858.1 |
  48129.3 |
  48400.6 |
  48671.8 |#############
  48943.1 |#############
  49214.3 |
  49485.6 |
  49756.8 |
  50028.1 |#############
  50299.3 |#############
  50570.5 |##########################
  50841.8 |#############
  51113.0 |##########################
  51384.3 |
  51655.5 |
  51926.8 |#############
  52198.0 |
  52469.3 |#############
  (1 below, 3 above range)

sequential (n=18, range 10085.5-11096.9 ns)
  10085.5 |
  10136.1 |
  10186.7 |########################################
  10237.2 |###########
  10287.8 |###########
  10338.4 |
  10389.0 |
  10439.5 |
  10490.1 |
  10540.7 |
  10591.2 |
  10641.8 |
  10692.4 |#####
  10743.0 |
  10793.5 |
  10844.1 |
  10894.7 |
  10945.2 |
  10995.8 |
  11046.4 |
  (2 below, 4 above range)

std_parallel (n=18, range 72471.7-86516.4 ns)
  72471.7 |####################
  73173.9 |####################
  73876.1 |########################################
  74578.4 |####################
  75280.6 |####################
  75982.8 |
  76685.1 |
  77387.3 |####################
  78089.5 |
  78791.8 |
  79494.0 |####################
  80196.3 |
  80898.5 |
  81600.7 |####################
  82303.0 |####################
  83005.2 |
  83707.4 |####################
  84409.7 |####################
  85111.9 |####################
  85814.1 |
  (1 below, 4 above range)

```

## Diagnostics

- **head_tail**: bridge=2338.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2281.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=729.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3561.6% of algo (FFI overhead may distort results)
- **nway**: bridge=218.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=901.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=162.5% of algo (FFI overhead may distort results)
