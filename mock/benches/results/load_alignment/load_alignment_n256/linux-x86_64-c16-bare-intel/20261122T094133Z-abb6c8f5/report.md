# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 43 ns to slowest 43 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (38 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (43 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### load_byte_pack's comparison is tie-heavy (19% tied pairs)

19% of paired samples for load_byte_pack are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader load_aligned vs stability leader load_byte_pack (+0% speed for 1.0x steadier)

load_aligned is fastest (43 ns, CV 87.5%); load_byte_pack gives up 0.0% median for 1.0x lower variance (CV 85.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### load_aligned is inconsistent: worst-20% is 3.7x its best-20%

load_aligned's best 20% of batches run at 35 ns but its worst 20% at 129 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (load_aligned) is the fastest** at 43.1 ns median
- Spread: 1.00x (fastest 43.1 ns, slowest 43.1 ns)

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
| Counter frequency | 2112580123 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| load_aligned | 2604ns | 1919ns | 1569ns | 1872ns | 5838ns | base |
| load_byte_pack | 2757ns | 1922ns | 1749ns | 2054ns | 5876ns | +5.86% |
| load_unaligned | 2789ns | 1920ns | 1916ns | 1943ns | 6198ns | +7.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 58ns | 35ns | 129ns | base | 4.403 |
| load_byte_pack | 62ns | 39ns | 131ns | +6.17% | 4.147 |
| load_unaligned | 62ns | 43ns | 137ns | +7.16% | 4.109 |

## Performance model

- Peak throughput: **7.264 Gops/s** (load_aligned; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 5.940 | 81.8% |
| load_byte_pack | 5.940 | 81.8% |
| load_unaligned | 5.940 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2604ns | 2604ns | base |
| load_byte_pack | 2757ns | 2757ns | +5.86% |
| load_unaligned | 2789ns | 2789ns | +7.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 43ns | base | --- | [43, 43] | --- | --- | --- | --- |
| load_byte_pack | 43ns | no significant difference | [+0, +0]ns | [43, 43] | no (adj: YES) | 0.0134 | 0.0134 | **31** (19%, HIGH) |
| load_unaligned | 43ns | no significant difference | [+0, +0]ns | [43, 43] | no (adj: YES) | 0.0134 | 0.0090 | **27** (17%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 43ns | +47.8% | +0.0% |
| 1 | 2 | 35ns | +22.4% | +22.1% |
| 1 | 3 | 35ns | +22.2% | +22.2% |
| 1 | 4 | 43ns | +0.0% | +0.0% |
| 1 | 5 | 43ns | +0.0% | +0.0% |
| 1 | 6 | 43ns | +0.0% | -0.2% |
| 1 | 7 | 43ns | +0.0% | +0.0% |
| 1 | 8 | 35ns | +0.0% | +22.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | 0.063 | ok |
| load_byte_pack | -0.002 | ok |
| load_unaligned | -0.161 | ok |

**Consistency summary:**

- **load_byte_pack**: won 0/8, lost 3/8
- **load_unaligned**: won 1/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 31.8ns | 58.1ns | 54.7% | HIGH |
| load_byte_pack | 33.7ns | 61.7ns | 54.6% | HIGH |
| load_unaligned | 34.1ns | 62.3ns | 54.7% | HIGH |

## Distribution (algo ns)

```
load_aligned (n=160, range 35.2-129.1 ns)
     35.2 |##############
     39.9 |########################################
     44.6 |
     49.3 |
     54.0 |
     58.7 |
     63.4 |
     68.1 |
     72.8 |
     77.5 |
     82.2 |
     86.9 |
     91.6 |
     96.3 |
    101.0 |
    105.7 |
    110.4 |
    115.1 |
    119.8 |
    124.5 |
  (17 below, 19 above range)

load_byte_pack (n=160, range 39.2-131.1 ns)
     39.2 |########################################
     43.8 |
     48.4 |#
     53.0 |#
     57.6 |
     62.2 |
     66.8 |##
     71.4 |
     76.0 |#
     80.6 |
     85.1 |
     89.7 |
     94.3 |
     98.9 |
    103.5 |
    108.1 |
    112.7 |
    117.3 |##
    121.9 |
    126.5 |##
  (16 below, 16 above range)

load_unaligned (n=160, range 43.0-136.8 ns)
     43.0 |########################################
     47.7 |
     52.4 |
     57.1 |
     61.8 |
     66.5 |
     71.1 |
     75.8 |
     80.5 |
     85.2 |
     89.9 |
     94.6 |
     99.3 |
    104.0 |
    108.7 |
    113.4 |
    118.1 |
    122.7 |
    127.4 |#
    132.1 |
  (2 below, 18 above range)

```

## Diagnostics

- **load_aligned**: CV=64.9% (high variance, measurements may be unstable)
- **load_aligned**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **load_aligned**: bridge=54.8% of algo (FFI overhead may distort results)
- **load_byte_pack**: CV=59.8% (high variance, measurements may be unstable)
- **load_byte_pack**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **load_byte_pack**: bridge=54.8% of algo (FFI overhead may distort results)
- **load_unaligned**: CV=62.1% (high variance, measurements may be unstable)
- **load_unaligned**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **load_unaligned**: bridge=54.8% of algo (FFI overhead may distort results)
