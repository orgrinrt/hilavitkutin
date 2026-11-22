# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (152 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 31 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 393% faster than the next best (popcount_swar)

popcount_std (31 ns) leads popcount_swar (152 ns) by 393%, a clear separation rather than a photo finish. CV 14.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 80% (significant)

popcount_std is -122 ns (80%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 4.9x slower than the field

popcount_handrolled (152 ns) is 4.9x the fastest (31 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 14.9%)

popcount_std wins on median (31 ns) yet has the highest variance (CV 14.9%), while popcount_handrolled is the steadiest (CV 3.2%, 152 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.9x the fastest

Fastest popcount_std (31 ns) to slowest popcount_handrolled (152 ns): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_swar's edge over baseline is significant but tiny (-1 ns, 0.53%)

popcount_swar differs from baseline popcount_handrolled by -1 ns (0.53%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: popcount_std** at 30.8 ns median (-79.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.94x (fastest 30.8 ns, slowest 152.1 ns)

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
| popcount_handrolled | 2264ns | 2250ns | 2244ns | 2251ns | 2324ns | base |
| popcount_std | 2142ns | 2136ns | 2129ns | 2137ns | 2170ns | -5.42% |
| popcount_swar | 2260ns | 2254ns | 2242ns | 2254ns | 2295ns | -0.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 153ns | 148ns | 161ns | base | 6.677 |
| popcount_std | 32ns | 26ns | 39ns | -79.22% | 32.141 |
| popcount_swar | 152ns | 147ns | 160ns | -0.60% | 6.717 |

## Performance model

- Peak throughput: **38.705 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 6.732 | 17.4% |
| popcount_std | 33.247 | 85.9% |
| popcount_swar | 6.750 | 17.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2264ns | 2264ns | base |
| popcount_std | 2142ns | 2142ns | -5.42% |
| popcount_swar | 2260ns | 2260ns | -0.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 152ns | base | --- | [151, 153] | --- | --- | --- | --- |
| popcount_std | 31ns | -121.7ns (-80.0%) | [-122, -121]ns | [30, 32] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 152ns | -0.8ns (-0.5%) | [-1, -0]ns | [151, 152] | YES | 0.0002 | 0.0002 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 151ns | -76.4% | +0.3% |
| 1 | 2 | 152ns | -80.5% | -0.5% |
| 1 | 3 | 152ns | -80.1% | +0.7% |
| 1 | 4 | 151ns | -79.6% | +0.2% |
| 1 | 5 | 152ns | -80.6% | -1.5% |
| 1 | 6 | 155ns | -79.8% | -1.7% |
| 1 | 7 | 151ns | -79.3% | +1.0% |
| 1 | 8 | 152ns | -80.4% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.153 | ok |
| popcount_std | -0.198 | ok |
| popcount_swar | -0.046 | ok |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.0ns | 153.4ns | 2.0% |  |
| popcount_std | 3.5ns | 31.9ns | 10.9% | HIGH |
| popcount_swar | 2.9ns | 152.4ns | 1.9% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 147.6-161.4 ns)
    147.6 |################
    148.3 |##################
    149.0 |#######################
    149.7 |###########
    150.4 |#####################################
    151.1 |########################################
    151.8 |#######
    152.4 |#######################
    153.1 |################
    153.8 |#######
    154.5 |################
    155.2 |#######################
    155.9 |#########
    156.6 |#######
    157.2 |##############
    157.9 |####
    158.6 |##############
    159.3 |
    160.0 |#######
    160.7 |####
  (16 below, 15 above range)

popcount_std (n=160, range 26.5-39.2 ns)
     26.5 |#######
     27.1 |#########################
     27.7 |#########################
     28.4 |#####################
     29.0 |########################################
     29.6 |###########
     30.3 |##############################
     30.9 |##############
     31.5 |##################
     32.2 |##
     32.8 |##############################
     33.4 |##############
     34.1 |###########
     34.7 |##
     35.3 |###########
     36.0 |#######
     36.6 |#########
     37.3 |###########
     37.9 |#######
     38.5 |##############
  (14 below, 11 above range)

popcount_swar (n=160, range 146.6-160.4 ns)
    146.6 |##################
    147.3 |##################################
    147.9 |################
    148.6 |##########################
    149.3 |################################
    150.0 |#############
    150.7 |#####################
    151.4 |#####################
    152.1 |#############################
    152.8 |########################################
    153.5 |#####
    154.2 |################
    154.8 |#############
    155.5 |#############
    156.2 |#####
    156.9 |########################
    157.6 |#####
    158.3 |#####################
    159.0 |##
    159.7 |
  (13 below, 12 above range)

```

## Diagnostics

- **popcount_std**: bridge=10.7% of algo (FFI overhead may distort results)
