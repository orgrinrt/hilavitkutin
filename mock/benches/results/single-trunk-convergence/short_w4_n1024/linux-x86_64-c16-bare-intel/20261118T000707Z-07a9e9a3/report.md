# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 37% faster than the next best (head_tail_dyn)

sequential (8.14 us) leads head_tail_dyn (11.15 us) by 37%, a clear separation rather than a photo finish. CV 101.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 7.5x slower than the field

std_parallel (61.44 us) is 7.5x the fastest (8.14 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 101.7%)

sequential wins on median (8.14 us) yet has the highest variance (CV 101.7%), while head_tail_dyn is the steadiest (CV 9.4%, 11.15 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### std_parallel shows alternating (throttle bounce) (autocorr -0.67)

std_parallel's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, head_tail_dyn, kernel_seq, head_tail, kernel_nway} vs {nway, std_parallel} (202% apart)

The field splits into a fast tier {sequential, head_tail_dyn, kernel_seq, head_tail, kernel_nway} and a slow tier {nway, std_parallel} with a 202% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.5x the fastest

Fastest sequential (8.14 us) to slowest std_parallel (61.44 us): 7.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### std_parallel is inconsistent: worst-20% is 3.8x its best-20%

std_parallel's best 20% of batches run at 35.73 us but its worst 20% at 136.47 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 8141.6 ns median (-37.6% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 7.55x (fastest 8141.6 ns, slowest 61442.1 ns)

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
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 13679ns | 13650ns | 11533ns | 12965ns | 15824ns | base |
| head_tail_dyn | 12055ns | 11788ns | 10554ns | 11623ns | 13452ns | -11.88% |
| kernel_nway | 14826ns | 14079ns | 12687ns | 13806ns | 17424ns | +8.38% |
| kernel_seq | 13939ns | 13348ns | 10207ns | 12796ns | 17521ns | +1.90% |
| nway | 42656ns | 41091ns | 37305ns | 39945ns | 49399ns | +211.83% |
| sequential | 14567ns | 8297ns | 6848ns | 10025ns | 25240ns | +6.49% |
| std_parallel | 79185ns | 61842ns | 36007ns | 54621ns | 137620ns | +478.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 12947ns | 10929ns | 14934ns | base | 0.079 |
| head_tail_dyn | 11364ns | 9987ns | 12636ns | -12.23% | 0.090 |
| kernel_nway | 14031ns | 12054ns | 16454ns | +8.37% | 0.073 |
| kernel_seq | 13056ns | 9453ns | 16367ns | +0.84% | 0.078 |
| nway | 42061ns | 36831ns | 48651ns | +224.87% | 0.024 |
| sequential | 14266ns | 6714ns | 24694ns | +10.19% | 0.072 |
| std_parallel | 78585ns | 35728ns | 136475ns | +506.97% | 0.013 |

## Performance model

- Peak throughput: **0.153 Gops/s** (sequential; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.079 | 51.5% |
| head_tail_dyn | 0.092 | 60.2% |
| kernel_nway | 0.076 | 50.1% |
| kernel_seq | 0.081 | 53.1% |
| nway | 0.025 | 16.6% |
| sequential | 0.126 | 82.5% |
| std_parallel | 0.017 | 10.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 13679ns | 13679ns | base |
| head_tail_dyn | 12055ns | 12055ns | -11.88% |
| kernel_nway | 14826ns | 14826ns | +8.38% |
| kernel_seq | 13939ns | 13939ns | +1.90% |
| nway | 42656ns | 42656ns | +211.83% |
| sequential | 14567ns | 14567ns | +6.49% |
| std_parallel | 79185ns | 79185ns | +478.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 13043ns | base | --- | [11568, 14147] | --- | --- | --- | --- |
| head_tail_dyn | 11149ns | -1355.7ns (-10.4%) | [-1781, -934]ns | [10648, 11925] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 13403ns | no significant difference | [-946, +3358]ns | [12279, 15529] | no | 0.3568 | 0.2379 | 0 |
| kernel_seq | 12652ns | no significant difference | [-530, +846]ns | [10822, 14355] | no | 0.5768 | 0.4807 | 0 |
| nway | 40542ns | +27317.4ns (+209.4%) | [+26066, +30428]ns | [37129, 44993] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 8142ns | no significant difference | [-4843, +9476]ns | [6934, 22401] | no | 0.8145 | 0.8145 | 0 |
| std_parallel | 61442ns | +48645.8ns (+373.0%) | [+27320, +77021]ns | [39507, 90184] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 12444ns | -9.2% | +7.7% | +1.7% | +214.1% | -7.4% | +588.1% |
| 1 | 2 | 11477ns | -7.9% | +42.4% | -13.7% | +221.6% | +30.5% | +220.3% |
| 1 | 3 | 14636ns | -11.7% | -16.3% | +8.8% | +217.7% | -51.8% | +489.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.361 | moderate- |
| head_tail_dyn | -0.365 | moderate- |
| kernel_nway | -0.617 | HIGH- (thermal bounce) |
| kernel_seq | -0.470 | moderate- |
| nway | -0.305 | moderate- |
| sequential | -0.456 | moderate- |
| std_parallel | -0.667 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 13467.2ns | 12947.0ns | 104.0% | HIGH |
| head_tail_dyn | 11871.6ns | 11364.2ns | 104.5% | HIGH |
| kernel_nway | 14647.8ns | 14031.1ns | 104.4% | HIGH |
| kernel_seq | 13736.4ns | 13055.7ns | 105.2% | HIGH |
| nway | 43022.5ns | 42061.1ns | 102.3% | HIGH |
| sequential | 14694.5ns | 14266.2ns | 103.0% | HIGH |
| std_parallel | 79849.0ns | 78585.3ns | 101.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 10928.8-14934.0 ns)
  10928.8 |
  11129.1 |####################
  11329.4 |########################################
  11529.6 |####################
  11729.9 |########################################
  11930.1 |
  12130.4 |
  12330.6 |
  12530.9 |
  12731.2 |
  12931.4 |########################################
  13131.7 |####################
  13331.9 |####################
  13532.2 |####################
  13732.5 |
  13932.7 |
  14133.0 |
  14333.2 |
  14533.5 |########################################
  14733.8 |####################
  (2 below, 2 above range)

head_tail_dyn (n=18, range 9987.0-12636.2 ns)
   9987.0 |
  10119.4 |
  10251.9 |
  10384.4 |
  10516.8 |########################################
  10649.3 |##########################
  10781.8 |
  10914.2 |#############
  11046.7 |#############
  11179.1 |#############
  11311.6 |
  11444.1 |##########################
  11576.5 |
  11709.0 |
  11841.5 |##########################
  11973.9 |
  12106.4 |
  12238.9 |
  12371.3 |
  12503.8 |
  (2 below, 4 above range)

kernel_nway (n=18, range 12054.2-16454.0 ns)
  12054.2 |########################################
  12274.2 |####################
  12494.2 |##########
  12714.2 |
  12934.2 |
  13154.2 |##########
  13374.2 |
  13594.2 |##########
  13814.1 |##########
  14034.1 |
  14254.1 |
  14474.1 |
  14694.1 |
  14914.1 |
  15134.1 |##########
  15354.1 |##########
  15574.1 |##########
  15794.0 |##########
  16014.0 |
  16234.0 |
  (1 below, 3 above range)

kernel_seq (n=18, range 9453.4-16367.1 ns)
   9453.4 |########################################
   9799.1 |
  10144.7 |####################
  10490.4 |####################
  10836.1 |####################
  11181.8 |
  11527.5 |####################
  11873.2 |####################
  12218.9 |####################
  12564.6 |####################
  12910.3 |
  13255.9 |
  13601.6 |########################################
  13947.3 |####################
  14293.0 |####################
  14638.7 |
  14984.4 |
  15330.1 |
  15675.8 |####################
  16021.5 |####################
  (1 below, 2 above range)

nway (n=18, range 36830.9-48650.6 ns)
  36830.9 |########################################
  37421.9 |
  38012.8 |########
  38603.8 |
  39194.8 |########
  39785.8 |########
  40376.8 |########
  40967.8 |
  41558.8 |
  42149.8 |########
  42740.8 |########
  43331.7 |
  43922.7 |
  44513.7 |########
  45104.7 |########
  45695.7 |
  46286.7 |
  46877.7 |
  47468.7 |########
  48059.6 |
  (1 below, 3 above range)

sequential (n=18, range 6713.9-24693.6 ns)
   6713.9 |########################################
   7612.9 |
   8511.9 |#####
   9410.9 |
  10309.9 |
  11208.8 |
  12107.8 |
  13006.8 |
  13905.8 |
  14804.8 |
  15703.8 |#####
  16602.7 |
  17501.7 |
  18400.7 |
  19299.7 |
  20198.7 |
  21097.7 |#####
  21996.6 |#####
  22895.6 |#####
  23794.6 |###########
  (2 below, 2 above range)

std_parallel (n=18, range 35727.5-136475.0 ns)
  35727.5 |########################################
  40764.9 |
  45802.3 |
  50839.7 |################
  55877.0 |
  60914.4 |
  65951.8 |
  70989.1 |################
  76026.5 |
  81063.9 |########
  86101.3 |########
  91138.6 |########
  96176.0 |########
  101213.4 |
  106250.8 |
  111288.1 |########
  116325.5 |
  121362.9 |
  126400.3 |
  131437.6 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=102.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=103.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=101.9% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=22.0% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=109.4% of algo (FFI overhead may distort results)
- **nway**: bridge=102.7% of algo (FFI overhead may distort results)
- **sequential**: CV=58.0% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **sequential**: bridge=102.8% of algo (FFI overhead may distort results)
- **std_parallel**: CV=69.0% (high variance, measurements may be unstable)
- **std_parallel**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **std_parallel**: bridge=101.8% of algo (FFI overhead may distort results)
