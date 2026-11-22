# Seq collection member: live-stream vs snapshot-copy across payload size

2 variants, 120 samples per variant.
Baseline: **seq_live**

## Highlights

Baseline for all deltas below: **seq_live**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### seq_live dominates: 129% faster than the next best (seq_snapshot)

seq_live (356.61 us) leads seq_snapshot (817.13 us) by 129%, a clear separation rather than a photo finish. CV 18.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (seq_live)

The baseline seq_live is the fastest (356.61 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### seq_snapshot is inconsistent: worst-20% is 1.6x its best-20%

seq_snapshot's best 20% of batches run at 805.17 us but its worst 20% at 1.26 ms (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (seq_live) is the fastest** at 356609.8 ns median
- 1 variant significantly slower than baseline
- Spread: 2.29x (fastest 356609.8 ns, slowest 817134.0 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| seq_live | 389094ns | 358060ns | 352195ns | 358583ns | 517524ns | base |
| seq_snapshot | 940141ns | 817391ns | 805440ns | 878042ns | 1261137ns | +141.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| seq_live | 387716ns | 350713ns | 516330ns | base | 10.818 |
| seq_snapshot | 939702ns | 805167ns | 1260318ns | +142.37% | 4.463 |

## Performance model

- Peak throughput: **11.959 Gops/s** (seq_live; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| seq_live | 11.762 | 98.3% |
| seq_snapshot | 5.133 | 42.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| seq_live | 389094ns | 389094ns | base |
| seq_snapshot | 940141ns | 940141ns | +141.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| seq_live | 356610ns | base | --- | [355479, 357476] | --- | --- | --- | --- |
| seq_snapshot | 817134ns | +462713.7ns (+129.8%) | [+458567, +467538]ns | [813556, 821830] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | seq_live | seq_snapshot |
|---|---|---|---|
| 1 | 1 | 352869ns | +128.0% |
| 1 | 2 | 350637ns | +245.2% |
| 1 | 3 | 530323ns | +135.5% |
| 1 | 4 | 358581ns | +242.7% |
| 1 | 5 | 521702ns | +138.2% |
| 2 | 1 | 355388ns | +130.6% |
| 2 | 2 | 530852ns | +134.2% |
| 2 | 3 | 356963ns | +128.6% |
| 2 | 4 | 356719ns | +127.1% |
| 2 | 5 | 352466ns | +129.9% |
| 3 | 1 | 351253ns | +131.2% |
| 3 | 2 | 351153ns | +130.1% |
| 3 | 3 | 352965ns | +129.1% |
| 3 | 4 | 356668ns | +127.5% |
| 3 | 5 | 360728ns | +123.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| seq_live | -0.246 | moderate- |
| seq_snapshot | 0.375 | moderate+ |

**Consistency summary:**

- **seq_snapshot**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| seq_live | 6897936.9ns | 387716.0ns | 1779.1% | HIGH |
| seq_snapshot | 6926510.1ns | 939701.6ns | 737.1% | HIGH |

## Distribution (algo ns)

```
seq_live (n=120, range 350713.2-516330.2 ns)
  350713.2 |########################################
  358994.1 |#####
  367274.9 |#
  375555.8 |
  383836.6 |
  392117.5 |
  400398.3 |
  408679.2 |
  416960.0 |
  425240.9 |
  433521.7 |
  441802.6 |
  450083.4 |
  458364.3 |
  466645.1 |
  474926.0 |
  483206.8 |
  491487.7 |
  499768.5 |
  508049.4 |#
  (13 below, 18 above range)

seq_snapshot (n=120, range 805166.5-1260317.9 ns)
  805166.5 |########################################
  827924.1 |
  850681.7 |
  873439.2 |
  896196.8 |
  918954.4 |
  941712.0 |
  964469.5 |
  987227.1 |
  1009984.7 |
  1032742.2 |
  1055499.8 |
  1078257.4 |
  1101014.9 |
  1123772.5 |
  1146530.1 |
  1169287.6 |#
  1192045.2 |###
  1214802.8 |##
  1237560.3 |######
  (9 below, 9 above range)

```

## Diagnostics

- **seq_live**: bridge=1928.2% of algo (FFI overhead may distort results)
- **seq_snapshot**: CV=20.7% (high variance, measurements may be unstable)
- **seq_snapshot**: bridge=849.5% of algo (FFI overhead may distort results)
