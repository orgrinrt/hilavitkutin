# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 38% faster than the next best (head_tail_dyn)

sequential (7.08 us) leads head_tail_dyn (9.75 us) by 38%, a clear separation rather than a photo finish. CV 124.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 4.9x slower than the field

std_parallel (34.52 us) is 4.9x the fastest (7.08 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 124.0%)

sequential wins on median (7.08 us) yet has the highest variance (CV 124.0%), while nway is the steadiest (CV 13.9%, 32.34 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.67)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, head_tail_dyn, head_tail, kernel_nway, kernel_seq} vs {nway, std_parallel} (127% apart)

The field splits into a fast tier {sequential, head_tail_dyn, head_tail, kernel_nway, kernel_seq} and a slow tier {nway, std_parallel} with a 127% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.9x the fastest

Fastest sequential (7.08 us) to slowest std_parallel (34.52 us): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.8x its best-20%

sequential's best 20% of batches run at 6.70 us but its worst 20% at 25.61 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7084.0 ns median (-33.9% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 4.87x (fastest 7084.0 ns, slowest 34522.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 1 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112789863 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 11274ns | 11282ns | 7747ns | 10447ns | 14280ns | base |
| head_tail_dyn | 12069ns | 10731ns | 8682ns | 10536ns | 16061ns | +7.04% |
| kernel_nway | 13218ns | 12829ns | 10983ns | 12267ns | 15761ns | +17.23% |
| kernel_seq | 14234ns | 15003ns | 8909ns | 13712ns | 17678ns | +26.25% |
| nway | 34109ns | 32812ns | 28887ns | 32463ns | 39188ns | +202.53% |
| sequential | 14323ns | 7223ns | 6834ns | 8948ns | 26129ns | +27.04% |
| std_parallel | 38863ns | 35108ns | 31908ns | 34479ns | 48917ns | +244.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 10512ns | 7255ns | 13298ns | base | 0.024 |
| head_tail_dyn | 11266ns | 8197ns | 15029ns | +7.17% | 0.023 |
| kernel_nway | 12319ns | 10377ns | 14518ns | +17.19% | 0.021 |
| kernel_seq | 13291ns | 8343ns | 16411ns | +26.43% | 0.019 |
| nway | 33607ns | 28485ns | 38608ns | +219.69% | 0.008 |
| sequential | 14037ns | 6700ns | 25606ns | +33.53% | 0.018 |
| std_parallel | 38178ns | 31406ns | 47996ns | +263.18% | 0.007 |

## Performance model

- Peak throughput: **0.038 Gops/s** (sequential; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.024 | 62.5% |
| head_tail_dyn | 0.026 | 68.7% |
| kernel_nway | 0.021 | 55.7% |
| kernel_seq | 0.018 | 46.9% |
| nway | 0.008 | 20.7% |
| sequential | 0.036 | 94.6% |
| std_parallel | 0.007 | 19.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 11274ns | 11274ns | base |
| head_tail_dyn | 12069ns | 12069ns | +7.04% |
| kernel_nway | 13218ns | 13218ns | +17.23% |
| kernel_seq | 14234ns | 14234ns | +26.25% |
| nway | 34109ns | 34109ns | +202.53% |
| sequential | 14323ns | 14323ns | +27.04% |
| std_parallel | 38863ns | 38863ns | +244.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 10712ns | base | --- | [8595, 11811] | --- | --- | --- | --- |
| head_tail_dyn | 9747ns | no significant difference | [-2575, +6584]ns | [9420, 13840] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 12037ns | +2122.7ns (+19.8%) | [+395, +2972]ns | [10822, 13410] | YES (adj: no) | 0.1925 | 0.0963 | 0 |
| kernel_seq | 14273ns | no significant difference | [-2589, +6331]ns | [10774, 15823] | no | 0.2855 | 0.2379 | 0 |
| nway | 32343ns | +21829.8ns (+203.8%) | [+20841, +23920]ns | [30848, 35434] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 7084ns | no significant difference | [-1734, +9090]ns | [6884, 21858] | no | 0.4807 | 0.4807 | 0 |
| std_parallel | 34522ns | +22793.7ns (+212.8%) | [+20741, +34120]ns | [31745, 42243] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 10712ns | -10.1% | +19.8% | +41.3% | +190.2% | +14.9% | +195.6% |
| 1 | 2 | 7635ns | +95.6% | +49.1% | +113.0% | +327.7% | -7.8% | +489.3% |
| 1 | 3 | 12339ns | -23.7% | -5.9% | -25.5% | +172.4% | -20.5% | +164.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.589 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.666 | HIGH- (thermal bounce) |
| kernel_nway | -0.256 | moderate- |
| kernel_seq | -0.258 | moderate- |
| nway | -0.013 | ok |
| sequential | -0.516 | HIGH- (thermal bounce) |
| std_parallel | -0.664 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 10807.5ns | 10512.3ns | 102.8% | HIGH |
| head_tail_dyn | 11690.2ns | 11266.5ns | 103.8% | HIGH |
| kernel_nway | 12854.4ns | 12319.4ns | 104.3% | HIGH |
| kernel_seq | 14012.2ns | 13290.9ns | 105.4% | HIGH |
| nway | 34875.1ns | 33606.6ns | 103.8% | HIGH |
| sequential | 14451.9ns | 14037.0ns | 103.0% | HIGH |
| std_parallel | 39156.7ns | 38178.4ns | 102.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 7254.9-13298.2 ns)
   7254.9 |#############
   7557.1 |
   7859.2 |#############
   8161.4 |
   8463.6 |##########################
   8765.7 |#############
   9067.9 |
   9370.1 |#############
   9672.2 |
   9974.4 |#############
  10276.6 |
  10578.7 |
  10880.9 |
  11183.1 |########################################
  11485.2 |#############
  11787.4 |#############
  12089.6 |#############
  12391.7 |
  12693.9 |
  12996.1 |#############
  (2 below, 2 above range)

