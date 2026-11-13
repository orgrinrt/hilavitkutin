# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 192% faster than the next best (kernel_seq)

sequential (7.22 us) leads kernel_seq (21.07 us) by 192%, a clear separation rather than a photo finish. CV 9.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 72% (significant)

sequential is -20.12 us (72%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 11.6x slower than the field

std_parallel (84.08 us) is 11.6x the fastest (7.22 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.66)

sequential's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {kernel_seq, head_tail_dyn, head_tail, kernel_nway, nway, std_parallel} (192% apart)

The field splits into a fast tier {sequential} and a slow tier {kernel_seq, head_tail_dyn, head_tail, kernel_nway, nway, std_parallel} with a 192% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 11.6x the fastest

Fastest sequential (7.22 us) to slowest std_parallel (84.08 us): 11.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.6x its best-20%

kernel_nway's best 20% of batches run at 31.75 us but its worst 20% at 49.53 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7224.8 ns median (-74.1% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 11.64x (fastest 7224.8 ns, slowest 84077.3 ns)

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
| head_tail | 29010ns | 28405ns | 25078ns | 27584ns | 33115ns | base |
| head_tail_dyn | 28292ns | 27959ns | 25492ns | 27813ns | 30409ns | -2.48% |
| kernel_nway | 42476ns | 42185ns | 32153ns | 40720ns | 50272ns | +46.42% |
| kernel_seq | 22064ns | 21567ns | 19061ns | 21021ns | 25130ns | -23.94% |
| nway | 49770ns | 49815ns | 46433ns | 49210ns | 52279ns | +71.56% |
| sequential | 7850ns | 7469ns | 7249ns | 7469ns | 8722ns | -72.94% |
| std_parallel | 84752ns | 84740ns | 78315ns | 82650ns | 91124ns | +192.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 28573ns | 24713ns | 32658ns | base | 0.009 |
| head_tail_dyn | 27870ns | 25038ns | 29983ns | -2.46% | 0.009 |
| kernel_nway | 41920ns | 31750ns | 49528ns | +46.71% | 0.006 |
| kernel_seq | 21651ns | 18709ns | 24729ns | -24.22% | 0.012 |
| nway | 49371ns | 45962ns | 51870ns | +72.79% | 0.005 |
| sequential | 7611ns | 7026ns | 8464ns | -73.36% | 0.034 |
| std_parallel | 84266ns | 77823ns | 90572ns | +194.92% | 0.003 |

## Performance model

- Peak throughput: **0.036 Gops/s** (sequential; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.009 | 25.2% |
| head_tail_dyn | 0.009 | 25.5% |
| kernel_nway | 0.006 | 16.8% |
| kernel_seq | 0.012 | 33.3% |
| nway | 0.005 | 14.2% |
| sequential | 0.035 | 97.3% |
| std_parallel | 0.003 | 8.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 29010ns | 29010ns | base |
| head_tail_dyn | 28292ns | 28292ns | -2.48% |
| kernel_nway | 42476ns | 42476ns | +46.42% |
| kernel_seq | 22064ns | 22064ns | -23.94% |
| nway | 49770ns | 49770ns | +71.56% |
| sequential | 7850ns | 7850ns | -72.94% |
| std_parallel | 84752ns | 84752ns | +192.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 27855ns | base | --- | [25840, 29075] | --- | --- | --- | --- |
| head_tail_dyn | 27600ns | no significant difference | [-2853, +1442]ns | [27140, 28857] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 41723ns | +12612.6ns (+45.3%) | [+9147, +20467]ns | [37138, 47247] | YES | 0.0002 | 0.0001 | 0 |
| kernel_seq | 21074ns | -5311.4ns (-19.1%) | [-7651, -4028]ns | [19597, 22899] | YES | 0.0000 | 0.0000 | 0 |
| nway | 49434ns | +21405.3ns (+76.8%) | [+19308, +23660]ns | [48114, 50544] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 7225ns | -20124.7ns (-72.2%) | [-21707, -18728]ns | [7083, 7718] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 84077ns | +57097.8ns (+205.0%) | [+51931, +59831]ns | [79244, 87587] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 25840ns | +6.8% | +50.4% | -22.8% | +95.9% | -72.6% | +212.6% |
| 1 | 2 | 28794ns | -4.3% | +29.0% | -31.9% | +72.3% | -70.2% | +209.5% |
| 1 | 3 | 27984ns | -4.1% | +76.3% | -14.8% | +74.0% | -74.2% | +183.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.338 | moderate- |
| head_tail_dyn | -0.145 | ok |
| kernel_nway | -0.247 | moderate- |
| kernel_seq | -0.209 | moderate- |
| nway | -0.000 | ok |
| sequential | -0.662 | HIGH- (thermal bounce) |
| std_parallel | -0.653 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 29581.7ns | 28572.7ns | 103.5% | HIGH |
| head_tail_dyn | 28039.0ns | 27869.8ns | 100.6% | HIGH |
| kernel_nway | 41758.3ns | 41919.9ns | 99.6% | HIGH |
| kernel_seq | 21919.6ns | 21651.3ns | 101.2% | HIGH |
| nway | 49949.3ns | 49371.4ns | 101.2% | HIGH |
| sequential | 7896.6ns | 7611.2ns | 103.7% | HIGH |
| std_parallel | 86181.1ns | 84266.4ns | 102.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 24712.8-32657.6 ns)
  24712.8 |#############
  25110.1 |#############
  25507.3 |#############
  25904.6 |#############
  26301.8 |
  26699.0 |#############
  27096.3 |##########################
  27493.5 |
  27890.8 |
  28288.0 |#############
  28685.2 |########################################
  29082.5 |#############
  29479.7 |
  29877.0 |
  30274.2 |#############
  30671.4 |
  31068.7 |
  31465.9 |#############
  31863.2 |
  32260.4 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 25038.3-29982.8 ns)
  25038.3 |##########################
  25285.5 |
  25532.7 |
  25780.0 |
  26027.2 |
  26274.4 |#############
  26521.6 |
  26768.9 |
  27016.1 |########################################
  27263.3 |#############
  27510.5 |##########################
  27757.8 |#############
  28005.0 |
  28252.2 |#############
  28499.4 |
  28746.7 |##########################
  28993.9 |
  29241.1 |#############
  29488.3 |
  29735.6 |
  (1 below, 3 above range)

