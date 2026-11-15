# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (layout_aos) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline layout_aos has the worst median (10 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest layout_soa at 10 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.5% (fastest 10 ns to slowest 10 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (9 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### layout_aos is inconsistent: worst-20% is 3.8x its best-20%

layout_aos's best 20% of batches run at 9 ns but its worst 20% at 33 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: layout_soa** at 10.4 ns median (-0.5% vs baseline)
- Spread: 1.00x (fastest 10.4 ns, slowest 10.4 ns)

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
| Counter frequency | 2112817009 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| layout_aos | 2659ns | 1885ns | 1539ns | 1994ns | 5775ns | base |
| layout_soa | 2594ns | 1890ns | 1611ns | 1898ns | 5662ns | -2.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 15ns | 9ns | 33ns | base | 4.308 |
| layout_soa | 14ns | 9ns | 31ns | -4.96% | 4.533 |

## Performance model

- Peak throughput: **7.496 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 6.124 | 81.7% |
| layout_soa | 6.154 | 82.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2659ns | 2659ns | base |
| layout_soa | 2594ns | 2594ns | -2.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 10ns | base | --- | [10, 10] | --- | --- | --- | --- |
| layout_soa | 10ns | no significant difference | [-0, +0]ns | [10, 10] | no | 0.6208 | 0.6208 | 13 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 9ns | +20.2% |
| 1 | 2 | 9ns | +20.9% |
| 1 | 3 | 10ns | -19.0% |
| 1 | 4 | 10ns | -1.0% |
| 1 | 5 | 10ns | -1.0% |
| 1 | 6 | 9ns | +19.8% |
| 1 | 7 | 10ns | -1.0% |
| 1 | 8 | 9ns | -1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | -0.135 | ok |
| layout_soa | -0.228 | moderate- |

**Consistency summary:**

- **layout_soa**: won 5/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 33.2ns | 14.9ns | 223.6% | HIGH |
| layout_soa | 32.7ns | 14.1ns | 231.9% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 8.5-32.6 ns)
      8.5 |########################
      9.7 |########################################
     10.9 |
     12.1 |
     13.3 |
     14.5 |
     15.7 |
     16.9 |
     18.1 |
     19.3 |
     20.5 |
     21.7 |
     22.9 |#
     24.1 |##
     25.3 |#
     26.5 |###
     27.7 |#
     28.9 |#
     30.1 |#
     31.3 |###
  (20 below, 12 above range)

layout_soa (n=160, range 8.8-30.6 ns)
      8.8 |
      9.9 |########################################
     11.0 |
     12.0 |
     13.1 |
     14.2 |
     15.3 |
     16.4 |
     17.5 |
     18.6 |
     19.7 |
     20.8 |
     21.9 |
     23.0 |
     24.1 |
     25.2 |
     26.3 |#
     27.4 |
     28.4 |#
     29.5 |
  (26 below, 17 above range)

```

## Diagnostics

- **layout_aos**: CV=65.5% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **layout_aos**: bridge=226.8% of algo (FFI overhead may distort results)
- **layout_soa**: CV=63.1% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **layout_soa**: bridge=229.8% of algo (FFI overhead may distort results)
