# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (minmax_branch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline minmax_branch has the worst median (185 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest minmax_branchless at 182 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (151 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### minmax_branch is inconsistent: worst-20% is 3.7x its best-20%

minmax_branch's best 20% of batches run at 149 ns but its worst 20% at 554 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 1.4% of the fastest

All 3 variants sit between 182 ns and 185 ns - a 1.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: minmax_branchless** at 182.4 ns median (-1.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 182.4 ns, slowest 184.9 ns)

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
| Counter frequency | 2112833092 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| minmax_branch | 2810ns | 2069ns | 1684ns | 2070ns | 6154ns | base |
| minmax_branchless | 2773ns | 2068ns | 1731ns | 2065ns | 5940ns | -1.31% |
| minmax_intrinsic | 3061ns | 2058ns | 1689ns | 2447ns | 6278ns | +8.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 253ns | 149ns | 554ns | base | 16.207 |
| minmax_branchless | 243ns | 150ns | 522ns | -3.81% | 16.849 |
| minmax_intrinsic | 274ns | 152ns | 561ns | +8.60% | 14.923 |

## Performance model

- Peak throughput: **27.572 Gops/s** (minmax_branch; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 22.147 | 80.3% |
| minmax_branchless | 22.456 | 81.4% |
| minmax_intrinsic | 22.171 | 80.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2810ns | 2810ns | base |
| minmax_branchless | 2773ns | 2773ns | -1.31% |
| minmax_intrinsic | 3061ns | 3061ns | +8.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 185ns | base | --- | [183, 192] | --- | --- | --- | --- |
| minmax_branchless | 182ns | -4.2ns (-2.3%) | [-5, -3]ns | [180, 189] | YES | 0.0002 | 0.0001 | 0 |
| minmax_intrinsic | 185ns | no significant difference | [-27, +17]ns | [171, 185] | no | 0.5801 | 0.5801 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 193ns | -2.2% | -17.9% |
| 1 | 2 | 151ns | +12.5% | +206.6% |
| 1 | 3 | 152ns | -3.5% | +22.3% |
| 1 | 4 | 151ns | +19.5% | +22.4% |
| 1 | 5 | 184ns | -7.7% | -2.1% |
| 1 | 6 | 192ns | -1.4% | -18.0% |
| 1 | 7 | 183ns | -19.1% | -17.9% |
| 1 | 8 | 199ns | -4.8% | -18.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | 0.319 | moderate+ |
| minmax_branchless | -0.487 | moderate- |
| minmax_intrinsic | -0.128 | ok |

**Consistency summary:**

- **minmax_branchless**: won 6/8, lost 2/8
- **minmax_intrinsic**: won 5/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 32.7ns | 252.7ns | 12.9% | HIGH |
| minmax_branchless | 31.9ns | 243.1ns | 13.1% | HIGH |
| minmax_intrinsic | 35.4ns | 274.5ns | 12.9% | HIGH |

## Distribution (algo ns)

```
minmax_branch (n=160, range 148.6-553.8 ns)
    148.6 |###############################
    168.8 |####################################
    189.1 |########################################
    209.3 |
    229.6 |
    249.9 |
    270.1 |
    290.4 |
    310.6 |
    330.9 |
    351.2 |
    371.4 |
    391.7 |#
    411.9 |
    432.2 |####
    452.5 |#
    472.7 |###
    493.0 |
    513.2 |#
    533.5 |
  (9 below, 15 above range)

minmax_branchless (n=160, range 150.1-521.8 ns)
    150.1 |#
    168.7 |########################################
    187.2 |#####################################
    205.8 |
    224.4 |
    243.0 |
    261.6 |
    280.2 |
    298.7 |
    317.3 |
    335.9 |
    354.5 |
    373.1 |
    391.7 |
    410.3 |
    428.8 |#
    447.4 |
    466.0 |#
    484.6 |###
    503.2 |
  (27 below, 18 above range)

minmax_intrinsic (n=160, range 152.0-561.1 ns)
    152.0 |########################################
    172.5 |#########################
    192.9 |#
    213.4 |
    233.9 |
    254.3 |##
    274.8 |
    295.2 |#
    315.7 |#
    336.1 |#
    356.6 |
    377.0 |
    397.5 |
    417.9 |
    438.4 |##
    458.9 |####
    479.3 |#
    499.8 |####
    520.2 |###
    540.7 |######
  (20 below, 11 above range)

```

## Diagnostics

- **minmax_branch**: CV=63.0% (high variance, measurements may be unstable)
- **minmax_branch**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **minmax_branch**: bridge=12.9% of algo (FFI overhead may distort results)
- **minmax_branchless**: CV=62.3% (high variance, measurements may be unstable)
- **minmax_branchless**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **minmax_branchless**: bridge=13.1% of algo (FFI overhead may distort results)
- **minmax_intrinsic**: CV=59.9% (high variance, measurements may be unstable)
- **minmax_intrinsic**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **minmax_intrinsic**: bridge=13.0% of algo (FFI overhead may distort results)
