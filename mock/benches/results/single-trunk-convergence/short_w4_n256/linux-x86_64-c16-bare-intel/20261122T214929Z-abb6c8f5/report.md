# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 21% faster than the next best (sequential)

kernel_seq (10.64 us) leads sequential (12.83 us) by 21%, a clear separation rather than a photo finish. CV 15.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 5.0x slower than the field

std_parallel (53.11 us) is 5.0x the fastest (10.64 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.67)

nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, sequential, head_tail_dyn, head_tail, kernel_nway} vs {nway, std_parallel} (207% apart)

The field splits into a fast tier {kernel_seq, sequential, head_tail_dyn, head_tail, kernel_nway} and a slow tier {nway, std_parallel} with a 207% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.0x the fastest

Fastest kernel_seq (10.64 us) to slowest std_parallel (53.11 us): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.6x its best-20%

sequential's best 20% of batches run at 6.79 us but its worst 20% at 24.43 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 10636.4 ns median (-19.2% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 4.99x (fastest 10636.4 ns, slowest 53109.4 ns)

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
| Counter frequency | 2112732635 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 14326ns | 13743ns | 11834ns | 13793ns | 16371ns | base |
| head_tail_dyn | 13703ns | 13708ns | 11859ns | 13166ns | 15431ns | -4.35% |
| kernel_nway | 15023ns | 14249ns | 11285ns | 13644ns | 18960ns | +4.87% |
| kernel_seq | 10979ns | 11194ns | 8272ns | 10442ns | 13139ns | -23.36% |
| nway | 47483ns | 42359ns | 37303ns | 41738ns | 61190ns | +231.46% |
| sequential | 15068ns | 13096ns | 6920ns | 11178ns | 24978ns | +5.18% |
| std_parallel | 66983ns | 53490ns | 37509ns | 48357ns | 109658ns | +367.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 13542ns | 11145ns | 15373ns | base | 0.019 |
| head_tail_dyn | 12917ns | 11249ns | 14554ns | -4.62% | 0.020 |
| kernel_nway | 14284ns | 10775ns | 18094ns | +5.48% | 0.018 |
| kernel_seq | 10263ns | 7830ns | 12110ns | -24.22% | 0.025 |
| nway | 46829ns | 36841ns | 60268ns | +245.80% | 0.005 |
| sequential | 14755ns | 6785ns | 24434ns | +8.95% | 0.017 |
| std_parallel | 66440ns | 37230ns | 108601ns | +390.61% | 0.004 |

## Performance model

- Peak throughput: **0.038 Gops/s** (sequential; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.019 | 51.5% |
| head_tail_dyn | 0.020 | 52.1% |
| kernel_nway | 0.019 | 49.8% |
| kernel_seq | 0.024 | 63.8% |
| nway | 0.006 | 16.2% |
| sequential | 0.020 | 52.9% |
| std_parallel | 0.005 | 12.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 14326ns | 14326ns | base |
| head_tail_dyn | 13703ns | 13703ns | -4.35% |
| kernel_nway | 15023ns | 15023ns | +4.87% |
| kernel_seq | 10979ns | 10979ns | -23.36% |
| nway | 47483ns | 47483ns | +231.46% |
| sequential | 15068ns | 15068ns | +5.18% |
| std_parallel | 66983ns | 66983ns | +367.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 13170ns | base | --- | [12820, 14644] | --- | --- | --- | --- |
| head_tail_dyn | 13026ns | no significant difference | [-1843, +522]ns | [11716, 13896] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 13620ns | no significant difference | [-631, +2415]ns | [11387, 16424] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 10636ns | -2332.7ns (-17.7%) | [-4908, -1880]ns | [8530, 11527] | YES | 0.0003 | 0.0001 | 0 |
| nway | 41801ns | +28063.8ns (+213.1%) | [+26171, +36349]ns | [39003, 50788] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 12834ns | no significant difference | [-6035, +8388]ns | [6936, 21841] | no | 1.0000 | 0.8145 | 0 |
| std_parallel | 53109ns | +40689.4ns (+308.9%) | [+27815, +45942]ns | [40499, 60249] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 14081ns | -18.7% | -3.3% | -41.8% | +178.5% | -26.8% | +375.2% |
| 1 | 2 | 14183ns | -6.3% | +24.7% | -19.2% | +293.2% | -33.1% | +170.0% |
| 1 | 3 | 11708ns | +10.1% | -6.4% | -2.4% | +226.7% | +63.2% | +358.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.188 | ok |
| head_tail_dyn | -0.299 | moderate- |
| kernel_nway | -0.564 | HIGH- (thermal bounce) |
| kernel_seq | -0.172 | ok |
| nway | -0.665 | HIGH- (thermal bounce) |
| sequential | -0.213 | moderate- |
| std_parallel | -0.523 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 13805.7ns | 13542.4ns | 101.9% | HIGH |
| head_tail_dyn | 13625.3ns | 12916.8ns | 105.5% | HIGH |
| kernel_nway | 14619.5ns | 14283.8ns | 102.4% | HIGH |
| kernel_seq | 10693.0ns | 10262.8ns | 104.2% | HIGH |
| nway | 47571.3ns | 46828.8ns | 101.6% | HIGH |
| sequential | 15323.3ns | 14754.5ns | 103.9% | HIGH |
| std_parallel | 65616.4ns | 66439.8ns | 98.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 11145.0-15372.9 ns)
  11145.0 |
  11356.4 |#############
  11567.8 |##########################
  11779.2 |
  11990.6 |
  12202.0 |
  12413.4 |
  12624.7 |#############
  12836.1 |##########################
  13047.5 |########################################
  13258.9 |
  13470.3 |
  13681.7 |
  13893.1 |#############
  14104.5 |
  14315.9 |##########################
  14527.3 |
  14738.7 |##########################
  14950.1 |##########################
  15161.5 |
  (1 below, 1 above range)

