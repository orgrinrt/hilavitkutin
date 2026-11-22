# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (26.07 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 26.07 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 26.07 us to slowest 26.07 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (136 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: branch_if** at 26066.7 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 26066.7 ns, slowest 26069.2 ns)

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
| branch_cmov | 28191ns | 28170ns | 28122ns | 28173ns | 28313ns | base |
| branch_if | 28210ns | 28167ns | 28122ns | 28174ns | 28406ns | +0.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 26089ns | 26026ns | 26201ns | base | 0.628 |
| branch_if | 26104ns | 26027ns | 26279ns | +0.06% | 0.628 |

## Performance model

- Peak throughput: **0.630 Gops/s** (branch_cmov; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.628 | 99.8% |
| branch_if | 0.629 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 28191ns | 28191ns | base |
| branch_if | 28210ns | 28210ns | +0.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 26069ns | base | --- | [26058, 26081] | --- | --- | --- | --- |
| branch_if | 26067ns | no significant difference | [-9, +6]ns | [26056, 26078] | no | 0.9366 | 0.9366 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 26096ns | -0.2% |
| 1 | 2 | 26046ns | +0.1% |
| 1 | 3 | 26078ns | +0.0% |
| 1 | 4 | 26095ns | -0.2% |
| 1 | 5 | 26051ns | +0.1% |
| 1 | 6 | 26055ns | +0.1% |
| 1 | 7 | 26056ns | +0.0% |
| 1 | 8 | 26087ns | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.274 | moderate- |
| branch_if | -0.105 | ok |

**Consistency summary:**

- **branch_if**: won 3/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 2.9ns | 26088.8ns | 0.0% |  |
| branch_if | 3.0ns | 26103.7ns | 0.0% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 26025.6-26200.9 ns)
  26025.6 |###################
  26034.3 |###########################
  26043.1 |########################################
  26051.9 |#################
  26060.6 |##########
  26069.4 |###############
  26078.2 |#################
  26086.9 |###############
  26095.7 |##########
  26104.5 |############
  26113.2 |########
  26122.0 |###
  26130.8 |#####
  26139.5 |###
  26148.3 |########
  26157.0 |######
  26165.8 |###
  26174.6 |###
  26183.3 |#
  26192.1 |
  (14 below, 13 above range)

branch_if (n=160, range 26026.8-26278.7 ns)
  26026.8 |#########################
  26039.4 |########################################
  26052.0 |####################
  26064.6 |################
  26077.2 |#############
  26089.8 |###############
  26102.4 |##########
  26115.0 |########
  26127.6 |##
  26140.2 |
  26152.8 |######
  26165.4 |#####
  26178.0 |##
  26190.6 |#
  26203.1 |##
  26215.7 |#
  26228.3 |##
  26240.9 |#
  26253.5 |#
  26266.1 |##
  (11 below, 7 above range)

```
