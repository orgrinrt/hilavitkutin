# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (26.15 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 26.09 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (61 ns) is smaller than the fastest variant's own run-to-run std-dev (341 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### branch_if's edge over baseline is significant but tiny (-40 ns, 0.15%)

branch_if differs from baseline branch_cmov by -40 ns (0.15%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: branch_if** at 26090.0 ns median (-0.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 26090.0 ns, slowest 26151.2 ns)

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
| branch_cmov | 28462ns | 28266ns | 28127ns | 28329ns | 29195ns | base |
| branch_if | 28348ns | 28181ns | 28098ns | 28212ns | 29007ns | -0.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 26338ns | 26036ns | 27013ns | base | 0.622 |
| branch_if | 26240ns | 26009ns | 26846ns | -0.37% | 0.624 |

## Performance model

- Peak throughput: **0.630 Gops/s** (branch_if; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.627 | 99.5% |
| branch_if | 0.628 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 28462ns | 28462ns | base |
| branch_if | 28348ns | 28348ns | -0.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 26151ns | base | --- | [26117, 26236] | --- | --- | --- | --- |
| branch_if | 26090ns | -40.0ns (-0.2%) | [-86, -15]ns | [26079, 26107] | YES | 0.0001 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 26316ns | -0.9% |
| 1 | 2 | 26144ns | -0.2% |
| 1 | 3 | 26109ns | +0.2% |
| 1 | 4 | 26955ns | -0.8% |
| 1 | 5 | 26117ns | -0.3% |
| 1 | 6 | 26176ns | -0.4% |
| 1 | 7 | 26068ns | -0.0% |
| 1 | 8 | 26077ns | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.210 | moderate- |
| branch_if | -0.099 | ok |

**Consistency summary:**

- **branch_if**: won 5/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 3.0ns | 26337.8ns | 0.0% |  |
| branch_if | 2.9ns | 26239.6ns | 0.0% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 26035.9-27013.4 ns)
  26035.9 |########################################
  26084.8 |#############
  26133.7 |###########
  26182.5 |#######
  26231.4 |#########
  26280.3 |###
  26329.2 |###
  26378.0 |#####
  26426.9 |#####
  26475.8 |#
  26524.7 |#
  26573.5 |##
  26622.4 |
  26671.3 |
  26720.2 |
  26769.0 |#
  26817.9 |#
  26866.8 |
  26915.7 |##########
  26964.6 |#######
  (15 below, 6 above range)

branch_if (n=160, range 26009.4-26845.6 ns)
  26009.4 |########################################
  26051.2 |########################################
  26093.0 |########################
  26134.9 |#########
  26176.7 |####
  26218.5 |#####
  26260.3 |####
  26302.1 |##
  26343.9 |##
  26385.7 |#
  26427.5 |##
  26469.3 |
  26511.1 |##
  26552.9 |#
  26594.8 |##
  26636.6 |
  26678.4 |##
  26720.2 |##
  26762.0 |####
  26803.8 |
  (13 below, 15 above range)

```
