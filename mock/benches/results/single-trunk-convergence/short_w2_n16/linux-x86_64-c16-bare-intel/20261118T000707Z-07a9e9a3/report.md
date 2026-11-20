# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 4.7x slower than the field

std_parallel (48.16 us) is 4.7x the fastest (10.33 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.66)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, head_tail_dyn, head_tail, kernel_nway, sequential} vs {nway, std_parallel} (161% apart)

The field splits into a fast tier {kernel_seq, head_tail_dyn, head_tail, kernel_nway, sequential} and a slow tier {nway, std_parallel} with a 161% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.7x the fastest

Fastest kernel_seq (10.33 us) to slowest std_parallel (48.16 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.3x its best-20%

sequential's best 20% of batches run at 6.72 us but its worst 20% at 22.27 us (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 10330.6 ns median (-15.6% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 4.66x (fastest 10330.6 ns, slowest 48155.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 2 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 13109ns | 12860ns | 11642ns | 12732ns | 14408ns | base |
| head_tail_dyn | 13256ns | 11426ns | 10171ns | 11495ns | 17441ns | +1.12% |
| kernel_nway | 14073ns | 13277ns | 10833ns | 12902ns | 17449ns | +7.36% |
| kernel_seq | 13574ns | 11244ns | 8484ns | 11438ns | 19322ns | +3.55% |
| nway | 38345ns | 37714ns | 31047ns | 36771ns | 44355ns | +192.51% |
| sequential | 14675ns | 14565ns | 6852ns | 11911ns | 22734ns | +11.95% |
| std_parallel | 48218ns | 48696ns | 37286ns | 46969ns | 55556ns | +267.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 12366ns | 11028ns | 13507ns | base | 0.001 |
| head_tail_dyn | 12495ns | 9516ns | 16497ns | +1.05% | 0.001 |
| kernel_nway | 13322ns | 10342ns | 16621ns | +7.73% | 0.001 |
| kernel_seq | 12650ns | 7849ns | 18081ns | +2.30% | 0.001 |
| nway | 37785ns | 30630ns | 43628ns | +205.57% | 0.000 |
| sequential | 14379ns | 6718ns | 22266ns | +16.28% | 0.001 |
| std_parallel | 47595ns | 36906ns | 54745ns | +284.90% | 0.000 |

## Performance model

- Peak throughput: **0.002 Gops/s** (sequential; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.001 | 54.9% |
| head_tail_dyn | 0.001 | 62.2% |
| kernel_nway | 0.001 | 53.9% |
| kernel_seq | 0.002 | 65.0% |
| nway | 0.000 | 18.0% |
| sequential | 0.001 | 47.1% |
| std_parallel | 0.000 | 14.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 13109ns | 13109ns | base |
| head_tail_dyn | 13256ns | 13256ns | +1.12% |
| kernel_nway | 14073ns | 14073ns | +7.36% |
| kernel_seq | 13574ns | 13574ns | +3.55% |
| nway | 38345ns | 38345ns | +192.51% |
| sequential | 14675ns | 14675ns | +11.95% |
| std_parallel | 48218ns | 48218ns | +267.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 12238ns | base | --- | [11603, 13115] | --- | --- | --- | --- |
| head_tail_dyn | 10794ns | no significant difference | [-1094, +2567]ns | [10295, 15639] | no | 0.4758 | 0.2379 | 0 |
| kernel_nway | 12473ns | no significant difference | [-588, +1131]ns | [11674, 13932] | no | 0.9774 | 0.8145 | 0 |
| kernel_seq | 10331ns | no significant difference | [-3195, +3079]ns | [9598, 15550] | no | 0.9774 | 0.8145 | 0 |
| nway | 37227ns | +24911.3ns (+203.6%) | [+23176, +26567]ns | [34791, 40023] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 14267ns | no significant difference | [-4294, +7897]ns | [6777, 20983] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 48156ns | +35554.8ns (+290.5%) | [+32745, +38426]ns | [44568, 51102] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 11465ns | -8.2% | -5.5% | -9.9% | +180.8% | +31.2% | +288.7% |
| 1 | 2 | 13105ns | +23.8% | +32.4% | +40.9% | +193.4% | -4.6% | +258.9% |
| 1 | 3 | 12552ns | -19.7% | -0.6% | -28.9% | +196.6% | +17.2% | +307.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.384 | moderate- |
| head_tail_dyn | -0.664 | HIGH- (thermal bounce) |
| kernel_nway | -0.628 | HIGH- (thermal bounce) |
| kernel_seq | -0.654 | HIGH- (thermal bounce) |
| nway | -0.282 | moderate- |
| sequential | -0.657 | HIGH- (thermal bounce) |
| std_parallel | -0.013 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 12682.9ns | 12365.6ns | 102.6% | HIGH |
| head_tail_dyn | 13040.8ns | 12495.4ns | 104.4% | HIGH |
| kernel_nway | 13494.2ns | 13321.6ns | 101.3% | HIGH |
| kernel_seq | 12989.2ns | 12649.6ns | 102.7% | HIGH |
| nway | 39504.8ns | 37785.4ns | 104.6% | HIGH |
| sequential | 14905.3ns | 14379.1ns | 103.7% | HIGH |
| std_parallel | 49193.8ns | 47594.9ns | 103.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 11028.0-13507.0 ns)
  11028.0 |
  11151.9 |#############
  11275.9 |
  11399.8 |##########################
  11523.8 |##########################
  11647.7 |##########################
  11771.7 |
  11895.6 |#############
  12019.6 |
  12143.5 |
  12267.5 |
  12391.4 |#############
  12515.4 |
  12639.3 |
  12763.3 |#############
  12887.2 |
  13011.2 |########################################
  13135.1 |#############
  13259.1 |#############
  13383.0 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 9516.3-16496.6 ns)
   9516.3 |##########
   9865.3 |####################
  10214.4 |####################
  10563.4 |########################################
  10912.4 |
  11261.4 |
  11610.4 |
  11959.4 |
  12308.5 |
  12657.5 |
  13006.5 |##########
  13355.5 |
  13704.5 |
  14053.5 |
  14402.6 |
  14751.6 |
  15100.6 |
  15449.6 |##############################
  15798.6 |
  16147.6 |
  (2 below, 3 above range)

