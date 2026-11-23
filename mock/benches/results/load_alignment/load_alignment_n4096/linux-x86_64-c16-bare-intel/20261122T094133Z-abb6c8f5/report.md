# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### load_aligned is fastest but the noisiest (CV 84.4%)

load_aligned wins on median (473 ns) yet has the highest variance (CV 84.4%), while load_unaligned is the steadiest (CV 68.8%, 577 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (105 ns) is smaller than the fastest variant's own run-to-run std-dev (399 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (473 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### load_unaligned's comparison is tie-heavy (11% tied pairs)

11% of paired samples for load_unaligned are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### load_aligned is inconsistent: worst-20% is 2.9x its best-20%

load_aligned's best 20% of batches run at 472 ns but its worst 20% at 1.37 us (2.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (load_aligned) is the fastest** at 472.6 ns median
- Spread: 1.22x (fastest 472.6 ns, slowest 577.4 ns)

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
| load_aligned | 2867ns | 2016ns | 2005ns | 2160ns | 5849ns | base |
| load_byte_pack | 2855ns | 2125ns | 2005ns | 2230ns | 5579ns | -0.42% |
| load_unaligned | 2982ns | 2454ns | 2006ns | 2378ns | 5770ns | +4.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 673ns | 472ns | 1366ns | base | 6.089 |
| load_byte_pack | 670ns | 472ns | 1305ns | -0.43% | 6.115 |
| load_unaligned | 699ns | 472ns | 1344ns | +3.88% | 5.862 |

## Performance model

- Peak throughput: **8.671 Gops/s** (load_byte_pack; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 8.667 | 100.0% |
| load_byte_pack | 8.232 | 94.9% |
| load_unaligned | 7.094 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2867ns | 2867ns | base |
| load_byte_pack | 2855ns | 2855ns | -0.42% |
| load_unaligned | 2982ns | 2982ns | +4.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 473ns | base | --- | [472, 482] | --- | --- | --- | --- |
| load_byte_pack | 498ns | no significant difference | [-0, +0]ns | [473, 577] | no | 1.0000 | 1.0000 | 10 |
| load_unaligned | 577ns | no significant difference | [+0, +100]ns | [577, 577] | no (adj: YES) | 0.0028 | 0.0014 | **17** (11%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 472ns | +0.0% | +0.0% |
| 1 | 2 | 472ns | +0.0% | +22.2% |
| 1 | 3 | 473ns | +22.2% | +22.2% |
| 1 | 4 | 472ns | +22.2% | +0.0% |
| 1 | 5 | 578ns | -18.2% | -0.0% |
| 1 | 6 | 577ns | -18.2% | -18.2% |
| 1 | 7 | 472ns | +0.0% | +22.2% |
| 1 | 8 | 472ns | +22.2% | +22.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | 0.291 | moderate+ |
| load_byte_pack | 0.058 | ok |
| load_unaligned | -0.475 | moderate- |

**Consistency summary:**

- **load_byte_pack**: won 2/8, lost 3/8
- **load_unaligned**: won 1/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 27.5ns | 672.7ns | 4.1% |  |
| load_byte_pack | 27.3ns | 669.8ns | 4.1% |  |
| load_unaligned | 28.5ns | 698.8ns | 4.1% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 472.4-1366.3 ns)
    472.4 |########################################
    517.1 |
    561.8 |##############
    606.5 |
    651.2 |
    695.9 |
    740.6 |
    785.3 |
    829.9 |
    874.6 |
    919.3 |
    964.0 |
   1008.7 |
   1053.4 |
   1098.1 |
   1142.8 |
   1187.5 |
   1232.2 |
   1276.9 |
   1321.6 |
  (0 below, 20 above range)

load_byte_pack (n=160, range 472.4-1304.8 ns)
    472.4 |########################################
    514.0 |
    555.6 |##########################
    597.2 |
    638.9 |
    680.5 |
    722.1 |
    763.7 |
    805.3 |
    847.0 |
    888.6 |
    930.2 |
    971.8 |
   1013.4 |
   1055.0 |
   1096.7 |
   1138.3 |
   1179.9 |
   1221.5 |
   1263.1 |#
  (4 below, 18 above range)

load_unaligned (n=160, range 472.4-1344.2 ns)
    472.4 |#########
    516.0 |
    559.6 |########################################
    603.2 |
    646.8 |
    690.4 |
    733.9 |
    777.5 |
    821.1 |
    864.7 |
    908.3 |
    951.9 |
    995.5 |
   1039.1 |
   1082.7 |
   1126.3 |
   1169.8 |
   1213.4 |
   1257.0 |
   1300.6 |
  (28 below, 19 above range)

```

## Diagnostics

- **load_aligned**: CV=59.3% (high variance, measurements may be unstable)
- **load_byte_pack**: CV=57.5% (high variance, measurements may be unstable)
- **load_unaligned**: CV=56.8% (high variance, measurements may be unstable)
