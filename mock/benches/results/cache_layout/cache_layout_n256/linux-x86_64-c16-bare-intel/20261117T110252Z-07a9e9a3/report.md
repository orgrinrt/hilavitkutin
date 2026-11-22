# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (layout_aos) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline layout_aos has the worst median (12 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest layout_soa at 12 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.8% (fastest 12 ns to slowest 12 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (11 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### layout_soa's comparison is tie-heavy (19% tied pairs)

19% of paired samples for layout_soa are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### layout_aos is inconsistent: worst-20% is 3.8x its best-20%

layout_aos's best 20% of batches run at 10 ns but its worst 20% at 39 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: layout_soa** at 12.2 ns median (-0.8% vs baseline)
- Spread: 1.01x (fastest 12.2 ns, slowest 12.3 ns)

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
| layout_aos | 2738ns | 1897ns | 1620ns | 1971ns | 6155ns | base |
| layout_soa | 2575ns | 1896ns | 1552ns | 1875ns | 5699ns | -5.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 18ns | 10ns | 39ns | base | 14.568 |
| layout_soa | 17ns | 10ns | 36ns | -5.46% | 15.409 |

## Performance model

- Peak throughput: **25.632 Gops/s** (layout_soa; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 20.813 | 81.2% |
| layout_soa | 20.984 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2738ns | 2738ns | base |
| layout_soa | 2575ns | 2575ns | -5.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 12ns | base | --- | [12, 12] | --- | --- | --- | --- |
| layout_soa | 12ns | no significant difference | [-0, +0]ns | [12, 12] | no (adj: YES) | 0.0001 | 0.0001 | **30** (19%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 12ns | +0.0% |
| 1 | 2 | 10ns | +21.4% |
| 1 | 3 | 12ns | -17.6% |
| 1 | 4 | 12ns | -0.8% |
| 1 | 5 | 12ns | +0.0% |
| 1 | 6 | 10ns | -0.5% |
| 1 | 7 | 12ns | -18.7% |
| 1 | 8 | 12ns | -18.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | -0.371 | moderate- |
| layout_soa | 0.154 | ok |

**Consistency summary:**

- **layout_soa**: won 5/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 34.6ns | 17.6ns | 196.8% | HIGH |
| layout_soa | 32.4ns | 16.6ns | 194.7% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 10.4-39.3 ns)
     10.4 |
     11.9 |########################################
     13.3 |
     14.7 |
     16.2 |
     17.6 |
     19.1 |
     20.5 |
     22.0 |
     23.4 |
     24.9 |
     26.3 |
     27.7 |
     29.2 |
     30.6 |
     32.1 |#
     33.5 |#
     35.0 |
     36.4 |##
     37.9 |
  (26 below, 17 above range)

layout_soa (n=160, range 10.0-36.4 ns)
     10.0 |#################################
     11.3 |########################################
     12.6 |#
     14.0 |
     15.3 |
     16.6 |
     17.9 |
     19.2 |
     20.6 |
     21.9 |
     23.2 |#
     24.5 |#
     25.8 |#
     27.2 |
     28.5 |#
     29.8 |#
     31.1 |#
     32.4 |#
     33.8 |#
     35.1 |#
  (4 below, 15 above range)

```

## Diagnostics

- **layout_aos**: CV=64.0% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **layout_aos**: bridge=194.3% of algo (FFI overhead may distort results)
- **layout_soa**: CV=63.8% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **layout_soa**: bridge=195.9% of algo (FFI overhead may distort results)
