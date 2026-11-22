# fold_light, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 29% faster than the next best (head_tail)

kernel_nway (55.83 us) leads head_tail (71.89 us) by 29%, a clear separation rather than a photo finish. CV 5.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway beats baseline by 20% (significant)

kernel_nway is -14.52 us (20%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### head_tail_dyn is an outlier: 2.5x slower than the field

head_tail_dyn (137.69 us) is 2.5x the fastest (55.83 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.65)

sequential's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: kernel_nway** at 55833.3 ns median (-22.3% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 2.47x (fastest 55833.3 ns, slowest 137685.4 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 72361ns | 72517ns | 68064ns | 72358ns | 74512ns | base |
| head_tail_dyn | 137668ns | 138096ns | 135769ns | 137575ns | 138756ns | +90.25% |
| kernel_nway | 56869ns | 56550ns | 53472ns | 55786ns | 60191ns | -21.41% |
| kernel_seq | 95285ns | 95002ns | 94051ns | 94823ns | 96594ns | +31.68% |
| nway | 75030ns | 74456ns | 72807ns | 74337ns | 77182ns | +3.69% |
| sequential | 101037ns | 100573ns | 99182ns | 100536ns | 102717ns | +39.63% |
| std_parallel | 79363ns | 79071ns | 77019ns | 78598ns | 81681ns | +9.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 71682ns | 67392ns | 73772ns | base | 14.628 |
| head_tail_dyn | 137286ns | 135397ns | 138415ns | +91.52% | 7.638 |
| kernel_nway | 56258ns | 52832ns | 59530ns | -21.52% | 18.639 |
| kernel_seq | 95041ns | 93865ns | 96272ns | +32.59% | 11.033 |
| nway | 74457ns | 72203ns | 76626ns | +3.87% | 14.083 |
| sequential | 100812ns | 98935ns | 102478ns | +40.64% | 10.401 |
| std_parallel | 78959ns | 76608ns | 81292ns | +10.15% | 13.280 |

## Performance model

- Peak throughput: **19.847 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 14.585 | 73.5% |
| head_tail_dyn | 7.616 | 38.4% |
| kernel_nway | 18.780 | 94.6% |
| kernel_seq | 11.069 | 55.8% |
| nway | 14.197 | 71.5% |
| sequential | 10.446 | 52.6% |
| std_parallel | 13.334 | 67.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 72361ns | 72361ns | base |
| head_tail_dyn | 137668ns | 137668ns | +90.25% |
| kernel_nway | 56869ns | 56869ns | -21.41% |
| kernel_seq | 95285ns | 95285ns | +31.68% |
| nway | 75030ns | 75030ns | +3.69% |
| sequential | 101037ns | 101037ns | +39.63% |
| std_parallel | 79363ns | 79363ns | +9.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 71892ns | base | --- | [71340, 73000] | --- | --- | --- | --- |
| head_tail_dyn | 137685ns | +65414.6ns (+91.0%) | [+64254, +66873]ns | [136550, 138254] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 55833ns | -14525.0ns (-20.2%) | [-17950, -13683]ns | [53840, 57583] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 94727ns | +23091.7ns (+32.1%) | [+21083, +23371]ns | [94319, 95596] | YES | 0.0000 | 0.0000 | 0 |
| nway | 73858ns | +2450.0ns (+3.4%) | [+269, +4529]ns | [73054, 75933] | YES (adj: no) | 0.0963 | 0.0963 | 0 |
| sequential | 100377ns | +28816.7ns (+40.1%) | [+27433, +30450]ns | [100056, 101696] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 78642ns | +6143.8ns (+8.5%) | [+5188, +8990]ns | [77752, 79856] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 72340ns | +91.0% | -21.6% | +30.4% | +2.7% | +38.6% | +9.6% |
| 1 | 2 | 71740ns | +92.2% | -20.8% | +33.6% | +4.4% | +42.7% | +8.7% |
| 1 | 3 | 71788ns | +90.2% | -22.9% | +32.6% | +2.5% | +40.1% | +10.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.210 | moderate- |
| head_tail_dyn | -0.074 | ok |
| kernel_nway | -0.206 | moderate- |
| kernel_seq | -0.457 | moderate- |
| nway | -0.485 | moderate- |
| sequential | -0.649 | HIGH- (thermal bounce) |
| std_parallel | -0.636 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 141113.9ns | 71681.7ns | 196.9% | HIGH |
| head_tail_dyn | 136872.2ns | 137285.9ns | 99.7% | HIGH |
| kernel_nway | 113534.7ns | 56258.1ns | 201.8% | HIGH |
| kernel_seq | 94985.2ns | 95041.4ns | 99.9% | HIGH |
| nway | 141532.6ns | 74456.9ns | 190.1% | HIGH |
| sequential | 101102.8ns | 100811.8ns | 100.3% | HIGH |
| std_parallel | 142972.5ns | 78959.0ns | 181.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 67391.7-73771.5 ns)
  67391.7 |##########
  67710.7 |
  68029.7 |
  68348.6 |
  68667.6 |
  68986.6 |##########
  69305.6 |
  69624.6 |
  69943.6 |
  70262.6 |
  70581.6 |
  70900.6 |##########
  71219.6 |########################################
  71538.6 |##########
  71857.6 |####################
  72176.6 |
  72495.5 |
  72814.5 |##############################
  73133.5 |
  73452.5 |##########
  (1 below, 3 above range)

