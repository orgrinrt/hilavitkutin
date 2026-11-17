# build-static: rebuilt after an edit to one root contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (12129.91 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 988.71 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 1127% faster than the next best (llvm_non_incremental)

llvm_incremental (988.71 ms) leads llvm_non_incremental (12129.91 ms) by 1127%, a clear separation rather than a photo finish. CV 9.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 92% (significant)

llvm_incremental is -11141.19 ms (92%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### llvm_incremental is fastest but the noisiest (CV 9.8%)

llvm_incremental wins on median (988.71 ms) yet has the highest variance (CV 9.8%), while llvm_non_incremental is the steadiest (CV 4.3%, 12129.91 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 12.3x the fastest

Fastest llvm_incremental (988.71 ms) to slowest llvm_non_incremental (12129.91 ms): 12.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 988713854.2 ns median (-91.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 12.27x (fastest 988713854.2 ns, slowest 12129908604.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | build-static: rebuilt after an edit to one root contract crate |
| Passes | 6 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6276626c73740001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 1035506722ns | 988716500ns | 956883708ns | 984612819ns | 1151159083ns | -91.67% |
| llvm_non_incremental | 12426675049ns | 12129993792ns | 12045890958ns | 12104804861ns | 13099872375ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 1035504125ns | 956881833ns | 1151155833ns | -91.67% |
| llvm_non_incremental | 12426640778ns | 12045886375ns | 13099862500ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 1035506722ns | 1035506722ns | -91.67% |
| llvm_non_incremental | 12426675049ns | 12426675049ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 12129908604ns | base | --- | [12050151229, 13099862500] | --- | --- | --- | --- |
| llvm_incremental | 988713854ns | -11141194749.9ns (-91.8%) | [-12089834958, -10942380250]ns | [966642688, 1151155833] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 12145992458ns | -91.8% |
| 1 | 2 | 12054416083ns | -89.7% |
| 1 | 3 | 12045886375ns | -91.9% |
| 1 | 4 | 12113824750ns | -91.9% |
| 1 | 5 | 13472740542ns | -92.1% |
| 1 | 6 | 12726984458ns | -92.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.344 | moderate- |
| llvm_non_incremental | 0.214 | moderate+ |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 85966479.2ns | 1035504125.0ns | 8.3% | HIGH |
| llvm_non_incremental | 72790798.6ns | 12426640777.8ns | 0.6% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 956881833.3-1151155833.3 ns)
  956881833.3 |####################
  966595533.3 |
  976309233.3 |########################################
  986022933.3 |
  995736633.3 |####################
  1005450333.3 |
  1015164033.3 |
  1024877733.3 |
  1034591433.3 |
  1044305133.3 |
  1054018833.3 |####################
  1063732533.3 |
  1073446233.3 |
  1083159933.3 |
  1092873633.3 |
  1102587333.3 |
  1112301033.3 |
  1122014733.3 |
  1131728433.3 |
  1141442133.3 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 12045886375.0-13099862500.0 ns)
  12045886375.0 |########################################
  12098585181.2 |########################################
  12151283987.5 |
  12203982793.8 |
  12256681600.0 |
  12309380406.2 |
  12362079212.5 |
  12414778018.8 |
  12467476825.0 |
  12520175631.2 |
  12572874437.5 |
  12625573243.8 |
  12678272050.0 |####################
  12730970856.2 |
  12783669662.5 |
  12836368468.8 |
  12889067275.0 |
  12941766081.2 |
  12994464887.5 |
  13047163693.8 |
  (0 below, 1 above range)

```
