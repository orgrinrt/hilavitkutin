# fold_heavy, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 46% faster than the next best (head_tail_dyn)

sequential (23.66 us) leads head_tail_dyn (34.52 us) by 46%, a clear separation rather than a photo finish. CV 7.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 41% (significant)

sequential is -15.58 us (41%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 7.7x slower than the field

std_parallel (182.01 us) is 7.7x the fastest (23.66 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.65)

kernel_seq's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 7.7x the fastest

Fastest sequential (23.66 us) to slowest std_parallel (182.01 us): 7.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### nway is inconsistent: worst-20% is 1.9x its best-20%

nway's best 20% of batches run at 60.76 us but its worst 20% at 112.41 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 23655.8 ns median (-37.7% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 7.69x (fastest 23655.8 ns, slowest 182007.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100242360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 41829ns | 38354ns | 35496ns | 38604ns | 49834ns | base |
| head_tail_dyn | 34961ns | 35336ns | 32788ns | 34730ns | 36394ns | -16.42% |
| kernel_nway | 76557ns | 79081ns | 58926ns | 73653ns | 89727ns | +83.02% |
| kernel_seq | 38641ns | 36512ns | 34159ns | 36098ns | 44695ns | -7.62% |
| nway | 86503ns | 79800ns | 61138ns | 77351ns | 112915ns | +106.80% |
| sequential | 24319ns | 23941ns | 22347ns | 23462ns | 26591ns | -41.86% |
| std_parallel | 180248ns | 182631ns | 154116ns | 171811ns | 205969ns | +330.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 41160ns | 35079ns | 48882ns | base | 0.100 |
| head_tail_dyn | 34429ns | 32377ns | 35936ns | -16.35% | 0.119 |
| kernel_nway | 75885ns | 58476ns | 88895ns | +84.37% | 0.054 |
| kernel_seq | 38163ns | 33797ns | 44199ns | -7.28% | 0.107 |
| nway | 86051ns | 60757ns | 112408ns | +109.07% | 0.048 |
| sequential | 24046ns | 22088ns | 26300ns | -41.58% | 0.170 |
| std_parallel | 179564ns | 153664ns | 205201ns | +336.26% | 0.023 |

## Performance model

- Peak throughput: **0.185 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.108 | 58.2% |
| head_tail_dyn | 0.119 | 64.0% |
| kernel_nway | 0.052 | 28.2% |
| kernel_seq | 0.114 | 61.5% |
| nway | 0.052 | 27.8% |
| sequential | 0.173 | 93.4% |
| std_parallel | 0.023 | 12.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 41829ns | 41829ns | base |
| head_tail_dyn | 34961ns | 34961ns | -16.42% |
| kernel_nway | 76557ns | 76557ns | +83.02% |
| kernel_seq | 38641ns | 38641ns | -7.62% |
| nway | 86503ns | 86503ns | +106.80% |
| sequential | 24319ns | 24319ns | -41.86% |
| std_parallel | 180248ns | 180248ns | +330.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 37956ns | base | --- | [36598, 44834] | --- | --- | --- | --- |
| head_tail_dyn | 34516ns | -4849.3ns (-12.8%) | [-10957, -1275]ns | [33330, 35520] | YES | 0.0016 | 0.0013 | 0 |
| kernel_nway | 78246ns | +35878.9ns (+94.5%) | [+21319, +43877]ns | [66146, 81191] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 35937ns | no significant difference | [-10244, +753]ns | [35139, 38595] | no | 0.4807 | 0.4807 | 0 |
| nway | 79343ns | +37110.3ns (+97.8%) | [+26633, +59250]ns | [70055, 95702] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 23656ns | -15575.2ns (-41.0%) | [-19162, -13304]ns | [22457, 25043] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 182007ns | +141657.9ns (+373.2%) | [+113863, +149265]ns | [161733, 187202] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 37388ns | -7.8% | +79.7% | -4.2% | +170.6% | -36.6% | +412.5% |
| 1 | 2 | 36271ns | -1.8% | +121.7% | +13.4% | +80.3% | -38.5% | +404.3% |
| 1 | 3 | 47285ns | -29.8% | +49.5% | -26.7% | +67.8% | -46.3% | +237.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.222 | moderate- |
| head_tail_dyn | -0.483 | moderate- |
| kernel_nway | -0.624 | HIGH- (thermal bounce) |
| kernel_seq | -0.648 | HIGH- (thermal bounce) |
| nway | -0.423 | moderate- |
| sequential | -0.467 | moderate- |
| std_parallel | -0.043 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 42159.9ns | 41159.9ns | 102.4% | HIGH |
| head_tail_dyn | 35281.0ns | 34429.3ns | 102.5% | HIGH |
| kernel_nway | 74256.6ns | 75884.6ns | 97.9% | HIGH |
| kernel_seq | 36601.8ns | 38162.6ns | 95.9% | HIGH |
| nway | 101075.7ns | 86051.5ns | 117.5% | HIGH |
| sequential | 24483.5ns | 24045.6ns | 101.8% | HIGH |
| std_parallel | 206547.5ns | 179564.0ns | 115.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 35078.7-48882.5 ns)
  35078.7 |##########################
  35768.9 |##########################
  36459.1 |##########################
  37149.3 |
  37839.5 |########################################
  38529.7 |
  39219.8 |
  39910.0 |
  40600.2 |#############
  41290.4 |#############
  41980.6 |
  42670.8 |#############
  43361.0 |
  44051.1 |
  44741.3 |
  45431.5 |
  46121.7 |
  46811.9 |##########################
  47502.1 |#############
  48192.3 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 32376.6-35936.2 ns)
  32376.6 |####################
  32554.6 |
  32732.6 |
  32910.5 |####################
  33088.5 |
  33266.5 |########################################
  33444.5 |
  33622.5 |
  33800.4 |
  33978.4 |
  34156.4 |####################
  34334.4 |########################################
  34512.4 |####################
  34690.4 |########################################
  34868.3 |
  35046.3 |
  35224.3 |
  35402.3 |########################################
  35580.3 |####################
  35758.2 |
  (2 below, 3 above range)

