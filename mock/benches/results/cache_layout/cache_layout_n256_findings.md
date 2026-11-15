# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (layout_aos) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline layout_aos has the worst median (1 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest layout_soa at 1 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### layout_soa is fastest but the noisiest (CV 81.8%)

layout_soa wins on median (1 ns) yet has the highest variance (CV 81.8%), while layout_aos is the steadiest (CV 63.8%, 1 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (1 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### layout_soa's comparison is tie-heavy (17% tied pairs)

17% of paired samples for layout_soa are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### layout_soa is inconsistent: worst-20% is 9.4x its best-20%

layout_soa's best 20% of batches run at 0 ns but its worst 20% at 2 ns (9.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: layout_soa** at 0.8 ns median (-33.3% vs baseline)
- Spread: 1.50x (fastest 0.8 ns, slowest 1.2 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| layout_aos | 2269ns | 2279ns | 2083ns | 2282ns | 2416ns | base |
| layout_soa | 2325ns | 2404ns | 2084ns | 2369ns | 2433ns | +2.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 1ns | 0ns | 2ns | base | 223.094 |
| layout_soa | 1ns | 0ns | 2ns | -15.80% | 264.942 |

## Performance model

- Peak throughput: **1204.706 Gops/s** (layout_soa; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 213.333 | 17.7% |
| layout_soa | 320.000 | 26.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2269ns | 2269ns | base |
| layout_soa | 2325ns | 2325ns | +2.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 1ns | base | --- | [1, 1] | --- | --- | --- | --- |
| layout_soa | 1ns | no significant difference | [-0, +0]ns | [1, 1] | no (adj: YES) | 0.0149 | 0.0149 | **27** (17%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 1ns | -25.0% |
| 1 | 2 | 1ns | +0.0% |
| 1 | 3 | 1ns | +0.0% |
| 1 | 4 | 1ns | +0.0% |
| 1 | 5 | 2ns | -57.9% |
| 1 | 6 | 1ns | -44.8% |
| 1 | 7 | 1ns | +33.3% |
| 1 | 8 | 1ns | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | 0.191 | ok |
| layout_soa | -0.462 | moderate- |

**Consistency summary:**

- **layout_soa**: won 3/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 3.2ns | 1.1ns | 276.9% | HIGH |
| layout_soa | 3.5ns | 1.0ns | 358.0% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 0.3-2.3 ns)
      0.3 |
      0.4 |####################################
      0.5 |
      0.6 |
      0.7 |
      0.8 |########################################
      0.9 |
      1.0 |
      1.1 |####################################
      1.2 |
      1.3 |
      1.4 |
      1.5 |
      1.6 |#######################
      1.7 |
      1.8 |
      1.9 |
      2.0 |##############
      2.1 |
      2.2 |
  (10 below, 14 above range)

layout_soa (n=160, range 0.2-2.0 ns)
      0.2 |
      0.3 |
      0.4 |#####################
      0.5 |
      0.6 |
      0.7 |
      0.8 |########################################
      0.8 |
      0.9 |
      1.0 |
      1.1 |
      1.2 |######################
      1.3 |
      1.4 |
      1.5 |
      1.6 |
      1.6 |##########
      1.7 |
      1.8 |
      1.9 |
  (15 below, 16 above range)

```

## Diagnostics

- **layout_aos**: CV=66.8% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 8.5x (possible bimodal distribution)
- **layout_aos**: bridge=241.7% of algo (FFI overhead may distort results)
- **layout_soa**: CV=67.8% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 9.4x (possible bimodal distribution)
- **layout_soa**: bridge=412.5% of algo (FFI overhead may distort results)
