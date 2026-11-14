# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (minmax_branch, minmax_intrinsic) are a dead heat (<1%)

minmax_branch (117 ns) and minmax_intrinsic (117 ns) differ by 0.00%, inside the noise, even though the wider field spreads 5.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### minmax_branch shows warm-up / thermal drift (autocorr +0.60)

minmax_branch's per-pass series has lag-1 autocorrelation +0.60, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6 ns) is smaller than the fastest variant's own run-to-run std-dev (8 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (minmax_branch)

The baseline minmax_branch is the fastest (117 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader minmax_branch vs stability leader minmax_branchless (+5% speed for 1.1x steadier)

minmax_branch is fastest (117 ns, CV 6.7%); minmax_branchless gives up 5.3% median for 1.1x lower variance (CV 6.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (minmax_branch) is the fastest** at 116.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.05x (fastest 116.7 ns, slowest 122.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5d49a3f1b7c06e82 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| minmax_branch | 2333ns | 2331ns | 2167ns | 2330ns | 2511ns | base |
| minmax_branchless | 2416ns | 2499ns | 2218ns | 2447ns | 2519ns | +3.54% |
| minmax_intrinsic | 2315ns | 2322ns | 2166ns | 2301ns | 2506ns | -0.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 118ns | 108ns | 130ns | base | 34.764 |
| minmax_branchless | 122ns | 111ns | 133ns | +3.71% | 33.520 |
| minmax_intrinsic | 117ns | 107ns | 129ns | -0.49% | 34.933 |

## Performance model

- Peak throughput: **38.293 Gops/s** (minmax_intrinsic; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 35.099 | 91.7% |
| minmax_branchless | 33.328 | 87.0% |
| minmax_intrinsic | 35.099 | 91.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2333ns | 2333ns | base |
| minmax_branchless | 2416ns | 2416ns | +3.54% |
| minmax_intrinsic | 2315ns | 2315ns | -0.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 117ns | base | --- | [116, 118] | --- | --- | --- | --- |
| minmax_branchless | 123ns | +2.7ns (+2.3%) | [+2, +4]ns | [121, 124] | YES | 0.0000 | 0.0000 | 5 |
| minmax_intrinsic | 117ns | no significant difference | [-4, +2]ns | [115, 118] | no | 0.9370 | 0.9370 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 118ns | +6.2% | -7.8% |
| 1 | 2 | 125ns | -3.2% | -1.2% |
| 1 | 3 | 124ns | -0.8% | -11.3% |
| 1 | 4 | 120ns | +4.4% | -3.5% |
| 1 | 5 | 116ns | +4.7% | +6.8% |
| 1 | 6 | 116ns | +7.0% | -4.1% |
| 1 | 7 | 110ns | +13.0% | +7.1% |
| 1 | 8 | 110ns | -0.2% | +10.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | 0.605 | HIGH+ (drift/warm-up) |
| minmax_branchless | -0.087 | ok |
| minmax_intrinsic | -0.544 | HIGH- (thermal bounce) |

**Consistency summary:**

- **minmax_branchless**: won 3/8, lost 5/8
- **minmax_intrinsic**: won 5/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 2.1ns | 117.8ns | 1.8% |  |
| minmax_branchless | 2.3ns | 122.2ns | 1.9% |  |
| minmax_intrinsic | 2.2ns | 117.3ns | 1.9% |  |

## Distribution (algo ns)

```
minmax_branch (n=160, range 107.5-129.5 ns)
    107.5 |#####################
    108.6 |##############
    109.7 |#########################
    110.8 |##################
    111.9 |####################################
    113.0 |#########################
    114.1 |################################
    115.2 |########################################
    116.3 |####################################
    117.4 |#############################
    118.5 |####################################
    119.6 |##########
    120.7 |##############
    121.8 |#########################
    122.9 |##########
    124.0 |#############################
    125.1 |#####################
    126.2 |##################
    127.3 |##################
    128.4 |##############
  (16 below, 12 above range)

minmax_branchless (n=160, range 110.7-132.6 ns)
    110.7 |##############
    111.8 |
    112.9 |#####
    114.0 |#################
    115.1 |########
    116.1 |##################################
    117.2 |##############
    118.3 |########
    119.4 |###############################
    120.5 |######################
    121.6 |#########################
    122.7 |#########################
    123.8 |#########################
    124.9 |########################################
    126.0 |#########################
    127.1 |####################
    128.2 |####################
    129.3 |########
    130.4 |#####
    131.5 |#################
  (15 below, 15 above range)

minmax_intrinsic (n=160, range 107.0-129.1 ns)
    107.0 |#####################
    108.1 |###########
    109.2 |###########
    110.3 |################
    111.4 |##############
    112.5 |#######################
    113.6 |################
    114.7 |############################
    115.8 |################
    116.9 |########################################
    118.0 |##############
    119.1 |#######
    120.2 |##################
    121.3 |#######
    122.4 |################
    123.5 |#######################
    124.6 |##
    125.7 |###########
    126.9 |###########
    128.0 |##
  (13 below, 13 above range)

```

## Diagnostics

- **minmax_branch**: autocorrelation=0.60 (measurement drift or warm-up artifact)
