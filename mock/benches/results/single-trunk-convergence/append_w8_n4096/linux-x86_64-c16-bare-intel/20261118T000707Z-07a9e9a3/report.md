# append, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 54% faster than the next best (head_tail)

sequential (10.43 us) leads head_tail (16.09 us) by 54%, a clear separation rather than a photo finish. CV 122.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 9.4x slower than the field

std_parallel (97.72 us) is 9.4x the fastest (10.43 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 122.0%)

sequential wins on median (10.43 us) yet has the highest variance (CV 122.0%), while head_tail_dyn is the steadiest (CV 10.6%, 17.13 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### std_parallel shows alternating (throttle bounce) (autocorr -0.66)

std_parallel's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, head_tail, kernel_seq, head_tail_dyn, kernel_nway} vs {nway, std_parallel} (171% apart)

The field splits into a fast tier {sequential, head_tail, kernel_seq, head_tail_dyn, kernel_nway} and a slow tier {nway, std_parallel} with a 171% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 9.4x the fastest

Fastest sequential (10.43 us) to slowest std_parallel (97.72 us): 9.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 4.5x its best-20%

kernel_nway's best 20% of batches run at 6.68 us but its worst 20% at 30.14 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 10434.8 ns median (-35.2% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 9.36x (fastest 10434.8 ns, slowest 97716.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 8 |
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
| head_tail | 17243ns | 16899ns | 11570ns | 16170ns | 21689ns | base |
| head_tail_dyn | 18552ns | 17742ns | 16440ns | 17550ns | 21112ns | +7.59% |
| kernel_nway | 21965ns | 22679ns | 6910ns | 20657ns | 31455ns | +27.38% |
| kernel_seq | 17964ns | 17406ns | 13389ns | 16791ns | 22012ns | +4.18% |
| nway | 60394ns | 58023ns | 52329ns | 56269ns | 70614ns | +250.25% |
| sequential | 20842ns | 10583ns | 9898ns | 13157ns | 37843ns | +20.87% |
| std_parallel | 90195ns | 98136ns | 64004ns | 88397ns | 105988ns | +423.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 16257ns | 11059ns | 20132ns | base | 0.252 |
| head_tail_dyn | 17586ns | 15906ns | 19417ns | +8.17% | 0.233 |
| kernel_nway | 21048ns | 6682ns | 30140ns | +29.47% | 0.195 |
| kernel_seq | 16991ns | 12913ns | 20229ns | +4.52% | 0.241 |
| nway | 59708ns | 51708ns | 69846ns | +267.27% | 0.069 |
| sequential | 20526ns | 9750ns | 37247ns | +26.26% | 0.200 |
| std_parallel | 89649ns | 63768ns | 105229ns | +451.44% | 0.046 |

## Performance model

- Peak throughput: **0.613 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.255 | 41.5% |
| head_tail_dyn | 0.239 | 39.0% |
| kernel_nway | 0.193 | 31.5% |
| kernel_seq | 0.244 | 39.8% |
| nway | 0.071 | 11.7% |
| sequential | 0.393 | 64.0% |
| std_parallel | 0.042 | 6.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 17243ns | 17243ns | base |
| head_tail_dyn | 18552ns | 18552ns | +7.59% |
| kernel_nway | 21965ns | 21965ns | +27.38% |
| kernel_seq | 17964ns | 17964ns | +4.18% |
| nway | 60394ns | 60394ns | +250.25% |
| sequential | 20842ns | 20842ns | +20.87% |
| std_parallel | 90195ns | 90195ns | +423.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 16094ns | base | --- | [14562, 17676] | --- | --- | --- | --- |
| head_tail_dyn | 17132ns | no significant difference | [-1001, +4554]ns | [16654, 17797] | no | 0.3568 | 0.2379 | 0 |
| kernel_nway | 21180ns | +5616.1ns (+34.9%) | [+3301, +8347]ns | [18203, 26079] | YES (adj: no) | 0.0618 | 0.0309 | 0 |
| kernel_seq | 16786ns | no significant difference | [-194, +1368]ns | [15672, 18543] | no | 0.5768 | 0.4807 | 0 |
| nway | 57331ns | +41268.9ns (+256.4%) | [+38713, +42180]ns | [53606, 59515] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10435ns | no significant difference | [-5658, +14054]ns | [10052, 30633] | no | 0.8145 | 0.8145 | 0 |
| std_parallel | 97716ns | +77784.9ns (+483.3%) | [+64163, +81977]ns | [74795, 99732] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 16208ns | +5.3% | +66.0% | +1.8% | +253.7% | -7.0% | +496.5% |
| 1 | 2 | 12379ns | +41.0% | +47.0% | +11.9% | +320.1% | +44.1% | +443.9% |
| 1 | 3 | 17676ns | -3.7% | +15.9% | +1.2% | +236.7% | -41.0% | +459.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.619 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.663 | HIGH- (thermal bounce) |
| kernel_nway | -0.329 | moderate- |
| kernel_seq | -0.590 | HIGH- (thermal bounce) |
| nway | -0.613 | HIGH- (thermal bounce) |
| sequential | -0.410 | moderate- |
| std_parallel | -0.664 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 74924.0ns | 16257.1ns | 460.9% | HIGH |
| head_tail_dyn | 79303.0ns | 17586.0ns | 450.9% | HIGH |
| kernel_nway | 84969.5ns | 21047.7ns | 403.7% | HIGH |
| kernel_seq | 79862.1ns | 16991.4ns | 470.0% | HIGH |
| nway | 118505.8ns | 59707.5ns | 198.5% | HIGH |
| sequential | 51478.8ns | 20526.3ns | 250.8% | HIGH |
| std_parallel | 119080.5ns | 89648.9ns | 132.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 11059.0-20131.6 ns)
  11059.0 |
  11512.6 |####################
  11966.2 |
  12419.9 |
  12873.5 |####################
  13327.1 |
  13780.8 |####################
  14234.4 |
  14688.0 |
  15141.7 |########################################
  15595.3 |########################################
  16048.9 |########################################
  16502.6 |####################
  16956.2 |####################
  17409.8 |
  17863.5 |########################################
  18317.1 |
  18770.7 |
  19224.4 |
  19678.0 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 15905.9-19416.8 ns)
  15905.9 |
  16081.4 |
  16257.0 |##########################
  16432.5 |##########################
  16608.1 |#############
  16783.6 |#############
  16959.2 |##########################
  17134.7 |########################################
  17310.3 |
  17485.8 |
  17661.4 |#############
  17836.9 |##########################
  18012.4 |
  18188.0 |
  18363.5 |
  18539.1 |
  18714.6 |
  18890.2 |
  19065.7 |
  19241.3 |
  (1 below, 3 above range)

