# build-static: rebuilt after an edit to one root contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (16745.89 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 1247.25 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 1243% faster than the next best (llvm_non_incremental)

llvm_incremental (1247.25 ms) leads llvm_non_incremental (16745.89 ms) by 1243%, a clear separation rather than a photo finish. CV 6.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 93% (significant)

llvm_incremental is -15490.28 ms (93%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### llvm_incremental is fastest but the noisiest (CV 6.3%)

llvm_incremental wins on median (1247.25 ms) yet has the highest variance (CV 6.3%), while llvm_non_incremental is the steadiest (CV 3.1%, 16745.89 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 13.4x the fastest

Fastest llvm_incremental (1247.25 ms) to slowest llvm_non_incremental (16745.89 ms): 13.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 1247253978.2 ns median (-92.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 13.43x (fastest 1247253978.2 ns, slowest 16745887707.6 ns)

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
| Counter frequency | 2100505716 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 1256551986ns | 1247257741ns | 1161599764ns | 1222222466ns | 1355522376ns | -92.49% |
| llvm_non_incremental | 16742554111ns | 16745891553ns | 15918402190ns | 16635385794ns | 17315382546ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 1256548110ns | 1161595754ns | 1355518602ns | -92.49% |
| llvm_non_incremental | 16742550103ns | 15918397866ns | 17315378630ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 1256551986ns | 1256551986ns | -92.49% |
| llvm_non_incremental | 16742554111ns | 16742554111ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 16745887708ns | base | --- | [16166383970, 17315378630] | --- | --- | --- | --- |
| llvm_incremental | 1247253978ns | -15490276986.7ns (-92.5%) | [-15998292090, -14969436903]ns | [1166871749, 1355518602] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 16826016658ns | -93.1% |
| 1 | 2 | 16665758757ns | -91.9% |
| 1 | 3 | 15918397866ns | -92.6% |
| 1 | 4 | 16414370074ns | -92.6% |
| 1 | 5 | 17566394673ns | -92.2% |
| 1 | 6 | 17064362588ns | -92.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.421 | moderate- |
| llvm_non_incremental | 0.204 | moderate+ |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 68975216.0ns | 1256548109.6ns | 5.5% | HIGH |
| llvm_non_incremental | 61801151.0ns | 16742550102.7ns | 0.4% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 1161595753.7-1355518601.6 ns)
  1161595753.7 |########################################
  1171291896.1 |########################################
  1180988038.5 |
  1190684180.9 |
  1200380323.3 |
  1210076465.7 |
  1219772608.1 |########################################
  1229468750.5 |
  1239164892.9 |
  1248861035.3 |
  1258557177.7 |
  1268253320.0 |########################################
  1277949462.4 |
  1287645604.8 |
  1297341747.2 |
  1307037889.6 |
  1316734032.0 |
  1326430174.4 |
  1336126316.8 |
  1345822459.2 |########################################
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 15918397866.2-17315378630.5 ns)
  15918397866.2 |########################################
  15988246904.4 |
  16058095942.6 |
  16127944980.8 |
  16197794019.1 |
  16267643057.3 |
  16337492095.5 |
  16407341133.7 |########################################
  16477190171.9 |
  16547039210.1 |
  16616888248.4 |########################################
  16686737286.6 |
  16756586324.8 |########################################
  16826435363.0 |
  16896284401.2 |
  16966133439.4 |
  17035982477.6 |########################################
  17105831515.9 |
  17175680554.1 |
  17245529592.3 |
  (0 below, 1 above range)

```
