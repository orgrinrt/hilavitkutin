# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (148 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 35 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 327% faster than the next best (popcount_swar)

popcount_std (35 ns) leads popcount_swar (148 ns) by 327%, a clear separation rather than a photo finish. CV 23.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 77% (significant)

popcount_std is -114 ns (77%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 4.3x slower than the field

popcount_handrolled (148 ns) is 4.3x the fastest (35 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 23.3%)

popcount_std wins on median (35 ns) yet has the highest variance (CV 23.3%), while popcount_handrolled is the steadiest (CV 3.2%, 148 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.3x the fastest

Fastest popcount_std (35 ns) to slowest popcount_handrolled (148 ns): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_std is inconsistent: worst-20% is 1.6x its best-20%

popcount_std's best 20% of batches run at 29 ns but its worst 20% at 45 ns (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### popcount_swar's edge over baseline is significant but tiny (-1 ns, 0.44%)

popcount_swar differs from baseline popcount_handrolled by -1 ns (0.44%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: popcount_std** at 34.6 ns median (-76.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.29x (fastest 34.6 ns, slowest 148.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Popcount lowering: std count_ones vs Kernighan loop vs SWAR |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x24680123456789ab |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| popcount_handrolled | 2231ns | 2224ns | 2216ns | 2224ns | 2265ns | base |
| popcount_std | 2125ns | 2110ns | 2104ns | 2114ns | 2179ns | -4.74% |
| popcount_swar | 2226ns | 2221ns | 2216ns | 2221ns | 2250ns | -0.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 149ns | 143ns | 156ns | base | 6.864 |
| popcount_std | 36ns | 29ns | 45ns | -76.12% | 28.745 |
| popcount_swar | 149ns | 143ns | 156ns | -0.36% | 6.889 |

## Performance model

- Peak throughput: **35.208 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 6.896 | 19.6% |
| popcount_std | 29.595 | 84.1% |
| popcount_swar | 6.924 | 19.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2231ns | 2231ns | base |
| popcount_std | 2125ns | 2125ns | -4.74% |
| popcount_swar | 2226ns | 2226ns | -0.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 148ns | base | --- | [148, 150] | --- | --- | --- | --- |
| popcount_std | 35ns | -114.1ns (-76.8%) | [-115, -113]ns | [33, 35] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 148ns | -0.6ns (-0.4%) | [-1, -0]ns | [147, 149] | YES | 0.0061 | 0.0061 | 5 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 148ns | -76.2% | -0.3% |
| 1 | 2 | 148ns | -78.0% | -0.4% |
| 1 | 3 | 148ns | -77.0% | -0.7% |
| 1 | 4 | 149ns | -76.2% | -0.2% |
| 1 | 5 | 150ns | -76.9% | -1.0% |
| 1 | 6 | 147ns | -77.4% | +0.5% |
| 1 | 7 | 148ns | -76.2% | +0.6% |
| 1 | 8 | 150ns | -76.9% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.263 | moderate- |
| popcount_std | -0.309 | moderate- |
| popcount_swar | -0.083 | ok |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 6/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 2.7ns | 149.2ns | 1.8% |  |
| popcount_std | 2.1ns | 35.6ns | 5.9% | HIGH |
| popcount_swar | 2.9ns | 148.7ns | 2.0% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 143.5-156.5 ns)
    143.5 |########
    144.1 |##################################
    144.8 |################################
    145.4 |################
    146.1 |##################
    146.7 |################
    147.4 |########################################
    148.0 |########
    148.7 |##########################
    149.3 |#############
    150.0 |#############################
    150.6 |##################
    151.3 |#############
    151.9 |#############
    152.6 |#####
    153.2 |##########################
    153.9 |########
    154.5 |################
    155.2 |##
    155.8 |
  (15 below, 15 above range)

popcount_std (n=160, range 29.1-45.3 ns)
     29.1 |##########################
     29.9 |##########################
     30.7 |#####################
     31.5 |#####################################
     32.3 |########################################
     33.1 |##################
     34.0 |################################
     34.8 |#############################
     35.6 |#####################
     36.4 |##################
     37.2 |########
     38.0 |################
     38.8 |##################
     39.6 |#############
     40.4 |##################
     41.3 |
     42.1 |#############
     42.9 |########
     43.7 |##
     44.5 |########
  (11 below, 7 above range)

popcount_swar (n=160, range 142.6-155.8 ns)
    142.6 |########
    143.3 |############################
    144.0 |######################
    144.6 |##############
    145.3 |########################################
    145.9 |###########
    146.6 |#########################
    147.2 |######################
    147.9 |###############################
    148.5 |######################
    149.2 |##############
    149.9 |######################
    150.5 |##
    151.2 |#########################
    151.8 |#################
    152.5 |######################
    153.1 |######################
    153.8 |#####
    154.5 |####################
    155.1 |##
  (15 below, 10 above range)

```

## Diagnostics

- **popcount_std**: CV=22.6% (high variance, measurements may be unstable)
