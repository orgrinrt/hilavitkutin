# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (44 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 1 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 1200% faster than the next best (iter_seq)

iter_strided (1 ns) leads iter_seq (10 ns) by 1200%, a clear separation rather than a photo finish. CV 85.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 98% (significant)

iter_strided is -43 ns (98%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 55.2x slower than the field

iter_gather (44 ns) is 55.2x the fastest (1 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 85.0%)

iter_strided wins on median (1 ns) yet has the highest variance (CV 85.0%), while iter_gather is the steadiest (CV 11.3%, 44 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 55.2x the fastest

Fastest iter_strided (1 ns) to slowest iter_gather (44 ns): 55.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_strided is inconsistent: worst-20% is 12.2x its best-20%

iter_strided's best 20% of batches run at 0 ns but its worst 20% at 2 ns (12.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 0.8 ns median (-98.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 55.25x (fastest 0.8 ns, slowest 44.2 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iter_gather | 2166ns | 2160ns | 2152ns | 2160ns | 2198ns | base |
| iter_seq | 2142ns | 2131ns | 2124ns | 2132ns | 2191ns | -1.09% |
| iter_strided | 2126ns | 2121ns | 2115ns | 2122ns | 2153ns | -1.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 44ns | 38ns | 50ns | base | 5.827 |
| iter_seq | 11ns | 8ns | 15ns | -75.17% | 23.471 |
| iter_strided | 1ns | 0ns | 2ns | -97.87% | 274.163 |

## Performance model

- Peak throughput: **1575.385 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 5.792 | 0.4% |
| iter_seq | 24.615 | 1.6% |
| iter_strided | 320.000 | 20.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 2166ns | 2166ns | base |
| iter_seq | 2142ns | 2142ns | -1.09% |
| iter_strided | 2126ns | 2126ns | -1.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 44ns | base | --- | [43, 45] | --- | --- | --- | --- |
| iter_seq | 10ns | -32.5ns (-73.5%) | [-33, -32]ns | [10, 11] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 1ns | -43.3ns (-98.1%) | [-44, -42]ns | [1, 1] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 45ns | -76.1% | -98.2% |
| 1 | 2 | 45ns | -77.8% | -98.2% |
| 1 | 3 | 44ns | -74.9% | -97.3% |
| 1 | 4 | 44ns | -73.2% | -98.6% |
| 1 | 5 | 45ns | -76.7% | -98.2% |
| 1 | 6 | 44ns | -76.1% | -98.2% |
| 1 | 7 | 44ns | -76.7% | -98.2% |
| 1 | 8 | 44ns | -76.0% | -98.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | 0.172 | ok |
| iter_seq | -0.017 | ok |
| iter_strided | -0.439 | moderate- |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 3.1ns | 43.9ns | 7.0% | HIGH |
| iter_seq | 2.5ns | 10.9ns | 22.9% | HIGH |
| iter_strided | 2.3ns | 0.9ns | 251.3% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 38.1-49.6 ns)
     38.1 |####
     38.6 |##############
     39.2 |##############
     39.8 |########
     40.4 |##########
     40.9 |##############
     41.5 |##########
     42.1 |####################
     42.7 |####################
     43.2 |##############
     43.8 |##########
     44.4 |######################
     45.0 |########################
     45.5 |##########
     46.1 |##############
     46.7 |########################################
     47.3 |########
     47.8 |################
     48.4 |######
     49.0 |####
  (12 below, 7 above range)

iter_seq (n=160, range 7.6-15.0 ns)
      7.6 |#############
      8.0 |################################
      8.4 |
      8.7 |########################
      9.1 |########################################
      9.5 |#############
      9.8 |##################
     10.2 |#####################################
     10.6 |#####################
     10.9 |##########################
     11.3 |
     11.7 |#############################
     12.0 |##################
     12.4 |##########################
     12.8 |#####################
     13.1 |#####################
     13.5 |##########
     13.9 |########
     14.2 |##
     14.6 |
  (14 below, 9 above range)

iter_strided (n=160, range 0.2-2.0 ns)
      0.2 |
      0.3 |
      0.3 |########################################
      0.4 |
      0.5 |
      0.6 |
      0.7 |##################################
      0.8 |
      0.9 |
      1.0 |
      1.1 |
      1.2 |#######################
      1.3 |
      1.3 |
      1.4 |
      1.5 |
      1.6 |####################
      1.7 |
      1.8 |
      1.9 |
  (19 below, 16 above range)

```

## Diagnostics

- **iter_seq**: CV=30.2% (high variance, measurements may be unstable)
- **iter_seq**: bridge=24.0% of algo (FFI overhead may distort results)
- **iter_strided**: CV=72.8% (high variance, measurements may be unstable)
- **iter_strided**: worst_20/best_20 = 12.2x (possible bimodal distribution)
- **iter_strided**: bridge=262.5% of algo (FFI overhead may distort results)
