# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (3 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### minmax_branch is inconsistent: worst-20% is 1.7x its best-20%

minmax_branch's best 20% of batches run at 16 ns but its worst 20% at 28 ns (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 4.2% of the fastest

All 3 variants sit between 19 ns and 20 ns - a 4.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: minmax_intrinsic** at 19.2 ns median (-3.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.04x (fastest 19.2 ns, slowest 20.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5d49a3f1b7c06e82 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| minmax_branch | 2140ns | 2134ns | 2128ns | 2134ns | 2168ns | base |
| minmax_branchless | 2137ns | 2133ns | 2126ns | 2133ns | 2157ns | -0.14% |
| minmax_intrinsic | 2138ns | 2132ns | 2127ns | 2132ns | 2167ns | -0.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 21ns | 16ns | 28ns | base | 12.236 |
| minmax_branchless | 20ns | 16ns | 25ns | -3.38% | 12.665 |
| minmax_intrinsic | 19ns | 16ns | 22ns | -9.25% | 13.484 |

## Performance model

- Peak throughput: **16.193 Gops/s** (minmax_intrinsic; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 12.929 | 79.8% |
| minmax_branchless | 12.800 | 79.0% |
| minmax_intrinsic | 13.333 | 82.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2140ns | 2140ns | base |
| minmax_branchless | 2137ns | 2137ns | -0.14% |
| minmax_intrinsic | 2138ns | 2138ns | -0.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 20ns | base | --- | [19, 20] | --- | --- | --- | --- |
| minmax_branchless | 20ns | no significant difference | [-1, +1]ns | [20, 20] | no | 0.8078 | 0.8078 | 8 |
| minmax_intrinsic | 19ns | -1.2ns (-6.1%) | [-2, -0]ns | [19, 19] | YES | 0.0055 | 0.0027 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 23ns | -15.3% | -20.1% |
| 1 | 2 | 19ns | -7.7% | -3.1% |
| 1 | 3 | 19ns | +6.3% | +1.1% |
| 1 | 4 | 19ns | +17.2% | -6.8% |
| 1 | 5 | 18ns | +23.0% | +3.8% |
| 1 | 6 | 19ns | +0.0% | +3.2% |
| 1 | 7 | 19ns | +5.3% | +7.4% |
| 1 | 8 | 23ns | -15.2% | -17.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | 0.014 | ok |
| minmax_branchless | 0.252 | moderate+ |
| minmax_intrinsic | 0.110 | ok |

**Consistency summary:**

- **minmax_branchless**: won 3/8, lost 4/8
- **minmax_intrinsic**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 2.5ns | 20.9ns | 11.8% | HIGH |
| minmax_branchless | 2.5ns | 20.2ns | 12.2% | HIGH |
| minmax_intrinsic | 2.3ns | 19.0ns | 11.9% | HIGH |

## Distribution (algo ns)

```
minmax_branch (n=160, range 16.4-28.4 ns)
     16.4 |###
     17.0 |##################
     17.6 |################
     18.2 |########################################
     18.8 |##########
     19.4 |##############################
     20.0 |#########
     20.6 |##################
     21.2 |######
     21.8 |##########
     22.4 |####################
     23.0 |#######
     23.6 |###
     24.2 |#########
     24.8 |
     25.4 |##########
     26.0 |
     26.6 |###
     27.2 |
     27.8 |###
  (14 below, 2 above range)

minmax_branchless (n=160, range 16.1-25.2 ns)
     16.1 |####################
     16.5 |#######################
     17.0 |#################################
     17.4 |####################
     17.9 |#######################
     18.3 |####################################
     18.8 |#######################
     19.3 |####################################
     19.7 |####################################
     20.2 |########################################
     20.6 |#######################
     21.1 |####################################
     21.5 |####################################
     22.0 |##########
     22.4 |################
     22.9 |#############
     23.3 |################
     23.8 |######
     24.2 |######
     24.7 |###
  (9 below, 12 above range)

minmax_intrinsic (n=160, range 15.8-22.3 ns)
     15.8 |
     16.1 |##############
     16.5 |###########
     16.8 |
     17.1 |##############
     17.4 |#########################
     17.7 |##############################
     18.1 |#########################
     18.4 |
     18.7 |#######################
     19.0 |########################################
     19.4 |############################
     19.7 |
     20.0 |##############################
     20.3 |#########################
     20.6 |##############
     21.0 |##################
     21.3 |
     21.6 |##################
     21.9 |
  (15 below, 8 above range)

```

## Diagnostics

- **minmax_branch**: CV=44.5% (high variance, measurements may be unstable)
- **minmax_branch**: bridge=12.6% of algo (FFI overhead may distort results)
- **minmax_branchless**: bridge=12.5% of algo (FFI overhead may distort results)
- **minmax_intrinsic**: bridge=10.9% of algo (FFI overhead may distort results)
