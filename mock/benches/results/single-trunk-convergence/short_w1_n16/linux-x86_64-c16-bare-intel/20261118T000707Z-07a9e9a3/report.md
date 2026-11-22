# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 29% faster than the next best (head_tail_dyn)

sequential (8.48 us) leads head_tail_dyn (10.90 us) by 29%, a clear separation rather than a photo finish. CV 96.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### head_tail_dyn beats baseline by 35% (significant)

head_tail_dyn is -5.40 us (35%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 4.4x slower than the field

std_parallel (37.02 us) is 4.4x the fastest (8.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 96.8%)

sequential wins on median (8.48 us) yet has the highest variance (CV 96.8%), while kernel_seq is the steadiest (CV 12.5%, 11.80 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail shows alternating (throttle bounce) (autocorr -0.66)

head_tail's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, head_tail_dyn, kernel_seq, kernel_nway, head_tail} vs {nway, std_parallel} (124% apart)

The field splits into a fast tier {sequential, head_tail_dyn, kernel_seq, kernel_nway, head_tail} and a slow tier {nway, std_parallel} with a 124% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.4x the fastest

Fastest sequential (8.48 us) to slowest std_parallel (37.02 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.6x its best-20%

sequential's best 20% of batches run at 6.79 us but its worst 20% at 24.09 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 8478.0 ns median (-44.7% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 4.37x (fastest 8478.0 ns, slowest 37020.6 ns)

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
| head_tail | 16809ns | 16091ns | 14092ns | 15891ns | 19545ns | base |
| head_tail_dyn | 11902ns | 11486ns | 8096ns | 10912ns | 15291ns | -29.19% |
| kernel_nway | 14017ns | 14021ns | 7562ns | 12493ns | 19529ns | -16.61% |
| kernel_seq | 12569ns | 12508ns | 10063ns | 12154ns | 14443ns | -25.23% |
| nway | 35870ns | 34905ns | 28489ns | 34550ns | 41540ns | +113.39% |
| sequential | 14021ns | 8649ns | 6920ns | 9333ns | 24605ns | -16.59% |
| std_parallel | 39343ns | 37557ns | 32429ns | 36495ns | 47073ns | +134.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 15889ns | 13415ns | 18349ns | base | 0.001 |
| head_tail_dyn | 11184ns | 7653ns | 14323ns | -29.61% | 0.001 |
| kernel_nway | 13188ns | 7158ns | 18459ns | -17.00% | 0.001 |
| kernel_seq | 11828ns | 9545ns | 13467ns | -25.55% | 0.001 |
| nway | 35309ns | 28093ns | 40877ns | +122.23% | 0.000 |
| sequential | 13737ns | 6786ns | 24092ns | -13.54% | 0.001 |
| std_parallel | 38673ns | 31949ns | 46186ns | +143.40% | 0.000 |

## Performance model

- Peak throughput: **0.002 Gops/s** (sequential; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.001 | 44.3% |
| head_tail_dyn | 0.001 | 62.2% |
| kernel_nway | 0.001 | 51.5% |
| kernel_seq | 0.001 | 57.5% |
| nway | 0.000 | 19.7% |
| sequential | 0.002 | 80.0% |
| std_parallel | 0.000 | 18.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 16809ns | 16809ns | base |
| head_tail_dyn | 11902ns | 11902ns | -29.19% |
| kernel_nway | 14017ns | 14017ns | -16.61% |
| kernel_seq | 12569ns | 12569ns | -25.23% |
| nway | 35870ns | 35870ns | +113.39% |
| sequential | 14021ns | 14021ns | -16.59% |
| std_parallel | 39343ns | 39343ns | +134.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 15332ns | base | --- | [14581, 16617] | --- | --- | --- | --- |
| head_tail_dyn | 10902ns | -5397.4ns (-35.2%) | [-6646, -3119]ns | [9498, 11629] | YES | 0.0020 | 0.0013 | 0 |
| kernel_nway | 13166ns | no significant difference | [-6824, +2921]ns | [9164, 16764] | no | 0.2855 | 0.2379 | 0 |
| kernel_seq | 11799ns | -4063.4ns (-26.5%) | [-5209, -1606]ns | [11075, 12644] | YES | 0.0000 | 0.0000 | 0 |
| nway | 34404ns | +18946.2ns (+123.6%) | [+16941, +21651]ns | [33104, 37504] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 8478ns | no significant difference | [-7673, +3775]ns | [7023, 19788] | no | 0.4807 | 0.4807 | 0 |
| std_parallel | 37021ns | +20079.6ns (+131.0%) | [+17636, +26776]ns | [33581, 43121] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 14993ns | -44.1% | -49.1% | -28.2% | +111.1% | +10.8% | +126.4% |
| 1 | 2 | 16801ns | -31.5% | -21.6% | -32.5% | +100.1% | -57.5% | +103.6% |
| 1 | 3 | 15167ns | -18.3% | +19.3% | -14.1% | +142.2% | -21.5% | +207.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.662 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.063 | ok |
| kernel_nway | -0.001 | ok |
| kernel_seq | -0.051 | ok |
| nway | -0.011 | ok |
| sequential | -0.502 | HIGH- (thermal bounce) |
| std_parallel | -0.156 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 16242.5ns | 15888.7ns | 102.2% | HIGH |
| head_tail_dyn | 11320.2ns | 11183.8ns | 101.2% | HIGH |
| kernel_nway | 13483.9ns | 13188.2ns | 102.2% | HIGH |
| kernel_seq | 12181.1ns | 11828.4ns | 103.0% | HIGH |
| nway | 36330.3ns | 35308.8ns | 102.9% | HIGH |
| sequential | 14193.6ns | 13736.7ns | 103.3% | HIGH |
| std_parallel | 38934.9ns | 38673.0ns | 100.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 13414.6-18349.3 ns)
  13414.6 |
  13661.3 |
  13908.1 |#############
  14154.8 |##########################
  14401.5 |
  14648.3 |#############
  14895.0 |
  15141.8 |########################################
  15388.5 |##########################
  15635.2 |#############
  15882.0 |
  16128.7 |
  16375.4 |#############
  16622.2 |#############
  16868.9 |##########################
  17115.6 |
  17362.4 |#############
  17609.1 |
  17855.8 |
  18102.6 |
  (2 below, 1 above range)

head_tail_dyn (n=18, range 7652.5-14322.9 ns)
   7652.5 |##########
   7986.1 |##########
   8319.6 |##########
   8653.1 |
   8986.6 |##########
   9320.1 |
   9653.7 |####################
   9987.2 |##########
  10320.7 |
  10654.2 |##########
  10987.7 |##########
  11321.3 |########################################
  11654.8 |##########
  11988.3 |
  12321.8 |
  12655.3 |
  12988.9 |##########
  13322.4 |
  13655.9 |
  13989.4 |
  (1 below, 2 above range)

kernel_nway (n=18, range 7157.5-18458.9 ns)
   7157.5 |####################
   7722.6 |####################
   8287.6 |####################
   8852.7 |
   9417.8 |
   9982.8 |####################
  10547.9 |
  11113.0 |####################
  11678.1 |####################
  12243.1 |####################
  12808.2 |
  13373.3 |########################################
  13938.3 |
  14503.4 |
  15068.5 |####################
  15633.5 |
  16198.6 |
  16763.7 |########################################
  17328.7 |
  17893.8 |########################################
  (2 below, 2 above range)

kernel_seq (n=18, range 9544.8-13467.1 ns)
   9544.8 |
   9740.9 |
   9937.0 |
  10133.1 |
  10329.2 |
  10525.3 |#############
  10721.5 |
  10917.6 |##########################
  11113.7 |#############
  11309.8 |##########################
  11505.9 |#############
  11702.0 |
  11898.1 |########################################
  12094.3 |
  12290.4 |
  12486.5 |#############
  12682.6 |#############
  12878.7 |
  13074.8 |#############
  13270.9 |
  (2 below, 3 above range)

nway (n=18, range 28092.8-40876.9 ns)
  28092.8 |
  28732.0 |
  29371.2 |
  30010.4 |####################
  30649.6 |
  31288.8 |
  31928.0 |####################
  32567.2 |####################
  33206.4 |########################################
  33845.6 |########################################
  34484.8 |########################################
  35124.1 |
  35763.3 |####################
  36402.5 |
  37041.7 |####################
  37680.9 |
  38320.1 |########################################
  38959.3 |
  39598.5 |
  40237.7 |####################
  (2 below, 2 above range)

sequential (n=18, range 6785.7-24091.7 ns)
   6785.7 |########################################
   7651.0 |
   8516.3 |
   9381.6 |#####
  10246.9 |
  11112.2 |
  11977.5 |
  12842.8 |#####
  13708.1 |
  14573.4 |
  15438.7 |
  16304.0 |#####
  17169.3 |
  18034.6 |
  18899.9 |
  19765.2 |##########
  20630.5 |
  21495.8 |#####
  22361.1 |
  23226.4 |#####
  (1 below, 2 above range)

std_parallel (n=18, range 31949.1-46185.5 ns)
  31949.1 |##########################
  32660.9 |
  33372.7 |##########################
  34084.6 |########################################
  34796.4 |
  35508.2 |
  36220.0 |
  36931.8 |
  37643.7 |
  38355.5 |
  39067.3 |##########################
  39779.1 |
  40490.9 |#############
  41202.8 |
  41914.6 |#############
  42626.4 |
  43338.2 |#############
  44050.0 |
  44761.9 |
  45473.7 |#############
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=105.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=28.4% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=105.0% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=34.2% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=101.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=103.0% of algo (FFI overhead may distort results)
- **nway**: bridge=103.1% of algo (FFI overhead may distort results)
- **sequential**: CV=59.7% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **sequential**: bridge=106.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.8% of algo (FFI overhead may distort results)
