# fold_heavy, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 129% faster than the next best (head_tail)

sequential (18.90 us) leads head_tail (43.18 us) by 129%, a clear separation rather than a photo finish. CV 110.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 5.1x slower than the field

std_parallel (95.76 us) is 5.1x the fastest (18.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 110.0%)

sequential wins on median (18.90 us) yet has the highest variance (CV 110.0%), while nway is the steadiest (CV 15.6%, 71.22 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.66)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {head_tail, kernel_nway, kernel_seq, head_tail_dyn, nway, std_parallel} (129% apart)

The field splits into a fast tier {sequential} and a slow tier {head_tail, kernel_nway, kernel_seq, head_tail_dyn, nway, std_parallel} with a 129% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.1x the fastest

Fastest sequential (18.90 us) to slowest std_parallel (95.76 us): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.4x its best-20%

sequential's best 20% of batches run at 18.33 us but its worst 20% at 62.65 us (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 18896.1 ns median (-56.2% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 5.07x (fastest 18896.1 ns, slowest 95760.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 1 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112765630 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 46324ns | 43649ns | 35466ns | 42459ns | 57550ns | base |
| head_tail_dyn | 53108ns | 51808ns | 34974ns | 49335ns | 67835ns | +14.65% |
| kernel_nway | 49412ns | 46022ns | 34746ns | 44616ns | 63938ns | +6.67% |
| kernel_seq | 48648ns | 48070ns | 21929ns | 46523ns | 65196ns | +5.02% |
| nway | 72952ns | 71752ns | 59092ns | 68953ns | 85881ns | +57.48% |
| sequential | 35878ns | 19040ns | 18467ns | 23482ns | 63178ns | -22.55% |
| std_parallel | 100309ns | 96200ns | 78403ns | 95092ns | 119087ns | +116.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 45378ns | 34955ns | 55880ns | base | 0.090 |
| head_tail_dyn | 52155ns | 34798ns | 66442ns | +14.94% | 0.079 |
| kernel_nway | 48592ns | 34567ns | 62735ns | +7.08% | 0.084 |
| kernel_seq | 47788ns | 21736ns | 63570ns | +5.31% | 0.086 |
| nway | 72371ns | 58639ns | 85168ns | +59.49% | 0.057 |
| sequential | 35590ns | 18330ns | 62650ns | -21.57% | 0.115 |
| std_parallel | 99486ns | 77971ns | 117660ns | +119.24% | 0.041 |

## Performance model

- Peak throughput: **0.223 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.095 | 42.4% |
| head_tail_dyn | 0.081 | 36.4% |
| kernel_nway | 0.090 | 40.4% |
| kernel_seq | 0.086 | 38.5% |
| nway | 0.058 | 25.7% |
| sequential | 0.217 | 97.0% |
| std_parallel | 0.043 | 19.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 46324ns | 46324ns | base |
| head_tail_dyn | 53108ns | 53108ns | +14.65% |
| kernel_nway | 49412ns | 49412ns | +6.67% |
| kernel_seq | 48648ns | 48648ns | +5.02% |
| nway | 72952ns | 72952ns | +57.48% |
| sequential | 35878ns | 35878ns | -22.55% |
| std_parallel | 100309ns | 100309ns | +116.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 43180ns | base | --- | [38895, 48899] | --- | --- | --- | --- |
| head_tail_dyn | 50294ns | no significant difference | [-3144, +13406]ns | [46869, 54743] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 45426ns | no significant difference | [-2237, +4639]ns | [40054, 53409] | no | 0.4807 | 0.4807 | 0 |
| kernel_seq | 47591ns | +6168.8ns (+14.3%) | [+3569, +10712]ns | [42224, 54039] | YES (adj: no) | 0.0618 | 0.0309 | 0 |
| nway | 71224ns | +25701.4ns (+59.5%) | [+22779, +31573]ns | [64395, 77869] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 18896ns | no significant difference | [-21946, +2873]ns | [18669, 55245] | no | 0.3568 | 0.2379 | 0 |
| std_parallel | 95760ns | +55599.9ns (+128.8%) | [+45137, +59849]ns | [89928, 108193] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 40298ns | +26.4% | +5.6% | +21.9% | +63.8% | -54.2% | +150.0% |
| 1 | 2 | 40960ns | +11.3% | +16.8% | +17.2% | +61.4% | -54.1% | +137.4% |
| 1 | 3 | 51586ns | -2.6% | +0.5% | -22.3% | +48.6% | -23.9% | +83.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.137 | ok |
| head_tail_dyn | -0.658 | HIGH- (thermal bounce) |
| kernel_nway | -0.004 | ok |
| kernel_seq | -0.106 | ok |
| nway | -0.162 | ok |
| sequential | -0.158 | ok |
| std_parallel | -0.005 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 45893.5ns | 45377.8ns | 101.1% | HIGH |
| head_tail_dyn | 54105.5ns | 52155.2ns | 103.7% | HIGH |
| kernel_nway | 49882.5ns | 48591.9ns | 102.7% | HIGH |
| kernel_seq | 47939.9ns | 47788.0ns | 100.3% | HIGH |
| nway | 73528.2ns | 72371.3ns | 101.6% | HIGH |
| sequential | 35842.0ns | 35590.5ns | 100.7% | HIGH |
| std_parallel | 103033.6ns | 99486.4ns | 103.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 34954.7-55879.5 ns)
  34954.7 |
  36001.0 |##########################
  37047.2 |
  38093.4 |########################################
  39139.7 |#############
  40185.9 |
  41232.2 |##########################
  42278.4 |
  43324.6 |##########################
  44370.9 |
  45417.1 |
  46463.4 |
  47509.6 |##########################
  48555.8 |#############
  49602.1 |#############
  50648.3 |
  51694.5 |
  52740.8 |#############
  53787.0 |
  54833.3 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 34797.9-66442.5 ns)
  34797.9 |#############
  36380.2 |
  37962.4 |#############
  39544.6 |
  41126.8 |
  42709.1 |
  44291.3 |
  45873.5 |##########################
  47455.8 |#############
  49038.0 |########################################
  50620.2 |#############
  52202.4 |##########################
  53784.7 |
  55366.9 |#############
  56949.1 |
  58531.4 |#############
  60113.6 |
  61695.8 |
  63278.0 |
  64860.3 |
  (2 below, 3 above range)

