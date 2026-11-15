# append, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 109% faster than the next best (kernel_seq)

sequential (11.15 us) leads kernel_seq (23.26 us) by 109%, a clear separation rather than a photo finish. CV 5.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 53% (significant)

sequential is -12.43 us (53%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 4.0x slower than the field

std_parallel (44.52 us) is 4.0x the fastest (11.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.56)

kernel_seq's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {kernel_seq, head_tail, kernel_nway, head_tail_dyn, nway, std_parallel} (109% apart)

The field splits into a fast tier {sequential} and a slow tier {kernel_seq, head_tail, kernel_nway, head_tail_dyn, nway, std_parallel} with a 109% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.0x the fastest

Fastest sequential (11.15 us) to slowest std_parallel (44.52 us): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.7x its best-20%

kernel_nway's best 20% of batches run at 24.46 us but its worst 20% at 41.79 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 11148.1 ns median (-52.4% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 3.99x (fastest 11148.1 ns, slowest 44524.9 ns)

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
| Counter frequency | 2100350026 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 25525ns | 23864ns | 21251ns | 23655ns | 30467ns | base |
| head_tail_dyn | 27760ns | 27495ns | 24496ns | 27130ns | 30337ns | +8.75% |
| kernel_nway | 31570ns | 26768ns | 25131ns | 26458ns | 42458ns | +23.68% |
| kernel_seq | 24560ns | 23977ns | 21893ns | 23554ns | 27403ns | -3.78% |
| nway | 40465ns | 38026ns | 32493ns | 37345ns | 49131ns | +58.53% |
| sequential | 11710ns | 11407ns | 11263ns | 11370ns | 12444ns | -54.12% |
| std_parallel | 45303ns | 45069ns | 35493ns | 42747ns | 54042ns | +77.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 24980ns | 20677ns | 29809ns | base | 0.164 |
| head_tail_dyn | 27059ns | 23936ns | 29478ns | +8.32% | 0.151 |
| kernel_nway | 30983ns | 24465ns | 41792ns | +24.03% | 0.132 |
| kernel_seq | 23823ns | 21459ns | 26312ns | -4.63% | 0.172 |
| nway | 40003ns | 32124ns | 48650ns | +60.14% | 0.102 |
| sequential | 11439ns | 11015ns | 12133ns | -54.21% | 0.358 |
| std_parallel | 44677ns | 35087ns | 53274ns | +78.85% | 0.092 |

## Performance model

- Peak throughput: **0.372 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.175 | 47.0% |
| head_tail_dyn | 0.153 | 41.1% |
| kernel_nway | 0.156 | 41.9% |
| kernel_seq | 0.176 | 47.4% |
| nway | 0.109 | 29.4% |
| sequential | 0.367 | 98.8% |
| std_parallel | 0.092 | 24.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 25525ns | 25525ns | base |
| head_tail_dyn | 27760ns | 27760ns | +8.75% |
| kernel_nway | 31570ns | 31570ns | +23.68% |
| kernel_seq | 24560ns | 24560ns | -3.78% |
| nway | 40465ns | 40465ns | +58.53% |
| sequential | 11710ns | 11710ns | -54.12% |
| std_parallel | 45303ns | 45303ns | +77.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 23436ns | base | --- | [22319, 26104] | --- | --- | --- | --- |
| head_tail_dyn | 26786ns | no significant difference | [-143, +5758]ns | [25859, 28365] | no | 0.1155 | 0.0963 | 0 |
| kernel_nway | 26292ns | no significant difference | [-162, +8854]ns | [25379, 30707] | no | 0.1155 | 0.0963 | 0 |
| kernel_seq | 23256ns | no significant difference | [-3019, +2140]ns | [22461, 24171] | no | 0.8145 | 0.8145 | 0 |
| nway | 37522ns | +12272.4ns (+52.4%) | [+9485, +22037]ns | [33291, 44739] | YES | 0.0003 | 0.0001 | 0 |
| sequential | 11148ns | -12430.6ns (-53.0%) | [-14092, -11072]ns | [11048, 11295] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 44525ns | +21387.6ns (+91.3%) | [+12755, +25194]ns | [38973, 47295] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 25250ns | +3.6% | +49.9% | -11.7% | +48.6% | -55.7% | +45.5% |
| 1 | 2 | 23860ns | +7.3% | +11.5% | +7.1% | +37.4% | -53.2% | +101.4% |
| 1 | 3 | 21527ns | +37.0% | +17.3% | +9.6% | +124.6% | -48.5% | +106.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.014 | ok |
| head_tail_dyn | -0.250 | moderate- |
| kernel_nway | -0.113 | ok |
| kernel_seq | -0.562 | HIGH- (thermal bounce) |
| nway | -0.360 | moderate- |
| sequential | -0.101 | ok |
| std_parallel | -0.366 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 47626.6ns | 24979.7ns | 190.7% | HIGH |
| head_tail_dyn | 50899.7ns | 27058.8ns | 188.1% | HIGH |
| kernel_nway | 53929.2ns | 30982.7ns | 174.1% | HIGH |
| kernel_seq | 46614.3ns | 23822.6ns | 195.7% | HIGH |
| nway | 72863.1ns | 40002.9ns | 182.1% | HIGH |
| sequential | 32344.8ns | 11438.7ns | 282.8% | HIGH |
| std_parallel | 64722.9ns | 44677.1ns | 144.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 20677.0-29809.5 ns)
  20677.0 |##########################
  21133.6 |
  21590.2 |#############
  22046.9 |#############
  22503.5 |########################################
  22960.1 |#############
  23416.7 |#############
  23873.4 |
  24330.0 |
  24786.6 |##########################
  25243.2 |#############
  25699.9 |
  26156.5 |
  26613.1 |#############
  27069.7 |
  27526.4 |
  27983.0 |
  28439.6 |
  28896.2 |##########################
  29352.9 |#############
  (1 below, 1 above range)

