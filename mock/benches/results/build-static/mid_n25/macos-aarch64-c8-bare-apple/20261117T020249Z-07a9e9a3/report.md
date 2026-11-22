# build-static: rebuilt after an edit to one mid contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (4991.35 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 581.68 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 758% faster than the next best (llvm_non_incremental)

llvm_incremental (581.68 ms) leads llvm_non_incremental (4991.35 ms) by 758%, a clear separation rather than a photo finish. CV 8.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 88% (significant)

llvm_incremental is -4401.88 ms (88%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### llvm_incremental is fastest but the noisiest (CV 8.2%)

llvm_incremental wins on median (581.68 ms) yet has the highest variance (CV 8.2%), while llvm_non_incremental is the steadiest (CV 5.7%, 4991.35 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 8.6x the fastest

Fastest llvm_incremental (581.68 ms) to slowest llvm_non_incremental (4991.35 ms): 8.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 581684645.9 ns median (-88.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 8.58x (fastest 581684645.9 ns, slowest 4991347270.8 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 576587465ns | 581687833ns | 502461833ns | 565036083ns | 630977354ns | -88.67% |
| llvm_non_incremental | 5090163750ns | 4991349396ns | 4841843250ns | 4945871514ns | 5430762354ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 576584722ns | 502460083ns | 630974875ns | -88.67% |
| llvm_non_incremental | 5090159993ns | 4841840500ns | 5430755938ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 576587465ns | 576587465ns | -88.67% |
| llvm_non_incremental | 5090163750ns | 5090163750ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 4991347271ns | base | --- | [4848376771, 5430755938] | --- | --- | --- | --- |
| llvm_incremental | 581684646ns | -4401884708.4ns (-88.2%) | [-4887328250, -4251512854]ns | [517094646, 630974875] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 4854913042ns | -86.9% |
| 1 | 2 | 4861993208ns | -88.1% |
| 1 | 3 | 5232569250ns | -90.4% |
| 1 | 4 | 5120701333ns | -87.8% |
| 1 | 5 | 5628942625ns | -89.6% |
| 1 | 6 | 4841840500ns | -89.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.276 | moderate- |
| llvm_non_incremental | -0.191 | ok |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 79578534.7ns | 576584722.2ns | 13.8% | HIGH |
| llvm_non_incremental | 68218166.7ns | 5090159993.1ns | 1.3% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 502460083.3-630974875.0 ns)
  502460083.3 |########################################
  508885822.9 |
  515311562.5 |
  521737302.1 |
  528163041.6 |########################################
  534588781.2 |
  541014520.8 |
  547440260.4 |
  553866000.0 |
  560291739.6 |
  566717479.1 |
  573143218.7 |########################################
  579568958.3 |########################################
  585994697.9 |
  592420437.5 |
  598846177.1 |
  605271916.7 |
  611697656.2 |
  618123395.8 |
  624549135.4 |########################################
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 4841840500.0-5430755937.5 ns)
  4841840500.0 |########################################
  4871286271.9 |
  4900732043.8 |
  4930177815.6 |
  4959623587.5 |
  4989069359.4 |
  5018515131.2 |
  5047960903.1 |
  5077406675.0 |
  5106852446.9 |#############
  5136298218.8 |
  5165743990.6 |
  5195189762.5 |
  5224635534.4 |#############
  5254081306.2 |
  5283527078.1 |
  5312972850.0 |
  5342418621.9 |
  5371864393.8 |
  5401310165.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **llvm_incremental**: bridge=13.2% of algo (FFI overhead may distort results)
