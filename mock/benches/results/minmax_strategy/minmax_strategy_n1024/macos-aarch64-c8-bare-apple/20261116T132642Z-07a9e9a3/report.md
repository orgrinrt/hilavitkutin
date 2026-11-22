# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (minmax_branch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline minmax_branch has the worst median (41 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest minmax_branchless at 40 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.5% (fastest 40 ns to slowest 41 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### minmax_branchless is fastest but the noisiest (CV 10.7%)

minmax_branchless wins on median (40 ns) yet has the highest variance (CV 10.7%), while minmax_intrinsic is the steadiest (CV 10.0%, 40 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader minmax_branchless vs stability leader minmax_intrinsic (+0% speed for 1.1x steadier)

minmax_branchless is fastest (40 ns, CV 10.7%); minmax_intrinsic gives up 0.0% median for 1.1x lower variance (CV 10.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: minmax_branchless** at 40.4 ns median (-0.5% vs baseline)
- Spread: 1.00x (fastest 40.4 ns, slowest 40.6 ns)

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
| minmax_branch | 2128ns | 2112ns | 2107ns | 2113ns | 2196ns | base |
| minmax_branchless | 2122ns | 2115ns | 2109ns | 2117ns | 2150ns | -0.29% |
| minmax_intrinsic | 2117ns | 2112ns | 2106ns | 2112ns | 2142ns | -0.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 41ns | 35ns | 47ns | base | 25.098 |
| minmax_branchless | 41ns | 35ns | 47ns | -0.48% | 25.219 |
| minmax_intrinsic | 40ns | 35ns | 46ns | -0.85% | 25.313 |

## Performance model

- Peak throughput: **29.404 Gops/s** (minmax_intrinsic; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 25.222 | 85.8% |
| minmax_branchless | 25.347 | 86.2% |
| minmax_intrinsic | 25.347 | 86.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2128ns | 2128ns | base |
| minmax_branchless | 2122ns | 2122ns | -0.29% |
| minmax_intrinsic | 2117ns | 2117ns | -0.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 41ns | base | --- | [40, 41] | --- | --- | --- | --- |
| minmax_branchless | 40ns | no significant difference | [-1, +0]ns | [39, 41] | no | 0.4655 | 0.4655 | 8 |
| minmax_intrinsic | 40ns | no significant difference | [-1, +0]ns | [40, 41] | no | 0.3410 | 0.1705 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 41ns | -0.5% | +0.5% |
| 1 | 2 | 40ns | +3.7% | -0.5% |
| 1 | 3 | 41ns | -2.5% | +2.1% |
| 1 | 4 | 41ns | -1.0% | -1.0% |
| 1 | 5 | 41ns | -2.0% | -2.4% |
| 1 | 6 | 40ns | -1.5% | +2.1% |
| 1 | 7 | 41ns | +0.0% | -3.4% |
| 1 | 8 | 40ns | +2.0% | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | -0.297 | moderate- |
| minmax_branchless | -0.241 | moderate- |
| minmax_intrinsic | -0.360 | moderate- |

**Consistency summary:**

- **minmax_branchless**: won 5/8, lost 2/8
- **minmax_intrinsic**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 2.4ns | 40.8ns | 5.9% | HIGH |
| minmax_branchless | 2.3ns | 40.6ns | 5.7% | HIGH |
| minmax_intrinsic | 2.2ns | 40.5ns | 5.5% | HIGH |

## Distribution (algo ns)

```
minmax_branch (n=160, range 35.2-47.0 ns)
     35.2 |##########
     35.8 |#########################
     36.4 |#######
     37.0 |######################
     37.5 |##########
     38.1 |###############
     38.7 |###############
     39.3 |#################
     39.9 |########################################
     40.5 |######################
     41.1 |#################
     41.7 |###########################
     42.3 |#################
     42.9 |####################
     43.5 |#######
     44.1 |#####
     44.7 |#################
     45.3 |####################
     45.8 |#####
     46.4 |
  (15 below, 16 above range)

minmax_branchless (n=160, range 35.0-47.2 ns)
     35.0 |##############
     35.6 |#####
     36.2 |##################################
     36.8 |#####
     37.4 |########################################
     38.0 |######################
     38.6 |#####################################
     39.2 |########
     39.8 |#####################################
     40.4 |######################
     41.1 |###########
     41.7 |########################################
     42.3 |#################
     42.9 |##############
     43.5 |########
     44.1 |####################
     44.7 |##
     45.3 |###########
     45.9 |###########
     46.6 |######################
  (13 below, 11 above range)

minmax_intrinsic (n=160, range 34.8-46.4 ns)
     34.8 |##########
     35.4 |##########
     36.0 |########
     36.6 |################
     37.1 |######
     37.7 |########
     38.3 |#####################
     38.9 |################
     39.4 |#############################
     40.0 |############
     40.6 |##################
     41.2 |########################################
     41.8 |################
     42.3 |############
     42.9 |######
     43.5 |##########
     44.1 |##########
     44.6 |######
     45.2 |##########
     45.8 |######
  (14 below, 13 above range)

```
