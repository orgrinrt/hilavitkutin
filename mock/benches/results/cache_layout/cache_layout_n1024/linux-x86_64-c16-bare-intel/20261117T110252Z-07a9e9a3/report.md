# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (layout_aos) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline layout_aos has the worst median (28 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest layout_soa at 28 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.4% (fastest 28 ns to slowest 28 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (24 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### layout_soa's comparison is tie-heavy (12% tied pairs)

12% of paired samples for layout_soa are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### layout_aos is inconsistent: worst-20% is 3.8x its best-20%

layout_aos's best 20% of batches run at 23 ns but its worst 20% at 86 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: layout_soa** at 27.9 ns median (-0.4% vs baseline)
- Spread: 1.00x (fastest 27.9 ns, slowest 28.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | AoS vs SoA (column-store) layout under partial-field iteration |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eadbeef01234567 |
| Counter frequency | 2112773604 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| layout_aos | 2657ns | 1913ns | 1564ns | 1917ns | 5969ns | base |
| layout_soa | 2699ns | 1914ns | 1565ns | 2089ns | 5663ns | +1.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 39ns | 23ns | 86ns | base | 26.540 |
| layout_soa | 39ns | 23ns | 81ns | +1.24% | 26.214 |

## Performance model

- Peak throughput: **44.790 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 36.571 | 81.7% |
| layout_soa | 36.703 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2657ns | 2657ns | base |
| layout_soa | 2699ns | 2699ns | +1.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 28ns | base | --- | [28, 28] | --- | --- | --- | --- |
| layout_soa | 28ns | no significant difference | [-0, +0]ns | [28, 28] | no | 0.6135 | 0.6135 | **19** (12%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 28ns | +0.2% |
| 1 | 2 | 23ns | +21.3% |
| 1 | 3 | 23ns | +22.3% |
| 1 | 4 | 28ns | -17.9% |
| 1 | 5 | 28ns | -18.2% |
| 1 | 6 | 28ns | +0.0% |
| 1 | 7 | 28ns | -17.9% |
| 1 | 8 | 23ns | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | 0.056 | ok |
| layout_soa | 0.120 | ok |

**Consistency summary:**

- **layout_soa**: won 3/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 32.7ns | 38.6ns | 84.8% | HIGH |
| layout_soa | 33.0ns | 39.1ns | 84.5% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 22.9-86.1 ns)
     22.9 |###############
     26.0 |########################################
     29.2 |
     32.3 |
     35.5 |
     38.7 |
     41.8 |
     45.0 |
     48.2 |
     51.3 |
     54.5 |
     57.6 |
     60.8 |
     64.0 |
     67.1 |
     70.3 |
     73.5 |
     76.6 |##
     79.8 |
     82.9 |#
  (12 below, 16 above range)

layout_soa (n=160, range 22.9-81.5 ns)
     22.9 |########################
     25.8 |########################################
     28.7 |
     31.7 |
     34.6 |
     37.5 |
     40.4 |
     43.4 |
     46.3 |#
     49.2 |
     52.2 |
     55.1 |
     58.0 |
     61.0 |##
     63.9 |
     66.8 |####
     69.7 |####
     72.7 |
     75.6 |##
     78.5 |
  (9 below, 13 above range)

```

## Diagnostics

- **layout_aos**: CV=63.7% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **layout_aos**: bridge=84.3% of algo (FFI overhead may distort results)
- **layout_soa**: CV=60.2% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **layout_soa**: bridge=84.6% of algo (FFI overhead may distort results)
