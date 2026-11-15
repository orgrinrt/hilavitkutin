# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (82 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 12 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 271% faster than the next best (iter_seq)

iter_strided (12 ns) leads iter_seq (43 ns) by 271%, a clear separation rather than a photo finish. CV 89.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 86% (significant)

iter_strided is -71 ns (86%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 7.1x slower than the field

iter_gather (82 ns) is 7.1x the fastest (12 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 89.0%)

iter_strided wins on median (12 ns) yet has the highest variance (CV 89.0%), while iter_gather is the steadiest (CV 81.3%, 82 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### iter_gather shows alternating (throttle bounce) (autocorr -0.68)

iter_gather's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 7.1x the fastest

Fastest iter_strided (12 ns) to slowest iter_gather (82 ns): 7.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_strided is inconsistent: worst-20% is 3.8x its best-20%

iter_strided's best 20% of batches run at 9 ns but its worst 20% at 36 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 11.6 ns median (-85.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 7.10x (fastest 11.6 ns, slowest 82.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Column iteration patterns: sequential vs strided vs data-dependent gather |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x3a5c8e91b2d4f607 |
| Counter frequency | 2112774242 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iter_gather | 2759ns | 1962ns | 1604ns | 2147ns | 5751ns | base |
| iter_seq | 2701ns | 1919ns | 1568ns | 2032ns | 5840ns | -2.12% |
| iter_strided | 2716ns | 1889ns | 1543ns | 2052ns | 5881ns | -1.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 115ns | 67ns | 238ns | base | 2.223 |
| iter_seq | 61ns | 35ns | 130ns | -47.42% | 4.228 |
| iter_strided | 17ns | 9ns | 36ns | -85.56% | 15.390 |

## Performance model

- Peak throughput: **26.965 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.107 | 11.5% |
| iter_seq | 5.953 | 22.1% |
| iter_strided | 22.069 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 2759ns | 2759ns | base |
| iter_seq | 2701ns | 2701ns | -2.12% |
| iter_strided | 2716ns | 2716ns | -1.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 82ns | base | --- | [82, 82] | --- | --- | --- | --- |
| iter_seq | 43ns | -39.4ns (-47.8%) | [-40, -39]ns | [43, 43] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 12ns | -70.8ns (-85.9%) | [-71, -71]ns | [12, 12] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 82ns | -57.3% | -85.8% |
| 1 | 2 | 68ns | -47.8% | -85.8% |
| 1 | 3 | 82ns | -57.2% | -88.5% |
| 1 | 4 | 68ns | -47.7% | -82.7% |
| 1 | 5 | 82ns | -47.7% | -85.8% |
| 1 | 6 | 67ns | -36.1% | -85.8% |
| 1 | 7 | 82ns | -47.7% | -85.9% |
| 1 | 8 | 82ns | -47.8% | -85.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.676 | HIGH- (thermal bounce) |
| iter_seq | 0.625 | HIGH+ (drift/warm-up) |
| iter_strided | -0.155 | ok |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 33.4ns | 115.2ns | 29.0% | HIGH |
| iter_seq | 33.0ns | 60.5ns | 54.5% | HIGH |
| iter_strided | 34.0ns | 16.6ns | 204.4% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 67.4-238.2 ns)
     67.4 |#########
     75.9 |########################################
     84.5 |
     93.0 |
    101.6 |
    110.1 |
    118.6 |
    127.2 |
    135.7 |
    144.3 |
    152.8 |
    161.3 |
    169.9 |
    178.4 |#
    187.0 |#
    195.5 |##
    204.0 |#
    212.6 |#
    221.1 |##
    229.7 |#
  (21 below, 14 above range)

iter_seq (n=160, range 35.2-130.3 ns)
     35.2 |################################
     39.9 |########################################
     44.7 |
     49.5 |
     54.2 |#
     59.0 |
     63.7 |#
     68.5 |
     73.2 |
     78.0 |#
     82.8 |
     87.5 |#
     92.3 |
     97.0 |##
    101.8 |#
    106.5 |##
    111.3 |
    116.1 |##
    120.8 |#
    125.6 |####
  (7 below, 14 above range)

iter_strided (n=160, range 9.5-35.8 ns)
      9.5 |##################
     10.8 |########################################
     12.1 |
     13.4 |
     14.8 |
     16.1 |#
     17.4 |
     18.7 |
     20.0 |
     21.3 |
     22.6 |
     24.0 |
     25.3 |
     26.6 |
     27.9 |#
     29.2 |#
     30.5 |#
     31.8 |##
     33.2 |###
     34.5 |###
  (5 below, 11 above range)

```

## Diagnostics

- **iter_gather**: CV=58.2% (high variance, measurements may be unstable)
- **iter_gather**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **iter_gather**: bridge=28.6% of algo (FFI overhead may distort results)
- **iter_seq**: CV=62.0% (high variance, measurements may be unstable)
- **iter_seq**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **iter_seq**: autocorrelation=0.63 (measurement drift or warm-up artifact)
- **iter_seq**: bridge=54.7% of algo (FFI overhead may distort results)
- **iter_strided**: CV=62.1% (high variance, measurements may be unstable)
- **iter_strided**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **iter_strided**: bridge=205.2% of algo (FFI overhead may distort results)
