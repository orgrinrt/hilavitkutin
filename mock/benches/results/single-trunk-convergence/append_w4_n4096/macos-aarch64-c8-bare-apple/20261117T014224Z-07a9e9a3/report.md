# append, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 17.2x slower than the field

std_parallel (48.07 us) is 17.2x the fastest (2.79 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 17.2x the fastest

Fastest head_tail_dyn (2.79 us) to slowest std_parallel (48.07 us): 17.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: head_tail_dyn** at 2787.5 ns median (-1.6% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 17.24x (fastest 2787.5 ns, slowest 48068.8 ns)

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
| head_tail | 3542ns | 3469ns | 3283ns | 3443ns | 3819ns | base |
| head_tail_dyn | 3439ns | 3431ns | 3326ns | 3407ns | 3542ns | -2.91% |
| kernel_nway | 12077ns | 12300ns | 10028ns | 11832ns | 13468ns | +240.99% |
| kernel_seq | 3651ns | 3525ns | 3349ns | 3476ns | 4063ns | +3.08% |
| nway | 40149ns | 40473ns | 38061ns | 40051ns | 41340ns | +1033.61% |
| sequential | 13179ns | 12962ns | 12618ns | 13032ns | 13679ns | +272.11% |
| std_parallel | 48753ns | 48446ns | 45650ns | 48141ns | 51223ns | +1276.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2936ns | 2747ns | 3181ns | base | 1.395 |
| head_tail_dyn | 2823ns | 2753ns | 2917ns | -3.82% | 1.451 |
| kernel_nway | 11307ns | 9368ns | 12606ns | +285.17% | 0.362 |
| kernel_seq | 3044ns | 2740ns | 3449ns | +3.67% | 1.346 |
| nway | 39685ns | 37574ns | 40916ns | +1251.83% | 0.103 |
| sequential | 12993ns | 12431ns | 13483ns | +342.58% | 0.315 |
| std_parallel | 48356ns | 45242ns | 50835ns | +1547.19% | 0.085 |

## Performance model

- Peak throughput: **1.495 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.446 | 96.7% |
| head_tail_dyn | 1.469 | 98.3% |
| kernel_nway | 0.357 | 23.9% |
| kernel_seq | 1.408 | 94.2% |
| nway | 0.102 | 6.8% |
| sequential | 0.320 | 21.4% |
| std_parallel | 0.085 | 5.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 3542ns | 3542ns | base |
| head_tail_dyn | 3439ns | 3439ns | -2.91% |
| kernel_nway | 12077ns | 12077ns | +240.99% |
| kernel_seq | 3651ns | 3651ns | +3.08% |
| nway | 40149ns | 40149ns | +1033.61% |
| sequential | 13179ns | 13179ns | +272.11% |
| std_parallel | 48753ns | 48753ns | +1276.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2833ns | base | --- | [2798, 3027] | --- | --- | --- | --- |
| head_tail_dyn | 2788ns | no significant difference | [-158, +25]ns | [2762, 2860] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 11475ns | +8595.8ns (+303.4%) | [+7465, +9212]ns | [10554, 12010] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 2908ns | no significant difference | [-35, +112]ns | [2802, 3108] | no | 0.4807 | 0.4807 | 0 |
| nway | 40012ns | +37131.2ns (+1310.5%) | [+35965, +37562]ns | [39104, 40512] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 12788ns | +9797.9ns (+345.8%) | [+9681, +10558]ns | [12706, 13373] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 48069ns | +45158.3ns (+1593.8%) | [+44279, +46283]ns | [47210, 49181] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2802ns | -0.9% | +309.5% | +1.7% | +1339.8% | +346.0% | +1623.3% |
| 1 | 2 | 3096ns | -10.3% | +264.3% | -3.0% | +1184.2% | +311.9% | +1472.3% |
| 1 | 3 | 2806ns | +1.9% | +300.0% | +5.3% | +1305.3% | +377.6% | +1587.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.667 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.166 | ok |
| kernel_nway | -0.064 | ok |
| kernel_seq | -0.368 | moderate- |
| nway | -0.019 | ok |
| sequential | -0.040 | ok |
| std_parallel | -0.351 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 112007.6ns | 2935.7ns | 3815.4% | HIGH |
| head_tail_dyn | 110346.8ns | 2823.4ns | 3908.3% | HIGH |
| kernel_nway | 142606.9ns | 11307.2ns | 1261.2% | HIGH |
| kernel_seq | 110471.3ns | 3043.5ns | 3629.7% | HIGH |
| nway | 121687.3ns | 39685.2ns | 306.6% | HIGH |
| sequential | 113203.9ns | 12992.6ns | 871.3% | HIGH |
| std_parallel | 134959.5ns | 48355.8ns | 279.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2747.2-3181.3 ns)
   2747.2 |####################
   2768.9 |
   2790.6 |########################################
   2812.3 |####################
   2834.0 |##########
   2855.7 |
   2877.4 |
   2899.1 |##########
   2920.8 |##########
   2942.5 |
   2964.2 |
   2986.0 |##########
   3007.7 |
   3029.4 |##########
   3051.1 |##########
   3072.8 |
   3094.5 |
   3116.2 |##########
   3137.9 |
   3159.6 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 2752.8-2917.4 ns)
   2752.8 |########################################
   2761.0 |##########
   2769.3 |##########
   2777.5 |##########
   2785.7 |####################
   2793.9 |##########
   2802.2 |
   2810.4 |
   2818.6 |
   2826.8 |
   2835.1 |
   2843.3 |##########
   2851.5 |##########
   2859.8 |##########
   2868.0 |
   2876.2 |
   2884.4 |
   2892.7 |##########
   2900.9 |
   2909.1 |
  (1 below, 3 above range)

