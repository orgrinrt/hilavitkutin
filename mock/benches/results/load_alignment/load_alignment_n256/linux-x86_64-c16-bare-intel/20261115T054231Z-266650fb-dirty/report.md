# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 43 ns to slowest 43 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### load_aligned is fastest but the noisiest (CV 92.4%)

load_aligned wins on median (43 ns) yet has the highest variance (CV 92.4%), while load_byte_pack is the steadiest (CV 83.1%, 43 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (40 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (43 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### load_byte_pack's comparison is tie-heavy (25% tied pairs)

25% of paired samples for load_byte_pack are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader load_aligned vs stability leader load_byte_pack (+0% speed for 1.1x steadier)

load_aligned is fastest (43 ns, CV 92.4%); load_byte_pack gives up 0.0% median for 1.1x lower variance (CV 83.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### load_aligned is inconsistent: worst-20% is 3.9x its best-20%

load_aligned's best 20% of batches run at 35 ns but its worst 20% at 137 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (load_aligned) is the fastest** at 43.0 ns median
- Spread: 1.00x (fastest 43.0 ns, slowest 43.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x2c8af49106b7ed53 |
| Counter frequency | 2112776509 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| load_aligned | 2705ns | 1918ns | 1567ns | 1944ns | 6127ns | base |
| load_byte_pack | 2570ns | 1918ns | 1567ns | 1870ns | 5675ns | -5.00% |
| load_unaligned | 2737ns | 1918ns | 1568ns | 1987ns | 6157ns | +1.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 61ns | 35ns | 137ns | base | 4.220 |
| load_byte_pack | 57ns | 35ns | 125ns | -5.65% | 4.473 |
| load_unaligned | 61ns | 35ns | 136ns | +0.59% | 4.196 |

## Performance model

- Peak throughput: **7.278 Gops/s** (load_byte_pack; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 5.953 | 81.8% |
| load_byte_pack | 5.953 | 81.8% |
| load_unaligned | 5.953 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2705ns | 2705ns | base |
| load_byte_pack | 2570ns | 2570ns | -5.00% |
| load_unaligned | 2737ns | 2737ns | +1.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 43ns | base | --- | [43, 43] | --- | --- | --- | --- |
| load_byte_pack | 43ns | no significant difference | [-0, +0]ns | [43, 43] | no | 0.2407 | 0.1203 | **40** (25%, HIGH) |
| load_unaligned | 43ns | no significant difference | [-0, +0]ns | [43, 43] | no | 0.6660 | 0.6660 | **26** (16%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 35ns | +0.0% | +0.0% |
| 1 | 2 | 35ns | -0.3% | +21.8% |
| 1 | 3 | 43ns | -0.1% | -0.1% |
| 1 | 4 | 43ns | +0.2% | -18.1% |
| 1 | 5 | 43ns | +0.0% | +0.0% |
| 1 | 6 | 43ns | +0.2% | -18.1% |
| 1 | 7 | 35ns | -0.1% | +22.0% |
| 1 | 8 | 43ns | -0.2% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | 0.058 | ok |
| load_byte_pack | 0.065 | ok |
| load_unaligned | -0.476 | moderate- |

**Consistency summary:**

- **load_byte_pack**: won 4/8, lost 2/8
- **load_unaligned**: won 3/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 33.3ns | 60.7ns | 54.9% | HIGH |
| load_byte_pack | 31.5ns | 57.2ns | 55.0% | HIGH |
| load_unaligned | 33.4ns | 61.0ns | 54.8% | HIGH |

## Distribution (algo ns)

```
load_aligned (n=160, range 35.2-137.1 ns)
     35.2 |##########
     40.3 |########################################
     45.4 |
     50.5 |
     55.6 |
     60.7 |
     65.8 |
     70.9 |#
     76.0 |
     81.1 |
     86.1 |
     91.2 |
     96.3 |
    101.4 |
    106.5 |##
    111.6 |
    116.7 |#
    121.8 |
    126.9 |###
    132.0 |
  (27 below, 17 above range)

load_byte_pack (n=160, range 35.2-125.4 ns)
     35.2 |#####################
     39.7 |########################################
     44.2 |
     48.7 |
     53.2 |
     57.7 |
     62.3 |
     66.8 |
     71.3 |
     75.8 |
     80.3 |#
     84.8 |
     89.3 |
     93.8 |
     98.4 |
    102.9 |
    107.4 |#
    111.9 |#
    116.4 |#
    120.9 |
  (8 below, 19 above range)

load_unaligned (n=160, range 35.2-135.9 ns)
     35.2 |#####################
     40.2 |########################################
     45.3 |
     50.3 |
     55.3 |
     60.4 |
     65.4 |
     70.4 |#
     75.5 |
     80.5 |
     85.5 |
     90.6 |
     95.6 |
    100.7 |
    105.7 |#
    110.7 |#
    115.8 |##
    120.8 |#
    125.8 |###
    130.9 |#
  (6 below, 16 above range)

```

## Diagnostics

- **load_aligned**: CV=65.5% (high variance, measurements may be unstable)
- **load_aligned**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **load_aligned**: bridge=54.7% of algo (FFI overhead may distort results)
- **load_byte_pack**: CV=62.4% (high variance, measurements may be unstable)
- **load_byte_pack**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **load_byte_pack**: bridge=54.7% of algo (FFI overhead may distort results)
- **load_unaligned**: CV=64.3% (high variance, measurements may be unstable)
- **load_unaligned**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **load_unaligned**: bridge=54.7% of algo (FFI overhead may distort results)
