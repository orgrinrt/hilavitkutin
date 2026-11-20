# fold_light, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 3.4x slower than the field

std_parallel (50.27 us) is 3.4x the fastest (14.75 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.66)

sequential's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.4x the fastest

Fastest head_tail_dyn (14.75 us) to slowest std_parallel (50.27 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader head_tail_dyn vs stability leader head_tail (+4% speed for 2.2x steadier)

head_tail_dyn is fastest (14.75 us, CV 18.0%); head_tail gives up 4.4% median for 2.2x lower variance (CV 8.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### head_tail_dyn is inconsistent: worst-20% is 1.5x its best-20%

head_tail_dyn's best 20% of batches run at 11.45 us but its worst 20% at 17.50 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 14748.1 ns median (-4.2% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 3.41x (fastest 14748.1 ns, slowest 50267.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 2 |
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
| head_tail | 16007ns | 16043ns | 13869ns | 15621ns | 17654ns | base |
| head_tail_dyn | 15503ns | 15364ns | 11962ns | 14734ns | 18427ns | -3.15% |
| kernel_nway | 16032ns | 15804ns | 13144ns | 15597ns | 18127ns | +0.16% |
| kernel_seq | 17236ns | 17295ns | 14580ns | 16795ns | 19225ns | +7.68% |
| nway | 53992ns | 50795ns | 43517ns | 49229ns | 66375ns | +237.31% |
| sequential | 26255ns | 25564ns | 23784ns | 25318ns | 28896ns | +64.03% |
| std_parallel | 50683ns | 50803ns | 44578ns | 49352ns | 55733ns | +216.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 15028ns | 13295ns | 16343ns | base | 0.273 |
| head_tail_dyn | 14688ns | 11446ns | 17500ns | -2.26% | 0.279 |
| kernel_nway | 15016ns | 12441ns | 16817ns | -0.08% | 0.273 |
| kernel_seq | 16140ns | 13850ns | 17676ns | +7.40% | 0.254 |
| nway | 53213ns | 42956ns | 65339ns | +254.10% | 0.077 |
| sequential | 25746ns | 23318ns | 28319ns | +71.32% | 0.159 |
| std_parallel | 49974ns | 44111ns | 54646ns | +232.55% | 0.082 |

## Performance model

- Peak throughput: **0.358 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.266 | 74.4% |
| head_tail_dyn | 0.278 | 77.6% |
| kernel_nway | 0.273 | 76.4% |
| kernel_seq | 0.247 | 68.9% |
| nway | 0.082 | 22.9% |
| sequential | 0.163 | 45.6% |
| std_parallel | 0.081 | 22.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 16007ns | 16007ns | base |
| head_tail_dyn | 15503ns | 15503ns | -3.15% |
| kernel_nway | 16032ns | 16032ns | +0.16% |
| kernel_seq | 17236ns | 17236ns | +7.68% |
| nway | 53992ns | 53992ns | +237.31% |
| sequential | 26255ns | 26255ns | +64.03% |
| std_parallel | 50683ns | 50683ns | +216.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 15391ns | base | --- | [13688, 15845] | --- | --- | --- | --- |
| head_tail_dyn | 14748ns | no significant difference | [-1764, +403]ns | [12060, 16252] | no | 0.9774 | 0.8145 | 0 |
| kernel_nway | 14987ns | no significant difference | [-1733, +1264]ns | [14651, 15547] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 16606ns | no significant difference | [-1180, +2954]ns | [14903, 17067] | no | 0.3568 | 0.2379 | 0 |
| nway | 50087ns | +33376.4ns (+216.9%) | [+31703, +40438]ns | [45733, 56190] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 25075ns | +10034.4ns (+65.2%) | [+9666, +11761]ns | [24320, 26986] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 50268ns | +34818.1ns (+226.2%) | [+32065, +36972]ns | [46562, 52588] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 15955ns | +3.3% | -21.5% | -12.8% | +273.1% | +58.7% | +215.1% |
| 1 | 2 | 14821ns | -0.5% | +5.5% | +12.7% | +212.4% | +67.1% | +226.8% |
| 1 | 3 | 13688ns | -15.4% | +9.2% | +24.7% | +248.0% | +84.5% | +280.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.000 | ok |
| head_tail_dyn | -0.019 | ok |
| kernel_nway | -0.301 | moderate- |
| kernel_seq | -0.111 | ok |
| nway | -0.222 | moderate- |
| sequential | -0.659 | HIGH- (thermal bounce) |
| std_parallel | -0.505 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 15936.0ns | 15027.8ns | 106.0% | HIGH |
| head_tail_dyn | 15490.9ns | 14687.6ns | 105.5% | HIGH |
| kernel_nway | 16615.0ns | 15015.9ns | 110.6% | HIGH |
| kernel_seq | 16652.8ns | 16140.0ns | 103.2% | HIGH |
| nway | 54386.1ns | 53213.1ns | 102.2% | HIGH |
| sequential | 26483.3ns | 25745.6ns | 102.9% | HIGH |
| std_parallel | 53956.2ns | 49974.4ns | 108.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 13294.6-16342.6 ns)
  13294.6 |
  13447.0 |########################################
  13599.4 |########################################
  13751.8 |
  13904.2 |
  14056.6 |
  14209.0 |
  14361.4 |
  14513.8 |####################
  14666.2 |
  14818.6 |
  14971.0 |####################
  15123.4 |####################
  15275.8 |
  15428.2 |########################################
  15580.6 |####################
  15733.0 |####################
  15885.4 |########################################
  16037.8 |
  16190.2 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 11446.3-17499.5 ns)
  11446.3 |########################################
  11748.9 |
  12051.6 |#############
  12354.3 |
  12656.9 |
  12959.6 |
  13262.2 |
  13564.9 |
  13867.6 |
  14170.2 |
  14472.9 |########################################
  14775.5 |##########################
  15078.2 |
  15380.9 |
  15683.5 |
  15986.2 |########################################
  16288.9 |
  16591.5 |#############
  16894.2 |#############
  17196.8 |#############
  (2 below, 1 above range)

