# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (41 ns) is smaller than the fastest variant's own run-to-run std-dev (354 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (acc_atomic_relaxed)

The baseline acc_atomic_relaxed is the fastest (20.46 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.2% of the fastest

All 3 variants sit between 20.46 us and 20.50 us - a 0.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### acc_atomic_seqcst's edge over baseline is significant but tiny (15 ns, 0.07%)

acc_atomic_seqcst differs from baseline acc_atomic_relaxed by 15 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (acc_atomic_relaxed) is the fastest** at 20457.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 20457.7 ns, slowest 20498.3 ns)

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
| acc_atomic_relaxed | 22736ns | 22566ns | 22504ns | 22589ns | 23410ns | base |
| acc_atomic_seqcst | 22773ns | 22604ns | 22505ns | 22672ns | 23345ns | +0.16% |
| acc_plain | 22765ns | 22589ns | 22498ns | 22640ns | 23407ns | +0.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 20606ns | 20402ns | 21201ns | base | 0.795 |
| acc_atomic_seqcst | 20645ns | 20405ns | 21155ns | +0.19% | 0.794 |
| acc_plain | 20641ns | 20401ns | 21223ns | +0.17% | 0.794 |

## Performance model

- Peak throughput: **0.803 Gops/s** (acc_plain; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.801 | 99.7% |
| acc_atomic_seqcst | 0.799 | 99.5% |
| acc_plain | 0.800 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 22736ns | 22736ns | base |
| acc_atomic_seqcst | 22773ns | 22773ns | +0.16% |
| acc_plain | 22765ns | 22765ns | +0.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 20458ns | base | --- | [20449, 20470] | --- | --- | --- | --- |
| acc_atomic_seqcst | 20498ns | +14.6ns (+0.1%) | [+0, +45]ns | [20467, 20540] | YES (adj: no) | 0.1374 | 0.0687 | 0 |
| acc_plain | 20485ns | no significant difference | [-2, +12]ns | [20455, 20499] | no | 0.5233 | 0.5233 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 20469ns | -0.1% | +0.3% |
| 1 | 2 | 20439ns | +0.4% | +0.2% |
| 1 | 3 | 20461ns | +1.2% | +0.4% |
| 1 | 4 | 20454ns | +0.1% | -0.2% |
| 1 | 5 | 20595ns | -0.6% | -0.5% |
| 1 | 6 | 20452ns | +0.5% | +0.1% |
| 1 | 7 | 20466ns | +0.1% | +0.0% |
| 1 | 8 | 20418ns | +0.9% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.169 | ok |
| acc_atomic_seqcst | -0.264 | moderate- |
| acc_plain | -0.355 | moderate- |

**Consistency summary:**

- **acc_atomic_seqcst**: won 1/8, lost 4/8
- **acc_plain**: won 2/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 3.0ns | 20605.9ns | 0.0% |  |
| acc_atomic_seqcst | 3.0ns | 20644.5ns | 0.0% |  |
| acc_plain | 2.9ns | 20641.2ns | 0.0% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 20401.7-21201.1 ns)
  20401.7 |########################################
  20441.6 |##########################
  20481.6 |#######
  20521.6 |##########
  20561.6 |###
  20601.5 |
  20641.5 |#
  20681.5 |
  20721.4 |#
  20761.4 |#
  20801.4 |
  20841.4 |
  20881.3 |##
  20921.3 |#
  20961.3 |###
  21001.2 |
  21041.2 |
  21081.2 |#
  21121.2 |##
  21161.1 |
  (12 below, 13 above range)

acc_atomic_seqcst (n=160, range 20405.2-21154.6 ns)
  20405.2 |########################################
  20442.7 |#############################
  20480.2 |####################
  20517.6 |#########
  20555.1 |#######
  20592.6 |##
  20630.1 |##
  20667.5 |##
  20705.0 |##
  20742.5 |#
  20779.9 |###############
  20817.4 |#####
  20854.9 |#####
  20892.3 |
  20929.8 |#####
  20967.3 |#####
  21004.8 |##
  21042.2 |
  21079.7 |##
  21117.2 |#######
  (19 below, 11 above range)

acc_plain (n=160, range 20401.1-21223.2 ns)
  20401.1 |########################################
  20442.2 |###################
  20483.3 |##############
  20524.4 |######
  20565.5 |##
  20606.6 |##
  20647.7 |
  20688.8 |###
  20729.9 |####
  20771.0 |#######
  20812.1 |#
  20853.2 |###
  20894.3 |#
  20935.4 |
  20976.5 |###
  21017.6 |
  21058.7 |###
  21099.8 |####
  21141.0 |
  21182.1 |
  (16 below, 10 above range)

```
