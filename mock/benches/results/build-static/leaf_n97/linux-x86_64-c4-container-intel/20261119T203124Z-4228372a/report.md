# build-static: rebuilt after an edit to one leaf implementation crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (5821.77 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 4011.75 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 45% faster than the next best (llvm_non_incremental)

llvm_incremental (4011.75 ms) leads llvm_non_incremental (5821.77 ms) by 45%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 31% (significant)

llvm_incremental is -1823.94 ms (31%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: llvm_incremental** at 4011750087.1 ns median (-31.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.45x (fastest 4011750087.1 ns, slowest 5821771836.5 ns)

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
| Counter frequency | 2100474318 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 4028218231ns | 4011754027ns | 3843819647ns | 3999085691ns | 4164116333ns | -30.80% |
| llvm_non_incremental | 5820998593ns | 5821776260ns | 5636182752ns | 5788763749ns | 5961758779ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 4028214025ns | 3843815243ns | 4164111954ns | -30.80% |
| llvm_non_incremental | 5820994171ns | 5636178838ns | 5961754261ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 4028218231ns | 4028218231ns | -30.80% |
| llvm_non_incremental | 5820998593ns | 5820998593ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 5821771836ns | base | --- | [5679456414, 5961754261] | --- | --- | --- | --- |
| llvm_incremental | 4011750087ns | -1823935788.9ns (-31.3%) | [-1953151079, -1601253569]ns | [3908780034, 4164111954] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 5953138924ns | -33.2% |
| 1 | 2 | 5970369599ns | -32.3% |
| 1 | 3 | 5636178838ns | -29.3% |
| 1 | 4 | 5809673228ns | -30.4% |
| 1 | 5 | 5833870445ns | -26.6% |
| 1 | 6 | 5722733991ns | -32.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.439 | moderate- |
| llvm_non_incremental | -0.086 | ok |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 299017447.0ns | 4028214025.0ns | 7.4% | HIGH |
| llvm_non_incremental | 289834774.5ns | 5820994170.6ns | 5.0% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 3843815243.0-4164111953.5 ns)
  3843815243.0 |####################
  3859830078.5 |
  3875844914.1 |
  3891859749.6 |
  3907874585.1 |
  3923889420.6 |
  3939904256.2 |
  3955919091.7 |
  3971933927.2 |########################################
  3987948762.7 |
  4003963598.2 |
  4019978433.8 |
  4035993269.3 |########################################
  4052008104.8 |
  4068022940.3 |
  4084037775.9 |
  4100052611.4 |
  4116067446.9 |
  4132082282.4 |
  4148097118.0 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 5636178837.5-5961754261.4 ns)
  5636178837.5 |########################################
  5652457608.7 |
  5668736379.9 |
  5685015151.1 |
  5701293922.3 |
  5717572693.5 |########################################
  5733851464.7 |
  5750130235.8 |
  5766409007.0 |
  5782687778.2 |
  5798966549.4 |########################################
  5815245320.6 |
  5831524091.8 |########################################
  5847802863.0 |
  5864081634.2 |
  5880360405.4 |
  5896639176.6 |
  5912917947.8 |
  5929196719.0 |
  5945475490.2 |########################################
  (0 below, 1 above range)

```
