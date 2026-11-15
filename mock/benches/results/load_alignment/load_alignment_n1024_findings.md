# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### load_aligned is fastest but the noisiest (CV 7.2%)

load_aligned wins on median (113 ns) yet has the highest variance (CV 7.2%), while load_byte_pack is the steadiest (CV 6.0%, 115 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### load_byte_pack shows alternating (throttle bounce) (autocorr -0.64)

load_byte_pack's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (8 ns) is smaller than the fastest variant's own run-to-run std-dev (8 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (113 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader load_aligned vs stability leader load_byte_pack (+2% speed for 1.2x steadier)

load_aligned is fastest (113 ns, CV 7.2%); load_byte_pack gives up 1.9% median for 1.2x lower variance (CV 6.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### load_unaligned's edge over baseline is significant but tiny (1 ns, 1.06%)

load_unaligned differs from baseline load_aligned by 1 ns (1.06%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (load_aligned) is the fastest** at 112.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.07x (fastest 112.9 ns, slowest 120.8 ns)

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
| load_aligned | 2360ns | 2307ns | 2193ns | 2354ns | 2546ns | base |
| load_byte_pack | 2371ns | 2354ns | 2223ns | 2365ns | 2537ns | +0.45% |
| load_unaligned | 2422ns | 2526ns | 2198ns | 2457ns | 2540ns | +2.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 115ns | 105ns | 127ns | base | 8.890 |
| load_byte_pack | 116ns | 107ns | 127ns | +0.94% | 8.807 |
| load_unaligned | 118ns | 106ns | 127ns | +2.76% | 8.651 |

## Performance model

- Peak throughput: **9.738 Gops/s** (load_aligned; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 9.070 | 93.1% |
| load_byte_pack | 8.904 | 91.4% |
| load_unaligned | 8.477 | 87.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2360ns | 2360ns | base |
| load_byte_pack | 2371ns | 2371ns | +0.45% |
| load_unaligned | 2422ns | 2422ns | +2.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 113ns | base | --- | [111, 115] | --- | --- | --- | --- |
| load_byte_pack | 115ns | no significant difference | [-0, +4]ns | [114, 117] | no | 0.1124 | 0.1124 | 1 |
| load_unaligned | 121ns | +1.2ns (+1.1%) | [+0, +2]ns | [118, 122] | YES | 0.0116 | 0.0058 | 7 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 108ns | +7.7% | +1.6% |
| 1 | 2 | 122ns | -10.1% | +1.4% |
| 1 | 3 | 107ns | +14.2% | +14.8% |
| 1 | 4 | 110ns | +3.8% | -1.7% |
| 1 | 5 | 111ns | +6.6% | +5.9% |
| 1 | 6 | 124ns | -8.8% | -1.5% |
| 1 | 7 | 125ns | -3.9% | -1.5% |
| 1 | 8 | 122ns | -10.4% | -0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | 0.134 | ok |
| load_byte_pack | -0.639 | HIGH- (thermal bounce) |
| load_unaligned | -0.140 | ok |

**Consistency summary:**

- **load_byte_pack**: won 4/8, lost 4/8
- **load_unaligned**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 3.2ns | 115.2ns | 2.8% |  |
| load_byte_pack | 3.0ns | 116.3ns | 2.6% |  |
| load_unaligned | 3.0ns | 118.4ns | 2.6% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 105.2-127.1 ns)
    105.2 |########################
    106.3 |########################
    107.3 |#################################
    108.4 |###############
    109.5 |########################################
    110.6 |########################################
    111.7 |#####################
    112.8 |#####################
    113.9 |########################
    115.0 |######
    116.1 |######
    117.2 |#####################
    118.3 |###
    119.4 |############
    120.5 |############
    121.6 |#####################
    122.7 |#########
    123.8 |#####################
    124.9 |########################
    126.0 |############
  (13 below, 18 above range)

load_byte_pack (n=160, range 107.3-126.8 ns)
    107.3 |########
    108.2 |############################
    109.2 |#############
    110.2 |######
    111.2 |#############
    112.1 |######################
    113.1 |########################
    114.1 |########################################
    115.1 |######
    116.0 |########
    117.0 |######################
    118.0 |######
    119.0 |###########
    119.9 |###############
    120.9 |########
    121.9 |###########
    122.9 |########
    123.9 |###############
    124.8 |###########
    125.8 |####
  (14 below, 16 above range)

load_unaligned (n=160, range 106.4-127.4 ns)
    106.4 |########
    107.5 |##########################
    108.5 |#############
    109.6 |#####
    110.6 |
    111.7 |#############
    112.7 |########################
    113.8 |##########
    114.8 |################
    115.9 |########
    116.9 |########################
    118.0 |#####
    119.0 |#############
    120.1 |#############
    121.1 |########################################
    122.2 |#####################
    123.2 |#####################################
    124.3 |################################
    125.3 |##########################
    126.4 |##########
  (14 below, 15 above range)

```
