# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 20.41 us to slowest 20.41 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (95 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (memcpy_intrinsic)

The baseline memcpy_intrinsic is the fastest (20.41 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (memcpy_intrinsic) is the fastest** at 20406.0 ns median
- Spread: 1.00x (fastest 20406.0 ns, slowest 20407.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4646464650504040 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 22545ns | 22511ns | 22502ns | 22523ns | 22653ns | base |
| memcpy_naive | 22560ns | 22524ns | 22502ns | 22531ns | 22707ns | +0.07% |
| memcpy_unrolled | 22541ns | 22513ns | 22501ns | 22526ns | 22625ns | -0.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 20434ns | 20400ns | 20528ns | base | 0.802 |
| memcpy_naive | 20450ns | 20401ns | 20583ns | +0.07% | 0.801 |
| memcpy_unrolled | 20433ns | 20400ns | 20506ns | -0.00% | 0.802 |

## Performance model

- Peak throughput: **0.803 Gops/s** (memcpy_intrinsic; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.803 | 100.0% |
| memcpy_naive | 0.803 | 100.0% |
| memcpy_unrolled | 0.803 | 100.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 22545ns | 22545ns | base |
| memcpy_naive | 22560ns | 22560ns | +0.07% |
| memcpy_unrolled | 22541ns | 22541ns | -0.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 20406ns | base | --- | [20405, 20407] | --- | --- | --- | --- |
| memcpy_naive | 20407ns | no significant difference | [-1, +1]ns | [20406, 20410] | no | 1.0000 | 1.0000 | 8 |
| memcpy_unrolled | 20407ns | no significant difference | [+0, +2]ns | [20406, 20410] | no | 0.1721 | 0.0861 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 20406ns | +0.0% | -0.0% |
| 1 | 2 | 20405ns | +0.0% | +0.0% |
| 1 | 3 | 20406ns | +0.0% | +0.0% |
| 1 | 4 | 20408ns | -0.0% | -0.0% |
| 1 | 5 | 20408ns | -0.0% | -0.0% |
| 1 | 6 | 20405ns | +0.0% | +0.0% |
| 1 | 7 | 20406ns | +0.0% | +0.0% |
| 1 | 8 | 20406ns | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | 0.232 | moderate+ |
| memcpy_naive | 0.086 | ok |
| memcpy_unrolled | 0.082 | ok |

**Consistency summary:**

- **memcpy_naive**: won 0/8, lost 0/8
- **memcpy_unrolled**: won 0/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 3.9ns | 20434.4ns | 0.0% |  |
| memcpy_naive | 3.9ns | 20449.5ns | 0.0% |  |
| memcpy_unrolled | 3.7ns | 20433.4ns | 0.0% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 20400.1-20527.6 ns)
  20400.1 |########################################
  20406.5 |#########
  20412.9 |#
  20419.2 |
  20425.6 |
  20432.0 |
  20438.4 |
  20444.7 |######
  20451.1 |#
  20457.5 |#####
  20463.8 |##
  20470.2 |#
  20476.6 |#
  20483.0 |###
  20489.3 |###
  20495.7 |
  20502.1 |
  20508.4 |
  20514.8 |
  20521.2 |
  (16 below, 4 above range)

memcpy_naive (n=160, range 20400.6-20582.7 ns)
  20400.6 |########################################
  20409.7 |#
  20418.8 |#
  20427.9 |#
  20437.0 |
  20446.1 |##########
  20455.3 |####
  20464.4 |##
  20473.5 |#
  20482.6 |###
  20491.7 |#
  20500.8 |#
  20509.9 |
  20519.0 |
  20528.1 |
  20537.2 |#
  20546.3 |
  20555.4 |
  20564.5 |
  20573.6 |
  (14 below, 7 above range)

memcpy_unrolled (n=160, range 20400.4-20505.7 ns)
  20400.4 |########################################
  20405.7 |############################
  20411.0 |
  20416.2 |
  20421.5 |
  20426.8 |
  20432.0 |
  20437.3 |#
  20442.6 |#
  20447.8 |#########
  20453.1 |######
  20458.4 |###
  20463.6 |#####
  20468.9 |###
  20474.2 |
  20479.4 |##
  20484.7 |##
  20489.9 |####
  20495.2 |
  20500.5 |#
  (16 below, 8 above range)

```
