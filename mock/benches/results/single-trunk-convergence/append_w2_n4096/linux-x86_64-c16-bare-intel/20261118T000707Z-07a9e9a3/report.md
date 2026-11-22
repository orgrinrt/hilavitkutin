# append, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 30% faster than the next best (kernel_nway)

sequential (11.17 us) leads kernel_nway (14.57 us) by 30%, a clear separation rather than a photo finish. CV 112.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 5.1x slower than the field

std_parallel (57.16 us) is 5.1x the fastest (11.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 112.9%)

sequential wins on median (11.17 us) yet has the highest variance (CV 112.9%), while std_parallel is the steadiest (CV 13.0%, 57.16 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### nway shows alternating (throttle bounce) (autocorr -0.65)

nway's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, kernel_nway, head_tail_dyn, head_tail, kernel_seq} vs {nway, std_parallel} (161% apart)

The field splits into a fast tier {sequential, kernel_nway, head_tail_dyn, head_tail, kernel_seq} and a slow tier {nway, std_parallel} with a 161% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.1x the fastest

Fastest sequential (11.17 us) to slowest std_parallel (57.16 us): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.7x its best-20%

sequential's best 20% of batches run at 9.80 us but its worst 20% at 36.23 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 11171.1 ns median (-36.8% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 5.12x (fastest 11171.1 ns, slowest 57156.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 2 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 18971ns | 18291ns | 15326ns | 17731ns | 22654ns | base |
| head_tail_dyn | 18247ns | 17111ns | 13965ns | 16369ns | 23204ns | -3.82% |
| kernel_nway | 16146ns | 15115ns | 12412ns | 14879ns | 19914ns | -14.89% |
| kernel_seq | 20431ns | 19052ns | 15828ns | 18359ns | 25841ns | +7.70% |
| nway | 48320ns | 48855ns | 38300ns | 45456ns | 57626ns | +154.71% |
| sequential | 20293ns | 11852ns | 9939ns | 12735ns | 36807ns | +6.97% |
| std_parallel | 55614ns | 57846ns | 43617ns | 54529ns | 63241ns | +193.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 17985ns | 14836ns | 20893ns | base | 0.228 |
| head_tail_dyn | 17240ns | 13385ns | 21430ns | -4.15% | 0.238 |
| kernel_nway | 15333ns | 11958ns | 18701ns | -14.75% | 0.267 |
| kernel_seq | 19257ns | 15310ns | 23548ns | +7.07% | 0.213 |
| nway | 47733ns | 37834ns | 56943ns | +165.40% | 0.086 |
| sequential | 19934ns | 9796ns | 36233ns | +10.83% | 0.205 |
| std_parallel | 54762ns | 43153ns | 62125ns | +204.48% | 0.075 |

## Performance model

- Peak throughput: **0.418 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.232 | 55.4% |
| head_tail_dyn | 0.249 | 59.6% |
| kernel_nway | 0.281 | 67.2% |
| kernel_seq | 0.222 | 53.1% |
| nway | 0.085 | 20.3% |
| sequential | 0.367 | 87.7% |
| std_parallel | 0.072 | 17.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 18971ns | 18971ns | base |
| head_tail_dyn | 18247ns | 18247ns | -3.82% |
| kernel_nway | 16146ns | 16146ns | -14.89% |
| kernel_seq | 20431ns | 20431ns | +7.70% |
| nway | 48320ns | 48320ns | +154.71% |
| sequential | 20293ns | 20293ns | +6.97% |
| std_parallel | 55614ns | 55614ns | +193.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 17688ns | base | --- | [16113, 18807] | --- | --- | --- | --- |
| head_tail_dyn | 16423ns | -1089.6ns (-6.2%) | [-2257, -470]ns | [15000, 18350] | YES | 0.0026 | 0.0013 | 0 |
| kernel_nway | 14568ns | -3000.6ns (-17.0%) | [-3305, -2557]ns | [13295, 16746] | YES | 0.0113 | 0.0075 | 0 |
| kernel_seq | 18450ns | no significant difference | [-251, +1256]ns | [16822, 20274] | no | 0.5768 | 0.4807 | 0 |
| nway | 48222ns | +28827.2ns (+163.0%) | [+24083, +31783]ns | [40400, 51097] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 11171ns | no significant difference | [-6733, +12811]ns | [9901, 28910] | no | 0.8145 | 0.8145 | 0 |
| std_parallel | 57156ns | +38478.6ns (+217.5%) | [+29414, +43127]ns | [48138, 61016] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 15559ns | -7.5% | -17.5% | +14.1% | +180.8% | +42.8% | +295.3% |
| 1 | 2 | 18460ns | +6.2% | +0.2% | +12.2% | +176.8% | -39.5% | +216.7% |
| 1 | 3 | 17447ns | -5.8% | -18.6% | -3.6% | +141.6% | -43.3% | +156.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.393 | moderate- |
| head_tail_dyn | -0.565 | HIGH- (thermal bounce) |
| kernel_nway | -0.631 | HIGH- (thermal bounce) |
| kernel_seq | -0.632 | HIGH- (thermal bounce) |
| nway | -0.649 | HIGH- (thermal bounce) |
| sequential | -0.116 | ok |
| std_parallel | -0.080 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 82967.9ns | 17985.4ns | 461.3% | HIGH |
| head_tail_dyn | 82193.9ns | 17239.8ns | 476.8% | HIGH |
| kernel_nway | 75910.2ns | 15332.9ns | 495.1% | HIGH |
| kernel_seq | 87793.7ns | 19257.0ns | 455.9% | HIGH |
| nway | 97855.2ns | 47733.1ns | 205.0% | HIGH |
| sequential | 49711.6ns | 19933.9ns | 249.4% | HIGH |
| std_parallel | 106096.2ns | 54761.6ns | 193.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 14835.7-20893.2 ns)
  14835.7 |
  15138.6 |#############
  15441.5 |
  15744.3 |#############
  16047.2 |##########################
  16350.1 |
  16653.0 |
  16955.8 |##########################
  17258.7 |
  17561.6 |########################################
  17864.5 |
  18167.3 |#############
  18470.2 |#############
  18773.1 |#############
  19076.0 |
  19378.8 |#############
  19681.7 |
  19984.6 |
  20287.5 |
  20590.3 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 13385.4-21429.8 ns)
  13385.4 |#############
  13787.6 |#############
  14189.8 |
  14592.0 |##########################
  14994.3 |##########################
  15396.5 |#############
  15798.7 |
  16200.9 |########################################
  16603.2 |#############
  17005.4 |
  17407.6 |
  17809.8 |
  18212.0 |##########################
  18614.3 |
  19016.5 |#############
  19418.7 |
  19820.9 |
  20223.2 |
  20625.4 |##########################
  21027.6 |
  (1 below, 1 above range)

