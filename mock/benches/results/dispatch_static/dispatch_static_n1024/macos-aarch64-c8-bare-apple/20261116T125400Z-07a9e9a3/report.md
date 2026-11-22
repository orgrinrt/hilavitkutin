# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 110 ns to slowest 110 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (3 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (110 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader dispatch_const_ptr vs stability leader dispatch_direct (+0% speed for 1.1x steadier)

dispatch_const_ptr is fastest (110 ns, CV 3.1%); dispatch_direct gives up 0.0% median for 1.1x lower variance (CV 2.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (dispatch_const_ptr) is the fastest** at 109.6 ns median
- Spread: 1.00x (fastest 109.6 ns, slowest 109.6 ns)

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
| dispatch_const_ptr | 2208ns | 2205ns | 2199ns | 2205ns | 2227ns | base |
| dispatch_direct | 2210ns | 2205ns | 2199ns | 2205ns | 2236ns | +0.08% |
| dispatch_trait | 2211ns | 2208ns | 2202ns | 2208ns | 2227ns | +0.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 110ns | 105ns | 114ns | base | 9.338 |
| dispatch_direct | 110ns | 105ns | 114ns | -0.03% | 9.341 |
| dispatch_trait | 110ns | 105ns | 117ns | +0.24% | 9.316 |

## Performance model

- Peak throughput: **9.763 Gops/s** (dispatch_const_ptr; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 9.343 | 95.7% |
| dispatch_direct | 9.343 | 95.7% |
| dispatch_trait | 9.343 | 95.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2208ns | 2208ns | base |
| dispatch_direct | 2210ns | 2210ns | +0.08% |
| dispatch_trait | 2211ns | 2211ns | +0.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 110ns | base | --- | [109, 111] | --- | --- | --- | --- |
| dispatch_direct | 110ns | no significant difference | [-1, +0]ns | [109, 110] | no | 0.5090 | 0.2545 | 9 |
| dispatch_trait | 110ns | no significant difference | [-0, +0]ns | [108, 110] | no | 1.0000 | 1.0000 | 7 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 110ns | -0.6% | -1.2% |
| 1 | 2 | 110ns | -0.7% | -2.3% |
| 1 | 3 | 111ns | -1.5% | -2.2% |
| 1 | 4 | 109ns | +1.0% | +1.0% |
| 1 | 5 | 110ns | -0.4% | -0.5% |
| 1 | 6 | 110ns | -0.4% | +0.0% |
| 1 | 7 | 109ns | -0.2% | -0.2% |
| 1 | 8 | 109ns | +0.6% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.171 | ok |
| dispatch_direct | 0.083 | ok |
| dispatch_trait | 0.390 | moderate+ |

**Consistency summary:**

- **dispatch_direct**: won 6/8, lost 2/8
- **dispatch_trait**: won 5/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 2.8ns | 109.7ns | 2.6% |  |
| dispatch_direct | 2.7ns | 109.6ns | 2.5% |  |
| dispatch_trait | 2.8ns | 109.9ns | 2.5% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 104.9-114.2 ns)
    104.9 |#################
    105.4 |####################
    105.8 |######################
    106.3 |#######
    106.8 |###############
    107.2 |############
    107.7 |############
    108.2 |#######
    108.6 |####################
    109.1 |#################
    109.6 |########################################
    110.0 |##########
    110.5 |#########################
    111.0 |###############
    111.4 |############
    111.9 |#################
    112.4 |#################
    112.8 |####################
    113.3 |###########################
    113.8 |############
  (11 below, 9 above range)

dispatch_direct (n=160, range 105.4-114.3 ns)
    105.4 |##############
    105.9 |##
    106.3 |######################
    106.8 |##############
    107.2 |####################
    107.6 |##################################
    108.1 |####################
    108.5 |##############
    109.0 |#########################
    109.4 |########################################
    109.9 |####################
    110.3 |############################
    110.7 |########
    111.2 |#################
    111.6 |############################
    112.1 |####################
    112.5 |##############
    113.0 |#################
    113.4 |##
    113.8 |##############
  (17 below, 10 above range)

dispatch_trait (n=160, range 105.0-116.5 ns)
    105.0 |######
    105.6 |############
    106.2 |########################################
    106.7 |####
    107.3 |############
    107.9 |#####################################
    108.5 |########
    109.0 |###############################
    109.6 |##############
    110.2 |#######################
    110.8 |#####################
    111.3 |########
    111.9 |##########
    112.5 |##################
    113.1 |######
    113.6 |##################
    114.2 |########
    114.8 |######
    115.4 |######
    116.0 |####
  (14 below, 3 above range)

```
