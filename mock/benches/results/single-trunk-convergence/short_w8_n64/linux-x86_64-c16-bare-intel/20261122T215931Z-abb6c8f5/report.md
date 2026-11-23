# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 34% faster than the next best (kernel_seq)

sequential (7.12 us) leads kernel_seq (9.53 us) by 34%, a clear separation rather than a photo finish. CV 112.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 9.9x slower than the field

std_parallel (70.82 us) is 9.9x the fastest (7.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 112.7%)

sequential wins on median (7.12 us) yet has the highest variance (CV 112.7%), while head_tail is the steadiest (CV 7.8%, 12.31 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sequential shows alternating (throttle bounce) (autocorr -0.67)

sequential's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, kernel_seq, head_tail, head_tail_dyn, kernel_nway} vs {nway, std_parallel} (144% apart)

The field splits into a fast tier {sequential, kernel_seq, head_tail, head_tail_dyn, kernel_nway} and a slow tier {nway, std_parallel} with a 144% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 9.9x the fastest

Fastest sequential (7.12 us) to slowest std_parallel (70.82 us): 9.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail_dyn is inconsistent: worst-20% is 4.1x its best-20%

head_tail_dyn's best 20% of batches run at 3.53 us but its worst 20% at 14.59 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7122.8 ns median (-42.1% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 9.94x (fastest 7122.8 ns, slowest 70818.1 ns)

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
| Counter frequency | 2112783970 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 13030ns | 13052ns | 11584ns | 12625ns | 14360ns | base |
| head_tail_dyn | 11078ns | 13562ns | 3777ns | 10566ns | 15498ns | -14.98% |
| kernel_nway | 19951ns | 19695ns | 16649ns | 19206ns | 22718ns | +53.12% |
| kernel_seq | 10394ns | 10126ns | 8934ns | 9947ns | 11794ns | -20.23% |
| nway | 48606ns | 46703ns | 42636ns | 45732ns | 55902ns | +273.04% |
| sequential | 13225ns | 7262ns | 6934ns | 8274ns | 23799ns | +1.50% |
| std_parallel | 78674ns | 71031ns | 65348ns | 70249ns | 97974ns | +503.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 12189ns | 10955ns | 13204ns | base | 0.005 |
| head_tail_dyn | 10438ns | 3529ns | 14595ns | -14.36% | 0.006 |
| kernel_nway | 19028ns | 15936ns | 21621ns | +56.11% | 0.003 |
| kernel_seq | 9624ns | 8381ns | 10752ns | -21.04% | 0.007 |
| nway | 47998ns | 42142ns | 55185ns | +293.79% | 0.001 |
| sequential | 12954ns | 6798ns | 23294ns | +6.28% | 0.005 |
| std_parallel | 78336ns | 65068ns | 97493ns | +542.69% | 0.001 |

## Performance model

- Peak throughput: **0.018 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.005 | 28.7% |
| head_tail_dyn | 0.005 | 27.5% |
| kernel_nway | 0.003 | 18.7% |
| kernel_seq | 0.007 | 37.0% |
| nway | 0.001 | 7.7% |
| sequential | 0.009 | 49.5% |
| std_parallel | 0.001 | 5.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 13030ns | 13030ns | base |
| head_tail_dyn | 11078ns | 11078ns | -14.98% |
| kernel_nway | 19951ns | 19951ns | +53.12% |
| kernel_seq | 10394ns | 10394ns | -20.23% |
| nway | 48606ns | 48606ns | +273.04% |
| sequential | 13225ns | 13225ns | +1.50% |
| std_parallel | 78674ns | 78674ns | +503.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 12310ns | base | --- | [11398, 12689] | --- | --- | --- | --- |
| head_tail_dyn | 12841ns | no significant difference | [-6970, +1829]ns | [4164, 14278] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 18880ns | +6291.8ns (+51.1%) | [+5482, +7690]ns | [17267, 20136] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 9528ns | -2437.9ns (-19.8%) | [-3043, -2150]ns | [8814, 10467] | YES | 0.0000 | 0.0000 | 0 |
| nway | 46086ns | +34734.6ns (+282.2%) | [+31260, +39072]ns | [43210, 51624] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 7123ns | no significant difference | [-4798, +5719]ns | [6940, 18243] | no | 0.4807 | 0.4807 | 0 |
| std_parallel | 70818ns | +58906.1ns (+478.5%) | [+55162, +67208]ns | [67353, 79039] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 12254ns | +10.3% | +56.1% | -15.0% | +339.4% | -42.2% | +470.8% |
| 1 | 2 | 12210ns | -70.3% | +37.3% | -21.3% | +281.4% | -2.8% | +462.4% |
| 1 | 3 | 12400ns | +14.7% | +76.0% | -29.5% | +243.6% | -42.2% | +556.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.310 | moderate- |
| head_tail_dyn | -0.664 | HIGH- (thermal bounce) |
| kernel_nway | -0.477 | moderate- |
| kernel_seq | -0.000 | ok |
| nway | -0.019 | ok |
| sequential | -0.666 | HIGH- (thermal bounce) |
| std_parallel | -0.222 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 12798.0ns | 12188.8ns | 105.0% | HIGH |
| head_tail_dyn | 10953.5ns | 10437.9ns | 104.9% | HIGH |
| kernel_nway | 18760.2ns | 19027.8ns | 98.6% | HIGH |
| kernel_seq | 10199.0ns | 9623.9ns | 106.0% | HIGH |
| nway | 48716.8ns | 47997.9ns | 101.5% | HIGH |
| sequential | 13420.8ns | 12954.5ns | 103.6% | HIGH |
| std_parallel | 76477.1ns | 78336.3ns | 97.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 10954.9-13204.2 ns)
  10954.9 |########################################
  11067.4 |#############
  11179.9 |
  11292.3 |
  11404.8 |
  11517.3 |
  11629.7 |##########################
  11742.2 |
  11854.7 |
  11967.1 |
  12079.6 |#############
  12192.0 |#############
  12304.5 |#############
  12417.0 |#############
  12529.4 |#############
  12641.9 |########################################
  12754.4 |
  12866.8 |#############
  12979.3 |
  13091.8 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 3528.6-14594.6 ns)
   3528.6 |########################################
   4081.9 |##########
   4635.2 |
   5188.5 |
   5741.8 |
   6295.1 |
   6848.4 |
   7401.7 |
   7955.0 |
   8508.3 |
   9061.6 |
   9614.9 |
  10168.2 |
  10721.5 |
  11274.8 |
  11828.1 |
  12381.4 |########################################
  12934.7 |##########
  13488.0 |##########
  14041.3 |########################################
  (1 below, 2 above range)

