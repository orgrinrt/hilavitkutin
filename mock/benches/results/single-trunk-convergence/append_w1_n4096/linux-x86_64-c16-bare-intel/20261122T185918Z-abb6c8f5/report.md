# append, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway is an outlier: 3.4x slower than the field

nway (46.21 us) is 3.4x the fastest (13.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {head_tail_dyn, sequential, head_tail, kernel_seq, kernel_nway} vs {std_parallel, nway} (181% apart)

The field splits into a fast tier {head_tail_dyn, sequential, head_tail, kernel_seq, kernel_nway} and a slow tier {std_parallel, nway} with a 181% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.4x the fastest

Fastest head_tail_dyn (13.57 us) to slowest nway (46.21 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.5x its best-20%

sequential's best 20% of batches run at 9.87 us but its worst 20% at 34.20 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 13571.4 ns median (-2.9% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 3.40x (fastest 13571.4 ns, slowest 46207.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 1 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112772868 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 15141ns | 14521ns | 11390ns | 14022ns | 18696ns | base |
| head_tail_dyn | 15013ns | 14000ns | 13219ns | 14092ns | 17292ns | -0.85% |
| kernel_nway | 16476ns | 15964ns | 13281ns | 15503ns | 19532ns | +8.81% |
| kernel_seq | 17329ns | 15195ns | 12967ns | 14783ns | 23330ns | +14.45% |
| nway | 48757ns | 46690ns | 39215ns | 45501ns | 58412ns | +222.01% |
| sequential | 20532ns | 14069ns | 10026ns | 14554ns | 34751ns | +35.60% |
| std_parallel | 44862ns | 44420ns | 33592ns | 41996ns | 54795ns | +196.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 14265ns | 10954ns | 17025ns | base | 0.287 |
| head_tail_dyn | 14260ns | 12790ns | 15902ns | -0.04% | 0.287 |
| kernel_nway | 15640ns | 12840ns | 18004ns | +9.64% | 0.262 |
| kernel_seq | 16434ns | 12485ns | 21656ns | +15.20% | 0.249 |
| nway | 48225ns | 38789ns | 57729ns | +238.06% | 0.085 |
| sequential | 20215ns | 9873ns | 34198ns | +41.70% | 0.203 |
| std_parallel | 44001ns | 33045ns | 53651ns | +208.45% | 0.093 |

## Performance model

- Peak throughput: **0.415 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.293 | 70.6% |
| head_tail_dyn | 0.302 | 72.8% |
| kernel_nway | 0.266 | 64.1% |
| kernel_seq | 0.279 | 67.3% |
| nway | 0.089 | 21.4% |
| sequential | 0.296 | 71.3% |
| std_parallel | 0.095 | 22.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 15141ns | 15141ns | base |
| head_tail_dyn | 15013ns | 15013ns | -0.85% |
| kernel_nway | 16476ns | 16476ns | +8.81% |
| kernel_seq | 17329ns | 17329ns | +14.45% |
| nway | 48757ns | 48757ns | +222.01% |
| sequential | 20532ns | 20532ns | +35.60% |
| std_parallel | 44862ns | 44862ns | +196.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 13983ns | base | --- | [12618, 15898] | --- | --- | --- | --- |
| head_tail_dyn | 13571ns | no significant difference | [-1045, +1185]ns | [13206, 15280] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 15399ns | +828.2ns (+5.9%) | [+201, +3218]ns | [14397, 16656] | YES (adj: no) | 0.1925 | 0.0963 | 0 |
| kernel_seq | 14675ns | no significant difference | [-127, +2464]ns | [13436, 16848] | no | 0.3568 | 0.2379 | 0 |
| nway | 46207ns | +32340.4ns (+231.3%) | [+28466, +38082]ns | [41428, 53124] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 13853ns | no significant difference | [-2450, +15511]ns | [10057, 32314] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 43285ns | +30342.5ns (+217.0%) | [+23891, +33979]ns | [37597, 47697] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 11942ns | +18.0% | +31.6% | +18.8% | +304.6% | +54.6% | +301.3% |
| 1 | 2 | 14113ns | +1.8% | +9.1% | +3.3% | +256.8% | +21.9% | +226.6% |
| 1 | 3 | 15521ns | -12.6% | -1.8% | +1.8% | +177.4% | -11.0% | +126.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.010 | ok |
| head_tail_dyn | -0.390 | moderate- |
| kernel_nway | -0.022 | ok |
| kernel_seq | -0.052 | ok |
| nway | -0.341 | moderate- |
| sequential | -0.044 | ok |
| std_parallel | -0.096 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 66940.7ns | 14265.2ns | 469.3% | HIGH |
| head_tail_dyn | 60943.4ns | 14259.8ns | 427.4% | HIGH |
| kernel_nway | 69032.4ns | 15640.2ns | 441.4% | HIGH |
| kernel_seq | 73406.4ns | 16434.1ns | 446.7% | HIGH |
| nway | 97668.2ns | 48225.4ns | 202.5% | HIGH |
| sequential | 50548.3ns | 20214.5ns | 250.1% | HIGH |
| std_parallel | 99099.7ns | 44000.8ns | 225.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 10953.8-17024.5 ns)
  10953.8 |
  11257.3 |
  11560.8 |####################
  11864.4 |
  12167.9 |########################################
  12471.5 |
  12775.0 |####################
  13078.5 |####################
  13382.1 |####################
  13685.6 |####################
  13989.1 |####################
  14292.7 |
  14596.2 |####################
  14899.8 |####################
  15203.3 |
  15506.8 |####################
  15810.4 |####################
  16113.9 |
  16417.4 |####################
  16721.0 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 12789.6-15902.1 ns)
  12789.6 |########################################
  12945.2 |####################
  13100.8 |########################################
  13256.5 |####################
  13412.1 |########################################
  13567.7 |####################
  13723.3 |
  13879.0 |
  14034.6 |
  14190.2 |
  14345.8 |
  14501.5 |
  14657.1 |####################
  14812.7 |####################
  14968.4 |
  15124.0 |####################
  15279.6 |####################
  15435.2 |########################################
  15590.9 |
  15746.5 |
  (1 below, 2 above range)

