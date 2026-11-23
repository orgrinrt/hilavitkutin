# append, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 4.1x slower than the field

std_parallel (54.15 us) is 4.1x the fastest (13.33 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail, kernel_seq) are a dead heat (<1%)

head_tail (13.33 us) and kernel_seq (13.44 us) differ by 0.76%, inside the noise, even though the wider field spreads 306.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### std_parallel shows alternating (throttle bounce) (autocorr -0.67)

std_parallel's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (13.33 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, kernel_seq, sequential, kernel_nway, head_tail_dyn} vs {nway, std_parallel} (209% apart)

The field splits into a fast tier {head_tail, kernel_seq, sequential, kernel_nway, head_tail_dyn} and a slow tier {nway, std_parallel} with a 209% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.1x the fastest

Fastest head_tail (13.33 us) to slowest std_parallel (54.15 us): 4.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.4x its best-20%

sequential's best 20% of batches run at 9.86 us but its worst 20% at 33.77 us (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 13334.6 ns median
- 4 variants significantly slower than baseline
- Spread: 4.06x (fastest 13334.6 ns, slowest 54147.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 2 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112781387 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 14407ns | 13868ns | 11465ns | 13322ns | 17505ns | base |
| head_tail_dyn | 16133ns | 16076ns | 13204ns | 15500ns | 18548ns | +11.98% |
| kernel_nway | 15739ns | 15312ns | 12592ns | 15018ns | 18394ns | +9.25% |
| kernel_seq | 14430ns | 13956ns | 7674ns | 13622ns | 19022ns | +0.16% |
| nway | 47637ns | 48170ns | 40735ns | 46128ns | 53353ns | +230.66% |
| sequential | 20320ns | 14432ns | 9999ns | 14426ns | 34321ns | +41.04% |
| std_parallel | 54633ns | 54706ns | 44513ns | 52283ns | 63217ns | +279.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 13555ns | 10951ns | 15937ns | base | 0.302 |
| head_tail_dyn | 15286ns | 12731ns | 17215ns | +12.77% | 0.268 |
| kernel_nway | 14888ns | 12051ns | 16997ns | +9.84% | 0.275 |
| kernel_seq | 13696ns | 7132ns | 17843ns | +1.04% | 0.299 |
| nway | 47057ns | 40283ns | 52689ns | +247.16% | 0.087 |
| sequential | 20007ns | 9856ns | 33767ns | +47.60% | 0.205 |
| std_parallel | 53798ns | 44043ns | 62034ns | +296.89% | 0.076 |

## Performance model

- Peak throughput: **0.574 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.307 | 53.5% |
| head_tail_dyn | 0.266 | 46.4% |
| kernel_nway | 0.278 | 48.3% |
| kernel_seq | 0.305 | 53.1% |
| nway | 0.086 | 15.0% |
| sequential | 0.288 | 50.2% |
| std_parallel | 0.076 | 13.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 14407ns | 14407ns | base |
| head_tail_dyn | 16133ns | 16133ns | +11.98% |
| kernel_nway | 15739ns | 15739ns | +9.25% |
| kernel_seq | 14430ns | 14430ns | +0.16% |
| nway | 47637ns | 47637ns | +230.66% |
| sequential | 20320ns | 20320ns | +41.04% |
| std_parallel | 54633ns | 54633ns | +279.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 13335ns | base | --- | [12187, 14485] | --- | --- | --- | --- |
| head_tail_dyn | 15386ns | +947.8ns (+7.1%) | [+60, +3901]ns | [14006, 16693] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| kernel_nway | 14757ns | +1001.3ns (+7.5%) | [+158, +2164]ns | [13984, 15444] | YES (adj: no) | 0.0618 | 0.0309 | 0 |
| kernel_seq | 13436ns | no significant difference | [-618, +3770]ns | [12243, 16326] | no | 0.2855 | 0.2379 | 0 |
| nway | 47599ns | +33058.2ns (+247.9%) | [+29496, +37222]ns | [42820, 50057] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 14213ns | no significant difference | [-2221, +14621]ns | [10023, 30273] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 54148ns | +38337.3ns (+287.5%) | [+35243, +44935]ns | [48134, 58082] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 15023ns | -2.4% | +0.5% | -18.5% | +201.9% | -32.7% | +236.4% |
| 1 | 2 | 12356ns | +34.3% | +23.9% | +38.5% | +305.1% | +84.8% | +403.1% |
| 1 | 3 | 12595ns | +11.2% | -0.1% | +3.6% | +237.9% | +27.0% | +301.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.215 | moderate- |
| head_tail_dyn | -0.628 | HIGH- (thermal bounce) |
| kernel_nway | -0.207 | moderate- |
| kernel_seq | -0.651 | HIGH- (thermal bounce) |
| nway | -0.576 | HIGH- (thermal bounce) |
| sequential | -0.524 | HIGH- (thermal bounce) |
| std_parallel | -0.667 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 67326.3ns | 13554.8ns | 496.7% | HIGH |
| head_tail_dyn | 68618.9ns | 15286.4ns | 448.9% | HIGH |
| kernel_nway | 74919.9ns | 14888.1ns | 503.2% | HIGH |
| kernel_seq | 64139.0ns | 13696.3ns | 468.3% | HIGH |
| nway | 99093.6ns | 47057.3ns | 210.6% | HIGH |
| sequential | 50305.9ns | 20006.9ns | 251.4% | HIGH |
| std_parallel | 105029.5ns | 53797.7ns | 195.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 10951.2-15937.2 ns)
  10951.2 |
  11200.5 |####################
  11449.8 |####################
  11699.1 |
  11948.4 |####################
  12197.7 |####################
  12447.0 |
  12696.3 |########################################
  12945.6 |
  13194.9 |########################################
  13444.2 |########################################
  13693.5 |
  13942.8 |
  14192.1 |####################
  14441.4 |########################################
  14690.7 |
  14940.0 |
  15189.3 |
  15438.6 |####################
  15687.9 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 12730.9-17215.3 ns)
  12730.9 |####################
  12955.1 |
  13179.3 |
  13403.5 |########################################
  13627.8 |####################
  13852.0 |
  14076.2 |########################################
  14300.4 |
  14524.6 |
  14748.9 |
  14973.1 |####################
  15197.3 |####################
  15421.5 |####################
  15645.7 |
  15870.0 |####################
  16094.2 |####################
  16318.4 |
  16542.6 |########################################
  16766.8 |####################
  16991.1 |########################################
  (1 below, 1 above range)

