# Seq collection member: live-stream vs snapshot-copy across payload size

2 variants, 120 samples per variant.
Baseline: **seq_live**

## Highlights

Baseline for all deltas below: **seq_live**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### seq_live dominates: 83% faster than the next best (seq_snapshot)

seq_live (820.93 us) leads seq_snapshot (1.50 ms) by 83%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (seq_live)

The baseline seq_live is the fastest (820.93 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (seq_live) is the fastest** at 820932.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.83x (fastest 820932.9 ns, slowest 1499208.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Seq collection member: live-stream vs snapshot-copy across payload size |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112730567 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| seq_live | 829519ns | 821103ns | 816230ns | 821873ns | 865745ns | base |
| seq_snapshot | 1458042ns | 1499712ns | 1226327ns | 1504361ns | 1550799ns | +75.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| seq_live | 829347ns | 816070ns | 865544ns | base | 1.264 |
| seq_snapshot | 1457595ns | 1226001ns | 1550318ns | +75.75% | 0.719 |

## Performance model

- Peak throughput: **1.285 Gops/s** (seq_live; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| seq_live | 1.277 | 99.4% |
| seq_snapshot | 0.699 | 54.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| seq_live | 829519ns | 829519ns | base |
| seq_snapshot | 1458042ns | 1458042ns | +75.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| seq_live | 820933ns | base | --- | [819247, 822000] | --- | --- | --- | --- |
| seq_snapshot | 1499209ns | +676652.7ns (+82.4%) | [+667666, +688255]ns | [1491447, 1516515] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | seq_live | seq_snapshot |
|---|---|---|---|
| 1 | 1 | 821615ns | +86.7% |
| 1 | 2 | 818222ns | +86.6% |
| 1 | 3 | 823593ns | +81.1% |
| 1 | 4 | 827041ns | +38.0% |
| 1 | 5 | 820601ns | +86.3% |
| 2 | 1 | 816508ns | +86.9% |
| 2 | 2 | 835768ns | +76.2% |
| 2 | 3 | 816526ns | +87.4% |
| 2 | 4 | 821747ns | +84.4% |
| 2 | 5 | 819073ns | +83.0% |
| 3 | 1 | 822602ns | +80.0% |
| 3 | 2 | 861237ns | +72.0% |
| 3 | 3 | 817243ns | +87.1% |
| 3 | 4 | 815870ns | +33.5% |
| 3 | 5 | 817368ns | +85.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| seq_live | -0.162 | ok |
| seq_snapshot | -0.207 | moderate- |

**Consistency summary:**

- **seq_snapshot**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| seq_live | 1331696.6ns | 829347.2ns | 160.6% | HIGH |
| seq_snapshot | 1732523.3ns | 1457594.9ns | 118.9% | HIGH |

## Distribution (algo ns)

```
seq_live (n=120, range 816070.4-865544.2 ns)
  816070.4 |########################################
  818544.1 |###########################
  821017.8 |########################
  823491.5 |###########
  825965.2 |#####
  828438.9 |#
  830912.6 |####
  833386.3 |
  835859.9 |
  838333.6 |
  840807.3 |#
  843281.0 |##
  845754.7 |##
  848228.4 |#
  850702.1 |
  853175.8 |####
  855649.5 |####
  858123.2 |
  860596.9 |
  863070.5 |#
  (12 below, 12 above range)

seq_snapshot (n=120, range 1226001.5-1550317.6 ns)
  1226001.5 |
  1242217.3 |
  1258433.1 |
  1274648.9 |
  1290864.7 |
  1307080.5 |
  1323296.3 |
  1339512.1 |
  1355727.9 |
  1371943.7 |
  1388159.5 |
  1404375.4 |
  1420591.2 |#####
  1436807.0 |
  1453022.8 |
  1469238.6 |###############################
  1485454.4 |########################################
  1501670.2 |####################
  1517886.0 |####################################
  1534101.8 |#####################
  (16 below, 11 above range)

```

## Diagnostics

- **seq_live**: bridge=161.7% of algo (FFI overhead may distort results)
- **seq_snapshot**: bridge=118.0% of algo (FFI overhead may distort results)
