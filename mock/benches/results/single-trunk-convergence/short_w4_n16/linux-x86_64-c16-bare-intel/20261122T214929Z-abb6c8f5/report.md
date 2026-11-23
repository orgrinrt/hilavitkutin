# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail dominates: 12% faster than the next best (sequential)

head_tail (10.86 us) leads sequential (12.16 us) by 12%, a clear separation rather than a photo finish. CV 22.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 4.4x slower than the field

std_parallel (48.04 us) is 4.4x the fastest (10.86 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.67)

std_parallel's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (10.86 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, sequential, kernel_seq, head_tail_dyn, kernel_nway} vs {nway, std_parallel} (180% apart)

The field splits into a fast tier {head_tail, sequential, kernel_seq, head_tail_dyn, kernel_nway} and a slow tier {nway, std_parallel} with a 180% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.4x the fastest

Fastest head_tail (10.86 us) to slowest std_parallel (48.04 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.6x its best-20%

sequential's best 20% of batches run at 6.68 us but its worst 20% at 24.07 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 10864.7 ns median
- 5 variants significantly slower than baseline
- Spread: 4.42x (fastest 10864.7 ns, slowest 48036.1 ns)

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
| head_tail | 12695ns | 11374ns | 10104ns | 11405ns | 15926ns | base |
| head_tail_dyn | 15166ns | 13182ns | 11968ns | 13452ns | 19336ns | +19.46% |
| kernel_nway | 14198ns | 13965ns | 12763ns | 13879ns | 15393ns | +11.84% |
| kernel_seq | 13769ns | 13217ns | 10219ns | 12543ns | 17383ns | +8.46% |
| nway | 39874ns | 37965ns | 34506ns | 37693ns | 45830ns | +214.09% |
| sequential | 14844ns | 12402ns | 6813ns | 11037ns | 24570ns | +16.93% |
| std_parallel | 56954ns | 48296ns | 36900ns | 45515ns | 84139ns | +348.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 12043ns | 9623ns | 15061ns | base | 0.001 |
| head_tail_dyn | 14284ns | 11340ns | 18132ns | +18.61% | 0.001 |
| kernel_nway | 13473ns | 12127ns | 14572ns | +11.87% | 0.001 |
| kernel_seq | 12969ns | 9692ns | 16284ns | +7.69% | 0.001 |
| nway | 39345ns | 34074ns | 45190ns | +226.70% | 0.000 |
| sequential | 14545ns | 6677ns | 24069ns | +20.77% | 0.001 |
| std_parallel | 56522ns | 36705ns | 83312ns | +369.32% | 0.000 |

## Performance model

- Peak throughput: **0.002 Gops/s** (sequential; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.001 | 61.5% |
| head_tail_dyn | 0.001 | 53.0% |
| kernel_nway | 0.001 | 49.9% |
| kernel_seq | 0.001 | 54.3% |
| nway | 0.000 | 17.8% |
| sequential | 0.001 | 54.9% |
| std_parallel | 0.000 | 13.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 12695ns | 12695ns | base |
| head_tail_dyn | 15166ns | 15166ns | +19.46% |
| kernel_nway | 14198ns | 14198ns | +11.84% |
| kernel_seq | 13769ns | 13769ns | +8.46% |
| nway | 39874ns | 39874ns | +214.09% |
| sequential | 14844ns | 14844ns | +16.93% |
| std_parallel | 56954ns | 56954ns | +348.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 10865ns | base | --- | [10171, 13750] | --- | --- | --- | --- |
| head_tail_dyn | 12596ns | +2409.7ns (+22.2%) | [+1463, +2931]ns | [12096, 16480] | YES | 0.0003 | 0.0001 | 0 |
| kernel_nway | 13373ns | +1945.1ns (+17.9%) | [+433, +2851]ns | [12880, 14102] | YES (adj: no) | 0.1155 | 0.0963 | 0 |
| kernel_seq | 12306ns | +728.0ns (+6.7%) | [+109, +1558]ns | [10762, 14796] | YES (adj: no) | 0.1155 | 0.0963 | 0 |
| nway | 37482ns | +26572.7ns (+244.6%) | [+22772, +29944]ns | [36340, 41793] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 12157ns | no significant difference | [-3781, +8576]ns | [6825, 21792] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 48036ns | +35069.2ns (+322.8%) | [+26869, +60467]ns | [39754, 73276] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 14576ns | +23.3% | -0.4% | +8.3% | +149.3% | -3.4% | +218.4% |
| 1 | 2 | 10865ns | +8.1% | +24.1% | +9.7% | +298.5% | +2.3% | +378.2% |
| 1 | 3 | 9798ns | +27.1% | +28.2% | +7.4% | +278.3% | +35.1% | +372.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.062 | ok |
| head_tail_dyn | -0.230 | moderate- |
| kernel_nway | -0.001 | ok |
| kernel_seq | -0.046 | ok |
| nway | -0.661 | HIGH- (thermal bounce) |
| sequential | -0.615 | HIGH- (thermal bounce) |
| std_parallel | -0.666 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 12383.9ns | 12043.2ns | 102.8% | HIGH |
| head_tail_dyn | 15293.7ns | 14284.1ns | 107.1% | HIGH |
| kernel_nway | 14057.9ns | 13472.9ns | 104.3% | HIGH |
| kernel_seq | 13301.5ns | 12969.0ns | 102.6% | HIGH |
| nway | 40171.2ns | 39345.2ns | 102.1% | HIGH |
| sequential | 14919.2ns | 14544.8ns | 102.6% | HIGH |
| std_parallel | 55878.5ns | 56521.6ns | 98.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 9622.7-15060.8 ns)
   9622.7 |########################################
   9894.6 |
  10166.5 |
  10438.4 |########################################
  10710.3 |
  10982.2 |####################
  11254.1 |
  11526.0 |
  11797.9 |
  12069.8 |
  12341.7 |
  12613.6 |
  12885.5 |
  13157.4 |##########
  13429.3 |##########
  13701.2 |##########
  13973.1 |##########
  14245.0 |
  14516.9 |
  14788.8 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 11339.6-18131.7 ns)
  11339.6 |##########################
  11679.2 |##########################
  12018.8 |##########################
  12358.4 |########################################
  12698.1 |#############
  13037.7 |
  13377.3 |
  13716.9 |
  14056.5 |
  14396.1 |
  14735.7 |
  15075.3 |#############
  15414.9 |
  15754.5 |
  16094.1 |#############
  16433.7 |#############
  16773.3 |#############
  17112.9 |
  17452.5 |
  17792.1 |
  (1 below, 3 above range)

