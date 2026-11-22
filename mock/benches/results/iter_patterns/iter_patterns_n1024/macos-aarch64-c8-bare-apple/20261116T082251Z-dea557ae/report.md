# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (255 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 4 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 2884% faster than the next best (iter_seq)

iter_strided (4 ns) leads iter_seq (110 ns) by 2884%, a clear separation rather than a photo finish. CV 38.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 99% (significant)

iter_strided is -251 ns (99%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 68.8x slower than the field

iter_gather (255 ns) is 68.8x the fastest (4 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 38.8%)

iter_strided wins on median (4 ns) yet has the highest variance (CV 38.8%), while iter_gather is the steadiest (CV 2.9%, 255 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 68.8x the fastest

Fastest iter_strided (4 ns) to slowest iter_gather (255 ns): 68.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_strided is inconsistent: worst-20% is 2.9x its best-20%

iter_strided's best 20% of batches run at 2 ns but its worst 20% at 6 ns (2.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 3.7 ns median (-98.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 68.81x (fastest 3.7 ns, slowest 254.6 ns)

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
| iter_gather | 2418ns | 2385ns | 2371ns | 2392ns | 2544ns | base |
| iter_seq | 2253ns | 2241ns | 2228ns | 2244ns | 2305ns | -6.81% |
| iter_strided | 2175ns | 2145ns | 2123ns | 2146ns | 2313ns | -10.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 256ns | 250ns | 263ns | base | 4.007 |
| iter_seq | 110ns | 106ns | 115ns | -56.82% | 9.278 |
| iter_strided | 4ns | 2ns | 6ns | -98.49% | 265.414 |

## Performance model

- Peak throughput: **492.752 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 4.022 | 0.8% |
| iter_seq | 9.275 | 1.9% |
| iter_strided | 276.757 | 56.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 2418ns | 2418ns | base |
| iter_seq | 2253ns | 2253ns | -6.81% |
| iter_strided | 2175ns | 2175ns | -10.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 255ns | base | --- | [254, 255] | --- | --- | --- | --- |
| iter_seq | 110ns | -144.3ns (-56.7%) | [-145, -144]ns | [110, 111] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 4ns | -251.3ns (-98.7%) | [-252, -250]ns | [3, 4] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 254ns | -56.8% | -98.5% |
| 1 | 2 | 256ns | -56.6% | -98.6% |
| 1 | 3 | 255ns | -56.6% | -98.7% |
| 1 | 4 | 258ns | -57.0% | -98.5% |
| 1 | 5 | 256ns | -56.8% | -98.6% |
| 1 | 6 | 255ns | -56.7% | -98.3% |
| 1 | 7 | 254ns | -56.6% | -98.4% |
| 1 | 8 | 255ns | -57.1% | -98.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | 0.075 | ok |
| iter_seq | -0.009 | ok |
| iter_strided | 0.073 | ok |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 2.6ns | 255.6ns | 1.0% |  |
| iter_seq | 2.7ns | 110.4ns | 2.5% |  |
| iter_strided | 14.7ns | 3.9ns | 382.2% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 249.9-263.5 ns)
    249.9 |#################
    250.6 |########################
    251.3 |####
    251.9 |###############
    252.6 |########################
    253.3 |########################################
    254.0 |###############################
    254.7 |#############
    255.3 |###############################
    256.0 |###############
    256.7 |############################
    257.4 |############################
    258.0 |########
    258.7 |###############
    259.4 |########
    260.1 |##
    260.8 |####
    261.4 |##
    262.1 |
    262.8 |####
  (10 below, 5 above range)

iter_seq (n=160, range 106.1-114.9 ns)
    106.1 |########
    106.5 |######
    107.0 |##############
    107.4 |############
    107.8 |############
    108.3 |##################
    108.7 |
    109.2 |########################################
    109.6 |#####################
    110.1 |#########################
    110.5 |#########################
    110.9 |############
    111.4 |############
    111.8 |################
    112.3 |##################
    112.7 |########
    113.2 |##
    113.6 |############
    114.1 |######
    114.5 |##############
  (13 below, 9 above range)

iter_strided (n=160, range 2.1-5.9 ns)
      2.1 |###############
      2.3 |
      2.5 |##################################
      2.7 |
      2.8 |######################################
      3.0 |
      3.2 |########################
      3.4 |
      3.6 |##################################
      3.8 |
      4.0 |
      4.2 |########################
      4.4 |
      4.6 |########################################
      4.8 |
      5.0 |##############################
      5.2 |
      5.3 |#####
      5.5 |
      5.7 |###############
  (11 below, 11 above range)

```

## Diagnostics

- **iter_strided**: CV=37.2% (high variance, measurements may be unstable)
- **iter_strided**: bridge=400.0% of algo (FFI overhead may distort results)