kernel_nway (n=18, range 12051.2-16997.3 ns)
  12051.2 |##########################
  12298.5 |
  12545.8 |
  12793.1 |#############
  13040.4 |
  13287.7 |
  13535.0 |#############
  13782.3 |
  14029.6 |#############
  14276.9 |##########################
  14524.2 |#############
  14771.5 |#############
  15018.8 |#############
  15266.2 |########################################
  15513.5 |
  15760.8 |#############
  16008.1 |
  16255.4 |
  16502.7 |
  16750.0 |
  (1 below, 3 above range)

kernel_seq (n=18, range 7131.5-17843.4 ns)
   7131.5 |
   7667.1 |
   8202.7 |
   8738.3 |
   9273.9 |
   9809.5 |
  10345.1 |
  10880.7 |#############
  11416.3 |##########################
  11951.9 |
  12487.5 |########################################
  13023.1 |#############
  13558.6 |#############
  14094.2 |#############
  14629.8 |
  15165.4 |#############
  15701.0 |#############
  16236.6 |
  16772.2 |########################################
  17307.8 |
  (2 below, 2 above range)

nway (n=18, range 40282.5-52689.4 ns)
  40282.5 |
  40902.9 |
  41523.2 |########################################
  42143.6 |####################
  42763.9 |####################
  43384.3 |####################
  44004.6 |
  44624.9 |
  45245.3 |
  45865.6 |
  46486.0 |
  47106.3 |########################################
  47726.7 |########################################
  48347.0 |########################################
  48967.4 |
  49587.7 |
  50208.0 |
  50828.4 |
  51448.7 |####################
  52069.1 |
  (2 below, 4 above range)

sequential (n=18, range 9855.9-33766.7 ns)
   9855.9 |########################################
  11051.4 |
  12247.0 |
  13442.5 |
  14638.1 |
  15833.6 |
  17029.1 |
  18224.7 |#####
  19420.2 |
  20615.8 |
  21811.3 |#####
  23006.8 |
  24202.4 |
  25397.9 |
  26593.5 |#####
  27789.0 |
  28984.5 |
  30180.1 |#################
  31375.6 |
  32571.2 |#####
  (2 below, 2 above range)

std_parallel (n=18, range 44042.6-62033.7 ns)
  44042.6 |
  44942.2 |
  45841.7 |
  46741.3 |##########################
  47640.8 |##########################
  48540.4 |
  49439.9 |
  50339.5 |##########################
  51239.0 |
  52138.6 |
  53038.1 |#############
  53937.7 |
  54837.3 |########################################
  55736.8 |#############
  56636.4 |
  57535.9 |
  58435.5 |
  59335.0 |#############
  60234.6 |
  61134.1 |#############
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=487.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=440.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=484.1% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=29.3% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=517.9% of algo (FFI overhead may distort results)
- **nway**: bridge=208.2% of algo (FFI overhead may distort results)
- **sequential**: CV=55.4% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **sequential**: bridge=250.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=193.4% of algo (FFI overhead may distort results)
