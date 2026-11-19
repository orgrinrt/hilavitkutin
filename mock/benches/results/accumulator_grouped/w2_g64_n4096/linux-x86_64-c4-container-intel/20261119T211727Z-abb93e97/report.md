# W = 2, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (grouped) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline grouped has the worst median (40.56 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sequential at 25.90 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sequential dominates: 39% faster than the next best (per_core)

sequential (25.90 us) leads per_core (35.92 us) by 39%, a clear separation rather than a photo finish. CV 15.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 37% (significant)

sequential is -15.13 us (37%) faster than baseline grouped, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### per_core is inconsistent: worst-20% is 3.4x its best-20%

per_core's best 20% of batches run at 29.89 us but its worst 20% at 100.83 us (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 25895.9 ns median (-36.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.57x (fastest 25895.9 ns, slowest 40563.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 2, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100372148 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 42180ns | 41440ns | 35506ns | 39680ns | 49266ns | base |
| per_core | 56314ns | 36373ns | 30329ns | 34733ns | 101680ns | +33.51% |
| sequential | 28630ns | 26643ns | 24933ns | 26484ns | 33698ns | -32.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 41507ns | 34858ns | 48706ns | base | 0.099 |
| per_core | 55701ns | 29894ns | 100830ns | +34.20% | 0.074 |
| sequential | 28098ns | 24453ns | 33163ns | -32.31% | 0.146 |

## Performance model

- Peak throughput: **0.168 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.101 | 60.3% |
| per_core | 0.114 | 68.1% |
| sequential | 0.158 | 94.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 42180ns | 42180ns | base |
| per_core | 56314ns | 56314ns | +33.51% |
| sequential | 28630ns | 28630ns | -32.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 40564ns | base | --- | [36882, 43844] | --- | --- | --- | --- |
| per_core | 35924ns | no significant difference | [-10515, +2571]ns | [32105, 38878] | no | 0.2379 | 0.2379 | 0 |
| sequential | 25896ns | -15132.5ns (-37.3%) | [-18097, -6307]ns | [24786, 30626] | YES | 0.0003 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 37233ns | +1.1% | -12.8% |
| 1 | 2 | 39642ns | -2.4% | -37.0% |
| 1 | 3 | 42272ns | -25.5% | -41.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.000 | ok |
| per_core | -0.251 | moderate- |
| sequential | -0.149 | ok |

**Consistency summary:**

- **per_core**: won 2/3, lost 1/3
- **sequential**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 40933.2ns | 41506.9ns | 98.6% | HIGH |
| per_core | 51772.8ns | 55701.0ns | 92.9% | HIGH |
| sequential | 27944.0ns | 28097.5ns | 99.5% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 34857.5-48706.0 ns)
  34857.5 |
  35549.9 |##########################
  36242.4 |#############
  36934.8 |########################################
  37627.2 |
  38319.6 |
  39012.1 |
  39704.5 |#############
  40396.9 |#############
  41089.3 |#############
  41781.8 |
  42474.2 |##########################
  43166.6 |
  43859.0 |
  44551.5 |##########################
  45243.9 |
  45936.3 |
  46628.7 |
  47321.2 |#############
  48013.6 |
  (2 below, 2 above range)

per_core (n=18, range 29893.9-100830.2 ns)
  29893.9 |################################
  33440.7 |########################################
  36987.6 |########################
  40534.4 |################
  44081.2 |########
  47628.0 |
  51174.8 |
  54721.6 |
  58268.4 |
  61815.2 |
  65362.1 |
  68908.9 |
  72455.7 |
  76002.5 |
  79549.3 |
  83096.1 |
  86642.9 |
  90189.7 |
  93736.5 |
  97283.4 |
  (2 below, 1 above range)

sequential (n=18, range 24452.7-33163.2 ns)
  24452.7 |########################################
  24888.2 |################
  25323.7 |
  25759.3 |
  26194.8 |
  26630.3 |########
  27065.8 |
  27501.4 |
  27936.9 |########
  28372.4 |
  28808.0 |
  29243.5 |########
  29679.0 |########
  30114.5 |
  30550.1 |
  30985.6 |
  31421.1 |################
  31856.6 |
  32292.2 |
  32727.7 |
  (2 below, 3 above range)

```

## Diagnostics

- **grouped**: bridge=101.1% of algo (FFI overhead may distort results)
- **per_core**: CV=148.2% (high variance, measurements may be unstable)
- **per_core**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **per_core**: bridge=97.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=102.0% of algo (FFI overhead may distort results)
