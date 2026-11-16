# fold_light, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 3.0x slower than the field

std_parallel (25.48 us) is 3.0x the fastest (8.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail_dyn, head_tail) are a dead heat (<1%)

head_tail_dyn (8.56 us) and head_tail (8.62 us) differ by 0.78%, inside the noise, even though the wider field spreads 197.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### std_parallel shows alternating (throttle bounce) (autocorr -0.58)

std_parallel's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn, head_tail, kernel_nway, kernel_seq} vs {sequential, nway, std_parallel} (78% apart)

The field splits into a fast tier {head_tail_dyn, head_tail, kernel_nway, kernel_seq} and a slow tier {sequential, nway, std_parallel} with a 78% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### nway is inconsistent: worst-20% is 2.2x its best-20%

nway's best 20% of batches run at 18.51 us but its worst 20% at 39.95 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 8558.4 ns median (-0.8% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 2.98x (fastest 8558.4 ns, slowest 25477.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 1 |
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
| head_tail | 9145ns | 9067ns | 8828ns | 9019ns | 9493ns | base |
| head_tail_dyn | 9102ns | 9019ns | 8735ns | 8964ns | 9492ns | -0.47% |
| kernel_nway | 9198ns | 9192ns | 8754ns | 9128ns | 9526ns | +0.58% |
| kernel_seq | 9269ns | 9269ns | 8869ns | 9164ns | 9626ns | +1.36% |
| nway | 26610ns | 19640ns | 18869ns | 19725ns | 40808ns | +190.98% |
| sequential | 16008ns | 15900ns | 15629ns | 15864ns | 16412ns | +75.04% |
| std_parallel | 25930ns | 25898ns | 24521ns | 25432ns | 27381ns | +183.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 8688ns | 8361ns | 9019ns | base | 7.543 |
| head_tail_dyn | 8629ns | 8279ns | 8986ns | -0.68% | 7.595 |
| kernel_nway | 8795ns | 8458ns | 9080ns | +1.23% | 7.451 |
| kernel_seq | 8834ns | 8524ns | 9112ns | +1.68% | 7.419 |
| nway | 26056ns | 18514ns | 39955ns | +199.90% | 2.515 |
| sequential | 15789ns | 15438ns | 16123ns | +81.73% | 4.151 |
| std_parallel | 25537ns | 24126ns | 26987ns | +193.93% | 2.566 |

## Performance model

- Peak throughput: **7.916 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 7.598 | 96.0% |
| head_tail_dyn | 7.658 | 96.7% |
| kernel_nway | 7.456 | 94.2% |
| kernel_seq | 7.437 | 93.9% |
| nway | 3.397 | 42.9% |
| sequential | 4.169 | 52.7% |
| std_parallel | 2.572 | 32.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 9145ns | 9145ns | base |
| head_tail_dyn | 9102ns | 9102ns | -0.47% |
| kernel_nway | 9198ns | 9198ns | +0.58% |
| kernel_seq | 9269ns | 9269ns | +1.36% |
| nway | 26610ns | 26610ns | +190.98% |
| sequential | 16008ns | 16008ns | +75.04% |
| std_parallel | 25930ns | 25930ns | +183.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 8625ns | base | --- | [8485, 8888] | --- | --- | --- | --- |
| head_tail_dyn | 8558ns | no significant difference | [-273, +123]ns | [8448, 8638] | no | 0.7210 | 0.4807 | 0 |
| kernel_nway | 8790ns | no significant difference | [-106, +304]ns | [8631, 8967] | no | 0.8145 | 0.8145 | 0 |
| kernel_seq | 8812ns | no significant difference | [-87, +354]ns | [8654, 8919] | no | 0.8145 | 0.8145 | 0 |
| nway | 19294ns | +10508.4ns (+121.8%) | [+10179, +29546]ns | [18994, 38488] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 15721ns | +7087.5ns (+82.2%) | [+6783, +7346]ns | [15608, 15871] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 25477ns | +16779.2ns (+194.5%) | [+15679, +17462]ns | [24427, 26042] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 8965ns | -5.8% | -3.7% | -3.4% | +115.2% | +75.4% | +173.8% |
| 1 | 2 | 8485ns | +0.1% | +3.2% | +4.4% | +122.0% | +88.6% | +206.2% |
| 1 | 3 | 8704ns | -0.7% | +3.3% | +1.5% | +352.9% | +78.4% | +188.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.470 | moderate- |
| head_tail_dyn | -0.061 | ok |
| kernel_nway | -0.022 | ok |
| kernel_seq | -0.226 | moderate- |
| nway | -0.178 | ok |
| sequential | -0.553 | HIGH- (thermal bounce) |
| std_parallel | -0.578 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 0/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 91795.8ns | 8688.2ns | 1056.6% | HIGH |
| head_tail_dyn | 90403.5ns | 8628.9ns | 1047.7% | HIGH |
| kernel_nway | 91809.7ns | 8795.4ns | 1043.8% | HIGH |
| kernel_seq | 92287.3ns | 8833.8ns | 1044.7% | HIGH |
| nway | 101918.8ns | 26056.2ns | 391.1% | HIGH |
| sequential | 94574.8ns | 15788.7ns | 599.0% | HIGH |
| std_parallel | 99628.0ns | 25537.3ns | 390.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 8361.1-9018.8 ns)
   8361.1 |########################################
   8394.0 |
   8426.9 |####################
   8459.7 |########################################
   8492.6 |
   8525.5 |####################
   8558.4 |########################################
   8591.3 |
   8624.2 |
   8657.0 |########################################
   8689.9 |####################
   8722.8 |
   8755.7 |
   8788.6 |
   8821.5 |####################
   8854.3 |
   8887.2 |
   8920.1 |########################################
   8953.0 |####################
   8985.9 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 8279.2-8986.1 ns)
   8279.2 |#############
   8314.5 |
   8349.9 |##########################
   8385.2 |#############
   8420.6 |
   8455.9 |#############
   8491.2 |##########################
   8526.6 |#############
   8561.9 |#############
   8597.3 |##########################
   8632.6 |########################################
   8668.0 |
   8703.3 |
   8738.7 |#############
   8774.0 |
   8809.4 |
   8844.7 |
   8880.1 |
   8915.4 |
   8950.8 |
  (1 below, 2 above range)