head_tail_dyn (n=18, range 8197.2-15028.9 ns)
   8197.2 |####################
   8538.8 |##########
   8880.4 |
   9222.0 |####################
   9563.6 |########################################
   9905.1 |
  10246.7 |
  10588.3 |##########
  10929.9 |##########
  11271.5 |
  11613.1 |
  11954.6 |
  12296.2 |
  12637.8 |
  12979.4 |
  13321.0 |
  13662.6 |##############################
  14004.1 |
  14345.7 |
  14687.3 |
  (1 below, 3 above range)

kernel_nway (n=18, range 10377.1-14517.9 ns)
  10377.1 |
  10584.2 |########################################
  10791.2 |########################################
  10998.2 |
  11205.3 |########################################
  11412.3 |
  11619.4 |
  11826.4 |####################
  12033.4 |########################################
  12240.5 |####################
  12447.5 |
  12654.6 |
  12861.6 |
  13068.6 |
  13275.7 |########################################
  13482.7 |
  13689.7 |
  13896.8 |
  14103.8 |####################
  14310.9 |
  (2 below, 3 above range)

kernel_seq (n=18, range 8343.4-16411.4 ns)
   8343.4 |####################
   8746.8 |####################
   9150.2 |
   9553.6 |####################
   9957.0 |####################
  10360.4 |
  10763.8 |
  11167.2 |####################
  11570.6 |
  11974.0 |####################
  12377.4 |
  12780.8 |
  13184.2 |####################
  13587.6 |####################
  13991.0 |
  14394.4 |########################################
  14797.8 |
  15201.2 |####################
  15604.6 |########################################
  16008.0 |####################
  (1 below, 3 above range)

nway (n=18, range 28485.3-38607.6 ns)
  28485.3 |####################
  28991.4 |
  29497.6 |
  30003.7 |####################
  30509.8 |########################################
  31015.9 |########################################
  31522.0 |
  32028.1 |########################################
  32534.2 |
  33040.3 |
  33546.4 |
  34052.6 |####################
  34558.7 |########################################
  35064.8 |
  35570.9 |####################
  36077.0 |
  36583.1 |####################
  37089.2 |####################
  37595.3 |
  38101.4 |
  (2 below, 2 above range)

sequential (n=18, range 6699.6-25606.3 ns)
   6699.6 |########################################
   7644.9 |
   8590.3 |
   9535.6 |
  10480.9 |
  11426.3 |
  12371.6 |#####
  13316.9 |
  14262.3 |
  15207.6 |
  16153.0 |
  17098.3 |#####
  18043.6 |
  18989.0 |
  19934.3 |
  20879.6 |
  21825.0 |#####
  22770.3 |#####
  23715.6 |#####
  24661.0 |#####
  (2 below, 2 above range)

std_parallel (n=18, range 31406.5-47996.0 ns)
  31406.5 |########################################
  32236.0 |
  33065.5 |################
  33894.9 |
  34724.4 |########
  35553.9 |
  36383.4 |
  37212.8 |########
  38042.3 |########
  38871.8 |
  39701.3 |
  40530.7 |
  41360.2 |########
  42189.7 |########
  43019.2 |
  43848.7 |
  44678.1 |
  45507.6 |
  46337.1 |
  47166.6 |########
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=24.4% (high variance, measurements may be unstable)
- **head_tail**: bridge=103.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=25.3% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=102.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=104.2% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=23.2% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=102.1% of algo (FFI overhead may distort results)
- **nway**: bridge=102.7% of algo (FFI overhead may distort results)
- **sequential**: CV=62.6% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **sequential**: bridge=108.2% of algo (FFI overhead may distort results)
- **std_parallel**: CV=20.7% (high variance, measurements may be unstable)
- **std_parallel**: bridge=102.3% of algo (FFI overhead may distort results)
