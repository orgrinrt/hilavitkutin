# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 64% faster than the next best (head_tail)

kernel_seq (2.49 us) leads head_tail (4.09 us) by 64%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq beats baseline by 39% (significant)

kernel_seq is -1.58 us (39%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 33.1x slower than the field

std_parallel (82.27 us) is 33.1x the fastest (2.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.63)

nway's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 33.1x the fastest

Fastest kernel_seq (2.49 us) to slowest std_parallel (82.27 us): 33.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 2488.3 ns median (-39.1% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 33.06x (fastest 2488.3 ns, slowest 82265.8 ns)

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
| head_tail | 4745ns | 4706ns | 4567ns | 4685ns | 4925ns | base |
| head_tail_dyn | 4768ns | 4725ns | 4590ns | 4702ns | 4956ns | +0.47% |
| kernel_nway | 16631ns | 15924ns | 14947ns | 15673ns | 18910ns | +250.47% |
| kernel_seq | 2939ns | 2930ns | 2684ns | 2890ns | 3139ns | -38.07% |
| nway | 50784ns | 50950ns | 47669ns | 50513ns | 52749ns | +970.21% |
| sequential | 10321ns | 10301ns | 10046ns | 10236ns | 10586ns | +117.49% |
| std_parallel | 82158ns | 82602ns | 74218ns | 80631ns | 88419ns | +1631.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4123ns | 3942ns | 4303ns | base | 0.016 |
| head_tail_dyn | 4176ns | 3982ns | 4375ns | +1.30% | 0.015 |
| kernel_nway | 15988ns | 14307ns | 18248ns | +287.80% | 0.004 |
| kernel_seq | 2508ns | 2442ns | 2576ns | -39.15% | 0.026 |
| nway | 50224ns | 47119ns | 52185ns | +1118.25% | 0.001 |
| sequential | 10171ns | 9903ns | 10423ns | +146.71% | 0.006 |
| std_parallel | 81800ns | 73862ns | 88061ns | +1884.19% | 0.001 |

## Performance model

- Peak throughput: **0.026 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.016 | 59.8% |
| head_tail_dyn | 0.016 | 59.2% |
| kernel_nway | 0.004 | 16.0% |
| kernel_seq | 0.026 | 98.1% |
| nway | 0.001 | 4.8% |
| sequential | 0.006 | 24.0% |
| std_parallel | 0.001 | 3.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4745ns | 4745ns | base |
| head_tail_dyn | 4768ns | 4768ns | +0.47% |
| kernel_nway | 16631ns | 16631ns | +250.47% |
| kernel_seq | 2939ns | 2939ns | -38.07% |
| nway | 50784ns | 50784ns | +970.21% |
| sequential | 10321ns | 10321ns | +117.49% |
| std_parallel | 82158ns | 82158ns | +1631.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4086ns | base | --- | [4029, 4159] | --- | --- | --- | --- |
| head_tail_dyn | 4127ns | no significant difference | [-19, +185]ns | [4057, 4221] | no | 0.0963 | 0.0963 | 0 |
| kernel_nway | 15282ns | +11260.0ns (+275.6%) | [+10593, +12355]ns | [14763, 16422] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2488ns | -1580.8ns (-38.7%) | [-1633, -1510]ns | [2467, 2542] | YES | 0.0000 | 0.0000 | 0 |
| nway | 50382ns | +46158.3ns (+1129.7%) | [+45209, +47508]ns | [49248, 51580] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10162ns | +5963.4ns (+146.0%) | [+5862, +6140]ns | [9955, 10170] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 82266ns | +78250.0ns (+1915.2%) | [+73432, +82649]ns | [77473, 86718] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4093ns | +2.9% | +285.1% | -37.6% | +1140.2% | +143.1% | +1909.2% |
| 1 | 2 | 4007ns | +1.2% | +270.0% | -37.7% | +1137.4% | +153.8% | +1953.2% |
| 1 | 3 | 4159ns | -1.5% | +264.8% | -40.7% | +1130.4% | +144.5% | +1864.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.543 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.329 | moderate- |
| kernel_nway | -0.406 | moderate- |
| kernel_seq | -0.031 | ok |
| nway | -0.627 | HIGH- (thermal bounce) |
| sequential | -0.171 | ok |
| std_parallel | -0.189 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 97559.5ns | 4122.6ns | 2366.5% | HIGH |
| head_tail_dyn | 97841.3ns | 4176.3ns | 2342.8% | HIGH |
| kernel_nway | 113607.2ns | 15987.6ns | 710.6% | HIGH |
| kernel_seq | 91570.1ns | 2508.4ns | 3650.5% | HIGH |
| nway | 108534.3ns | 50223.5ns | 216.1% | HIGH |
| sequential | 91000.5ns | 10170.7ns | 894.7% | HIGH |
| std_parallel | 133006.0ns | 81799.8ns | 162.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 3942.2-4303.1 ns)
   3942.2 |#############
   3960.3 |
   3978.3 |#############
   3996.4 |
   4014.4 |#############
   4032.4 |########################################
   4050.5 |
   4068.5 |#############
   4086.6 |##########################
   4104.6 |
   4122.6 |
   4140.7 |##########################
   4158.7 |#############
   4176.8 |
   4194.8 |
   4212.9 |
   4230.9 |#############
   4248.9 |
   4267.0 |#############
   4285.0 |#############
  (2 below, 1 above range)

head_tail_dyn (n=18, range 3981.7-4374.7 ns)
   3981.7 |
   4001.3 |##########
   4021.0 |
   4040.6 |########################################
   4060.3 |##########
   4079.9 |
   4099.6 |##########
   4119.2 |####################
   4138.9 |
   4158.5 |##########
   4178.2 |
   4197.9 |####################
   4217.5 |##########
   4237.2 |####################
   4256.8 |
   4276.5 |
   4296.1 |
   4315.8 |
   4335.4 |
   4355.1 |
  (1 below, 2 above range)

kernel_nway (n=18, range 14306.6-18248.3 ns)
  14306.6 |####################
  14503.7 |##########
  14700.8 |########################################
  14897.9 |
  15095.0 |##########
  15292.1 |####################
  15489.1 |##########
  15686.2 |
  15883.3 |##########
  16080.4 |
  16277.5 |
  16474.6 |
  16671.6 |
  16868.7 |##########
  17065.8 |
  17262.9 |##########
  17460.0 |
  17657.1 |##########
  17854.1 |
  18051.2 |
  (1 below, 2 above range)

kernel_seq (n=18, range 2442.2-2576.4 ns)
   2442.2 |####################
   2448.9 |####################
   2455.6 |####################
   2462.4 |########################################
   2469.1 |########################################
   2475.8 |
   2482.5 |########################################
   2489.2 |
   2495.9 |
   2502.6 |
   2509.3 |
   2516.0 |
   2522.7 |####################
   2529.4 |####################
   2536.1 |####################
   2542.8 |####################
   2549.6 |####################
   2556.3 |
   2563.0 |
   2569.7 |
  (1 below, 3 above range)

nway (n=18, range 47118.9-52184.7 ns)
  47118.9 |
  47372.2 |
  47625.5 |
  47878.8 |
  48132.1 |########################################
  48385.4 |
  48638.6 |
  48891.9 |####################
  49145.2 |
  49398.5 |####################
  49651.8 |########################################
  49905.1 |
  50158.4 |####################
  50411.7 |####################
  50665.0 |
  50918.3 |####################
  51171.6 |########################################
  51424.8 |
  51678.1 |####################
  51931.4 |########################################
  (2 below, 2 above range)

sequential (n=18, range 9902.8-10423.3 ns)
   9902.8 |######
   9928.8 |####################
   9954.9 |######
   9980.9 |
  10006.9 |
  10032.9 |
  10059.0 |
  10085.0 |
  10111.0 |
  10137.0 |####################
  10163.1 |########################################
  10189.1 |
  10215.1 |
  10241.1 |
  10267.2 |######
  10293.2 |######
  10319.2 |
  10345.3 |
  10371.3 |
  10397.3 |
  (1 below, 1 above range)

std_parallel (n=18, range 73861.7-88061.1 ns)
  73861.7 |####################
  74571.6 |####################
  75281.6 |####################
  75991.6 |
  76701.6 |####################
  77411.5 |
  78121.5 |########################################
  78831.5 |####################
  79541.4 |
  80251.4 |
  80961.4 |####################
  81671.4 |
  82381.3 |
  83091.3 |####################
  83801.3 |####################
  84511.3 |
  85221.2 |########################################
  85931.2 |
  86641.2 |
  87351.1 |########################################
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=2405.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=2375.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=742.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3657.9% of algo (FFI overhead may distort results)
- **nway**: bridge=215.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=902.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=163.0% of algo (FFI overhead may distort results)
