# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 121% faster than the next best (kernel_seq)

sequential (10.40 us) leads kernel_seq (22.98 us) by 121%, a clear separation rather than a photo finish. CV 10.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 63% (significant)

sequential is -16.99 us (63%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 6.1x slower than the field

std_parallel (63.93 us) is 6.1x the fastest (10.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.66)

head_tail's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {kernel_seq, head_tail, head_tail_dyn, kernel_nway, nway, std_parallel} (121% apart)

The field splits into a fast tier {sequential} and a slow tier {kernel_seq, head_tail, head_tail_dyn, kernel_nway, nway, std_parallel} with a 121% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.1x the fastest

Fastest sequential (10.40 us) to slowest std_parallel (63.93 us): 6.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### std_parallel is inconsistent: worst-20% is 1.5x its best-20%

std_parallel's best 20% of batches run at 52.96 us but its worst 20% at 81.14 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 10405.0 ns median (-61.6% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 6.14x (fastest 10405.0 ns, slowest 63927.9 ns)

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
| Counter frequency | 2100342298 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 29102ns | 27493ns | 25176ns | 27066ns | 34120ns | base |
| head_tail_dyn | 29391ns | 28375ns | 25139ns | 27384ns | 34529ns | +0.99% |
| kernel_nway | 31744ns | 30866ns | 26291ns | 29501ns | 37835ns | +9.08% |
| kernel_seq | 23632ns | 23463ns | 20848ns | 23185ns | 25694ns | -18.80% |
| nway | 44202ns | 43119ns | 38352ns | 42669ns | 49427ns | +51.89% |
| sequential | 10936ns | 10748ns | 9027ns | 10493ns | 12555ns | -62.42% |
| std_parallel | 67205ns | 64415ns | 53463ns | 62200ns | 81585ns | +130.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 28481ns | 24736ns | 33107ns | base | 0.002 |
| head_tail_dyn | 28956ns | 24769ns | 34038ns | +1.67% | 0.002 |
| kernel_nway | 31226ns | 25876ns | 37214ns | +9.64% | 0.002 |
| kernel_seq | 23134ns | 20326ns | 25265ns | -18.77% | 0.003 |
| nway | 43809ns | 38028ns | 48960ns | +53.82% | 0.001 |
| sequential | 10419ns | 8712ns | 11679ns | -63.42% | 0.006 |
| std_parallel | 66699ns | 52956ns | 81141ns | +134.19% | 0.001 |

## Performance model

- Peak throughput: **0.007 Gops/s** (sequential; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.002 | 32.1% |
| head_tail_dyn | 0.002 | 31.1% |
| kernel_nway | 0.002 | 28.6% |
| kernel_seq | 0.003 | 37.9% |
| nway | 0.001 | 20.4% |
| sequential | 0.006 | 83.7% |
| std_parallel | 0.001 | 13.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 29102ns | 29102ns | base |
| head_tail_dyn | 29391ns | 29391ns | +0.99% |
| kernel_nway | 31744ns | 31744ns | +9.08% |
| kernel_seq | 23632ns | 23632ns | -18.80% |
| nway | 44202ns | 44202ns | +51.89% |
| sequential | 10936ns | 10936ns | -62.42% |
| std_parallel | 67205ns | 67205ns | +130.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 27113ns | base | --- | [26044, 30060] | --- | --- | --- | --- |
| head_tail_dyn | 28009ns | no significant difference | [-844, +1745]ns | [25861, 30346] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 30491ns | no significant difference | [-1211, +7916]ns | [27184, 33690] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 22985ns | -4707.1ns (-17.4%) | [-6738, -3084]ns | [22168, 23991] | YES | 0.0000 | 0.0000 | 0 |
| nway | 42756ns | +13896.4ns (+51.3%) | [+11392, +16671]ns | [40488, 46794] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10405ns | -16992.7ns (-62.7%) | [-19396, -15257]ns | [9738, 11123] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 63928ns | +38115.2ns (+140.6%) | [+31371, +42268]ns | [57594, 68514] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 26044ns | -3.5% | +46.2% | -18.7% | +76.7% | -60.2% | +154.2% |
| 1 | 2 | 32908ns | -7.8% | -13.1% | -27.1% | +40.0% | -68.0% | +78.6% |
| 1 | 3 | 26953ns | +2.2% | +0.0% | -13.8% | +50.2% | -62.2% | +118.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.657 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.523 | HIGH- (thermal bounce) |
| kernel_nway | -0.095 | ok |
| kernel_seq | -0.335 | moderate- |
| nway | -0.172 | ok |
| sequential | -0.483 | moderate- |
| std_parallel | -0.174 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 1/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 28186.4ns | 28480.6ns | 99.0% | HIGH |
| head_tail_dyn | 28269.1ns | 28955.6ns | 97.6% | HIGH |
| kernel_nway | 31602.0ns | 31225.8ns | 101.2% | HIGH |
| kernel_seq | 23844.9ns | 23133.5ns | 103.1% | HIGH |
| nway | 44939.6ns | 43809.0ns | 102.6% | HIGH |
| sequential | 34181.4ns | 10418.6ns | 328.1% | HIGH |
| std_parallel | 65285.9ns | 66698.7ns | 97.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 24735.7-33107.3 ns)
  24735.7 |#############
  25154.3 |##########################
  25572.9 |#############
  25991.5 |#############
  26410.0 |##########################
  26828.6 |########################################
  27247.2 |
  27665.8 |#############
  28084.3 |
  28502.9 |
  28921.5 |#############
  29340.1 |
  29758.7 |
  30177.2 |
  30595.8 |
  31014.4 |#############
  31433.0 |
  31851.5 |
  32270.1 |
  32688.7 |##########################
  (1 below, 2 above range)

head_tail_dyn (n=18, range 24768.9-34038.5 ns)
  24768.9 |########################################
  25232.3 |
  25695.8 |##########
  26159.3 |##########
  26622.8 |##########
  27086.3 |
  27549.8 |##########
  28013.2 |##############################
  28476.7 |
  28940.2 |
  29403.7 |
  29867.2 |##########
  30330.6 |##########
  30794.1 |
  31257.6 |
  31721.1 |
  32184.6 |##########
  32648.0 |
  33111.5 |
  33575.0 |
  (1 below, 3 above range)

kernel_nway (n=18, range 25875.8-37214.4 ns)
  25875.8 |
  26442.7 |########################################
  27009.6 |########################################
  27576.6 |#############
  28143.5 |
  28710.4 |
  29277.4 |#############
  29844.3 |
  30411.2 |
  30978.2 |##########################
  31545.1 |
  32112.0 |#############
  32679.0 |
  33245.9 |#############
  33812.8 |#############
  34379.8 |
  34946.7 |
  35513.6 |
  36080.6 |
  36647.5 |#############
  (1 below, 3 above range)

kernel_seq (n=18, range 20326.5-25265.3 ns)
  20326.5 |####################
  20573.4 |####################
  20820.3 |
  21067.3 |
  21314.2 |
  21561.2 |####################
  21808.1 |####################
  22055.1 |
  22302.0 |####################
  22548.9 |########################################
  22795.9 |####################
  23042.8 |########################################
  23289.8 |####################
  23536.7 |
  23783.7 |####################
  24030.6 |####################
  24277.5 |
  24524.5 |
  24771.4 |####################
  25018.4 |####################
  (1 below, 2 above range)

nway (n=18, range 38027.7-48959.6 ns)
  38027.7 |#############
  38574.3 |
  39120.9 |#############
  39667.5 |#############
  40214.1 |##########################
  40760.7 |
  41307.3 |
  41853.8 |########################################
  42400.4 |
  42947.0 |#############
  43493.6 |
  44040.2 |
  44586.8 |#############
  45133.4 |#############
  45680.0 |
  46226.6 |
  46773.2 |########################################
  47319.8 |#############
  47866.4 |
  48413.0 |
  (1 below, 2 above range)

sequential (n=18, range 8712.1-11679.2 ns)
   8712.1 |
   8860.5 |
   9008.8 |
   9157.2 |
   9305.5 |
   9453.9 |
   9602.2 |########################################
   9750.6 |
   9898.9 |##########
  10047.3 |##########
  10195.6 |
  10344.0 |####################
  10492.3 |####################
  10640.7 |
  10789.0 |##########
  10937.4 |
  11085.7 |
  11234.1 |
  11382.4 |##########
  11530.8 |##########
  (2 below, 3 above range)

std_parallel (n=18, range 52956.0-81141.1 ns)
  52956.0 |
  54365.2 |####################
  55774.5 |####################
  57183.7 |########################################
  58593.0 |
  60002.2 |####################
  61411.5 |####################
  62820.7 |####################
  64230.0 |########################################
  65639.3 |####################
  67048.5 |####################
  68457.8 |####################
  69867.0 |####################
  71276.3 |
  72685.5 |
  74094.8 |
  75504.0 |####################
  76913.3 |
  78322.5 |
  79731.8 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=99.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=98.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=94.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=101.4% of algo (FFI overhead may distort results)
- **nway**: bridge=101.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=107.9% of algo (FFI overhead may distort results)
- **std_parallel**: CV=21.0% (high variance, measurements may be unstable)
- **std_parallel**: bridge=96.5% of algo (FFI overhead may distort results)
