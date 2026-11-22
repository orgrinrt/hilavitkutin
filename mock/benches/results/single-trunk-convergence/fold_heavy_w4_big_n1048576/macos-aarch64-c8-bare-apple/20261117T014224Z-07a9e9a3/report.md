# fold_heavy, frames of tens of milliseconds, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway beats baseline by 44% (significant)

kernel_nway is -193.92 us (44%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### kernel_seq is an outlier: 3.1x slower than the field

kernel_seq (754.96 us) is 3.1x the fastest (242.93 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.67)

nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.1x the fastest

Fastest kernel_nway (242.93 us) to slowest kernel_seq (754.96 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### std_parallel is inconsistent: worst-20% is 1.6x its best-20%

std_parallel's best 20% of batches run at 257.01 us but its worst 20% at 398.79 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 242927.1 ns median (-44.4% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.11x (fastest 242927.1 ns, slowest 754958.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, frames of tens of milliseconds, W = 4 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 438427ns | 437557ns | 434938ns | 437326ns | 441823ns | base |
| head_tail_dyn | 442571ns | 441495ns | 435417ns | 439941ns | 450094ns | +0.95% |
| kernel_nway | 244439ns | 243635ns | 241010ns | 243330ns | 247818ns | -44.25% |
| kernel_seq | 755674ns | 755307ns | 744875ns | 752115ns | 766411ns | +72.36% |
| nway | 264073ns | 250182ns | 248604ns | 250108ns | 292755ns | -39.77% |
| sequential | 762403ns | 751089ns | 746542ns | 749771ns | 789281ns | +73.90% |
| std_parallel | 315446ns | 288068ns | 257375ns | 278792ns | 399464ns | -28.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 437608ns | 434302ns | 440635ns | base | 2.396 |
| head_tail_dyn | 441726ns | 434719ns | 449380ns | +0.94% | 2.374 |
| kernel_nway | 243648ns | 240365ns | 246943ns | -44.32% | 4.304 |
| kernel_seq | 755326ns | 744573ns | 765969ns | +72.60% | 1.388 |
| nway | 263385ns | 247854ns | 292083ns | -39.81% | 3.981 |
| sequential | 761929ns | 746365ns | 788417ns | +74.11% | 1.376 |
| std_parallel | 314885ns | 257010ns | 398792ns | -28.04% | 3.330 |

## Performance model

- Peak throughput: **4.362 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.400 | 55.0% |
| head_tail_dyn | 2.382 | 54.6% |
| kernel_nway | 4.316 | 98.9% |
| kernel_seq | 1.389 | 31.8% |
| nway | 4.203 | 96.3% |
| sequential | 1.397 | 32.0% |
| std_parallel | 3.647 | 83.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 438427ns | 438427ns | base |
| head_tail_dyn | 442571ns | 442571ns | +0.95% |
| kernel_nway | 244439ns | 244439ns | -44.25% |
| kernel_seq | 755674ns | 755674ns | +72.36% |
| nway | 264073ns | 264073ns | -39.77% |
| sequential | 762403ns | 762403ns | +73.90% |
| std_parallel | 315446ns | 315446ns | -28.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 436875ns | base | --- | [435312, 440635] | --- | --- | --- | --- |
| head_tail_dyn | 440224ns | no significant difference | [-2880, +13880]ns | [435573, 449380] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 242927ns | -193916.7ns (-44.4%) | [-195672, -192292]ns | [241073, 246943] | YES | 0.0375 | 0.0313 | 0 |
| kernel_seq | 754958ns | +318213.5ns (+72.8%) | [+305849, +329094]ns | [745052, 765969] | YES | 0.0375 | 0.0313 | 0 |
| nway | 249500ns | -187010.4ns (-42.8%) | [-188536, -147120]ns | [248573, 292083] | YES | 0.0375 | 0.0313 | 0 |
| sequential | 750797ns | +312474.0ns (+71.5%) | [+309182, +351307]ns | [746573, 788417] | YES | 0.0375 | 0.0313 | 0 |
| std_parallel | 287542ns | -148968.7ns (-34.1%) | [-180880, -38318]ns | [258323, 398792] | YES | 0.0375 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 436510ns | +2.2% | -44.6% | +73.5% | -42.8% | +71.4% | -34.1% |
| 1 | 2 | 439203ns | +0.9% | -43.9% | +69.6% | -33.5% | +70.6% | -41.2% |
| 1 | 3 | 437109ns | -0.4% | -44.4% | +74.7% | -43.1% | +80.4% | -8.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.637 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.041 | ok |
| kernel_nway | -0.629 | HIGH- (thermal bounce) |
| kernel_seq | -0.592 | HIGH- (thermal bounce) |
| nway | -0.666 | HIGH- (thermal bounce) |
| sequential | -0.150 | ok |
| std_parallel | -0.291 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 438803.8ns | 437607.7ns | 100.3% | HIGH |
| head_tail_dyn | 446225.7ns | 441725.7ns | 101.0% | HIGH |
| kernel_nway | 238296.9ns | 243647.6ns | 97.8% | HIGH |
| kernel_seq | 754539.9ns | 755326.4ns | 99.9% | HIGH |
| nway | 252598.9ns | 263385.4ns | 95.9% | HIGH |
| sequential | 761241.3ns | 761928.8ns | 99.9% | HIGH |
| std_parallel | 313031.2ns | 314885.4ns | 99.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 434302.1-440635.5 ns)
  434302.1 |########################################
  434618.8 |
  434935.4 |
  435252.1 |
  435568.8 |
  435885.4 |
  436202.1 |########################################
  436518.8 |########################################
  436835.4 |########################################
  437152.1 |########################################
  437468.8 |
  437785.4 |
  438102.1 |
  438418.8 |
  438735.4 |
  439052.1 |
  439368.8 |
  439685.4 |
  440002.1 |
  440318.8 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 434718.8-449380.2 ns)
  434718.8 |########################################
  435451.9 |
  436184.9 |########################################
  436918.0 |
  437651.1 |
  438384.2 |
  439117.2 |########################################
  439850.3 |
  440583.4 |########################################
  441316.4 |
  442049.5 |
  442782.6 |
  443515.6 |
  444248.7 |
  444981.8 |
  445714.8 |########################################
  446447.9 |
  447181.0 |
  447914.1 |
  448647.1 |
  (0 below, 1 above range)