kernel_nway (n=18, range 8458.3-9079.9 ns)
   8458.3 |#############
   8489.4 |
   8520.5 |
   8551.5 |#############
   8582.6 |##########################
   8613.7 |
   8644.8 |#############
   8675.8 |#############
   8706.9 |#############
   8738.0 |
   8769.1 |########################################
   8800.2 |#############
   8831.2 |
   8862.3 |
   8893.4 |
   8924.5 |
   8955.5 |##########################
   8986.6 |#############
   9017.7 |
   9048.8 |
  (1 below, 3 above range)

kernel_seq (n=18, range 8523.6-9111.8 ns)
   8523.6 |########################################
   8553.0 |########################################
   8582.4 |
   8611.8 |
   8641.2 |
   8670.7 |
   8700.1 |
   8729.5 |########################################
   8758.9 |########################################
   8788.3 |
   8817.7 |####################
   8847.1 |####################
   8876.5 |########################################
   8905.9 |####################
   8935.4 |####################
   8964.8 |####################
   8994.2 |
   9023.6 |
   9053.0 |
   9082.4 |
  (1 below, 2 above range)

nway (n=18, range 18513.9-39954.8 ns)
  18513.9 |########################################
  19585.9 |
  20658.0 |####
  21730.0 |
  22802.1 |
  23874.1 |
  24946.2 |
  26018.2 |
  27090.3 |
  28162.3 |
  29234.4 |
  30306.4 |
  31378.4 |
  32450.5 |
  33522.5 |
  34594.6 |
  35666.6 |
  36738.7 |
  37810.7 |########
  38882.8 |########
  (2 below, 2 above range)

sequential (n=18, range 15437.5-16122.9 ns)
  15437.5 |##########
  15471.8 |
  15506.1 |##########
  15540.3 |####################
  15574.6 |
  15608.9 |
  15643.1 |##########
  15677.4 |##########
  15711.7 |########################################
  15746.0 |
  15780.2 |##########
  15814.5 |##########
  15848.8 |
  15883.0 |
  15917.3 |##########
  15951.6 |
  15985.8 |####################
  16020.1 |
  16054.4 |
  16088.6 |
  (1 below, 2 above range)

std_parallel (n=18, range 24126.4-26986.8 ns)
  24126.4 |####################
  24269.4 |########################################
  24412.4 |####################
  24555.5 |####################
  24698.5 |####################
  24841.5 |
  24984.5 |
  25127.6 |
  25270.6 |####################
  25413.6 |
  25556.6 |####################
  25699.6 |
  25842.7 |########################################
  25985.7 |########################################
  26128.7 |
  26271.7 |
  26414.7 |
  26557.8 |####################
  26700.8 |
  26843.8 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=1060.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=1044.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1049.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1045.6% of algo (FFI overhead may distort results)
- **nway**: CV=38.0% (high variance, measurements may be unstable)
- **nway**: bridge=492.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=601.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=391.1% of algo (FFI overhead may distort results)
