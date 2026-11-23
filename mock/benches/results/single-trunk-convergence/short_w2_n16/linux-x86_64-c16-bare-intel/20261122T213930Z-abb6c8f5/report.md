# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 34% faster than the next best (kernel_seq)

sequential (7.16 us) leads kernel_seq (9.60 us) by 34%, a clear separation rather than a photo finish. CV 120.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 6.6x slower than the field

std_parallel (47.42 us) is 6.6x the fastest (7.16 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 120.8%)

sequential wins on median (7.16 us) yet has the highest variance (CV 120.8%), while head_tail is the steadiest (CV 9.1%, 11.72 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### std_parallel shows alternating (throttle bounce) (autocorr -0.59)

std_parallel's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, kernel_seq, head_tail, kernel_nway, head_tail_dyn} vs {nway, std_parallel} (155% apart)

The field splits into a fast tier {sequential, kernel_seq, head_tail, kernel_nway, head_tail_dyn} and a slow tier {nway, std_parallel} with a 155% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.6x the fastest

Fastest sequential (7.16 us) to slowest std_parallel (47.42 us): 6.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.6x its best-20%

sequential's best 20% of batches run at 6.69 us but its worst 20% at 24.33 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7158.4 ns median (-38.9% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 6.62x (fastest 7158.4 ns, slowest 47422.6 ns)

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
| Counter frequency | 2112817100 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 12431ns | 12305ns | 10727ns | 12026ns | 13890ns | base |
| head_tail_dyn | 13562ns | 14391ns | 7381ns | 12011ns | 18978ns | +9.10% |
| kernel_nway | 14177ns | 13692ns | 11126ns | 13596ns | 16576ns | +14.05% |
| kernel_seq | 10691ns | 10303ns | 8609ns | 10003ns | 12763ns | -14.00% |
| nway | 37354ns | 35411ns | 32318ns | 34270ns | 44498ns | +200.49% |
| sequential | 13658ns | 7301ns | 6825ns | 8470ns | 24857ns | +9.87% |
| std_parallel | 49166ns | 48160ns | 39229ns | 46620ns | 57954ns | +295.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 11762ns | 10235ns | 12959ns | base | 0.001 |
| head_tail_dyn | 12854ns | 6851ns | 18049ns | +9.28% | 0.001 |
| kernel_nway | 13337ns | 10533ns | 15533ns | +13.38% | 0.001 |
| kernel_seq | 10017ns | 8099ns | 11888ns | -14.84% | 0.002 |
| nway | 36863ns | 31887ns | 43904ns | +213.40% | 0.000 |
| sequential | 13377ns | 6689ns | 24333ns | +13.72% | 0.001 |
| std_parallel | 48519ns | 38872ns | 57067ns | +312.49% | 0.000 |

## Performance model

- Peak throughput: **0.002 Gops/s** (sequential; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.001 | 57.1% |
| head_tail_dyn | 0.001 | 48.8% |
| kernel_nway | 0.001 | 52.4% |
| kernel_seq | 0.002 | 69.7% |
| nway | 0.000 | 19.1% |
| sequential | 0.002 | 93.4% |
| std_parallel | 0.000 | 14.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 12431ns | 12431ns | base |
| head_tail_dyn | 13562ns | 13562ns | +9.10% |
| kernel_nway | 14177ns | 14177ns | +14.05% |
| kernel_seq | 10691ns | 10691ns | -14.00% |
| nway | 37354ns | 37354ns | +200.49% |
| sequential | 13658ns | 13658ns | +9.87% |
| std_parallel | 49166ns | 49166ns | +295.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 11721ns | base | --- | [10795, 12813] | --- | --- | --- | --- |
| head_tail_dyn | 13699ns | no significant difference | [-3469, +6000]ns | [7528, 16029] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 12757ns | +1549.4ns (+13.2%) | [+294, +2372]ns | [12568, 14246] | YES | 0.0113 | 0.0075 | 0 |
| kernel_seq | 9596ns | -1746.1ns (-14.9%) | [-2584, -1311]ns | [8921, 10346] | YES | 0.0003 | 0.0001 | 0 |
| nway | 34934ns | +22442.0ns (+191.5%) | [+21397, +26866]ns | [32632, 38765] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 7158ns | no significant difference | [-4469, +8098]ns | [6799, 19988] | no | 0.4807 | 0.4807 | 0 |
| std_parallel | 47423ns | +35463.4ns (+302.6%) | [+32517, +40547]ns | [43021, 52923] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 10711ns | +71.9% | +2.5% | -14.2% | +232.6% | +17.5% | +310.3% |
| 1 | 2 | 12826ns | +6.8% | +4.7% | -26.5% | +160.2% | +4.8% | +343.8% |
| 1 | 3 | 12094ns | -40.8% | +11.4% | -11.1% | +182.8% | -43.0% | +299.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.390 | moderate- |
| head_tail_dyn | -0.006 | ok |
| kernel_nway | -0.156 | ok |
| kernel_seq | -0.094 | ok |
| nway | -0.405 | moderate- |
| sequential | -0.240 | moderate- |
| std_parallel | -0.594 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 12293.3ns | 11762.3ns | 104.5% | HIGH |
| head_tail_dyn | 13404.6ns | 12853.9ns | 104.3% | HIGH |
| kernel_nway | 13961.8ns | 13336.6ns | 104.7% | HIGH |
| kernel_seq | 10153.5ns | 10016.6ns | 101.4% | HIGH |
| nway | 37247.5ns | 36862.7ns | 101.0% | HIGH |
| sequential | 13892.7ns | 13376.6ns | 103.9% | HIGH |
| std_parallel | 50000.5ns | 48518.6ns | 103.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 10234.7-12959.2 ns)
  10234.7 |#############
  10371.0 |
  10507.2 |
  10643.4 |########################################
  10779.6 |#############
  10915.9 |
  11052.1 |#############
  11188.3 |
  11324.5 |#############
  11460.8 |#############
  11597.0 |
  11733.2 |
  11869.4 |#############
  12005.7 |
  12141.9 |
  12278.1 |#############
  12414.3 |
  12550.6 |#############
  12686.8 |#############
  12823.0 |########################################
  (1 below, 2 above range)

head_tail_dyn (n=18, range 6850.7-18049.0 ns)
   6850.7 |########################################
   7410.6 |##########################
   7970.5 |
   8530.4 |
   9090.4 |
   9650.3 |
  10210.2 |
  10770.1 |
  11330.0 |
  11889.9 |#############
  12449.9 |#############
  13009.8 |
  13569.7 |########################################
  14129.6 |#############
  14689.5 |
  15249.4 |#############
  15809.4 |
  16369.3 |#############
  16929.2 |
  17489.1 |
  (1 below, 4 above range)

kernel_nway (n=18, range 10533.3-15532.9 ns)
  10533.3 |
  10783.3 |#############
  11033.3 |
  11283.2 |
  11533.2 |
  11783.2 |
  12033.2 |
  12283.1 |
  12533.1 |########################################
  12783.1 |
  13033.1 |
  13283.1 |
  13533.0 |
  13783.0 |
  14033.0 |##########################
  14283.0 |######
  14533.0 |
  14782.9 |######
  15032.9 |
  15282.9 |
  (2 below, 2 above range)

kernel_seq (n=18, range 8099.2-11888.4 ns)
   8099.2 |
   8288.7 |
   8478.1 |
   8667.6 |#############
   8857.0 |########################################
   9046.5 |
   9236.0 |#############
   9425.4 |##########################
   9614.9 |#############
   9804.3 |##########################
   9993.8 |#############
  10183.2 |
  10372.7 |
  10562.2 |#############
  10751.6 |##########################
  10941.1 |
  11130.5 |
  11320.0 |
  11509.5 |
  11698.9 |
  (2 below, 2 above range)

nway (n=18, range 31887.2-43904.2 ns)
  31887.2 |########################################
  32488.1 |########################################
  33088.9 |#############
  33689.8 |
  34290.6 |#############
  34891.5 |##########################
  35492.3 |#############
  36093.2 |
  36694.0 |
  37294.9 |#############
  37895.7 |
  38496.6 |
  39097.4 |#############
  39698.3 |
  40299.1 |
  40900.0 |
  41500.8 |
  42101.7 |
  42702.5 |
  43303.4 |
  (1 below, 4 above range)

sequential (n=18, range 6688.9-24333.2 ns)
   6688.9 |########################################
   7571.1 |#####
   8453.3 |
   9335.5 |
  10217.7 |
  11100.0 |
  11982.2 |
  12864.4 |
  13746.6 |
  14628.8 |
  15511.0 |
  16393.3 |
  17275.5 |
  18157.7 |##########
  19039.9 |
  19922.1 |
  20804.4 |#####
  21686.6 |#####
  22568.8 |#####
  23451.0 |
  (2 below, 2 above range)

std_parallel (n=18, range 38871.5-57067.0 ns)
  38871.5 |##########################
  39781.3 |
  40691.1 |
  41600.8 |#############
  42510.6 |##########################
  43420.4 |
  44330.2 |#############
  45239.9 |#############
  46149.7 |#############
  47059.5 |
  47969.3 |#############
  48879.0 |
  49788.8 |########################################
  50698.6 |
  51608.4 |
  52518.1 |
  53427.9 |
  54337.7 |
  55247.5 |#############
  56157.2 |#############
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=104.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=35.7% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=105.1% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=105.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=103.3% of algo (FFI overhead may distort results)
- **nway**: bridge=98.4% of algo (FFI overhead may distort results)
- **sequential**: CV=64.6% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **sequential**: bridge=101.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=109.1% of algo (FFI overhead may distort results)
