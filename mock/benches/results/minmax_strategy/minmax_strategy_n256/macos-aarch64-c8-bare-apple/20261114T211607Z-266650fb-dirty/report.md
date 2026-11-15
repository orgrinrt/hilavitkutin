# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (minmax_branch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline minmax_branch has the worst median (24 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest minmax_intrinsic at 22 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### minmax_intrinsic is fastest but the noisiest (CV 14.3%)

minmax_intrinsic wins on median (22 ns) yet has the highest variance (CV 14.3%), while minmax_branch is the steadiest (CV 12.4%, 24 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### minmax_intrinsic shows alternating (throttle bounce) (autocorr -0.62)

minmax_intrinsic's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (3 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader minmax_intrinsic vs stability leader minmax_branch (+8% speed for 1.1x steadier)

minmax_intrinsic is fastest (22 ns, CV 14.3%); minmax_branch gives up 7.6% median for 1.1x lower variance (CV 12.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### minmax_intrinsic is inconsistent: worst-20% is 1.5x its best-20%

minmax_intrinsic's best 20% of batches run at 18 ns but its worst 20% at 27 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: minmax_intrinsic** at 22.5 ns median (-7.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.08x (fastest 22.5 ns, slowest 24.2 ns)

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
| minmax_branch | 2251ns | 2249ns | 2130ns | 2246ns | 2386ns | base |
| minmax_branchless | 2210ns | 2244ns | 2094ns | 2203ns | 2344ns | -1.82% |
| minmax_intrinsic | 2194ns | 2177ns | 2090ns | 2181ns | 2340ns | -2.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 24ns | 20ns | 28ns | base | 10.633 |
| minmax_branchless | 23ns | 19ns | 27ns | -3.30% | 10.996 |
| minmax_intrinsic | 22ns | 18ns | 27ns | -7.53% | 11.499 |

## Performance model

- Peak throughput: **14.535 Gops/s** (minmax_intrinsic; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 10.579 | 72.8% |
| minmax_branchless | 10.802 | 74.3% |
| minmax_intrinsic | 11.378 | 78.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2251ns | 2251ns | base |
| minmax_branchless | 2210ns | 2210ns | -1.82% |
| minmax_intrinsic | 2194ns | 2194ns | -2.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 24ns | base | --- | [24, 25] | --- | --- | --- | --- |
| minmax_branchless | 24ns | no significant difference | [-1, +0]ns | [23, 24] | no | 0.2236 | 0.2236 | 8 |
| minmax_intrinsic | 22ns | -2.5ns (-10.3%) | [-3, -2]ns | [22, 23] | YES | 0.0000 | 0.0000 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 23ns | +2.6% | +0.0% |
| 1 | 2 | 24ns | +4.4% | -16.5% |
| 1 | 3 | 24ns | -1.8% | -1.0% |
| 1 | 4 | 24ns | -6.9% | -24.4% |
| 1 | 5 | 25ns | -1.6% | -10.8% |
| 1 | 6 | 28ns | -30.2% | -15.3% |
| 1 | 7 | 25ns | -9.1% | -23.0% |
| 1 | 8 | 20ns | +19.6% | +25.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | 0.074 | ok |
| minmax_branchless | -0.200 | ok |
| minmax_intrinsic | -0.621 | HIGH- (thermal bounce) |

**Consistency summary:**

- **minmax_branchless**: won 5/8, lost 3/8
- **minmax_intrinsic**: won 6/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 3.3ns | 24.1ns | 13.9% | HIGH |
| minmax_branchless | 3.3ns | 23.3ns | 14.3% | HIGH |
| minmax_intrinsic | 3.1ns | 22.3ns | 13.8% | HIGH |

## Distribution (algo ns)

```
minmax_branch (n=160, range 19.9-28.5 ns)
     19.9 |#####
     20.3 |##########
     20.7 |##########
     21.2 |#############
     21.6 |######
     22.0 |###
     22.5 |#####
     22.9 |########################################
     23.3 |###############
     23.7 |
     24.2 |#################################
     24.6 |#######################
     25.0 |########
     25.5 |###############
     25.9 |######
     26.3 |###
     26.7 |##########
     27.2 |######
     27.6 |#####
     28.0 |#####
  (12 below, 13 above range)

minmax_branchless (n=160, range 19.0-27.2 ns)
     19.0 |######
     19.4 |########
     19.9 |####
     20.3 |########
     20.7 |############
     21.1 |##############
     21.5 |##########
     21.9 |######################
     22.3 |####################
     22.7 |################
     23.1 |################
     23.5 |########################################
     23.9 |##########
     24.3 |##########################
     24.8 |######
     25.2 |##################
     25.6 |################
     26.0 |########
     26.4 |######
     26.8 |############
  (11 below, 10 above range)

minmax_intrinsic (n=160, range 17.6-26.6 ns)
     17.6 |##############################
     18.1 |###############
     18.5 |#######
     19.0 |#####
     19.4 |############
     19.9 |####################
     20.3 |##########
     20.8 |###################################
     21.2 |
     21.7 |########################################
     22.1 |##########
     22.6 |###########################
     23.0 |####################
     23.5 |####################
     23.9 |############
     24.4 |############
     24.8 |#########################
     25.3 |###############
     25.7 |###############
     26.2 |##########
  (9 below, 14 above range)

```

## Diagnostics

- **minmax_branch**: bridge=13.6% of algo (FFI overhead may distort results)
- **minmax_branchless**: bridge=13.9% of algo (FFI overhead may distort results)
- **minmax_intrinsic**: bridge=12.9% of algo (FFI overhead may distort results)