kernel_nway (n=18, range 12840.0-18003.6 ns)
  12840.0 |
  13098.2 |####################
  13356.4 |####################
  13614.6 |
  13872.7 |
  14130.9 |####################
  14389.1 |####################
  14647.3 |####################
  14905.5 |####################
  15163.6 |####################
  15421.8 |########################################
  15680.0 |
  15938.2 |
  16196.3 |####################
  16454.5 |####################
  16712.7 |####################
  16970.9 |
  17229.1 |
  17487.2 |
  17745.4 |####################
  (2 below, 3 above range)

kernel_seq (n=18, range 12484.9-21656.4 ns)
  12484.9 |########################################
  12943.5 |########################################
  13402.1 |########################################
  13860.7 |####################
  14319.2 |########################################
  14777.8 |####################
  15236.4 |
  15694.9 |
  16153.5 |####################
  16612.1 |########################################
  17070.7 |####################
  17529.2 |
  17987.8 |
  18446.4 |####################
  18905.0 |
  19363.5 |
  19822.1 |
  20280.7 |
  20739.3 |
  21197.8 |
  (1 below, 2 above range)

nway (n=18, range 38789.1-57728.6 ns)
  38789.1 |####################
  39736.1 |
  40683.0 |########################################
  41630.0 |####################
  42577.0 |
  43524.0 |########################################
  44470.9 |
  45417.9 |####################
  46364.9 |####################
  47311.9 |
  48258.8 |
  49205.8 |####################
  50152.8 |####################
  51099.8 |
  52046.7 |
  52993.7 |########################################
  53940.7 |####################
  54887.7 |
  55834.6 |
  56781.6 |
  (2 below, 3 above range)

sequential (n=18, range 9873.2-34198.1 ns)
   9873.2 |########################################
  11089.4 |
  12305.7 |
  13521.9 |
  14738.2 |
  15954.4 |
  17170.7 |#####
  18386.9 |
  19603.1 |
  20819.4 |
  22035.6 |
  23251.9 |#####
  24468.1 |
  25684.4 |#####
  26900.6 |
  28116.8 |
  29333.1 |
  30549.3 |
  31765.6 |##########
  32981.8 |#####
  (1 below, 3 above range)

std_parallel (n=18, range 33045.0-53651.4 ns)
  33045.0 |####################
  34075.4 |
  35105.7 |####################
  36136.0 |####################
  37166.3 |
  38196.6 |####################
  39226.9 |
  40257.3 |####################
  41287.6 |
  42317.9 |########################################
  43348.2 |########################################
  44378.5 |
  45408.9 |########################################
  46439.2 |
  47469.5 |
  48499.8 |####################
  49530.1 |
  50560.4 |
  51590.8 |####################
  52621.1 |####################
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=472.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=436.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=443.3% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=33.0% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=482.2% of algo (FFI overhead may distort results)
- **nway**: bridge=198.8% of algo (FFI overhead may distort results)
- **sequential**: CV=54.6% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **sequential**: bridge=257.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=227.2% of algo (FFI overhead may distort results)
