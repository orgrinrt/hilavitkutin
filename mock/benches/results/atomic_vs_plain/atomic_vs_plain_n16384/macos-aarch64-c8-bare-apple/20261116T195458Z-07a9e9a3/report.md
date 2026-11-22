# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (acc_atomic_relaxed) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline acc_atomic_relaxed has the worst median (20.44 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest acc_plain at 20.41 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (28 ns) is smaller than the fastest variant's own run-to-run std-dev (112 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.1% of the fastest

All 3 variants sit between 20.41 us and 20.44 us - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### acc_atomic_seqcst's edge over baseline is significant but tiny (-3 ns, 0.02%)

acc_atomic_seqcst differs from baseline acc_atomic_relaxed by -3 ns (0.02%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: acc_plain** at 20409.0 ns median (-0.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.00x (fastest 20409.0 ns, slowest 20437.1 ns)

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
| acc_atomic_relaxed | 22625ns | 22548ns | 22507ns | 22553ns | 22961ns | base |
| acc_atomic_seqcst | 22557ns | 22532ns | 22501ns | 22533ns | 22685ns | -0.30% |
| acc_plain | 22563ns | 22520ns | 22502ns | 22529ns | 22725ns | -0.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 20508ns | 20401ns | 20820ns | base | 0.799 |
| acc_atomic_seqcst | 20449ns | 20400ns | 20568ns | -0.29% | 0.801 |
| acc_plain | 20454ns | 20400ns | 20605ns | -0.26% | 0.801 |

## Performance model

- Peak throughput: **0.803 Gops/s** (acc_atomic_seqcst; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.802 | 99.8% |
| acc_atomic_seqcst | 0.802 | 99.9% |
| acc_plain | 0.803 | 100.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 22625ns | 22625ns | base |
| acc_atomic_seqcst | 22557ns | 22557ns | -0.30% |
| acc_plain | 22563ns | 22563ns | -0.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 20437ns | base | --- | [20427, 20450] | --- | --- | --- | --- |
| acc_atomic_seqcst | 20416ns | -3.3ns (-0.0%) | [-23, -1]ns | [20408, 20430] | YES | 0.0177 | 0.0089 | 0 |
| acc_plain | 20409ns | -2.1ns (-0.0%) | [-18, -1]ns | [20405, 20421] | YES | 0.0261 | 0.0261 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 20411ns | +0.2% | -0.0% |
| 1 | 2 | 20475ns | -0.2% | -0.3% |
| 1 | 3 | 20452ns | -0.2% | -0.2% |
| 1 | 4 | 20479ns | -0.3% | -0.3% |
| 1 | 5 | 20407ns | +0.2% | +0.1% |
| 1 | 6 | 20415ns | -0.0% | +0.2% |
| 1 | 7 | 20447ns | -0.2% | -0.2% |
| 1 | 8 | 20416ns | -0.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.139 | ok |
| acc_atomic_seqcst | 0.102 | ok |
| acc_plain | -0.182 | ok |

**Consistency summary:**

- **acc_atomic_seqcst**: won 4/8, lost 2/8
- **acc_plain**: won 4/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 3.0ns | 20507.9ns | 0.0% |  |
| acc_atomic_seqcst | 2.8ns | 20448.8ns | 0.0% |  |
| acc_plain | 3.1ns | 20453.6ns | 0.0% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 20401.0-20819.9 ns)
  20401.0 |########################################
  20421.9 |################
  20442.9 |##################
  20463.8 |########
  20484.8 |########
  20505.7 |#
  20526.7 |####
  20547.6 |#
  20568.6 |
  20589.5 |###
  20610.5 |
  20631.4 |#
  20652.3 |
  20673.3 |
  20694.2 |
  20715.2 |
  20736.1 |
  20757.1 |
  20778.0 |
  20799.0 |
  (15 below, 10 above range)

acc_atomic_seqcst (n=160, range 20399.8-20568.1 ns)
  20399.8 |########################################
  20408.2 |#######
  20416.6 |######
  20425.0 |#####
  20433.4 |###
  20441.9 |#########
  20450.3 |#
  20458.7 |##
  20467.1 |####
  20475.5 |####
  20483.9 |#
  20492.4 |####
  20500.8 |
  20509.2 |#
  20517.6 |##
  20526.0 |##
  20534.5 |
  20542.9 |
  20551.3 |
  20559.7 |
  (15 below, 9 above range)

acc_plain (n=160, range 20400.3-20604.9 ns)
  20400.3 |########################################
  20410.5 |###
  20420.8 |####
  20431.0 |####
  20441.2 |#####
  20451.5 |#
  20461.7 |##
  20471.9 |#
  20482.2 |##
  20492.4 |##
  20502.6 |#
  20512.8 |#
  20523.1 |#
  20533.3 |
  20543.5 |
  20553.8 |#
  20564.0 |#
  20574.2 |
  20584.4 |
  20594.7 |
  (11 below, 9 above range)

```
