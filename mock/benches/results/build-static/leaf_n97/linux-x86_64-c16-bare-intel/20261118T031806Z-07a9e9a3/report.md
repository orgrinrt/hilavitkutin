# build-static: rebuilt after an edit to one leaf implementation crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (3920.12 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 2639.11 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 49% faster than the next best (llvm_non_incremental)

llvm_incremental (2639.11 ms) leads llvm_non_incremental (3920.12 ms) by 49%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 33% (significant)

llvm_incremental is -1289.93 ms (33%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: llvm_incremental** at 2639111156.2 ns median (-32.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.49x (fastest 2639111156.2 ns, slowest 3920121799.2 ns)

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
| llvm_incremental | 2651877532ns | 2639112904ns | 2618614992ns | 2632910362ns | 2696959557ns | -32.39% |
| llvm_non_incremental | 3922514035ns | 3920123670ns | 3894088465ns | 3913899555ns | 3949648539ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 2651875809ns | 2618613009ns | 2696957960ns | -32.39% |
| llvm_non_incremental | 3922511987ns | 3894086659ns | 3949646178ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 2651877532ns | 2651877532ns | -32.39% |
| llvm_non_incremental | 3922514035ns | 3922514035ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 3920121799ns | base | --- | [3897767983, 3949646178] | --- | --- | --- | --- |
| llvm_incremental | 2639111156ns | -1289930085.6ns (-32.9%) | [-1316359827, -1205618621]ns | [2619558311, 2696957960] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 3911066502ns | -30.5% |
| 1 | 2 | 3939543144ns | -33.5% |
| 1 | 3 | 3929177096ns | -33.4% |
| 1 | 4 | 3901449307ns | -32.5% |
| 1 | 5 | 3894086659ns | -31.3% |
| 1 | 6 | 3959749213ns | -33.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.133 | ok |
| llvm_non_incremental | -0.220 | moderate- |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 202576146.0ns | 2651875808.9ns | 7.6% | HIGH |
| llvm_non_incremental | 197548151.5ns | 3922511986.8ns | 5.0% | HIGH |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 2618613008.9-2696957959.7 ns)
  2618613008.9 |########################################
  2622530256.4 |
  2626447504.0 |
  2630364751.5 |####################
  2634281999.1 |
  2638199246.6 |
  2642116494.1 |
  2646033741.7 |####################
  2649950989.2 |
  2653868236.7 |
  2657785484.3 |
  2661702731.8 |
  2665619979.3 |
  2669537226.9 |
  2673454474.4 |####################
  2677371722.0 |
  2681288969.5 |
  2685206217.0 |
  2689123464.6 |
  2693040712.1 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 3894086658.9-3949646178.2 ns)
  3894086658.9 |########################################
  3896864634.9 |
  3899642610.8 |########################################
  3902420586.8 |
  3905198562.8 |
  3907976538.7 |
  3910754514.7 |########################################
  3913532490.7 |
  3916310466.6 |
  3919088442.6 |
  3921866418.6 |
  3924644394.5 |
  3927422370.5 |########################################
  3930200346.5 |
  3932978322.4 |
  3935756298.4 |
  3938534274.4 |########################################
  3941312250.3 |
  3944090226.3 |
  3946868202.3 |
  (0 below, 1 above range)

```
