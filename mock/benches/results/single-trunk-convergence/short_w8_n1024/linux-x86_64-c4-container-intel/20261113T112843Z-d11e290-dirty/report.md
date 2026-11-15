# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 169% faster than the next best (kernel_seq)

sequential (8.17 us) leads kernel_seq (21.97 us) by 169%, a clear separation rather than a photo finish. CV 23.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 70% (significant)

sequential is -21.92 us (70%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 20.1x slower than the field

std_parallel (163.89 us) is 20.1x the fastest (8.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.67)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 20.1x the fastest

Fastest sequential (8.17 us) to slowest std_parallel (163.89 us): 20.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### std_parallel is inconsistent: worst-20% is 1.6x its best-20%

std_parallel's best 20% of batches run at 149.87 us but its worst 20% at 234.54 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 8169.6 ns median (-74.0% vs baseline)
- 3 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 20.06x (fastest 8169.6 ns, slowest 163893.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 8 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100242360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 32626ns | 31847ns | 28179ns | 31041ns | 37226ns | base |
| head_tail_dyn | 29332ns | 29197ns | 26001ns | 28654ns | 32016ns | -10.09% |
| kernel_nway | 69994ns | 68024ns | 58170ns | 66662ns | 80904ns | +114.54% |
| kernel_seq | 22932ns | 22508ns | 19803ns | 22094ns | 25753ns | -29.71% |
| nway | 96069ns | 97834ns | 82293ns | 95619ns | 103632ns | +194.46% |
| sequential | 9525ns | 8556ns | 7872ns | 8467ns | 11939ns | -70.80% |
| std_parallel | 184580ns | 164346ns | 150393ns | 162221ns | 235211ns | +465.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 32145ns | 27690ns | 36662ns | base | 0.032 |
| head_tail_dyn | 28902ns | 25630ns | 31530ns | -10.09% | 0.035 |
| kernel_nway | 69409ns | 57764ns | 80247ns | +115.92% | 0.015 |
| kernel_seq | 22394ns | 19443ns | 25020ns | -30.34% | 0.046 |
| nway | 95386ns | 81804ns | 102841ns | +196.74% | 0.011 |
| sequential | 9160ns | 7575ns | 11483ns | -71.50% | 0.112 |
| std_parallel | 184010ns | 149874ns | 234535ns | +472.44% | 0.006 |

## Performance model

- Peak throughput: **0.135 Gops/s** (sequential; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.033 | 24.1% |
| head_tail_dyn | 0.036 | 26.3% |
| kernel_nway | 0.015 | 11.2% |
| kernel_seq | 0.047 | 34.5% |
| nway | 0.011 | 7.8% |
| sequential | 0.125 | 92.7% |
| std_parallel | 0.006 | 4.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 32626ns | 32626ns | base |
| head_tail_dyn | 29332ns | 29332ns | -10.09% |
| kernel_nway | 69994ns | 69994ns | +114.54% |
| kernel_seq | 22932ns | 22932ns | -29.71% |
| nway | 96069ns | 96069ns | +194.46% |
| sequential | 9525ns | 9525ns | -70.80% |
| std_parallel | 184580ns | 184580ns | +465.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 31376ns | base | --- | [29590, 34030] | --- | --- | --- | --- |
| head_tail_dyn | 28811ns | -1894.8ns (-6.0%) | [-4531, -991]ns | [27141, 29725] | YES | 0.0075 | 0.0075 | 0 |
| kernel_nway | 67556ns | +34821.2ns (+111.0%) | [+28825, +45362]ns | [63743, 75630] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 21969ns | -8710.6ns (-27.8%) | [-12422, -6617]ns | [21223, 23359] | YES | 0.0002 | 0.0001 | 0 |
| nway | 97249ns | +64029.6ns (+204.1%) | [+58611, +67625]ns | [92463, 100084] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 8170ns | -21919.1ns (-69.9%) | [-25382, -19392]ns | [7964, 10054] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 163893ns | +130733.1ns (+416.7%) | [+127052, +143261]ns | [158222, 180764] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 35689ns | -17.7% | +74.1% | -40.5% | +172.5% | -77.6% | +384.1% |
| 1 | 2 | 29590ns | -4.1% | +166.8% | -26.6% | +224.2% | -65.6% | +436.0% |
| 1 | 3 | 30355ns | -3.1% | +117.1% | -18.3% | +223.1% | -73.4% | +480.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.237 | moderate- |
| head_tail_dyn | -0.666 | HIGH- (thermal bounce) |
| kernel_nway | -0.636 | HIGH- (thermal bounce) |
| kernel_seq | -0.100 | ok |
| nway | -0.567 | HIGH- (thermal bounce) |
| sequential | -0.665 | HIGH- (thermal bounce) |
| std_parallel | -0.645 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 32456.2ns | 32144.9ns | 101.0% | HIGH |
| head_tail_dyn | 29731.4ns | 28901.5ns | 102.9% | HIGH |
| kernel_nway | 72334.8ns | 69408.6ns | 104.2% | HIGH |
| kernel_seq | 22888.0ns | 22393.7ns | 102.2% | HIGH |
| nway | 96385.9ns | 95385.5ns | 101.0% | HIGH |
| sequential | 9323.8ns | 9160.5ns | 101.8% | HIGH |
| std_parallel | 174396.6ns | 184010.5ns | 94.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 27689.8-36662.5 ns)
  27689.8 |
  28138.5 |####################
  28587.1 |
  29035.7 |########################################
  29484.4 |####################
  29933.0 |####################
  30381.6 |####################
  30830.2 |####################
  31278.9 |####################
  31727.5 |########################################
  32176.1 |
  32624.8 |
  33073.4 |####################
  33522.0 |
  33970.7 |
  34419.3 |####################
  34867.9 |
  35316.6 |####################
  35765.2 |####################
  36213.8 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 25629.8-31530.3 ns)
  25629.8 |
  25924.8 |
  26219.8 |#############
  26514.9 |#############
  26809.9 |#############
  27104.9 |#############
  27399.9 |
  27695.0 |
  27990.0 |
  28285.0 |########################################
  28580.0 |
  28875.1 |#############
  29170.1 |#############
  29465.1 |##########################
  29760.1 |#############
  30055.1 |
  30350.2 |
  30645.2 |
  30940.2 |
  31235.2 |
  (2 below, 4 above range)

kernel_nway (n=18, range 57763.8-80247.0 ns)
  57763.8 |####################
  58887.9 |
  60012.1 |
  61136.3 |####################
  62260.4 |####################
  63384.6 |
  64508.7 |########################################
  65632.9 |####################
  66757.1 |########################################
  67881.2 |####################
  69005.4 |
  70129.6 |####################
  71253.7 |
  72377.9 |
  73502.0 |
  74626.2 |####################
  75750.4 |
  76874.5 |####################
  77998.7 |####################
  79122.9 |####################
  (2 below, 2 above range)

kernel_seq (n=18, range 19442.6-25019.6 ns)
  19442.6 |
  19721.5 |
  20000.3 |#############
  20279.2 |
  20558.0 |
  20836.9 |#############
  21115.7 |########################################
  21394.6 |#############
  21673.4 |#############
  21952.3 |##########################
  22231.1 |#############
  22510.0 |#############
  22788.8 |
  23067.7 |
  23346.5 |
  23625.4 |
  23904.2 |##########################
  24183.1 |
  24461.9 |
  24740.8 |
  (2 below, 3 above range)

nway (n=18, range 81804.1-102841.1 ns)
  81804.1 |
  82856.0 |
  83907.8 |#############
  84959.7 |#############
  86011.5 |
  87063.4 |
  88115.2 |
  89167.1 |
  90218.9 |
  91270.8 |
  92322.6 |##########################
  93374.4 |
  94426.3 |#############
  95478.1 |#############
  96530.0 |#############
  97581.8 |#############
  98633.7 |########################################
  99685.5 |#############
  100737.4 |#############
  101789.2 |
  (2 below, 3 above range)

sequential (n=18, range 7575.2-11482.7 ns)
   7575.2 |
   7770.6 |########################################
   7966.0 |########################################
   8161.4 |####################
   8356.7 |
   8552.1 |##########
   8747.5 |
   8942.9 |
   9138.2 |
   9333.6 |
   9529.0 |
   9724.3 |
   9919.7 |##########
  10115.1 |##########
  10310.5 |
  10505.8 |##########
  10701.2 |
  10896.6 |
  11092.0 |##########
  11287.3 |
  (1 below, 2 above range)

std_parallel (n=18, range 149873.7-234535.4 ns)
  149873.7 |#############
  154106.8 |########################################
  158339.8 |########################################
  162572.9 |########################################
  166806.0 |#############
  171039.1 |
  175272.2 |
  179505.3 |##########################
  183738.4 |
  187971.4 |
  192204.5 |#############
  196437.6 |
  200670.7 |#############
  204903.8 |
  209136.9 |
  213370.0 |#############
  217603.1 |
  221836.1 |
  226069.2 |
  230302.3 |
  (1 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=102.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=103.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=102.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=101.6% of algo (FFI overhead may distort results)
- **nway**: bridge=100.9% of algo (FFI overhead may distort results)
- **sequential**: CV=20.7% (high variance, measurements may be unstable)
- **sequential**: bridge=109.9% of algo (FFI overhead may distort results)
- **std_parallel**: CV=34.6% (high variance, measurements may be unstable)
- **std_parallel**: bridge=103.2% of algo (FFI overhead may distort results)
