# build-static: rebuilt after an edit to one mid contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (6084.57 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 586.04 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 938% faster than the next best (llvm_non_incremental)

llvm_incremental (586.04 ms) leads llvm_non_incremental (6084.57 ms) by 938%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 90% (significant)

llvm_incremental is -5490.10 ms (90%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 10.4x the fastest

Fastest llvm_incremental (586.04 ms) to slowest llvm_non_incremental (6084.57 ms): 10.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 586037846.8 ns median (-90.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 10.38x (fastest 586037846.8 ns, slowest 6084569635.1 ns)

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
| Counter frequency | 2100505716 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 580675341ns | 586046794ns | 540669645ns | 578538311ns | 603883735ns | -90.55% |
| llvm_non_incremental | 6141547893ns | 6084573748ns | 5821778484ns | 6047535791ns | 6442450750ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 580669350ns | 540665111ns | 603879489ns | -90.55% |
| llvm_non_incremental | 6141543637ns | 5821774562ns | 6442445951ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 580675341ns | 580675341ns | -90.55% |
| llvm_non_incremental | 6141547893ns | 6141547893ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 6084569635ns | base | --- | [5897615325, 6442445951] | --- | --- | --- | --- |
| llvm_incremental | 586037847ns | -5490102275.4ns (-90.2%) | [-5871365172, -5321155414]ns | [552090713, 603879489] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 6140647558ns | -90.2% |
| 1 | 2 | 6115003148ns | -90.1% |
| 1 | 3 | 5973456089ns | -90.1% |
| 1 | 4 | 6054136122ns | -90.4% |
| 1 | 5 | 6744244343ns | -92.0% |
| 1 | 6 | 5821774562ns | -90.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | 0.450 | moderate+ |
| llvm_non_incremental | -0.451 | moderate- |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 68755283.4ns | 580669349.9ns | 11.8% | HIGH |
| llvm_non_incremental | 72011715.8ns | 6141543637.0ns | 1.2% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 540665111.0-603879489.4 ns)
  540665111.0 |########################################
  543825829.9 |
  546986548.8 |
  550147267.8 |
  553307986.7 |
  556468705.6 |
  559629424.5 |
  562790143.4 |########################################
  565950862.4 |
  569111581.3 |
  572272300.2 |
  575433019.1 |
  578593738.0 |
  581754457.0 |########################################
  584915175.9 |
  588075894.8 |########################################
  591236613.7 |
  594397332.6 |
  597558051.6 |
  600718770.5 |########################################
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 5821774561.6-6442445950.6 ns)
  5821774561.6 |########################################
  5852808131.1 |
  5883841700.5 |
  5914875270.0 |
  5945908839.4 |########################################
  5976942408.9 |
  6007975978.3 |
  6039009547.8 |########################################
  6070043117.2 |
  6101076686.7 |########################################
  6132110256.1 |########################################
  6163143825.6 |
  6194177395.0 |
  6225210964.5 |
  6256244533.9 |
  6287278103.4 |
  6318311672.8 |
  6349345242.2 |
  6380378811.7 |
  6411412381.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **llvm_incremental**: bridge=11.7% of algo (FFI overhead may distort results)
