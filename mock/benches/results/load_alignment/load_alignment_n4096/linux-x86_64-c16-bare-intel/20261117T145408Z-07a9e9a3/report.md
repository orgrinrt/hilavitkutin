# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (94 ns) is smaller than the fastest variant's own run-to-run std-dev (410 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (484 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### load_byte_pack's comparison is tie-heavy (20% tied pairs)

20% of paired samples for load_byte_pack are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader load_aligned vs stability leader load_unaligned (+7% speed for 1.1x steadier)

load_aligned is fastest (484 ns, CV 84.7%); load_unaligned gives up 6.9% median for 1.1x lower variance (CV 74.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### load_byte_pack is inconsistent: worst-20% is 4.9x its best-20%

load_byte_pack's best 20% of batches run at 472 ns but its worst 20% at 2.32 us (4.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (load_aligned) is the fastest** at 483.9 ns median
- Spread: 1.19x (fastest 483.9 ns, slowest 577.4 ns)

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
| load_aligned | 2923ns | 2035ns | 2013ns | 2235ns | 5897ns | base |
| load_byte_pack | 3939ns | 2462ns | 2013ns | 2576ns | 9957ns | +34.79% |
| load_unaligned | 2870ns | 2190ns | 2012ns | 2241ns | 5616ns | -1.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 682ns | 472ns | 1368ns | base | 6.005 |
| load_byte_pack | 920ns | 472ns | 2319ns | +34.93% | 4.450 |
| load_unaligned | 671ns | 472ns | 1305ns | -1.69% | 6.108 |

## Performance model

- Peak throughput: **8.670 Gops/s** (load_aligned; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 8.465 | 97.6% |
| load_byte_pack | 7.094 | 81.8% |
| load_unaligned | 7.920 | 91.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2923ns | 2923ns | base |
| load_byte_pack | 3939ns | 3939ns | +34.79% |
| load_unaligned | 2870ns | 2870ns | -1.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 484ns | base | --- | [473, 577] | --- | --- | --- | --- |
| load_byte_pack | 577ns | no significant difference | [+0, +0]ns | [484, 578] | no | 0.7910 | 0.7910 | **32** (20%, HIGH) |
| load_unaligned | 517ns | no significant difference | [+0, +0]ns | [473, 577] | no | 0.3491 | 0.1745 | **21** (13%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 472ns | +0.0% | +0.0% |
| 1 | 2 | 578ns | -18.2% | -18.2% |
| 1 | 3 | 472ns | +139.7% | +0.0% |
| 1 | 4 | 472ns | +0.0% | +22.2% |
| 1 | 5 | 578ns | +0.0% | -18.2% |
| 1 | 6 | 472ns | +0.0% | +22.2% |
| 1 | 7 | 472ns | +160.6% | +0.0% |
| 1 | 8 | 578ns | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | -0.475 | moderate- |
| load_byte_pack | -0.388 | moderate- |
| load_unaligned | -0.475 | moderate- |

**Consistency summary:**

- **load_byte_pack**: won 1/8, lost 2/8
- **load_unaligned**: won 2/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 27.9ns | 682.1ns | 4.1% |  |
| load_byte_pack | 37.2ns | 920.4ns | 4.0% |  |
| load_unaligned | 27.5ns | 670.6ns | 4.1% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 472.4-1367.6 ns)
    472.4 |########################################
    517.2 |
    561.9 |##################################
    606.7 |
    651.4 |##
    696.2 |
    741.0 |
    785.7 |
    830.5 |
    875.2 |
    920.0 |
    964.7 |
   1009.5 |
   1054.3 |
   1099.0 |
   1143.8 |
   1188.5 |
   1233.3 |
   1278.1 |
   1322.8 |
  (27 below, 21 above range)

load_byte_pack (n=160, range 472.4-2319.1 ns)
    472.4 |########################################
    564.8 |#######################
    657.1 |##
    749.4 |##
    841.8 |
    934.1 |##
   1026.4 |
   1118.8 |##
   1211.1 |#
   1303.4 |##
   1395.8 |###
   1488.1 |##
   1580.4 |##
   1672.7 |##
   1765.1 |#
   1857.4 |#
   1949.7 |#
   2042.1 |
   2134.4 |
   2226.7 |
  (15 below, 7 above range)

load_unaligned (n=160, range 472.5-1305.1 ns)
    472.5 |########################################
    514.1 |
    555.7 |################################
    597.3 |
    639.0 |
    680.6 |
    722.2 |
    763.9 |
    805.5 |
    847.1 |
    888.8 |#
    930.4 |
    972.0 |
   1013.6 |
   1055.3 |
   1096.9 |
   1138.5 |
   1180.2 |
   1221.8 |
   1263.4 |
  (14 below, 21 above range)

```

## Diagnostics

- **load_aligned**: CV=60.1% (high variance, measurements may be unstable)
- **load_byte_pack**: CV=103.8% (high variance, measurements may be unstable)
- **load_byte_pack**: worst_20/best_20 = 4.9x (possible bimodal distribution)
- **load_unaligned**: CV=57.6% (high variance, measurements may be unstable)
