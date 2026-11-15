# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Highlights

Baseline for all deltas below: **sb_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sb_none dominates: 23% faster than the next best (sb_plan)

sb_none (10.04 us) leads sb_plan (12.35 us) by 23%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sb_value is an outlier: 15.4x slower than the field

sb_value (155.06 us) is 15.4x the fastest (10.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (sb_none)

The baseline sb_none is the fastest (10.04 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 15.4x the fastest

Fastest sb_none (10.04 us) to slowest sb_value (155.06 us): 15.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (sb_none) is the fastest** at 10037.5 ns median
- 2 variants significantly slower than baseline
- Spread: 15.45x (fastest 10037.5 ns, slowest 155056.2 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sb_none | 13673ns | 13725ns | 12793ns | 13522ns | 14340ns | base |
| sb_plan | 16072ns | 15969ns | 15051ns | 15839ns | 16933ns | +17.55% |
| sb_value | 160145ns | 159017ns | 156890ns | 158368ns | 164437ns | +1071.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 10134ns | 9944ns | 10394ns | base | 0.101 |
| sb_plan | 12500ns | 12265ns | 12853ns | +23.35% | 0.082 |
| sb_value | 156053ns | 152968ns | 159819ns | +1439.92% | 0.007 |

## Performance model

- Peak throughput: **0.103 Gops/s** (sb_none; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.102 | 99.1% |
| sb_plan | 0.083 | 80.5% |
| sb_value | 0.007 | 6.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 13673ns | 13673ns | base |
| sb_plan | 16072ns | 16072ns | +17.55% |
| sb_value | 160145ns | 160145ns | +1071.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 10038ns | base | --- | [9971, 10229] | --- | --- | --- | --- |
| sb_plan | 12350ns | +2333.3ns (+23.2%) | [+2192, +2554]ns | [12296, 12604] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 155056ns | +144760.4ns (+1442.2%) | [+143625, +146560]ns | [153721, 156535] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|---|
| 1 | 1 | 9967ns | +30.2% | +1449.9% |
| 1 | 2 | 10238ns | +20.1% | +1397.9% |
| 1 | 3 | 10075ns | +22.0% | +1427.7% |
| 2 | 1 | 9971ns | +23.9% | +1465.5% |
| 2 | 2 | 10050ns | +22.3% | +1448.6% |
| 2 | 3 | 10004ns | +23.4% | +1434.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | -0.255 | moderate- |
| sb_plan | -0.067 | ok |
| sb_value | 0.075 | ok |

**Consistency summary:**

- **sb_plan**: won 0/6, lost 6/6
- **sb_value**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 5592387.9ns | 10133.8ns | 55185.5% | HIGH |
| sb_plan | 5601757.2ns | 12500.0ns | 44814.1% | HIGH |
| sb_value | 5582726.6ns | 156052.5ns | 3577.5% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 9944.5-10394.4 ns)
   9944.5 |########################################
   9967.0 |########################################
   9989.5 |#############
  10012.0 |#############
  10034.5 |#############
  10057.0 |#############
  10079.5 |
  10102.0 |
  10124.5 |
  10147.0 |
  10169.5 |#############
  10191.9 |
  10214.4 |#############
  10236.9 |##########################
  10259.4 |
  10281.9 |
  10304.4 |
  10326.9 |
  10349.4 |#############
  10371.9 |
  (1 below, 2 above range)

sb_plan (n=18, range 12265.3-12852.8 ns)
  12265.3 |####################
  12294.6 |########################################
  12324.0 |####################
  12353.4 |
  12382.8 |
  12412.1 |##########
  12441.5 |
  12470.9 |
  12500.3 |##########
  12529.6 |
  12559.0 |##########
  12588.4 |
  12617.8 |##########
  12647.2 |##########
  12676.5 |
  12705.9 |
  12735.3 |
  12764.7 |
  12794.0 |
  12823.4 |##########
  (2 below, 2 above range)

sb_value (n=18, range 152968.0-159819.4 ns)
  152968.0 |
  153310.6 |########################################
  153653.2 |########################################
  153995.7 |#############
  154338.3 |#############
  154680.9 |
  155023.5 |
  155366.0 |#############
  155708.6 |#############
  156051.2 |##########################
  156393.7 |
  156736.3 |#############
  157078.9 |#############
  157421.4 |
  157764.0 |#############
  158106.6 |
  158449.2 |
  158791.7 |
  159134.3 |
  159476.9 |
  (1 below, 2 above range)

```

## Diagnostics

- **sb_none**: bridge=55602.2% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=45198.6% of algo (FFI overhead may distort results)
- **sb_value**: bridge=3596.2% of algo (FFI overhead may distort results)
