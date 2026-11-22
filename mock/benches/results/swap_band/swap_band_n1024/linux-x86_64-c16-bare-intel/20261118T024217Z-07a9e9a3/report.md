# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Highlights

Baseline for all deltas below: **sb_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sb_none dominates: 39% faster than the next best (sb_plan)

sb_none (8.29 us) leads sb_plan (11.48 us) by 39%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sb_value is an outlier: 34.0x slower than the field

sb_value (282.01 us) is 34.0x the fastest (8.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (sb_none)

The baseline sb_none is the fastest (8.29 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 34.0x the fastest

Fastest sb_none (8.29 us) to slowest sb_value (282.01 us): 34.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (sb_none) is the fastest** at 8291.0 ns median
- 2 variants significantly slower than baseline
- Spread: 34.01x (fastest 8291.0 ns, slowest 282006.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | next-frame asymmetry: clean skip vs value-swap cone vs plan band |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000002 |
| Counter frequency | 2112775712 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sb_none | 10348ns | 10322ns | 10219ns | 10301ns | 10482ns | base |
| sb_plan | 13514ns | 13486ns | 13414ns | 13480ns | 13615ns | +30.60% |
| sb_value | 283544ns | 284176ns | 235415ns | 278106ns | 315765ns | +2640.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 8332ns | 8235ns | 8450ns | base | 0.123 |
| sb_plan | 11489ns | 11396ns | 11550ns | +37.88% | 0.089 |
| sb_value | 281236ns | 233347ns | 313418ns | +3275.26% | 0.004 |

## Performance model

- Peak throughput: **0.124 Gops/s** (sb_none; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.124 | 99.3% |
| sb_plan | 0.089 | 71.7% |
| sb_value | 0.004 | 2.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 10348ns | 10348ns | base |
| sb_plan | 13514ns | 13514ns | +30.60% |
| sb_value | 283544ns | 283544ns | +2640.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 8291ns | base | --- | [8270, 8338] | --- | --- | --- | --- |
| sb_plan | 11484ns | +3191.1ns (+38.5%) | [+3147, +3225]ns | [11464, 11523] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 282007ns | +273673.7ns (+3300.9%) | [+267101, +285381]ns | [275409, 293670] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|---|
| 1 | 1 | 8282ns | +38.5% | +3246.1% |
| 1 | 2 | 8240ns | +38.9% | +3716.4% |
| 1 | 3 | 8267ns | +39.4% | +3577.8% |
| 2 | 1 | 8318ns | +37.8% | +3330.1% |
| 2 | 2 | 8403ns | +37.4% | +2685.3% |
| 2 | 3 | 8332ns | +38.4% | +3297.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | 0.439 | moderate+ |
| sb_plan | -0.092 | ok |
| sb_value | 0.106 | ok |

**Consistency summary:**

- **sb_plan**: won 0/6, lost 6/6
- **sb_value**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 3924718.3ns | 8332.3ns | 47102.6% | HIGH |
| sb_plan | 3850786.8ns | 11488.9ns | 33517.4% | HIGH |
| sb_value | 3912132.9ns | 281236.2ns | 1391.0% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 8235.3-8449.9 ns)
   8235.3 |#############
   8246.0 |##########################
   8256.7 |#############
   8267.5 |########################################
   8278.2 |#############
   8288.9 |
   8299.7 |#############
   8310.4 |#############
   8321.1 |#############
   8331.9 |##########################
   8342.6 |##########################
   8353.3 |
   8364.0 |
   8374.8 |
   8385.5 |
   8396.2 |#############
   8407.0 |
   8417.7 |
   8428.4 |
   8439.2 |
  (1 below, 1 above range)

sb_plan (n=18, range 11396.2-11549.6 ns)
  11396.2 |
  11403.9 |
  11411.5 |
  11419.2 |
  11426.9 |
  11434.6 |########################################
  11442.2 |
  11449.9 |####################
  11457.6 |####################
  11465.2 |####################
  11472.9 |########################################
  11480.6 |########################################
  11488.2 |
  11495.9 |
  11503.6 |########################################
  11511.3 |
  11518.9 |########################################
  11526.6 |####################
  11534.3 |####################
  11541.9 |####################
  (1 below, 1 above range)

sb_value (n=18, range 233346.7-313417.7 ns)
  233346.7 |##########################
  237350.2 |#############
  241353.8 |
  245357.3 |
  249360.9 |
  253364.4 |
  257368.0 |
  261371.5 |
  265375.1 |
  269378.6 |
  273382.2 |##########################
  277385.7 |########################################
  281389.3 |##########################
  285392.8 |##########################
  289396.4 |
  293399.9 |
  297403.5 |#############
  301407.0 |#############
  305410.6 |
  309414.1 |#############
  (1 below, 2 above range)

```

## Diagnostics

- **sb_none**: bridge=46434.7% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=33519.5% of algo (FFI overhead may distort results)
- **sb_value**: bridge=1359.6% of algo (FFI overhead may distort results)