kernel_nway (n=18, range 6681.7-30140.3 ns)
   6681.7 |
   7854.6 |#############
   9027.5 |
  10200.5 |#############
  11373.4 |
  12546.3 |
  13719.3 |
  14892.2 |
  16065.1 |
  17238.0 |##########################
  18411.0 |
  19583.9 |########################################
  20756.8 |#############
  21929.8 |
  23102.7 |#############
  24275.6 |#############
  25448.6 |##########################
  26621.5 |#############
  27794.4 |
  28967.4 |#############
  (2 below, 2 above range)

kernel_seq (n=18, range 12913.3-20228.5 ns)
  12913.3 |
  13279.0 |#############
  13644.8 |
  14010.6 |#############
  14376.3 |
  14742.1 |
  15107.8 |
  15473.6 |########################################
  15839.4 |#############
  16205.1 |#############
  16570.9 |
  16936.7 |#############
  17302.4 |##########################
  17668.2 |
  18033.9 |#############
  18399.7 |
  18765.5 |##########################
  19131.2 |
  19497.0 |#############
  19862.8 |
  (2 below, 2 above range)

nway (n=18, range 51708.3-69846.2 ns)
  51708.3 |########################################
  52615.2 |
  53522.1 |########################################
  54429.0 |
  55335.9 |
  56242.8 |##########################
  57149.7 |##########################
  58056.6 |##########################
  58963.5 |
  59870.3 |#############
  60777.2 |
  61684.1 |#############
  62591.0 |
  63497.9 |
  64404.8 |
  65311.7 |
  66218.6 |
  67125.5 |
  68032.4 |
  68939.3 |#############
  (1 below, 2 above range)

sequential (n=18, range 9749.9-37247.0 ns)
   9749.9 |########################################
  11124.8 |
  12499.6 |
  13874.5 |
  15249.3 |
  16624.2 |
  17999.0 |
  19373.9 |#####
  20748.7 |
  22123.6 |
  23498.4 |
  24873.3 |#####
  26248.1 |
  27623.0 |
  28997.8 |
  30372.7 |#####
  31747.5 |
  33122.4 |#####
  34497.2 |
  35872.1 |#####
  (2 below, 3 above range)

std_parallel (n=18, range 63767.7-105228.9 ns)
  63767.7 |####################
  65840.7 |
  67913.8 |
  69986.8 |####################
  72059.9 |
  74133.0 |
  76206.0 |
  78279.1 |##########
  80352.1 |
  82425.2 |
  84498.3 |##########
  86571.3 |
  88644.4 |
  90717.4 |
  92790.5 |##########
  94863.6 |
  96936.6 |########################################
  99009.7 |####################
  101082.7 |####################
  103155.8 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=22.2% (high variance, measurements may be unstable)
- **head_tail**: bridge=451.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=471.6% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=41.5% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **kernel_nway**: bridge=420.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=475.8% of algo (FFI overhead may distort results)
- **nway**: bridge=206.1% of algo (FFI overhead may distort results)
- **sequential**: CV=62.0% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **sequential**: bridge=254.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=132.3% of algo (FFI overhead may distort results)
