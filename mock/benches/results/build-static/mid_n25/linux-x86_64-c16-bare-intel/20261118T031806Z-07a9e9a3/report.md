# build-static: rebuilt after an edit to one mid contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (4863.22 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 376.53 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 1192% faster than the next best (llvm_non_incremental)

llvm_incremental (376.53 ms) leads llvm_non_incremental (4863.22 ms) by 1192%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 92% (significant)

llvm_incremental is -4486.14 ms (92%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 12.9x the fastest

Fastest llvm_incremental (376.53 ms) to slowest llvm_non_incremental (4863.22 ms): 12.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 376525074.8 ns median (-92.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 12.92x (fastest 376525074.8 ns, slowest 4863222900.2 ns)

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
| Counter frequency | 2112687708 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 379371678ns | 376526960ns | 373102662ns | 375476449ns | 388349029ns | -92.12% |
| llvm_non_incremental | 4813617356ns | 4863225371ns | 4532565581ns | 4846892714ns | 4904230207ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 379369415ns | 373099693ns | 388347227ns | -92.12% |
| llvm_non_incremental | 4813614963ns | 4532563484ns | 4904227538ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 379371678ns | 379371678ns | -92.12% |
| llvm_non_incremental | 4813617356ns | 4813617356ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 4863222900ns | base | --- | [4673394451, 4904227538] | --- | --- | --- | --- |
| llvm_incremental | 376525075ns | -4486137304.0ns (-92.2%) | [-4517979036, -4298620305]ns | [373235943, 388347227] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 4842036396ns | -92.2% |
| 1 | 2 | 4884409404ns | -92.4% |
| 1 | 3 | 4814225418ns | -92.2% |
| 1 | 4 | 4911097974ns | -92.1% |
| 1 | 5 | 4532563484ns | -91.8% |
| 1 | 6 | 4897357102ns | -92.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.364 | moderate- |
| llvm_non_incremental | -0.482 | moderate- |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 46663893.4ns | 379369414.8ns | 12.3% | HIGH |
| llvm_non_incremental | 41984176.9ns | 4813614963.0ns | 0.9% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 373099693.2-388347227.1 ns)
  373099693.2 |########################################
  373862069.9 |
  374624446.6 |
  375386823.3 |
  376149200.0 |########################################
  376911576.7 |
  377673953.4 |
  378436330.1 |
  379198706.8 |
  379961083.5 |
  380723460.2 |
  381485836.9 |
  382248213.6 |
  383010590.3 |
  383772967.0 |
  384535343.7 |
  385297720.4 |
  386060097.1 |####################
  386822473.8 |
  387584850.5 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 4532563484.0-4904227538.1 ns)
  4532563484.0 |########################################
  4551146686.7 |
  4569729889.4 |
  4588313092.1 |
  4606896294.8 |
  4625479497.5 |
  4644062700.2 |
  4662645902.9 |
  4681229105.6 |
  4699812308.3 |
  4718395511.0 |
  4736978713.7 |
  4755561916.4 |
  4774145119.1 |
  4792728321.8 |
  4811311524.5 |########################################
  4829894727.2 |########################################
  4848477929.9 |
  4867061132.6 |########################################
  4885644335.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **llvm_incremental**: bridge=12.4% of algo (FFI overhead may distort results)
