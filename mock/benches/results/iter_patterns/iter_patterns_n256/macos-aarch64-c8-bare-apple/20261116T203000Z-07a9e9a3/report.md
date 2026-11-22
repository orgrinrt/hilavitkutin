# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (43 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 1 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 1925% faster than the next best (iter_seq)

iter_strided (1 ns) leads iter_seq (16 ns) by 1925%, a clear separation rather than a photo finish. CV 86.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 98% (significant)

iter_strided is -42 ns (98%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 53.6x slower than the field

iter_gather (43 ns) is 53.6x the fastest (1 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 86.3%)

iter_strided wins on median (1 ns) yet has the highest variance (CV 86.3%), while iter_gather is the steadiest (CV 8.7%, 43 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 53.6x the fastest

Fastest iter_strided (1 ns) to slowest iter_gather (43 ns): 53.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_strided is inconsistent: worst-20% is 9.8x its best-20%

iter_strided's best 20% of batches run at 0 ns but its worst 20% at 2 ns (9.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 0.8 ns median (-98.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 53.62x (fastest 0.8 ns, slowest 42.9 ns)

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
| iter_gather | 2145ns | 2134ns | 2128ns | 2135ns | 2191ns | base |
| iter_seq | 2132ns | 2110ns | 2101ns | 2121ns | 2196ns | -0.60% |
| iter_strided | 2102ns | 2097ns | 2091ns | 2097ns | 2130ns | -1.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 43ns | 37ns | 47ns | base | 6.023 |
| iter_seq | 16ns | 13ns | 20ns | -61.55% | 15.663 |
| iter_strided | 1ns | 0ns | 2ns | -97.65% | 256.481 |

## Performance model

- Peak throughput: **1204.706 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 5.967 | 0.5% |
| iter_seq | 15.802 | 1.3% |
| iter_strided | 320.000 | 26.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 2145ns | 2145ns | base |
| iter_seq | 2132ns | 2132ns | -0.60% |
| iter_strided | 2102ns | 2102ns | -1.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 43ns | base | --- | [42, 44] | --- | --- | --- | --- |
| iter_seq | 16ns | -26.2ns (-61.1%) | [-27, -26]ns | [16, 17] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 1ns | -42.1ns (-98.1%) | [-42, -41]ns | [1, 1] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 43ns | -61.7% | -98.1% |
| 1 | 2 | 42ns | -62.3% | -97.1% |
| 1 | 3 | 44ns | -63.3% | -98.2% |
| 1 | 4 | 44ns | -63.4% | -97.7% |
| 1 | 5 | 42ns | -59.8% | -98.1% |
| 1 | 6 | 43ns | -60.1% | -98.1% |
| 1 | 7 | 43ns | -60.1% | -98.1% |
| 1 | 8 | 43ns | -62.2% | -97.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.175 | ok |
| iter_seq | 0.318 | moderate+ |
| iter_strided | -0.386 | moderate- |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 2.4ns | 42.5ns | 5.6% | HIGH |
| iter_seq | 2.8ns | 16.3ns | 17.0% | HIGH |
| iter_strided | 2.3ns | 1.0ns | 234.9% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 37.0-47.2 ns)
     37.0 |####
     37.5 |#############
     38.0 |####
     38.5 |######
     39.0 |####################
     39.5 |####################
     40.0 |########
     40.6 |###########
     41.1 |###############
     41.6 |#############
     42.1 |######################
     42.6 |#################
     43.1 |#################
     43.6 |#############
     44.2 |########################################
     44.7 |###############
     45.2 |##
     45.7 |#####################################
     46.2 |######
     46.7 |######
  (12 below, 14 above range)

iter_seq (n=160, range 12.9-19.5 ns)
     12.9 |###########
     13.2 |#################
     13.6 |########
     13.9 |#########################
     14.2 |
     14.6 |####################
     14.9 |######################
     15.2 |##################################
     15.6 |######################
     15.9 |########################################
     16.2 |
     16.6 |######################
     16.9 |#########################
     17.2 |#####################################
     17.6 |
     17.9 |#####################################
     18.2 |######################
     18.5 |####################
     18.9 |#################
     19.2 |
  (11 below, 14 above range)

iter_strided (n=160, range 0.2-2.1 ns)
      0.2 |
      0.3 |
      0.4 |##########################
      0.5 |
      0.6 |
      0.7 |
      0.8 |########################################
      0.9 |
      1.0 |
      1.1 |
      1.1 |##############################
      1.2 |
      1.3 |
      1.4 |
      1.5 |
      1.6 |##############
      1.7 |
      1.8 |
      1.9 |
      2.0 |
  (15 below, 16 above range)

```

## Diagnostics

- **iter_seq**: bridge=15.4% of algo (FFI overhead may distort results)
- **iter_strided**: CV=69.1% (high variance, measurements may be unstable)
- **iter_strided**: worst_20/best_20 = 9.8x (possible bimodal distribution)
- **iter_strided**: bridge=262.5% of algo (FFI overhead may distort results)
