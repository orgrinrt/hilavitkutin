# fold_heavy, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 52% faster than the next best (head_tail)

sequential (23.12 us) leads head_tail (35.14 us) by 52%, a clear separation rather than a photo finish. CV 5.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 34% (significant)

sequential is -11.94 us (34%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 4.0x slower than the field

std_parallel (92.83 us) is 4.0x the fastest (23.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.66)

head_tail's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.0x the fastest

Fastest sequential (23.12 us) to slowest std_parallel (92.83 us): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### nway is inconsistent: worst-20% is 8.9x its best-20%

nway's best 20% of batches run at 57.14 us but its worst 20% at 506.58 us (8.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 23116.2 ns median (-34.2% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 4.02x (fastest 23116.2 ns, slowest 92826.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100312120 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 38204ns | 35869ns | 32879ns | 35097ns | 45527ns | base |
| head_tail_dyn | 39497ns | 35533ns | 31731ns | 34676ns | 50611ns | +3.38% |
| kernel_nway | 48084ns | 48396ns | 41392ns | 47207ns | 52745ns | +25.86% |
| kernel_seq | 47004ns | 49636ns | 37275ns | 46712ns | 52307ns | +23.03% |
| nway | 212753ns | 70002ns | 57547ns | 68215ns | 507164ns | +456.88% |
| sequential | 23594ns | 23358ns | 22283ns | 22984ns | 25164ns | -38.24% |
| std_parallel | 104901ns | 93204ns | 81974ns | 90980ns | 137246ns | +174.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 37614ns | 32492ns | 44876ns | base | 0.109 |
| head_tail_dyn | 38984ns | 31271ns | 50003ns | +3.64% | 0.105 |
| kernel_nway | 47532ns | 41034ns | 52166ns | +26.37% | 0.086 |
| kernel_seq | 46485ns | 36871ns | 51794ns | +23.58% | 0.088 |
| nway | 212294ns | 57141ns | 506581ns | +464.40% | 0.019 |
| sequential | 23267ns | 22042ns | 24783ns | -38.14% | 0.176 |
| std_parallel | 104357ns | 81599ns | 136625ns | +177.44% | 0.039 |

## Performance model

- Peak throughput: **0.186 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.117 | 62.7% |
| head_tail_dyn | 0.117 | 62.7% |
| kernel_nway | 0.086 | 46.0% |
| kernel_seq | 0.083 | 44.8% |
| nway | 0.059 | 31.7% |
| sequential | 0.177 | 95.4% |
| std_parallel | 0.044 | 23.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 38204ns | 38204ns | base |
| head_tail_dyn | 39497ns | 39497ns | +3.38% |
| kernel_nway | 48084ns | 48084ns | +25.86% |
| kernel_seq | 47004ns | 47004ns | +23.03% |
| nway | 212753ns | 212753ns | +456.88% |
| sequential | 23594ns | 23594ns | -38.24% |
| std_parallel | 104901ns | 104901ns | +174.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 35137ns | base | --- | [33045, 38634] | --- | --- | --- | --- |
| head_tail_dyn | 35144ns | no significant difference | [-3974, +6298]ns | [32636, 39922] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 47874ns | +13652.9ns (+38.9%) | [+5369, +15732]ns | [44581, 50022] | YES | 0.0002 | 0.0001 | 0 |
| kernel_seq | 49212ns | +12477.5ns (+35.5%) | [+4010, +14676]ns | [41880, 50862] | YES | 0.0090 | 0.0075 | 0 |
| nway | 69639ns | +34009.4ns (+96.8%) | [+28334, +49086]ns | [64744, 83390] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 23116ns | -11940.8ns (-34.0%) | [-14749, -10500]ns | [22130, 23868] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 92827ns | +58556.4ns (+166.7%) | [+48887, +68401]ns | [85209, 109351] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 33468ns | -1.6% | +41.7% | +10.7% | +225.0% | -28.0% | +172.0% |
| 1 | 2 | 37406ns | -12.2% | +18.2% | +37.6% | +75.0% | -39.7% | +127.0% |
| 1 | 3 | 33745ns | +25.3% | +48.2% | +45.8% | +103.1% | -34.0% | +253.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.664 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.173 | ok |
| kernel_nway | -0.534 | HIGH- (thermal bounce) |
| kernel_seq | -0.257 | moderate- |
| nway | -0.205 | moderate- |
| sequential | -0.097 | ok |
| std_parallel | -0.272 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 36417.3ns | 37614.4ns | 96.8% | HIGH |
| head_tail_dyn | 36019.9ns | 38984.3ns | 92.4% | HIGH |
| kernel_nway | 49336.7ns | 47531.6ns | 103.8% | HIGH |
| kernel_seq | 48059.0ns | 46484.6ns | 103.4% | HIGH |
| nway | 90671.4ns | 212294.0ns | 42.7% | HIGH |
| sequential | 23860.8ns | 23266.7ns | 102.6% | HIGH |
| std_parallel | 96479.4ns | 104357.3ns | 92.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 32491.5-44876.5 ns)
  32491.5 |########################################
  33110.8 |#############
  33730.0 |#############
  34349.3 |##########################
  34968.5 |#############
  35587.8 |#############
  36207.0 |
  36826.3 |#############
  37445.5 |#############
  38064.8 |
  38684.0 |
  39303.2 |########################################
  39922.5 |
  40541.7 |
  41161.0 |
  41780.2 |
  42399.5 |
  43018.7 |
  43638.0 |
  44257.2 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 31270.9-50002.5 ns)
  31270.9 |
  32207.5 |########################################
  33144.1 |########
  34080.7 |########
  35017.3 |
  35953.8 |########################
  36890.4 |
  37827.0 |
  38763.6 |########
  39700.2 |########
  40636.7 |########
  41573.3 |
  42509.9 |########
  43446.5 |
  44383.1 |
  45319.6 |
  46256.2 |
  47192.8 |
  48129.4 |
  49066.0 |
  (2 below, 2 above range)

