# build-static: rebuilt after an edit to one leaf implementation crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (5697.29 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 3809.88 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 50% faster than the next best (llvm_non_incremental)

llvm_incremental (3809.88 ms) leads llvm_non_incremental (5697.29 ms) by 50%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 33% (significant)

llvm_incremental is -1891.51 ms (33%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: llvm_incremental** at 3809881221.7 ns median (-33.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.50x (fastest 3809881221.7 ns, slowest 5697293524.4 ns)

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
| Counter frequency | 2100332830 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 3797248701ns | 3809885157ns | 3650158239ns | 3773661736ns | 3906174378ns | -33.12% |
| llvm_non_incremental | 5677493454ns | 5697308282ns | 5251435064ns | 5653754155ns | 5926131598ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 3797243848ns | 3650153985ns | 3906170438ns | -33.12% |
| llvm_non_incremental | 5677485850ns | 5251430708ns | 5926127555ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 3797248701ns | 3797248701ns | -33.12% |
| llvm_non_incremental | 5677493454ns | 5677493454ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 5697293524ns | base | --- | [5409036471, 5926127555] | --- | --- | --- | --- |
| llvm_incremental | 3809881222ns | -1891513759.8ns (-33.2%) | [-2240096108, -1509116138]ns | [3675679886, 3906170438] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 5618216991ns | -30.4% |
| 1 | 2 | 6012790922ns | -39.3% |
| 1 | 3 | 5839464189ns | -36.3% |
| 1 | 4 | 5251430708ns | -25.7% |
| 1 | 5 | 5566642235ns | -30.0% |
| 1 | 6 | 5776370058ns | -35.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.179 | ok |
| llvm_non_incremental | 0.005 | ok |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 74909875.6ns | 3797243848.3ns | 2.0% |  |
| llvm_non_incremental | 61016906.9ns | 5677485850.4ns | 1.1% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 3650153985.1-3906170437.7 ns)
  3650153985.1 |####################
  3662954807.7 |
  3675755630.4 |
  3688556453.0 |####################
  3701357275.6 |
  3714158098.2 |####################
  3726958920.9 |
  3739759743.5 |
  3752560566.1 |
  3765361388.8 |
  3778162211.4 |
  3790963034.0 |
  3803763856.7 |
  3816564679.3 |
  3829365501.9 |
  3842166324.5 |
  3854967147.2 |
  3867767969.8 |
  3880568792.4 |
  3893369615.1 |########################################
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 5251430707.7-5926127555.5 ns)
  5251430707.7 |########################################
  5285165550.1 |
  5318900392.5 |
  5352635234.9 |
  5386370077.2 |
  5420104919.6 |
  5453839762.0 |
  5487574604.4 |
  5521309446.8 |
  5555044289.2 |########################################
  5588779131.6 |########################################
  5622513974.0 |
  5656248816.4 |
  5689983658.7 |
  5723718501.1 |
  5757453343.5 |########################################
  5791188185.9 |
  5824923028.3 |########################################
  5858657870.7 |
  5892392713.1 |
  (0 below, 1 above range)

```
