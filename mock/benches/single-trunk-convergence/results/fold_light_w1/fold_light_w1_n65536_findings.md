# fold_light, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 22% faster than the next best (head_tail)

sequential (22.70 us) leads head_tail (27.61 us) by 22%, a clear separation rather than a photo finish. CV 19.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 23% (significant)

sequential is -6.27 us (23%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 2.4x slower than the field

std_parallel (53.54 us) is 2.4x the fastest (22.70 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.66)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, head_tail, kernel_nway, head_tail_dyn, kernel_seq} vs {nway, std_parallel} (46% apart)

The field splits into a fast tier {sequential, head_tail, kernel_nway, head_tail_dyn, kernel_seq} and a slow tier {nway, std_parallel} with a 46% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### head_tail is inconsistent: worst-20% is 9.8x its best-20%

head_tail's best 20% of batches run at 11.03 us but its worst 20% at 108.48 us (9.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 22698.7 ns median (-17.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.36x (fastest 22698.7 ns, slowest 53544.8 ns)

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
| Counter frequency | 2100350026 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 51767ns | 28100ns | 11320ns | 27097ns | 108996ns | base |
| head_tail_dyn | 35400ns | 33081ns | 29536ns | 33146ns | 41713ns | -31.62% |
| kernel_nway | 28115ns | 29570ns | 11944ns | 24622ns | 41440ns | -45.69% |
| kernel_seq | 42460ns | 34401ns | 24550ns | 33603ns | 64701ns | -17.98% |
| nway | 49117ns | 50083ns | 42864ns | 47401ns | 54819ns | -5.12% |
| sequential | 21870ns | 23134ns | 15754ns | 20931ns | 26337ns | -57.75% |
| std_parallel | 56463ns | 54258ns | 45382ns | 53414ns | 66578ns | +9.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 51304ns | 11027ns | 108481ns | base | 1.277 |
| head_tail_dyn | 34907ns | 29173ns | 41269ns | -31.96% | 1.877 |
| kernel_nway | 27687ns | 11669ns | 41000ns | -46.03% | 2.367 |
| kernel_seq | 41727ns | 23779ns | 63586ns | -18.67% | 1.571 |
| nway | 48652ns | 42465ns | 54316ns | -5.17% | 1.347 |
| sequential | 21517ns | 15471ns | 25953ns | -58.06% | 3.046 |
| std_parallel | 55950ns | 44786ns | 66117ns | +9.06% | 1.171 |

## Performance model

- Peak throughput: **5.943 Gops/s** (head_tail; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.374 | 39.9% |
| head_tail_dyn | 2.006 | 33.7% |
| kernel_nway | 2.270 | 38.2% |
| kernel_seq | 1.927 | 32.4% |
| nway | 1.321 | 22.2% |
| sequential | 2.887 | 48.6% |
| std_parallel | 1.224 | 20.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 51767ns | 51767ns | base |
| head_tail_dyn | 35400ns | 35400ns | -31.62% |
| kernel_nway | 28115ns | 28115ns | -45.69% |
| kernel_seq | 42460ns | 42460ns | -17.98% |
| nway | 49117ns | 49117ns | -5.12% |
| sequential | 21870ns | 21870ns | -57.75% |
| std_parallel | 56463ns | 56463ns | +9.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 27607ns | base | --- | [24878, 63179] | --- | --- | --- | --- |
| head_tail_dyn | 32675ns | no significant difference | [-33371, +14193]ns | [30289, 40437] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 28869ns | no significant difference | [-42348, +9343]ns | [13255, 37162] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 34003ns | no significant difference | [-31031, +13081]ns | [31582, 38095] | no | 0.1444 | 0.0963 | 0 |
| nway | 49617ns | no significant difference | [-15528, +29123]ns | [43926, 52084] | no | 0.1444 | 0.0963 | 0 |
| sequential | 22699ns | -6267.5ns (-22.7%) | [-40535, -3007]ns | [16949, 23845] | YES (adj: no) | 0.1444 | 0.0309 | 0 |
| std_parallel | 53545ns | no significant difference | [-6744, +39935]ns | [51411, 58530] | no | 0.1444 | 0.0963 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 18005ns | +66.7% | -33.8% | +82.0% | +188.0% | -10.8% | +190.8% |
| 1 | 2 | 27607ns | +47.9% | +52.2% | +34.1% | +55.5% | -17.8% | +120.9% |
| 1 | 3 | 121299ns | -74.5% | -76.2% | -73.6% | -58.0% | -80.3% | -58.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.121 | ok |
| head_tail_dyn | -0.662 | HIGH- (thermal bounce) |
| kernel_nway | -0.456 | moderate- |
| kernel_seq | -0.652 | HIGH- (thermal bounce) |
| nway | -0.660 | HIGH- (thermal bounce) |
| sequential | -0.095 | ok |
| std_parallel | -0.647 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 55281.1ns | 51304.5ns | 107.8% | HIGH |
| head_tail_dyn | 34657.3ns | 34907.1ns | 99.3% | HIGH |
| kernel_nway | 28644.4ns | 27687.3ns | 103.5% | HIGH |
| kernel_seq | 34947.5ns | 41727.3ns | 83.8% | HIGH |
| nway | 91078.7ns | 48652.0ns | 187.2% | HIGH |
| sequential | 21646.8ns | 21516.9ns | 100.6% | HIGH |
| std_parallel | 54488.1ns | 55950.4ns | 97.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 11026.6-108481.3 ns)
  11026.6 |#####
  15899.3 |
  20772.0 |##########
  25644.8 |########################################
  30517.5 |
  35390.3 |
  40263.0 |
  45135.7 |
  50008.5 |
  54881.2 |
  59753.9 |
  64626.7 |
  69499.4 |
  74372.2 |
  79244.9 |
  84117.6 |
  88990.4 |
  93863.1 |#####
  98735.9 |
  103608.6 |
  (2 below, 4 above range)

