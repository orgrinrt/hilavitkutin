# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 184% faster than the next best (head_tail_dyn)

sequential (7.61 us) leads head_tail_dyn (21.57 us) by 184%, a clear separation rather than a photo finish. CV 9.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 66% (significant)

sequential is -16.13 us (66%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 5.2x slower than the field

std_parallel (39.76 us) is 5.2x the fastest (7.61 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.66)

std_parallel's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {head_tail_dyn, kernel_seq, head_tail, kernel_nway, nway, std_parallel} (184% apart)

The field splits into a fast tier {sequential} and a slow tier {head_tail_dyn, kernel_seq, head_tail, kernel_nway, nway, std_parallel} with a 184% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.2x the fastest

Fastest sequential (7.61 us) to slowest std_parallel (39.76 us): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail_dyn is inconsistent: worst-20% is 1.6x its best-20%

head_tail_dyn's best 20% of batches run at 19.24 us but its worst 20% at 31.24 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7608.9 ns median (-69.0% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 5.23x (fastest 7608.9 ns, slowest 39760.4 ns)

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
| Counter frequency | 2100350026 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 24770ns | 24964ns | 19461ns | 23103ns | 29924ns | base |
| head_tail_dyn | 24829ns | 21969ns | 19603ns | 21974ns | 31723ns | +0.24% |
| kernel_nway | 25833ns | 25379ns | 20435ns | 24526ns | 30492ns | +4.29% |
| kernel_seq | 23496ns | 24046ns | 19715ns | 23343ns | 25615ns | -5.14% |
| nway | 35293ns | 35536ns | 30525ns | 34340ns | 39107ns | +42.49% |
| sequential | 8083ns | 7845ns | 7327ns | 7714ns | 9015ns | -67.37% |
| std_parallel | 41100ns | 40203ns | 35810ns | 39405ns | 46287ns | +65.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 24274ns | 19123ns | 29289ns | base | 0.042 |
| head_tail_dyn | 24376ns | 19238ns | 31241ns | +0.42% | 0.042 |
| kernel_nway | 25022ns | 19966ns | 28964ns | +3.08% | 0.041 |
| kernel_seq | 23044ns | 19356ns | 25167ns | -5.06% | 0.044 |
| nway | 34918ns | 30159ns | 38704ns | +43.85% | 0.029 |
| sequential | 7836ns | 7097ns | 8743ns | -67.72% | 0.131 |
| std_parallel | 40601ns | 35253ns | 45859ns | +67.26% | 0.025 |

## Performance model

- Peak throughput: **0.144 Gops/s** (sequential; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.042 | 28.9% |
| head_tail_dyn | 0.047 | 32.9% |
| kernel_nway | 0.041 | 28.5% |
| kernel_seq | 0.043 | 30.1% |
| nway | 0.029 | 20.2% |
| sequential | 0.135 | 93.3% |
| std_parallel | 0.026 | 17.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 24770ns | 24770ns | base |
| head_tail_dyn | 24829ns | 24829ns | +0.24% |
| kernel_nway | 25833ns | 25833ns | +4.29% |
| kernel_seq | 23496ns | 23496ns | -5.14% |
| nway | 35293ns | 35293ns | +42.49% |
| sequential | 8083ns | 8083ns | -67.37% |
| std_parallel | 41100ns | 41100ns | +65.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 24572ns | base | --- | [19639, 26813] | --- | --- | --- | --- |
| head_tail_dyn | 21574ns | no significant difference | [-4785, +990]ns | [20262, 25634] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 24916ns | no significant difference | [-902, +3212]ns | [22257, 28284] | no | 0.3568 | 0.2379 | 0 |
| kernel_seq | 23607ns | no significant difference | [-3128, +3108]ns | [22320, 24421] | no | 0.5768 | 0.4807 | 0 |
| nway | 35162ns | +9783.2ns (+39.8%) | [+7221, +16851]ns | [31908, 36767] | YES | 0.0026 | 0.0013 | 0 |
| sequential | 7609ns | -16128.8ns (-65.6%) | [-19024, -12336]ns | [7135, 8159] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 39760ns | +15085.8ns (+61.4%) | [+13398, +19612]ns | [38026, 40434] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 19198ns | +2.2% | +29.6% | +20.4% | +91.6% | -62.2% | +104.4% |
| 1 | 2 | 26924ns | -19.9% | +6.3% | -7.2% | +20.9% | -73.0% | +49.1% |
| 1 | 3 | 24572ns | +19.8% | -15.2% | -16.9% | +34.1% | -64.5% | +60.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.360 | moderate- |
| head_tail_dyn | -0.072 | ok |
| kernel_nway | -0.486 | moderate- |
| kernel_seq | -0.437 | moderate- |
| nway | -0.217 | moderate- |
| sequential | -0.169 | ok |
| std_parallel | -0.663 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 24228.6ns | 24273.7ns | 99.8% | HIGH |
| head_tail_dyn | 25658.9ns | 24376.3ns | 105.3% | HIGH |
| kernel_nway | 26432.3ns | 25022.1ns | 105.6% | HIGH |
| kernel_seq | 23092.8ns | 23044.3ns | 100.2% | HIGH |
| nway | 35470.0ns | 34918.2ns | 101.6% | HIGH |
| sequential | 8223.5ns | 7835.9ns | 104.9% | HIGH |
| std_parallel | 41073.6ns | 40600.6ns | 101.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 19123.1-29289.0 ns)
  19123.1 |########################################
  19631.4 |#############
  20139.7 |
  20648.0 |
  21156.3 |
  21664.6 |#############
  22172.9 |#############
  22681.2 |
  23189.5 |#############
  23697.8 |
  24206.1 |
  24714.4 |
  25222.7 |#############
  25731.0 |##########################
  26239.3 |#############
  26747.6 |#############
  27255.9 |#############
  27764.1 |#############
  28272.4 |
  28780.7 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 19237.7-31240.6 ns)
  19237.7 |########################################
  19837.8 |##########################
  20438.0 |#############
  21038.1 |##########################
  21638.3 |#############
  22238.4 |
  22838.6 |#############
  23438.7 |
  24038.8 |
  24639.0 |#############
  25239.1 |##########################
  25839.3 |
  26439.4 |
  27039.6 |
  27639.7 |
  28239.9 |#############
  28840.0 |
  29440.2 |
  30040.3 |##########################
  30640.5 |
  (1 below, 1 above range)

