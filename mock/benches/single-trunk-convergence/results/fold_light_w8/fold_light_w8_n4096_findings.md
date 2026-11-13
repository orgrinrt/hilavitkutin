# fold_light, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 224% faster than the next best (kernel_seq)

sequential (7.72 us) leads kernel_seq (25.01 us) by 224%, a clear separation rather than a photo finish. CV 15.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 76% (significant)

sequential is -23.32 us (76%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 20.2x slower than the field

std_parallel (155.75 us) is 20.2x the fastest (7.72 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.66)

std_parallel's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 20.2x the fastest

Fastest sequential (7.72 us) to slowest std_parallel (155.75 us): 20.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.6x its best-20%

kernel_nway's best 20% of batches run at 51.25 us but its worst 20% at 83.02 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7721.1 ns median (-74.9% vs baseline)
- 2 variants significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 20.17x (fastest 7721.1 ns, slowest 155745.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 8 |
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
| head_tail | 30910ns | 31202ns | 25075ns | 29766ns | 35542ns | base |
| head_tail_dyn | 34574ns | 33961ns | 31228ns | 33455ns | 37925ns | +11.85% |
| kernel_nway | 68421ns | 62200ns | 51612ns | 63737ns | 83852ns | +121.36% |
| kernel_seq | 27439ns | 25633ns | 23238ns | 24992ns | 33212ns | -11.23% |
| nway | 71836ns | 72171ns | 54261ns | 69249ns | 84504ns | +132.40% |
| sequential | 8555ns | 8210ns | 7651ns | 8004ns | 9834ns | -72.32% |
| std_parallel | 174890ns | 156217ns | 142555ns | 156273ns | 218983ns | +465.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 30352ns | 24694ns | 34926ns | base | 0.135 |
| head_tail_dyn | 33958ns | 30764ns | 37287ns | +11.88% | 0.121 |
| kernel_nway | 67775ns | 51251ns | 83016ns | +123.30% | 0.060 |
| kernel_seq | 26865ns | 22803ns | 32658ns | -11.49% | 0.152 |
| nway | 71375ns | 53902ns | 84007ns | +135.16% | 0.057 |
| sequential | 8270ns | 7432ns | 9566ns | -72.75% | 0.495 |
| std_parallel | 174239ns | 141922ns | 218070ns | +474.07% | 0.024 |

## Performance model

- Peak throughput: **0.551 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.133 | 24.1% |
| head_tail_dyn | 0.123 | 22.3% |
| kernel_nway | 0.067 | 12.1% |
| kernel_seq | 0.164 | 29.7% |
| nway | 0.057 | 10.4% |
| sequential | 0.530 | 96.3% |
| std_parallel | 0.026 | 4.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 30910ns | 30910ns | base |
| head_tail_dyn | 34574ns | 34574ns | +11.85% |
| kernel_nway | 68421ns | 68421ns | +121.36% |
| kernel_seq | 27439ns | 27439ns | -11.23% |
| nway | 71836ns | 71836ns | +132.40% |
| sequential | 8555ns | 8555ns | -72.32% |
| std_parallel | 174890ns | 174890ns | +465.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 30777ns | base | --- | [26373, 33453] | --- | --- | --- | --- |
| head_tail_dyn | 33307ns | +3701.3ns (+12.0%) | [+1368, +5595]ns | [32062, 34182] | YES | 0.0075 | 0.0075 | 0 |
| kernel_nway | 61361ns | +35760.7ns (+116.2%) | [+27952, +47542]ns | [57876, 80113] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 25006ns | -3564.6ns (-11.6%) | [-6756, -1899]ns | [23371, 27584] | YES | 0.0075 | 0.0075 | 0 |
| nway | 71542ns | +44575.2ns (+144.8%) | [+30995, +47059]ns | [61954, 78185] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 7721ns | -23318.5ns (-75.8%) | [-25269, -17880]ns | [7473, 8162] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 155746ns | +121992.1ns (+396.4%) | [+117427, +157610]ns | [149223, 184062] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 30777ns | +8.9% | +97.4% | -17.9% | +92.7% | -75.8% | +394.2% |
| 1 | 2 | 25087ns | +23.8% | +118.3% | -8.6% | +205.6% | -67.5% | +637.1% |
| 1 | 3 | 34998ns | +0.1% | +139.2% | -25.9% | +121.6% | -77.9% | +320.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.547 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.573 | HIGH- (thermal bounce) |
| kernel_nway | -0.291 | moderate- |
| kernel_seq | -0.636 | HIGH- (thermal bounce) |
| nway | -0.142 | ok |
| sequential | -0.570 | HIGH- (thermal bounce) |
| std_parallel | -0.657 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 31322.8ns | 30351.6ns | 103.2% | HIGH |
| head_tail_dyn | 34675.3ns | 33957.8ns | 102.1% | HIGH |
| kernel_nway | 71531.6ns | 67774.8ns | 105.5% | HIGH |
| kernel_seq | 30684.0ns | 26865.0ns | 114.2% | HIGH |
| nway | 70575.7ns | 71375.2ns | 98.9% | HIGH |
| sequential | 8574.0ns | 8269.6ns | 103.7% | HIGH |
| std_parallel | 186629.8ns | 174239.4ns | 107.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 24693.6-34925.7 ns)
  24693.6 |########################################
  25205.2 |
  25716.8 |####################
  26228.4 |####################
  26740.0 |
  27251.6 |
  27763.2 |
  28274.8 |
  28786.4 |
  29298.0 |####################
  29809.6 |########################################
  30321.2 |
  30832.8 |########################################
  31344.5 |
  31856.1 |
  32367.7 |####################
  32879.3 |####################
  33390.9 |####################
  33902.5 |
  34414.1 |####################
  (2 below, 3 above range)

head_tail_dyn (n=18, range 30764.1-37286.6 ns)
  30764.1 |########################################
  31090.3 |####################
  31416.4 |####################
  31742.5 |
  32068.6 |
  32394.7 |####################
  32720.9 |########################################
  33047.0 |########################################
  33373.1 |
  33699.2 |########################################
  34025.3 |########################################
  34351.5 |
  34677.6 |####################
  35003.7 |
  35329.8 |####################
  35656.0 |
  35982.1 |
  36308.2 |
  36634.3 |
  36960.4 |
  (1 below, 2 above range)

kernel_nway (n=18, range 51251.3-83015.8 ns)
  51251.3 |
  52839.5 |
  54427.8 |########################################
  56016.0 |####################
  57604.2 |####################
  59192.4 |########################################
  60780.7 |########################################
  62368.9 |
  63957.1 |
  65545.3 |
  67133.6 |
  68721.8 |
  70310.0 |
  71898.2 |
  73486.5 |
  75074.7 |####################
  76662.9 |
  78251.1 |########################################
  79839.4 |####################
  81427.6 |
  (2 below, 4 above range)

kernel_seq (n=18, range 22803.3-32658.2 ns)
  22803.3 |########################################
  23296.0 |##########
  23788.8 |
  24281.5 |####################
  24774.3 |##############################
  25267.0 |##########
  25759.8 |
  26252.5 |
  26745.3 |
  27238.0 |####################
  27730.8 |
  28223.5 |
  28716.2 |##########
  29209.0 |
  29701.7 |
  30194.5 |
  30687.2 |
  31180.0 |
  31672.7 |
  32165.5 |##########
  (1 below, 2 above range)

nway (n=18, range 53902.1-84007.3 ns)
  53902.1 |
  55407.3 |
  56912.6 |####################
  58417.8 |########################################
  59923.1 |####################
  61428.4 |
  62933.6 |####################
  64438.9 |
  65944.1 |
  67449.4 |
  68954.7 |########################################
  70459.9 |########################################
  71965.2 |
  73470.4 |####################
  74975.7 |
  76481.0 |########################################
  77986.2 |####################
  79491.5 |####################
  80996.7 |####################
  82502.0 |
  (1 below, 2 above range)

sequential (n=18, range 7431.6-9565.6 ns)
   7431.6 |########################################
   7538.3 |########
   7645.0 |########
   7751.7 |########
   7858.4 |
   7965.1 |
   8071.8 |################################
   8178.5 |
   8285.2 |
   8391.9 |
   8498.6 |
   8605.3 |
   8712.0 |
   8818.7 |########
   8925.4 |
   9032.1 |
   9138.8 |
   9245.5 |
   9352.2 |
   9458.9 |
  (2 below, 3 above range)

std_parallel (n=18, range 141921.6-218070.4 ns)
  141921.6 |##########################
  145729.0 |##########################
  149536.4 |########################################
  153343.9 |##########################
  157151.3 |
  160958.8 |#############
  164766.2 |
  168573.6 |
  172381.1 |
  176188.5 |#############
  179996.0 |#############
  183803.4 |##########################
  187610.8 |#############
  191418.3 |#############
  195225.7 |
  199033.2 |
  202840.6 |
  206648.0 |
  210455.5 |
  214262.9 |
  (1 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=104.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=122.9% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=20.8% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=100.1% of algo (FFI overhead may distort results)
- **nway**: bridge=102.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=105.3% of algo (FFI overhead may distort results)
- **std_parallel**: CV=29.8% (high variance, measurements may be unstable)
- **std_parallel**: bridge=116.9% of algo (FFI overhead may distort results)
