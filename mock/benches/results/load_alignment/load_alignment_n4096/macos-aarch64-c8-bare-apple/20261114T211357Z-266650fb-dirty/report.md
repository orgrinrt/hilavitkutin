# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### load_aligned is fastest but the noisiest (CV 11.7%)

load_aligned wins on median (616 ns) yet has the highest variance (CV 11.7%), while load_unaligned is the steadiest (CV 3.9%, 630 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (15 ns) is smaller than the fastest variant's own run-to-run std-dev (72 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (616 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader load_aligned vs stability leader load_unaligned (+2% speed for 3.0x steadier)

load_aligned is fastest (616 ns, CV 11.7%); load_unaligned gives up 2.3% median for 3.0x lower variance (CV 3.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 2.4% of the fastest

All 3 variants sit between 616 ns and 631 ns - a 2.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### load_unaligned's edge over baseline is significant but tiny (5 ns, 0.88%)

load_unaligned differs from baseline load_aligned by 5 ns (0.88%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (load_aligned) is the fastest** at 616.0 ns median
- 2 variants significantly slower than baseline
- Spread: 1.02x (fastest 616.0 ns, slowest 630.8 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| load_aligned | 2809ns | 2798ns | 2671ns | 2787ns | 3011ns | base |
| load_byte_pack | 2845ns | 2869ns | 2682ns | 2842ns | 3015ns | +1.29% |
| load_unaligned | 2842ns | 2870ns | 2686ns | 2849ns | 2980ns | +1.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 619ns | 588ns | 669ns | base | 6.612 |
| load_byte_pack | 626ns | 590ns | 662ns | +0.98% | 6.548 |
| load_unaligned | 626ns | 591ns | 658ns | +1.00% | 6.547 |

## Performance model

- Peak throughput: **6.963 Gops/s** (load_aligned; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 6.649 | 95.5% |
| load_byte_pack | 6.493 | 93.3% |
| load_unaligned | 6.497 | 93.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2809ns | 2809ns | base |
| load_byte_pack | 2845ns | 2845ns | +1.29% |
| load_unaligned | 2842ns | 2842ns | +1.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 616ns | base | --- | [610, 623] | --- | --- | --- | --- |
| load_byte_pack | 631ns | +13.3ns (+2.2%) | [+6, +21]ns | [629, 632] | YES | 0.0006 | 0.0006 | 2 |
| load_unaligned | 630ns | +5.4ns (+0.9%) | [+2, +17]ns | [630, 632] | YES | 0.0006 | 0.0004 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 622ns | +1.6% | -5.1% |
| 1 | 2 | 633ns | -6.8% | -3.4% |
| 1 | 3 | 590ns | +7.5% | +6.9% |
| 1 | 4 | 597ns | +2.4% | +14.0% |
| 1 | 5 | 591ns | +7.3% | +6.6% |
| 1 | 6 | 613ns | +3.7% | +3.5% |
| 1 | 7 | 632ns | +0.4% | +0.1% |
| 1 | 8 | 629ns | -5.8% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | 0.322 | moderate+ |
| load_byte_pack | -0.369 | moderate- |
| load_unaligned | 0.170 | ok |

**Consistency summary:**

- **load_byte_pack**: won 2/8, lost 6/8
- **load_unaligned**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 2.8ns | 619.4ns | 0.5% |  |
| load_byte_pack | 2.8ns | 625.5ns | 0.4% |  |
| load_unaligned | 3.1ns | 625.6ns | 0.5% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 588.3-668.5 ns)
    588.3 |########################################
    592.3 |#############
    596.3 |###
    600.3 |###
    604.3 |#
    608.3 |#########
    612.4 |##########
    616.4 |####
    620.4 |##########
    624.4 |######
    628.4 |##############################
    632.4 |#####################
    636.4 |#####
    640.4 |
    644.5 |#
    648.5 |#
    652.5 |
    656.5 |#
    660.5 |
    664.5 |
  (11 below, 5 above range)

load_byte_pack (n=160, range 590.4-662.1 ns)
    590.4 |####################
    594.0 |##########
    597.6 |#
    601.2 |########
    604.7 |#
    608.3 |########
    611.9 |####
    615.5 |######
    619.1 |##
    622.7 |#
    626.3 |############
    629.9 |########################################
    633.4 |######################################
    637.0 |#############
    640.6 |
    644.2 |#
    647.8 |#
    651.4 |#
    655.0 |
    658.6 |#
  (14 below, 16 above range)

load_unaligned (n=160, range 591.4-658.0 ns)
    591.4 |##########
    594.7 |######
    598.0 |
    601.4 |####
    604.7 |####
    608.0 |#########
    611.4 |####
    614.7 |##
    618.0 |#
    621.3 |#
    624.7 |###
    628.0 |####################################
    631.3 |########################################
    634.7 |##################
    638.0 |######
    641.3 |###
    644.6 |#
    648.0 |
    651.3 |
    654.6 |
  (18 below, 14 above range)

```
