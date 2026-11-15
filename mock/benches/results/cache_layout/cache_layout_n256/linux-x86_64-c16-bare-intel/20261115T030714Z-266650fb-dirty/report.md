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

### layout_soa is fastest but the noisiest (CV 93.0%)

layout_soa wins on median (12 ns) yet has the highest variance (CV 93.0%), while layout_aos is the steadiest (CV 89.5%, 12 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (11 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### layout_soa's comparison is tie-heavy (11% tied pairs)

11% of paired samples for layout_soa are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader layout_soa vs stability leader layout_aos (+1% speed for 1.0x steadier)

layout_soa is fastest (12 ns, CV 93.0%); layout_aos gives up 0.8% median for 1.0x lower variance (CV 89.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### layout_soa is inconsistent: worst-20% is 3.9x its best-20%

layout_soa's best 20% of batches run at 10 ns but its worst 20% at 38 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: layout_soa** at 12.1 ns median (-0.8% vs baseline)
- Spread: 1.01x (fastest 12.1 ns, slowest 12.2 ns)

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
| layout_aos | 2659ns | 1889ns | 1596ns | 1918ns | 5946ns | base |
| layout_soa | 2599ns | 1885ns | 1542ns | 1809ns | 6026ns | -2.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 17ns | 10ns | 38ns | base | 15.022 |
| layout_soa | 17ns | 10ns | 38ns | -2.50% | 15.407 |

## Performance model

- Peak throughput: **25.883 Gops/s** (layout_soa; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 20.984 | 81.1% |
| layout_soa | 21.157 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2659ns | 2659ns | base |
| layout_soa | 2599ns | 2599ns | -2.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 12ns | base | --- | [12, 12] | --- | --- | --- | --- |
| layout_soa | 12ns | no significant difference | [-0, +0]ns | [12, 12] | no (adj: YES) | 0.0118 | 0.0118 | **17** (11%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 12ns | -18.4% |
| 1 | 2 | 10ns | +0.0% |
| 1 | 3 | 10ns | +22.0% |
| 1 | 4 | 12ns | +0.0% |
| 1 | 5 | 12ns | -18.0% |
| 1 | 6 | 12ns | +0.4% |
| 1 | 7 | 12ns | -18.0% |
| 1 | 8 | 12ns | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | 0.292 | moderate+ |
| layout_soa | -0.372 | moderate- |

**Consistency summary:**

- **layout_soa**: won 3/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 33.6ns | 17.0ns | 197.2% | HIGH |
| layout_soa | 33.2ns | 16.6ns | 200.0% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 10.3-37.9 ns)
     10.3 |
     11.6 |########################################
     13.0 |
     14.4 |
     15.8 |
     17.2 |
     18.5 |
     19.9 |
     21.3 |
     22.7 |
     24.1 |
     25.5 |
     26.8 |
     28.2 |#
     29.6 |
     31.0 |
     32.4 |
     33.8 |#
     35.1 |#
     36.5 |
  (27 below, 16 above range)

layout_soa (n=160, range 9.9-38.2 ns)
      9.9 |#######################################
     11.3 |########################################
     12.7 |
     14.1 |
     15.6 |
     17.0 |
     18.4 |
     19.8 |
     21.2 |
     22.6 |
     24.1 |
     25.5 |
     26.9 |
     28.3 |#
     29.7 |#
     31.1 |
     32.6 |#
     34.0 |
     35.4 |#
     36.8 |#
  (3 below, 17 above range)

```

## Diagnostics

- **layout_aos**: CV=64.1% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **layout_aos**: bridge=195.1% of algo (FFI overhead may distort results)
- **layout_soa**: CV=67.7% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **layout_soa**: bridge=195.9% of algo (FFI overhead may distort results)
