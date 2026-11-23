# build-static: rebuilt after an edit to one root contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (50602.21 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 3877.48 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 1205% faster than the next best (llvm_non_incremental)

llvm_incremental (3877.48 ms) leads llvm_non_incremental (50602.21 ms) by 1205%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 92% (significant)

llvm_incremental is -46782.45 ms (92%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 13.1x the fastest

Fastest llvm_incremental (3877.48 ms) to slowest llvm_non_incremental (50602.21 ms): 13.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 3877478999.6 ns median (-92.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 13.05x (fastest 3877478999.6 ns, slowest 50602213632.9 ns)

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
| Counter frequency | 2112699331 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 3869861655ns | 3877480670ns | 3765913279ns | 3874473008ns | 3914918815ns | -92.36% |
| llvm_non_incremental | 50624780145ns | 50602215573ns | 50456703104ns | 50577066103ns | 50780389729ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 3869859435ns | 3765911007ns | 3914916171ns | -92.36% |
| llvm_non_incremental | 50624778201ns | 50456701551ns | 50780387489ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 3869861655ns | 3869861655ns | -92.36% |
| llvm_non_incremental | 50624780145ns | 50624780145ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 50602213633ns | base | --- | [50491733480, 50780387489] | --- | --- | --- | --- |
| llvm_incremental | 3877479000ns | -46782452961.3ns (-92.5%) | [-46888555465, -46593747871]ns | [3817183134, 3914916171] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 50778086586ns | -92.3% |
| 1 | 2 | 50526765410ns | -92.3% |
| 1 | 3 | 50456701551ns | -92.2% |
| 1 | 4 | 50544835556ns | -92.5% |
| 1 | 5 | 50782688392ns | -92.4% |
| 1 | 6 | 50659591710ns | -92.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.474 | moderate- |
| llvm_non_incremental | 0.083 | ok |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 263519369.0ns | 3869859434.9ns | 6.8% | HIGH |
| llvm_non_incremental | 233158818.9ns | 50624778200.8ns | 0.5% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 3765911006.8-3914916171.1 ns)
  3765911006.8 |########################################
  3773361265.0 |
  3780811523.2 |
  3788261781.5 |
  3795712039.7 |
  3803162297.9 |
  3810612556.1 |
  3818062814.3 |
  3825513072.5 |
  3832963330.8 |
  3840413589.0 |
  3847863847.2 |
  3855314105.4 |
  3862764363.6 |########################################
  3870214621.8 |########################################
  3877664880.1 |########################################
  3885115138.3 |
  3892565396.5 |
  3900015654.7 |########################################
  3907465912.9 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 50456701550.8-50780387489.0 ns)
  50456701550.8 |########################################
  50472885847.7 |
  50489070144.6 |
  50505254441.5 |
  50521438738.4 |########################################
  50537623035.4 |########################################
  50553807332.3 |
  50569991629.2 |
  50586175926.1 |
  50602360223.0 |
  50618544519.9 |
  50634728816.8 |
  50650913113.7 |########################################
  50667097410.6 |
  50683281707.5 |
  50699466004.4 |
  50715650301.4 |
  50731834598.3 |
  50748018895.2 |
  50764203192.1 |########################################
  (0 below, 1 above range)

```
