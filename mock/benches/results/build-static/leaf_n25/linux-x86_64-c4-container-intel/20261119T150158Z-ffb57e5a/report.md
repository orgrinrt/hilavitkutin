# build-static: rebuilt after an edit to one leaf implementation crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (5874.19 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 3920.36 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 50% faster than the next best (llvm_non_incremental)

llvm_incremental (3920.36 ms) leads llvm_non_incremental (5874.19 ms) by 50%, a clear separation rather than a photo finish. CV 4.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 36% (significant)

llvm_incremental is -2104.38 ms (36%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### llvm_incremental shows alternating (throttle bounce) (autocorr -0.85)

llvm_incremental's per-pass series has lag-1 autocorrelation -0.85, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: llvm_incremental** at 3920360843.2 ns median (-33.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.50x (fastest 3920360843.2 ns, slowest 5874188211.0 ns)

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
| Counter frequency | 2100505716 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 3884787897ns | 3920364818ns | 3636187095ns | 3836384965ns | 4081692696ns | -34.81% |
| llvm_non_incremental | 5959459163ns | 5874195963ns | 5635898855ns | 5849202393ns | 6286624472ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 3884783612ns | 3636182812ns | 4081688452ns | -34.81% |
| llvm_non_incremental | 5959453783ns | 5635893988ns | 6286620524ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 3884787897ns | 3884787897ns | -34.81% |
| llvm_non_incremental | 5959459163ns | 5959459163ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 5874188211ns | base | --- | [5717552614, 6286620524] | --- | --- | --- | --- |
| llvm_incremental | 3920360843ns | -2104380666.2ns (-35.8%) | [-2483765683, -1635864163]ns | [3652301542, 4081688452] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 5803603496ns | -33.3% |
| 1 | 2 | 5799211241ns | -31.5% |
| 1 | 3 | 5944772926ns | -38.3% |
| 1 | 4 | 5635893988ns | -25.7% |
| 1 | 5 | 6198503192ns | -41.3% |
| 1 | 6 | 6374737856ns | -37.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.846 | HIGH- (thermal bounce) |
| llvm_non_incremental | 0.140 | ok |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 72800375.5ns | 3884783612.4ns | 1.9% |  |
| llvm_non_incremental | 66406735.8ns | 5959453783.1ns | 1.1% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 3636182811.7-4081688451.6 ns)
  3636182811.7 |########################################
  3658458093.7 |########################################
  3680733375.7 |
  3703008657.7 |
  3725283939.7 |
  3747559221.7 |
  3769834503.7 |
  3792109785.7 |
  3814385067.7 |
  3836660349.7 |
  3858935631.7 |########################################
  3881210913.6 |
  3903486195.6 |
  3925761477.6 |
  3948036759.6 |########################################
  3970312041.6 |########################################
  3992587323.6 |
  4014862605.6 |
  4037137887.6 |
  4059413169.6 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 5635893988.1-6286620523.9 ns)
  5635893988.1 |####################
  5668430314.9 |
  5700966641.7 |
  5733502968.5 |
  5766039295.2 |
  5798575622.0 |########################################
  5831111948.8 |
  5863648275.6 |
  5896184602.4 |
  5928720929.2 |####################
  5961257256.0 |
  5993793582.8 |
  6026329909.6 |
  6058866236.3 |
  6091402563.1 |
  6123938889.9 |
  6156475216.7 |
  6189011543.5 |####################
  6221547870.3 |
  6254084197.1 |
  (0 below, 1 above range)

```
