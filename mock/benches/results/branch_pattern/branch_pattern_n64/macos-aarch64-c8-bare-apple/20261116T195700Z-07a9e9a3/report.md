# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (83 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 82 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### branch_if is fastest but the noisiest (CV 8.2%)

branch_if wins on median (82 ns) yet has the highest variance (CV 8.2%), while branch_cmov is the steadiest (CV 4.5%, 83 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (7 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader branch_if vs stability leader branch_cmov (+0% speed for 1.8x steadier)

branch_if is fastest (82 ns, CV 8.2%); branch_cmov gives up 0.5% median for 1.8x lower variance (CV 4.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### branch_if's edge over baseline is significant but tiny (-1 ns, 0.97%)

branch_if differs from baseline branch_cmov by -1 ns (0.97%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: branch_if** at 82.5 ns median (-0.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 82.5 ns, slowest 82.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Branchful if/else vs branchless cmov/csel under data-dependent input |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x76543210fedcba98 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| branch_cmov | 2204ns | 2196ns | 2164ns | 2195ns | 2273ns | base |
| branch_if | 2189ns | 2171ns | 2164ns | 2177ns | 2250ns | -0.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 83ns | 78ns | 88ns | base | 0.770 |
| branch_if | 83ns | 78ns | 89ns | -0.07% | 0.771 |

## Performance model

- Peak throughput: **0.824 Gops/s** (branch_if; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.772 | 93.7% |
| branch_if | 0.776 | 94.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 2204ns | 2204ns | base |
| branch_if | 2189ns | 2189ns | -0.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 83ns | base | --- | [82, 84] | --- | --- | --- | --- |
| branch_if | 82ns | -0.8ns (-1.0%) | [-1, -0]ns | [82, 83] | YES | 0.0033 | 0.0033 | 9 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 82ns | +3.8% |
| 1 | 2 | 84ns | -3.5% |
| 1 | 3 | 82ns | +0.5% |
| 1 | 4 | 83ns | -2.3% |
| 1 | 5 | 86ns | -3.1% |
| 1 | 6 | 85ns | -0.2% |
| 1 | 7 | 83ns | -0.2% |
| 1 | 8 | 82ns | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | 0.018 | ok |
| branch_if | -0.053 | ok |

**Consistency summary:**

- **branch_if**: won 6/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 2.8ns | 83.1ns | 3.3% |  |
| branch_if | 2.7ns | 83.0ns | 3.3% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 77.7-88.1 ns)
     77.7 |##
     78.2 |#######
     78.7 |##########
     79.3 |##########
     79.8 |##########
     80.3 |###########################
     80.8 |#################
     81.3 |#########################
     81.9 |####################
     82.4 |####################
     82.9 |###########################
     83.4 |############
     83.9 |######################
     84.5 |#################
     85.0 |########################################
     85.5 |############
     86.0 |#######
     86.5 |############
     87.0 |######################
     87.6 |#######
  (15 below, 12 above range)

branch_if (n=160, range 77.7-89.4 ns)
     77.7 |########
     78.3 |####
     78.9 |##########
     79.4 |###########################
     80.0 |######
     80.6 |##############
     81.2 |########################################
     81.8 |##############
     82.4 |###############################
     82.9 |#####################
     83.5 |##########
     84.1 |###################################
     84.7 |######
     85.3 |#######################
     85.9 |##########
     86.4 |############
     87.0 |##############
     87.6 |
     88.2 |######
     88.8 |##
  (13 below, 4 above range)

```
