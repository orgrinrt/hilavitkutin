# append, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 23% faster than the next best (head_tail)

kernel_seq (33.29 us) leads head_tail (40.97 us) by 23%, a clear separation rather than a photo finish. CV 150.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 7.9x slower than the field

std_parallel (264.52 us) is 7.9x the fastest (33.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq is fastest but the noisiest (CV 150.9%)

kernel_seq wins on median (33.29 us) yet has the highest variance (CV 150.9%), while sequential is the steadiest (CV 0.7%, 56.85 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sequential shows alternating (throttle bounce) (autocorr -0.66)

sequential's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, head_tail, head_tail_dyn, sequential, kernel_nway} vs {nway, std_parallel} (275% apart)

The field splits into a fast tier {kernel_seq, head_tail, head_tail_dyn, sequential, kernel_nway} and a slow tier {nway, std_parallel} with a 275% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.9x the fastest

Fastest kernel_seq (33.29 us) to slowest std_parallel (264.52 us): 7.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 7.4x its best-20%

kernel_nway's best 20% of batches run at 19.37 us but its worst 20% at 143.69 us (7.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 33293.1 ns median (-18.7% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 7.95x (fastest 33293.1 ns, slowest 264521.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 8 |
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
| head_tail | 55054ns | 41205ns | 24023ns | 35800ns | 99452ns | base |
| head_tail_dyn | 64815ns | 42132ns | 23827ns | 37607ns | 126121ns | +17.73% |
| kernel_nway | 75488ns | 59979ns | 19672ns | 47407ns | 145518ns | +37.12% |
| kernel_seq | 63269ns | 33646ns | 23973ns | 32364ns | 129273ns | +14.92% |
| nway | 279484ns | 224763ns | 118854ns | 213096ns | 459383ns | +407.65% |
| sequential | 57205ns | 57025ns | 56758ns | 57028ns | 57694ns | +3.91% |
| std_parallel | 262341ns | 265914ns | 78125ns | 221130ns | 416265ns | +376.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 54333ns | 23812ns | 97771ns | base | 1.206 |
| head_tail_dyn | 63985ns | 23655ns | 124145ns | +17.76% | 1.024 |
| kernel_nway | 74595ns | 19369ns | 143687ns | +37.29% | 0.879 |
| kernel_seq | 62483ns | 23799ns | 127366ns | +15.00% | 1.049 |
| nway | 277215ns | 118244ns | 455137ns | +410.22% | 0.236 |
| sequential | 57027ns | 56561ns | 57516ns | +4.96% | 1.149 |
| std_parallel | 260619ns | 77654ns | 413452ns | +379.67% | 0.251 |

## Performance model

- Peak throughput: **3.384 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.600 | 47.3% |
| head_tail_dyn | 1.568 | 46.3% |
| kernel_nway | 1.100 | 32.5% |
| kernel_seq | 1.968 | 58.2% |
| nway | 0.293 | 8.7% |
| sequential | 1.153 | 34.1% |
| std_parallel | 0.248 | 7.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 55054ns | 55054ns | base |
| head_tail_dyn | 64815ns | 64815ns | +17.73% |
| kernel_nway | 75488ns | 75488ns | +37.12% |
| kernel_seq | 63269ns | 63269ns | +14.92% |
| nway | 279484ns | 279484ns | +407.65% |
| sequential | 57205ns | 57205ns | +3.91% |
| std_parallel | 262341ns | 262341ns | +376.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 40971ns | base | --- | [29363, 52008] | --- | --- | --- | --- |
| head_tail_dyn | 41792ns | no significant difference | [-9716, +19356]ns | [31837, 69668] | no | 0.3568 | 0.2379 | 0 |
| kernel_nway | 59569ns | no significant difference | [-14825, +32749]ns | [27042, 93054] | no | 0.8145 | 0.8145 | 0 |
| kernel_seq | 33293ns | no significant difference | [-12415, +19430]ns | [27556, 84833] | no | 0.8145 | 0.8145 | 0 |
| nway | 223358ns | +196193.1ns (+478.9%) | [+111920, +346164]ns | [162215, 384518] | YES | 0.0004 | 0.0001 | 0 |
| sequential | 56854ns | +16521.4ns (+40.3%) | [+5891, +26985]ns | [56723, 57439] | YES (adj: no) | 0.0618 | 0.0309 | 0 |
| std_parallel | 264522ns | +194207.4ns (+474.0%) | [+106518, +288214]ns | [145501, 332441] | YES | 0.0004 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 43501ns | -3.9% | +24.7% | -42.2% | +499.1% | +30.4% | +571.6% |
| 1 | 2 | 41745ns | -32.3% | +80.2% | -18.7% | +840.0% | +37.6% | +280.8% |
| 1 | 3 | 24586ns | +78.7% | +43.8% | +117.1% | +653.2% | +131.0% | +1178.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.121 | ok |
| head_tail_dyn | -0.656 | HIGH- (thermal bounce) |
| kernel_nway | -0.517 | HIGH- (thermal bounce) |
| kernel_seq | -0.030 | ok |
| nway | -0.581 | HIGH- (thermal bounce) |
| sequential | -0.659 | HIGH- (thermal bounce) |
| std_parallel | -0.655 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 463625.3ns | 54333.0ns | 853.3% | HIGH |
| head_tail_dyn | 436899.0ns | 63984.9ns | 682.8% | HIGH |
| kernel_nway | 712936.7ns | 74595.4ns | 955.7% | HIGH |
| kernel_seq | 497136.8ns | 62482.8ns | 795.6% | HIGH |
| nway | 1531542.3ns | 277215.3ns | 552.5% | HIGH |
| sequential | 296313.7ns | 57026.8ns | 519.6% | HIGH |
| std_parallel | 1392174.5ns | 260618.6ns | 534.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 23811.9-97771.2 ns)
  23811.9 |####################
  27509.9 |####################
  31207.9 |##########
  34905.8 |##########
  38603.8 |########################################
  42301.8 |
  45999.7 |
  49697.7 |####################
  53395.6 |
  57093.6 |
  60791.6 |
  64489.5 |
  68187.5 |
  71885.5 |##########
  75583.4 |
  79281.4 |
  82979.3 |
  86677.3 |
  90375.3 |
  94073.2 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 23654.9-124145.1 ns)
  23654.9 |################
  28679.4 |################
  33703.9 |
  38728.4 |########################################
  43752.9 |########
  48777.4 |
  53801.9 |
  58826.4 |
  63850.9 |########
  68875.5 |########
  73900.0 |
  78924.5 |
  83949.0 |
  88973.5 |
  93998.0 |
  99022.5 |
  104047.0 |
  109071.5 |
  114096.0 |
  119120.6 |########
  (2 below, 3 above range)