kernel_nway (n=18, range 12441.4-16816.9 ns)
  12441.4 |##############################
  12660.2 |
  12878.9 |
  13097.7 |
  13316.5 |
  13535.3 |
  13754.0 |
  13972.8 |
  14191.6 |
  14410.4 |##########
  14629.1 |####################
  14847.9 |##############################
  15066.7 |####################
  15285.4 |
  15504.2 |########################################
  15723.0 |##########
  15941.8 |
  16160.5 |
  16379.3 |
  16598.1 |
  (1 below, 1 above range)

kernel_seq (n=18, range 13849.8-17675.7 ns)
  13849.8 |##########################
  14041.1 |
  14232.4 |
  14423.7 |#############
  14615.0 |
  14806.3 |
  14997.6 |
  15188.9 |#############
  15380.2 |
  15571.5 |
  15762.8 |
  15954.1 |#############
  16145.4 |
  16336.6 |#############
  16527.9 |########################################
  16719.2 |#############
  16910.5 |##########################
  17101.8 |
  17293.1 |#############
  17484.4 |#############
  (2 below, 2 above range)

nway (n=18, range 42956.0-65339.2 ns)
  42956.0 |#############
  44075.2 |##########################
  45194.3 |##########################
  46313.5 |#############
  47432.7 |#############
  48551.8 |
  49671.0 |########################################
  50790.1 |
  51909.3 |
  53028.4 |
  54147.6 |
  55266.8 |########################################
  56385.9 |
  57505.1 |
  58624.2 |
  59743.4 |
  60862.5 |
  61981.7 |#############
  63100.9 |
  64220.0 |#############
  (1 below, 2 above range)

sequential (n=18, range 23318.3-28319.5 ns)
  23318.3 |########################################
  23568.3 |
  23818.4 |
  24068.4 |########################################
  24318.5 |########################################
  24568.6 |####################
  24818.6 |####################
  25068.7 |####################
  25318.7 |
  25568.8 |########################################
  25818.9 |
  26068.9 |
  26319.0 |
  26569.0 |####################
  26819.1 |
  27069.2 |
  27319.2 |
  27569.3 |####################
  27819.3 |
  28069.4 |########################################
  (1 below, 2 above range)

std_parallel (n=18, range 44111.4-54646.5 ns)
  44111.4 |
  44638.2 |
  45164.9 |#############
  45691.7 |#############
  46218.4 |#############
  46745.2 |#############
  47271.9 |
  47798.7 |
  48325.4 |#############
  48852.2 |
  49378.9 |#############
  49905.7 |#############
  50432.4 |########################################
  50959.2 |
  51485.9 |
  52012.7 |#############
  52539.4 |#############
  53066.2 |
  53592.9 |
  54119.7 |#############
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=101.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=103.3% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=104.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=101.7% of algo (FFI overhead may distort results)
- **nway**: bridge=102.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=103.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=108.0% of algo (FFI overhead may distort results)