kernel_nway (n=18, range 12127.2-14572.0 ns)
  12127.2 |#############
  12249.4 |
  12371.7 |##########################
  12493.9 |
  12616.1 |#############
  12738.4 |
  12860.6 |
  12982.9 |
  13105.1 |########################################
  13227.4 |#############
  13349.6 |#############
  13471.8 |
  13594.1 |##########################
  13716.3 |
  13838.6 |
  13960.8 |#############
  14083.1 |
  14205.3 |
  14327.5 |
  14449.8 |##########################
  (1 below, 3 above range)

kernel_seq (n=18, range 9691.7-16284.2 ns)
   9691.7 |
  10021.4 |########################################
  10351.0 |########################################
  10680.6 |####################
  11010.2 |
  11339.8 |####################
  11669.5 |####################
  11999.1 |####################
  12328.7 |####################
  12658.3 |####################
  12988.0 |
  13317.6 |
  13647.2 |########################################
  13976.8 |
  14306.4 |
  14636.1 |
  14965.7 |
  15295.3 |
  15624.9 |########################################
  15954.6 |
  (1 below, 3 above range)

nway (n=18, range 34073.5-45190.0 ns)
  34073.5 |#############
  34629.4 |#############
  35185.2 |
  35741.0 |#############
  36296.8 |########################################
  36852.6 |#############
  37408.5 |#############
  37964.3 |
  38520.1 |
  39075.9 |#############
  39631.8 |#############
  40187.6 |
  40743.4 |
  41299.2 |#############
  41855.0 |
  42410.9 |
  42966.7 |##########################
  43522.5 |#############
  44078.3 |
  44634.2 |
  (2 below, 2 above range)

sequential (n=18, range 6676.8-24069.1 ns)
   6676.8 |########################################
   7546.4 |
   8416.0 |#####
   9285.6 |
  10155.3 |
  11024.9 |
  11894.5 |
  12764.1 |
  13633.7 |
  14503.3 |#####
  15372.9 |
  16242.6 |
  17112.2 |
  17981.8 |
  18851.4 |###########
  19721.0 |
  20590.6 |
  21460.2 |#####
  22329.9 |#####
  23199.5 |#####
  (1 below, 3 above range)

std_parallel (n=18, range 36704.8-83312.1 ns)
  36704.8 |##############################
  39035.2 |########################################
  41365.5 |
  43695.9 |##########
  46026.3 |
  48356.6 |
  50687.0 |####################
  53017.3 |
  55347.7 |
  57678.1 |##########
  60008.4 |
  62338.8 |
  64669.2 |
  66999.5 |
  69329.9 |##########
  71660.2 |
  73990.6 |##########
  76321.0 |
  78651.3 |##########
  80981.7 |##########
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=20.0% (high variance, measurements may be unstable)
- **head_tail**: bridge=103.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=21.0% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=108.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=102.9% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=20.9% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=100.6% of algo (FFI overhead may distort results)
- **nway**: bridge=101.5% of algo (FFI overhead may distort results)
- **sequential**: CV=54.3% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **sequential**: bridge=105.8% of algo (FFI overhead may distort results)
- **std_parallel**: CV=36.7% (high variance, measurements may be unstable)
- **std_parallel**: bridge=98.9% of algo (FFI overhead may distort results)
