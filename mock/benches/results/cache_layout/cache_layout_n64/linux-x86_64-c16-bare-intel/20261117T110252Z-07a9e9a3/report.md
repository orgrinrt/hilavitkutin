# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (layout_aos) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline layout_aos has the worst median (10 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest layout_soa at 10 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.0% (fastest 10 ns to slowest 10 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (9 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### layout_aos is inconsistent: worst-20% is 3.9x its best-20%

layout_aos's best 20% of batches run at 9 ns but its worst 20% at 33 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: layout_soa** at 10.4 ns median (-1.0% vs baseline)
- Spread: 1.01x (fastest 10.4 ns, slowest 10.5 ns)

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
| layout_aos | 2651ns | 1895ns | 1550ns | 1882ns | 6058ns | base |
| layout_soa | 2786ns | 1896ns | 1560ns | 2152ns | 5913ns | +5.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 15ns | 9ns | 33ns | base | 4.368 |
| layout_soa | 15ns | 9ns | 32ns | +4.37% | 4.185 |

## Performance model

- Peak throughput: **7.485 Gops/s** (layout_soa; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 6.095 | 81.4% |
| layout_soa | 6.154 | 82.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2651ns | 2651ns | base |
| layout_soa | 2786ns | 2786ns | +5.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 10ns | base | --- | [10, 10] | --- | --- | --- | --- |
| layout_soa | 10ns | no significant difference | [-0, +0]ns | [10, 10] | no | 0.0600 | 0.0600 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 9ns | +20.9% |
| 1 | 2 | 9ns | -1.7% |
| 1 | 3 | 9ns | -1.2% |
| 1 | 4 | 11ns | -1.9% |
| 1 | 5 | 11ns | -0.9% |
| 1 | 6 | 11ns | +142.5% |
| 1 | 7 | 9ns | +19.5% |
| 1 | 8 | 11ns | -0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | 0.142 | ok |
| layout_soa | -0.062 | ok |

**Consistency summary:**

- **layout_soa**: won 5/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 33.1ns | 14.7ns | 226.1% | HIGH |
| layout_soa | 35.2ns | 15.3ns | 229.9% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 8.6-33.3 ns)
      8.6 |######################################
      9.8 |########################################
     11.1 |
     12.3 |
     13.5 |
     14.8 |
     16.0 |
     17.2 |
     18.5 |
     19.7 |
     20.9 |
     22.2 |
     23.4 |
     24.6 |
     25.9 |
     27.1 |#
     28.4 |##
     29.6 |
     30.8 |###
     32.1 |##
  (6 below, 16 above range)

layout_soa (n=160, range 8.5-32.2 ns)
      8.5 |###
      9.7 |########################################
     10.9 |
     12.1 |
     13.3 |
     14.5 |
     15.6 |
     16.8 |
     18.0 |
     19.2 |
     20.4 |
     21.5 |
     22.7 |
     23.9 |
     25.1 |
     26.3 |##
     27.5 |
     28.6 |#
     29.8 |#
     31.0 |
  (24 below, 16 above range)

```

## Diagnostics

- **layout_aos**: CV=66.2% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **layout_aos**: bridge=225.7% of algo (FFI overhead may distort results)
- **layout_soa**: CV=60.4% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **layout_soa**: bridge=230.8% of algo (FFI overhead may distort results)
