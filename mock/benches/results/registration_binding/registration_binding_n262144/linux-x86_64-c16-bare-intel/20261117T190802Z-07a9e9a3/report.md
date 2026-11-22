# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 7.4x slower than the field

reg_bound_record (293.26 us) is 7.4x the fastest (39.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_typed, reg_bound_units) are a dead heat (<1%)

reg_typed (39.49 us) and reg_bound_units (39.72 us) differ by 0.58%, inside the noise, even though the wider field spreads 642.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### reg_typed_twin shows warm-up / thermal drift (autocorr +0.58)

reg_typed_twin's per-pass series has lag-1 autocorrelation +0.58, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (reg_typed)

The baseline reg_typed is the fastest (39.49 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {reg_typed, reg_bound_units, reg_typed_twin, reg_bound_columns, reg_bound_strided} vs {reg_bound_record} (306% apart)

The field splits into a fast tier {reg_typed, reg_bound_units, reg_typed_twin, reg_bound_columns, reg_bound_strided} and a slow tier {reg_bound_record} with a 306% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.4x the fastest

Fastest reg_typed (39.49 us) to slowest reg_bound_record (293.26 us): 7.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (reg_typed) is the fastest** at 39485.1 ns median
- 5 variants significantly slower than baseline
- Spread: 7.43x (fastest 39485.1 ns, slowest 293263.0 ns)

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
| reg_bound_columns | 48029ns | 50358ns | 41180ns | 48424ns | 53692ns | +7.53% |
| reg_bound_record | 303181ns | 294913ns | 293457ns | 295131ns | 337058ns | +578.80% |
| reg_bound_strided | 77785ns | 73883ns | 73028ns | 75290ns | 90027ns | +74.16% |
| reg_bound_units | 44789ns | 41364ns | 41148ns | 43501ns | 52295ns | +0.28% |
| reg_typed | 44664ns | 41157ns | 40808ns | 43630ns | 51623ns | base |
| reg_typed_twin | 46812ns | 49713ns | 40885ns | 46692ns | 53100ns | +4.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 46156ns | 39520ns | 51661ns | +7.72% | 5.680 |
| reg_bound_record | 301465ns | 291793ns | 335128ns | +603.57% | 0.870 |
| reg_bound_strided | 76035ns | 71353ns | 87982ns | +77.45% | 3.448 |
| reg_bound_units | 43005ns | 39495ns | 50228ns | +0.36% | 6.096 |
| reg_typed | 42848ns | 39137ns | 49535ns | base | 6.118 |
| reg_typed_twin | 44927ns | 39222ns | 51061ns | +4.85% | 5.835 |

## Performance model

- Peak throughput: **6.698 Gops/s** (reg_typed; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 5.424 | 81.0% |
| reg_bound_record | 0.894 | 13.3% |
| reg_bound_strided | 3.631 | 54.2% |
| reg_bound_units | 6.601 | 98.5% |
| reg_typed | 6.639 | 99.1% |
| reg_typed_twin | 5.499 | 82.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 48029ns | 48029ns | +7.53% |
| reg_bound_record | 303181ns | 303181ns | +578.80% |
| reg_bound_strided | 77785ns | 77785ns | +74.16% |
| reg_bound_units | 44789ns | 44789ns | +0.28% |
| reg_typed | 44664ns | 44664ns | base |
| reg_typed_twin | 46812ns | 46812ns | +4.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 39485ns | base | --- | [39324, 41314] | --- | --- | --- | --- |
| reg_bound_columns | 48329ns | +1700.0ns (+4.3%) | [+819, +3008]ns | [42465, 50454] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_record | 293263ns | +252689.7ns (+640.0%) | [+252395, +252968]ns | [292884, 293724] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 72197ns | +32423.0ns (+82.1%) | [+32147, +32693]ns | [71943, 72409] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 39716ns | +385.6ns (+1.0%) | [+326, +444]ns | [39674, 40289] | YES | 0.0004 | 0.0003 | 0 |
| reg_typed_twin | 47674ns | +234.2ns (+0.6%) | [+94, +8355]ns | [43911, 47716] | YES | 0.0011 | 0.0011 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 39718ns | +6.7% | +637.1% | +80.4% | -0.5% | +27.8% |
| 1 | 2 | 47786ns | +6.3% | +517.2% | +50.4% | +8.7% | -0.2% |
| 1 | 3 | 39222ns | +29.6% | +644.9% | +83.3% | +23.3% | +23.2% |
| 1 | 4 | 39314ns | +1.0% | +795.3% | +97.7% | +22.9% | +21.4% |
| 1 | 5 | 49475ns | -2.8% | +492.3% | +46.4% | -20.0% | -20.7% |
| 1 | 6 | 39284ns | +0.5% | +645.0% | +122.6% | +1.0% | +0.3% |
| 1 | 7 | 39167ns | +32.7% | +646.7% | +104.0% | +6.1% | +0.3% |
| 1 | 8 | 47845ns | +1.0% | +513.3% | +49.8% | -17.4% | -18.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.425 | moderate- |
| reg_bound_record | -0.179 | ok |
| reg_bound_strided | 0.045 | ok |
| reg_bound_units | 0.228 | moderate+ |
| reg_typed | -0.473 | moderate- |
| reg_typed_twin | 0.582 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **reg_bound_columns**: won 1/8, lost 7/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 3/8, lost 5/8
- **reg_typed_twin**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 576258.6ns | 46155.9ns | 1248.5% | HIGH |
| reg_bound_record | 526654.7ns | 301465.5ns | 174.7% | HIGH |
| reg_bound_strided | 534727.8ns | 76035.0ns | 703.3% | HIGH |
| reg_bound_units | 549675.3ns | 43004.5ns | 1278.2% | HIGH |
| reg_typed | 553513.1ns | 42848.1ns | 1291.8% | HIGH |
| reg_typed_twin | 578299.7ns | 44926.7ns | 1287.2% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 39520.4-51661.1 ns)
  39520.4 |###################
  40127.5 |#
  40734.5 |
  41341.5 |###
  41948.6 |##############
  42555.6 |
  43162.6 |
  43769.7 |
  44376.7 |
  44983.8 |
  45590.8 |
  46197.8 |
  46804.9 |
  47411.9 |
  48018.9 |################
  48626.0 |
  49233.0 |
  49840.0 |
  50447.1 |########################################
  51054.1 |
  (21 below, 20 above range)

reg_bound_record (n=160, range 291793.4-335128.5 ns)
  291793.4 |########################################
  293960.1 |###############
  296126.9 |
  298293.6 |
  300460.4 |
  302627.2 |
  304793.9 |
  306960.7 |
  309127.4 |
  311294.2 |
  313460.9 |
  315627.7 |
  317794.4 |
  319961.2 |
  322127.9 |
  324294.7 |
  326461.5 |
  328628.2 |
  330795.0 |
  332961.7 |
  (12 below, 23 above range)

reg_bound_strided (n=160, range 71353.1-87982.4 ns)
  71353.1 |########################################
  72184.5 |##############
  73016.0 |
  73847.5 |
  74678.9 |
  75510.4 |
  76341.9 |
  77173.3 |#############
  78004.8 |
  78836.3 |
  79667.7 |
  80499.2 |
  81330.7 |
  82162.1 |
  82993.6 |
  83825.1 |
  84656.5 |
  85488.0 |
  86319.5 |#
  87150.9 |############
  (14 below, 10 above range)

reg_bound_units (n=160, range 39494.9-50227.7 ns)
  39494.9 |########################################
  40031.5 |#
  40568.1 |
  41104.8 |####
  41641.4 |###
  42178.1 |
  42714.7 |
  43251.4 |
  43788.0 |
  44324.7 |
  44861.3 |
  45397.9 |
  45934.6 |
  46471.2 |
  47007.9 |
  47544.5 |
  48081.2 |#################
  48617.8 |
  49154.4 |
  49691.1 |
  (14 below, 17 above range)

reg_typed (n=160, range 39137.3-49534.9 ns)
  39137.3 |########################################
  39657.2 |#
  40177.1 |
  40697.0 |
  41216.8 |#######
  41736.7 |#
  42256.6 |
  42776.5 |
  43296.4 |
  43816.2 |
  44336.1 |
  44856.0 |
  45375.9 |
  45895.8 |
  46415.6 |
  46935.5 |#
  47455.4 |#######################
  47975.3 |
  48495.1 |
  49015.0 |
  (13 below, 12 above range)

reg_typed_twin (n=160, range 39221.7-51060.5 ns)
  39221.7 |#####################################
  39813.7 |##
  40405.6 |
  40997.6 |
  41589.5 |
  42181.4 |
  42773.4 |
  43365.3 |
  43957.3 |
  44549.2 |
  45141.1 |
  45733.1 |
  46325.0 |
  46917.0 |
  47508.9 |########################################
  48100.8 |##
  48692.8 |
  49284.7 |
  49876.7 |
  50468.6 |#####################
  (13 below, 5 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=1293.0% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=174.3% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=708.2% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=1288.7% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=1295.1% of algo (FFI overhead may distort results)
- **reg_typed_twin**: autocorrelation=0.58 (measurement drift or warm-up artifact)
- **reg_typed_twin**: bridge=1310.2% of algo (FFI overhead may distort results)