kernel_nway (n=18, range 9368.1-12606.2 ns)
   9368.1 |
   9530.0 |####################
   9691.9 |
   9853.8 |
  10015.7 |####################
  10177.6 |####################
  10339.5 |####################
  10501.4 |
  10663.3 |####################
  10825.2 |####################
  10987.2 |####################
  11149.1 |####################
  11311.0 |
  11472.9 |
  11634.8 |########################################
  11796.7 |####################
  11958.6 |########################################
  12120.5 |
  12282.4 |
  12444.3 |####################
  (1 below, 3 above range)

kernel_seq (n=18, range 2740.3-3449.3 ns)
   2740.3 |##########################
   2775.7 |########################################
   2811.2 |
   2846.6 |#############
   2882.1 |#############
   2917.5 |#############
   2953.0 |#############
   2988.4 |
   3023.9 |##########################
   3059.3 |
   3094.8 |
   3130.2 |
   3165.7 |##########################
   3201.1 |#############
   3236.6 |
   3272.0 |
   3307.5 |
   3342.9 |
   3378.4 |
   3413.8 |
  (2 below, 2 above range)

nway (n=18, range 37573.6-40916.0 ns)
  37573.6 |
  37740.7 |
  37907.8 |####################
  38075.0 |##########
  38242.1 |
  38409.2 |
  38576.3 |
  38743.4 |
  38910.5 |
  39077.7 |##############################
  39244.8 |
  39411.9 |
  39579.0 |####################
  39746.1 |
  39913.3 |
  40080.4 |
  40247.5 |########################################
  40414.6 |
  40581.7 |##########
  40748.8 |
  (1 below, 4 above range)

sequential (n=18, range 12430.6-13483.3 ns)
  12430.6 |#############
  12483.2 |
  12535.8 |#############
  12588.5 |
  12641.1 |#############
  12693.8 |########################################
  12746.4 |#############
  12799.0 |#############
  12851.7 |
  12904.3 |
  12957.0 |
  13009.6 |
  13062.2 |
  13114.9 |
  13167.5 |
  13220.1 |
  13272.8 |#############
  13325.4 |##########################
  13378.1 |##########################
  13430.7 |#############
  (2 below, 2 above range)

std_parallel (n=18, range 45241.7-50834.7 ns)
  45241.7 |
  45521.3 |
  45801.0 |
  46080.6 |#############
  46360.3 |#############
  46639.9 |
  46919.6 |##########################
  47199.2 |#############
  47478.9 |#############
  47758.5 |##########################
  48038.2 |########################################
  48317.8 |
  48597.5 |
  48877.1 |#############
  49156.8 |#############
  49436.5 |
  49716.1 |
  49995.8 |
  50275.4 |
  50555.1 |#############
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=3904.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=3982.1% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1263.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=3747.3% of algo (FFI overhead may distort results)
- **nway**: bridge=301.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=872.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=279.5% of algo (FFI overhead may distort results)
