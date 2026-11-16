# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (acc_atomic_relaxed) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline acc_atomic_relaxed has the worst median (273 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest acc_atomic_seqcst at 272 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.6% of the fastest

All 3 variants sit between 272 ns and 273 ns - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### acc_atomic_seqcst's edge over baseline is significant but tiny (-2 ns, 0.62%)

acc_atomic_seqcst differs from baseline acc_atomic_relaxed by -2 ns (0.62%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: acc_atomic_seqcst** at 271.7 ns median (-0.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.01x (fastest 271.7 ns, slowest 273.3 ns)

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
| acc_atomic_relaxed | 2407ns | 2400ns | 2372ns | 2401ns | 2460ns | base |
| acc_atomic_seqcst | 2396ns | 2395ns | 2351ns | 2395ns | 2443ns | -0.45% |
| acc_plain | 2410ns | 2397ns | 2367ns | 2397ns | 2490ns | +0.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 273ns | 267ns | 281ns | base | 0.936 |
| acc_atomic_seqcst | 272ns | 266ns | 277ns | -0.70% | 0.943 |
| acc_plain | 273ns | 267ns | 282ns | -0.19% | 0.938 |

## Performance model

- Peak throughput: **0.961 Gops/s** (acc_atomic_seqcst; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.937 | 97.5% |
| acc_atomic_seqcst | 0.942 | 98.1% |
| acc_plain | 0.940 | 97.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 2407ns | 2407ns | base |
| acc_atomic_seqcst | 2396ns | 2396ns | -0.45% |
| acc_plain | 2410ns | 2410ns | +0.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 273ns | base | --- | [272, 274] | --- | --- | --- | --- |
| acc_atomic_seqcst | 272ns | -1.7ns (-0.6%) | [-3, -1]ns | [270, 272] | YES | 0.0004 | 0.0002 | 3 |
| acc_plain | 272ns | -1.2ns (-0.5%) | [-2, -0]ns | [272, 273] | YES | 0.0099 | 0.0099 | 5 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 272ns | -1.9% | -0.1% |
| 1 | 2 | 273ns | -1.1% | -2.2% |
| 1 | 3 | 280ns | -2.7% | -2.8% |
| 1 | 4 | 274ns | +0.2% | +0.1% |
| 1 | 5 | 270ns | +0.9% | +1.1% |
| 1 | 6 | 270ns | +0.8% | +0.9% |
| 1 | 7 | 273ns | -0.7% | +0.0% |
| 1 | 8 | 273ns | -0.5% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | 0.163 | ok |
| acc_atomic_seqcst | 0.326 | moderate+ |
| acc_plain | 0.031 | ok |

**Consistency summary:**

- **acc_atomic_seqcst**: won 5/8, lost 3/8
- **acc_plain**: won 3/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 2.6ns | 273.5ns | 1.0% |  |
| acc_atomic_seqcst | 2.6ns | 271.5ns | 1.0% |  |
| acc_plain | 2.7ns | 272.9ns | 1.0% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 267.2-280.9 ns)
    267.2 |#########
    267.9 |##################
    268.5 |###############
    269.2 |##################
    269.9 |####################################
    270.6 |#################################
    271.3 |###############
    272.0 |#################################
    272.6 |####################################
    273.3 |##################
    274.0 |########################################
    274.7 |#########
    275.4 |####################################
    276.1 |###########################
    276.8 |######
    277.4 |#####################
    278.1 |############
    278.8 |######
    279.5 |#########
    280.2 |############
  (15 below, 9 above range)

acc_atomic_seqcst (n=160, range 266.4-276.6 ns)
    266.4 |#######
    266.9 |####
    267.4 |#########################
    267.9 |##############
    268.4 |###########
    268.9 |#########
    269.5 |################
    270.0 |########################################
    270.5 |#######
    271.0 |###########
    271.5 |################
    272.0 |################################
    272.5 |###########
    273.0 |################
    273.5 |##############
    274.0 |################
    274.5 |##############################
    275.0 |##############
    275.6 |#######
    276.1 |
  (14 below, 15 above range)

acc_plain (n=160, range 266.5-282.0 ns)
    266.5 |################
    267.3 |############
    268.1 |################
    268.9 |#################################
    269.6 |############
    270.4 |######
    271.2 |#####################
    272.0 |#####################################
    272.7 |########################################
    273.5 |#########################
    274.3 |#############################
    275.1 |################
    275.8 |######
    276.6 |##########
    277.4 |##########
    278.2 |######
    278.9 |##
    279.7 |
    280.5 |
    281.3 |
  (12 below, 3 above range)

```
