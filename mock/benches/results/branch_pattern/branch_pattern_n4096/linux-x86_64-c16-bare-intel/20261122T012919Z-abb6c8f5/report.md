# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (6.78 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 3.41 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### branch_if dominates: 99% faster than the next best (branch_cmov)

branch_if (3.41 us) leads branch_cmov (6.78 us) by 99%, a clear separation rather than a photo finish. CV 8.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### branch_if beats baseline by 51% (significant)

branch_if is -3.43 us (51%) faster than baseline branch_cmov, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: branch_if** at 3409.1 ns median (-49.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.99x (fastest 3409.1 ns, slowest 6778.9 ns)

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
| Counter frequency | 2112827530 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| branch_cmov | 8127ns | 8669ns | 7101ns | 8245ns | 8798ns | base |
| branch_if | 4964ns | 5305ns | 4342ns | 5043ns | 5349ns | -38.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 6363ns | 5557ns | 6885ns | base | 0.644 |
| branch_if | 3194ns | 2795ns | 3443ns | -49.81% | 1.283 |

## Performance model

- Peak throughput: **1.466 Gops/s** (branch_if; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.604 | 41.2% |
| branch_if | 1.201 | 82.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 8127ns | 8127ns | base |
| branch_if | 4964ns | 4964ns | -38.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 6779ns | base | --- | [6744, 6819] | --- | --- | --- | --- |
| branch_if | 3409ns | -3429.8ns (-50.6%) | [-3463, -3414]ns | [3390, 3415] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 6834ns | -50.4% |
| 1 | 2 | 5587ns | -38.8% |
| 1 | 3 | 6836ns | -59.1% |
| 1 | 4 | 6841ns | -50.1% |
| 1 | 5 | 6838ns | -59.0% |
| 1 | 6 | 5603ns | -38.5% |
| 1 | 7 | 5589ns | -38.8% |
| 1 | 8 | 6774ns | -58.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.130 | ok |
| branch_if | -0.484 | moderate- |

**Consistency summary:**

- **branch_if**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 22.3ns | 6363.3ns | 0.4% |  |
| branch_if | 22.9ns | 3193.6ns | 0.7% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 5556.7-6885.1 ns)
   5556.7 |##############################
   5623.1 |####
   5689.5 |
   5755.9 |
   5822.4 |
   5888.8 |
   5955.2 |
   6021.6 |
   6088.0 |
   6154.5 |
   6220.9 |
   6287.3 |
   6353.7 |
   6420.1 |
   6486.6 |
   6553.0 |
   6619.4 |
   6685.8 |######
   6752.2 |################
   6818.7 |########################################
  (11 below, 11 above range)

branch_if (n=160, range 2794.8-3443.5 ns)
   2794.8 |########################
   2827.2 |
   2859.6 |
   2892.1 |
   2924.5 |
   2956.9 |
   2989.4 |
   3021.8 |
   3054.3 |
   3086.7 |
   3119.1 |
   3151.6 |
   3184.0 |
   3216.4 |
   3248.9 |
   3281.3 |
   3313.8 |
   3346.2 |
   3378.6 |###########
   3411.1 |########################################
  (13 below, 9 above range)

```
