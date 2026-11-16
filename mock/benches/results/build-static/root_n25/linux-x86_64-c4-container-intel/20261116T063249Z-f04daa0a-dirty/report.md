# build-static: rebuilt after an edit to one root contract crate

2 variants, 3 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (14310.33 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 1119.72 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 1178% faster than the next best (llvm_non_incremental)

llvm_incremental (1119.72 ms) leads llvm_non_incremental (14310.33 ms) by 1178%, a clear separation rather than a photo finish. CV 4.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 91% (significant)

llvm_incremental is -13088.60 ms (91%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 12.8x the fastest

Fastest llvm_incremental (1119.72 ms) to slowest llvm_non_incremental (14310.33 ms): 12.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 1119721653.6 ns median (-92.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 12.78x (fastest 1119721653.6 ns, slowest 14310330138.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | build-static: rebuilt after an edit to one root contract crate |
| Passes | 3 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6276626c73740001 |
| Counter frequency | 2300379024 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 1152886035ns | 1119731087ns | 1117189510ns | -0ns | 1152886035ns | -92.02% |
| llvm_non_incremental | 14448145125ns | 14310339554ns | 14185049032ns | -0ns | 14448145125ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 1152873733ns | 1117172965ns | 1152873733ns | -92.02% |
| llvm_non_incremental | 14448136381ns | 14185040272ns | 14448136381ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 1152886035ns | 1152886035ns | -92.02% |
| llvm_non_incremental | 14448145125ns | 14448145125ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 14310330138ns | base | --- | [14185040272, 14849038733] | --- | --- | --- | --- |
| llvm_incremental | 1119721654ns | -13088603558.9ns (-91.5%) | [-13731865768, -13065318618]ns | [1117172965, 1221726580] | YES (adj: no) | 0.2500 | 0.2500 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 14310330138ns | -91.5% |
| 1 | 2 | 14185040272ns | -92.1% |
| 1 | 3 | 14849038733ns | -92.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.154 | ok |
| llvm_non_incremental | -0.278 | moderate- |

**Consistency summary:**

- **llvm_incremental**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 64182777.9ns | 1152873732.6ns | 5.6% | HIGH |
| llvm_non_incremental | 65422927.7ns | 14448136381.0ns | 0.5% |  |

## Distribution (algo ns)

```
llvm_incremental (n=3, range 1117172964.8-1152873732.6 ns)
  1117172964.8 |########################################
  1118958003.2 |########################################
  1120743041.6 |
  1122528080.0 |
  1124313118.4 |
  1126098156.8 |
  1127883195.1 |
  1129668233.5 |
  1131453271.9 |
  1133238310.3 |
  1135023348.7 |
  1136808387.1 |
  1138593425.5 |
  1140378463.9 |
  1142163502.3 |
  1143948540.7 |
  1145733579.1 |
  1147518617.5 |
  1149303655.8 |
  1151088694.2 |
  (0 below, 1 above range)

llvm_non_incremental (n=3, range 14185040271.9-14448136381.0 ns)
  14185040271.9 |########################################
  14198195077.4 |
  14211349882.8 |
  14224504688.3 |
  14237659493.7 |
  14250814299.2 |
  14263969104.6 |
  14277123910.1 |
  14290278715.5 |
  14303433521.0 |########################################
  14316588326.5 |
  14329743131.9 |
  14342897937.4 |
  14356052742.8 |
  14369207548.3 |
  14382362353.7 |
  14395517159.2 |
  14408671964.6 |
  14421826770.1 |
  14434981575.5 |
  (0 below, 1 above range)

```
