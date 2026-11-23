# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cs_plain dominates: 16% faster than the next best (cs_exch)

cs_plain (6.21 ms) leads cs_exch (7.18 ms) by 16%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### cs_plain shows alternating (throttle bounce) (autocorr -0.54)

cs_plain's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (cs_plain)

The baseline cs_plain is the fastest (6.21 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (cs_plain) is the fastest** at 6209034.3 ns median
- 3 variants significantly slower than baseline
- Spread: 1.47x (fastest 6209034.3 ns, slowest 9108894.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | a buffered column's swap: plain floor vs Snapshot copy vs Exchange |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000004 |
| Counter frequency | 2112756002 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cs_exch | 7210400ns | 7182124ns | 7130123ns | 7174011ns | 7305121ns | +15.41% |
| cs_plain | 6247608ns | 6212758ns | 6144500ns | 6209076ns | 6356959ns | base |
| cs_snap | 9141166ns | 9112707ns | 8963220ns | 9090346ns | 9306371ns | +46.31% |
| cs_snap_twin | 9080119ns | 9007127ns | 8944628ns | 8997419ns | 9271913ns | +45.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 7206648ns | 7126396ns | 7301426ns | +15.42% | 0.001 |
| cs_plain | 6243833ns | 6140772ns | 6353293ns | base | 0.001 |
| cs_snap | 9137366ns | 8959465ns | 9302512ns | +46.34% | 0.001 |
| cs_snap_twin | 9076381ns | 8940829ns | 9268314ns | +45.37% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (cs_plain; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.001 | 85.5% |
| cs_plain | 0.001 | 98.9% |
| cs_snap | 0.001 | 67.4% |
| cs_snap_twin | 0.001 | 68.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 7210400ns | 7210400ns | +15.41% |
| cs_plain | 6247608ns | 6247608ns | base |
| cs_snap | 9141166ns | 9141166ns | +46.31% |
| cs_snap_twin | 9080119ns | 9080119ns | +45.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 6209034ns | base | --- | [6184876, 6307130] | --- | --- | --- | --- |
| cs_exch | 7178429ns | +960804.9ns (+15.5%) | [+902018, +1021585]ns | [7151727, 7226537] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap | 9108895ns | +2888941.0ns (+46.5%) | [+2793227, +2997621]ns | [9070007, 9211861] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap_twin | 9003419ns | +2738167.5ns (+44.1%) | [+2665247, +2981873]ns | [8968328, 9192382] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 6196594ns | +16.9% | +47.4% | +48.6% |
| 1 | 2 | 6392239ns | +12.7% | +42.0% | +41.3% |
| 1 | 3 | 6226037ns | +18.1% | +44.3% | +43.7% |
| 2 | 1 | 6192578ns | +15.5% | +45.1% | +44.5% |
| 2 | 2 | 6341486ns | +13.6% | +47.3% | +41.6% |
| 2 | 3 | 6163804ns | +15.7% | +49.8% | +52.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | -0.315 | moderate- |
| cs_plain | -0.541 | HIGH- (thermal bounce) |
| cs_snap | 0.180 | ok |
| cs_snap_twin | 0.013 | ok |

**Consistency summary:**

- **cs_exch**: won 0/6, lost 6/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 30910148.8ns | 7206648.2ns | 428.9% | HIGH |
| cs_plain | 31013376.9ns | 6243832.9ns | 496.7% | HIGH |
| cs_snap | 30809889.0ns | 9137365.5ns | 337.2% | HIGH |
| cs_snap_twin | 30692662.6ns | 9076380.6ns | 338.2% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 7126396.2-7301426.3 ns)
  7126396.2 |####################
  7135147.7 |####################
  7143899.2 |########################################
  7152650.7 |####################
  7161402.2 |####################
  7170153.8 |####################
  7178905.3 |####################
  7187656.8 |
  7196408.3 |####################
  7205159.8 |########################################
  7213911.3 |
  7222662.8 |
  7231414.3 |
  7240165.8 |########################################
  7248917.3 |
  7257668.8 |
  7266420.3 |
  7275171.8 |
  7283923.3 |
  7292674.8 |
  (2 below, 3 above range)

cs_plain (n=18, range 6140772.2-6353292.5 ns)
  6140772.2 |
  6151398.2 |#############
  6162024.2 |#############
  6172650.2 |#############
  6183276.3 |#############
  6193902.3 |########################################
  6204528.3 |#############
  6215154.3 |
  6225780.3 |#############
  6236406.3 |
  6247032.4 |
  6257658.4 |
  6268284.4 |#############
  6278910.4 |#############
  6289536.4 |
  6300162.4 |
  6310788.5 |
  6321414.5 |#############
  6332040.5 |#############
  6342666.5 |#############
  (2 below, 2 above range)

cs_snap (n=18, range 8959464.9-9302511.8 ns)
  8959464.9 |#############
  8976617.2 |##########################
  8993769.6 |
  9010921.9 |
  9028074.3 |
  9045226.6 |
  9062379.0 |########################################
  9079531.3 |#############
  9096683.6 |##########################
  9113836.0 |
  9130988.3 |##########################
  9148140.7 |
  9165293.0 |
  9182445.4 |
  9199597.7 |#############
  9216750.0 |#############
  9233902.4 |
  9251054.7 |
  9268207.1 |
  9285359.4 |#############
  (1 below, 3 above range)

cs_snap_twin (n=18, range 8940829.3-9268313.8 ns)
  8940829.3 |##########################
  8957203.5 |#############
  8973577.7 |########################################
  8989951.9 |#############
  9006326.2 |#############
  9022700.4 |#############
  9039074.6 |
  9055448.9 |#############
  9071823.1 |
  9088197.3 |
  9104571.5 |
  9120945.8 |
  9137320.0 |
  9153694.2 |
  9170068.4 |#############
  9186442.7 |
  9202816.9 |##########################
  9219191.1 |#############
  9235565.3 |
  9251939.6 |
  (2 below, 2 above range)

```

## Diagnostics

- **cs_exch**: bridge=427.9% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=495.9% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=337.1% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=340.8% of algo (FFI overhead may distort results)