kernel_nway (n=6, range 240364.6-246942.8 ns)
  240364.6 |########################################
  240693.5 |
  241022.4 |
  241351.3 |
  241680.2 |########################################
  242009.1 |
  242338.0 |
  242667.0 |########################################
  242995.9 |########################################
  243324.8 |
  243653.7 |########################################
  243982.6 |
  244311.5 |
  244640.4 |
  244969.3 |
  245298.2 |
  245627.1 |
  245956.0 |
  246284.9 |
  246613.8 |
  (0 below, 1 above range)

kernel_seq (n=6, range 744572.9-765968.7 ns)
  744572.9 |########################################
  745642.7 |
  746712.5 |
  747782.3 |
  748852.1 |
  749921.8 |
  750991.6 |
  752061.4 |####################
  753131.2 |
  754201.0 |
  755270.8 |
  756340.6 |####################
  757410.4 |
  758480.2 |
  759550.0 |
  760619.8 |
  761689.5 |####################
  762759.3 |
  763829.1 |
  764898.9 |
  (0 below, 1 above range)

nway (n=6, range 247854.2-292083.3 ns)
  247854.2 |########################################
  250065.7 |
  252277.1 |
  254488.6 |
  256700.0 |
  258911.5 |
  261122.9 |
  263334.4 |
  265545.9 |
  267757.3 |##########
  269968.8 |
  272180.2 |
  274391.7 |
  276603.1 |
  278814.6 |
  281026.1 |
  283237.5 |
  285449.0 |
  287660.4 |
  289871.9 |
  (0 below, 1 above range)

sequential (n=6, range 746364.6-788416.7 ns)
  746364.6 |########################################
  748467.2 |####################
  750569.8 |####################
  752672.4 |
  754775.0 |
  756877.6 |
  758980.2 |
  761082.8 |
  763185.4 |
  765288.0 |
  767390.6 |
  769493.2 |
  771595.8 |
  773698.4 |
  775801.0 |
  777903.6 |
  780006.2 |
  782108.8 |
  784211.4 |####################
  786314.0 |
  (0 below, 1 above range)

std_parallel (n=6, range 257010.4-398791.7 ns)
  257010.4 |########################################
  264099.5 |####################
  271188.5 |
  278277.6 |
  285366.7 |
  292455.7 |
  299544.8 |
  306633.9 |####################
  313722.9 |
  320812.0 |
  327901.0 |
  334990.1 |
  342079.2 |
  349168.2 |
  356257.3 |
  363346.4 |
  370435.4 |
  377524.5 |
  384613.6 |
  391702.6 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=98.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.7% of algo (FFI overhead may distort results)
- **nway**: bridge=99.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=96.8% of algo (FFI overhead may distort results)
