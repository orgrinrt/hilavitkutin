# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Highlights

Baseline for all deltas below: **sb_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sb_none dominates: 25% faster than the next best (sb_plan)

sb_none (10.57 us) leads sb_plan (13.23 us) by 25%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sb_value is an outlier: 493.2x slower than the field

sb_value (5.21 ms) is 493.2x the fastest (10.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (sb_none)

The baseline sb_none is the fastest (10.57 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 493.2x the fastest

Fastest sb_none (10.57 us) to slowest sb_value (5.21 ms): 493.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (sb_none) is the fastest** at 10573.1 ns median
- 2 variants significantly slower than baseline
- Spread: 493.18x (fastest 10573.1 ns, slowest 5214466.2 ns)

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
| Counter frequency | 2112749428 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sb_none | 13957ns | 13917ns | 13526ns | 13883ns | 14284ns | base |
| sb_plan | 16574ns | 16618ns | 16245ns | 16553ns | 16769ns | +18.75% |
| sb_value | 5211086ns | 5217889ns | 5144250ns | 5205462ns | 5252939ns | +37236.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 10628ns | 10455ns | 10817ns | base | 0.771 |
| sb_plan | 13214ns | 12980ns | 13393ns | +24.33% | 0.620 |
| sb_value | 5207559ns | 5140665ns | 5249374ns | +48899.18% | 0.002 |

## Performance model

- Peak throughput: **0.784 Gops/s** (sb_none; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.775 | 98.9% |
| sb_plan | 0.619 | 79.0% |
| sb_value | 0.002 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 13957ns | 13957ns | base |
| sb_plan | 16574ns | 16574ns | +18.75% |
| sb_value | 5211086ns | 5211086ns | +37236.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 10573ns | base | --- | [10522, 10753] | --- | --- | --- | --- |
| sb_plan | 13231ns | +2596.8ns (+24.6%) | [+2512, +2721]ns | [13115, 13333] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 5214466ns | +5203944.3ns (+49218.7%) | [+5173876, +5221559]ns | [5180943, 5232041] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|---|
| 1 | 1 | 10497ns | +27.1% | +49754.7% |
| 1 | 2 | 10471ns | +24.6% | +50082.7% |
| 1 | 3 | 10755ns | +21.9% | +48436.8% |
| 2 | 1 | 10561ns | +25.4% | +49175.2% |
| 2 | 2 | 10807ns | +24.7% | +47964.5% |
| 2 | 3 | 10752ns | +23.0% | +47731.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | 0.011 | ok |
| sb_plan | 0.011 | ok |
| sb_value | 0.356 | moderate+ |

**Consistency summary:**

- **sb_plan**: won 0/6, lost 6/6
- **sb_value**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 31082123.5ns | 10627.9ns | 292459.2% | HIGH |
| sb_plan | 30755893.2ns | 13214.1ns | 232750.2% | HIGH |
| sb_value | 30870877.2ns | 5207559.3ns | 592.8% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 10455.2-10817.4 ns)
  10455.2 |####################
  10473.3 |
  10491.4 |########################################
  10509.5 |####################
  10527.6 |########################################
  10545.7 |########################################
  10563.8 |
  10582.0 |####################
  10600.1 |
  10618.2 |########################################
  10636.3 |
  10654.4 |
  10672.5 |
  10690.6 |
  10708.8 |
  10726.9 |
  10745.0 |########################################
  10763.1 |
  10781.2 |####################
  10799.3 |####################
  (1 below, 2 above range)

sb_plan (n=18, range 12979.7-13392.5 ns)
  12979.7 |
  13000.3 |
  13021.0 |
  13041.6 |########################################
  13062.3 |
  13082.9 |
  13103.5 |########################################
  13124.2 |########################################
  13144.8 |
  13165.5 |
  13186.1 |
  13206.8 |####################
  13227.4 |####################
  13248.0 |####################
  13268.7 |####################
  13289.3 |
  13310.0 |####################
  13330.6 |########################################
  13351.2 |
  13371.9 |####################
  (2 below, 2 above range)

sb_value (n=18, range 5140664.9-5249374.0 ns)
  5140664.9 |#############
  5146100.4 |
  5151535.8 |
  5156971.3 |
  5162406.7 |
  5167842.2 |
  5173277.6 |########################################
  5178713.1 |
  5184148.5 |#############
  5189584.0 |#############
  5195019.4 |
  5200454.9 |#############
  5205890.4 |#############
  5211325.8 |
  5216761.3 |#############
  5222196.7 |#############
  5227632.2 |##########################
  5233067.6 |##########################
  5238503.1 |
  5243938.5 |
  (1 below, 3 above range)

```

## Diagnostics

- **sb_none**: bridge=294394.2% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=231887.7% of algo (FFI overhead may distort results)
- **sb_value**: bridge=589.8% of algo (FFI overhead may distort results)
