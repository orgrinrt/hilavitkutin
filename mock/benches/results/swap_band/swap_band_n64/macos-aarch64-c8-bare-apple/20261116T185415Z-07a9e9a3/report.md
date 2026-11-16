# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Highlights

Baseline for all deltas below: **sb_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sb_none dominates: 23% faster than the next best (sb_plan)

sb_none (10.19 us) leads sb_plan (12.50 us) by 23%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sb_none shows alternating (throttle bounce) (autocorr -0.66)

sb_none's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (sb_none)

The baseline sb_none is the fastest (10.19 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (sb_none) is the fastest** at 10187.5 ns median
- 2 variants significantly slower than baseline
- Spread: 1.77x (fastest 10187.5 ns, slowest 17983.3 ns)

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
| sb_none | 13057ns | 13042ns | 12425ns | 12981ns | 13486ns | base |
| sb_plan | 15557ns | 15490ns | 14725ns | 15353ns | 16279ns | +19.15% |
| sb_value | 20908ns | 20875ns | 20167ns | 20721ns | 21558ns | +60.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 10240ns | 10150ns | 10377ns | base | 0.006 |
| sb_plan | 12657ns | 12462ns | 12986ns | +23.61% | 0.005 |
| sb_value | 18092ns | 17907ns | 18351ns | +76.69% | 0.004 |

## Performance model

- Peak throughput: **0.006 Gops/s** (sb_none; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.006 | 99.6% |
| sb_plan | 0.005 | 81.2% |
| sb_value | 0.004 | 56.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 13057ns | 13057ns | base |
| sb_plan | 15557ns | 15557ns | +19.15% |
| sb_value | 20908ns | 20908ns | +60.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 10188ns | base | --- | [10156, 10240] | --- | --- | --- | --- |
| sb_plan | 12504ns | +2339.5ns (+23.0%) | [+2277, +2410]ns | [12488, 12571] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 17983ns | +7783.4ns (+76.4%) | [+7746, +7960]ns | [17950, 18123] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|---|
| 1 | 1 | 10175ns | +22.8% | +76.5% |
| 1 | 2 | 10188ns | +22.7% | +76.4% |
| 1 | 3 | 10188ns | +22.5% | +76.4% |
| 2 | 1 | 10150ns | +23.0% | +77.1% |
| 2 | 2 | 10242ns | +30.8% | +82.0% |
| 2 | 3 | 10162ns | +23.2% | +77.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | -0.661 | HIGH- (thermal bounce) |
| sb_plan | -0.205 | moderate- |
| sb_value | -0.188 | ok |

**Consistency summary:**

- **sb_plan**: won 0/6, lost 6/6
- **sb_value**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 355796.3ns | 10239.6ns | 3474.7% | HIGH |
| sb_plan | 359537.5ns | 12656.9ns | 2840.6% | HIGH |
| sb_value | 356006.2ns | 18091.9ns | 1967.8% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 10150.0-10377.1 ns)
  10150.0 |########################################
  10161.4 |######
  10172.7 |######
  10184.1 |####################
  10195.4 |
  10206.8 |
  10218.1 |######
  10229.5 |######
  10240.8 |######
  10252.2 |
  10263.5 |
  10274.9 |
  10286.3 |
  10297.6 |
  10309.0 |######
  10320.3 |######
  10331.7 |
  10343.0 |
  10354.4 |
  10365.7 |
  (0 below, 2 above range)

sb_plan (n=18, range 12462.5-12986.1 ns)
  12462.5 |################################
  12488.7 |########################################
  12514.9 |################
  12541.0 |########
  12567.2 |########
  12593.4 |
  12619.6 |
  12645.8 |
  12671.9 |
  12698.1 |
  12724.3 |
  12750.5 |
  12776.7 |
  12802.8 |
  12829.0 |
  12855.2 |
  12881.4 |
  12907.6 |
  12933.7 |########
  12959.9 |
  (1 below, 3 above range)

sb_value (n=18, range 17906.9-18351.4 ns)
  17906.9 |##############################
  17929.2 |##########
  17951.4 |########################################
  17973.6 |####################
  17995.8 |
  18018.0 |
  18040.3 |
  18062.5 |
  18084.7 |
  18106.9 |####################
  18129.2 |##########
  18151.4 |##########
  18173.6 |
  18195.8 |
  18218.1 |
  18240.3 |
  18262.5 |
  18284.7 |##########
  18307.0 |
  18329.2 |
  (1 below, 2 above range)

```

## Diagnostics

- **sb_none**: bridge=3489.9% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=2850.8% of algo (FFI overhead may distort results)
- **sb_value**: bridge=1979.4% of algo (FFI overhead may distort results)
