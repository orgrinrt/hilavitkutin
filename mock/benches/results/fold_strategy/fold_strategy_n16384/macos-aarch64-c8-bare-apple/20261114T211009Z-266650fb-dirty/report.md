# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 83% faster than the next best (fold_paired)

fold_quad (704 ns) leads fold_paired (1.29 us) by 83%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 46% (significant)

fold_quad is -586 ns (46%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 3.8x slower than the field

fold_sequential (2.68 us) is 3.8x the fastest (704 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### fold_paired shows alternating (throttle bounce) (autocorr -0.66)

fold_paired's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.8x the fastest

Fastest fold_quad (704 ns) to slowest fold_sequential (2.68 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: fold_quad** at 704.2 ns median (-45.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.81x (fastest 704.2 ns, slowest 2683.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x239fb80547ae61c2 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fold_paired | 3477ns | 3405ns | 3344ns | 3450ns | 3691ns | base |
| fold_quad | 2946ns | 2924ns | 2825ns | 2926ns | 3129ns | -15.26% |
| fold_sequential | 4823ns | 4921ns | 4594ns | 4831ns | 5026ns | +38.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 1317ns | 1266ns | 1400ns | base | 12.437 |
| fold_quad | 710ns | 678ns | 756ns | -46.13% | 23.088 |
| fold_sequential | 2633ns | 2507ns | 2744ns | +99.85% | 6.223 |

## Performance model

- Peak throughput: **24.157 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 12.746 | 52.8% |
| fold_quad | 23.266 | 96.3% |
| fold_sequential | 6.106 | 25.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 3477ns | 3477ns | base |
| fold_quad | 2946ns | 2946ns | -15.26% |
| fold_sequential | 4823ns | 4823ns | +38.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 1285ns | base | --- | [1273, 1319] | --- | --- | --- | --- |
| fold_quad | 704ns | -586.0ns (-45.6%) | [-602, -568]ns | [702, 706] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 2683ns | +1323.1ns (+102.9%) | [+1289, +1327]ns | [2626, 2685] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 1312ns | -46.0% | +97.8% |
| 1 | 2 | 1271ns | -44.6% | +111.5% |
| 1 | 3 | 1330ns | -48.3% | +88.7% |
| 1 | 4 | 1359ns | -48.1% | +97.7% |
| 1 | 5 | 1271ns | -43.7% | +97.5% |
| 1 | 6 | 1362ns | -49.4% | +97.3% |
| 1 | 7 | 1269ns | -44.5% | +111.6% |
| 1 | 8 | 1359ns | -48.1% | +97.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.656 | HIGH- (thermal bounce) |
| fold_quad | -0.337 | moderate- |
| fold_sequential | -0.506 | HIGH- (thermal bounce) |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 3.1ns | 1317.3ns | 0.2% |  |
| fold_quad | 3.3ns | 709.6ns | 0.5% |  |
| fold_sequential | 3.2ns | 2632.7ns | 0.1% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 1266.4-1400.1 ns)
   1266.4 |########################################
   1273.1 |#####
   1279.8 |##
   1286.5 |#
   1293.2 |##
   1299.9 |##
   1306.5 |
   1313.2 |##
   1319.9 |##
   1326.6 |
   1333.3 |
   1340.0 |
   1346.6 |
   1353.3 |#################
   1360.0 |####################
   1366.7 |#
   1373.4 |
   1380.1 |
   1386.8 |
   1393.4 |
  (15 below, 11 above range)

fold_quad (n=160, range 678.2-756.3 ns)
    678.2 |#####
    682.1 |##
    686.0 |###
    689.9 |#
    693.8 |#######
    697.7 |################################
    701.6 |########################################
    705.5 |##################################
    709.4 |#####
    713.3 |##
    717.3 |#
    721.2 |#
    725.1 |#
    729.0 |#
    732.9 |#
    736.8 |
    740.7 |
    744.6 |#
    748.5 |#
    752.4 |####
  (13 below, 21 above range)

fold_sequential (n=160, range 2506.6-2744.5 ns)
   2506.6 |##################
   2518.5 |
   2530.4 |###
   2542.3 |#
   2554.2 |###
   2566.1 |
   2578.0 |
   2589.9 |###
   2601.8 |##
   2613.6 |####
   2625.5 |##
   2637.4 |
   2649.3 |
   2661.2 |##
   2673.1 |########
   2685.0 |########################################
   2696.9 |##
   2708.8 |
   2720.7 |
   2732.6 |
  (14 below, 9 above range)

```
