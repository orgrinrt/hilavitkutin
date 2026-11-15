# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_trait shows alternating (throttle bounce) (autocorr -0.51)

dispatch_trait's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5 ns) is smaller than the fastest variant's own run-to-run std-dev (8 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (120 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader dispatch_const_ptr vs stability leader dispatch_direct (+4% speed for 1.4x steadier)

dispatch_const_ptr is fastest (120 ns, CV 6.6%); dispatch_direct gives up 4.2% median for 1.4x lower variance (CV 4.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 4.2% of the fastest

All 3 variants sit between 120 ns and 125 ns - a 4.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (dispatch_const_ptr) is the fastest** at 120.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.04x (fastest 120.4 ns, slowest 125.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Static dispatch shapes: direct vs sealed-trait vs const fn pointer |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x7eedfacedeadbeef |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2398ns | 2375ns | 2196ns | 2415ns | 2551ns | base |
| dispatch_direct | 2492ns | 2534ns | 2331ns | 2525ns | 2552ns | +3.90% |
| dispatch_trait | 2451ns | 2531ns | 2228ns | 2487ns | 2566ns | +2.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 120ns | 109ns | 131ns | base | 8.533 |
| dispatch_direct | 124ns | 116ns | 132ns | +3.73% | 8.226 |
| dispatch_trait | 123ns | 111ns | 135ns | +2.40% | 8.333 |

## Performance model

- Peak throughput: **9.374 Gops/s** (dispatch_const_ptr; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 8.505 | 90.7% |
| dispatch_direct | 8.166 | 87.1% |
| dispatch_trait | 8.305 | 88.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2398ns | 2398ns | base |
| dispatch_direct | 2492ns | 2492ns | +3.90% |
| dispatch_trait | 2451ns | 2451ns | +2.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 120ns | base | --- | [117, 123] | --- | --- | --- | --- |
| dispatch_direct | 125ns | +3.0ns (+2.5%) | [+1, +6]ns | [125, 126] | YES | 0.0006 | 0.0003 | 4 |
| dispatch_trait | 123ns | no significant difference | [-0, +5]ns | [122, 125] | no | 0.2979 | 0.2979 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 112ns | +11.5% | +7.4% |
| 1 | 2 | 126ns | +0.0% | -3.1% |
| 1 | 3 | 112ns | +12.2% | +13.7% |
| 1 | 4 | 126ns | +0.7% | -12.4% |
| 1 | 5 | 123ns | +1.3% | +2.7% |
| 1 | 6 | 126ns | -3.3% | -0.2% |
| 1 | 7 | 121ns | +4.8% | -0.5% |
| 1 | 8 | 112ns | +10.8% | +10.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.366 | moderate- |
| dispatch_direct | -0.326 | moderate- |
| dispatch_trait | -0.511 | HIGH- (thermal bounce) |

**Consistency summary:**

- **dispatch_direct**: won 1/8, lost 6/8
- **dispatch_trait**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 3.1ns | 120.0ns | 2.6% |  |
| dispatch_direct | 3.1ns | 124.5ns | 2.5% |  |
| dispatch_trait | 3.1ns | 122.9ns | 2.5% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 109.2-130.6 ns)
    109.2 |##
    110.3 |###############################
    111.4 |####################
    112.4 |########################################
    113.5 |########
    114.6 |########
    115.7 |##################################
    116.7 |######################
    117.8 |########
    118.9 |########
    119.9 |###########
    121.0 |########
    122.1 |#########################
    123.1 |######################
    124.2 |#################
    125.3 |#####################################
    126.3 |#########################
    127.4 |####################
    128.5 |########
    129.6 |##############
  (13 below, 15 above range)

dispatch_direct (n=160, range 115.7-131.6 ns)
    115.7 |##
    116.4 |#######
    117.2 |#####
    118.0 |############
    118.8 |###############
    119.6 |##########
    120.4 |############
    121.2 |############
    122.0 |#################
    122.8 |#################
    123.6 |###############
    124.4 |####################
    125.2 |########################################
    126.0 |######################
    126.8 |####################
    127.6 |#####################################
    128.4 |#####
    129.2 |#################
    130.0 |######################
    130.8 |#######
  (16 below, 16 above range)

dispatch_trait (n=160, range 110.7-134.8 ns)
    110.7 |##################
    111.9 |#####
    113.1 |################
    114.3 |##########
    115.5 |##########
    116.7 |##########
    117.9 |##########
    119.1 |#####################
    120.3 |################################
    121.5 |#####################
    122.7 |########################################
    124.0 |########################
    125.2 |################################
    126.4 |##################################
    127.6 |########################
    128.8 |########################
    130.0 |#############
    131.2 |################
    132.4 |########
    133.6 |########
  (14 below, 3 above range)

```
