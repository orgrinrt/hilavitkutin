# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Highlights

Baseline for all deltas below: **sb_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sb_none dominates: 24% faster than the next best (sb_plan)

sb_none (10.02 us) leads sb_plan (12.43 us) by 24%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (sb_none)

The baseline sb_none is the fastest (10.02 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (sb_none) is the fastest** at 10022.9 ns median
- 2 variants significantly slower than baseline
- Spread: 1.77x (fastest 10022.9 ns, slowest 17770.8 ns)

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
| sb_none | 12761ns | 12688ns | 12242ns | 12681ns | 13142ns | base |
| sb_plan | 15171ns | 15015ns | 14599ns | 14959ns | 15774ns | +18.88% |
| sb_value | 20285ns | 20127ns | 19701ns | 20025ns | 20966ns | +58.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 10132ns | 9946ns | 10412ns | base | 0.006 |
| sb_plan | 12581ns | 12258ns | 12996ns | +24.17% | 0.005 |
| sb_value | 17996ns | 17542ns | 18622ns | +77.61% | 0.004 |

## Performance model

- Peak throughput: **0.006 Gops/s** (sb_none; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.006 | 99.2% |
| sb_plan | 0.005 | 80.0% |
| sb_value | 0.004 | 56.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 12761ns | 12761ns | base |
| sb_plan | 15171ns | 15171ns | +18.88% |
| sb_value | 20285ns | 20285ns | +58.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 10023ns | base | --- | [9958, 10277] | --- | --- | --- | --- |
| sb_plan | 12425ns | +2418.8ns (+24.1%) | [+2319, +2552]ns | [12385, 12623] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 17771ns | +7693.8ns (+76.8%) | [+7429, +8142]ns | [17656, 18092] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|---|
| 1 | 1 | 10312ns | +18.7% | +70.3% |
| 1 | 2 | 9958ns | +24.6% | +85.9% |
| 1 | 3 | 10012ns | +23.7% | +77.4% |
| 2 | 1 | 10433ns | +21.3% | +70.4% |
| 2 | 2 | 9958ns | +24.8% | +77.4% |
| 2 | 3 | 9967ns | +28.3% | +89.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | -0.339 | moderate- |
| sb_plan | -0.094 | ok |
| sb_value | -0.349 | moderate- |

**Consistency summary:**

- **sb_plan**: won 0/6, lost 6/6
- **sb_value**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 353470.1ns | 10131.9ns | 3488.7% | HIGH |
| sb_plan | 355638.7ns | 12581.0ns | 2826.8% | HIGH |
| sb_value | 355759.5ns | 17995.6ns | 1976.9% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 9945.8-10411.8 ns)
   9945.8 |########################################
   9969.1 |########
   9992.4 |########
  10015.7 |########
  10039.0 |########
  10062.3 |
  10085.6 |
  10108.9 |
  10132.2 |
  10155.5 |########
  10178.8 |
  10202.1 |
  10225.4 |########
  10248.7 |
  10272.0 |
  10295.3 |########
  10318.6 |
  10341.9 |
  10365.2 |########
  10388.5 |
  (2 below, 3 above range)

sb_plan (n=18, range 12258.3-12995.8 ns)
  12258.3 |
  12295.2 |##########
  12332.1 |##########
  12369.0 |####################
  12405.8 |########################################
  12442.7 |####################
  12479.6 |
  12516.5 |
  12553.3 |
  12590.2 |##########
  12627.1 |##########
  12664.0 |
  12700.8 |
  12737.7 |
  12774.6 |##########
  12811.5 |
  12848.3 |
  12885.2 |##########
  12922.1 |
  12959.0 |
  (2 below, 2 above range)

sb_value (n=18, range 17541.7-18622.2 ns)
  17541.7 |########################################
  17595.7 |
  17649.7 |########################################
  17703.8 |#############
  17757.8 |##########################
  17811.8 |##########################
  17865.8 |#############
  17919.9 |
  17973.9 |
  18027.9 |
  18082.0 |
  18136.0 |
  18190.0 |
  18244.0 |#############
  18298.1 |
  18352.1 |
  18406.1 |
  18460.2 |#############
  18514.2 |
  18568.2 |
  (1 below, 3 above range)

```

## Diagnostics

- **sb_none**: bridge=3477.2% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=2854.9% of algo (FFI overhead may distort results)
- **sb_value**: bridge=1982.3% of algo (FFI overhead may distort results)
