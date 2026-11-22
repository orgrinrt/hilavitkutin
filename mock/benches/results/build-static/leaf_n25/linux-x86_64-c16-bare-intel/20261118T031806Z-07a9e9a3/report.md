# build-static: rebuilt after an edit to one leaf implementation crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (3669.55 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 2372.08 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 55% faster than the next best (llvm_non_incremental)

llvm_incremental (2372.08 ms) leads llvm_non_incremental (3669.55 ms) by 55%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 35% (significant)

llvm_incremental is -1289.06 ms (35%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: llvm_incremental** at 2372078229.9 ns median (-35.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.55x (fastest 2372078229.9 ns, slowest 3669554880.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | build-static: rebuilt after an edit to one leaf implementation crate |
| Passes | 6 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6276626c73740001 |
| Counter frequency | 2112687708 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 2372015530ns | 2372080568ns | 2355714077ns | 2368147458ns | 2385968363ns | -35.46% |
| llvm_non_incremental | 3675408147ns | 3669561409ns | 3616929685ns | 3665200554ns | 3719958768ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 2372013366ns | 2355711492ns | 2385966560ns | -35.46% |
| llvm_non_incremental | 3675404460ns | 3616927596ns | 3719956531ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 2372015530ns | 2372015530ns | -35.46% |
| llvm_non_incremental | 3675408147ns | 3675408147ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 3669554880ns | base | --- | [3636701969, 3719956531] | --- | --- | --- | --- |
| llvm_incremental | 2372078230ns | -1289058920.2ns (-35.1%) | [-1354947130, -1266167232]ns | [2357995307, 2385966560] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 3739947038ns | -36.9% |
| 1 | 2 | 3665311431ns | -34.7% |
| 1 | 3 | 3673798330ns | -35.3% |
| 1 | 4 | 3656476342ns | -35.1% |
| 1 | 5 | 3616927596ns | -34.9% |
| 1 | 6 | 3699966024ns | -36.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.129 | ok |
| llvm_non_incremental | -0.108 | ok |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 45716338.2ns | 2372013365.8ns | 1.9% |  |
| llvm_non_incremental | 45215699.9ns | 3675404460.1ns | 1.2% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 2355711491.8-2385966560.4 ns)
  2355711491.8 |########################################
  2357224245.2 |
  2358736998.7 |
  2360249752.1 |########################################
  2361762505.5 |
  2363275259.0 |
  2364788012.4 |
  2366300765.8 |
  2367813519.2 |
  2369326272.7 |########################################
  2370839026.1 |
  2372351779.5 |
  2373864533.0 |########################################
  2375377286.4 |
  2376890039.8 |########################################
  2378402793.2 |
  2379915546.7 |
  2381428300.1 |
  2382941053.5 |
  2384453807.0 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 3616927595.6-3719956531.2 ns)
  3616927595.6 |########################################
  3622079042.4 |
  3627230489.2 |
  3632381935.9 |
  3637533382.7 |
  3642684829.5 |
  3647836276.3 |
  3652987723.0 |########################################
  3658139169.8 |
  3663290616.6 |########################################
  3668442063.4 |
  3673593510.2 |########################################
  3678744956.9 |
  3683896403.7 |
  3689047850.5 |
  3694199297.3 |
  3699350744.0 |########################################
  3704502190.8 |
  3709653637.6 |
  3714805084.4 |
  (0 below, 1 above range)

```
