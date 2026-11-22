# append, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 5.2x slower than the field

std_parallel (73.92 us) is 5.2x the fastest (14.32 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 71.0%)

sequential wins on median (14.32 us) yet has the highest variance (CV 71.0%), while kernel_seq is the steadiest (CV 13.4%, 15.10 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### std_parallel shows alternating (throttle bounce) (autocorr -0.51)

std_parallel's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, kernel_nway, kernel_seq, head_tail_dyn, head_tail} vs {nway, std_parallel} (189% apart)

The field splits into a fast tier {sequential, kernel_nway, kernel_seq, head_tail_dyn, head_tail} and a slow tier {nway, std_parallel} with a 189% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.2x the fastest

Fastest sequential (14.32 us) to slowest std_parallel (73.92 us): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader sequential vs stability leader kernel_seq (+5% speed for 5.3x steadier)

sequential is fastest (14.32 us, CV 71.0%); kernel_seq gives up 5.4% median for 5.3x lower variance (CV 13.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### sequential is inconsistent: worst-20% is 3.2x its best-20%

sequential's best 20% of batches run at 9.96 us but its worst 20% at 32.12 us (3.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 14324.4 ns median (-6.5% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 5.16x (fastest 14324.4 ns, slowest 73923.4 ns)

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
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 16183ns | 16026ns | 11578ns | 15584ns | 19384ns | base |
| head_tail_dyn | 17176ns | 16023ns | 13351ns | 15236ns | 21999ns | +6.14% |
| kernel_nway | 15918ns | 15418ns | 10332ns | 14945ns | 20171ns | -1.64% |
| kernel_seq | 15682ns | 15673ns | 12421ns | 14806ns | 18625ns | -3.10% |
| nway | 47656ns | 44810ns | 41417ns | 44641ns | 55299ns | +194.48% |
| sequential | 20024ns | 14523ns | 10113ns | 14923ns | 32632ns | +23.74% |
| std_parallel | 64450ns | 74507ns | 33552ns | 60641ns | 85613ns | +298.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 15164ns | 11077ns | 17493ns | base | 0.270 |
| head_tail_dyn | 16305ns | 12897ns | 20634ns | +7.53% | 0.251 |
| kernel_nway | 15062ns | 9925ns | 18677ns | -0.67% | 0.272 |
| kernel_seq | 14730ns | 11949ns | 16841ns | -2.86% | 0.278 |
| nway | 47060ns | 40933ns | 54558ns | +210.35% | 0.087 |
| sequential | 19721ns | 9964ns | 32120ns | +30.06% | 0.208 |
| std_parallel | 63837ns | 33374ns | 84439ns | +320.99% | 0.064 |

## Performance model

- Peak throughput: **0.413 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.267 | 64.8% |
| head_tail_dyn | 0.270 | 65.4% |
| kernel_nway | 0.277 | 67.2% |
| kernel_seq | 0.271 | 65.7% |
| nway | 0.093 | 22.4% |
| sequential | 0.286 | 69.3% |
| std_parallel | 0.055 | 13.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 16183ns | 16183ns | base |
| head_tail_dyn | 17176ns | 17176ns | +6.14% |
| kernel_nway | 15918ns | 15918ns | -1.64% |
| kernel_seq | 15682ns | 15682ns | -3.10% |
| nway | 47656ns | 47656ns | +194.48% |
| sequential | 20024ns | 20024ns | +23.74% |
| std_parallel | 64450ns | 64450ns | +298.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 15318ns | base | --- | [14385, 16463] | --- | --- | --- | --- |
| head_tail_dyn | 15174ns | no significant difference | [-1952, +2145]ns | [13722, 16530] | no | 0.5768 | 0.4807 | 0 |
| kernel_nway | 14774ns | no significant difference | [-886, +544]ns | [13690, 17056] | no | 0.5768 | 0.4807 | 0 |
| kernel_seq | 15098ns | no significant difference | [-1874, +867]ns | [13043, 15742] | no | 0.5768 | 0.4807 | 0 |
| nway | 44257ns | +29604.1ns (+193.3%) | [+28032, +31584]ns | [43326, 47463] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 14324ns | no significant difference | [-3844, +14024]ns | [10158, 29479] | no | 0.8145 | 0.8145 | 0 |
| std_parallel | 73923ns | +57612.7ns (+376.1%) | [+32725, +64500]ns | [42353, 80878] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 15947ns | -13.1% | -8.6% | -12.5% | +201.6% | +7.2% | +281.1% |
| 1 | 2 | 15404ns | -4.5% | -3.8% | -9.9% | +176.8% | -26.9% | +420.9% |
| 1 | 3 | 14385ns | +14.9% | +4.6% | +8.2% | +205.5% | +57.4% | +196.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.020 | ok |
| head_tail_dyn | -0.029 | ok |
| kernel_nway | -0.000 | ok |
| kernel_seq | -0.193 | ok |
| nway | -0.313 | moderate- |
| sequential | -0.508 | HIGH- (thermal bounce) |
| std_parallel | -0.512 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 77558.9ns | 15163.5ns | 511.5% | HIGH |
| head_tail_dyn | 73460.1ns | 16305.0ns | 450.5% | HIGH |
| kernel_nway | 73183.7ns | 15061.5ns | 485.9% | HIGH |
| kernel_seq | 75382.3ns | 14730.2ns | 511.8% | HIGH |
| nway | 94852.2ns | 47059.7ns | 201.6% | HIGH |
| sequential | 49839.9ns | 19721.1ns | 252.7% | HIGH |
| std_parallel | 95677.9ns | 63837.2ns | 149.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 11076.8-17493.4 ns)
  11076.8 |
  11397.6 |
  11718.5 |
  12039.3 |
  12360.1 |
  12680.9 |
  13001.8 |
  13322.6 |####################
  13643.4 |####################
  13964.3 |########################################
  14285.1 |####################
  14605.9 |####################
  14926.8 |####################
  15247.6 |########################################
  15568.4 |########################################
  15889.2 |####################
  16210.1 |
  16530.9 |####################
  16851.7 |########################################
  17172.6 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 12897.1-20634.2 ns)
  12897.1 |##########################
  13283.9 |##########################
  13670.8 |#############
  14057.6 |##########################
  14444.5 |
  14831.4 |#############
  15218.2 |#############
  15605.1 |#############
  15991.9 |#############
  16378.8 |########################################
  16765.6 |
  17152.5 |
  17539.3 |#############
  17926.2 |
  18313.1 |
  18699.9 |#############
  19086.8 |
  19473.6 |
  19860.5 |
  20247.3 |
  (1 below, 1 above range)

