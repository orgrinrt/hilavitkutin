# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 94% faster than the next best (kernel_seq)

sequential (6.98 us) leads kernel_seq (13.51 us) by 94%, a clear separation rather than a photo finish. CV 122.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 10.1x slower than the field

std_parallel (70.16 us) is 10.1x the fastest (6.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 122.6%)

sequential wins on median (6.98 us) yet has the highest variance (CV 122.6%), while nway is the steadiest (CV 11.2%, 53.76 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.60)

kernel_nway's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, kernel_seq, head_tail_dyn, head_tail, kernel_nway} vs {nway, std_parallel} (200% apart)

The field splits into a fast tier {sequential, kernel_seq, head_tail_dyn, head_tail, kernel_nway} and a slow tier {nway, std_parallel} with a 200% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 10.1x the fastest

Fastest sequential (6.98 us) to slowest std_parallel (70.16 us): 10.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.7x its best-20%

sequential's best 20% of batches run at 6.74 us but its worst 20% at 24.93 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 6977.9 ns median (-50.2% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 10.05x (fastest 6977.9 ns, slowest 70155.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 8 |
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
| head_tail | 15172ns | 14746ns | 12608ns | 14486ns | 17482ns | base |
| head_tail_dyn | 15051ns | 14771ns | 12167ns | 14002ns | 18066ns | -0.79% |
| kernel_nway | 19472ns | 18568ns | 16080ns | 17963ns | 23432ns | +28.34% |
| kernel_seq | 13107ns | 14176ns | 7556ns | 12275ns | 17131ns | -13.61% |
| nway | 53519ns | 54474ns | 44251ns | 52237ns | 60077ns | +252.76% |
| sequential | 14195ns | 7115ns | 6872ns | 9129ns | 25454ns | -6.44% |
| std_parallel | 100941ns | 70404ns | 62578ns | 69433ns | 167383ns | +565.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 14372ns | 12076ns | 16425ns | base | 0.001 |
| head_tail_dyn | 14189ns | 11552ns | 16850ns | -1.27% | 0.001 |
| kernel_nway | 18719ns | 15249ns | 22613ns | +30.25% | 0.001 |
| kernel_seq | 12318ns | 7078ns | 16009ns | -14.29% | 0.001 |
| nway | 52854ns | 43767ns | 59296ns | +267.76% | 0.000 |
| sequential | 13908ns | 6738ns | 24930ns | -3.23% | 0.001 |
| std_parallel | 100447ns | 62351ns | 166396ns | +598.91% | 0.000 |

## Performance model

- Peak throughput: **0.002 Gops/s** (sequential; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.001 | 48.0% |
| head_tail_dyn | 0.001 | 48.8% |
| kernel_nway | 0.001 | 37.5% |
| kernel_seq | 0.001 | 49.9% |
| nway | 0.000 | 12.5% |
| sequential | 0.002 | 96.6% |
| std_parallel | 0.000 | 9.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 15172ns | 15172ns | base |
| head_tail_dyn | 15051ns | 15051ns | -0.79% |
| kernel_nway | 19472ns | 19472ns | +28.34% |
| kernel_seq | 13107ns | 13107ns | -13.61% |
| nway | 53519ns | 53519ns | +252.76% |
| sequential | 14195ns | 14195ns | -6.44% |
| std_parallel | 100941ns | 100941ns | +565.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 14026ns | base | --- | [13375, 15428] | --- | --- | --- | --- |
| head_tail_dyn | 13809ns | no significant difference | [-698, +303]ns | [12626, 15154] | no | 0.5768 | 0.4807 | 0 |
| kernel_nway | 17948ns | +5148.2ns (+36.7%) | [+1296, +7770]ns | [16325, 21407] | YES (adj: no) | 0.0618 | 0.0309 | 0 |
| kernel_seq | 13510ns | no significant difference | [-4793, +324]ns | [8468, 14890] | no | 0.3568 | 0.2379 | 0 |
| nway | 53760ns | +37853.7ns (+269.9%) | [+35871, +41555]ns | [49756, 55214] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 6978ns | no significant difference | [-6400, +3999]ns | [6859, 20863] | no | 0.8145 | 0.8145 | 0 |
| std_parallel | 70156ns | +56130.1ns (+400.2%) | [+53267, +72384]ns | [66642, 89827] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 13084ns | -4.3% | +37.2% | -42.7% | +280.3% | -46.7% | +420.7% |
| 1 | 2 | 13735ns | -2.6% | +65.2% | +7.2% | +302.0% | -19.7% | +409.7% |
| 1 | 3 | 16024ns | +1.2% | -2.2% | -10.1% | +238.2% | -25.1% | +460.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.063 | ok |
| head_tail_dyn | -0.058 | ok |
| kernel_nway | -0.600 | HIGH- (thermal bounce) |
| kernel_seq | -0.190 | ok |
| nway | -0.278 | moderate- |
| sequential | -0.074 | ok |
| std_parallel | -0.124 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 14958.0ns | 14372.0ns | 104.1% | HIGH |
| head_tail_dyn | 14557.2ns | 14189.4ns | 102.6% | HIGH |
| kernel_nway | 18992.4ns | 18718.9ns | 101.5% | HIGH |
| kernel_seq | 12767.0ns | 12317.5ns | 103.6% | HIGH |
| nway | 53729.5ns | 52853.9ns | 101.7% | HIGH |
| sequential | 15803.3ns | 13907.8ns | 113.6% | HIGH |
| std_parallel | 102728.8ns | 100446.7ns | 102.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 12076.5-16425.4 ns)
  12076.5 |
  12293.9 |
  12511.4 |#############
  12728.8 |#############
  12946.3 |
  13163.7 |#############
  13381.2 |########################################
  13598.6 |
  13816.0 |#############
  14033.5 |#############
  14250.9 |#############
  14468.4 |
  14685.8 |
  14903.3 |#############
  15120.7 |#############
  15338.1 |
  15555.6 |#############
  15773.0 |#############
  15990.5 |
  16207.9 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 11552.0-16850.0 ns)
  11552.0 |
  11816.9 |
  12081.8 |#############
  12346.7 |##########################
  12611.6 |########################################
  12876.5 |
  13141.4 |
  13406.3 |#############
  13671.2 |
  13936.1 |#############
  14201.0 |##########################
  14465.9 |
  14730.8 |#############
  14995.7 |
  15260.6 |#############
  15525.5 |
  15790.4 |
  16055.3 |
  16320.2 |
  16585.1 |
  (2 below, 4 above range)

