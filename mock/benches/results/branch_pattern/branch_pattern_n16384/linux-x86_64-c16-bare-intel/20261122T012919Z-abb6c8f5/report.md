# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### branch_cmov dominates: 11% faster than the next best (branch_if)

branch_cmov (27.72 us) leads branch_if (30.73 us) by 11%, a clear separation rather than a photo finish. CV 8.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (branch_cmov)

The baseline branch_cmov is the fastest (27.72 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (branch_cmov) is the fastest** at 27721.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.11x (fastest 27721.4 ns, slowest 30730.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Branchful if/else vs branchless cmov/csel under data-dependent input |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x76543210fedcba98 |
| Counter frequency | 2112827530 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| branch_cmov | 28316ns | 29609ns | 24200ns | 29159ns | 29903ns | base |
| branch_if | 32626ns | 32626ns | 28105ns | 32468ns | 37622ns | +15.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 26508ns | 22654ns | 28002ns | base | 0.618 |
| branch_if | 30707ns | 26407ns | 35433ns | +15.84% | 0.534 |

## Performance model

- Peak throughput: **0.723 Gops/s** (branch_cmov; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.591 | 81.7% |
| branch_if | 0.533 | 73.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 28316ns | 28316ns | base |
| branch_if | 32626ns | 32626ns | +15.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 27721ns | base | --- | [27690, 27733] | --- | --- | --- | --- |
| branch_if | 30731ns | +3288.3ns (+11.9%) | [+3192, +3394]ns | [30614, 30845] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 27758ns | -12.6% |
| 1 | 2 | 27734ns | +11.8% |
| 1 | 3 | 27718ns | +12.7% |
| 1 | 4 | 27755ns | +11.4% |
| 1 | 5 | 27763ns | +10.7% |
| 1 | 6 | 27736ns | +5.1% |
| 1 | 7 | 22701ns | +31.9% |
| 1 | 8 | 22689ns | +34.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | 0.459 | moderate+ |
| branch_if | -0.070 | ok |

**Consistency summary:**

- **branch_if**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 22.7ns | 26508.0ns | 0.1% |  |
| branch_if | 25.0ns | 30707.5ns | 0.1% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 22654.4-28002.0 ns)
  22654.4 |###################
  22921.8 |
  23189.2 |
  23456.6 |
  23723.9 |
  23991.3 |
  24258.7 |
  24526.1 |
  24793.4 |
  25060.8 |
  25328.2 |
  25595.6 |
  25862.9 |
  26130.3 |
  26397.7 |
  26665.1 |
  26932.4 |
  27199.8 |
  27467.2 |################################
  27734.6 |########################################
  (8 below, 2 above range)

branch_if (n=160, range 26407.1-35433.1 ns)
  26407.1 |
  26858.4 |
  27309.7 |
  27761.0 |
  28212.3 |
  28663.6 |#######
  29114.9 |######
  29566.2 |##########
  30017.5 |##########
  30468.8 |########################################
  30920.1 |###################################
  31371.4 |#####
  31822.7 |
  32274.0 |
  32725.3 |
  33176.6 |
  33627.9 |
  34079.2 |
  34530.5 |
  34981.8 |
  (19 below, 1 above range)

```

## Diagnostics

- **branch_if**: CV=35.0% (high variance, measurements may be unstable)
