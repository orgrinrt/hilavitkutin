# fold_light, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 6.6x slower than the field

std_parallel (57.33 us) is 6.6x the fastest (8.65 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.64)

head_tail's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, head_tail, head_tail_dyn, sequential, kernel_nway} vs {nway, std_parallel} (91% apart)

The field splits into a fast tier {kernel_seq, head_tail, head_tail_dyn, sequential, kernel_nway} and a slow tier {nway, std_parallel} with a 91% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.6x the fastest

Fastest kernel_seq (8.65 us) to slowest std_parallel (57.33 us): 6.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### nway is inconsistent: worst-20% is 3.0x its best-20%

nway's best 20% of batches run at 17.07 us but its worst 20% at 51.10 us (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 8650.0 ns median (-1.1% vs baseline)
- 5 variants significantly slower than baseline
- Spread: 6.63x (fastest 8650.0 ns, slowest 57331.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 4 |
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
| head_tail | 9764ns | 9419ns | 8867ns | 9368ns | 10806ns | base |
| head_tail_dyn | 12788ns | 12750ns | 12367ns | 12696ns | 13136ns | +30.97% |
| kernel_nway | 21777ns | 21675ns | 17232ns | 21009ns | 25201ns | +123.03% |
| kernel_seq | 9152ns | 9094ns | 8667ns | 9001ns | 9622ns | -6.26% |
| nway | 38002ns | 40683ns | 17325ns | 35821ns | 51613ns | +289.21% |
| sequential | 15760ns | 15754ns | 15262ns | 15635ns | 16197ns | +61.42% |
| std_parallel | 54355ns | 57752ns | 41400ns | 54047ns | 61296ns | +456.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9145ns | 8262ns | 10194ns | base | 7.166 |
| head_tail_dyn | 12296ns | 11954ns | 12616ns | +34.45% | 5.330 |
| kernel_nway | 21158ns | 16632ns | 24576ns | +131.36% | 3.097 |
| kernel_seq | 8732ns | 8317ns | 9170ns | -4.52% | 7.506 |
| nway | 37540ns | 17067ns | 51099ns | +310.49% | 1.746 |
| sequential | 15574ns | 15076ns | 16004ns | +70.30% | 4.208 |
| std_parallel | 53984ns | 41132ns | 60874ns | +490.30% | 1.214 |

## Performance model

- Peak throughput: **7.932 Gops/s** (head_tail; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 7.495 | 94.5% |
| head_tail_dyn | 5.327 | 67.2% |
| kernel_nway | 3.114 | 39.3% |
| kernel_seq | 7.576 | 95.5% |
| nway | 1.635 | 20.6% |
| sequential | 4.208 | 53.0% |
| std_parallel | 1.143 | 14.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 9764ns | 9764ns | base |
| head_tail_dyn | 12788ns | 12788ns | +30.97% |
| kernel_nway | 21777ns | 21777ns | +123.03% |
| kernel_seq | 9152ns | 9152ns | -6.26% |
| nway | 38002ns | 38002ns | +289.21% |
| sequential | 15760ns | 15760ns | +61.42% |
| std_parallel | 54355ns | 54355ns | +456.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 8744ns | base | --- | [8531, 9735] | --- | --- | --- | --- |
| head_tail_dyn | 12302ns | +3722.9ns (+42.6%) | [+2473, +3875]ns | [12062, 12502] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 21044ns | +11812.6ns (+135.1%) | [+9854, +14225]ns | [18879, 23600] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 8650ns | no significant difference | [-1175, +390]ns | [8448, 8854] | no | 0.8145 | 0.8145 | 0 |
| nway | 40092ns | +31264.6ns (+357.6%) | [+21988, +34879]ns | [30225, 44556] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 15575ns | +6710.4ns (+76.7%) | [+5458, +7271]ns | [15233, 15750] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 57331ns | +47110.4ns (+538.8%) | [+40271, +50237]ns | [47471, 59596] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 8729ns | +43.2% | +103.4% | -0.0% | +352.0% | +78.0% | +596.9% |
| 1 | 2 | 9960ns | +20.5% | +129.4% | -15.4% | +386.8% | +52.9% | +477.5% |
| 1 | 3 | 8435ns | +46.5% | +177.9% | +5.8% | +103.9% | +87.0% | +417.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.645 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.615 | HIGH- (thermal bounce) |
| kernel_nway | -0.115 | ok |
| kernel_seq | -0.565 | HIGH- (thermal bounce) |
| nway | -0.348 | moderate- |
| sequential | -0.542 | HIGH- (thermal bounce) |
| std_parallel | -0.075 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 89106.0ns | 9145.1ns | 974.4% | HIGH |
| head_tail_dyn | 94511.6ns | 12295.8ns | 768.6% | HIGH |
| kernel_nway | 104257.2ns | 21158.3ns | 492.7% | HIGH |
| kernel_seq | 90377.3ns | 8731.7ns | 1035.0% | HIGH |
| nway | 107609.3ns | 37539.6ns | 286.7% | HIGH |
| sequential | 93306.5ns | 15574.3ns | 599.1% | HIGH |
| std_parallel | 118975.9ns | 53983.8ns | 220.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 8262.5-10193.8 ns)
   8262.5 |########################################
   8359.1 |
   8455.6 |##########################
   8552.2 |##########################
   8648.8 |#############
   8745.3 |#############
   8841.9 |
   8938.4 |#############
   9035.0 |
   9131.6 |
   9228.1 |
   9324.7 |
   9421.2 |
   9517.8 |#############
   9614.4 |
   9710.9 |##########################
   9807.5 |#############
   9904.1 |
  10000.6 |
  10097.2 |##########################
  (1 below, 1 above range)

head_tail_dyn (n=18, range 11954.2-12616.0 ns)
  11954.2 |########################################
  11987.3 |#############
  12020.3 |
  12053.4 |
  12086.5 |#############
  12119.6 |#############
  12152.7 |
  12185.8 |#############
  12218.9 |
  12252.0 |
  12285.1 |########################################
  12318.2 |
  12351.2 |
  12384.3 |#############
  12417.4 |
  12450.5 |
  12483.6 |##########################
  12516.7 |#############
  12549.8 |#############
  12582.9 |
  (1 below, 2 above range)

kernel_nway (n=18, range 16631.9-24575.7 ns)
  16631.9 |
  17029.1 |####################
  17426.3 |####################
  17823.5 |####################
  18220.7 |
  18617.9 |####################
  19015.1 |####################
  19412.2 |
  19809.4 |
  20206.6 |########################################
  20603.8 |
  21001.0 |########################################
  21398.2 |
  21795.4 |####################
  22192.6 |
  22589.7 |####################
  22986.9 |
  23384.1 |####################
  23781.3 |########################################
  24178.5 |
  (1 below, 3 above range)

kernel_seq (n=18, range 8316.7-9170.2 ns)
   8316.7 |#############
   8359.3 |#############
   8402.0 |##########################
   8444.7 |#############
   8487.4 |#############
   8530.0 |
   8572.7 |
   8615.4 |########################################
   8658.1 |#############
   8700.7 |
   8743.4 |#############
   8786.1 |
   8828.8 |#############
   8871.4 |##########################
   8914.1 |
   8956.8 |#############
   8999.5 |
   9042.1 |
   9084.8 |
   9127.5 |
  (1 below, 2 above range)

nway (n=18, range 17066.7-51099.3 ns)
  17066.7 |########################################
  18768.3 |
  20469.9 |
  22171.6 |#############
  23873.2 |
  25574.8 |
  27276.5 |
  28978.1 |
  30679.7 |
  32381.4 |
  34083.0 |
  35784.6 |#############
  37486.2 |##########################
  39187.9 |##########################
  40889.5 |##########################
  42591.1 |#############
  44292.8 |
  45994.4 |##########################
  47696.0 |
  49397.7 |#############
  (1 below, 2 above range)

sequential (n=18, range 15076.4-16004.2 ns)
  15076.4 |####################
  15122.8 |########################################
  15169.2 |####################
  15215.6 |
  15262.0 |####################
  15308.3 |####################
  15354.7 |
  15401.1 |
  15447.5 |
  15493.9 |####################
  15540.3 |####################
  15586.7 |####################
  15633.1 |
  15679.4 |########################################
  15725.8 |########################################
  15772.2 |########################################
  15818.6 |
  15865.0 |
  15911.4 |
  15957.8 |
  (1 below, 2 above range)

std_parallel (n=18, range 41132.0-60873.6 ns)
  41132.0 |#############
  42119.1 |
  43106.1 |
  44093.2 |
  45080.3 |#############
  46067.4 |#############
  47054.5 |
  48041.6 |#############
  49028.6 |
  50015.7 |
  51002.8 |
  51989.9 |
  52977.0 |
  53964.1 |
  54951.1 |#############
  55938.2 |##########################
  56925.3 |#############
  57912.4 |########################################
  58899.5 |
  59886.6 |##########################
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=1013.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=764.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=465.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1046.4% of algo (FFI overhead may distort results)
- **nway**: CV=36.0% (high variance, measurements may be unstable)
- **nway**: bridge=269.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=599.2% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=209.2% of algo (FFI overhead may distort results)