kernel_nway (n=18, range 11957.5-18701.0 ns)
  11957.5 |
  12294.7 |#############
  12631.9 |
  12969.0 |##########################
  13306.2 |#############
  13643.4 |
  13980.5 |#############
  14317.7 |########################################
  14654.9 |
  14992.1 |#############
  15329.2 |#############
  15666.4 |
  16003.6 |#############
  16340.8 |
  16678.0 |
  17015.1 |#############
  17352.3 |
  17689.5 |
  18026.7 |#############
  18363.8 |
  (2 below, 3 above range)

kernel_seq (n=18, range 15310.1-23547.6 ns)
  15310.1 |
  15722.0 |#############
  16133.9 |
  16545.7 |########################################
  16957.6 |#############
  17369.5 |#############
  17781.4 |
  18193.2 |########################################
  18605.1 |#############
  19017.0 |
  19428.9 |
  19840.7 |#############
  20252.6 |##########################
  20664.5 |
  21076.4 |#############
  21488.2 |
  21900.1 |
  22312.0 |
  22723.9 |
  23135.8 |
  (2 below, 2 above range)

nway (n=18, range 37834.5-56942.7 ns)
  37834.5 |#############
  38789.9 |########################################
  39745.3 |
  40700.7 |#############
  41656.1 |
  42611.5 |#############
  43566.9 |
  44522.4 |
  45477.8 |
  46433.2 |#############
  47388.6 |#############
  48344.0 |########################################
  49299.4 |#############
  50254.8 |
  51210.2 |
  52165.6 |#############
  53121.0 |
  54076.4 |
  55031.8 |##########################
  55987.3 |
  (1 below, 2 above range)

sequential (n=18, range 9796.2-36232.8 ns)
   9796.2 |########################################
  11118.1 |#####
  12439.9 |
  13761.7 |
  15083.6 |#####
  16405.4 |
  17727.2 |
  19049.0 |
  20370.9 |
  21692.7 |
  23014.5 |
  24336.4 |#####
  25658.2 |
  26980.0 |
  28301.9 |#####
  29623.7 |
  30945.5 |#####
  32267.3 |#####
  33589.2 |#####
  34911.0 |
  (2 below, 2 above range)

std_parallel (n=18, range 43153.1-62124.8 ns)
  43153.1 |#############
  44101.7 |##########################
  45050.3 |
  45998.9 |
  46947.4 |#############
  47896.0 |#############
  48844.6 |
  49793.2 |
  50741.8 |
  51690.4 |#############
  52638.9 |
  53587.5 |
  54536.1 |
  55484.7 |
  56433.3 |##########################
  57381.9 |#############
  58330.5 |
  59279.0 |##########################
  60227.6 |#############
  61176.2 |########################################
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=474.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=23.1% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=494.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=520.8% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=22.4% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=436.6% of algo (FFI overhead may distort results)
- **nway**: bridge=205.0% of algo (FFI overhead may distort results)
- **sequential**: CV=63.3% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **sequential**: bridge=247.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=182.0% of algo (FFI overhead may distort results)
