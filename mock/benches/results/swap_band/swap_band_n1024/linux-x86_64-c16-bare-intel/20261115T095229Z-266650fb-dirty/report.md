# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Highlights

Baseline for all deltas below: **sb_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sb_none dominates: 27% faster than the next best (sb_plan)

sb_none (9.90 us) leads sb_plan (12.61 us) by 27%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sb_value is an outlier: 23.8x slower than the field

sb_value (235.25 us) is 23.8x the fastest (9.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sb_none shows alternating (throttle bounce) (autocorr -0.57)

sb_none's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (sb_none)

The baseline sb_none is the fastest (9.90 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 23.8x the fastest

Fastest sb_none (9.90 us) to slowest sb_value (235.25 us): 23.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (sb_none) is the fastest** at 9895.6 ns median
- 2 variants significantly slower than baseline
- Spread: 23.77x (fastest 9895.6 ns, slowest 235245.4 ns)

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
| Counter frequency | 2112803027 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sb_none | 11924ns | 11893ns | 11716ns | 11857ns | 12128ns | base |
| sb_plan | 14695ns | 14639ns | 14268ns | 14579ns | 15081ns | +23.24% |
| sb_value | 245079ns | 237493ns | 223572ns | 235359ns | 270411ns | +1955.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 9892ns | 9667ns | 10090ns | base | 0.104 |
| sb_plan | 12652ns | 12232ns | 12993ns | +27.90% | 0.081 |
| sb_value | 242850ns | 221541ns | 268116ns | +2354.91% | 0.004 |

## Performance model

- Peak throughput: **0.106 Gops/s** (sb_none; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.103 | 97.7% |
| sb_plan | 0.081 | 76.7% |
| sb_value | 0.004 | 4.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 11924ns | 11924ns | base |
| sb_plan | 14695ns | 14695ns | +23.24% |
| sb_value | 245079ns | 245079ns | +1955.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 9896ns | base | --- | [9753, 9970] | --- | --- | --- | --- |
| sb_plan | 12610ns | +2750.8ns (+27.8%) | [+2494, +3007]ns | [12454, 12888] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 235245ns | +225339.1ns (+2277.2%) | [+220033, +232402]ns | [229994, 241999] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|---|
| 1 | 1 | 10132ns | +21.4% | +2189.9% |
| 1 | 2 | 9691ns | +27.6% | +2345.5% |
| 1 | 3 | 9949ns | +26.0% | +2133.0% |
| 2 | 1 | 9887ns | +30.8% | +2867.6% |
| 2 | 2 | 9904ns | +28.8% | +2205.8% |
| 2 | 3 | 9828ns | +31.4% | +2367.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | -0.569 | HIGH- (thermal bounce) |
| sb_plan | 0.414 | moderate+ |
| sb_value | -0.474 | moderate- |

**Consistency summary:**

- **sb_plan**: won 0/6, lost 6/6
- **sb_value**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 3847177.2ns | 9892.4ns | 38890.2% | HIGH |
| sb_plan | 3848461.5ns | 12651.9ns | 30418.0% | HIGH |
| sb_value | 3831214.1ns | 242849.6ns | 1577.6% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 9667.4-10090.2 ns)
   9667.4 |####################
   9688.5 |####################
   9709.7 |
   9730.8 |########################################
   9752.0 |####################
   9773.1 |####################
   9794.2 |
   9815.4 |####################
   9836.5 |
   9857.6 |
   9878.8 |####################
   9899.9 |####################
   9921.1 |####################
   9942.2 |########################################
   9963.3 |
   9984.5 |####################
  10005.6 |
  10026.8 |
  10047.9 |
  10069.0 |
  (1 below, 4 above range)

sb_plan (n=18, range 12232.2-12992.9 ns)
  12232.2 |
  12270.2 |####################
  12308.2 |
  12346.3 |##########
  12384.3 |
  12422.3 |####################
  12460.4 |
  12498.4 |##########
  12536.5 |##########
  12574.5 |##########
  12612.5 |##########
  12650.6 |
  12688.6 |
  12726.6 |##########
  12764.7 |##########
  12802.7 |
  12840.7 |
  12878.8 |########################################
  12916.8 |##########
  12954.8 |
  (1 below, 1 above range)

sb_value (n=18, range 221540.8-268115.9 ns)
  221540.8 |#############
  223869.6 |#############
  226198.3 |##########################
  228527.1 |
  230855.8 |##########################
  233184.6 |########################################
  235513.3 |##########################
  237842.1 |
  240170.9 |##########################
  242499.6 |
  244828.4 |#############
  247157.1 |
  249485.9 |
  251814.6 |
  254143.4 |
  256472.1 |
  258800.9 |
  261129.6 |
  263458.4 |
  265787.1 |
  (1 below, 3 above range)

```

## Diagnostics

- **sb_none**: bridge=38879.3% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=30526.3% of algo (FFI overhead may distort results)
- **sb_value**: bridge=1627.2% of algo (FFI overhead may distort results)
