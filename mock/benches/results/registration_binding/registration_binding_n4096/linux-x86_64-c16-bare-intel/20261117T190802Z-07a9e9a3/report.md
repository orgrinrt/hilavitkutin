# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 7.4x slower than the field

reg_bound_record (4.60 us) is 7.4x the fastest (618 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_typed, reg_typed_twin) are a dead heat (<1%)

reg_typed (618 ns) and reg_typed_twin (622 ns) differ by 0.62%, inside the noise, even though the wider field spreads 644.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### reg_bound_columns shows warm-up / thermal drift (autocorr +0.62)

reg_bound_columns's per-pass series has lag-1 autocorrelation +0.62, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (reg_typed)

The baseline reg_typed is the fastest (618 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {reg_typed, reg_typed_twin, reg_bound_units, reg_bound_columns, reg_bound_strided} vs {reg_bound_record} (305% apart)

The field splits into a fast tier {reg_typed, reg_typed_twin, reg_bound_units, reg_bound_columns, reg_bound_strided} and a slow tier {reg_bound_record} with a 305% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.4x the fastest

Fastest reg_typed (618 ns) to slowest reg_bound_record (4.60 us): 7.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### reg_bound_columns is inconsistent: worst-20% is 1.6x its best-20%

reg_bound_columns's best 20% of batches run at 625 ns but its worst 20% at 1.02 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### reg_bound_columns's edge over baseline is significant but tiny (11 ns, 1.82%)

reg_bound_columns differs from baseline reg_typed by 11 ns (1.82%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (reg_typed) is the fastest** at 618.0 ns median
- 5 variants significantly slower than baseline
- Spread: 7.44x (fastest 618.0 ns, slowest 4600.8 ns)

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
| Counter frequency | 2112819991 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| reg_bound_columns | 2590ns | 2442ns | 2168ns | 2416ns | 3534ns | +15.74% |
| reg_bound_record | 6483ns | 6198ns | 6094ns | 6283ns | 7471ns | +189.71% |
| reg_bound_strided | 2903ns | 2684ns | 2670ns | 2854ns | 3285ns | +29.75% |
| reg_bound_units | 2242ns | 2169ns | 2163ns | 2170ns | 2536ns | +0.18% |
| reg_typed | 2238ns | 2162ns | 2157ns | 2164ns | 2540ns | base |
| reg_typed_twin | 2353ns | 2173ns | 2160ns | 2313ns | 2667ns | +5.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 747ns | 625ns | 1019ns | +16.96% | 5.481 |
| reg_bound_record | 4826ns | 4528ns | 5551ns | +655.24% | 0.849 |
| reg_bound_strided | 1225ns | 1124ns | 1387ns | +91.75% | 3.343 |
| reg_bound_units | 647ns | 625ns | 729ns | +1.20% | 6.334 |
| reg_typed | 639ns | 617ns | 724ns | base | 6.410 |
| reg_typed_twin | 674ns | 619ns | 762ns | +5.53% | 6.074 |

## Performance model

- Peak throughput: **6.643 Gops/s** (reg_typed; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 5.700 | 85.8% |
| reg_bound_record | 0.890 | 13.4% |
| reg_bound_strided | 3.606 | 54.3% |
| reg_bound_units | 6.540 | 98.5% |
| reg_typed | 6.628 | 99.8% |
| reg_typed_twin | 6.587 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 2590ns | 2590ns | +15.74% |
| reg_bound_record | 6483ns | 6483ns | +189.71% |
| reg_bound_strided | 2903ns | 2903ns | +29.75% |
| reg_bound_units | 2242ns | 2242ns | +0.18% |
| reg_typed | 2238ns | 2238ns | base |
| reg_typed_twin | 2353ns | 2353ns | +5.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 618ns | base | --- | [618, 618] | --- | --- | --- | --- |
| reg_bound_columns | 719ns | +11.2ns (+1.8%) | [+10, +13]ns | [629, 765] | YES | 0.0000 | 0.0000 | 1 |
| reg_bound_record | 4601ns | +3942.0ns (+637.9%) | [+3933, +4030]ns | [4553, 4651] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 1136ns | +515.3ns (+83.4%) | [+513, +522]ns | [1133, 1141] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 626ns | +8.4ns (+1.4%) | [+8, +9]ns | [626, 626] | YES | 0.0000 | 0.0000 | 0 |
| reg_typed_twin | 622ns | +3.7ns (+0.6%) | [+3, +4]ns | [621, 631] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 618ns | +1.3% | +795.2% | +123.5% | +1.4% | +22.8% |
| 1 | 2 | 618ns | +1.5% | +636.0% | +82.6% | +1.3% | +0.5% |
| 1 | 3 | 618ns | +1.5% | +633.1% | +83.1% | +1.4% | +1.2% |
| 1 | 4 | 618ns | +1.4% | +798.6% | +123.4% | +1.3% | +22.8% |
| 1 | 5 | 618ns | +24.0% | +662.0% | +83.4% | +1.3% | +0.5% |
| 1 | 6 | 617ns | +24.0% | +652.8% | +83.2% | +24.1% | +0.5% |
| 1 | 7 | 756ns | +1.4% | +499.5% | +49.4% | -17.1% | +0.4% |
| 1 | 8 | 618ns | +24.2% | +635.8% | +123.5% | +1.3% | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | 0.622 | HIGH+ (drift/warm-up) |
| reg_bound_record | -0.185 | ok |
| reg_bound_strided | -0.276 | moderate- |
| reg_bound_units | -0.163 | ok |
| reg_typed | -0.165 | ok |
| reg_typed_twin | -0.476 | moderate- |

**Consistency summary:**

- **reg_bound_columns**: won 0/8, lost 8/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 1/8, lost 7/8
- **reg_typed_twin**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 9557.4ns | 747.4ns | 1278.8% | HIGH |
| reg_bound_record | 8473.2ns | 4826.0ns | 175.6% | HIGH |
| reg_bound_strided | 8697.7ns | 1225.3ns | 709.8% | HIGH |
| reg_bound_units | 8279.1ns | 646.6ns | 1280.3% | HIGH |
| reg_typed | 8282.7ns | 639.0ns | 1296.2% | HIGH |
| reg_typed_twin | 8697.0ns | 674.4ns | 1289.7% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 625.4-1018.7 ns)
    625.4 |####################################
    645.0 |
    664.7 |
    684.4 |
    704.0 |
    723.7 |
    743.4 |#
    763.1 |########################################
    782.7 |
    802.4 |
    822.1 |
    841.7 |
    861.4 |
    881.1 |
    900.7 |
    920.4 |
    940.1 |
    959.7 |
    979.4 |
    999.1 |
  (11 below, 2 above range)

reg_bound_record (n=160, range 4528.1-5550.8 ns)
   4528.1 |########################################
   4579.2 |
   4630.4 |############
   4681.5 |############
   4732.7 |
   4783.8 |
   4834.9 |
   4886.1 |
   4937.2 |
   4988.3 |
   5039.5 |
   5090.6 |
   5141.7 |
   5192.9 |
   5244.0 |
   5295.1 |
   5346.3 |
   5397.4 |
   5448.5 |
   5499.7 |###############
  (17 below, 15 above range)

reg_bound_strided (n=160, range 1124.4-1387.2 ns)
   1124.4 |########################################
   1137.5 |########
   1150.7 |
   1163.8 |
   1176.9 |
   1190.1 |
   1203.2 |
   1216.4 |
   1229.5 |
   1242.6 |
   1255.8 |
   1268.9 |
   1282.1 |
   1295.2 |
   1308.3 |
   1321.5 |
   1334.6 |
   1347.7 |
   1360.9 |###
   1374.0 |#######################
  (13 below, 12 above range)

reg_bound_units (n=160, range 625.1-728.9 ns)
    625.1 |########################################
    630.3 |
    635.5 |#
    640.7 |
    645.9 |
    651.0 |
    656.2 |
    661.4 |
    666.6 |
    671.8 |
    677.0 |
    682.2 |
    687.4 |
    692.5 |
    697.7 |
    702.9 |
    708.1 |
    713.3 |
    718.5 |
    723.7 |
  (16 below, 21 above range)

reg_typed (n=160, range 616.6-723.8 ns)
    616.6 |########################################
    622.0 |#
    627.3 |#
    632.7 |
    638.1 |
    643.4 |
    648.8 |
    654.1 |
    659.5 |
    664.8 |
    670.2 |
    675.6 |
    680.9 |
    686.3 |
    691.6 |
    697.0 |
    702.3 |
    707.7 |
    713.1 |
    718.4 |
  (10 below, 21 above range)

reg_typed_twin (n=160, range 619.3-762.1 ns)
    619.3 |########################################
    626.4 |##########
    633.6 |
    640.7 |
    647.8 |
    655.0 |
    662.1 |
    669.3 |
    676.4 |
    683.6 |
    690.7 |
    697.8 |
    705.0 |
    712.1 |
    719.3 |
    726.4 |
    733.6 |
    740.7 |
    747.8 |
    755.0 |##################################
  (17 below, 5 above range)

```

## Diagnostics

- **reg_bound_columns**: CV=61.1% (high variance, measurements may be unstable)
- **reg_bound_columns**: autocorrelation=0.62 (measurement drift or warm-up artifact)
- **reg_bound_columns**: bridge=1243.7% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=174.5% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=707.0% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=1281.6% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=1297.1% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=1289.7% of algo (FFI overhead may distort results)
