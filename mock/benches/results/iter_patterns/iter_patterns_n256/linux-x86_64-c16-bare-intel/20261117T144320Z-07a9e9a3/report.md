# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (82 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 10 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 269% faster than the next best (iter_seq)

iter_strided (10 ns) leads iter_seq (35 ns) by 269%, a clear separation rather than a photo finish. CV 109.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 86% (significant)

iter_strided is -71 ns (86%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 8.6x slower than the field

iter_gather (82 ns) is 8.6x the fastest (10 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 109.4%)

iter_strided wins on median (10 ns) yet has the highest variance (CV 109.4%), while iter_gather is the steadiest (CV 89.1%, 82 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 8.6x the fastest

Fastest iter_strided (10 ns) to slowest iter_gather (82 ns): 8.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_seq is inconsistent: worst-20% is 3.7x its best-20%

iter_seq's best 20% of batches run at 35 ns but its worst 20% at 132 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 9.6 ns median (-88.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 8.57x (fastest 9.6 ns, slowest 82.3 ns)

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
| Counter frequency | 2112828630 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iter_gather | 2736ns | 1964ns | 1608ns | 1994ns | 6092ns | base |
| iter_seq | 2567ns | 1592ns | 1575ns | 1779ns | 5925ns | -6.18% |
| iter_strided | 2575ns | 1556ns | 1549ns | 1839ns | 5807ns | -5.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 114ns | 67ns | 251ns | base | 2.249 |
| iter_seq | 57ns | 35ns | 132ns | -49.69% | 4.470 |
| iter_strided | 16ns | 9ns | 35ns | -86.13% | 16.221 |

## Performance model

- Peak throughput: **26.965 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.111 | 11.5% |
| iter_seq | 7.232 | 26.8% |
| iter_strided | 26.667 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 2736ns | 2736ns | base |
| iter_seq | 2567ns | 2567ns | -6.18% |
| iter_strided | 2575ns | 2575ns | -5.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 82ns | base | --- | [68, 82] | --- | --- | --- | --- |
| iter_seq | 35ns | -39.2ns (-47.6%) | [-39, -32]ns | [35, 43] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 10ns | -70.7ns (-85.8%) | [-73, -58]ns | [10, 10] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 82ns | -57.2% | -85.8% |
| 1 | 2 | 68ns | -47.8% | -85.7% |
| 1 | 3 | 68ns | -47.7% | -85.8% |
| 1 | 4 | 68ns | -47.7% | -85.8% |
| 1 | 5 | 68ns | -47.5% | -85.8% |
| 1 | 6 | 68ns | -47.7% | -85.8% |
| 1 | 7 | 83ns | -47.8% | -88.4% |
| 1 | 8 | 82ns | -47.7% | -88.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | 0.259 | moderate+ |
| iter_seq | 0.456 | moderate+ |
| iter_strided | 0.005 | ok |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 33.2ns | 113.8ns | 29.2% | HIGH |
| iter_seq | 31.3ns | 57.3ns | 54.7% | HIGH |
| iter_strided | 32.3ns | 15.8ns | 204.6% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 67.4-251.3 ns)
     67.4 |########################################
     76.6 |#######################################
     85.8 |
     95.0 |
    104.2 |
    113.4 |
    122.6 |
    131.8 |
    141.0 |
    150.2 |
    159.4 |
    168.6 |##
    177.8 |#
    186.9 |
    196.1 |##
    205.3 |##
    214.5 |#
    223.7 |#
    232.9 |
    242.1 |####
  (23 below, 16 above range)

iter_seq (n=160, range 35.2-131.9 ns)
     35.2 |########################################
     40.0 |##################
     44.9 |
     49.7 |
     54.5 |
     59.4 |
     64.2 |
     69.0 |
     73.9 |
     78.7 |
     83.5 |
     88.4 |
     93.2 |
     98.0 |
    102.9 |
    107.7 |
    112.5 |#
    117.4 |#
    122.2 |
    127.0 |##
  (4 below, 15 above range)

iter_strided (n=160, range 9.5-35.3 ns)
      9.5 |########################################
     10.8 |######
     12.1 |
     13.4 |
     14.7 |
     16.0 |
     17.2 |#
     18.5 |
     19.8 |
     21.1 |
     22.4 |
     23.7 |
     25.0 |
     26.3 |#
     27.6 |#
     28.9 |
     30.2 |#
     31.5 |#
     32.7 |
     34.0 |#
  (2 below, 14 above range)

```

## Diagnostics

- **iter_gather**: CV=64.4% (high variance, measurements may be unstable)
- **iter_gather**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **iter_gather**: bridge=28.7% of algo (FFI overhead may distort results)
- **iter_seq**: CV=67.4% (high variance, measurements may be unstable)
- **iter_seq**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **iter_seq**: bridge=55.1% of algo (FFI overhead may distort results)
- **iter_strided**: CV=66.6% (high variance, measurements may be unstable)
- **iter_strided**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **iter_strided**: bridge=204.2% of algo (FFI overhead may distort results)
