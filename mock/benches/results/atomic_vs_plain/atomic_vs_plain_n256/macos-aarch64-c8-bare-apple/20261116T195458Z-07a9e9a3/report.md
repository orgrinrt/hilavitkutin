# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.3% (fastest 269 ns to slowest 270 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (6 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (acc_atomic_relaxed)

The baseline acc_atomic_relaxed is the fastest (269 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (acc_atomic_relaxed) is the fastest** at 268.8 ns median
- Spread: 1.00x (fastest 268.8 ns, slowest 269.6 ns)

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
| acc_atomic_relaxed | 2378ns | 2368ns | 2362ns | 2368ns | 2423ns | base |
| acc_atomic_seqcst | 2382ns | 2369ns | 2363ns | 2375ns | 2421ns | +0.18% |
| acc_plain | 2399ns | 2369ns | 2361ns | 2380ns | 2493ns | +0.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 270ns | 264ns | 278ns | base | 0.949 |
| acc_atomic_seqcst | 270ns | 264ns | 278ns | +0.10% | 0.948 |
| acc_plain | 271ns | 265ns | 282ns | +0.50% | 0.944 |

## Performance model

- Peak throughput: **0.968 Gops/s** (acc_atomic_relaxed; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.952 | 98.3% |
| acc_atomic_seqcst | 0.951 | 98.2% |
| acc_plain | 0.950 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 2378ns | 2378ns | base |
| acc_atomic_seqcst | 2382ns | 2382ns | +0.18% |
| acc_plain | 2399ns | 2399ns | +0.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 269ns | base | --- | [268, 269] | --- | --- | --- | --- |
| acc_atomic_seqcst | 269ns | no significant difference | [-0, +1]ns | [268, 270] | no | 0.4713 | 0.4713 | 4 |
| acc_plain | 270ns | no significant difference | [+0, +1]ns | [269, 270] | no | 0.2510 | 0.1255 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 274ns | -2.4% | -2.1% |
| 1 | 2 | 268ns | -0.4% | +0.2% |
| 1 | 3 | 268ns | +0.2% | +0.1% |
| 1 | 4 | 268ns | -0.2% | +0.1% |
| 1 | 5 | 268ns | +0.2% | +0.4% |
| 1 | 6 | 269ns | +1.5% | +3.2% |
| 1 | 7 | 268ns | +2.0% | +2.2% |
| 1 | 8 | 268ns | +0.1% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | 0.001 | ok |
| acc_atomic_seqcst | 0.410 | moderate+ |
| acc_plain | 0.257 | moderate+ |

**Consistency summary:**

- **acc_atomic_seqcst**: won 3/8, lost 4/8
- **acc_plain**: won 1/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 2.4ns | 269.8ns | 0.9% |  |
| acc_atomic_seqcst | 2.5ns | 270.1ns | 0.9% |  |
| acc_plain | 2.4ns | 271.1ns | 0.9% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 264.3-278.3 ns)
    264.3 |######################
    265.0 |##########
    265.7 |##################################
    266.4 |######################
    267.1 |##########
    267.8 |##############################
    268.5 |########################################
    269.2 |##########
    269.9 |##############
    270.6 |##########
    271.3 |##
    272.0 |##########################
    272.7 |##############
    273.4 |##
    274.1 |##########
    274.8 |############
    275.5 |##
    276.2 |####
    276.9 |
    277.6 |##
  (13 below, 9 above range)

acc_atomic_seqcst (n=160, range 264.4-278.2 ns)
    264.4 |##################
    265.1 |####
    265.8 |#####################
    266.5 |############################
    267.2 |#####################
    267.8 |################################
    268.5 |########################################
    269.2 |#########
    269.9 |#########################
    270.6 |##############################
    271.3 |###########
    272.0 |##################
    272.7 |############################
    273.4 |
    274.1 |###########
    274.7 |##
    275.4 |#######
    276.1 |#########
    276.8 |##
    277.5 |##
  (16 below, 5 above range)

acc_plain (n=160, range 264.6-281.7 ns)
    264.6 |####################
    265.4 |####################
    266.3 |####################
    267.1 |##########################
    268.0 |########################
    268.9 |########################################
    269.7 |############################
    270.6 |############################
    271.4 |######################
    272.3 |
    273.1 |#############
    274.0 |#############
    274.9 |######
    275.7 |########
    276.6 |###############
    277.4 |####
    278.3 |########
    279.1 |##
    280.0 |####
    280.9 |####
  (15 below, 4 above range)

```
