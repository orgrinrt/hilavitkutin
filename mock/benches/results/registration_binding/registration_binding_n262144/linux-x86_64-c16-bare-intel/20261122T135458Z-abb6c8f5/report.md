# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 6.1x slower than the field

reg_bound_record (292.47 us) is 6.1x the fastest (47.66 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_typed_twin, reg_typed) are a dead heat (<1%)

reg_typed_twin (47.66 us) and reg_typed (47.88 us) differ by 0.47%, inside the noise, even though the wider field spreads 513.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### reg_typed shows alternating (throttle bounce) (autocorr -0.77)

reg_typed's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {reg_typed_twin, reg_typed, reg_bound_units, reg_bound_columns, reg_bound_strided} vs {reg_bound_record} (277% apart)

The field splits into a fast tier {reg_typed_twin, reg_typed, reg_bound_units, reg_bound_columns, reg_bound_strided} and a slow tier {reg_bound_record} with a 277% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.1x the fastest

Fastest reg_typed_twin (47.66 us) to slowest reg_bound_record (292.47 us): 6.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: reg_typed_twin** at 47658.6 ns median (-0.5% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 6.14x (fastest 47658.6 ns, slowest 292468.1 ns)

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
| Counter frequency | 2112791800 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| reg_bound_columns | 47244ns | 50360ns | 41192ns | 47302ns | 53119ns | +0.56% |
| reg_bound_record | 304332ns | 294192ns | 293103ns | 294287ns | 345698ns | +547.81% |
| reg_bound_strided | 81029ns | 79297ns | 72898ns | 79082ns | 94998ns | +72.48% |
| reg_bound_units | 47702ns | 50233ns | 41177ns | 47913ns | 53591ns | +1.54% |
| reg_typed | 46978ns | 49923ns | 40944ns | 47450ns | 51598ns | base |
| reg_typed_twin | 45907ns | 49701ns | 40674ns | 45798ns | 51464ns | -2.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 45354ns | 39521ns | 51067ns | +0.64% | 5.780 |
| reg_bound_record | 302555ns | 291399ns | 343659ns | +571.39% | 0.866 |
| reg_bound_strided | 79197ns | 71197ns | 92934ns | +75.74% | 3.310 |
| reg_bound_units | 45803ns | 39501ns | 51525ns | +1.64% | 5.723 |
| reg_typed | 45064ns | 39252ns | 49539ns | base | 5.817 |
| reg_typed_twin | 44030ns | 39000ns | 49408ns | -2.30% | 5.954 |

## Performance model

- Peak throughput: **6.722 Gops/s** (reg_typed_twin; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 5.425 | 80.7% |
| reg_bound_record | 0.896 | 13.3% |
| reg_bound_strided | 3.377 | 50.2% |
| reg_bound_units | 5.438 | 80.9% |
| reg_typed | 5.475 | 81.5% |
| reg_typed_twin | 5.500 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 47244ns | 47244ns | +0.56% |
| reg_bound_record | 304332ns | 304332ns | +547.81% |
| reg_bound_strided | 81029ns | 81029ns | +72.48% |
| reg_bound_units | 47702ns | 47702ns | +1.54% |
| reg_typed | 46978ns | 46978ns | base |
| reg_typed_twin | 45907ns | 45907ns | -2.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 47882ns | base | --- | [47861, 47910] | --- | --- | --- | --- |
| reg_bound_columns | 48318ns | +618.0ns (+1.3%) | [+393, +720]ns | [44705, 48368] | YES | 0.0011 | 0.0011 | 0 |
| reg_bound_record | 292468ns | +252382.5ns (+527.1%) | [+245299, +252720]ns | [292255, 292757] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 77617ns | +32208.0ns (+67.3%) | [+30518, +38306]ns | [71987, 77712] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 48207ns | +337.5ns (+0.7%) | [+245, +493]ns | [48159, 48369] | YES | 0.0000 | 0.0000 | 0 |
| reg_typed_twin | 47659ns | -289.5ns (-0.6%) | [-378, -239]ns | [39374, 47775] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 47907ns | +8.4% | +509.0% | +49.1% | +1.0% | -0.0% |
| 1 | 2 | 47908ns | +1.4% | +509.0% | +82.9% | -17.7% | -18.5% |
| 1 | 3 | 39253ns | +23.2% | +644.4% | +122.3% | +22.8% | +27.4% |
| 1 | 4 | 51383ns | -23.0% | +470.9% | +84.3% | +0.2% | -7.2% |
| 1 | 5 | 39273ns | +23.2% | +645.0% | +81.6% | +23.4% | -0.6% |
| 1 | 6 | 48124ns | -17.9% | +631.2% | +48.9% | +0.1% | -0.4% |
| 1 | 7 | 39370ns | +1.0% | +642.5% | +97.4% | +0.7% | -0.9% |
| 1 | 8 | 47894ns | -12.8% | +508.8% | +49.6% | -10.8% | -18.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | 0.143 | ok |
| reg_bound_record | -0.154 | ok |
| reg_bound_strided | 0.144 | ok |
| reg_bound_units | 0.052 | ok |
| reg_typed | -0.766 | HIGH- (thermal bounce) |
| reg_typed_twin | -0.345 | moderate- |

**Consistency summary:**

- **reg_bound_columns**: won 3/8, lost 5/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 2/8, lost 6/8
- **reg_typed_twin**: won 6/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 577695.7ns | 45354.0ns | 1273.7% | HIGH |
| reg_bound_record | 530130.2ns | 302555.1ns | 175.2% | HIGH |
| reg_bound_strided | 551033.5ns | 79197.0ns | 695.8% | HIGH |
| reg_bound_units | 582830.9ns | 45802.8ns | 1272.5% | HIGH |
| reg_typed | 582469.4ns | 45064.2ns | 1292.5% | HIGH |
| reg_typed_twin | 570782.7ns | 44029.5ns | 1296.4% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 39520.8-51067.2 ns)
  39520.8 |########################
  40098.1 |#
  40675.4 |
  41252.7 |#####
  41830.1 |#
  42407.4 |
  42984.7 |
  43562.0 |
  44139.4 |
  44716.7 |
  45294.0 |
  45871.3 |
  46448.6 |
  47026.0 |
  47603.3 |
  48180.6 |########################################
  48757.9 |#
  49335.3 |
  49912.6 |
  50489.9 |
  (15 below, 22 above range)

reg_bound_record (n=160, range 291398.9-343659.0 ns)
  291398.9 |########################################
  294011.9 |#
  296624.9 |
  299237.9 |
  301850.9 |
  304463.9 |
  307076.9 |
  309689.9 |
  312302.9 |
  314915.9 |
  317529.0 |
  320142.0 |
  322755.0 |
  325368.0 |
  327981.0 |
  330594.0 |
  333207.0 |
  335820.0 |
  338433.0 |
  341046.0 |
  (13 below, 27 above range)

reg_bound_strided (n=160, range 71196.6-92933.7 ns)
  71196.6 |########################################
  72283.4 |
  73370.3 |
  74457.1 |
  75544.0 |
  76630.9 |############
  77717.7 |#######
  78804.6 |
  79891.4 |
  80978.3 |
  82065.1 |
  83152.0 |
  84238.8 |
  85325.7 |
  86412.5 |#############
  87499.4 |########
  88586.2 |
  89673.1 |
  90760.0 |
  91846.8 |
  (14 below, 21 above range)

reg_bound_units (n=160, range 39501.3-51524.5 ns)
  39501.3 |#################
  40102.5 |
  40703.7 |
  41304.8 |
  41906.0 |
  42507.1 |#######
  43108.3 |
  43709.5 |
  44310.6 |
  44911.8 |
  45512.9 |
  46114.1 |
  46715.2 |
  47316.4 |
  47917.6 |########################################
  48518.7 |#
  49119.9 |
  49721.0 |
  50322.2 |###
  50923.3 |#######
  (21 below, 11 above range)

reg_typed (n=160, range 39252.1-49538.7 ns)
  39252.1 |###################################
  39766.4 |
  40280.8 |
  40795.1 |
  41309.4 |
  41823.8 |#
  42338.1 |
  42852.4 |
  43366.8 |
  43881.1 |
  44395.4 |
  44909.8 |
  45424.1 |
  45938.4 |
  46452.8 |
  46967.1 |
  47481.4 |########################################
  47995.7 |############################
  48510.1 |#
  49024.4 |
  (13 below, 13 above range)

reg_typed_twin (n=160, range 39000.5-49407.6 ns)
  39000.5 |########################################
  39520.8 |
  40041.2 |
  40561.5 |####
  41081.9 |
  41602.2 |
  42122.6 |
  42643.0 |
  43163.3 |
  43683.7 |
  44204.0 |
  44724.4 |
  45244.8 |
  45765.1 |
  46285.5 |
  46805.8 |
  47326.2 |##############
  47846.6 |############################
  48366.9 |
  48887.3 |
  (12 below, 21 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=1293.3% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=174.8% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=659.0% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=1296.1% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=1304.6% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=1310.2% of algo (FFI overhead may distort results)