kernel_nway (n=18, range 10342.2-16621.0 ns)
  10342.2 |
  10656.2 |########################################
  10970.1 |
  11284.0 |
  11598.0 |##########################
  11911.9 |#############
  12225.9 |########################################
  12539.8 |##########################
  12853.7 |
  13167.7 |
  13481.6 |#############
  13795.5 |
  14109.5 |#############
  14423.4 |
  14737.4 |
  15051.3 |
  15365.2 |
  15679.2 |
  15993.1 |
  16307.0 |
  (1 below, 4 above range)

kernel_seq (n=18, range 7848.8-18080.7 ns)
   7848.8 |
   8360.4 |#############
   8872.0 |#############
   9383.6 |##########################
   9895.2 |########################################
  10406.8 |#############
  10918.4 |
  11430.0 |
  11941.6 |#############
  12453.2 |
  12964.8 |
  13476.4 |
  13987.9 |
  14499.5 |#############
  15011.1 |
  15522.7 |##########################
  16034.3 |
  16545.9 |
  17057.5 |
  17569.1 |#############
  (2 below, 3 above range)

nway (n=18, range 30629.7-43627.8 ns)
  30629.7 |####################
  31279.6 |
  31929.5 |
  32579.4 |
  33229.3 |####################
  33879.2 |
  34529.1 |########################################
  35179.0 |####################
  35828.9 |####################
  36478.8 |####################
  37128.7 |
  37778.6 |########################################
  38428.5 |########################################
  39078.4 |
  39728.3 |
  40378.2 |
  41028.2 |####################
  41678.1 |####################
  42328.0 |
  42977.9 |
  (2 below, 3 above range)

sequential (n=18, range 6718.1-22266.4 ns)
   6718.1 |########################################
   7495.5 |
   8272.9 |
   9050.3 |######
   9827.7 |######
  10605.1 |
  11382.6 |
  12160.0 |
  12937.4 |
  13714.8 |
  14492.2 |
  15269.6 |
  16047.0 |
  16824.5 |
  17601.9 |######
  18379.3 |
  19156.7 |#############
  19934.1 |
  20711.5 |####################
  21488.9 |
  (1 below, 3 above range)

std_parallel (n=18, range 36906.0-54744.9 ns)
  36906.0 |
  37797.9 |####################
  38689.9 |
  39581.8 |########################################
  40473.8 |
  41365.7 |
  42257.7 |
  43149.6 |
  44041.6 |########################################
  44933.5 |####################
  45825.4 |####################
  46717.4 |
  47609.3 |####################
  48501.3 |########################################
  49393.2 |
  50285.2 |########################################
  51177.1 |####################
  52069.1 |####################
  52961.0 |####################
  53852.9 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=104.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=23.9% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=104.7% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=20.4% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=103.2% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=33.9% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=102.1% of algo (FFI overhead may distort results)
- **nway**: bridge=102.1% of algo (FFI overhead may distort results)
- **sequential**: CV=49.0% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **sequential**: bridge=107.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=104.0% of algo (FFI overhead may distort results)
