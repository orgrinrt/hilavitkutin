# fold_heavy, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail_dyn dominates: 55% faster than the next best (kernel_nway)

head_tail_dyn (33.96 us) leads kernel_nway (52.66 us) by 55%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### head_tail_dyn beats baseline by 37% (significant)

head_tail_dyn is -20.05 us (37%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 2.7x slower than the field

std_parallel (91.88 us) is 2.7x the fastest (33.96 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.59)

sequential's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### kernel_nway is inconsistent: worst-20% is 2.0x its best-20%

kernel_nway's best 20% of batches run at 46.26 us but its worst 20% at 94.56 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 33962.5 ns median (-37.5% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.71x (fastest 33962.5 ns, slowest 91881.2 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 54330ns | 54802ns | 50856ns | 54368ns | 56010ns | base |
| head_tail_dyn | 34589ns | 34423ns | 33440ns | 34276ns | 35632ns | -36.34% |
| kernel_nway | 65635ns | 53292ns | 46885ns | 51962ns | 95519ns | +20.81% |
| kernel_seq | 53844ns | 53452ns | 52504ns | 53269ns | 55376ns | -0.90% |
| nway | 65006ns | 64400ns | 61228ns | 63829ns | 68661ns | +19.65% |
| sequential | 57959ns | 57802ns | 57251ns | 57730ns | 58656ns | +6.68% |
| std_parallel | 93519ns | 92267ns | 87051ns | 91532ns | 99734ns | +72.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 53836ns | 50282ns | 55578ns | base | 1.217 |
| head_tail_dyn | 34136ns | 32981ns | 35147ns | -36.59% | 1.920 |
| kernel_nway | 64876ns | 46256ns | 94560ns | +20.51% | 1.010 |
| kernel_seq | 53575ns | 52321ns | 55003ns | -0.48% | 1.223 |
| nway | 64419ns | 60626ns | 68095ns | +19.66% | 1.017 |
| sequential | 57766ns | 57082ns | 58438ns | +7.30% | 1.135 |
| std_parallel | 93115ns | 86624ns | 99305ns | +72.96% | 0.704 |

## Performance model

- Peak throughput: **1.987 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.207 | 60.7% |
| head_tail_dyn | 1.930 | 97.1% |
| kernel_nway | 1.245 | 62.6% |
| kernel_seq | 1.232 | 62.0% |
| nway | 1.027 | 51.7% |
| sequential | 1.137 | 57.2% |
| std_parallel | 0.713 | 35.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 54330ns | 54330ns | base |
| head_tail_dyn | 34589ns | 34589ns | -36.34% |
| kernel_nway | 65635ns | 65635ns | +20.81% |
| kernel_seq | 53844ns | 53844ns | -0.90% |
| nway | 65006ns | 65006ns | +19.65% |
| sequential | 57959ns | 57959ns | +6.68% |
| std_parallel | 93519ns | 93519ns | +72.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 54315ns | base | --- | [53202, 55185] | --- | --- | --- | --- |
| head_tail_dyn | 33962ns | -20047.9ns (-36.9%) | [-20810, -18850]ns | [33629, 34321] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 52660ns | no significant difference | [-4915, +4231]ns | [49050, 55452] | no | 0.2855 | 0.2379 | 0 |
| kernel_seq | 53194ns | no significant difference | [-2019, +1275]ns | [52804, 54038] | no | 1.0000 | 1.0000 | 0 |
| nway | 63821ns | +8931.2ns (+16.4%) | [+8154, +12044]ns | [62458, 65492] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 57615ns | +3433.3ns (+6.3%) | [+2525, +4840]ns | [57433, 57983] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 91881ns | +37854.2ns (+69.7%) | [+35885, +40710]ns | [89331, 94748] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 54844ns | -37.1% | -8.1% | -3.6% | +15.8% | +4.9% | +69.5% |
| 1 | 2 | 54385ns | -37.6% | -10.5% | -2.8% | +16.5% | +6.8% | +70.5% |
| 1 | 3 | 53083ns | -37.0% | +24.9% | +2.7% | +24.7% | +7.9% | +71.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.047 | ok |
| head_tail_dyn | -0.000 | ok |
| kernel_nway | -0.220 | moderate- |
| kernel_seq | -0.174 | ok |
| nway | -0.197 | ok |
| sequential | -0.591 | HIGH- (thermal bounce) |
| std_parallel | -0.104 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 112660.9ns | 53836.1ns | 209.3% | HIGH |
| head_tail_dyn | 97681.7ns | 34136.1ns | 286.2% | HIGH |
| kernel_nway | 111322.4ns | 64875.7ns | 171.6% | HIGH |
| kernel_seq | 107100.7ns | 53575.0ns | 199.9% | HIGH |
| nway | 126508.8ns | 64419.0ns | 196.4% | HIGH |
| sequential | 115684.3ns | 57766.0ns | 200.3% | HIGH |
| std_parallel | 116054.9ns | 93114.8ns | 124.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 50281.9-55577.8 ns)
  50281.9 |
  50546.7 |
  50811.5 |
  51076.3 |
  51341.1 |
  51605.9 |
  51870.7 |
  52135.5 |####################
  52400.3 |####################
  52665.1 |
  52929.8 |####################
  53194.6 |####################
  53459.4 |
  53724.2 |########################################
  53989.0 |####################
  54253.8 |
  54518.6 |########################################
  54783.4 |########################################
  55048.2 |
  55313.0 |########################################
  (2 below, 3 above range)

