# fold_heavy, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway beats baseline by 30% (significant)

kernel_nway is -15.44 us (30%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 2.8x slower than the field

std_parallel (96.53 us) is 2.8x the fastest (35.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {kernel_nway, sequential, head_tail_dyn, kernel_seq, head_tail, nway} vs {std_parallel} (53% apart)

The field splits into a fast tier {kernel_nway, sequential, head_tail_dyn, kernel_seq, head_tail, nway} and a slow tier {std_parallel} with a 53% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### sequential is inconsistent: worst-20% is 3.4x its best-20%

sequential's best 20% of batches run at 18.61 us but its worst 20% at 64.16 us (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 35007.3 ns median (-31.1% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.76x (fastest 35007.3 ns, slowest 96530.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 4 |
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
| head_tail | 48980ns | 51501ns | 31182ns | 46521ns | 61567ns | base |
| head_tail_dyn | 41407ns | 43468ns | 22126ns | 39514ns | 53887ns | -15.46% |
| kernel_nway | 37300ns | 35693ns | 31410ns | 35344ns | 43177ns | -23.85% |
| kernel_seq | 47283ns | 46773ns | 31034ns | 45613ns | 57912ns | -3.46% |
| nway | 107995ns | 63819ns | 57524ns | 64687ns | 198192ns | +120.49% |
| sequential | 38888ns | 37038ns | 18755ns | 28383ns | 64712ns | -20.60% |
| std_parallel | 101536ns | 97162ns | 85608ns | 96058ns | 117717ns | +107.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 48009ns | 30676ns | 59973ns | base | 0.085 |
| head_tail_dyn | 40717ns | 21937ns | 52877ns | -15.19% | 0.101 |
| kernel_nway | 36282ns | 30812ns | 41440ns | -24.43% | 0.113 |
| kernel_seq | 46342ns | 30648ns | 56429ns | -3.47% | 0.088 |
| nway | 106877ns | 57016ns | 195995ns | +122.62% | 0.038 |
| sequential | 38567ns | 18611ns | 64158ns | -19.67% | 0.106 |
| std_parallel | 100681ns | 85050ns | 116530ns | +109.71% | 0.041 |

## Performance model

- Peak throughput: **0.220 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.081 | 36.6% |
| head_tail_dyn | 0.097 | 44.0% |
| kernel_nway | 0.117 | 53.2% |
| kernel_seq | 0.089 | 40.3% |
| nway | 0.065 | 29.4% |
| sequential | 0.112 | 50.7% |
| std_parallel | 0.042 | 19.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 48980ns | 48980ns | base |
| head_tail_dyn | 41407ns | 41407ns | -15.46% |
| kernel_nway | 37300ns | 37300ns | -23.85% |
| kernel_seq | 47283ns | 47283ns | -3.46% |
| nway | 107995ns | 107995ns | +120.49% |
| sequential | 38888ns | 38888ns | -20.60% |
| std_parallel | 101536ns | 101536ns | +107.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 50815ns | base | --- | [38432, 56826] | --- | --- | --- | --- |
| head_tail_dyn | 42313ns | -8956.8ns (-17.6%) | [-11712, -3508]ns | [32933, 50250] | YES | 0.0151 | 0.0075 | 0 |
| kernel_nway | 35007ns | -15445.0ns (-30.4%) | [-20454, -1474]ns | [32769, 38438] | YES (adj: no) | 0.1155 | 0.0963 | 0 |
| kernel_seq | 46156ns | no significant difference | [-7552, +2324]ns | [42075, 51100] | no | 0.1155 | 0.0963 | 0 |
| nway | 63225ns | +12294.5ns (+24.2%) | [+8337, +76511]ns | [61052, 129110] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 36732ns | no significant difference | [-28686, +9616]ns | [18747, 52492] | no | 0.8145 | 0.8145 | 0 |
| std_parallel | 96530ns | +51466.4ns (+101.3%) | [+42872, +62349]ns | [93036, 102666] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 33427ns | +50.3% | +15.0% | +52.7% | +539.3% | +47.4% | +182.1% |
| 1 | 2 | 56980ns | -21.0% | -43.4% | -15.0% | +9.6% | -66.9% | +73.0% |
| 1 | 3 | 47993ns | -24.4% | -28.8% | -13.8% | +24.7% | -60.9% | +141.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.416 | moderate- |
| head_tail_dyn | -0.014 | ok |
| kernel_nway | -0.359 | moderate- |
| kernel_seq | -0.045 | ok |
| nway | -0.158 | ok |
| sequential | -0.165 | ok |
| std_parallel | -0.071 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 48949.7ns | 48008.6ns | 102.0% | HIGH |
| head_tail_dyn | 40609.9ns | 40716.7ns | 99.7% | HIGH |
| kernel_nway | 36611.0ns | 36281.9ns | 100.9% | HIGH |
| kernel_seq | 47028.6ns | 46341.7ns | 101.5% | HIGH |
| nway | 108445.3ns | 106877.3ns | 101.5% | HIGH |
| sequential | 39405.1ns | 38567.5ns | 102.2% | HIGH |
| std_parallel | 101349.7ns | 100680.8ns | 100.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 30675.8-59972.5 ns)
  30675.8 |####################
  32140.6 |####################
  33605.5 |####################
  35070.3 |
  36535.2 |
  38000.0 |########################################
  39464.8 |
  40929.7 |
  42394.5 |
  43859.3 |####################
  45324.2 |####################
  46789.0 |
  48253.8 |####################
  49718.7 |
  51183.5 |####################
  52648.4 |####################
  54113.2 |########################################
  55578.0 |
  57042.9 |
  58507.7 |########################################
  (1 below, 3 above range)

head_tail_dyn (n=18, range 21937.0-52877.0 ns)
  21937.0 |
  23484.0 |
  25031.0 |
  26578.0 |
  28125.0 |####################
  29672.0 |####################
  31219.0 |####################
  32766.0 |####################
  34313.0 |####################
  35860.0 |
  37407.0 |####################
  38954.0 |
  40501.0 |####################
  42048.0 |####################
  43595.0 |
  45142.0 |####################
  46689.0 |
  48236.0 |########################################
  49783.0 |####################
  51330.0 |########################################
  (2 below, 2 above range)

kernel_nway (n=18, range 30812.3-41439.6 ns)
  30812.3 |####################
  31343.6 |
  31875.0 |########################################
  32406.4 |
  32937.7 |####################
  33469.1 |####################
  34000.5 |####################
  34531.8 |####################
  35063.2 |####################
  35594.6 |
  36125.9 |
  36657.3 |
  37188.7 |
  37720.0 |########################################
  38251.4 |########################################
  38782.8 |####################
  39314.1 |
  39845.5 |
  40376.9 |
  40908.2 |
  (2 below, 3 above range)

kernel_seq (n=18, range 30648.1-56428.7 ns)
  30648.1 |
  31937.2 |
  33226.2 |
  34515.2 |
  35804.2 |####################
  37093.3 |
  38382.3 |####################
  39671.3 |####################
  40960.4 |####################
  42249.4 |####################
  43538.4 |####################
  44827.5 |########################################
  46116.5 |####################
  47405.5 |####################
  48694.5 |########################################
  49983.6 |
  51272.6 |####################
  52561.6 |
  53850.7 |########################################
  55139.7 |
  (1 below, 2 above range)

nway (n=18, range 57016.3-195995.5 ns)
  57016.3 |########################################
  63965.3 |####
  70914.2 |####
  77863.2 |####
  84812.2 |
  91761.1 |
  98710.1 |
  105659.0 |
  112608.0 |
  119556.9 |
  126505.9 |
  133454.9 |
  140403.8 |
  147352.8 |
  154301.7 |
  161250.7 |
  168199.6 |####
  175148.6 |
  182097.6 |
  189046.5 |
  (1 below, 4 above range)

sequential (n=18, range 18610.5-64157.9 ns)
  18610.5 |########################################
  20887.9 |
  23165.3 |
  25442.6 |
  27720.0 |
  29997.4 |
  32274.7 |######
  34552.1 |
  36829.5 |######
  39106.9 |######
  41384.2 |
  43661.6 |
  45939.0 |######
  48216.3 |
  50493.7 |######
  52771.1 |
  55048.4 |######
  57325.8 |
  59603.2 |######
  61880.5 |######
  (2 below, 2 above range)

std_parallel (n=18, range 85050.4-116530.3 ns)
  85050.4 |####################
  86624.4 |
  88198.4 |####################
  89772.4 |
  91346.4 |########################################
  92920.4 |########################################
  94494.4 |####################
  96068.4 |########################################
  97642.4 |####################
  99216.3 |####################
  100790.3 |####################
  102364.3 |
  103938.3 |####################
  105512.3 |
  107086.3 |####################
  108660.3 |
  110234.3 |
  111808.3 |
  113382.3 |
  114956.3 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=23.4% (high variance, measurements may be unstable)
- **head_tail**: bridge=103.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=28.6% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=99.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=103.0% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=22.6% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=102.7% of algo (FFI overhead may distort results)
- **nway**: CV=66.6% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **nway**: bridge=112.5% of algo (FFI overhead may distort results)
- **sequential**: CV=53.4% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **sequential**: bridge=102.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=103.3% of algo (FFI overhead may distort results)
