# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (42 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 1 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 1925% faster than the next best (iter_seq)

iter_strided (1 ns) leads iter_seq (16 ns) by 1925%, a clear separation rather than a photo finish. CV 76.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 98% (significant)

iter_strided is -41 ns (98%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 53.1x slower than the field

iter_gather (42 ns) is 53.1x the fastest (1 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 76.3%)

iter_strided wins on median (1 ns) yet has the highest variance (CV 76.3%), while iter_gather is the steadiest (CV 8.3%, 42 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 53.1x the fastest

Fastest iter_strided (1 ns) to slowest iter_gather (42 ns): 53.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_strided is inconsistent: worst-20% is 7.4x its best-20%

iter_strided's best 20% of batches run at 0 ns but its worst 20% at 2 ns (7.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 0.8 ns median (-98.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 53.12x (fastest 0.8 ns, slowest 42.5 ns)

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
| iter_gather | 2150ns | 2133ns | 2129ns | 2134ns | 2218ns | base |
| iter_seq | 2109ns | 2105ns | 2099ns | 2105ns | 2131ns | -1.89% |
| iter_strided | 2099ns | 2096ns | 2091ns | 2096ns | 2116ns | -2.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 42ns | 37ns | 47ns | base | 6.053 |
| iter_seq | 16ns | 13ns | 20ns | -61.45% | 15.702 |
| iter_strided | 1ns | 0ns | 2ns | -97.69% | 261.893 |

## Performance model

- Peak throughput: **975.238 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 6.024 | 0.6% |
| iter_seq | 15.802 | 1.6% |
| iter_strided | 320.000 | 32.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 2150ns | 2150ns | base |
| iter_seq | 2109ns | 2109ns | -1.89% |
| iter_strided | 2099ns | 2099ns | -2.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 42ns | base | --- | [42, 43] | --- | --- | --- | --- |
| iter_seq | 16ns | -26.2ns (-61.6%) | [-27, -26]ns | [16, 17] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 1ns | -41.5ns (-97.5%) | [-42, -41]ns | [1, 1] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 42ns | -60.6% | -98.1% |
| 1 | 2 | 42ns | -62.2% | -98.1% |
| 1 | 3 | 43ns | -63.9% | -97.2% |
| 1 | 4 | 44ns | -62.8% | -98.2% |
| 1 | 5 | 43ns | -61.0% | -97.2% |
| 1 | 6 | 42ns | -60.1% | -98.1% |
| 1 | 7 | 43ns | -62.1% | -98.1% |
| 1 | 8 | 41ns | -60.5% | -98.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.004 | ok |
| iter_seq | 0.320 | moderate+ |
| iter_strided | -0.375 | moderate- |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 2.4ns | 42.3ns | 5.6% | HIGH |
| iter_seq | 2.7ns | 16.3ns | 16.6% | HIGH |
| iter_strided | 2.2ns | 1.0ns | 223.1% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 37.1-47.1 ns)
     37.1 |########
     37.6 |
     38.1 |########
     38.6 |########
     39.1 |########################
     39.6 |##################
     40.1 |#####################
     40.6 |################
     41.1 |################
     41.6 |########################################
     42.1 |########################
     42.6 |########################
     43.1 |##########################
     43.6 |##################
     44.1 |#############################
     44.6 |########
     45.1 |##########
     45.6 |#####################
     46.1 |##################
     46.6 |#############
  (16 below, 11 above range)

iter_seq (n=160, range 12.7-19.8 ns)
     12.7 |##
     13.1 |################
     13.4 |
     13.8 |#####
     14.1 |################
     14.5 |#####################
     14.8 |#############################
     15.2 |########################
     15.5 |##################################
     15.9 |#############################
     16.3 |
     16.6 |########################################
     17.0 |##########################
     17.3 |##################
     17.7 |##################
     18.0 |##################
     18.4 |
     18.7 |##################
     19.1 |##################
     19.4 |#####
  (16 below, 15 above range)

iter_strided (n=160, range 0.3-1.9 ns)
      0.3 |
      0.3 |####################
      0.4 |
      0.5 |
      0.6 |
      0.7 |
      0.8 |########################################
      0.8 |
      0.9 |
      1.0 |
      1.1 |
      1.2 |########################
      1.3 |
      1.4 |
      1.4 |
      1.5 |
      1.6 |
      1.7 |#########
      1.8 |
      1.9 |
  (11 below, 15 above range)

```

## Diagnostics

- **iter_seq**: bridge=15.4% of algo (FFI overhead may distort results)
- **iter_strided**: CV=62.4% (high variance, measurements may be unstable)
- **iter_strided**: worst_20/best_20 = 7.4x (possible bimodal distribution)
- **iter_strided**: bridge=262.5% of algo (FFI overhead may distort results)
