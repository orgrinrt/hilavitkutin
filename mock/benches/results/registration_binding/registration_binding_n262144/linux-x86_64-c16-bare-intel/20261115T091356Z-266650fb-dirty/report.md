# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 6.1x slower than the field

reg_bound_record (292.09 us) is 6.1x the fastest (47.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_typed_twin, reg_typed) are a dead heat (<1%)

reg_typed_twin (47.69 us) and reg_typed (47.70 us) differ by 0.03%, inside the noise, even though the wider field spreads 512.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### reg_typed_twin shows alternating (throttle bounce) (autocorr -0.55)

reg_typed_twin's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {reg_typed_twin, reg_typed, reg_bound_units, reg_bound_columns, reg_bound_strided} vs {reg_bound_record} (303% apart)

The field splits into a fast tier {reg_typed_twin, reg_typed, reg_bound_units, reg_bound_columns, reg_bound_strided} and a slow tier {reg_bound_record} with a 303% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.1x the fastest

Fastest reg_typed_twin (47.69 us) to slowest reg_bound_record (292.09 us): 6.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### reg_typed_twin's edge over baseline is significant but tiny (-33 ns, 0.07%)

reg_typed_twin differs from baseline reg_typed by -33 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: reg_typed_twin** at 47688.3 ns median (-0.0% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 6.12x (fastest 47688.3 ns, slowest 292090.5 ns)

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
| reg_bound_columns | 50519ns | 50604ns | 44674ns | 51371ns | 53811ns | +5.97% |
| reg_bound_record | 300164ns | 293762ns | 292678ns | 293884ns | 326493ns | +529.65% |
| reg_bound_strided | 79041ns | 74168ns | 73991ns | 76657ns | 91245ns | +65.80% |
| reg_bound_units | 46728ns | 50195ns | 41090ns | 47088ns | 51286ns | -1.98% |
| reg_typed | 47672ns | 49746ns | 40724ns | 49211ns | 50002ns | base |
| reg_typed_twin | 46846ns | 49719ns | 40690ns | 47463ns | 51151ns | -1.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 48526ns | 42885ns | 51757ns | +6.15% | 5.402 |
| reg_bound_record | 298445ns | 290986ns | 324629ns | +552.85% | 0.878 |
| reg_bound_strided | 77264ns | 72330ns | 89192ns | +69.02% | 3.393 |
| reg_bound_units | 44813ns | 39398ns | 49221ns | -1.97% | 5.850 |
| reg_typed | 45714ns | 39043ns | 47943ns | base | 5.734 |
| reg_typed_twin | 44934ns | 39016ns | 49099ns | -1.71% | 5.834 |

## Performance model

- Peak throughput: **6.719 Gops/s** (reg_typed_twin; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 5.398 | 80.3% |
| reg_bound_record | 0.897 | 13.4% |
| reg_bound_strided | 3.617 | 53.8% |
| reg_bound_units | 5.448 | 81.1% |
| reg_typed | 5.495 | 81.8% |
| reg_typed_twin | 5.497 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 50519ns | 50519ns | +5.97% |
| reg_bound_record | 300164ns | 300164ns | +529.65% |
| reg_bound_strided | 79041ns | 79041ns | +65.80% |
| reg_bound_units | 46728ns | 46728ns | -1.98% |
| reg_typed | 47672ns | 47672ns | base |
| reg_typed_twin | 46846ns | 46846ns | -1.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 47704ns | base | --- | [47694, 47719] | --- | --- | --- | --- |
| reg_bound_columns | 48565ns | +838.6ns (+1.8%) | [+790, +886]ns | [48522, 48606] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_record | 292090ns | +244877.7ns (+513.3%) | [+244421, +245362]ns | [291881, 292346] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 72477ns | +31023.7ns (+65.0%) | [+30482, +33270]ns | [72431, 72544] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 48119ns | +421.9ns (+0.9%) | [+380, +448]ns | [47826, 48151] | YES | 0.0000 | 0.0000 | 0 |
| reg_typed_twin | 47688ns | -33.2ns (-0.1%) | [-71, -7]ns | [47670, 47696] | YES | 0.0325 | 0.0325 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 47704ns | +1.7% | +514.0% | +51.9% | +1.0% | +4.3% |
| 1 | 2 | 47728ns | +1.8% | +511.0% | +52.9% | -17.2% | +0.0% |
| 1 | 3 | 47715ns | +7.9% | +510.3% | +51.8% | +1.0% | -18.2% |
| 1 | 4 | 39026ns | +1.9% | +650.1% | +85.4% | +23.5% | +22.2% |
| 1 | 5 | 39068ns | +32.4% | +646.9% | +85.7% | +0.9% | -0.1% |
| 1 | 6 | 47778ns | +1.6% | +512.2% | +51.4% | -17.4% | -0.2% |
| 1 | 7 | 47723ns | +8.4% | +511.2% | +86.0% | +4.8% | -18.2% |
| 1 | 8 | 47700ns | +1.4% | +513.7% | +86.3% | +0.9% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.511 | HIGH- (thermal bounce) |
| reg_bound_record | -0.419 | moderate- |
| reg_bound_strided | 0.451 | moderate+ |
| reg_bound_units | -0.148 | ok |
| reg_typed | 0.289 | moderate+ |
| reg_typed_twin | -0.549 | HIGH- (thermal bounce) |

**Consistency summary:**

- **reg_bound_columns**: won 0/8, lost 8/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 2/8, lost 6/8
- **reg_typed_twin**: won 3/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 609442.7ns | 48526.4ns | 1255.9% | HIGH |
| reg_bound_record | 521577.5ns | 298444.7ns | 174.8% | HIGH |
| reg_bound_strided | 540804.6ns | 77264.3ns | 699.9% | HIGH |
| reg_bound_units | 578501.5ns | 44813.3ns | 1290.9% | HIGH |
| reg_typed | 597576.4ns | 45713.9ns | 1307.2% | HIGH |
| reg_typed_twin | 584493.5ns | 44933.5ns | 1300.8% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 42885.3-51756.5 ns)
  42885.3 |
  43328.9 |
  43772.4 |
  44216.0 |
  44659.6 |
  45103.1 |
  45546.7 |
  45990.2 |
  46433.8 |
  46877.4 |
  47320.9 |
  47764.5 |
  48208.0 |########################################
  48651.6 |#
  49095.2 |
  49538.7 |
  49982.3 |
  50425.9 |
  50869.4 |
  51313.0 |########################
  (21 below, 12 above range)

reg_bound_record (n=160, range 290986.4-324629.5 ns)
  290986.4 |########################################
  292668.6 |############
  294350.7 |
  296032.9 |
  297715.0 |
  299397.2 |
  301079.3 |
  302761.5 |
  304443.6 |
  306125.8 |
  307807.9 |
  309490.1 |
  311172.2 |
  312854.4 |
  314536.5 |
  316218.7 |
  317900.8 |
  319583.0 |
  321265.1 |
  322947.3 |
  (14 below, 17 above range)

reg_bound_strided (n=160, range 72330.1-89192.0 ns)
  72330.1 |########################################
  73173.2 |
  74016.3 |
  74859.4 |
  75702.5 |
  76545.6 |
  77388.7 |#
  78231.7 |#####
  79074.8 |
  79917.9 |
  80761.0 |
  81604.1 |
  82447.2 |
  83290.3 |
  84133.4 |
  84976.5 |
  85819.6 |
  86662.7 |
  87505.8 |
  88348.9 |#################
  (13 below, 2 above range)

reg_bound_units (n=160, range 39398.3-49220.7 ns)
  39398.3 |######################
  39889.4 |
  40380.5 |#
  40871.7 |
  41362.8 |##
  41853.9 |
  42345.0 |
  42836.1 |
  43327.3 |
  43818.4 |
  44309.5 |
  44800.6 |
  45291.8 |
  45782.9 |
  46274.0 |
  46765.1 |
  47256.2 |
  47747.4 |########################################
  48238.5 |##
  48729.6 |
  (16 below, 12 above range)

reg_typed (n=160, range 39043.4-47942.9 ns)
  39043.4 |#####
  39488.4 |
  39933.3 |
  40378.3 |
  40823.3 |#
  41268.3 |
  41713.2 |
  42158.2 |
  42603.2 |
  43048.2 |
  43493.1 |
  43938.1 |
  44383.1 |
  44828.1 |
  45273.0 |
  45718.0 |
  46163.0 |
  46608.0 |
  47052.9 |
  47497.9 |########################################
  (18 below, 5 above range)

reg_typed_twin (n=160, range 39016.4-49098.7 ns)
  39016.4 |##################
  39520.5 |
  40024.6 |
  40528.7 |
  41032.8 |##
  41537.0 |
  42041.1 |
  42545.2 |
  43049.3 |
  43553.4 |
  44057.5 |
  44561.6 |
  45065.8 |
  45569.9 |
  46074.0 |
  46578.1 |
  47082.2 |
  47586.3 |########################################
  48090.5 |
  48594.6 |
  (12 below, 18 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=1286.5% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=175.1% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=705.7% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=1298.7% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=1309.9% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=1310.4% of algo (FFI overhead may distort results)
