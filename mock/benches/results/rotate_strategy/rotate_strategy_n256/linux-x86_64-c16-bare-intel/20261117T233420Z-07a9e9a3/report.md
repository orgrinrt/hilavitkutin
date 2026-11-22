# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 2.8x slower than the field

rot_variable_amount (59 ns) is 2.8x the fastest (21 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rot_intrinsic_const is fastest but the noisiest (CV 108.7%)

rot_intrinsic_const wins on median (21 ns) yet has the highest variance (CV 108.7%), while rot_variable_amount is the steadiest (CV 90.4%, 59 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (rot_intrinsic_const, rot_manual_shifts) are a dead heat (<1%)

rot_intrinsic_const (21 ns) and rot_manual_shifts (21 ns) differ by 0.24%, inside the noise, even though the wider field spreads 182.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (rot_intrinsic_const)

The baseline rot_intrinsic_const is the fastest (21 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### rot_manual_shifts's comparison is tie-heavy (23% tied pairs)

23% of paired samples for rot_manual_shifts are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### rot_variable_amount is inconsistent: worst-20% is 3.8x its best-20%

rot_variable_amount's best 20% of batches run at 48 ns but its worst 20% at 183 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (rot_intrinsic_const) is the fastest** at 20.9 ns median
- 1 variant significantly slower than baseline
- Spread: 2.82x (fastest 20.9 ns, slowest 58.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Rotate strategy: intrinsic const vs manual shifts vs variable amount |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x71f692ab4e08d325 |
| Counter frequency | 2112804954 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rot_intrinsic_const | 2570ns | 1564ns | 1559ns | 1841ns | 5766ns | base |
| rot_manual_shifts | 2521ns | 1565ns | 1560ns | 1744ns | 5812ns | -1.91% |
| rot_variable_amount | 2654ns | 1939ns | 1587ns | 1857ns | 6111ns | +3.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 34ns | 21ns | 76ns | base | 7.528 |
| rot_manual_shifts | 33ns | 21ns | 76ns | -1.89% | 7.673 |
| rot_variable_amount | 80ns | 48ns | 183ns | +135.07% | 3.203 |

## Performance model

- Peak throughput: **12.345 Gops/s** (rot_intrinsic_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 12.278 | 99.5% |
| rot_manual_shifts | 12.249 | 99.2% |
| rot_variable_amount | 4.354 | 35.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2570ns | 2570ns | base |
| rot_manual_shifts | 2521ns | 2521ns | -1.91% |
| rot_variable_amount | 2654ns | 2654ns | +3.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 21ns | base | --- | [21, 21] | --- | --- | --- | --- |
| rot_manual_shifts | 21ns | no significant difference | [+0, +0]ns | [21, 21] | no | 0.7185 | 0.7185 | **37** (23%, HIGH) |
| rot_variable_amount | 59ns | +37.9ns (+181.8%) | [+27, +38]ns | [48, 59] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 21ns | +0.0% | +183.2% |
| 1 | 2 | 21ns | +0.2% | +131.7% |
| 1 | 3 | 25ns | -17.9% | +89.8% |
| 1 | 4 | 21ns | +0.0% | +183.2% |
| 1 | 5 | 21ns | -0.2% | +131.2% |
| 1 | 6 | 21ns | +21.8% | +130.6% |
| 1 | 7 | 21ns | +0.5% | +131.7% |
| 1 | 8 | 21ns | +0.0% | +182.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.173 | ok |
| rot_manual_shifts | -0.152 | ok |
| rot_variable_amount | -0.275 | moderate- |

**Consistency summary:**

- **rot_manual_shifts**: won 2/8, lost 3/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 32.0ns | 34.0ns | 94.2% | HIGH |
| rot_manual_shifts | 31.1ns | 33.4ns | 93.1% | HIGH |
| rot_variable_amount | 32.5ns | 79.9ns | 40.6% | HIGH |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 20.7-75.8 ns)
     20.7 |########################################
     23.5 |########
     26.2 |
     29.0 |
     31.8 |
     34.5 |
     37.3 |
     40.0 |
     42.8 |
     45.5 |
     48.3 |
     51.0 |
     53.8 |
     56.6 |
     59.3 |##
     62.1 |
     64.8 |###
     67.6 |##
     70.3 |#
     73.1 |
  (20 below, 14 above range)

rot_manual_shifts (n=160, range 20.8-76.3 ns)
     20.8 |########################################
     23.5 |######
     26.3 |
     29.1 |
     31.9 |
     34.6 |
     37.4 |
     40.2 |
     43.0 |
     45.8 |
     48.5 |
     51.3 |
     54.1 |
     56.9 |
     59.6 |
     62.4 |
     65.2 |#
     68.0 |#
     70.7 |
     73.5 |
  (14 below, 15 above range)

rot_variable_amount (n=160, range 48.1-182.5 ns)
     48.1 |#######################################
     54.8 |########################################
     61.6 |
     68.3 |
     75.0 |#
     81.7 |
     88.4 |#
     95.2 |
    101.9 |
    108.6 |
    115.3 |
    122.0 |
    128.8 |
    135.5 |##
    142.2 |
    148.9 |#
    155.6 |
    162.3 |##
    169.1 |#
    175.8 |##
  (25 below, 18 above range)

```

## Diagnostics

- **rot_intrinsic_const**: CV=66.6% (high variance, measurements may be unstable)
- **rot_intrinsic_const**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **rot_intrinsic_const**: bridge=93.0% of algo (FFI overhead may distort results)
- **rot_manual_shifts**: CV=68.0% (high variance, measurements may be unstable)
- **rot_manual_shifts**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **rot_manual_shifts**: bridge=92.8% of algo (FFI overhead may distort results)
- **rot_variable_amount**: CV=66.5% (high variance, measurements may be unstable)
- **rot_variable_amount**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **rot_variable_amount**: bridge=40.5% of algo (FFI overhead may distort results)
