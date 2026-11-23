# build-static: rebuilt after an edit to one mid contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (4718.05 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 389.77 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 1110% faster than the next best (llvm_non_incremental)

llvm_incremental (389.77 ms) leads llvm_non_incremental (4718.05 ms) by 1110%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 92% (significant)

llvm_incremental is -4333.16 ms (92%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 12.1x the fastest

Fastest llvm_incremental (389.77 ms) to slowest llvm_non_incremental (4718.05 ms): 12.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 389772575.1 ns median (-91.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 12.10x (fastest 389772575.1 ns, slowest 4718047824.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | build-static: rebuilt after an edit to one mid contract crate |
| Passes | 6 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6276626c73740001 |
| Counter frequency | 2112712847 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 389945674ns | 389774467ns | 379341966ns | 388724409ns | 397079425ns | -91.68% |
| llvm_non_incremental | 4689318740ns | 4718050258ns | 4503024066ns | 4706085417ns | 4757316061ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 389943481ns | 379339838ns | 397077514ns | -91.68% |
| llvm_non_incremental | 4689316384ns | 4503020576ns | 4757313998ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 389945674ns | 389945674ns | -91.68% |
| llvm_non_incremental | 4689318740ns | 4689318740ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 4718047824ns | base | --- | [4592587329, 4757313998] | --- | --- | --- | --- |
| llvm_incremental | 389772575ns | -4333160329.9ns (-91.8%) | [-4364938979, -4200019401]ns | [382980353, 397077514] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 4758994996ns | -91.6% |
| 1 | 2 | 4704854505ns | -91.7% |
| 1 | 3 | 4682154082ns | -91.5% |
| 1 | 4 | 4731241144ns | -92.0% |
| 1 | 5 | 4503020576ns | -91.4% |
| 1 | 6 | 4755633001ns | -91.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.201 | moderate- |
| llvm_non_incremental | -0.424 | moderate- |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 46469325.7ns | 389943480.8ns | 11.9% | HIGH |
| llvm_non_incremental | 45076520.1ns | 4689316384.0ns | 1.0% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 379339837.6-397077514.1 ns)
  379339837.6 |########################################
  380226721.4 |
  381113605.2 |
  382000489.1 |
  382887372.9 |
  383774256.7 |
  384661140.6 |
  385548024.4 |
  386434908.2 |########################################
  387321792.0 |
  388208675.9 |
  389095559.7 |########################################
  389982443.5 |########################################
  390869327.3 |
  391756211.2 |
  392643095.0 |
  393529978.8 |
  394416862.6 |
  395303746.5 |########################################
  396190630.3 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 4503020576.4-4757313998.4 ns)
  4503020576.4 |########################################
  4515735247.5 |
  4528449918.6 |
  4541164589.7 |
  4553879260.8 |
  4566593931.9 |
  4579308603.0 |
  4592023274.1 |
  4604737945.2 |
  4617452616.3 |
  4630167287.4 |
  4642881958.5 |
  4655596629.6 |
  4668311300.7 |
  4681025971.8 |########################################
  4693740642.9 |########################################
  4706455314.0 |
  4719169985.1 |########################################
  4731884656.2 |
  4744599327.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **llvm_incremental**: bridge=11.8% of algo (FFI overhead may distort results)
