# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (82 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 12 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 270% faster than the next best (iter_seq)

iter_strided (12 ns) leads iter_seq (43 ns) by 270%, a clear separation rather than a photo finish. CV 88.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 86% (significant)

iter_strided is -71 ns (86%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 7.1x slower than the field

iter_gather (82 ns) is 7.1x the fastest (12 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 88.6%)

iter_strided wins on median (12 ns) yet has the highest variance (CV 88.6%), while iter_gather is the steadiest (CV 86.9%, 82 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 7.1x the fastest

Fastest iter_strided (12 ns) to slowest iter_gather (82 ns): 7.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_seq is inconsistent: worst-20% is 3.7x its best-20%

iter_seq's best 20% of batches run at 35 ns but its worst 20% at 131 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 11.6 ns median (-85.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 7.07x (fastest 11.6 ns, slowest 82.4 ns)

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
| iter_gather | 2748ns | 1961ns | 1601ns | 2073ns | 5917ns | base |
| iter_seq | 2631ns | 1921ns | 1569ns | 1895ns | 5902ns | -4.25% |
| iter_strided | 2714ns | 1888ns | 1544ns | 2104ns | 5712ns | -1.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 115ns | 67ns | 246ns | base | 2.228 |
| iter_seq | 59ns | 35ns | 131ns | -48.93% | 4.361 |
| iter_strided | 17ns | 10ns | 35ns | -85.47% | 15.332 |

## Performance model

- Peak throughput: **26.912 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.107 | 11.5% |
| iter_seq | 5.940 | 22.1% |
| iter_strided | 21.974 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 2748ns | 2748ns | base |
| iter_seq | 2631ns | 2631ns | -4.25% |
| iter_strided | 2714ns | 2714ns | -1.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 82ns | base | --- | [82, 82] | --- | --- | --- | --- |
| iter_seq | 43ns | -39.4ns (-47.8%) | [-39, -39]ns | [43, 43] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 12ns | -71.0ns (-86.1%) | [-73, -71]ns | [12, 12] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 83ns | -47.8% | -88.4% |
| 1 | 2 | 68ns | -36.2% | -85.8% |
| 1 | 3 | 68ns | -47.7% | -82.7% |
| 1 | 4 | 68ns | -36.2% | -82.7% |
| 1 | 5 | 68ns | -47.7% | -85.8% |
| 1 | 6 | 82ns | -47.8% | -85.8% |
| 1 | 7 | 83ns | -47.8% | -88.3% |
| 1 | 8 | 82ns | -57.3% | -88.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | 0.373 | moderate+ |
| iter_seq | -0.474 | moderate- |
| iter_strided | -0.141 | ok |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 33.4ns | 114.9ns | 29.0% | HIGH |
| iter_seq | 32.2ns | 58.7ns | 54.8% | HIGH |
| iter_strided | 34.1ns | 16.7ns | 204.1% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 67.4-246.1 ns)
     67.4 |######################
     76.4 |########################################
     85.3 |
     94.2 |
    103.2 |
    112.1 |
    121.0 |
    130.0 |
    138.9 |
    147.8 |
    156.8 |#
    165.7 |
    174.6 |
    183.6 |#
    192.5 |###
    201.4 |##
    210.4 |##
    219.3 |#
    228.3 |#
    237.2 |
  (19 below, 15 above range)

iter_seq (n=160, range 35.2-130.6 ns)
     35.2 |#############
     40.0 |########################################
     44.8 |
     49.5 |
     54.3 |
     59.1 |
     63.8 |
     68.6 |
     73.4 |
     78.1 |
     82.9 |
     87.7 |
     92.5 |
     97.2 |
    102.0 |
    106.8 |#
    111.5 |
    116.3 |#
    121.1 |
    125.8 |#
  (18 below, 16 above range)

iter_strided (n=160, range 9.5-35.0 ns)
      9.5 |##################################
     10.8 |########################################
     12.1 |
     13.3 |
     14.6 |
     15.9 |
     17.2 |#
     18.4 |
     19.7 |#
     21.0 |
     22.3 |
     23.5 |
     24.8 |
     26.1 |####
     27.4 |#####
     28.6 |##
     29.9 |###
     31.2 |##
     32.5 |###
     33.7 |#
  (27 below, 15 above range)

```

## Diagnostics

- **iter_gather**: CV=62.3% (high variance, measurements may be unstable)
- **iter_gather**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **iter_gather**: bridge=28.6% of algo (FFI overhead may distort results)
- **iter_seq**: CV=64.0% (high variance, measurements may be unstable)
- **iter_seq**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **iter_seq**: bridge=54.5% of algo (FFI overhead may distort results)
- **iter_strided**: CV=61.8% (high variance, measurements may be unstable)
- **iter_strided**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **iter_strided**: bridge=204.3% of algo (FFI overhead may distort results)