kernel_nway (n=18, range 34566.8-62735.4 ns)
  34566.8 |####################
  35975.3 |
  37383.7 |
  38792.1 |########################################
  40200.5 |########################################
  41609.0 |
  43017.4 |########################################
  44425.8 |
  45834.2 |####################
  47242.7 |
  48651.1 |####################
  50059.5 |####################
  51467.9 |####################
  52876.4 |
  54284.8 |####################
  55693.2 |####################
  57101.6 |
  58510.1 |
  59918.5 |####################
  61326.9 |
  (2 below, 2 above range)

kernel_seq (n=18, range 21735.5-63569.8 ns)
  21735.5 |
  23827.2 |
  25918.9 |
  28010.6 |
  30102.4 |####################
  32194.1 |
  34285.8 |
  36377.5 |
  38469.2 |
  40560.9 |########################################
  42652.6 |########################################
  44744.3 |########################################
  46836.1 |
  48927.8 |########################################
  51019.5 |########################################
  53111.2 |########################################
  55202.9 |
  57294.6 |
  59386.3 |
  61478.1 |####################
  (2 below, 2 above range)

nway (n=18, range 58639.3-85168.2 ns)
  58639.3 |####################
  59965.8 |########################################
  61292.2 |
  62618.7 |####################
  63945.1 |####################
  65271.6 |
  66598.0 |####################
  67924.4 |####################
  69250.9 |
  70577.3 |########################################
  71903.8 |
  73230.2 |########################################
  74556.7 |
  75883.1 |
  77209.5 |########################################
  78536.0 |####################
  79862.4 |
  81188.9 |
  82515.3 |
  83841.8 |
  (1 below, 3 above range)

sequential (n=18, range 18329.5-62650.4 ns)
  18329.5 |########################################
  20545.5 |
  22761.6 |
  24977.6 |
  27193.7 |#####
  29409.7 |
  31625.8 |
  33841.8 |
  36057.8 |
  38273.9 |
  40489.9 |
  42706.0 |
  44922.0 |
  47138.1 |
  49354.1 |#####
  51570.1 |
  53786.2 |#####
  56002.2 |
  58218.3 |##########
  60434.3 |
  (2 below, 3 above range)

std_parallel (n=18, range 77971.3-117660.0 ns)
  77971.3 |
  79955.7 |
  81940.1 |#############
  83924.6 |#############
  85909.0 |#############
  87893.4 |
  89877.9 |
  91862.3 |#############
  93846.8 |########################################
  95831.2 |#############
  97815.6 |#############
  99800.1 |
  101784.5 |
  103768.9 |
  105753.4 |#############
  107737.8 |########################################
  109722.3 |
  111706.7 |#############
  113691.1 |
  115675.6 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=22.3% (high variance, measurements may be unstable)
- **head_tail**: bridge=102.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=25.4% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=101.0% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=25.3% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=100.8% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=33.1% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=101.8% of algo (FFI overhead may distort results)
- **nway**: bridge=100.5% of algo (FFI overhead may distort results)
- **sequential**: CV=58.4% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **sequential**: bridge=100.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=103.4% of algo (FFI overhead may distort results)
