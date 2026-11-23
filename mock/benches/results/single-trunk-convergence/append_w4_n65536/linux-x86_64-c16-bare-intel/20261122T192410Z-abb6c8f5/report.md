# append, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 55% faster than the next best (head_tail_dyn)

kernel_nway (17.70 us) leads head_tail_dyn (27.35 us) by 55%, a clear separation rather than a photo finish. CV 200.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway beats baseline by 59% (significant)

kernel_nway is -24.12 us (59%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### nway is an outlier: 4.3x slower than the field

nway (75.24 us) is 4.3x the fastest (17.70 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway is fastest but the noisiest (CV 200.1%)

kernel_nway wins on median (17.70 us) yet has the highest variance (CV 200.1%), while sequential is the steadiest (CV 0.6%, 56.83 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### nway shows alternating (throttle bounce) (autocorr -0.63)

nway's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.3x the fastest

Fastest kernel_nway (17.70 us) to slowest nway (75.24 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### nway is inconsistent: worst-20% is 5.3x its best-20%

nway's best 20% of batches run at 42.03 us but its worst 20% at 221.88 us (5.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 17698.5 ns median (-56.4% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.25x (fastest 17698.5 ns, slowest 75236.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112846685 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 56940ns | 40833ns | 30806ns | 40657ns | 94433ns | base |
| head_tail_dyn | 45523ns | 27594ns | 23611ns | 25861ns | 85972ns | -20.05% |
| kernel_nway | 36625ns | 17967ns | 14593ns | 17293ns | 76639ns | -35.68% |
| kernel_seq | 53479ns | 31244ns | 24538ns | 31146ns | 101448ns | -6.08% |
| nway | 113249ns | 75597ns | 42204ns | 63546ns | 223326ns | +98.89% |
| sequential | 57068ns | 57024ns | 56688ns | 56937ns | 57454ns | +0.22% |
| std_parallel | 57039ns | 49330ns | 39967ns | 48590ns | 78250ns | +0.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 56320ns | 30573ns | 93020ns | base | 1.164 |
| head_tail_dyn | 44927ns | 23432ns | 84589ns | -20.23% | 1.459 |
| kernel_nway | 36081ns | 14381ns | 75450ns | -35.94% | 1.816 |
| kernel_seq | 52805ns | 24371ns | 99842ns | -6.24% | 1.241 |
| nway | 112596ns | 42028ns | 221879ns | +99.92% | 0.582 |
| sequential | 56885ns | 56504ns | 57268ns | +1.00% | 1.152 |
| std_parallel | 56564ns | 39731ns | 77400ns | +0.43% | 1.159 |

## Performance model

- Peak throughput: **4.557 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.615 | 35.4% |
| head_tail_dyn | 2.396 | 52.6% |
| kernel_nway | 3.703 | 81.3% |
| kernel_seq | 2.111 | 46.3% |
| nway | 0.871 | 19.1% |
| sequential | 1.153 | 25.3% |
| std_parallel | 1.337 | 29.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 56940ns | 56940ns | base |
| head_tail_dyn | 45523ns | 45523ns | -20.05% |
| kernel_nway | 36625ns | 36625ns | -35.68% |
| kernel_seq | 53479ns | 53479ns | -6.08% |
| nway | 113249ns | 113249ns | +98.89% |
| sequential | 57068ns | 57068ns | +0.22% |
| std_parallel | 57039ns | 57039ns | +0.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 40577ns | base | --- | [40179, 49089] | --- | --- | --- | --- |
| head_tail_dyn | 27349ns | -16627.6ns (-41.0%) | [-17143, -3416]ns | [23590, 31880] | YES | 0.0026 | 0.0013 | 0 |
| kernel_nway | 17698ns | -24122.5ns (-59.4%) | [-25713, -11650]ns | [14816, 35008] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 31040ns | no significant difference | [-14181, +3408]ns | [28530, 61121] | no | 0.0963 | 0.0963 | 0 |
| nway | 75237ns | +34645.6ns (+85.4%) | [+11114, +68607]ns | [42666, 150350] | YES | 0.0004 | 0.0001 | 0 |
| sequential | 56831ns | +16169.8ns (+39.8%) | [+8616, +16895]ns | [56652, 57045] | YES | 0.0113 | 0.0075 | 0 |
| std_parallel | 49021ns | +5501.0ns (+13.6%) | [+568, +16178]ns | [43665, 73154] | YES (adj: no) | 0.0963 | 0.0963 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 40800ns | -42.1% | -63.3% | -23.6% | +68.1% | +40.2% | +19.2% |
| 1 | 2 | 40553ns | -41.4% | -60.1% | -35.4% | +256.0% | +39.7% | +13.2% |
| 1 | 3 | 36948ns | -20.0% | -41.3% | +65.4% | +15.5% | +53.8% | +68.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.135 | ok |
| head_tail_dyn | -0.152 | ok |
| kernel_nway | -0.082 | ok |
| kernel_seq | -0.247 | moderate- |
| nway | -0.627 | HIGH- (thermal bounce) |
| sequential | -0.377 | moderate- |
| std_parallel | -0.265 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 405018.4ns | 56320.2ns | 719.1% | HIGH |
| head_tail_dyn | 407630.8ns | 44927.3ns | 907.3% | HIGH |
| kernel_nway | 365001.4ns | 36080.6ns | 1011.6% | HIGH |
| kernel_seq | 443081.4ns | 52804.6ns | 839.1% | HIGH |
| nway | 564813.1ns | 112596.5ns | 501.6% | HIGH |
| sequential | 296664.8ns | 56884.7ns | 521.5% | HIGH |
| std_parallel | 397881.4ns | 56564.1ns | 703.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 30572.8-93019.9 ns)
  30572.8 |####
  33695.1 |
  36817.5 |
  39939.8 |########################################
  43062.2 |
  46184.6 |####
  49306.9 |####
  52429.3 |
  55551.6 |####
  58674.0 |
  61796.4 |
  64918.7 |
  68041.1 |
  71163.4 |
  74285.8 |
  77408.2 |
  80530.5 |
  83652.9 |
  86775.2 |
  89897.6 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 23431.5-84588.8 ns)
  23431.5 |########################################
  26489.4 |###############
  29547.3 |#####
  32605.1 |#####
  35663.0 |
  38720.9 |
  41778.7 |
  44836.6 |
  47894.5 |
  50952.3 |
  54010.2 |
  57068.0 |
  60125.9 |
  63183.8 |
  66241.6 |
  69299.5 |
  72357.4 |#####
  75415.2 |
  78473.1 |
  81531.0 |
  (1 below, 3 above range)

kernel_nway (n=18, range 14380.8-75450.4 ns)
  14380.8 |########################################
  17434.3 |##########
  20487.8 |
  23541.3 |#####
  26594.8 |
  29648.2 |
  32701.7 |#####
  35755.2 |#####
  38808.7 |
  41862.2 |
  44915.6 |#####
  47969.1 |
  51022.6 |
  54076.1 |
  57129.6 |
  60183.0 |
  63236.5 |
  66290.0 |
  69343.5 |
  72397.0 |
  (1 below, 3 above range)

kernel_seq (n=18, range 24371.3-99841.9 ns)
  24371.3 |###########
  28144.9 |########################################
  31918.4 |
  35691.9 |
  39465.4 |#####
  43239.0 |
  47012.5 |#####
  50786.0 |
  54559.6 |
  58333.1 |
  62106.6 |
  65880.2 |
  69653.7 |#####
  73427.2 |
  77200.7 |
  80974.3 |#####
  84747.8 |
  88521.3 |
  92294.9 |#####
  96068.4 |
  (2 below, 2 above range)

nway (n=18, range 42028.0-221878.9 ns)
  42028.0 |########################################
  51020.5 |######
  60013.1 |######
  69005.6 |
  77998.2 |######
  86990.7 |######
  95983.3 |######
  104975.8 |
  113968.3 |
  122960.9 |
  131953.4 |
  140946.0 |
  149938.5 |######
  158931.1 |
  167923.6 |
  176916.2 |######
  185908.7 |######
  194901.3 |
  203893.8 |
  212886.4 |
  (1 below, 3 above range)

sequential (n=18, range 56503.7-57267.9 ns)
  56503.7 |
  56541.9 |
  56580.2 |########################################
  56618.4 |#############
  56656.6 |#############
  56694.8 |#############
  56733.0 |#############
  56771.2 |
  56809.4 |#############
  56847.6 |##########################
  56885.8 |##########################
  56924.0 |
  56962.2 |
  57000.4 |
  57038.7 |
  57076.9 |
  57115.1 |
  57153.3 |#############
  57191.5 |#############
  57229.7 |
  (1 below, 3 above range)

std_parallel (n=18, range 39731.5-77400.2 ns)
  39731.5 |########################################
  41614.9 |########################################
  43498.3 |########################################
  45381.8 |########################################
  47265.2 |
  49148.7 |
  51032.1 |########################################
  52915.5 |
  54799.0 |
  56682.4 |
  58565.9 |
  60449.3 |
  62332.7 |####################
  64216.2 |
  66099.6 |
  67983.0 |
  69866.5 |
  71749.9 |####################
  73633.4 |########################################
  75516.8 |####################
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=65.6% (high variance, measurements may be unstable)
- **head_tail**: worst_20/best_20 = 3.0x (possible bimodal distribution)
- **head_tail**: bridge=907.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=82.5% (high variance, measurements may be unstable)
- **head_tail_dyn**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **head_tail_dyn**: bridge=1322.4% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=98.2% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 5.2x (possible bimodal distribution)
- **kernel_nway**: bridge=1922.4% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=75.7% (high variance, measurements may be unstable)
- **kernel_seq**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **kernel_seq**: bridge=1209.4% of algo (FFI overhead may distort results)
- **nway**: CV=76.7% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 5.3x (possible bimodal distribution)
- **nway**: bridge=563.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=520.9% of algo (FFI overhead may distort results)
- **std_parallel**: CV=28.1% (high variance, measurements may be unstable)
- **std_parallel**: bridge=783.3% of algo (FFI overhead may distort results)
