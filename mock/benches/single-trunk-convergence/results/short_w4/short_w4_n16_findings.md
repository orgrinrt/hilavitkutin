# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 163% faster than the next best (kernel_seq)

sequential (7.55 us) leads kernel_seq (19.86 us) by 163%, a clear separation rather than a photo finish. CV 20.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 76% (significant)

sequential is -23.58 us (76%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 13.1x slower than the field

std_parallel (98.97 us) is 13.1x the fastest (7.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.57)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {kernel_seq, head_tail_dyn, head_tail, kernel_nway, nway, std_parallel} (163% apart)

The field splits into a fast tier {sequential} and a slow tier {kernel_seq, head_tail_dyn, head_tail, kernel_nway, nway, std_parallel} with a 163% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 13.1x the fastest

Fastest sequential (7.55 us) to slowest std_parallel (98.97 us): 13.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 3.0x its best-20%

kernel_nway's best 20% of batches run at 36.57 us but its worst 20% at 110.02 us (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7551.3 ns median (-75.8% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 13.11x (fastest 7551.3 ns, slowest 98966.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 4 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100312120 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 34976ns | 31712ns | 24307ns | 30388ns | 47192ns | base |
| head_tail_dyn | 31592ns | 30662ns | 26352ns | 29674ns | 37090ns | -9.67% |
| kernel_nway | 63140ns | 40485ns | 37122ns | 40190ns | 110574ns | +80.52% |
| kernel_seq | 20912ns | 20222ns | 19037ns | 20022ns | 23183ns | -40.21% |
| nway | 59355ns | 59399ns | 46847ns | 54316ns | 73169ns | +69.70% |
| sequential | 8705ns | 7789ns | 7233ns | 7727ns | 10908ns | -75.11% |
| std_parallel | 99760ns | 99402ns | 80453ns | 93341ns | 119043ns | +185.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 34483ns | 23926ns | 46667ns | base | 0.000 |
| head_tail_dyn | 31137ns | 25888ns | 36641ns | -9.70% | 0.001 |
| kernel_nway | 62662ns | 36567ns | 110024ns | +81.72% | 0.000 |
| kernel_seq | 20481ns | 18673ns | 22711ns | -40.61% | 0.001 |
| nway | 58960ns | 46521ns | 72723ns | +70.98% | 0.000 |
| sequential | 8444ns | 7012ns | 10598ns | -75.51% | 0.002 |
| std_parallel | 99228ns | 79972ns | 118390ns | +187.76% | 0.000 |

## Performance model

- Peak throughput: **0.002 Gops/s** (sequential; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.001 | 22.5% |
| head_tail_dyn | 0.001 | 23.2% |
| kernel_nway | 0.000 | 17.5% |
| kernel_seq | 0.001 | 35.3% |
| nway | 0.000 | 11.9% |
| sequential | 0.002 | 92.9% |
| std_parallel | 0.000 | 7.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 34976ns | 34976ns | base |
| head_tail_dyn | 31592ns | 31592ns | -9.67% |
| kernel_nway | 63140ns | 63140ns | +80.52% |
| kernel_seq | 20912ns | 20912ns | -40.21% |
| nway | 59355ns | 59355ns | +69.70% |
| sequential | 8705ns | 8705ns | -75.11% |
| std_parallel | 99760ns | 99760ns | +185.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 31228ns | base | --- | [27670, 33299] | --- | --- | --- | --- |
| head_tail_dyn | 30215ns | no significant difference | [-4170, +2977]ns | [28011, 32997] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 40088ns | +7864.7ns (+25.2%) | [+5750, +18343]ns | [38071, 46572] | YES | 0.0090 | 0.0075 | 0 |
| kernel_seq | 19859ns | -11635.8ns (-37.3%) | [-14466, -7824]ns | [19007, 21294] | YES | 0.0000 | 0.0000 | 0 |
| nway | 58981ns | +21655.4ns (+69.3%) | [+14808, +34406]ns | [48666, 64732] | YES | 0.0020 | 0.0013 | 0 |
| sequential | 7551ns | -23577.2ns (-75.5%) | [-25482, -20456]ns | [7070, 10315] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 98967ns | +59989.1ns (+192.1%) | [+54156, +75226]ns | [83548, 109439] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 41985ns | -26.3% | -7.6% | -46.4% | +27.3% | -74.8% | +167.6% |
| 1 | 2 | 32075ns | +5.8% | +24.8% | -40.4% | +49.4% | -77.0% | +194.3% |
| 1 | 3 | 27670ns | +5.4% | +215.3% | -30.0% | +135.5% | -74.0% | +199.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.031 | ok |
| head_tail_dyn | -0.575 | HIGH- (thermal bounce) |
| kernel_nway | -0.154 | ok |
| kernel_seq | -0.206 | moderate- |
| nway | -0.371 | moderate- |
| sequential | -0.138 | ok |
| std_parallel | -0.010 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 33000.3ns | 34483.1ns | 95.7% | HIGH |
| head_tail_dyn | 31344.4ns | 31136.8ns | 100.7% | HIGH |
| kernel_nway | 48393.7ns | 62662.3ns | 77.2% | HIGH |
| kernel_seq | 21409.6ns | 20480.8ns | 104.5% | HIGH |
| nway | 55998.1ns | 58960.0ns | 95.0% | HIGH |
| sequential | 8887.4ns | 8444.0ns | 105.3% | HIGH |
| std_parallel | 98842.4ns | 99227.6ns | 99.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 23926.1-46666.9 ns)
  23926.1 |#############
  25063.1 |#############
  26200.1 |#############
  27337.2 |##########################
  28474.2 |#############
  29611.3 |#############
  30748.3 |##########################
  31885.4 |########################################
  33022.4 |#############
  34159.4 |
  35296.5 |
  36433.5 |#############
  37570.6 |
  38707.6 |
  39844.6 |
  40981.7 |
  42118.7 |
  43255.8 |
  44392.8 |
  45529.8 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 25887.7-36640.8 ns)
  25887.7 |#############
  26425.4 |#############
  26963.0 |#############
  27500.7 |#############
  28038.3 |##########################
  28576.0 |#############
  29113.6 |
  29651.3 |
  30188.9 |########################################
  30726.6 |
  31264.2 |#############
  31801.9 |
  32339.5 |
  32877.2 |##########################
  33414.8 |
  33952.5 |
  34490.1 |
  35027.8 |
  35565.4 |
  36103.1 |
  (1 below, 4 above range)

