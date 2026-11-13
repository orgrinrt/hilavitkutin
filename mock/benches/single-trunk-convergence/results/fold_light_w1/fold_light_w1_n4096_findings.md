# fold_light, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 130% faster than the next best (sequential)

kernel_nway (3.79 us) leads sequential (8.72 us) by 130%, a clear separation rather than a photo finish. CV 307.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 10.6x slower than the field

std_parallel (40.28 us) is 10.6x the fastest (3.79 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway is fastest but the noisiest (CV 307.4%)

kernel_nway wins on median (3.79 us) yet has the highest variance (CV 307.4%), while sequential is the steadiest (CV 22.1%, 8.72 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### std_parallel shows alternating (throttle bounce) (autocorr -0.67)

std_parallel's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 10.6x the fastest

Fastest kernel_nway (3.79 us) to slowest std_parallel (40.28 us): 10.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail is inconsistent: worst-20% is 13.8x its best-20%

head_tail's best 20% of batches run at 2.20 us but its worst 20% at 30.49 us (13.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 3792.2 ns median (-83.5% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 10.62x (fastest 3792.2 ns, slowest 40275.1 ns)

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
| head_tail | 19161ns | 23429ns | 2497ns | 16622ns | 31301ns | base |
| head_tail_dyn | 43297ns | 26304ns | 20239ns | 24634ns | 82820ns | +125.97% |
| kernel_nway | 12289ns | 4191ns | 3536ns | 4181ns | 28827ns | -35.87% |
| kernel_seq | 29162ns | 26685ns | 22961ns | 25812ns | 37288ns | +52.19% |
| nway | 32037ns | 35722ns | 16035ns | 29503ns | 43840ns | +67.20% |
| sequential | 9932ns | 8964ns | 7621ns | 9010ns | 12472ns | -48.16% |
| std_parallel | 44116ns | 40968ns | 37173ns | 40505ns | 53005ns | +130.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 18635ns | 2205ns | 30494ns | base | 0.220 |
| head_tail_dyn | 42725ns | 19881ns | 82049ns | +129.27% | 0.096 |
| kernel_nway | 11810ns | 3264ns | 28170ns | -36.63% | 0.347 |
| kernel_seq | 28566ns | 22531ns | 36429ns | +53.29% | 0.143 |
| nway | 31601ns | 15710ns | 43328ns | +69.58% | 0.130 |
| sequential | 9639ns | 7403ns | 12102ns | -48.27% | 0.425 |
| std_parallel | 43550ns | 36568ns | 52370ns | +133.70% | 0.094 |

## Performance model

- Peak throughput: **1.858 Gops/s** (head_tail; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.178 | 9.6% |
| head_tail_dyn | 0.159 | 8.5% |
| kernel_nway | 1.080 | 58.1% |
| kernel_seq | 0.156 | 8.4% |
| nway | 0.116 | 6.3% |
| sequential | 0.470 | 25.3% |
| std_parallel | 0.102 | 5.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 19161ns | 19161ns | base |
| head_tail_dyn | 43297ns | 43297ns | +125.97% |
| kernel_nway | 12289ns | 12289ns | -35.87% |
| kernel_seq | 29162ns | 29162ns | +52.19% |
| nway | 32037ns | 32037ns | +67.20% |
| sequential | 9932ns | 9932ns | -48.16% |
| std_parallel | 44116ns | 44116ns | +130.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 23020ns | base | --- | [3214, 24042] | --- | --- | --- | --- |
| head_tail_dyn | 25824ns | +5221.8ns (+22.7%) | [+2924, +18183]ns | [21523, 27972] | YES | 0.0003 | 0.0001 | 0 |
| kernel_nway | 3792ns | no significant difference | [-17144, +2114]ns | [3353, 26524] | no | 0.4807 | 0.4807 | 0 |
| kernel_seq | 26223ns | +5081.8ns (+22.1%) | [+2814, +21482]ns | [24114, 28656] | YES | 0.0463 | 0.0309 | 0 |
| nway | 35264ns | +13496.7ns (+58.6%) | [+12658, +14528]ns | [16045, 38801] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 8722ns | no significant difference | [-13950, +3654]ns | [8257, 11292] | no | 0.2855 | 0.2379 | 0 |
| std_parallel | 40275ns | +19516.9ns (+84.8%) | [+16189, +33578]ns | [39465, 43021] | YES | 0.0003 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 23123ns | +12.5% | +17.8% | +14.8% | +68.1% | -47.1% | +71.9% |
| 1 | 2 | 24210ns | +21.7% | -84.3% | -1.5% | +45.7% | -64.4% | +75.2% |
| 1 | 3 | 2360ns | +764.4% | +39.7% | +1030.1% | +570.9% | +220.6% | +1580.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.193 | ok |
| head_tail_dyn | -0.415 | moderate- |
| kernel_nway | -0.156 | ok |
| kernel_seq | -0.666 | HIGH- (thermal bounce) |
| nway | -0.091 | ok |
| sequential | -0.061 | ok |
| std_parallel | -0.666 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 22775.6ns | 18635.0ns | 122.2% | HIGH |
| head_tail_dyn | 26878.8ns | 42725.0ns | 62.9% | HIGH |
| kernel_nway | 11600.8ns | 11809.6ns | 98.2% | HIGH |
| kernel_seq | 29084.2ns | 28565.9ns | 101.8% | HIGH |
| nway | 31798.1ns | 31601.0ns | 100.6% | HIGH |
| sequential | 10554.6ns | 9639.1ns | 109.5% | HIGH |
| std_parallel | 46085.8ns | 43550.3ns | 105.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 2204.7-30494.2 ns)
   2204.7 |##############################
   3619.2 |##########
   5033.7 |
   6448.2 |
   7862.6 |
   9277.1 |
  10691.6 |
  12106.0 |
  13520.5 |
  14935.0 |
  16349.5 |
  17763.9 |
  19178.4 |
  20592.9 |##########
  22007.3 |########################################
  23421.8 |########################################
  24836.3 |####################
  26250.8 |
  27665.2 |
  29079.7 |
  (2 below, 1 above range)

head_tail_dyn (n=18, range 19880.9-82048.8 ns)
  19880.9 |########################################
  22989.3 |########################################
  26097.7 |##############################
  29206.1 |##############################
  32314.5 |
  35422.9 |
  38531.3 |
  41639.7 |
  44748.1 |##########
  47856.5 |
  50964.9 |
  54073.3 |
  57181.7 |
  60290.0 |
  63398.4 |
  66506.8 |
  69615.2 |
  72723.6 |
  75832.0 |
  78940.4 |
  (2 below, 1 above range)

kernel_nway (n=18, range 3263.7-28169.6 ns)
   3263.7 |########################################
   4509.0 |####
   5754.3 |
   6999.6 |
   8244.9 |
   9490.2 |
  10735.5 |
  11980.8 |
  13226.1 |
  14471.4 |
  15716.7 |
  16962.0 |
  18207.2 |
  19452.5 |
  20697.8 |
  21943.1 |
  23188.4 |
  24433.7 |
  25679.0 |############
  26924.3 |####
  (1 below, 2 above range)

kernel_seq (n=18, range 22530.9-36428.9 ns)
  22530.9 |##########
  23225.8 |####################
  23920.7 |####################
  24615.6 |##########
  25310.5 |##########
  26005.4 |########################################
  26700.3 |
  27395.2 |##########
  28090.1 |
  28785.0 |
  29479.9 |##########
  30174.8 |
  30869.7 |##########
  31564.6 |
  32259.5 |
  32954.4 |##########
  33649.3 |
  34344.2 |
  35039.1 |
  35734.0 |
  (1 below, 2 above range)

nway (n=18, range 15709.5-43327.9 ns)
  15709.5 |########################################
  17090.4 |
  18471.3 |
  19852.3 |
  21233.2 |
  22614.1 |
  23995.0 |
  25375.9 |
  26756.9 |
  28137.8 |
  29518.7 |
  30899.6 |
  32280.5 |########
  33661.5 |################
  35042.4 |########
  36423.3 |########
  37804.2 |################################
  39185.2 |########
  40566.1 |########
  41947.0 |
  (1 below, 1 above range)

sequential (n=18, range 7403.0-12102.3 ns)
   7403.0 |####################
   7638.0 |####################
   7873.0 |
   8107.9 |####################
   8342.9 |########################################
   8577.9 |########################################
   8812.8 |####################
   9047.8 |
   9282.7 |
   9517.7 |####################
   9752.7 |
   9987.6 |
  10222.6 |####################
  10457.6 |####################
  10692.5 |
  10927.5 |
  11162.4 |
  11397.4 |
  11632.4 |
  11867.3 |
  (2 below, 5 above range)

std_parallel (n=18, range 36567.8-52370.5 ns)
  36567.8 |####################
  37357.9 |
  38148.1 |
  38938.2 |########################################
  39728.3 |####################
  40518.5 |##############################
  41308.6 |##########
  42098.7 |
  42888.9 |
  43679.0 |##########
  44469.1 |
  45259.3 |##########
  46049.4 |
  46839.5 |
  47629.7 |
  48419.8 |
  49209.9 |
  50000.1 |
  50790.2 |##########
  51580.3 |##########
  (1 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=75.6% (high variance, measurements may be unstable)
- **head_tail**: worst_20/best_20 = 13.8x (possible bimodal distribution)
- **head_tail**: bridge=101.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=163.6% (high variance, measurements may be unstable)
- **head_tail_dyn**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **head_tail_dyn**: bridge=104.2% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=98.7% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 8.6x (possible bimodal distribution)
- **kernel_nway**: bridge=102.0% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=27.1% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=107.6% of algo (FFI overhead may distort results)
- **nway**: CV=40.0% (high variance, measurements may be unstable)
- **nway**: bridge=100.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=102.1% of algo (FFI overhead may distort results)
- **std_parallel**: CV=22.2% (high variance, measurements may be unstable)
- **std_parallel**: bridge=101.8% of algo (FFI overhead may distort results)