head_tail_dyn (n=18, range 135397.2-138415.3 ns)
  135397.2 |
  135548.1 |#############
  135699.0 |#############
  135849.9 |
  136000.8 |#############
  136151.7 |
  136302.6 |
  136453.5 |#############
  136604.4 |##########################
  136755.3 |
  136906.2 |#############
  137057.1 |
  137208.0 |
  137358.9 |
  137509.8 |#############
  137660.8 |#############
  137811.7 |
  137962.6 |
  138113.5 |########################################
  138264.4 |########################################
  (1 below, 2 above range)

kernel_nway (n=18, range 52831.9-59529.9 ns)
  52831.9 |####################
  53166.8 |####################
  53501.7 |########################################
  53836.6 |####################
  54171.5 |
  54506.4 |####################
  54841.3 |
  55176.2 |########################################
  55511.1 |
  55846.0 |
  56180.9 |####################
  56515.8 |####################
  56850.7 |########################################
  57185.6 |
  57520.5 |
  57855.4 |####################
  58190.3 |
  58525.2 |####################
  58860.1 |
  59195.0 |
  (1 below, 3 above range)

kernel_seq (n=18, range 93865.3-96272.2 ns)
  93865.3 |
  93985.6 |#############
  94106.0 |#############
  94226.3 |########################################
  94346.7 |##########################
  94467.0 |
  94587.4 |#############
  94707.7 |#############
  94828.1 |
  94948.4 |
  95068.8 |#############
  95189.1 |#############
  95309.4 |
  95429.8 |#############
  95550.1 |#############
  95670.5 |
  95790.8 |
  95911.2 |#############
  96031.5 |
  96151.9 |#############
  (1 below, 2 above range)

nway (n=18, range 72202.8-76626.4 ns)
  72202.8 |
  72423.9 |#############
  72645.1 |#############
  72866.3 |##########################
  73087.5 |#############
  73308.7 |#############
  73529.9 |
  73751.0 |########################################
  73972.2 |
  74193.4 |
  74414.6 |
  74635.8 |#############
  74856.9 |
  75078.1 |#############
  75299.3 |
  75520.5 |
  75741.7 |#############
  75962.9 |########################################
  76184.0 |
  76405.2 |
  (1 below, 2 above range)

sequential (n=18, range 98934.7-102478.5 ns)
  98934.7 |#############
  99111.9 |
  99289.1 |
  99466.3 |
  99643.5 |#############
  99820.7 |#############
  99997.9 |########################################
  100175.0 |#############
  100352.2 |##########################
  100529.4 |#############
  100706.6 |
  100883.8 |
  101061.0 |#############
  101238.2 |
  101415.3 |
  101592.5 |#############
  101769.7 |#############
  101946.9 |#############
  102124.1 |
  102301.3 |
  (1 below, 3 above range)

std_parallel (n=18, range 76608.3-81291.7 ns)
  76608.3 |####################
  76842.5 |####################
  77076.7 |
  77310.8 |
  77545.0 |####################
  77779.2 |########################################
  78013.3 |####################
  78247.5 |####################
  78481.7 |
  78715.8 |####################
  78950.0 |########################################
  79184.2 |
  79418.3 |####################
  79652.5 |
  79886.7 |####################
  80120.8 |
  80355.0 |####################
  80589.2 |####################
  80823.3 |
  81057.5 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=199.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=203.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.2% of algo (FFI overhead may distort results)
- **nway**: bridge=192.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=183.4% of algo (FFI overhead may distort results)
