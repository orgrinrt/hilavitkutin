# build-static: rebuilt after an edit to one root contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (8761.35 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 666.89 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 1214% faster than the next best (llvm_non_incremental)

llvm_incremental (666.89 ms) leads llvm_non_incremental (8761.35 ms) by 1214%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 92% (significant)

llvm_incremental is -8088.20 ms (92%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 13.1x the fastest

Fastest llvm_incremental (666.89 ms) to slowest llvm_non_incremental (8761.35 ms): 13.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 666886270.1 ns median (-92.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 13.14x (fastest 666886270.1 ns, slowest 8761354479.3 ns)

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
| Counter frequency | 2112687708 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 664274215ns | 666888509ns | 651233442ns | 662294603ns | 673764017ns | -92.40% |
| llvm_non_incremental | 8741282451ns | 8761357248ns | 8631989952ns | 8750017868ns | 8782825575ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 664271902ns | 651231130ns | 673761318ns | -92.40% |
| llvm_non_incremental | 8741279806ns | 8631987736ns | 8782823046ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 664274215ns | 664274215ns | -92.40% |
| llvm_non_incremental | 8741282451ns | 8741282451ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 8761354479ns | base | --- | [8679661892, 8782823046] | --- | --- | --- | --- |
| llvm_incremental | 666886270ns | -8088201646.7ns (-92.3%) | [-8124222112, -8018599952]ns | [652168119, 673761318] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 8727336048ns | -92.5% |
| 1 | 2 | 8772040597ns | -92.3% |
| 1 | 3 | 8772037633ns | -92.6% |
| 1 | 4 | 8793605494ns | -92.4% |
| 1 | 5 | 8631987736ns | -92.2% |
| 1 | 6 | 8750671326ns | -92.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.230 | moderate- |
| llvm_non_incremental | -0.274 | moderate- |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 46559772.7ns | 664271902.2ns | 7.0% | HIGH |
| llvm_non_incremental | 46918691.8ns | 8741279805.7ns | 0.5% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 651231130.0-673761317.6 ns)
  651231130.0 |########################################
  652357639.4 |########################################
  653484148.8 |
  654610658.1 |
  655737167.5 |
  656863676.9 |
  657990186.3 |
  659116695.7 |
  660243205.1 |
  661369714.4 |
  662496223.8 |
  663622733.2 |########################################
  664749242.6 |
  665875752.0 |
  667002261.4 |
  668128770.7 |
  669255280.1 |########################################
  670381789.5 |
  671508298.9 |########################################
  672634808.3 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 8631987736.2-8782823045.5 ns)
  8631987736.2 |####################
  8639529501.7 |
  8647071267.1 |
  8654613032.6 |
  8662154798.1 |
  8669696563.5 |
  8677238329.0 |
  8684780094.5 |
  8692321859.9 |
  8699863625.4 |
  8707405390.9 |
  8714947156.3 |
  8722488921.8 |####################
  8730030687.3 |
  8737572452.7 |
  8745114218.2 |####################
  8752655983.7 |
  8760197749.1 |
  8767739514.6 |########################################
  8775281280.1 |
  (0 below, 1 above range)

```
