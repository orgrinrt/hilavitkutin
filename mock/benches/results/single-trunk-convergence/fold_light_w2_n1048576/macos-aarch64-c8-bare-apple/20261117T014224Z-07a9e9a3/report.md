# fold_light, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail dominates: 31% faster than the next best (kernel_nway)

head_tail (57.22 us) leads kernel_nway (74.93 us) by 31%, a clear separation rather than a photo finish. CV 17.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### head_tail_dyn is an outlier: 2.4x slower than the field

head_tail_dyn (136.51 us) is 2.4x the fastest (57.22 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail is fastest but the noisiest (CV 17.6%)

head_tail wins on median (57.22 us) yet has the highest variance (CV 17.6%), while kernel_seq is the steadiest (CV 0.4%, 94.27 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### nway shows alternating (throttle bounce) (autocorr -0.54)

nway's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (57.22 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (head_tail) is the fastest** at 57218.8 ns median
- 5 variants significantly slower than baseline
- Spread: 2.39x (fastest 57218.8 ns, slowest 136508.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 2 |
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
| head_tail | 62364ns | 57885ns | 50151ns | 58009ns | 75003ns | base |
| head_tail_dyn | 137138ns | 136802ns | 135553ns | 136731ns | 138542ns | +119.90% |
| kernel_nway | 73720ns | 75621ns | 66075ns | 74723ns | 76038ns | +18.21% |
| kernel_seq | 94496ns | 94452ns | 94018ns | 94413ns | 94860ns | +51.52% |
| nway | 79870ns | 79877ns | 78767ns | 79657ns | 80740ns | +28.07% |
| sequential | 99939ns | 98329ns | 97353ns | 98294ns | 103699ns | +60.25% |
| std_parallel | 78891ns | 77373ns | 75157ns | 76878ns | 83779ns | +26.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 61825ns | 49938ns | 74362ns | base | 16.960 |
| head_tail_dyn | 136824ns | 135274ns | 138171ns | +121.31% | 7.664 |
| kernel_nway | 73059ns | 65340ns | 75420ns | +18.17% | 14.352 |
| kernel_seq | 94290ns | 93833ns | 94608ns | +52.51% | 11.121 |
| nway | 79270ns | 78206ns | 80154ns | +28.22% | 13.228 |
| sequential | 99730ns | 97160ns | 103468ns | +61.31% | 10.514 |
| std_parallel | 78488ns | 74751ns | 83367ns | +26.95% | 13.360 |

## Performance model

- Peak throughput: **20.998 Gops/s** (head_tail; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 18.326 | 87.3% |
| head_tail_dyn | 7.681 | 36.6% |
| kernel_nway | 13.994 | 66.6% |
| kernel_seq | 11.123 | 53.0% |
| nway | 13.238 | 63.0% |
| sequential | 10.687 | 50.9% |
| std_parallel | 13.620 | 64.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 62364ns | 62364ns | base |
| head_tail_dyn | 137138ns | 137138ns | +119.90% |
| kernel_nway | 73720ns | 73720ns | +18.21% |
| kernel_seq | 94496ns | 94496ns | +51.52% |
| nway | 79870ns | 79870ns | +28.07% |
| sequential | 99939ns | 99939ns | +60.25% |
| std_parallel | 78891ns | 78891ns | +26.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 57219ns | base | --- | [53523, 73983] | --- | --- | --- | --- |
| head_tail_dyn | 136508ns | +79229.1ns (+138.5%) | [+64340, +84083]ns | [136310, 137090] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 74929ns | no significant difference | [-1212, +21771]ns | [72988, 75283] | no | 0.2379 | 0.2379 | 0 |
| kernel_seq | 94273ns | +37022.9ns (+64.7%) | [+20885, +40608]ns | [94173, 94358] | YES | 0.0000 | 0.0000 | 0 |
| nway | 79212ns | +21631.2ns (+37.8%) | [+5558, +25192]ns | [78773, 79548] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 98121ns | +41354.2ns (+72.3%) | [+23821, +47144]ns | [97517, 101838] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 76985ns | +17952.1ns (+31.4%) | [+10567, +23950]ns | [75531, 81006] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 74373ns | +83.1% | -5.9% | +26.9% | +6.7% | +30.9% | +12.1% |
| 1 | 2 | 56346ns | +142.5% | +33.5% | +67.2% | +40.3% | +73.7% | +36.4% |
| 1 | 3 | 50310ns | +171.4% | +49.6% | +87.3% | +57.5% | +104.0% | +49.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.051 | ok |
| head_tail_dyn | -0.294 | moderate- |
| kernel_nway | -0.162 | ok |
| kernel_seq | -0.160 | ok |
| nway | -0.536 | HIGH- (thermal bounce) |
| sequential | -0.120 | ok |
| std_parallel | -0.067 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 122369.2ns | 61825.2ns | 197.9% | HIGH |
| head_tail_dyn | 136819.0ns | 136823.8ns | 100.0% | HIGH |
| kernel_nway | 144010.2ns | 73058.8ns | 197.1% | HIGH |
| kernel_seq | 94243.5ns | 94290.3ns | 100.0% | HIGH |
| nway | 155608.3ns | 79269.9ns | 196.3% | HIGH |
| sequential | 99950.5ns | 99730.1ns | 100.2% | HIGH |
| std_parallel | 148537.5ns | 78487.5ns | 189.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 49937.5-74362.5 ns)
  49937.5 |##########################
  51158.8 |
  52380.0 |#############
  53601.2 |#############
  54822.5 |#############
  56043.8 |##########################
  57265.0 |#############
  58486.2 |
  59707.5 |
  60928.8 |#############
  62150.0 |
  63371.2 |
  64592.5 |
  65813.8 |
  67035.0 |
  68256.2 |
  69477.5 |
  70698.8 |#############
  71920.0 |
  73141.2 |########################################
  (2 below, 3 above range)

head_tail_dyn (n=18, range 135273.6-138170.8 ns)
  135273.6 |
  135418.5 |##########
  135563.3 |
  135708.2 |
  135853.0 |
  135997.9 |##########
  136142.8 |####################
  136287.6 |####################
  136432.5 |########################################
  136577.3 |##########
  136722.2 |
  136867.1 |##########
  137011.9 |
  137156.8 |##########
  137301.7 |
  137446.5 |
  137591.4 |
  137736.2 |
  137881.1 |
  138026.0 |##########
  (1 below, 3 above range)

kernel_nway (n=18, range 65340.3-75420.1 ns)
  65340.3 |
  65844.3 |#####
  66348.3 |
  66852.2 |
  67356.2 |#####
  67860.2 |
  68364.2 |
  68868.2 |
  69372.2 |
  69876.2 |
  70380.2 |
  70884.2 |
  71388.2 |
  71892.2 |#####
  72396.2 |#####
  72900.2 |
  73404.2 |##########
  73908.2 |#####
  74412.1 |#####
  74916.1 |########################################
  (1 below, 1 above range)

kernel_seq (n=18, range 93833.3-94607.7 ns)
  93833.3 |
  93872.0 |
  93910.8 |
  93949.5 |
  93988.2 |
  94026.9 |
  94065.6 |#############
  94104.3 |
  94143.1 |########################################
  94181.8 |#############
  94220.5 |#############
  94259.2 |########################################
  94297.9 |##########################
  94336.6 |
  94375.4 |########################################
  94414.1 |
  94452.8 |
  94491.5 |
  94530.2 |
  94568.9 |
  (2 below, 2 above range)

nway (n=18, range 78205.6-80154.2 ns)
  78205.6 |
  78303.0 |##########
  78400.4 |
  78497.9 |####################
  78595.3 |##########
  78692.7 |
  78790.1 |
  78887.6 |##########
  78985.0 |##########
  79082.4 |
  79179.9 |########################################
  79277.3 |##########
  79374.7 |##########
  79472.2 |
  79569.6 |
  79667.0 |##########
  79764.4 |
  79861.9 |
  79959.3 |##########
  80056.7 |
  (1 below, 3 above range)

sequential (n=18, range 97159.7-103468.1 ns)
  97159.7 |########################################
  97475.2 |##########################
  97790.6 |##########################
  98106.0 |#############
  98421.4 |#############
  98736.8 |
  99052.2 |
  99367.7 |
  99683.1 |#############
  99998.5 |
  100313.9 |
  100629.3 |
  100944.7 |
  101260.1 |
  101575.6 |##########################
  101891.0 |
  102206.4 |
  102521.8 |
  102837.2 |#############
  103152.6 |#############
  (2 below, 2 above range)

std_parallel (n=18, range 74751.4-83367.4 ns)
  74751.4 |########################################
  75182.2 |########################################
  75613.0 |####################
  76043.8 |########################################
  76474.6 |####################
  76905.4 |####################
  77336.2 |####################
  77767.0 |####################
  78197.8 |
  78628.6 |
  79059.4 |
  79490.2 |####################
  79921.0 |
  80351.8 |
  80782.6 |
  81213.4 |
  81644.2 |
  82075.0 |####################
  82505.8 |####################
  82936.6 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=203.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.1% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=199.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.9% of algo (FFI overhead may distort results)
- **nway**: bridge=198.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=197.2% of algo (FFI overhead may distort results)
