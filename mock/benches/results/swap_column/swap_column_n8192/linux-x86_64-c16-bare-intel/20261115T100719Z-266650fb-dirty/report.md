# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cs_plain dominates: 14% faster than the next best (cs_exch)

cs_plain (6.29 ms) leads cs_exch (7.15 ms) by 14%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (cs_plain)

The baseline cs_plain is the fastest (6.29 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (cs_plain) is the fastest** at 6286429.7 ns median
- 3 variants significantly slower than baseline
- Spread: 1.45x (fastest 6286429.7 ns, slowest 9112223.9 ns)

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
| Counter frequency | 2112762221 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cs_exch | 7154719ns | 7155621ns | 7099086ns | 7141223ns | 7202779ns | +13.59% |
| cs_plain | 6298582ns | 6290203ns | 6171722ns | 6276600ns | 6394985ns | base |
| cs_snap | 9121692ns | 9089671ns | 9020848ns | 9080024ns | 9234617ns | +44.82% |
| cs_snap_twin | 9100583ns | 9116009ns | 8879021ns | 9053165ns | 9282491ns | +44.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 7150922ns | 7095254ns | 7198984ns | +13.60% | 0.001 |
| cs_plain | 6294849ns | 6168054ns | 6391206ns | base | 0.001 |
| cs_snap | 9117964ns | 9017138ns | 9230807ns | +44.85% | 0.001 |
| cs_snap_twin | 9096782ns | 8875314ns | 9278710ns | +44.51% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (cs_plain; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.001 | 86.2% |
| cs_plain | 0.001 | 98.1% |
| cs_snap | 0.001 | 67.9% |
| cs_snap_twin | 0.001 | 67.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 7154719ns | 7154719ns | +13.59% |
| cs_plain | 6298582ns | 6298582ns | base |
| cs_snap | 9121692ns | 9121692ns | +44.82% |
| cs_snap_twin | 9100583ns | 9100583ns | +44.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 6286430ns | base | --- | [6249510, 6361486] | --- | --- | --- | --- |
| cs_exch | 7151977ns | +849289.0ns (+13.5%) | [+814386, +904209]ns | [7120656, 7167492] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap | 9086130ns | +2838522.6ns (+45.2%) | [+2776630, +2892325]ns | [9043334, 9146710] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap_twin | 9112224ns | +2808811.2ns (+44.7%) | [+2655537, +2910927]ns | [8955537, 9230059] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 6286525ns | +14.1% | +43.9% | +47.4% |
| 1 | 2 | 6251505ns | +14.6% | +46.3% | +44.8% |
| 1 | 3 | 6274998ns | +13.5% | +48.1% | +46.2% |
| 2 | 1 | 6274045ns | +13.5% | +44.0% | +42.6% |
| 2 | 2 | 6286334ns | +13.2% | +44.7% | +41.2% |
| 2 | 3 | 6420474ns | +11.7% | +40.8% | +44.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | 0.176 | ok |
| cs_plain | 0.060 | ok |
| cs_snap | -0.150 | ok |
| cs_snap_twin | -0.196 | ok |

**Consistency summary:**

- **cs_exch**: won 0/6, lost 6/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 30761749.5ns | 7150922.1ns | 430.2% | HIGH |
| cs_plain | 30902329.4ns | 6294849.2ns | 490.9% | HIGH |
| cs_snap | 30821537.1ns | 9117963.7ns | 338.0% | HIGH |
| cs_snap_twin | 30825986.8ns | 9096782.4ns | 338.9% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 7095254.2-7198984.3 ns)
  7095254.2 |####################
  7100440.7 |
  7105627.2 |
  7110813.7 |####################
  7116000.2 |########################################
  7121186.7 |####################
  7126373.2 |########################################
  7131559.8 |
  7136746.3 |
  7141932.8 |
  7147119.3 |####################
  7152305.8 |########################################
  7157492.3 |####################
  7162678.8 |####################
  7167865.3 |########################################
  7173051.8 |
  7178238.3 |
  7183424.8 |
  7188611.3 |
  7193797.8 |
  (1 below, 3 above range)

cs_plain (n=18, range 6168054.2-6391205.8 ns)
  6168054.2 |####################
  6179211.7 |
  6190369.3 |####################
  6201526.9 |####################
  6212684.5 |
  6223842.1 |
  6234999.6 |
  6246157.2 |########################################
  6257314.8 |
  6268472.4 |########################################
  6279630.0 |########################################
  6290787.5 |
  6301945.1 |########################################
  6313102.7 |
  6324260.3 |
  6335417.9 |
  6346575.4 |####################
  6357733.0 |
  6368890.6 |########################################
  6380048.2 |
  (1 below, 3 above range)

cs_snap (n=18, range 9017138.2-9230807.0 ns)
  9017138.2 |
  9027821.6 |##########################
  9038505.1 |########################################
  9049188.5 |
  9059871.9 |#############
  9070555.4 |##########################
  9081238.8 |
  9091922.3 |#############
  9102605.7 |
  9113289.1 |#############
  9123972.6 |#############
  9134656.0 |
  9145339.5 |##########################
  9156022.9 |
  9166706.4 |
  9177389.8 |
  9188073.2 |
  9198756.7 |
  9209440.1 |
  9220123.6 |
  (1 below, 4 above range)

cs_snap_twin (n=18, range 8875314.1-9278710.1 ns)
  8875314.1 |####################
  8895483.9 |
  8915653.7 |
  8935823.5 |########################################
  8955993.3 |########################################
  8976163.1 |
  8996332.9 |####################
  9016502.7 |
  9036672.5 |####################
  9056842.3 |
  9077012.1 |
  9097181.9 |
  9117351.7 |
  9137521.5 |
  9157691.3 |####################
  9177861.1 |########################################
  9198030.9 |####################
  9218200.7 |
  9238370.5 |####################
  9258540.3 |########################################
  (2 below, 2 above range)

```

## Diagnostics

- **cs_exch**: bridge=428.8% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=490.8% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=337.8% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=337.2% of algo (FFI overhead may distort results)
