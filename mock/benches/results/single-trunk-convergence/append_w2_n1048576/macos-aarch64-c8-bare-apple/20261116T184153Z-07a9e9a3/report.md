# append, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 13% faster than the next best (std_parallel)

kernel_nway (80.96 us) leads std_parallel (91.79 us) by 13%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway beats baseline by 28% (significant)

kernel_nway is -32.09 us (28%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 9.4x slower than the field

sequential (758.65 us) is 9.4x the fastest (80.96 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.63)

kernel_seq's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, std_parallel, kernel_seq, head_tail, head_tail_dyn} vs {nway, sequential} (248% apart)

The field splits into a fast tier {kernel_nway, std_parallel, kernel_seq, head_tail, head_tail_dyn} and a slow tier {nway, sequential} with a 248% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 9.4x the fastest

Fastest kernel_nway (80.96 us) to slowest sequential (758.65 us): 9.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 80964.6 ns median (-28.8% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 9.37x (fastest 80964.6 ns, slowest 758650.0 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 118234ns | 115217ns | 113125ns | 114876ns | 125825ns | base |
| head_tail_dyn | 121352ns | 116588ns | 113293ns | 115838ns | 133651ns | +2.64% |
| kernel_nway | 83420ns | 83156ns | 81676ns | 82960ns | 84981ns | -29.45% |
| kernel_seq | 114438ns | 114017ns | 112515ns | 113841ns | 116294ns | -3.21% |
| nway | 402955ns | 401360ns | 395397ns | 399964ns | 411221ns | +240.81% |
| sequential | 762914ns | 760638ns | 754740ns | 759812ns | 771652ns | +545.26% |
| std_parallel | 96971ns | 93779ns | 85969ns | 91710ns | 110362ns | -17.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 116574ns | 111636ns | 123988ns | base | 8.995 |
| head_tail_dyn | 119344ns | 111717ns | 131039ns | +2.38% | 8.786 |
| kernel_nway | 81178ns | 79288ns | 82768ns | -30.36% | 12.917 |
| kernel_seq | 112895ns | 111560ns | 114472ns | -3.16% | 9.288 |
| nway | 400649ns | 393583ns | 408902ns | +243.69% | 2.617 |
| sequential | 760984ns | 752985ns | 769256ns | +552.79% | 1.378 |
| std_parallel | 94934ns | 84389ns | 108122ns | -18.56% | 11.045 |

## Performance model

- Peak throughput: **13.225 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 9.227 | 69.8% |
| head_tail_dyn | 9.171 | 69.3% |
| kernel_nway | 12.951 | 97.9% |
| kernel_seq | 9.307 | 70.4% |
| nway | 2.633 | 19.9% |
| sequential | 1.382 | 10.5% |
| std_parallel | 11.424 | 86.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 118234ns | 118234ns | base |
| head_tail_dyn | 121352ns | 121352ns | +2.64% |
| kernel_nway | 83420ns | 83420ns | -29.45% |
| kernel_seq | 114438ns | 114438ns | -3.21% |
| nway | 402955ns | 402955ns | +240.81% |
| sequential | 762914ns | 762914ns | +545.26% |
| std_parallel | 96971ns | 96971ns | -17.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 113640ns | base | --- | [112583, 115462] | --- | --- | --- | --- |
| head_tail_dyn | 114340ns | no significant difference | [-1848, +4904]ns | [112196, 119702] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 80965ns | -32091.7ns (-28.2%) | [-34904, -31300]ns | [80444, 81481] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 112671ns | no significant difference | [-2798, +317]ns | [111827, 113140] | no | 0.5768 | 0.4807 | 0 |
| nway | 398185ns | +283897.9ns (+249.8%) | [+281294, +286429]ns | [396577, 400121] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 758650ns | +644322.9ns (+567.0%) | [+641281, +647790]ns | [756765, 762946] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 91785ns | -23568.8ns (-20.7%) | [-26623, -19296]ns | [86388, 94021] | YES | 0.0002 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 113773ns | +2.8% | -28.2% | -0.7% | +249.5% | +566.2% | -20.0% |
| 1 | 2 | 113640ns | -1.1% | -29.0% | -1.5% | +250.3% | +567.2% | -18.7% |
| 1 | 3 | 115746ns | +2.8% | -30.2% | -2.6% | +244.2% | +559.4% | -25.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.200 | moderate- |
| head_tail_dyn | -0.605 | HIGH- (thermal bounce) |
| kernel_nway | -0.192 | ok |
| kernel_seq | -0.631 | HIGH- (thermal bounce) |
| nway | -0.002 | ok |
| sequential | -0.145 | ok |
| std_parallel | -0.304 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 5879327.8ns | 116573.8ns | 5043.4% | HIGH |
| head_tail_dyn | 5939493.3ns | 119344.4ns | 4976.8% | HIGH |
| kernel_nway | 5854085.2ns | 81178.5ns | 7211.4% | HIGH |
| kernel_seq | 5860982.4ns | 112894.9ns | 5191.5% | HIGH |
| nway | 6147744.9ns | 400648.6ns | 1534.4% | HIGH |
| sequential | 6492557.9ns | 760984.0ns | 853.2% | HIGH |
| std_parallel | 5922081.7ns | 94934.0ns | 6238.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 111636.1-123987.5 ns)
  111636.1 |#############
  112253.7 |########################################
  112871.2 |##########################
  113488.8 |########################################
  114106.4 |#############
  114723.9 |
  115341.5 |##########################
  115959.1 |#############
  116576.7 |
  117194.2 |
  117811.8 |
  118429.4 |#############
  119046.9 |
  119664.5 |#############
  120282.1 |
  120899.6 |
  121517.2 |
  122134.8 |
  122752.4 |
  123369.9 |
  (2 below, 1 above range)

head_tail_dyn (n=18, range 111716.7-131038.9 ns)
  111716.7 |########################################
  112682.8 |
  113648.9 |#############
  114615.0 |######
  115581.1 |
  116547.2 |
  117513.3 |#############
  118479.4 |######
  119445.6 |######
  120411.7 |
  121377.8 |
  122343.9 |
  123310.0 |
  124276.1 |######
  125242.2 |
  126208.3 |
  127174.5 |
  128140.6 |
  129106.7 |
  130072.8 |
  (1 below, 3 above range)

kernel_nway (n=18, range 79287.5-82768.1 ns)
  79287.5 |
  79461.5 |#############
  79635.6 |##########################
  79809.6 |
  79983.6 |
  80157.6 |#############
  80331.7 |
  80505.7 |#############
  80679.7 |########################################
  80853.8 |
  81027.8 |#############
  81201.8 |##########################
  81375.8 |#############
  81549.9 |#############
  81723.9 |#############
  81897.9 |
  82072.0 |
  82246.0 |
  82420.0 |
  82594.0 |
  (1 below, 3 above range)

kernel_seq (n=18, range 111559.7-114471.5 ns)
  111559.7 |#############
  111705.3 |##########################
  111850.9 |#############
  111996.5 |#############
  112142.1 |#############
  112287.7 |
  112433.3 |
  112578.9 |#############
  112724.4 |########################################
  112870.0 |#############
  113015.6 |
  113161.2 |
  113306.8 |#############
  113452.4 |
  113598.0 |#############
  113743.6 |
  113889.2 |
  114034.7 |#############
  114180.3 |
  114325.9 |
  (2 below, 2 above range)

nway (n=18, range 393583.3-408902.1 ns)
  393583.3 |
  394349.3 |#############
  395115.2 |#############
  395881.1 |##########################
  396647.1 |########################################
  397413.0 |#############
  398179.0 |#############
  398944.9 |##########################
  399710.8 |##########################
  400476.8 |
  401242.7 |
  402008.6 |
  402774.6 |#############
  403540.5 |
  404306.5 |#############
  405072.4 |
  405838.3 |
  406604.3 |
  407370.2 |
  408136.1 |
  (1 below, 2 above range)

sequential (n=18, range 752984.7-769256.2 ns)
  752984.7 |
  753798.3 |#############
  754611.9 |
  755425.4 |#############
  756239.0 |#############
  757052.6 |########################################
  757866.2 |#############
  758679.7 |#############
  759493.3 |#############
  760306.9 |
  761120.5 |#############
  761934.1 |
  762747.6 |#############
  763561.2 |##########################
  764374.8 |
  765188.4 |
  766001.9 |#############
  766815.5 |
  767629.1 |#############
  768442.7 |
  (2 below, 1 above range)

std_parallel (n=18, range 84388.9-108121.5 ns)
  84388.9 |##########
  85575.5 |##############################
  86762.2 |
  87948.8 |
  89135.4 |####################
  90322.1 |
  91508.7 |########################################
  92695.3 |
  93882.0 |####################
  95068.6 |
  96255.2 |####################
  97441.8 |##########
  98628.5 |
  99815.1 |
  101001.7 |
  102188.4 |
  103375.0 |
  104561.6 |
  105748.3 |
  106934.9 |
  (2 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=5165.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=5155.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=7226.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=5199.1% of algo (FFI overhead may distort results)
- **nway**: bridge=1542.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=854.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=6394.4% of algo (FFI overhead may distort results)