head_tail_dyn (n=18, range 29173.0-41268.8 ns)
  29173.0 |##########################
  29777.8 |##########################
  30382.6 |#############
  30987.3 |
  31592.1 |#############
  32196.9 |#############
  32801.7 |#############
  33406.5 |
  34011.3 |
  34616.1 |
  35220.9 |
  35825.7 |#############
  36430.5 |
  37035.3 |
  37640.1 |
  38244.9 |
  38849.7 |
  39454.5 |#############
  40059.2 |#############
  40664.0 |########################################
  (2 below, 2 above range)

kernel_nway (n=18, range 11669.5-41000.1 ns)
  11669.5 |##############################
  13136.0 |##########
  14602.5 |
  16069.1 |
  17535.6 |
  19002.1 |
  20468.6 |
  21935.2 |
  23401.7 |
  24868.2 |
  26334.8 |
  27801.3 |########################################
  29267.8 |
  30734.4 |##########
  32200.9 |##########
  33667.4 |
  35133.9 |##########
  36600.5 |
  38067.0 |##########
  39533.5 |
  (2 below, 4 above range)

kernel_seq (n=18, range 23778.8-63586.3 ns)
  23778.8 |#############
  25769.1 |#############
  27759.5 |
  29749.9 |########################################
  31740.3 |########################################
  33730.7 |#############
  35721.0 |########################################
  37711.4 |##########################
  39701.8 |
  41692.2 |
  43682.6 |
  45672.9 |
  47663.3 |
  49653.7 |
  51644.1 |
  53634.4 |
  55624.8 |
  57615.2 |
  59605.6 |
  61596.0 |
  (1 below, 3 above range)

nway (n=18, range 42465.4-54315.9 ns)
  42465.4 |########################################
  43057.9 |#############
  43650.4 |
  44242.9 |##########################
  44835.5 |
  45428.0 |#############
  46020.5 |
  46613.0 |
  47205.6 |
  47798.1 |
  48390.6 |
  48983.2 |#############
  49575.7 |##########################
  50168.2 |
  50760.7 |
  51353.3 |#############
  51945.8 |########################################
  52538.3 |
  53130.8 |
  53723.4 |
  (1 below, 3 above range)

sequential (n=18, range 15470.7-25952.6 ns)
  15470.7 |##########################
  15994.8 |##########################
  16518.9 |
  17043.0 |
  17567.1 |#############
  18091.2 |
  18615.3 |
  19139.4 |
  19663.4 |
  20187.5 |
  20711.6 |
  21235.7 |##########################
  21759.8 |
  22283.9 |##########################
  22808.0 |
  23332.1 |########################################
  23856.2 |#############
  24380.3 |#############
  24904.4 |
  25428.5 |#############
  (1 below, 2 above range)

std_parallel (n=18, range 44786.0-66117.5 ns)
  44786.0 |#############
  45852.6 |#############
  46919.2 |
  47985.8 |
  49052.3 |#############
  50118.9 |
  51185.5 |########################################
  52252.0 |#############
  53318.6 |##########################
  54385.2 |#############
  55451.8 |
  56518.3 |#############
  57584.9 |#############
  58651.5 |#############
  59718.0 |
  60784.6 |
  61851.2 |
  62917.8 |#############
  63984.3 |
  65050.9 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=90.6% (high variance, measurements may be unstable)
- **head_tail**: worst_20/best_20 = 9.8x (possible bimodal distribution)
- **head_tail**: bridge=99.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=102.1% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=43.3% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **kernel_nway**: bridge=103.7% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=55.5% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=94.4% of algo (FFI overhead may distort results)
- **nway**: bridge=97.0% of algo (FFI overhead may distort results)
- **sequential**: CV=20.4% (high variance, measurements may be unstable)
- **sequential**: bridge=101.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=101.0% of algo (FFI overhead may distort results)
