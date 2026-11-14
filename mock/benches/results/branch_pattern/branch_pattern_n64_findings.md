# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (93 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 90 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### branch_if is fastest but the noisiest (CV 11.4%)

branch_if wins on median (90 ns) yet has the highest variance (CV 11.4%), while branch_cmov is the steadiest (CV 6.9%, 93 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (10 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader branch_if vs stability leader branch_cmov (+3% speed for 1.7x steadier)

branch_if is fastest (90 ns, CV 11.4%); branch_cmov gives up 2.5% median for 1.7x lower variance (CV 6.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### branch_if's edge over baseline is significant but tiny (-1 ns, 1.56%)

branch_if differs from baseline branch_cmov by -1 ns (1.56%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: branch_if** at 90.4 ns median (-2.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.03x (fastest 90.4 ns, slowest 92.7 ns)

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
| branch_cmov | 2394ns | 2482ns | 2175ns | 2431ns | 2503ns | base |
| branch_if | 2361ns | 2485ns | 2160ns | 2378ns | 2512ns | -1.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 91ns | 81ns | 98ns | base | 0.704 |
| branch_if | 90ns | 80ns | 101ns | -1.00% | 0.711 |

## Performance model

- Peak throughput: **0.804 Gops/s** (branch_if; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.690 | 85.9% |
| branch_if | 0.708 | 88.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 2394ns | 2394ns | base |
| branch_if | 2361ns | 2361ns | -1.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 93ns | base | --- | [91, 94] | --- | --- | --- | --- |
| branch_if | 90ns | -1.4ns (-1.6%) | [-2, -1]ns | [88, 92] | YES | 0.0023 | 0.0023 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 84ns | +13.1% |
| 1 | 2 | 97ns | -1.9% |
| 1 | 3 | 92ns | +3.4% |
| 1 | 4 | 95ns | -13.0% |
| 1 | 5 | 81ns | +10.3% |
| 1 | 6 | 93ns | -7.9% |
| 1 | 7 | 95ns | -13.9% |
| 1 | 8 | 94ns | +1.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.320 | moderate- |
| branch_if | 0.040 | ok |

**Consistency summary:**

- **branch_if**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 2.9ns | 90.9ns | 3.2% |  |
| branch_if | 2.9ns | 90.0ns | 3.2% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 80.9-98.4 ns)
     80.9 |#######
     81.8 |##########
     82.6 |###############
     83.5 |##########
     84.4 |#####
     85.2 |###############
     86.1 |#######
     87.0 |###############
     87.9 |##########
     88.7 |#################
     89.6 |############
     90.5 |#####
     91.4 |####################
     92.2 |######################
     93.1 |###########################
     94.0 |######################
     94.9 |#################
     95.7 |##############################
     96.6 |#################
     97.5 |########################################
  (16 below, 13 above range)

branch_if (n=160, range 79.6-100.6 ns)
     79.6 |###############
     80.7 |##########
     81.7 |#########################
     82.8 |############
     83.8 |###############
     84.9 |################################
     85.9 |#####
     87.0 |####################
     88.0 |############
     89.1 |############
     90.1 |#####
     91.2 |####################
     92.2 |#########################
     93.3 |############
     94.3 |###########################
     95.4 |########################################
     96.4 |######################
     97.5 |##############################
     98.5 |#######
     99.6 |#####
  (15 below, 3 above range)

```
