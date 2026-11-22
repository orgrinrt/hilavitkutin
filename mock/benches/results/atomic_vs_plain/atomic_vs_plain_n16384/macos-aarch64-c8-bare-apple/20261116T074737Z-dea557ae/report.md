# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (acc_atomic_relaxed) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline acc_atomic_relaxed has the worst median (20.46 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest acc_plain at 20.43 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (28 ns) is smaller than the fastest variant's own run-to-run std-dev (315 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.1% of the fastest

All 3 variants sit between 20.43 us and 20.46 us - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### acc_atomic_seqcst's edge over baseline is significant but tiny (-14 ns, 0.07%)

acc_atomic_seqcst differs from baseline acc_atomic_relaxed by -14 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: acc_plain** at 20431.1 ns median (-0.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.00x (fastest 20431.1 ns, slowest 20458.8 ns)

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
| acc_atomic_relaxed | 22756ns | 22567ns | 22495ns | 22604ns | 23472ns | base |
| acc_atomic_seqcst | 22657ns | 22544ns | 22493ns | 22549ns | 23145ns | -0.43% |
| acc_plain | 22645ns | 22537ns | 22494ns | 22540ns | 23109ns | -0.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 20632ns | 20403ns | 21279ns | base | 0.794 |
| acc_atomic_seqcst | 20540ns | 20401ns | 20966ns | -0.45% | 0.798 |
| acc_plain | 20531ns | 20400ns | 20944ns | -0.49% | 0.798 |

## Performance model

- Peak throughput: **0.803 Gops/s** (acc_plain; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.801 | 99.7% |
| acc_atomic_seqcst | 0.801 | 99.8% |
| acc_plain | 0.802 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 22756ns | 22756ns | base |
| acc_atomic_seqcst | 22657ns | 22657ns | -0.43% |
| acc_plain | 22645ns | 22645ns | -0.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 20459ns | base | --- | [20449, 20485] | --- | --- | --- | --- |
| acc_atomic_seqcst | 20445ns | -14.0ns (-0.1%) | [-42, -3]ns | [20420, 20451] | YES | 0.0052 | 0.0052 | 2 |
| acc_plain | 20431ns | -20.5ns (-0.1%) | [-45, -3]ns | [20421, 20448] | YES | 0.0052 | 0.0033 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 21384ns | -4.3% | -4.5% |
| 1 | 2 | 20421ns | -0.0% | +0.2% |
| 1 | 3 | 20468ns | +0.0% | -0.2% |
| 1 | 4 | 20406ns | +0.2% | +0.3% |
| 1 | 5 | 20514ns | -0.3% | -0.5% |
| 1 | 6 | 20453ns | -0.1% | -0.0% |
| 1 | 7 | 20859ns | -2.2% | -0.1% |
| 1 | 8 | 20455ns | -0.1% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.157 | ok |
| acc_atomic_seqcst | -0.103 | ok |
| acc_plain | -0.197 | ok |

**Consistency summary:**

- **acc_atomic_seqcst**: won 4/8, lost 1/8
- **acc_plain**: won 5/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 2.7ns | 20632.0ns | 0.0% |  |
| acc_atomic_seqcst | 2.8ns | 20540.1ns | 0.0% |  |
| acc_plain | 2.7ns | 20531.0ns | 0.0% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 20402.7-21279.2 ns)
  20402.7 |########################################
  20446.6 |########################
  20490.4 |##############
  20534.2 |#####
  20578.0 |
  20621.8 |#
  20665.7 |
  20709.5 |
  20753.3 |###
  20797.1 |
  20841.0 |
  20884.8 |
  20928.6 |##
  20972.4 |
  21016.3 |
  21060.1 |
  21103.9 |#####
  21147.7 |####
  21191.6 |#
  21235.4 |#
  (17 below, 15 above range)

acc_atomic_seqcst (n=160, range 20401.5-20965.9 ns)
  20401.5 |########################################
  20429.7 |####################
  20457.9 |##########
  20486.1 |#####
  20514.4 |
  20542.6 |##
  20570.8 |##
  20599.0 |##
  20627.3 |##
  20655.5 |
  20683.7 |
  20711.9 |
  20740.1 |
  20768.4 |##
  20796.6 |
  20824.8 |
  20853.0 |
  20881.2 |
  20909.5 |
  20937.7 |#
  (15 below, 11 above range)

acc_plain (n=160, range 20400.2-20944.1 ns)
  20400.2 |########################################
  20427.3 |##############
  20454.5 |#############
  20481.7 |#####
  20508.9 |
  20536.1 |#
  20563.3 |
  20590.5 |
  20617.7 |###
  20644.9 |
  20672.1 |
  20699.3 |
  20726.5 |
  20753.7 |
  20780.9 |
  20808.1 |#
  20835.3 |#
  20862.5 |
  20889.7 |
  20916.9 |
  (15 below, 12 above range)

```