head_tail_dyn (n=18, range 11249.0-14553.5 ns)
  11249.0 |
  11414.2 |##########################
  11579.5 |#############
  11744.7 |#############
  11909.9 |##########################
  12075.1 |
  12240.4 |
  12405.6 |
  12570.8 |
  12736.0 |
  12901.3 |########################################
  13066.5 |
  13231.7 |
  13396.9 |#############
  13562.2 |
  13727.4 |#############
  13892.6 |#############
  14057.9 |
  14223.1 |
  14388.3 |#############
  (2 below, 3 above range)

kernel_nway (n=18, range 10775.2-18094.0 ns)
  10775.2 |########################################
  11141.1 |##########################
  11507.0 |
  11873.0 |
  12238.9 |
  12604.9 |##########################
  12970.8 |
  13336.7 |##########################
  13702.7 |
  14068.6 |#############
  14434.6 |
  14800.5 |
  15166.4 |#############
  15532.4 |
  15898.3 |#############
  16264.3 |
  16630.2 |
  16996.1 |#############
  17362.1 |#############
  17728.0 |#############
  (1 below, 2 above range)

kernel_seq (n=18, range 7830.3-12110.1 ns)
   7830.3 |####################
   8044.3 |####################
   8258.3 |####################
   8472.3 |########################################
   8686.3 |
   8900.3 |
   9114.2 |
   9328.2 |
   9542.2 |
   9756.2 |########################################
   9970.2 |
  10184.2 |
  10398.2 |
  10612.1 |########################################
  10826.1 |####################
  11040.1 |
  11254.1 |####################
  11468.1 |########################################
  11682.1 |
  11896.1 |####################
  (1 below, 3 above range)

nway (n=18, range 36841.2-60268.4 ns)
  36841.2 |##########
  38012.5 |########################################
  39183.9 |##########
  40355.3 |
  41526.6 |####################
  42698.0 |
  43869.4 |##########
  45040.7 |
  46212.1 |
  47383.4 |##########
  48554.8 |##########
  49726.2 |
  50897.5 |
  52068.9 |##########
  53240.3 |##########
  54411.6 |
  55583.0 |
  56754.4 |##########
  57925.7 |
  59097.1 |
  (2 below, 2 above range)

sequential (n=18, range 6785.1-24433.5 ns)
   6785.1 |########################################
   7667.5 |
   8549.9 |
   9432.3 |
  10314.8 |
  11197.2 |#####
  12079.6 |
  12962.0 |#####
  13844.4 |
  14726.9 |
  15609.3 |
  16491.7 |
  17374.1 |#####
  18256.5 |
  19139.0 |
  20021.4 |#####
  20903.8 |#####
  21786.2 |
  22668.7 |###########
  23551.1 |
  (1 below, 3 above range)

std_parallel (n=18, range 37230.5-108601.1 ns)
  37230.5 |########################################
  40799.0 |
  44367.6 |################
  47936.1 |########
  51504.6 |########
  55073.2 |################
  58641.7 |################
  62210.2 |########
  65778.7 |########
  69347.3 |
  72915.8 |
  76484.3 |
  80052.9 |
  83621.4 |
  87189.9 |
  90758.5 |
  94327.0 |
  97895.5 |
  101464.1 |
  105032.6 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=106.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=104.0% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=21.6% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=102.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=104.3% of algo (FFI overhead may distort results)
- **nway**: CV=24.7% (high variance, measurements may be unstable)
- **nway**: bridge=103.7% of algo (FFI overhead may distort results)
- **sequential**: CV=53.8% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **sequential**: bridge=107.0% of algo (FFI overhead may distort results)
- **std_parallel**: CV=75.5% (high variance, measurements may be unstable)
- **std_parallel**: bridge=103.5% of algo (FFI overhead may distort results)
