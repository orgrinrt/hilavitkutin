# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (fpt_match_16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline fpt_match_16 has the worst median (180 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest fpt_table_16 at 176 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 2.6% (fastest 176 ns to slowest 180 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4 ns) is smaller than the fastest variant's own run-to-run std-dev (151 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### fpt_table_16 is inconsistent: worst-20% is 3.6x its best-20%

fpt_table_16's best 20% of batches run at 148 ns but its worst 20% at 527 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: fpt_table_16** at 175.9 ns median (-2.5% vs baseline)
- Spread: 1.03x (fastest 175.9 ns, slowest 180.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fn-pointer table vs match dispatch for 16-variant hot loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x478ab261e0d9f352 |
| Counter frequency | 2112762479 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fpt_match_16 | 2648ns | 2061ns | 1688ns | 1890ns | 5883ns | base |
| fpt_table_16 | 2782ns | 2064ns | 1692ns | 2043ns | 6090ns | +5.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 233ns | 146ns | 518ns | base | 4.394 |
| fpt_table_16 | 240ns | 148ns | 527ns | +3.13% | 4.260 |

## Performance model

- Peak throughput: **6.993 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 5.675 | 81.1% |
| fpt_table_16 | 5.820 | 83.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2648ns | 2648ns | base |
| fpt_table_16 | 2782ns | 2782ns | +5.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 180ns | base | --- | [150, 181] | --- | --- | --- | --- |
| fpt_table_16 | 176ns | no significant difference | [-5, +9]ns | [175, 177] | no | 1.0000 | 1.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 145ns | +19.6% |
| 1 | 2 | 148ns | +19.5% |
| 1 | 3 | 185ns | -16.4% |
| 1 | 4 | 181ns | -3.1% |
| 1 | 5 | 148ns | +5.1% |
| 1 | 6 | 150ns | -3.1% |
| 1 | 7 | 186ns | -4.3% |
| 1 | 8 | 148ns | +18.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.140 | ok |
| fpt_table_16 | -0.126 | ok |

**Consistency summary:**

- **fpt_table_16**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 31.3ns | 233.1ns | 13.4% | HIGH |
| fpt_table_16 | 33.0ns | 240.4ns | 13.7% | HIGH |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 146.4-517.6 ns)
    146.4 |########################################
    165.0 |################
    183.6 |###############
    202.1 |
    220.7 |
    239.2 |
    257.8 |
    276.4 |
    294.9 |
    313.5 |
    332.0 |
    350.6 |
    369.2 |
    387.7 |
    406.3 |
    424.8 |#
    443.4 |#
    462.0 |#
    480.5 |##
    499.1 |#
  (14 below, 15 above range)

fpt_table_16 (n=160, range 147.8-526.9 ns)
    147.8 |###########
    166.7 |########################################
    185.7 |##
    204.7 |
    223.6 |
    242.6 |
    261.5 |#
    280.5 |
    299.4 |
    318.4 |
    337.4 |
    356.3 |
    375.3 |
    394.2 |
    413.2 |
    432.2 |
    451.1 |
    470.1 |#
    489.0 |#
    508.0 |
  (22 below, 19 above range)

```

## Diagnostics

- **fpt_match_16**: CV=66.2% (high variance, measurements may be unstable)
- **fpt_match_16**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **fpt_match_16**: bridge=13.4% of algo (FFI overhead may distort results)
- **fpt_table_16**: CV=62.6% (high variance, measurements may be unstable)
- **fpt_table_16**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **fpt_table_16**: bridge=13.9% of algo (FFI overhead may distort results)
