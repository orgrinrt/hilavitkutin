# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.2% (fastest 272 ns to slowest 273 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (5 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (acc_atomic_relaxed)

The baseline acc_atomic_relaxed is the fastest (272 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (acc_atomic_relaxed) is the fastest** at 272.3 ns median
- Spread: 1.00x (fastest 272.3 ns, slowest 272.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4823d76519bef0a3 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 2415ns | 2411ns | 2384ns | 2411ns | 2456ns | base |
| acc_atomic_seqcst | 2411ns | 2411ns | 2388ns | 2411ns | 2435ns | -0.13% |
| acc_plain | 2413ns | 2412ns | 2399ns | 2411ns | 2432ns | -0.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 273ns | 268ns | 279ns | base | 0.938 |
| acc_atomic_seqcst | 273ns | 268ns | 277ns | -0.14% | 0.939 |
| acc_plain | 273ns | 269ns | 279ns | +0.14% | 0.937 |

## Performance model

- Peak throughput: **0.957 Gops/s** (acc_atomic_relaxed; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.940 | 98.3% |
| acc_atomic_seqcst | 0.939 | 98.2% |
| acc_plain | 0.938 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 2415ns | 2415ns | base |
| acc_atomic_seqcst | 2411ns | 2411ns | -0.13% |
| acc_plain | 2413ns | 2413ns | -0.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 272ns | base | --- | [272, 273] | --- | --- | --- | --- |
| acc_atomic_seqcst | 272ns | no significant difference | [-1, +0]ns | [272, 273] | no | 0.6576 | 0.3288 | 9 |
| acc_plain | 273ns | no significant difference | [-1, +1]ns | [272, 274] | no | 0.8066 | 0.8066 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 270ns | +1.2% | +0.5% |
| 1 | 2 | 274ns | -1.0% | -0.4% |
| 1 | 3 | 270ns | +1.3% | +1.1% |
| 1 | 4 | 275ns | -0.9% | -0.7% |
| 1 | 5 | 274ns | -1.1% | +0.0% |
| 1 | 6 | 273ns | +0.2% | +0.0% |
| 1 | 7 | 274ns | +0.1% | +0.0% |
| 1 | 8 | 272ns | +0.2% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.339 | moderate- |
| acc_atomic_seqcst | -0.145 | ok |
| acc_plain | 0.107 | ok |

**Consistency summary:**

- **acc_atomic_seqcst**: won 3/8, lost 4/8
- **acc_plain**: won 2/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 2.5ns | 272.9ns | 0.9% |  |
| acc_atomic_seqcst | 2.5ns | 272.5ns | 0.9% |  |
| acc_plain | 2.4ns | 273.3ns | 0.9% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 267.6-279.1 ns)
    267.6 |####
    268.1 |######
    268.7 |##############
    269.3 |################
    269.9 |###############################
    270.5 |######
    271.0 |##############
    271.6 |########################################
    272.2 |##########
    272.8 |##################
    273.4 |##########
    273.9 |############
    274.5 |#####################################
    275.1 |######
    275.7 |#####################
    276.3 |##
    276.8 |######
    277.4 |##########
    278.0 |####
    278.6 |######
  (16 below, 10 above range)

acc_atomic_seqcst (n=160, range 268.4-276.8 ns)
    268.4 |################
    268.8 |################
    269.2 |#############
    269.7 |#######################
    270.1 |################
    270.5 |########################################
    270.9 |####################################
    271.3 |##########################
    271.8 |###
    272.2 |##########################
    272.6 |#################################
    273.0 |#################################
    273.4 |####################
    273.8 |#######################
    274.3 |#################################
    274.7 |####################
    275.1 |##########
    275.5 |##########################
    275.9 |#############
    276.4 |################
  (16 below, 9 above range)

acc_plain (n=160, range 268.8-278.6 ns)
    268.8 |################
    269.3 |########
    269.8 |#############
    270.3 |#####################
    270.8 |##################################
    271.3 |##########################
    271.8 |################################
    272.2 |################
    272.7 |##################
    273.2 |################
    273.7 |#############################
    274.2 |########################################
    274.7 |################################
    275.2 |#####################
    275.6 |##########
    276.1 |########
    276.6 |#####
    277.1 |################
    277.6 |#####
    278.1 |########
  (13 below, 5 above range)

```
