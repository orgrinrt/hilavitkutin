# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 8.5x slower than the field

reg_bound_record (29.31 us) is 8.5x the fastest (3.44 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_bound_columns, reg_typed_twin) are a dead heat (<1%)

reg_bound_columns (3.44 us) and reg_typed_twin (3.47 us) differ by 0.61%, inside the noise, even though the wider field spreads 750.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {reg_bound_columns, reg_typed_twin, reg_typed, reg_bound_units, reg_bound_strided} vs {reg_bound_record} (367% apart)

The field splits into a fast tier {reg_bound_columns, reg_typed_twin, reg_typed, reg_bound_units, reg_bound_strided} and a slow tier {reg_bound_record} with a 367% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.5x the fastest

Fastest reg_bound_columns (3.44 us) to slowest reg_bound_record (29.31 us): 8.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader reg_bound_columns vs stability leader reg_typed_twin (+1% speed for 1.2x steadier)

reg_bound_columns is fastest (3.44 us, CV 33.3%); reg_typed_twin gives up 0.6% median for 1.2x lower variance (CV 27.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### reg_typed is inconsistent: worst-20% is 2.6x its best-20%

reg_typed's best 20% of batches run at 3.19 us but its worst 20% at 8.15 us (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: reg_bound_columns** at 3444.8 ns median (-1.3% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 8.51x (fastest 3444.8 ns, slowest 29309.2 ns)

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
| Counter frequency | 2100480538 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| reg_bound_columns | 5891ns | 5763ns | 4433ns | 5780ns | 7683ns | -14.46% |
| reg_bound_record | 36414ns | 32426ns | 26684ns | 34968ns | 50483ns | +428.70% |
| reg_bound_strided | 9737ns | 8639ns | 7596ns | 9229ns | 13402ns | +41.37% |
| reg_bound_units | 6395ns | 5891ns | 5117ns | 6032ns | 8761ns | -7.15% |
| reg_typed | 6887ns | 5846ns | 5305ns | 5878ns | 11500ns | base |
| reg_typed_twin | 6119ns | 5748ns | 4652ns | 6012ns | 7907ns | -11.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 3649ns | 2682ns | 5138ns | -17.54% | 4.489 |
| reg_bound_record | 33725ns | 24333ns | 47426ns | +662.01% | 0.486 |
| reg_bound_strided | 7333ns | 5554ns | 10449ns | +65.68% | 2.234 |
| reg_bound_units | 3953ns | 3098ns | 5499ns | -10.68% | 4.144 |
| reg_typed | 4426ns | 3187ns | 8146ns | base | 3.702 |
| reg_typed_twin | 3861ns | 2808ns | 5411ns | -12.76% | 4.243 |

## Performance model

- Peak throughput: **6.110 Gops/s** (reg_bound_columns; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 4.756 | 77.8% |
| reg_bound_record | 0.559 | 9.1% |
| reg_bound_strided | 2.609 | 42.7% |
| reg_bound_units | 4.625 | 75.7% |
| reg_typed | 4.692 | 76.8% |
| reg_typed_twin | 4.727 | 77.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 5891ns | 5891ns | -14.46% |
| reg_bound_record | 36414ns | 36414ns | +428.70% |
| reg_bound_strided | 9737ns | 9737ns | +41.37% |
| reg_bound_units | 6395ns | 6395ns | -7.15% |
| reg_typed | 6887ns | 6887ns | base |
| reg_typed_twin | 6119ns | 6119ns | -11.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 3492ns | base | --- | [3428, 3605] | --- | --- | --- | --- |
| reg_bound_columns | 3445ns | no significant difference | [-282, +50]ns | [3406, 3511] | no | 0.9370 | 0.9370 | 0 |
| reg_bound_record | 29309ns | +25937.3ns (+742.8%) | [+24362, +30080]ns | [28267, 33346] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 6280ns | +2757.6ns (+79.0%) | [+2608, +2885]ns | [6169, 6742] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 3543ns | no significant difference | [-117, +104]ns | [3468, 3749] | no | 0.9370 | 0.9370 | 0 |
| reg_typed_twin | 3466ns | no significant difference | [-122, +160]ns | [3426, 3549] | no | 0.9370 | 0.8126 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3616ns | -3.7% | +1048.1% | +71.0% | -11.3% | -5.1% |
| 1 | 2 | 7292ns | -52.0% | +274.1% | +32.4% | -52.2% | -62.5% |
| 1 | 3 | 3434ns | -16.4% | +601.0% | +78.7% | -2.9% | +5.2% |
| 1 | 4 | 3404ns | +3.9% | +651.1% | +75.5% | +33.9% | +0.8% |
| 1 | 5 | 4074ns | -34.4% | +901.0% | +33.7% | -13.6% | -17.4% |
| 1 | 6 | 3187ns | +10.5% | +901.6% | +85.6% | +4.3% | +7.9% |
| 1 | 7 | 3936ns | +13.0% | +624.1% | +142.8% | -11.5% | +26.2% |
| 1 | 8 | 3247ns | +12.0% | +1459.8% | +137.8% | +28.8% | +63.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | 0.031 | ok |
| reg_bound_record | -0.091 | ok |
| reg_bound_strided | -0.063 | ok |
| reg_bound_units | -0.173 | ok |
| reg_typed | -0.219 | moderate- |
| reg_typed_twin | 0.427 | moderate+ |

**Consistency summary:**

- **reg_bound_columns**: won 4/8, lost 4/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 5/8, lost 3/8
- **reg_typed_twin**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 43690.1ns | 3649.5ns | 1197.2% | HIGH |
| reg_bound_record | 45657.3ns | 33724.7ns | 135.4% | HIGH |
| reg_bound_strided | 47712.1ns | 7332.6ns | 650.7% | HIGH |
| reg_bound_units | 44780.1ns | 3953.2ns | 1132.8% | HIGH |
| reg_typed | 46222.0ns | 4425.7ns | 1044.4% | HIGH |
| reg_typed_twin | 44702.1ns | 3861.0ns | 1157.8% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 2681.6-5138.1 ns)
   2681.6 |################
   2804.4 |########
   2927.3 |
   3050.1 |
   3172.9 |######
   3295.7 |########################################
   3418.6 |##################################
   3541.4 |######################
   3664.2 |############
   3787.0 |########
   3909.8 |####
   4032.7 |####
   4155.5 |#
   4278.3 |####
   4401.1 |######
   4524.0 |##
   4646.8 |##
   4769.6 |#
   4892.4 |##
   5015.3 |######
  (18 below, 7 above range)

reg_bound_record (n=160, range 24332.5-47426.1 ns)
  24332.5 |#########################
  25487.2 |##################
  26641.9 |########################################
  27796.6 |#############################
  28951.2 |#########
  30105.9 |###
  31260.6 |###
  32415.3 |#########
  33570.0 |###
  34724.6 |###
  35879.3 |#####
  37034.0 |###
  38188.7 |#######
  39343.3 |################
  40498.0 |#######################
  41652.7 |##########
  42807.4 |###
  43962.1 |#########
  45116.7 |#######
  46271.4 |###
  (17 below, 13 above range)

reg_bound_strided (n=160, range 5553.9-10449.5 ns)
   5553.9 |##########
   5798.6 |#######################
   6043.4 |########################################
   6288.2 |#######
   6533.0 |####
   6777.8 |###
   7022.5 |
   7267.3 |###
   7512.1 |#######
   7756.9 |###
   8001.7 |###
   8246.4 |#######
   8491.2 |#
   8736.0 |####
   8980.8 |###
   9225.6 |#######
   9470.3 |########
   9715.1 |#######
   9959.9 |
  10204.7 |#
  (10 below, 10 above range)

reg_bound_units (n=160, range 3097.8-5499.4 ns)
   3097.8 |##################
   3217.9 |#################
   3338.0 |########################################
   3458.0 |##########################
   3578.1 |######
   3698.2 |#####
   3818.3 |#
   3938.4 |########
   4058.4 |############
   4178.5 |#####
   4298.6 |##########
   4418.7 |#############
   4538.7 |#########
   4658.8 |#####
   4778.9 |####
   4899.0 |#
   5019.1 |
   5139.1 |
   5259.2 |
   5379.3 |####
  (9 below, 9 above range)

reg_typed (n=160, range 3187.0-8146.2 ns)
   3187.0 |########################################
   3435.0 |###################
   3682.9 |#####
   3930.9 |###########
   4178.9 |#####
   4426.8 |#####
   4674.8 |##
   4922.7 |
   5170.7 |
   5418.7 |
   5666.6 |#
   5914.6 |#
   6162.5 |
   6410.5 |
   6658.5 |
   6906.4 |
   7154.4 |
   7402.4 |
   7650.3 |
   7898.3 |
  (14 below, 10 above range)

reg_typed_twin (n=160, range 2808.2-5410.9 ns)
   2808.2 |#######
   2938.3 |#
   3068.5 |##
   3198.6 |#####
   3328.7 |########################################
   3458.9 |####################
   3589.0 |##########
   3719.1 |#####
   3849.3 |
   3979.4 |
   4109.6 |##
   4239.7 |#
   4369.8 |##
   4500.0 |####
   4630.1 |####
   4760.2 |####
   4890.4 |######
   5020.5 |#####
   5150.6 |####
   5280.8 |#####
  (19 below, 14 above range)

```

## Diagnostics

- **reg_bound_columns**: CV=31.4% (high variance, measurements may be unstable)
- **reg_bound_columns**: bridge=1283.3% of algo (FFI overhead may distort results)
- **reg_bound_record**: CV=26.4% (high variance, measurements may be unstable)
- **reg_bound_record**: bridge=155.0% of algo (FFI overhead may distort results)
- **reg_bound_strided**: CV=26.3% (high variance, measurements may be unstable)
- **reg_bound_strided**: bridge=710.0% of algo (FFI overhead may distort results)
- **reg_bound_units**: CV=28.6% (high variance, measurements may be unstable)
- **reg_bound_units**: bridge=1234.8% of algo (FFI overhead may distort results)
- **reg_typed**: CV=66.3% (high variance, measurements may be unstable)
- **reg_typed**: bridge=1268.3% of algo (FFI overhead may distort results)
- **reg_typed_twin**: CV=24.5% (high variance, measurements may be unstable)
- **reg_typed_twin**: bridge=1274.6% of algo (FFI overhead may distort results)
