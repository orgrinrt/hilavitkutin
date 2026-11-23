# fold_light, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 15% faster than the next best (head_tail)

kernel_seq (12.13 us) leads head_tail (13.98 us) by 15%, a clear separation rather than a photo finish. CV 37.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 7.3x slower than the field

std_parallel (88.05 us) is 7.3x the fastest (12.13 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq is fastest but the noisiest (CV 37.9%)

kernel_seq wins on median (12.13 us) yet has the highest variance (CV 37.9%), while kernel_nway is the steadiest (CV 8.5%, 15.61 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 7.3x the fastest

Fastest kernel_seq (12.13 us) to slowest std_parallel (88.05 us): 7.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail_dyn is inconsistent: worst-20% is 2.1x its best-20%

head_tail_dyn's best 20% of batches run at 11.99 us but its worst 20% at 24.67 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 12130.2 ns median (-13.2% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 7.26x (fastest 12130.2 ns, slowest 88048.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112780093 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 16721ns | 14997ns | 13682ns | 14776ns | 21159ns | base |
| head_tail_dyn | 18506ns | 16364ns | 12578ns | 15478ns | 26013ns | +10.67% |
| kernel_nway | 16276ns | 16576ns | 14230ns | 15867ns | 17914ns | -2.66% |
| kernel_seq | 15856ns | 12784ns | 12524ns | 13404ns | 21200ns | -5.17% |
| nway | 49100ns | 46293ns | 40113ns | 45961ns | 58304ns | +193.64% |
| sequential | 23788ns | 23658ns | 20846ns | 23170ns | 26188ns | +42.27% |
| std_parallel | 90786ns | 89157ns | 77435ns | 87869ns | 101836ns | +442.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 15773ns | 13107ns | 20042ns | base | 0.260 |
| head_tail_dyn | 17539ns | 11991ns | 24674ns | +11.20% | 0.234 |
| kernel_nway | 15244ns | 13566ns | 16615ns | -3.35% | 0.269 |
| kernel_seq | 14640ns | 11886ns | 19352ns | -7.18% | 0.280 |
| nway | 48321ns | 39591ns | 57323ns | +206.36% | 0.085 |
| sequential | 23294ns | 20422ns | 25586ns | +47.69% | 0.176 |
| std_parallel | 89992ns | 76907ns | 100762ns | +470.56% | 0.046 |

## Performance model

- Peak throughput: **0.345 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.293 | 85.0% |
| head_tail_dyn | 0.264 | 76.6% |
| kernel_nway | 0.262 | 76.1% |
| kernel_seq | 0.338 | 98.0% |
| nway | 0.090 | 26.1% |
| sequential | 0.176 | 51.2% |
| std_parallel | 0.047 | 13.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 16721ns | 16721ns | base |
| head_tail_dyn | 18506ns | 18506ns | +10.67% |
| kernel_nway | 16276ns | 16276ns | -2.66% |
| kernel_seq | 15856ns | 15856ns | -5.17% |
| nway | 49100ns | 49100ns | +193.64% |
| sequential | 23788ns | 23788ns | +42.27% |
| std_parallel | 90786ns | 90786ns | +442.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 13980ns | base | --- | [13316, 19192] | --- | --- | --- | --- |
| head_tail_dyn | 15513ns | no significant difference | [-1255, +4162]ns | [12836, 23821] | no | 0.2379 | 0.2379 | 0 |
| kernel_nway | 15608ns | no significant difference | [-4449, +2700]ns | [13833, 16064] | no | 0.2379 | 0.2379 | 0 |
| kernel_seq | 12130ns | -1532.1ns (-11.0%) | [-2114, -1099]ns | [11934, 16228] | YES | 0.0020 | 0.0013 | 0 |
| nway | 45619ns | +30508.4ns (+218.2%) | [+25828, +36014]ns | [43766, 50036] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 23209ns | +8640.3ns (+61.8%) | [+4917, +9419]ns | [21931, 24297] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 88048ns | +73615.9ns (+526.6%) | [+69652, +79785]ns | [84757, 95531] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 20409ns | +21.2% | -33.0% | -18.3% | +118.5% | +15.2% | +390.7% |
| 1 | 2 | 13157ns | +17.9% | +23.2% | -8.4% | +220.7% | +76.9% | +531.2% |
| 1 | 3 | 13980ns | -12.0% | +12.0% | -14.8% | +259.1% | +64.4% | +528.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.230 | moderate- |
| head_tail_dyn | -0.048 | ok |
| kernel_nway | -0.296 | moderate- |
| kernel_seq | -0.152 | ok |
| nway | -0.357 | moderate- |
| sequential | -0.002 | ok |
| std_parallel | -0.342 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 16738.7ns | 15772.6ns | 106.1% | HIGH |
| head_tail_dyn | 18452.1ns | 17538.6ns | 105.2% | HIGH |
| kernel_nway | 16139.0ns | 15244.2ns | 105.9% | HIGH |
| kernel_seq | 14483.6ns | 14639.6ns | 98.9% | HIGH |
| nway | 49808.2ns | 48320.5ns | 103.1% | HIGH |
| sequential | 24028.9ns | 23294.4ns | 103.2% | HIGH |
| std_parallel | 88753.5ns | 89992.0ns | 98.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 13107.2-20042.1 ns)
  13107.2 |########################################
  13453.9 |
  13800.7 |########################################
  14147.4 |
  14494.2 |########
  14840.9 |
  15187.7 |
  15534.4 |
  15881.2 |
  16227.9 |
  16574.7 |
  16921.4 |
  17268.1 |
  17614.9 |
  17961.6 |########
  18308.4 |
  18655.1 |
  19001.9 |
  19348.6 |
  19695.4 |
  (1 below, 5 above range)

head_tail_dyn (n=18, range 11991.3-24674.1 ns)
  11991.3 |##############################
  12625.5 |##########
  13259.6 |
  13893.8 |
  14527.9 |
  15162.0 |########################################
  15796.2 |####################
  16430.3 |
  17064.5 |
  17698.6 |
  18332.7 |
  18966.9 |
  19601.0 |
  20235.2 |
  20869.3 |
  21503.4 |
  22137.6 |
  22771.7 |##########
  23405.9 |
  24040.0 |##########
  (2 below, 4 above range)

kernel_nway (n=18, range 13566.1-16614.7 ns)
  13566.1 |########################################
  13718.6 |#############
  13871.0 |#############
  14023.4 |
  14175.9 |#############
  14328.3 |
  14480.7 |
  14633.1 |
  14785.6 |
  14938.0 |
  15090.4 |
  15242.9 |
  15395.3 |##########################
  15547.7 |
  15700.2 |##########################
  15852.6 |#############
  16005.0 |########################################
  16157.4 |
  16309.9 |#############
  16462.3 |
  (1 below, 2 above range)

kernel_seq (n=18, range 11885.6-19351.6 ns)
  11885.6 |########################################
  12258.9 |####
  12632.2 |
  13005.5 |
  13378.8 |
  13752.1 |
  14125.4 |
  14498.7 |
  14872.0 |
  15245.3 |####
  15618.6 |
  15991.9 |########
  16365.2 |########
  16738.5 |
  17111.8 |
  17485.1 |
  17858.4 |
  18231.7 |
  18605.0 |
  18978.3 |####
  (1 below, 1 above range)

nway (n=18, range 39590.7-57323.3 ns)
  39590.7 |#############
  40477.4 |
  41364.0 |
  42250.6 |
  43137.3 |########################################
  44023.9 |########################################
  44910.5 |
  45797.1 |#############
  46683.8 |
  47570.4 |##########################
  48457.0 |
  49343.7 |#############
  50230.3 |#############
  51116.9 |#############
  52003.6 |
  52890.2 |
  53776.8 |#############
  54663.5 |
  55550.1 |
  56436.7 |
  (2 below, 2 above range)

sequential (n=18, range 20421.6-25585.9 ns)
  20421.6 |
  20679.8 |
  20938.0 |
  21196.2 |
  21454.5 |####################
  21712.7 |########################################
  21970.9 |########################################
  22229.1 |####################
  22487.3 |
  22745.5 |####################
  23003.8 |
  23262.0 |####################
  23520.2 |####################
  23778.4 |
  24036.6 |########################################
  24294.8 |####################
  24553.1 |
  24811.3 |
  25069.5 |
  25327.7 |
  (2 below, 4 above range)

std_parallel (n=18, range 76906.5-100761.6 ns)
  76906.5 |####################
  78099.3 |####################
  79292.0 |####################
  80484.8 |
  81677.5 |
  82870.3 |####################
  84063.0 |
  85255.8 |########################################
  86448.5 |########################################
  87641.3 |
  88834.0 |####################
  90026.8 |
  91219.6 |####################
  92412.3 |
  93605.1 |####################
  94797.8 |########################################
  95990.6 |
  97183.3 |####################
  98376.1 |
  99568.8 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=104.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=30.0% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=103.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=104.0% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=31.4% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=108.0% of algo (FFI overhead may distort results)
- **nway**: bridge=100.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=102.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=99.2% of algo (FFI overhead may distort results)