kernel_nway (n=18, range 36567.4-110023.8 ns)
  36567.4 |########################################
  40240.2 |####
  43913.1 |#############
  47585.9 |####
  51258.7 |
  54931.5 |
  58604.3 |
  62277.1 |
  65950.0 |
  69622.8 |
  73295.6 |
  76968.4 |
  80641.2 |
  84314.0 |
  87986.9 |
  91659.7 |
  95332.5 |
  99005.3 |
  102678.1 |
  106350.9 |
  (1 below, 3 above range)

kernel_seq (n=18, range 18672.8-22711.0 ns)
  18672.8 |##########
  18874.7 |########################################
  19076.7 |##########
  19278.6 |
  19480.5 |
  19682.4 |####################
  19884.3 |##########
  20086.2 |##########
  20288.1 |
  20490.0 |##########
  20691.9 |##########
  20893.9 |
  21095.8 |
  21297.7 |
  21499.6 |
  21701.5 |##########
  21903.4 |
  22105.3 |##########
  22307.2 |
  22509.1 |
  (1 below, 3 above range)

nway (n=18, range 46520.7-72723.3 ns)
  46520.7 |########################################
  47830.9 |########################################
  49141.0 |#############
  50451.1 |
  51761.2 |
  53071.4 |
  54381.5 |
  55691.6 |
  57001.8 |#############
  58311.9 |
  59622.0 |#############
  60932.1 |#############
  62242.3 |#############
  63552.4 |#############
  64862.5 |##########################
  66172.7 |
  67482.8 |
  68792.9 |#############
  70103.0 |#############
  71413.2 |
  (1 below, 1 above range)

sequential (n=18, range 7011.7-10597.5 ns)
   7011.7 |########################################
   7191.0 |##########
   7370.3 |####################
   7549.6 |##########
   7728.9 |
   7908.2 |##########
   8087.5 |##########
   8266.8 |
   8446.0 |
   8625.3 |
   8804.6 |
   8983.9 |
   9163.2 |
   9342.5 |
   9521.8 |
   9701.1 |
   9880.4 |
  10059.6 |
  10238.9 |####################
  10418.2 |##########
  (2 below, 3 above range)

std_parallel (n=18, range 79972.0-118389.9 ns)
  79972.0 |##########
  81892.9 |########################################
  83813.8 |##########
  85734.7 |
  87655.6 |
  89576.5 |
  91497.4 |##########
  93418.3 |
  95339.2 |##########
  97260.1 |
  99180.9 |
  101101.8 |##########
  103022.7 |##########
  104943.6 |##########
  106864.5 |
  108785.4 |####################
  110706.3 |
  112627.2 |##########
  114548.1 |##########
  116469.0 |##########
  (1 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=33.9% (high variance, measurements may be unstable)
- **head_tail**: bridge=95.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=98.9% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=86.1% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 3.0x (possible bimodal distribution)
- **kernel_nway**: bridge=101.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=102.4% of algo (FFI overhead may distort results)
- **nway**: CV=22.3% (high variance, measurements may be unstable)
- **nway**: bridge=94.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=102.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=90.3% of algo (FFI overhead may distort results)
