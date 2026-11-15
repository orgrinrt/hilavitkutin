# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 578 ns to slowest 578 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (388 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (578 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### load_byte_pack's comparison is tie-heavy (22% tied pairs)

22% of paired samples for load_byte_pack are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### load_byte_pack is inconsistent: worst-20% is 2.5x its best-20%

load_byte_pack's best 20% of batches run at 525 ns but its worst 20% at 1.33 us (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (load_aligned) is the fastest** at 577.5 ns median
- Spread: 1.00x (fastest 577.5 ns, slowest 577.5 ns)

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
| load_aligned | 3020ns | 2454ns | 2227ns | 2456ns | 5506ns | base |
| load_byte_pack | 3063ns | 2455ns | 2228ns | 2457ns | 5718ns | +1.43% |
| load_unaligned | 3144ns | 2456ns | 2448ns | 2458ns | 5900ns | +4.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 709ns | 525ns | 1286ns | base | 5.779 |
| load_byte_pack | 717ns | 525ns | 1329ns | +1.21% | 5.710 |
| load_unaligned | 737ns | 577ns | 1373ns | +3.93% | 5.561 |

## Performance model

- Peak throughput: **7.799 Gops/s** (load_aligned; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 7.093 | 90.9% |
| load_byte_pack | 7.093 | 90.9% |
| load_unaligned | 7.093 | 90.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 3020ns | 3020ns | base |
| load_byte_pack | 3063ns | 3063ns | +1.43% |
| load_unaligned | 3144ns | 3144ns | +4.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 578ns | base | --- | [577, 578] | --- | --- | --- | --- |
| load_byte_pack | 578ns | no significant difference | [+0, +0]ns | [577, 578] | no | 0.7207 | 0.7207 | **35** (22%, HIGH) |
| load_unaligned | 578ns | no significant difference | [+0, +0]ns | [578, 578] | no | 0.3108 | 0.1554 | **33** (21%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 578ns | -18.2% | +0.0% |
| 1 | 2 | 578ns | +0.0% | +0.0% |
| 1 | 3 | 472ns | +22.2% | +22.2% |
| 1 | 4 | 578ns | +0.0% | +0.0% |
| 1 | 5 | 577ns | +0.0% | +0.0% |
| 1 | 6 | 578ns | +0.0% | +0.0% |
| 1 | 7 | 578ns | +0.0% | +0.0% |
| 1 | 8 | 578ns | +0.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | -0.161 | ok |
| load_byte_pack | -0.018 | ok |
| load_unaligned | -0.018 | ok |

**Consistency summary:**

- **load_byte_pack**: won 1/8, lost 1/8
- **load_unaligned**: won 0/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 28.9ns | 708.7ns | 4.1% |  |
| load_byte_pack | 29.2ns | 717.3ns | 4.1% |  |
| load_unaligned | 30.0ns | 736.6ns | 4.1% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 525.2-1286.1 ns)
    525.2 |
    563.3 |########################################
    601.3 |
    639.4 |
    677.4 |
    715.4 |
    753.5 |
    791.5 |
    829.6 |
    867.6 |
    905.7 |
    943.7 |
    981.8 |
   1019.8 |
   1057.8 |
   1095.9 |
   1133.9 |
   1172.0 |
   1210.0 |
   1248.1 |
  (16 below, 19 above range)

load_byte_pack (n=160, range 525.3-1328.9 ns)
    525.3 |
    565.5 |########################################
    605.6 |
    645.8 |
    686.0 |
    726.2 |
    766.4 |
    806.5 |
    846.7 |
    886.9 |
    927.1 |
    967.3 |
   1007.5 |
   1047.6 |
   1087.8 |
   1128.0 |
   1168.2 |
   1208.4 |
   1248.6 |
   1288.7 |
  (16 below, 17 above range)

load_unaligned (n=160, range 577.4-1372.8 ns)
    577.4 |########################################
    617.1 |
    656.9 |
    696.7 |
    736.4 |
    776.2 |
    816.0 |
    855.8 |
    895.5 |
    935.3 |
    975.1 |
   1014.8 |
   1054.6 |
   1094.4 |
   1134.2 |
   1173.9 |
   1213.7 |
   1253.5 |
   1293.2 |
   1333.0 |
  (11 below, 19 above range)

```

## Diagnostics

- **load_aligned**: CV=54.7% (high variance, measurements may be unstable)
- **load_byte_pack**: CV=56.0% (high variance, measurements may be unstable)
- **load_unaligned**: CV=52.8% (high variance, measurements may be unstable)