kernel_nway (n=18, range 31749.9-49527.7 ns)
  31749.9 |
  32638.8 |
  33527.7 |####################
  34416.6 |
  35305.5 |####################
  36194.4 |########################################
  37083.2 |########################################
  37972.1 |####################
  38861.0 |
  39749.9 |####################
  40638.8 |
  41527.7 |
  42416.6 |
  43305.4 |####################
  44194.3 |
  45083.2 |####################
  45972.1 |####################
  46861.0 |########################################
  47749.9 |
  48638.8 |####################
  (1 below, 3 above range)

kernel_seq (n=18, range 18709.0-24729.2 ns)
  18709.0 |########################################
  19010.0 |####################
  19311.0 |####################
  19612.1 |########################################
  19913.1 |####################
  20214.1 |####################
  20515.1 |
  20816.1 |
  21117.1 |
  21418.1 |
  21719.1 |####################
  22020.1 |####################
  22321.1 |########################################
  22622.1 |
  22923.2 |
  23224.2 |########################################
  23525.2 |
  23826.2 |
  24127.2 |####################
  24428.2 |####################
  (1 below, 1 above range)

nway (n=18, range 45962.1-51869.5 ns)
  45962.1 |####################
  46257.5 |
  46552.9 |####################
  46848.2 |
  47143.6 |
  47439.0 |
  47734.3 |####################
  48029.7 |####################
  48325.1 |
  48620.5 |####################
  48915.8 |####################
  49211.2 |########################################
  49506.6 |
  49801.9 |########################################
  50097.3 |####################
  50392.7 |
  50688.0 |
  50983.4 |########################################
  51278.8 |####################
  51574.1 |
  (2 below, 2 above range)

sequential (n=18, range 7026.3-8463.6 ns)
   7026.3 |########################################
   7098.2 |########
   7170.1 |################
   7241.9 |########
   7313.8 |
   7385.6 |
   7457.5 |########
   7529.4 |
   7601.2 |########
   7673.1 |################
   7745.0 |
   7816.8 |
   7888.7 |
   7960.5 |
   8032.4 |
   8104.3 |
   8176.1 |
   8248.0 |
   8319.9 |
   8391.7 |########
  (1 below, 3 above range)

std_parallel (n=18, range 77822.6-90572.3 ns)
  77822.6 |#############
  78460.1 |##########################
  79097.6 |########################################
  79735.1 |
  80372.5 |
  81010.0 |
  81647.5 |#############
  82285.0 |
  82922.5 |
  83560.0 |#############
  84197.4 |#############
  84834.9 |
  85472.4 |#############
  86109.9 |
  86747.4 |##########################
  87384.9 |#############
  88022.4 |
  88659.8 |#############
  89297.3 |#############
  89934.8 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=100.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=95.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=102.4% of algo (FFI overhead may distort results)
- **nway**: bridge=98.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=109.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=102.7% of algo (FFI overhead may distort results)
