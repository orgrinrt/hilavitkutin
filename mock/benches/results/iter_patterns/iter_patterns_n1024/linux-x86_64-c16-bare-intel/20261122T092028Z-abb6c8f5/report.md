# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (298 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 25 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 491% faster than the next best (iter_seq)

iter_strided (25 ns) leads iter_seq (149 ns) by 491%, a clear separation rather than a photo finish. CV 88.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 93% (significant)

iter_strided is -277 ns (93%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 11.8x slower than the field

iter_gather (298 ns) is 11.8x the fastest (25 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 88.6%)

iter_strided wins on median (25 ns) yet has the highest variance (CV 88.6%), while iter_gather is the steadiest (CV 79.6%, 298 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### iter_strided shows alternating (throttle bounce) (autocorr -0.62)

iter_strided's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 11.8x the fastest

Fastest iter_strided (25 ns) to slowest iter_gather (298 ns): 11.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_strided is inconsistent: worst-20% is 3.7x its best-20%

iter_strided's best 20% of batches run at 21 ns but its worst 20% at 77 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 25.3 ns median (-91.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 11.76x (fastest 25.3 ns, slowest 297.5 ns)

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
| Counter frequency | 2112768594 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iter_gather | 2985ns | 2178ns | 2171ns | 2180ns | 6213ns | base |
| iter_seq | 2720ns | 2026ns | 1657ns | 1985ns | 5986ns | -8.90% |
| iter_strided | 2607ns | 1903ns | 1556ns | 1863ns | 5889ns | -12.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 406ns | 297ns | 839ns | base | 2.522 |
| iter_seq | 200ns | 122ns | 438ns | -50.80% | 5.127 |
| iter_strided | 34ns | 21ns | 77ns | -91.54% | 29.811 |

## Performance model

- Peak throughput: **49.566 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.442 | 6.9% |
| iter_seq | 6.854 | 13.8% |
| iter_strided | 40.474 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 2985ns | 2985ns | base |
| iter_seq | 2720ns | 2720ns | -8.90% |
| iter_strided | 2607ns | 2607ns | -12.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 298ns | base | --- | [298, 298] | --- | --- | --- | --- |
| iter_seq | 149ns | -148.3ns (-49.8%) | [-175, -148]ns | [149, 150] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 25ns | -276.7ns (-93.0%) | [-277, -273]ns | [25, 25] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 298ns | -49.7% | -91.5% |
| 1 | 2 | 298ns | -49.8% | -93.0% |
| 1 | 3 | 298ns | -58.9% | -91.5% |
| 1 | 4 | 298ns | -58.9% | -93.0% |
| 1 | 5 | 298ns | -49.8% | -91.5% |
| 1 | 6 | 298ns | -49.8% | -93.0% |
| 1 | 7 | 298ns | -49.7% | -93.0% |
| 1 | 8 | 298ns | -58.9% | -91.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.193 | ok |
| iter_seq | 0.060 | ok |
| iter_strided | -0.625 | HIGH- (thermal bounce) |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 32.5ns | 406.0ns | 8.0% | HIGH |
| iter_seq | 31.9ns | 199.7ns | 16.0% | HIGH |
| iter_strided | 32.2ns | 34.4ns | 93.7% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 297.3-839.3 ns)
    297.3 |########################################
    324.4 |
    351.5 |
    378.6 |
    405.7 |
    432.8 |
    459.9 |
    487.0 |
    514.1 |
    541.2 |
    568.3 |
    595.4 |
    622.5 |
    649.6 |
    676.7 |
    703.8 |
    730.9 |
    758.0 |#
    785.1 |
    812.2 |
  (11 below, 22 above range)

iter_seq (n=160, range 122.3-437.7 ns)
    122.3 |#############
    138.0 |########################################
    153.8 |
    169.6 |
    185.3 |
    201.1 |
    216.9 |
    232.6 |
    248.4 |
    264.2 |
    280.0 |
    295.7 |
    311.5 |
    327.3 |
    343.0 |
    358.8 |
    374.6 |
    390.4 |
    406.1 |#
    421.9 |
  (16 below, 18 above range)

iter_strided (n=160, range 20.7-77.0 ns)
     20.7 |############################
     23.5 |########################################
     26.3 |#
     29.1 |
     31.9 |
     34.7 |
     37.6 |
     40.4 |
     43.2 |
     46.0 |
     48.8 |
     51.6 |
     54.5 |
     57.3 |
     60.1 |#
     62.9 |##
     65.7 |#
     68.5 |###
     71.3 |#
     74.2 |#
  (13 below, 13 above range)

```

## Diagnostics

- **iter_gather**: CV=58.3% (high variance, measurements may be unstable)
- **iter_seq**: CV=63.4% (high variance, measurements may be unstable)
- **iter_seq**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **iter_seq**: bridge=16.0% of algo (FFI overhead may distort results)
- **iter_strided**: CV=65.3% (high variance, measurements may be unstable)
- **iter_strided**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **iter_strided**: bridge=93.1% of algo (FFI overhead may distort results)