kernel_nway (n=18, range 19369.0-143687.4 ns)
  19369.0 |########################################
  25584.9 |########################################
  31800.8 |
  38016.8 |#############
  44232.7 |#############
  50448.6 |
  56664.5 |
  62880.4 |#############
  69096.4 |#############
  75312.3 |
  81528.2 |##########################
  87744.1 |
  93960.0 |
  100176.0 |#############
  106391.9 |
  112607.8 |#############
  118823.7 |
  125039.7 |
  131255.6 |
  137471.5 |
  (1 below, 3 above range)

kernel_seq (n=18, range 23798.5-127365.9 ns)
  23798.5 |########################################
  28976.9 |########################
  34155.3 |################
  39333.6 |########
  44512.0 |
  49690.4 |
  54868.8 |
  60047.1 |########
  65225.5 |
  70403.9 |
  75582.2 |
  80760.6 |
  85939.0 |
  91117.3 |
  96295.7 |
  101474.1 |
  106652.5 |########
  111830.8 |
  117009.2 |
  122187.6 |########
  (1 below, 3 above range)

nway (n=18, range 118243.9-455136.6 ns)
  118243.9 |##########
  135088.5 |
  151933.1 |########################################
  168777.8 |####################
  185622.4 |
  202467.1 |##########
  219311.7 |##########
  236156.3 |
  253001.0 |
  269845.6 |
  286690.2 |##########
  303534.9 |##########
  320379.5 |##########
  337224.2 |
  354068.8 |
  370913.4 |
  387758.1 |
  404602.7 |
  421447.4 |
  438292.0 |##########
  (1 below, 4 above range)

sequential (n=18, range 56561.0-57516.3 ns)
  56561.0 |####################
  56608.8 |
  56656.5 |####################
  56704.3 |####################
  56752.1 |##########
  56799.8 |
  56847.6 |####################
  56895.4 |##########
  56943.1 |
  56990.9 |
  57038.7 |
  57086.4 |
  57134.2 |
  57182.0 |
  57229.7 |
  57277.5 |
  57325.3 |
  57373.0 |
  57420.8 |########################################
  57468.6 |
  (1 below, 3 above range)

std_parallel (n=18, range 77654.4-413452.5 ns)
  77654.4 |########################################
  94444.3 |########################################
  111234.2 |
  128024.1 |
  144814.0 |
  161603.9 |
  178393.8 |####################
  195183.7 |
  211973.6 |####################
  228763.5 |####################
  245553.4 |####################
  262343.3 |####################
  279133.2 |####################
  295923.2 |
  312713.1 |####################
  329503.0 |########################################
  346292.9 |
  363082.8 |####################
  379872.7 |####################
  396662.6 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=71.8% (high variance, measurements may be unstable)
- **head_tail**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **head_tail**: bridge=1021.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=77.0% (high variance, measurements may be unstable)
- **head_tail_dyn**: worst_20/best_20 = 5.2x (possible bimodal distribution)
- **head_tail_dyn**: bridge=885.5% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=81.3% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 7.4x (possible bimodal distribution)
- **kernel_nway**: bridge=843.5% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=80.4% (high variance, measurements may be unstable)
- **kernel_seq**: worst_20/best_20 = 5.4x (possible bimodal distribution)
- **kernel_seq**: bridge=1227.6% of algo (FFI overhead may distort results)
- **nway**: CV=50.8% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **nway**: bridge=545.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=520.7% of algo (FFI overhead may distort results)
- **std_parallel**: CV=53.1% (high variance, measurements may be unstable)
- **std_parallel**: worst_20/best_20 = 5.3x (possible bimodal distribution)
- **std_parallel**: bridge=550.4% of algo (FFI overhead may distort results)