kernel_nway (n=18, range 9925.0-18676.8 ns)
   9925.0 |
  10362.6 |
  10800.2 |
  11237.8 |
  11675.4 |
  12113.0 |#############
  12550.5 |##########################
  12988.1 |#############
  13425.7 |
  13863.3 |########################################
  14300.9 |#############
  14738.5 |#############
  15176.1 |##########################
  15613.7 |
  16051.3 |
  16488.8 |#############
  16926.4 |
  17364.0 |#############
  17801.6 |
  18239.2 |##########################
  (1 below, 2 above range)

kernel_seq (n=18, range 11949.5-16840.6 ns)
  11949.5 |##########################
  12194.0 |#############
  12438.6 |
  12683.1 |#############
  12927.7 |
  13172.3 |#############
  13416.8 |
  13661.4 |
  13905.9 |
  14150.5 |
  14395.1 |#############
  14639.6 |#############
  14884.2 |#############
  15128.7 |########################################
  15373.3 |#############
  15617.9 |
  15862.4 |##########################
  16107.0 |#############
  16351.5 |
  16596.1 |
  (1 below, 2 above range)

nway (n=18, range 40932.7-54557.8 ns)
  40932.7 |
  41613.9 |##########
  42295.2 |
  42976.4 |########################################
  43657.7 |##############################
  44338.9 |##########
  45020.2 |
  45701.5 |
  46382.7 |##########
  47064.0 |##########
  47745.2 |##############################
  48426.5 |
  49107.7 |
  49789.0 |
  50470.3 |
  51151.5 |
  51832.8 |
  52514.0 |##########
  53195.3 |
  53876.5 |
  (2 below, 1 above range)

sequential (n=18, range 9964.1-32120.1 ns)
   9964.1 |########################################
  11071.9 |
  12179.7 |######
  13287.5 |
  14395.3 |
  15503.1 |######
  16610.9 |
  17718.7 |
  18826.5 |
  19934.3 |
  21042.1 |
  22149.9 |
  23257.7 |######
  24365.5 |
  25473.3 |
  26581.1 |
  27688.9 |
  28796.7 |#############
  29904.5 |######
  31012.3 |######
  (2 below, 3 above range)

std_parallel (n=18, range 33373.6-84438.7 ns)
  33373.6 |########################################
  35926.8 |
  38480.1 |
  41033.3 |
  43586.6 |
  46139.9 |
  48693.1 |##########################
  51246.4 |
  53799.6 |
  56352.9 |
  58906.1 |
  61459.4 |
  64012.6 |
  66565.9 |
  69119.2 |#############
  71672.4 |#############
  74225.7 |#############
  76778.9 |##########################
  79332.2 |##########################
  81885.4 |##########################
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=498.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=33.1% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=481.1% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=23.2% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=502.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=497.8% of algo (FFI overhead may distort results)
- **nway**: bridge=209.6% of algo (FFI overhead may distort results)
- **sequential**: CV=51.6% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **sequential**: bridge=254.1% of algo (FFI overhead may distort results)
- **std_parallel**: CV=33.5% (high variance, measurements may be unstable)
- **std_parallel**: bridge=148.5% of algo (FFI overhead may distort results)