kernel_nway (n=18, range 19966.1-28963.7 ns)
  19966.1 |
  20415.9 |#############
  20865.8 |#############
  21315.7 |
  21765.6 |#############
  22215.5 |#############
  22665.4 |
  23115.2 |#############
  23565.1 |
  24015.0 |##########################
  24464.9 |
  24914.8 |
  25364.6 |#############
  25814.5 |
  26264.4 |#############
  26714.3 |
  27164.2 |#############
  27614.1 |
  28063.9 |########################################
  28513.8 |##########################
  (2 below, 1 above range)

kernel_seq (n=18, range 19356.2-25166.6 ns)
  19356.2 |
  19646.7 |
  19937.2 |#############
  20227.8 |
  20518.3 |#############
  20808.8 |
  21099.3 |
  21389.8 |
  21680.3 |
  21970.9 |#############
  22261.4 |#############
  22551.9 |##########################
  22842.4 |
  23132.9 |
  23423.5 |########################################
  23714.0 |
  24004.5 |#############
  24295.0 |##########################
  24585.5 |#############
  24876.0 |#############
  (2 below, 2 above range)

nway (n=18, range 30158.7-38704.2 ns)
  30158.7 |##########################
  30585.9 |
  31013.2 |
  31440.5 |##########################
  31867.8 |#############
  32295.0 |
  32722.3 |
  33149.6 |
  33576.9 |#############
  34004.1 |#############
  34431.4 |#############
  34858.7 |
  35286.0 |#############
  35713.2 |#############
  36140.5 |#############
  36567.8 |########################################
  36995.1 |#############
  37422.3 |
  37849.6 |
  38276.9 |
  (1 below, 2 above range)

sequential (n=18, range 7097.3-8742.6 ns)
   7097.3 |########################################
   7179.6 |
   7261.8 |
   7344.1 |################
   7426.4 |
   7508.6 |########
   7590.9 |########
   7673.2 |
   7755.4 |
   7837.7 |########
   7919.9 |
   8002.2 |########
   8084.5 |########
   8166.7 |########
   8249.0 |
   8331.3 |
   8413.5 |
   8495.8 |
   8578.1 |########
   8660.3 |
  (1 below, 3 above range)

std_parallel (n=18, range 35253.0-45858.9 ns)
  35253.0 |
  35783.3 |#############
  36313.6 |#############
  36843.9 |
  37374.2 |#############
  37904.5 |
  38434.8 |########################################
  38965.0 |
  39495.3 |########################################
  40025.6 |########################################
  40555.9 |#############
  41086.2 |
  41616.5 |
  42146.8 |
  42677.1 |
  43207.4 |
  43737.7 |#############
  44268.0 |
  44798.3 |
  45328.6 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=96.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=26.9% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=111.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=109.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=98.8% of algo (FFI overhead may distort results)
- **nway**: bridge=101.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=106.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=98.3% of algo (FFI overhead may distort results)