kernel_nway (n=18, range 41034.2-52166.4 ns)
  41034.2 |
  41590.8 |
  42147.4 |########################################
  42704.0 |####################
  43260.6 |
  43817.2 |
  44373.8 |
  44930.5 |
  45487.1 |
  46043.7 |########################################
  46600.3 |####################
  47156.9 |####################
  47713.5 |
  48270.1 |####################
  48826.8 |####################
  49383.4 |########################################
  49940.0 |####################
  50496.6 |
  51053.2 |
  51609.8 |####################
  (2 below, 3 above range)

kernel_seq (n=18, range 36870.9-51793.6 ns)
  36870.9 |##############################
  37617.0 |##########
  38363.1 |
  39109.3 |
  39855.4 |
  40601.5 |
  41347.7 |
  42093.8 |
  42839.9 |
  43586.1 |
  44332.2 |
  45078.4 |##########
  45824.5 |
  46570.6 |
  47316.8 |####################
  48062.9 |
  48809.0 |####################
  49555.2 |
  50301.3 |########################################
  51047.4 |
  (1 below, 4 above range)

nway (n=18, range 57141.4-506580.6 ns)
  57141.4 |########################################
  79613.4 |#######
  102085.3 |###
  124557.3 |
  147029.3 |
  169501.2 |
  191973.2 |
  214445.1 |###
  236917.1 |
  259389.0 |
  281861.0 |
  304333.0 |
  326804.9 |
  349276.9 |
  371748.8 |
  394220.8 |
  416692.7 |
  439164.7 |
  461636.7 |
  484108.6 |
  (2 below, 1 above range)

sequential (n=18, range 22041.9-24783.2 ns)
  22041.9 |########################################
  22178.9 |
  22316.0 |################
  22453.1 |
  22590.1 |
  22727.2 |
  22864.3 |
  23001.3 |########
  23138.4 |########
  23275.5 |################
  23412.5 |
  23549.6 |
  23686.6 |
  23823.7 |########
  23960.8 |########
  24097.8 |
  24234.9 |########
  24372.0 |
  24509.0 |########
  24646.1 |
  (1 below, 2 above range)

std_parallel (n=18, range 81598.5-136625.3 ns)
  81598.5 |########################################
  84349.8 |##########
  87101.2 |
  89852.5 |##############################
  92603.9 |##########
  95355.2 |##########
  98106.5 |####################
  100857.9 |
  103609.2 |
  106360.5 |
  109111.9 |
  111863.2 |
  114614.6 |
  117365.9 |##########
  120117.2 |##########
  122868.6 |
  125619.9 |
  128371.3 |
  131122.6 |##########
  133873.9 |##########
  (1 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=20.8% (high variance, measurements may be unstable)
- **head_tail**: bridge=101.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=27.8% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=95.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=103.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=101.3% of algo (FFI overhead may distort results)
- **nway**: CV=253.4% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 8.9x (possible bimodal distribution)
- **nway**: bridge=94.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=98.7% of algo (FFI overhead may distort results)
- **std_parallel**: CV=29.9% (high variance, measurements may be unstable)
- **std_parallel**: bridge=99.7% of algo (FFI overhead may distort results)