kernel_nway (n=18, range 15249.3-22612.6 ns)
  15249.3 |
  15617.4 |########################################
  15985.6 |####################
  16353.8 |########################################
  16721.9 |####################
  17090.1 |####################
  17458.3 |
  17826.4 |
  18194.6 |
  18562.8 |########################################
  18930.9 |####################
  19299.1 |
  19667.3 |
  20035.4 |
  20403.6 |
  20771.8 |
  21139.9 |####################
  21508.1 |####################
  21876.3 |
  22244.4 |####################
  (2 below, 3 above range)

kernel_seq (n=18, range 7077.8-16008.5 ns)
   7077.8 |########################################
   7524.3 |####################
   7970.8 |####################
   8417.4 |
   8863.9 |####################
   9310.5 |
   9757.0 |
  10203.5 |
  10650.1 |
  11096.6 |
  11543.1 |
  11989.7 |########################################
  12436.2 |
  12882.8 |####################
  13329.3 |
  13775.8 |########################################
  14222.4 |########################################
  14668.9 |
  15115.4 |####################
  15562.0 |####################
  (1 below, 3 above range)

nway (n=18, range 43766.8-59296.4 ns)
  43766.8 |
  44543.3 |
  45319.8 |
  46096.2 |
  46872.7 |#############
  47649.2 |#############
  48425.7 |
  49202.2 |##########################
  49978.7 |##########################
  50755.1 |
  51531.6 |
  52308.1 |
  53084.6 |#############
  53861.1 |##########################
  54637.6 |########################################
  55414.0 |
  56190.5 |
  56967.0 |
  57743.5 |#############
  58520.0 |
  (2 below, 3 above range)

sequential (n=18, range 6738.2-24929.9 ns)
   6738.2 |########################################
   7647.8 |
   8557.4 |
   9467.0 |
  10376.6 |
  11286.2 |
  12195.7 |
  13105.3 |
  14014.9 |
  14924.5 |#####
  15834.1 |
  16743.7 |#####
  17653.2 |
  18562.8 |
  19472.4 |
  20382.0 |##########
  21291.6 |
  22201.2 |
  23110.7 |#####
  24020.3 |
  (2 below, 3 above range)

std_parallel (n=18, range 62351.2-166395.8 ns)
  62351.2 |########################################
  67553.4 |########################################
  72755.6 |####################
  77957.9 |
  83160.1 |
  88362.3 |####################
  93564.5 |
  98766.8 |##########
  103969.0 |
  109171.2 |
  114373.5 |
  119575.7 |##########
  124777.9 |
  129980.2 |
  135182.4 |
  140384.6 |
  145586.8 |
  150789.1 |
  155991.3 |
  161193.5 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=103.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=103.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=99.5% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=29.1% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=103.5% of algo (FFI overhead may distort results)
- **nway**: bridge=101.8% of algo (FFI overhead may distort results)
- **sequential**: CV=61.5% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **sequential**: bridge=103.2% of algo (FFI overhead may distort results)
- **std_parallel**: CV=72.4% (high variance, measurements may be unstable)
- **std_parallel**: bridge=99.6% of algo (FFI overhead may distort results)