head_tail_dyn (n=18, range 32980.6-35147.2 ns)
  32980.6 |
  33088.9 |#############
  33197.2 |#############
  33305.6 |
  33413.9 |
  33522.2 |########################################
  33630.6 |#############
  33738.9 |
  33847.2 |##########################
  33955.6 |#############
  34063.9 |#############
  34172.2 |##########################
  34280.6 |
  34388.9 |#############
  34497.2 |
  34605.6 |##########################
  34713.9 |
  34822.2 |
  34930.6 |
  35038.9 |#############
  (1 below, 1 above range)

kernel_nway (n=18, range 46255.6-94559.7 ns)
  46255.6 |##########
  48670.8 |########################################
  51086.0 |########################################
  53501.2 |####################
  55916.4 |##########
  58331.6 |##########
  60746.8 |
  63162.0 |
  65577.2 |
  67992.4 |
  70407.6 |##########
  72822.8 |
  75238.0 |
  77653.3 |
  80068.5 |
  82483.7 |
  84898.9 |
  87314.1 |
  89729.3 |
  92144.5 |
  (2 below, 2 above range)

kernel_seq (n=18, range 52320.8-55002.8 ns)
  52320.8 |##########################
  52454.9 |
  52589.0 |##########################
  52723.1 |
  52857.2 |########################################
  52991.3 |#############
  53125.4 |
  53259.5 |#############
  53393.6 |#############
  53527.7 |#############
  53661.8 |
  53795.9 |#############
  53930.0 |
  54064.1 |#############
  54198.2 |
  54332.3 |
  54466.4 |
  54600.5 |
  54734.6 |#############
  54868.7 |
  (1 below, 3 above range)

nway (n=18, range 60626.4-68095.1 ns)
  60626.4 |
  60999.8 |
  61373.2 |########################################
  61746.7 |
  62120.1 |#############
  62493.6 |##########################
  62867.0 |#############
  63240.4 |#############
  63613.9 |
  63987.3 |##########################
  64360.8 |##########################
  64734.2 |
  65107.6 |
  65481.1 |
  65854.5 |
  66227.9 |#############
  66601.4 |
  66974.8 |
  67348.3 |
  67721.7 |##########################
  (1 below, 2 above range)

sequential (n=18, range 57082.0-58437.5 ns)
  57082.0 |########################################
  57149.7 |####################
  57217.5 |
  57285.3 |
  57353.1 |####################
  57420.8 |####################
  57488.6 |########################################
  57556.4 |####################
  57624.2 |####################
  57692.0 |####################
  57759.7 |####################
  57827.5 |####################
  57895.3 |
  57963.1 |
  58030.8 |
  58098.6 |########################################
  58166.4 |
  58234.2 |####################
  58301.9 |
  58369.7 |####################
  (1 below, 1 above range)

std_parallel (n=18, range 86623.6-99304.9 ns)
  86623.6 |
  87257.7 |
  87891.7 |########################################
  88525.8 |
  89159.9 |
  89793.9 |
  90428.0 |##########################
  91062.0 |
  91696.1 |########################################
  92330.2 |
  92964.2 |#############
  93598.3 |#############
  94232.4 |#############
  94866.4 |##########################
  95500.5 |
  96134.6 |
  96768.6 |
  97402.7 |
  98036.8 |
  98670.8 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=207.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=287.8% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=54.3% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=205.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=199.4% of algo (FFI overhead may distort results)
- **nway**: bridge=197.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=200.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=126.0% of algo (FFI overhead may distort results)