head_tail_dyn (n=18, range 23936.5-29477.6 ns)
  23936.5 |#############
  24213.6 |#############
  24490.6 |
  24767.7 |
  25044.7 |#############
  25321.8 |#############
  25598.8 |#############
  25875.9 |
  26152.9 |
  26430.0 |#############
  26707.1 |########################################
  26984.1 |#############
  27261.2 |
  27538.2 |
  27815.3 |
  28092.3 |##########################
  28369.4 |#############
  28646.4 |
  28923.5 |
  29200.6 |#############
  (1 below, 3 above range)

kernel_nway (n=18, range 24464.9-41792.5 ns)
  24464.9 |########################
  25331.2 |########################
  26197.6 |########################################
  27064.0 |
  27930.4 |
  28796.8 |
  29663.1 |
  30529.5 |
  31395.9 |
  32262.3 |
  33128.7 |
  33995.0 |########
  34861.4 |
  35727.8 |
  36594.2 |
  37460.6 |
  38326.9 |
  39193.3 |
  40059.7 |
  40926.1 |########
  (2 below, 3 above range)

kernel_seq (n=18, range 21459.4-26312.5 ns)
  21459.4 |####################
  21702.0 |####################
  21944.7 |
  22187.3 |########################################
  22430.0 |########################################
  22672.6 |####################
  22915.3 |####################
  23158.0 |####################
  23400.6 |
  23643.3 |########################################
  23885.9 |####################
  24128.6 |####################
  24371.2 |
  24613.9 |####################
  24856.6 |
  25099.2 |
  25341.9 |
  25584.5 |
  25827.2 |
  26069.8 |
  (1 below, 3 above range)

nway (n=18, range 32124.2-48650.2 ns)
  32124.2 |########################################
  32950.5 |########################################
  33776.8 |
  34603.1 |
  35429.4 |
  36255.7 |########################################
  37082.0 |########################################
  37908.3 |
  38734.6 |
  39560.9 |####################
  40387.2 |
  41213.5 |
  42039.8 |
  42866.1 |
  43692.4 |####################
  44518.7 |########################################
  45345.0 |
  46171.3 |
  46997.6 |####################
  47823.9 |
  (2 below, 3 above range)

sequential (n=18, range 11015.1-12132.9 ns)
  11015.1 |########################################
  11071.0 |##########
  11126.8 |##############################
  11182.7 |##############################
  11238.6 |
  11294.5 |
  11350.4 |##########
  11406.3 |
  11462.2 |
  11518.1 |
  11574.0 |
  11629.8 |
  11685.7 |
  11741.6 |
  11797.5 |
  11853.4 |
  11909.3 |
  11965.2 |
  12021.1 |
  12077.0 |
  (2 below, 4 above range)

std_parallel (n=18, range 35087.4-53273.7 ns)
  35087.4 |####################
  35996.7 |####################
  36906.1 |########################################
  37815.4 |
  38724.7 |
  39634.0 |
  40543.3 |####################
  41452.6 |########################################
  42361.9 |
  43271.2 |
  44180.6 |########################################
  45089.9 |########################################
  45999.2 |
  46908.5 |########################################
  47817.8 |####################
  48727.1 |####################
  49636.4 |
  50545.7 |####################
  51455.1 |
  52364.4 |
  (1 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=196.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=183.0% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=30.5% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=195.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=199.1% of algo (FFI overhead may distort results)
- **nway**: bridge=173.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=278.7% of algo (FFI overhead may distort results)
- **std_parallel**: CV=20.6% (high variance, measurements may be unstable)
- **std_parallel**: bridge=143.8% of algo (FFI overhead may distort results)