kernel_nway (n=18, range 15936.2-21620.8 ns)
  15936.2 |####################
  16220.4 |
  16504.6 |####################
  16788.9 |
  17073.1 |####################
  17357.3 |####################
  17641.6 |
  17925.8 |####################
  18210.0 |####################
  18494.2 |
  18778.5 |########################################
  19062.7 |
  19346.9 |
  19631.2 |########################################
  19915.4 |####################
  20199.6 |####################
  20483.9 |
  20768.1 |
  21052.3 |
  21336.6 |
  (2 below, 4 above range)

kernel_seq (n=18, range 8381.2-10751.9 ns)
   8381.2 |####################
   8499.8 |
   8618.3 |####################
   8736.8 |####################
   8855.4 |####################
   8973.9 |########################################
   9092.4 |####################
   9211.0 |
   9329.5 |
   9448.0 |
   9566.6 |
   9685.1 |
   9803.6 |
   9922.2 |####################
  10040.7 |########################################
  10159.2 |####################
  10277.8 |
  10396.3 |
  10514.8 |
  10633.4 |####################
  (2 below, 4 above range)

nway (n=18, range 42141.9-55185.0 ns)
  42141.9 |########################################
  42794.1 |##########################
  43446.2 |
  44098.4 |##########################
  44750.5 |
  45402.7 |#############
  46054.8 |#############
  46707.0 |#############
  47359.1 |
  48011.3 |
  48663.4 |#############
  49315.6 |#############
  49967.8 |
  50619.9 |
  51272.1 |
  51924.2 |
  52576.4 |
  53228.5 |#############
  53880.7 |#############
  54532.8 |
  (1 below, 3 above range)

sequential (n=18, range 6798.0-23293.8 ns)
   6798.0 |########################################
   7622.8 |
   8447.6 |
   9272.3 |
  10097.1 |
  10921.9 |
  11746.7 |
  12571.5 |
  13396.3 |
  14221.1 |
  15045.9 |
  15870.7 |
  16695.5 |########
  17520.3 |
  18345.1 |
  19169.9 |####
  19994.6 |
  20819.4 |
  21644.2 |####
  22469.0 |
  (2 below, 3 above range)

std_parallel (n=18, range 65067.6-97493.1 ns)
  65067.6 |########################################
  66688.9 |##########################
  68310.1 |#############
  69931.4 |########################################
  71552.7 |#############
  73174.0 |
  74795.2 |#############
  76416.5 |
  78037.8 |#############
  79659.1 |#############
  81280.3 |#############
  82901.6 |
  84522.9 |
  86144.2 |#############
  87765.4 |
  89386.7 |
  91008.0 |
  92629.3 |
  94250.5 |
  95871.8 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=105.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=46.2% (high variance, measurements may be unstable)
- **head_tail_dyn**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **head_tail_dyn**: bridge=105.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=97.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=102.3% of algo (FFI overhead may distort results)
- **nway**: bridge=101.3% of algo (FFI overhead may distort results)
- **sequential**: CV=62.0% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **sequential**: bridge=102.5% of algo (FFI overhead may distort results)
- **std_parallel**: CV=25.9% (high variance, measurements may be unstable)
- **std_parallel**: bridge=98.6% of algo (FFI overhead may distort results)
