# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 46% faster than the next best (head_tail)

sequential (7.12 us) leads head_tail (10.43 us) by 46%, a clear separation rather than a photo finish. CV 125.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 5.4x slower than the field

std_parallel (38.45 us) is 5.4x the fastest (7.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 125.7%)

sequential wins on median (7.12 us) yet has the highest variance (CV 125.7%), while nway is the steadiest (CV 12.1%, 34.98 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sequential shows alternating (throttle bounce) (autocorr -0.65)

sequential's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, head_tail, head_tail_dyn, kernel_nway, kernel_seq} vs {nway, std_parallel} (170% apart)

The field splits into a fast tier {sequential, head_tail, head_tail_dyn, kernel_nway, kernel_seq} and a slow tier {nway, std_parallel} with a 170% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.4x the fastest

Fastest sequential (7.12 us) to slowest std_parallel (38.45 us): 5.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.8x its best-20%

sequential's best 20% of batches run at 6.77 us but its worst 20% at 25.90 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7122.4 ns median (-31.7% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 5.40x (fastest 7122.4 ns, slowest 38452.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 1 |
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
| head_tail | 10956ns | 10987ns | 7813ns | 10446ns | 13292ns | base |
| head_tail_dyn | 12357ns | 11534ns | 9649ns | 11332ns | 15247ns | +12.79% |
| kernel_nway | 12361ns | 12614ns | 9866ns | 12002ns | 14146ns | +12.83% |
| kernel_seq | 13469ns | 13643ns | 10797ns | 13054ns | 15427ns | +22.94% |
| nway | 36029ns | 35489ns | 29719ns | 35015ns | 40706ns | +228.86% |
| sequential | 14349ns | 7256ns | 6913ns | 8765ns | 26442ns | +30.97% |
| std_parallel | 39765ns | 39096ns | 31399ns | 37570ns | 47241ns | +262.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 10233ns | 7331ns | 12313ns | base | 0.006 |
| head_tail_dyn | 11563ns | 9062ns | 14074ns | +13.00% | 0.006 |
| kernel_nway | 11572ns | 9338ns | 13135ns | +13.08% | 0.006 |
| kernel_seq | 12696ns | 10271ns | 14447ns | +24.07% | 0.005 |
| nway | 35452ns | 29292ns | 39962ns | +246.44% | 0.002 |
| sequential | 14062ns | 6774ns | 25900ns | +37.41% | 0.005 |
| std_parallel | 39020ns | 30969ns | 46264ns | +281.31% | 0.002 |

## Performance model

- Peak throughput: **0.009 Gops/s** (sequential; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.006 | 65.0% |
| head_tail_dyn | 0.006 | 61.7% |
| kernel_nway | 0.005 | 57.5% |
| kernel_seq | 0.005 | 52.3% |
| nway | 0.002 | 19.4% |
| sequential | 0.009 | 95.1% |
| std_parallel | 0.002 | 17.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 10956ns | 10956ns | base |
| head_tail_dyn | 12357ns | 12357ns | +12.79% |
| kernel_nway | 12361ns | 12361ns | +12.83% |
| kernel_seq | 13469ns | 13469ns | +22.94% |
| nway | 36029ns | 36029ns | +228.86% |
| sequential | 14349ns | 14349ns | +30.97% |
| std_parallel | 39765ns | 39765ns | +262.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 10429ns | base | --- | [8943, 11589] | --- | --- | --- | --- |
| head_tail_dyn | 10972ns | no significant difference | [-792, +3136]ns | [10123, 13107] | no | 0.3568 | 0.2379 | 0 |
| kernel_nway | 11784ns | no significant difference | [-819, +4611]ns | [10641, 12478] | no | 0.8145 | 0.8145 | 0 |
| kernel_seq | 12947ns | +2440.9ns (+23.4%) | [+534, +4362]ns | [11457, 13673] | YES | 0.0151 | 0.0075 | 0 |
| nway | 34976ns | +25595.8ns (+245.4%) | [+22958, +26732]ns | [32915, 37628] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 7122ns | no significant difference | [-3162, +12874]ns | [6872, 22162] | no | 0.8145 | 0.8145 | 0 |
| std_parallel | 38453ns | +28819.2ns (+276.3%) | [+24017, +31317]ns | [34467, 41905] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 12036ns | +5.9% | -11.6% | +20.3% | +197.5% | -32.7% | +248.2% |
| 1 | 2 | 7797ns | +51.0% | +70.0% | +72.8% | +319.8% | +70.9% | +393.2% |
| 1 | 3 | 10747ns | -9.2% | +12.3% | +0.0% | +248.6% | -35.6% | +195.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.608 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.026 | ok |
| kernel_nway | -0.468 | moderate- |
| kernel_seq | -0.044 | ok |
| nway | -0.587 | HIGH- (thermal bounce) |
| sequential | -0.647 | HIGH- (thermal bounce) |
| std_parallel | -0.022 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 10590.5ns | 10233.1ns | 103.5% | HIGH |
| head_tail_dyn | 11866.9ns | 11563.3ns | 102.6% | HIGH |
| kernel_nway | 11705.9ns | 11571.9ns | 101.2% | HIGH |
| kernel_seq | 13182.7ns | 12695.9ns | 103.8% | HIGH |
| nway | 36956.3ns | 35451.9ns | 104.2% | HIGH |
| sequential | 14382.4ns | 14061.6ns | 102.3% | HIGH |
| std_parallel | 39391.2ns | 39019.7ns | 101.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 7330.5-12313.1 ns)
   7330.5 |
   7579.7 |####################
   7828.8 |####################
   8077.9 |
   8327.0 |
   8576.2 |####################
   8825.3 |
   9074.4 |####################
   9323.5 |
   9572.7 |
   9821.8 |####################
  10070.9 |####################
  10320.1 |########################################
  10569.2 |####################
  10818.3 |####################
  11067.4 |
  11316.6 |####################
  11565.7 |####################
  11814.8 |####################
  12063.9 |########################################
  (2 below, 1 above range)

head_tail_dyn (n=18, range 9061.7-14074.1 ns)
   9061.7 |
   9312.3 |
   9562.9 |########################################
   9813.5 |####################
  10064.1 |####################
  10314.8 |####################
  10565.4 |####################
  10816.0 |####################
  11066.6 |####################
  11317.2 |
  11567.9 |########################################
  11818.5 |
  12069.1 |
  12319.7 |
  12570.3 |
  12821.0 |####################
  13071.6 |####################
  13322.2 |
  13572.8 |
  13823.4 |####################
  (2 below, 3 above range)

kernel_nway (n=18, range 9338.4-13134.6 ns)
   9338.4 |
   9528.2 |########################################
   9718.0 |
   9907.9 |####################
  10097.7 |
  10287.5 |####################
  10477.3 |
  10667.1 |
  10856.9 |####################
  11046.7 |####################
  11236.5 |####################
  11426.3 |
  11616.1 |####################
  11805.9 |########################################
  11995.7 |####################
  12185.5 |####################
  12375.3 |
  12565.1 |####################
  12755.0 |
  12944.8 |####################
  (1 below, 3 above range)

kernel_seq (n=18, range 10270.5-14446.5 ns)
  10270.5 |
  10479.3 |####################
  10688.1 |
  10896.9 |####################
  11105.7 |####################
  11314.5 |
  11523.3 |####################
  11732.1 |
  11940.9 |####################
  12149.7 |####################
  12358.5 |
  12567.3 |
  12776.1 |####################
  12984.9 |####################
  13193.7 |
  13402.5 |########################################
  13611.3 |########################################
  13820.1 |####################
  14028.9 |
  14237.7 |
  (2 below, 3 above range)

nway (n=18, range 29291.9-39962.1 ns)
  29291.9 |
  29825.4 |
  30358.9 |
  30892.4 |
  31425.9 |#############
  31959.4 |##########################
  32492.9 |
  33026.5 |
  33560.0 |########################################
  34093.5 |#############
  34627.0 |
  35160.5 |#############
  35694.0 |
  36227.5 |
  36761.0 |#############
  37294.5 |########################################
  37828.0 |
  38361.5 |
  38895.1 |##########################
  39428.6 |
  (2 below, 2 above range)

sequential (n=18, range 6774.2-25899.5 ns)
   6774.2 |########################################
   7730.5 |
   8686.8 |#####
   9643.0 |
  10599.3 |
  11555.6 |
  12511.8 |
  13468.1 |
  14424.4 |
  15380.6 |
  16336.9 |
  17293.1 |
  18249.4 |
  19205.7 |#####
  20161.9 |
  21118.2 |#####
  22074.5 |
  23030.7 |
  23987.0 |#####
  24943.3 |#####
  (2 below, 3 above range)

std_parallel (n=18, range 30969.1-46263.9 ns)
  30969.1 |##########################
  31733.8 |#############
  32498.6 |#############
  33263.3 |
  34028.1 |
  34792.8 |
  35557.5 |#############
  36322.3 |##########################
  37087.0 |
  37851.8 |#############
  38616.5 |#############
  39381.2 |
  40146.0 |########################################
  40910.7 |
  41675.5 |
  42440.2 |#############
  43204.9 |#############
  43969.7 |
  44734.4 |
  45499.2 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=103.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=103.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=99.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=102.2% of algo (FFI overhead may distort results)
- **nway**: bridge=105.1% of algo (FFI overhead may distort results)
- **sequential**: CV=63.7% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **sequential**: bridge=101.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=102.1% of algo (FFI overhead may distort results)
