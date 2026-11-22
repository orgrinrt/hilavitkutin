# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 7.5x slower than the field

reg_bound_record (89.02 us) is 7.5x the fastest (11.92 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_typed_twin, reg_bound_columns) are a dead heat (<1%)

reg_typed_twin (11.92 us) and reg_bound_columns (11.97 us) differ by 0.43%, inside the noise, even though the wider field spreads 646.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### reg_bound_strided shows alternating (throttle bounce) (autocorr -0.61)

reg_bound_strided's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {reg_typed_twin, reg_bound_columns, reg_typed, reg_bound_units, reg_bound_strided} vs {reg_bound_record} (304% apart)

The field splits into a fast tier {reg_typed_twin, reg_bound_columns, reg_typed, reg_bound_units, reg_bound_strided} and a slow tier {reg_bound_record} with a 304% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.5x the fastest

Fastest reg_typed_twin (11.92 us) to slowest reg_bound_record (89.02 us): 7.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: reg_typed_twin** at 11922.5 ns median (-3.5% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 7.47x (fastest 11922.5 ns, slowest 89018.6 ns)

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
| reg_bound_columns | 13286ns | 13907ns | 11531ns | 13358ns | 14827ns | -4.75% |
| reg_bound_record | 89034ns | 90973ns | 80305ns | 90970ns | 91958ns | +538.28% |
| reg_bound_strided | 23318ns | 23990ns | 19618ns | 23661ns | 25989ns | +67.16% |
| reg_bound_units | 14074ns | 14511ns | 12594ns | 14368ns | 14671ns | +0.89% |
| reg_typed | 13949ns | 14321ns | 12448ns | 14187ns | 14738ns | base |
| reg_typed_twin | 13751ns | 13861ns | 12333ns | 13924ns | 14648ns | -1.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 11456ns | 9950ns | 12782ns | -4.88% | 5.721 |
| reg_bound_record | 87124ns | 78574ns | 89999ns | +623.42% | 0.752 |
| reg_bound_strided | 21486ns | 18023ns | 23985ns | +78.40% | 3.050 |
| reg_bound_units | 12183ns | 10882ns | 12731ns | +1.16% | 5.379 |
| reg_typed | 12043ns | 10722ns | 12743ns | base | 5.442 |
| reg_typed_twin | 11845ns | 10614ns | 12645ns | -1.65% | 5.533 |

## Performance model

- Peak throughput: **6.586 Gops/s** (reg_bound_columns; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 5.473 | 83.1% |
| reg_bound_record | 0.736 | 11.2% |
| reg_bound_strided | 2.973 | 45.1% |
| reg_bound_units | 5.208 | 79.1% |
| reg_typed | 5.303 | 80.5% |
| reg_typed_twin | 5.497 | 83.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 13286ns | 13286ns | -4.75% |
| reg_bound_record | 89034ns | 89034ns | +538.28% |
| reg_bound_strided | 23318ns | 23318ns | +67.16% |
| reg_bound_units | 14074ns | 14074ns | +0.89% |
| reg_typed | 13949ns | 13949ns | base |
| reg_typed_twin | 13751ns | 13751ns | -1.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 12358ns | base | --- | [12022, 12474] | --- | --- | --- | --- |
| reg_bound_columns | 11974ns | -339.4ns (-2.7%) | [-501, -65]ns | [11966, 11979] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_record | 89019ns | +76766.4ns (+621.2%) | [+76545, +76958]ns | [88959, 89071] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 22042ns | +10032.1ns (+81.2%) | [+9913, +10988]ns | [21926, 23454] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 12585ns | +170.7ns (+1.4%) | [+121, +193]ns | [12563, 12600] | YES | 0.0000 | 0.0000 | 0 |
| reg_typed_twin | 11922ns | -101.3ns (-0.8%) | [-248, -87]ns | [11917, 11962] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 12495ns | -17.3% | +611.9% | +54.2% | +1.6% | -4.6% |
| 1 | 2 | 12492ns | -4.1% | +616.2% | +88.2% | +1.7% | -4.7% |
| 1 | 3 | 12052ns | -18.7% | +640.9% | +49.6% | -15.5% | +2.9% |
| 1 | 4 | 12003ns | -0.2% | +644.1% | +82.9% | +4.9% | -0.7% |
| 1 | 5 | 12493ns | -4.1% | +608.8% | +88.9% | +0.9% | -4.2% |
| 1 | 6 | 9777ns | +3.5% | +811.0% | +85.0% | +29.0% | -0.2% |
| 1 | 7 | 11998ns | +0.1% | +504.8% | +96.3% | +0.5% | -0.7% |
| 1 | 8 | 12377ns | -0.9% | +617.1% | +78.5% | -2.1% | -1.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.432 | moderate- |
| reg_bound_record | -0.137 | ok |
| reg_bound_strided | -0.607 | HIGH- (thermal bounce) |
| reg_bound_units | -0.251 | moderate- |
| reg_typed | -0.150 | ok |
| reg_typed_twin | -0.087 | ok |

**Consistency summary:**

- **reg_bound_columns**: won 6/8, lost 1/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 2/8, lost 6/8
- **reg_typed_twin**: won 7/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 147413.2ns | 11455.5ns | 1286.8% | HIGH |
| reg_bound_record | 152703.4ns | 87123.8ns | 175.3% | HIGH |
| reg_bound_strided | 146422.5ns | 21485.5ns | 681.5% | HIGH |
| reg_bound_units | 152791.0ns | 12183.4ns | 1254.1% | HIGH |
| reg_typed | 153467.2ns | 12043.3ns | 1274.3% | HIGH |
| reg_typed_twin | 153449.8ns | 11844.7ns | 1295.5% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 9950.1-12781.9 ns)
   9950.1 |#
  10091.7 |##########
  10233.2 |######
  10374.8 |
  10516.4 |
  10658.0 |
  10799.6 |
  10941.2 |
  11082.8 |
  11224.4 |
  11366.0 |
  11507.6 |
  11649.2 |
  11790.7 |
  11932.3 |########################################
  12073.9 |#
  12215.5 |
  12357.1 |####
  12498.7 |
  12640.3 |
  (17 below, 2 above range)

reg_bound_record (n=160, range 78573.8-89999.3 ns)
  78573.8 |
  79145.1 |
  79716.3 |
  80287.6 |
  80858.9 |
  81430.2 |
  82001.5 |
  82572.7 |
  83144.0 |
  83715.3 |
  84286.6 |
  84857.8 |
  85429.1 |
  86000.4 |
  86571.7 |
  87142.9 |
  87714.2 |
  88285.5 |#################
  88856.8 |########################################
  89428.1 |############
  (20 below, 5 above range)

reg_bound_strided (n=160, range 18022.8-23984.7 ns)
  18022.8 |##########
  18320.9 |
  18619.0 |
  18917.1 |
  19215.2 |#############
  19513.3 |
  19811.4 |
  20109.5 |
  20407.6 |
  20705.7 |
  21003.8 |
  21301.9 |
  21600.0 |###
  21898.1 |###############
  22196.2 |
  22494.3 |
  22792.4 |
  23090.5 |
  23388.6 |########################################
  23686.7 |##
  (17 below, 2 above range)

reg_bound_units (n=160, range 10881.6-12730.6 ns)
  10881.6 |
  10974.1 |
  11066.5 |
  11159.0 |
  11251.4 |
  11343.9 |
  11436.3 |
  11528.8 |
  11621.2 |
  11713.7 |
  11806.1 |
  11898.6 |
  11991.0 |############
  12083.5 |##########################
  12175.9 |
  12268.4 |
  12360.8 |
  12453.3 |
  12545.7 |########################################
  12638.2 |##################################
  (20 below, 6 above range)

reg_typed (n=160, range 10722.3-12742.9 ns)
  10722.3 |
  10823.4 |
  10924.4 |
  11025.4 |
  11126.4 |
  11227.5 |
  11328.5 |
  11429.5 |
  11530.6 |
  11631.6 |
  11732.6 |
  11833.7 |
  11934.7 |###########################
  12035.7 |###
  12136.7 |
  12237.8 |
  12338.8 |
  12439.8 |########################################
  12540.9 |
  12641.9 |
  (18 below, 1 above range)

reg_typed_twin (n=160, range 10614.2-12644.9 ns)
  10614.2 |
  10715.7 |
  10817.3 |
  10918.8 |
  11020.3 |
  11121.9 |
  11223.4 |
  11324.9 |
  11426.5 |
  11528.0 |
  11629.5 |
  11731.1 |
  11832.6 |########################################
  11934.1 |############
  12035.7 |#
  12137.2 |#################
  12238.7 |#
  12340.3 |############
  12441.8 |
  12543.3 |
  (20 below, 2 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=1304.1% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=175.4% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=708.5% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=1240.7% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=1263.2% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=1309.3% of algo (FFI overhead may distort results)