kernel_nway (n=18, range 58476.4-88895.2 ns)
  58476.4 |
  59997.3 |
  61518.3 |####################
  63039.2 |
  64560.1 |####################
  66081.1 |########################################
  67602.0 |####################
  69123.0 |
  70643.9 |
  72164.9 |
  73685.8 |####################
  75206.8 |
  76727.7 |####################
  78248.6 |########################################
  79769.6 |########################################
  81290.5 |########################################
  82811.5 |
  84332.4 |
  85853.4 |
  87374.3 |
  (2 below, 3 above range)

kernel_seq (n=18, range 33796.8-44198.9 ns)
  33796.8 |####################
  34316.9 |##########
  34837.0 |####################
  35357.1 |####################
  35877.2 |########################################
  36397.3 |
  36917.4 |
  37437.5 |
  37957.6 |
  38477.8 |####################
  38997.9 |
  39518.0 |
  40038.1 |##########
  40558.2 |
  41078.3 |
  41598.4 |##########
  42118.5 |
  42638.6 |
  43158.7 |##########
  43678.8 |
  (1 below, 1 above range)

nway (n=18, range 60756.7-112407.8 ns)
  60756.7 |########################################
  63339.2 |
  65921.8 |####################
  68504.3 |########################################
  71086.9 |########################################
  73669.4 |####################
  76252.0 |
  78834.6 |
  81417.1 |
  83999.7 |####################
  86582.2 |
  89164.8 |########################################
  91747.3 |
  94329.9 |########################################
  96912.4 |
  99495.0 |
  102077.6 |
  104660.1 |####################
  107242.7 |####################
  109825.2 |
  (1 below, 2 above range)

sequential (n=18, range 22087.8-26299.5 ns)
  22087.8 |##########################
  22298.4 |##########################
  22509.0 |#############
  22719.6 |
  22930.2 |##########################
  23140.8 |
  23351.3 |
  23561.9 |
  23772.5 |
  23983.1 |
  24193.7 |########################################
  24404.3 |
  24614.8 |#############
  24825.4 |
  25036.0 |
  25246.6 |#############
  25457.2 |
  25667.8 |
  25878.3 |
  26088.9 |##########################
  (2 below, 2 above range)

std_parallel (n=18, range 153663.6-205201.4 ns)
  153663.6 |#############
  156240.5 |##########################
  158817.3 |#############
  161394.2 |########################################
  163971.1 |
  166548.0 |
  169124.9 |
  171701.8 |
  174278.7 |
  176855.6 |
  179432.5 |#############
  182009.4 |##########################
  184586.3 |##########################
  187163.1 |#############
  189740.0 |
  192316.9 |
  194893.8 |##########################
  197470.7 |
  200047.6 |
  202624.5 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=105.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.3% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=88.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.0% of algo (FFI overhead may distort results)
- **nway**: CV=26.6% (high variance, measurements may be unstable)
- **nway**: bridge=94.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=104.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.0% of algo (FFI overhead may distort results)
