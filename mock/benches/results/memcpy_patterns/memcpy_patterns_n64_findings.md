# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (memcpy_intrinsic) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline memcpy_intrinsic has the worst median (40 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest memcpy_unrolled at 39 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 2.0% (fastest 39 ns to slowest 40 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (5 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### memcpy_intrinsic is inconsistent: worst-20% is 1.5x its best-20%

memcpy_intrinsic's best 20% of batches run at 34 ns but its worst 20% at 50 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: memcpy_unrolled** at 39.2 ns median (-2.0% vs baseline)
- Spread: 1.02x (fastest 39.2 ns, slowest 40.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4646464650504040 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 2402ns | 2447ns | 2269ns | 2418ns | 2487ns | base |
| memcpy_naive | 2355ns | 2438ns | 2150ns | 2386ns | 2467ns | -1.98% |
| memcpy_unrolled | 2326ns | 2279ns | 2140ns | 2341ns | 2466ns | -3.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 41ns | 34ns | 50ns | base | 1.561 |
| memcpy_naive | 39ns | 32ns | 47ns | -3.64% | 1.620 |
| memcpy_unrolled | 39ns | 32ns | 47ns | -4.48% | 1.635 |

## Performance model

- Peak throughput: **2.024 Gops/s** (memcpy_unrolled; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 1.600 | 79.1% |
| memcpy_naive | 1.616 | 79.9% |
| memcpy_unrolled | 1.633 | 80.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 2402ns | 2402ns | base |
| memcpy_naive | 2355ns | 2355ns | -1.98% |
| memcpy_unrolled | 2326ns | 2326ns | -3.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 40ns | base | --- | [39, 41] | --- | --- | --- | --- |
| memcpy_naive | 40ns | no significant difference | [-1, +0]ns | [39, 40] | no | 0.3723 | 0.3723 | 8 |
| memcpy_unrolled | 39ns | no significant difference | [-2, +0]ns | [38, 40] | no (adj: YES) | 0.0287 | 0.0144 | 9 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 41ns | -12.3% | -3.0% |
| 1 | 2 | 40ns | -2.0% | +0.0% |
| 1 | 3 | 36ns | +2.9% | +6.9% |
| 1 | 4 | 39ns | -1.0% | -3.3% |
| 1 | 5 | 40ns | +1.5% | -5.8% |
| 1 | 6 | 41ns | -1.5% | +3.2% |
| 1 | 7 | 42ns | -0.6% | -2.1% |
| 1 | 8 | 42ns | -1.6% | -10.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | 0.276 | moderate+ |
| memcpy_naive | 0.205 | moderate+ |
| memcpy_unrolled | -0.047 | ok |

**Consistency summary:**

- **memcpy_naive**: won 6/8, lost 2/8
- **memcpy_unrolled**: won 5/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 4.5ns | 41.0ns | 10.9% | HIGH |
| memcpy_naive | 3.9ns | 39.5ns | 10.0% | HIGH |
| memcpy_unrolled | 3.9ns | 39.2ns | 9.9% | HIGH |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 33.5-50.4 ns)
     33.5 |########
     34.4 |########
     35.2 |######################
     36.1 |####################
     36.9 |####################
     37.8 |########################################
     38.6 |########################################
     39.4 |#########################
     40.3 |#########################
     41.1 |#########################
     42.0 |####################
     42.8 |##################################
     43.7 |###############################
     44.5 |######################
     45.4 |######################
     46.2 |########
     47.1 |#####
     47.9 |
     48.8 |
     49.6 |#################
  (16 below, 4 above range)

memcpy_naive (n=160, range 31.8-47.0 ns)
     31.8 |########
     32.5 |########
     33.3 |########
     34.0 |#########################
     34.8 |####################
     35.6 |##############
     36.3 |#################
     37.1 |####################
     37.8 |###############################
     38.6 |############################
     39.4 |##################################
     40.1 |############################
     40.9 |#################
     41.7 |#################
     42.4 |########
     43.2 |##############
     43.9 |#################
     44.7 |########################################
     45.5 |##############
     46.2 |##
  (14 below, 14 above range)

memcpy_unrolled (n=160, range 31.6-46.6 ns)
     31.6 |##########
     32.4 |############
     33.1 |##########
     33.9 |####################
     34.6 |##
     35.4 |######################
     36.1 |###############
     36.9 |###########################
     37.6 |#####################################
     38.4 |#######
     39.1 |#################
     39.9 |######################
     40.6 |########################################
     41.4 |####################
     42.1 |##########
     42.9 |###############
     43.6 |#######
     44.4 |####################
     45.1 |#################
     45.8 |#####
  (12 below, 12 above range)

```

## Diagnostics

- **memcpy_intrinsic**: CV=24.6% (high variance, measurements may be unstable)
