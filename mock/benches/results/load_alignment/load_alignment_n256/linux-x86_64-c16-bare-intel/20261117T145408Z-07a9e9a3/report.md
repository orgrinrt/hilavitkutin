# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### load_aligned is fastest but the noisiest (CV 105.2%)

load_aligned wins on median (35 ns) yet has the highest variance (CV 105.2%), while load_unaligned is the steadiest (CV 81.7%, 43 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (8 ns) is smaller than the fastest variant's own run-to-run std-dev (37 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (35 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### load_byte_pack's comparison is tie-heavy (13% tied pairs)

13% of paired samples for load_byte_pack are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### load_byte_pack is inconsistent: worst-20% is 3.7x its best-20%

load_byte_pack's best 20% of batches run at 35 ns but its worst 20% at 129 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (load_aligned) is the fastest** at 35.4 ns median
- Spread: 1.22x (fastest 35.4 ns, slowest 43.1 ns)

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
| Counter frequency | 2112743394 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| load_aligned | 2643ns | 1593ns | 1574ns | 2004ns | 5630ns | base |
| load_byte_pack | 2580ns | 1925ns | 1575ns | 1820ns | 5867ns | -2.38% |
| load_unaligned | 2562ns | 1926ns | 1574ns | 1868ns | 5632ns | -3.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 59ns | 35ns | 126ns | base | 4.333 |
| load_byte_pack | 57ns | 35ns | 129ns | -3.05% | 4.469 |
| load_unaligned | 57ns | 35ns | 124ns | -3.52% | 4.491 |

## Performance model

- Peak throughput: **7.275 Gops/s** (load_aligned; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 7.232 | 99.4% |
| load_byte_pack | 5.940 | 81.6% |
| load_unaligned | 5.940 | 81.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2643ns | 2643ns | base |
| load_byte_pack | 2580ns | 2580ns | -2.38% |
| load_unaligned | 2562ns | 2562ns | -3.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 35ns | base | --- | [35, 43] | --- | --- | --- | --- |
| load_byte_pack | 43ns | no significant difference | [+0, +0]ns | [43, 43] | no | 0.0541 | 0.0271 | **21** (13%, HIGH) |
| load_unaligned | 43ns | no significant difference | [+0, +8]ns | [43, 43] | no | 0.0600 | 0.0600 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 35ns | +22.1% | +0.0% |
| 1 | 2 | 35ns | +22.1% | +22.1% |
| 1 | 3 | 43ns | +0.0% | -18.3% |
| 1 | 4 | 43ns | -18.1% | -18.1% |
| 1 | 5 | 35ns | +22.1% | +22.1% |
| 1 | 6 | 35ns | +0.0% | +22.1% |
| 1 | 7 | 35ns | +0.0% | +0.0% |
| 1 | 8 | 35ns | +0.0% | +22.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | 0.292 | moderate+ |
| load_byte_pack | 0.124 | ok |
| load_unaligned | -0.375 | moderate- |

**Consistency summary:**

- **load_byte_pack**: won 1/8, lost 3/8
- **load_unaligned**: won 2/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 32.2ns | 59.1ns | 54.6% | HIGH |
| load_byte_pack | 31.9ns | 57.3ns | 55.7% | HIGH |
| load_unaligned | 31.2ns | 57.0ns | 54.7% | HIGH |

## Distribution (algo ns)

```
load_aligned (n=160, range 35.2-125.6 ns)
     35.2 |########################################
     39.7 |################
     44.2 |
     48.8 |
     53.3 |#
     57.8 |
     62.3 |
     66.8 |
     71.4 |
     75.9 |
     80.4 |#
     84.9 |
     89.4 |
     94.0 |
     98.5 |##
    103.0 |##
    107.5 |####
    112.1 |#
    116.6 |###
    121.1 |
  (3 below, 11 above range)

load_byte_pack (n=160, range 35.2-129.2 ns)
     35.2 |#########################
     39.9 |########################################
     44.6 |#
     49.3 |#
     54.0 |
     58.7 |
     63.4 |
     68.1 |#
     72.8 |
     77.5 |
     82.2 |
     86.9 |
     91.6 |
     96.3 |
    101.0 |
    105.7 |
    110.4 |
    115.1 |#
    119.8 |
    124.5 |
  (24 below, 22 above range)

load_unaligned (n=160, range 35.2-124.1 ns)
     35.2 |#####################
     39.7 |########################################
     44.1 |
     48.6 |
     53.0 |#
     57.5 |
     61.9 |
     66.3 |
     70.8 |
     75.2 |
     79.7 |
     84.1 |
     88.6 |
     93.0 |#
     97.5 |#
    101.9 |#
    106.3 |#
    110.8 |####
    115.2 |#
    119.7 |
  (17 below, 12 above range)

```

## Diagnostics

- **load_aligned**: CV=63.0% (high variance, measurements may be unstable)
- **load_aligned**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **load_aligned**: bridge=55.1% of algo (FFI overhead may distort results)
- **load_byte_pack**: CV=69.0% (high variance, measurements may be unstable)
- **load_byte_pack**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **load_byte_pack**: bridge=54.8% of algo (FFI overhead may distort results)
- **load_unaligned**: CV=61.7% (high variance, measurements may be unstable)
- **load_unaligned**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **load_unaligned**: bridge=54.8% of algo (FFI overhead may distort results)
