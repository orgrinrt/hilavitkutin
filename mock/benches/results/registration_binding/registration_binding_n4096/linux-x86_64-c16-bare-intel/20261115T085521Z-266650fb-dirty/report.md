# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 8.9x slower than the field

reg_bound_record (5.53 us) is 8.9x the fastest (621 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### reg_typed is fastest but the noisiest (CV 10.8%)

reg_typed wins on median (621 ns) yet has the highest variance (CV 10.8%), while reg_typed_twin is the steadiest (CV 7.3%, 621 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (reg_typed, reg_typed_twin) are a dead heat (<1%)

reg_typed (621 ns) and reg_typed_twin (621 ns) differ by 0.00%, inside the noise, even though the wider field spreads 791.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### reg_bound_strided shows alternating (throttle bounce) (autocorr -0.67)

reg_bound_strided's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (reg_typed)

The baseline reg_typed is the fastest (621 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {reg_typed, reg_typed_twin, reg_bound_units, reg_bound_columns, reg_bound_strided} vs {reg_bound_record} (387% apart)

The field splits into a fast tier {reg_typed, reg_typed_twin, reg_bound_units, reg_bound_columns, reg_bound_strided} and a slow tier {reg_bound_record} with a 387% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.9x the fastest

Fastest reg_typed (621 ns) to slowest reg_bound_record (5.53 us): 8.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader reg_typed vs stability leader reg_typed_twin (+0% speed for 1.5x steadier)

reg_typed is fastest (621 ns, CV 10.8%); reg_typed_twin gives up 0.0% median for 1.5x lower variance (CV 7.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### reg_bound_columns's edge over baseline is significant but tiny (7 ns, 1.19%)

reg_bound_columns differs from baseline reg_typed by 7 ns (1.19%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (reg_typed) is the fastest** at 620.5 ns median
- 4 variants significantly slower than baseline
- Spread: 8.91x (fastest 620.5 ns, slowest 5530.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Registration: typed binding vs a run-time binding table over a column workload |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x71c40e2b95a3d86f |
| Counter frequency | 2112866659 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| reg_bound_columns | 2480ns | 2642ns | 2164ns | 2519ns | 2681ns | +5.63% |
| reg_bound_record | 6946ns | 7437ns | 6089ns | 7056ns | 7474ns | +195.84% |
| reg_bound_strided | 2904ns | 2685ns | 2671ns | 2855ns | 3287ns | +23.70% |
| reg_bound_units | 2298ns | 2175ns | 2167ns | 2219ns | 2665ns | -2.14% |
| reg_typed | 2348ns | 2171ns | 2159ns | 2308ns | 2658ns | base |
| reg_typed_twin | 2228ns | 2164ns | 2158ns | 2165ns | 2486ns | -5.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 716ns | 625ns | 774ns | +6.51% | 5.721 |
| reg_bound_record | 5163ns | 4528ns | 5552ns | +668.17% | 0.793 |
| reg_bound_strided | 1229ns | 1131ns | 1389ns | +82.78% | 3.334 |
| reg_bound_units | 665ns | 627ns | 771ns | -1.12% | 6.163 |
| reg_typed | 672ns | 619ns | 760ns | base | 6.094 |
| reg_typed_twin | 639ns | 619ns | 711ns | -4.99% | 6.414 |

## Performance model

- Peak throughput: **6.613 Gops/s** (reg_typed_twin; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 5.380 | 81.4% |
| reg_bound_record | 0.741 | 11.2% |
| reg_bound_strided | 3.607 | 54.6% |
| reg_bound_units | 6.511 | 98.5% |
| reg_typed | 6.601 | 99.8% |
| reg_typed_twin | 6.601 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 2480ns | 2480ns | +5.63% |
| reg_bound_record | 6946ns | 6946ns | +195.84% |
| reg_bound_strided | 2904ns | 2904ns | +23.70% |
| reg_bound_units | 2298ns | 2298ns | -2.14% |
| reg_typed | 2348ns | 2348ns | base |
| reg_typed_twin | 2228ns | 2228ns | -5.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 621ns | base | --- | [620, 621] | --- | --- | --- | --- |
| reg_bound_columns | 761ns | +7.4ns (+1.2%) | [+7, +13]ns | [760, 763] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_record | 5530ns | +4774.7ns (+769.4%) | [+4770, +4785]ns | [5528, 5532] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 1135ns | +514.4ns (+82.9%) | [+514, +520]ns | [1134, 1142] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 629ns | +8.4ns (+1.3%) | [+8, +9]ns | [629, 630] | YES | 0.0000 | 0.0000 | 0 |
| reg_typed_twin | 621ns | no significant difference | [-0, +0]ns | [620, 621] | no | 0.3351 | 0.3351 | 5 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 620ns | +1.1% | +793.0% | +82.7% | +1.4% | +0.0% |
| 1 | 2 | 620ns | +1.1% | +793.5% | +82.8% | +1.4% | +0.0% |
| 1 | 3 | 620ns | +23.1% | +793.1% | +123.2% | +1.3% | +0.0% |
| 1 | 4 | 620ns | +23.6% | +633.5% | +82.9% | +1.4% | +0.2% |
| 1 | 5 | 758ns | -17.6% | +629.7% | +82.8% | +1.4% | -18.2% |
| 1 | 6 | 620ns | +23.2% | +630.5% | +82.6% | +1.4% | +0.1% |
| 1 | 7 | 759ns | +0.7% | +497.3% | +82.7% | +1.3% | -0.1% |
| 1 | 8 | 759ns | +0.7% | +629.9% | +49.3% | -17.2% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | 0.049 | ok |
| reg_bound_record | -0.130 | ok |
| reg_bound_strided | -0.675 | HIGH- (thermal bounce) |
| reg_bound_units | -0.376 | moderate- |
| reg_typed | 0.060 | ok |
| reg_typed_twin | -0.162 | ok |

**Consistency summary:**

- **reg_bound_columns**: won 1/8, lost 7/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 1/8, lost 7/8
- **reg_typed_twin**: won 2/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 9193.6ns | 715.9ns | 1284.2% | HIGH |
| reg_bound_record | 9144.0ns | 5163.5ns | 177.1% | HIGH |
| reg_bound_strided | 8695.6ns | 1228.6ns | 707.8% | HIGH |
| reg_bound_units | 8472.3ns | 664.6ns | 1274.7% | HIGH |
| reg_typed | 8685.4ns | 672.2ns | 1292.1% | HIGH |
| reg_typed_twin | 8246.2ns | 638.6ns | 1291.3% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 624.7-774.0 ns)
    624.7 |####################
    632.2 |##
    639.6 |
    647.1 |
    654.6 |
    662.0 |
    669.5 |
    677.0 |
    684.4 |
    691.9 |
    699.4 |
    706.8 |
    714.3 |
    721.8 |
    729.2 |
    736.7 |
    744.2 |
    751.6 |###
    759.1 |########################################
    766.6 |######
  (12 below, 4 above range)

reg_bound_record (n=160, range 4528.2-5552.0 ns)
   4528.2 |####################
   4579.4 |
   4630.6 |
   4681.7 |
   4732.9 |
   4784.1 |
   4835.3 |
   4886.5 |
   4937.7 |
   4988.9 |
   5040.1 |
   5091.3 |
   5142.5 |
   5193.7 |
   5244.9 |
   5296.1 |
   5347.3 |
   5398.5 |
   5449.7 |
   5500.9 |########################################
  (16 below, 14 above range)

reg_bound_strided (n=160, range 1130.6-1389.0 ns)
   1130.6 |########################################
   1143.5 |#
   1156.5 |
   1169.4 |
   1182.3 |
   1195.2 |
   1208.1 |
   1221.1 |
   1234.0 |
   1246.9 |
   1259.8 |
   1272.7 |
   1285.7 |
   1298.6 |
   1311.5 |
   1324.4 |
   1337.3 |
   1350.3 |
   1363.2 |
   1376.1 |#########################
  (13 below, 6 above range)

reg_bound_units (n=160, range 627.2-770.7 ns)
    627.2 |########################################
    634.3 |#####
    641.5 |
    648.7 |
    655.9 |
    663.0 |
    670.2 |
    677.4 |
    684.6 |
    691.8 |
    698.9 |
    706.1 |
    713.3 |
    720.5 |
    727.7 |
    734.8 |
    742.0 |
    749.2 |
    756.4 |
    763.6 |##############
  (12 below, 7 above range)

reg_typed (n=160, range 619.4-759.8 ns)
    619.4 |########################################
    626.4 |
    633.5 |
    640.5 |
    647.5 |
    654.5 |
    661.5 |
    668.6 |
    675.6 |
    682.6 |
    689.6 |
    696.6 |
    703.7 |
    710.7 |
    717.7 |
    724.7 |
    731.7 |
    738.7 |
    745.8 |
    752.8 |#########################
  (12 below, 5 above range)

reg_typed_twin (n=160, range 619.4-711.2 ns)
    619.4 |########################################
    624.0 |
    628.6 |####
    633.2 |
    637.8 |
    642.4 |
    647.0 |
    651.6 |
    656.1 |
    660.7 |
    665.3 |
    669.9 |
    674.5 |
    679.1 |
    683.7 |
    688.3 |
    692.9 |
    697.5 |
    702.1 |
    706.6 |
  (14 below, 20 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=1288.4% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=177.2% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=707.0% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=1276.0% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=1292.3% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=1292.1% of algo (FFI overhead may distort results)
