# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### minmax_branch is fastest but the noisiest (CV 6.4%)

minmax_branch wins on median (401 ns) yet has the highest variance (CV 6.4%), while minmax_branchless is the steadiest (CV 4.9%, 411 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (10 ns) is smaller than the fastest variant's own run-to-run std-dev (25 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (minmax_branch)

The baseline minmax_branch is the fastest (401 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader minmax_branch vs stability leader minmax_branchless (+3% speed for 1.3x steadier)

minmax_branch is fastest (401 ns, CV 6.4%); minmax_branchless gives up 2.5% median for 1.3x lower variance (CV 4.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 2.5% of the fastest

All 3 variants sit between 401 ns and 411 ns - a 2.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### minmax_branchless's edge over baseline is significant but tiny (5 ns, 1.35%)

minmax_branchless differs from baseline minmax_branch by 5 ns (1.35%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (minmax_branch) is the fastest** at 401.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.03x (fastest 401.0 ns, slowest 411.2 ns)

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
| minmax_branch | 2594ns | 2591ns | 2441ns | 2566ns | 2830ns | base |
| minmax_branchless | 2614ns | 2626ns | 2451ns | 2603ns | 2809ns | +0.77% |
| minmax_intrinsic | 2638ns | 2620ns | 2443ns | 2638ns | 2832ns | +1.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 405ns | 377ns | 448ns | base | 40.477 |
| minmax_branchless | 411ns | 382ns | 441ns | +1.51% | 39.876 |
| minmax_intrinsic | 409ns | 380ns | 440ns | +1.16% | 40.013 |

## Performance model

- Peak throughput: **43.440 Gops/s** (minmax_branch; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 40.853 | 94.0% |
| minmax_branchless | 39.844 | 91.7% |
| minmax_intrinsic | 40.246 | 92.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2594ns | 2594ns | base |
| minmax_branchless | 2614ns | 2614ns | +0.77% |
| minmax_intrinsic | 2638ns | 2638ns | +1.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 401ns | base | --- | [392, 404] | --- | --- | --- | --- |
| minmax_branchless | 411ns | +5.4ns (+1.3%) | [+3, +9]ns | [409, 413] | YES | 0.0006 | 0.0003 | 2 |
| minmax_intrinsic | 407ns | no significant difference | [+0, +11]ns | [405, 410] | no | 0.0669 | 0.0669 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 406ns | +1.3% | -6.7% |
| 1 | 2 | 408ns | +3.3% | -0.4% |
| 1 | 3 | 384ns | +0.5% | +5.6% |
| 1 | 4 | 435ns | -4.6% | -0.6% |
| 1 | 5 | 378ns | +2.6% | +13.9% |
| 1 | 6 | 390ns | +5.9% | +4.6% |
| 1 | 7 | 415ns | -4.4% | +1.6% |
| 1 | 8 | 448ns | -2.2% | -11.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | -0.185 | ok |
| minmax_branchless | -0.472 | moderate- |
| minmax_intrinsic | 0.128 | ok |

**Consistency summary:**

- **minmax_branchless**: won 3/8, lost 5/8
- **minmax_intrinsic**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 2.9ns | 404.8ns | 0.7% |  |
| minmax_branchless | 3.0ns | 410.9ns | 0.7% |  |
| minmax_intrinsic | 2.9ns | 409.5ns | 0.7% |  |

## Distribution (algo ns)

```
minmax_branch (n=160, range 377.2-447.5 ns)
    377.2 |################################
    380.7 |################################
    384.2 |########################################
    387.7 |###########
    391.2 |#######################
    394.8 |##
    398.3 |##################
    401.8 |###################################
    405.3 |################################
    408.8 |###########
    412.3 |#######
    415.9 |##
    419.4 |####
    422.9 |##
    426.4 |
    429.9 |##
    433.5 |################
    437.0 |###########
    440.5 |####
    444.0 |################
  (13 below, 15 above range)

minmax_branchless (n=160, range 382.4-440.5 ns)
    382.4 |#########################
    385.3 |#################
    388.2 |######################
    391.1 |#######
    394.1 |##
    397.0 |
    399.9 |#######
    402.8 |############
    405.7 |###################################
    408.6 |########################################
    411.5 |##############################
    414.4 |####################
    417.3 |##############################
    420.2 |######################
    423.1 |##########
    426.0 |##########
    428.9 |
    431.8 |##
    434.7 |##########
    437.6 |###############
  (14 below, 18 above range)

minmax_intrinsic (n=160, range 380.1-439.6 ns)
    380.1 |#################
    383.1 |##########
    386.1 |###############
    389.0 |#######
    392.0 |#####
    395.0 |###############
    398.0 |############
    400.9 |###################################
    403.9 |################################
    406.9 |########################################
    409.9 |######################
    412.9 |####################
    415.8 |##
    418.8 |#####
    421.8 |##
    424.8 |##
    427.7 |##########
    430.7 |############
    433.7 |##############################
    436.7 |######################
  (17 below, 15 above range)

```
