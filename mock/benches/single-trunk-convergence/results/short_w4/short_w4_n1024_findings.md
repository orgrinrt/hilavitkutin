# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 119% faster than the next best (kernel_seq)

sequential (9.62 us) leads kernel_seq (21.10 us) by 119%, a clear separation rather than a photo finish. CV 16.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 64% (significant)

sequential is -17.50 us (64%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 10.2x slower than the field

std_parallel (97.93 us) is 10.2x the fastest (9.62 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.66)

nway's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {kernel_seq, head_tail, head_tail_dyn, kernel_nway, nway, std_parallel} (119% apart)

The field splits into a fast tier {sequential} and a slow tier {kernel_seq, head_tail, head_tail_dyn, kernel_nway, nway, std_parallel} with a 119% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 10.2x the fastest

Fastest sequential (9.62 us) to slowest std_parallel (97.93 us): 10.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail is inconsistent: worst-20% is 1.7x its best-20%

head_tail's best 20% of batches run at 24.69 us but its worst 20% at 42.47 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 9617.6 ns median (-64.7% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 10.18x (fastest 9617.6 ns, slowest 97929.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 4 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100312120 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 32109ns | 27804ns | 25108ns | 27273ns | 42865ns | base |
| head_tail_dyn | 29717ns | 28201ns | 25901ns | 27854ns | 34421ns | -7.45% |
| kernel_nway | 44001ns | 42271ns | 38411ns | 41482ns | 50573ns | +37.03% |
| kernel_seq | 22921ns | 21537ns | 20128ns | 21607ns | 26287ns | -28.62% |
| nway | 63723ns | 62640ns | 49051ns | 59393ns | 77555ns | +98.46% |
| sequential | 9593ns | 9930ns | 7362ns | 9182ns | 11326ns | -70.12% |
| std_parallel | 99362ns | 98345ns | 82556ns | 95518ns | 113532ns | +209.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 31657ns | 24689ns | 42468ns | base | 0.032 |
| head_tail_dyn | 29278ns | 25521ns | 33949ns | -7.51% | 0.035 |
| kernel_nway | 43534ns | 38020ns | 50083ns | +37.52% | 0.024 |
| kernel_seq | 22495ns | 19745ns | 25885ns | -28.94% | 0.046 |
| nway | 63194ns | 48702ns | 76869ns | +99.62% | 0.016 |
| sequential | 9303ns | 7144ns | 10998ns | -70.61% | 0.110 |
| std_parallel | 98839ns | 82144ns | 112985ns | +212.22% | 0.010 |

## Performance model

- Peak throughput: **0.143 Gops/s** (sequential; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.038 | 26.2% |
| head_tail_dyn | 0.037 | 25.7% |
| kernel_nway | 0.025 | 17.1% |
| kernel_seq | 0.049 | 33.9% |
| nway | 0.016 | 11.5% |
| sequential | 0.106 | 74.3% |
| std_parallel | 0.010 | 7.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 32109ns | 32109ns | base |
| head_tail_dyn | 29717ns | 29717ns | -7.45% |
| kernel_nway | 44001ns | 44001ns | +37.03% |
| kernel_seq | 22921ns | 22921ns | -28.62% |
| nway | 63723ns | 63723ns | +98.46% |
| sequential | 9593ns | 9593ns | -70.12% |
| std_parallel | 99362ns | 99362ns | +209.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 27256ns | base | --- | [26270, 29209] | --- | --- | --- | --- |
| head_tail_dyn | 27817ns | no significant difference | [-884, +3420]ns | [26290, 32525] | no | 0.2379 | 0.2379 | 0 |
| kernel_nway | 41684ns | +14407.8ns (+52.9%) | [+10651, +17742]ns | [39543, 44207] | YES | 0.0002 | 0.0001 | 0 |
| kernel_seq | 21096ns | -5222.4ns (-19.2%) | [-5734, -3958]ns | [20663, 25288] | YES | 0.0000 | 0.0000 | 0 |
| nway | 62162ns | +33467.2ns (+122.8%) | [+22679, +41637]ns | [51700, 69923] | YES | 0.0002 | 0.0001 | 0 |
| sequential | 9618ns | -17497.2ns (-64.2%) | [-19496, -16906]ns | [7796, 10228] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 97930ns | +68371.9ns (+250.9%) | [+64388, +80020]ns | [90641, 107384] | YES | 0.0002 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 27031ns | -4.3% | +67.0% | -22.5% | +158.3% | -63.5% | +221.3% |
| 1 | 2 | 28742ns | +16.2% | +39.4% | -10.4% | +72.5% | -64.1% | +283.8% |
| 1 | 3 | 24882ns | +10.8% | +66.4% | -18.7% | +170.3% | -71.2% | +305.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.461 | moderate- |
| head_tail_dyn | -0.635 | HIGH- (thermal bounce) |
| kernel_nway | -0.330 | moderate- |
| kernel_seq | -0.657 | HIGH- (thermal bounce) |
| nway | -0.658 | HIGH- (thermal bounce) |
| sequential | -0.249 | moderate- |
| std_parallel | -0.429 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 28186.4ns | 31656.8ns | 89.0% | HIGH |
| head_tail_dyn | 30082.2ns | 29278.2ns | 102.7% | HIGH |
| kernel_nway | 50519.0ns | 43533.7ns | 116.0% | HIGH |
| kernel_seq | 23175.9ns | 22495.3ns | 103.0% | HIGH |
| nway | 78561.4ns | 63194.2ns | 124.3% | HIGH |
| sequential | 9589.1ns | 9303.2ns | 103.1% | HIGH |
| std_parallel | 99073.0ns | 98838.8ns | 100.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 24688.6-42468.0 ns)
  24688.6 |########################################
  25577.6 |##########################
  26466.6 |########################################
  27355.5 |########################################
  28244.5 |#############
  29133.5 |#############
  30022.5 |##########################
  30911.4 |
  31800.4 |
  32689.4 |
  33578.3 |
  34467.3 |
  35356.3 |#############
  36245.3 |
  37134.2 |
  38023.2 |
  38912.2 |
  39801.1 |
  40690.1 |
  41579.1 |
  (1 below, 1 above range)

head_tail_dyn (n=18, range 25521.3-33948.6 ns)
  25521.3 |##########################
  25942.7 |#############
  26364.1 |##########################
  26785.4 |#############
  27206.8 |#############
  27628.1 |
  28049.5 |#############
  28470.9 |
  28892.2 |#############
  29313.6 |#############
  29735.0 |
  30156.3 |
  30577.7 |
  30999.1 |
  31420.4 |
  31841.8 |
  32263.1 |########################################
  32684.5 |
  33105.9 |
  33527.2 |
  (2 below, 3 above range)

kernel_nway (n=18, range 38019.7-50082.8 ns)
  38019.7 |##########################
  38622.9 |
  39226.0 |########################################
  39829.2 |
  40432.3 |##########################
  41035.5 |#############
  41638.6 |#############
  42241.8 |
  42844.9 |##########################
  43448.1 |#############
  44051.2 |#############
  44654.4 |
  45257.5 |#############
  45860.7 |
  46463.8 |
  47067.0 |#############
  47670.2 |
  48273.3 |
  48876.5 |
  49479.6 |
  (1 below, 2 above range)

kernel_seq (n=18, range 19744.5-25884.7 ns)
  19744.5 |#############
  20051.5 |##########################
  20358.6 |#############
  20665.6 |########################################
  20972.6 |########################################
  21279.6 |
  21586.6 |
  21893.6 |
  22200.6 |
  22507.6 |
  22814.6 |
  23121.6 |
  23428.6 |#############
  23735.7 |
  24042.7 |
  24349.7 |
  24656.7 |
  24963.7 |#############
  25270.7 |#############
  25577.7 |##########################
  (1 below, 2 above range)

nway (n=18, range 48702.4-76868.8 ns)
  48702.4 |########################################
  50110.8 |####################
  51519.1 |####################
  52927.4 |
  54335.7 |
  55744.0 |
  57152.4 |
  58560.7 |########################################
  59969.0 |####################
  61377.3 |
  62785.6 |####################
  64194.0 |####################
  65602.3 |
  67010.6 |
  68418.9 |########################################
  69827.2 |####################
  71235.6 |
  72643.9 |
  74052.2 |
  75460.5 |####################
  (2 below, 3 above range)

sequential (n=18, range 7143.8-10998.3 ns)
   7143.8 |##########################
   7336.5 |
   7529.3 |
   7722.0 |##########################
   7914.7 |
   8107.4 |
   8300.2 |
   8492.9 |
   8685.6 |
   8878.3 |#############
   9071.1 |#############
   9263.8 |
   9456.5 |#############
   9649.2 |#############
   9842.0 |########################################
  10034.7 |
  10227.4 |
  10420.2 |#############
  10612.9 |#############
  10805.6 |#############
  (2 below, 2 above range)

std_parallel (n=18, range 82143.9-112984.5 ns)
  82143.9 |
  83685.9 |
  85228.0 |####################
  86770.0 |####################
  88312.0 |
  89854.1 |########################################
  91396.1 |####################
  92938.1 |
  94480.2 |####################
  96022.2 |
  97564.2 |########################################
  99106.2 |####################
  100648.3 |####################
  102190.3 |
  103732.3 |
  105274.4 |####################
  106816.4 |
  108358.4 |####################
  109900.5 |
  111442.5 |
  (2 below, 4 above range)

```

## Diagnostics

- **head_tail**: CV=52.4% (high variance, measurements may be unstable)
- **head_tail**: bridge=102.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=106.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=99.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=110.1% of algo (FFI overhead may distort results)
- **nway**: bridge=95.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=104.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=96.8% of algo (FFI overhead may distort results)
