# build-static: rebuilt after an edit to one mid contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (5756.40 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 578.84 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 894% faster than the next best (llvm_non_incremental)

llvm_incremental (578.84 ms) leads llvm_non_incremental (5756.40 ms) by 894%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 90% (significant)

llvm_incremental is -5195.03 ms (90%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### llvm_incremental is fastest but the noisiest (CV 5.7%)

llvm_incremental wins on median (578.84 ms) yet has the highest variance (CV 5.7%), while llvm_non_incremental is the steadiest (CV 4.4%, 5756.40 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 9.9x the fastest

Fastest llvm_incremental (578.84 ms) to slowest llvm_non_incremental (5756.40 ms): 9.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 578836039.8 ns median (-89.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 9.94x (fastest 578836039.8 ns, slowest 5756395768.4 ns)

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
| Counter frequency | 2100332830 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 574454985ns | 578840586ns | 511622308ns | 573961036ns | 606612248ns | -90.12% |
| llvm_non_incremental | 5816441505ns | 5756399210ns | 5440448243ns | 5733192470ns | 6129311687ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 574450804ns | 511618111ns | 606608352ns | -90.12% |
| llvm_non_incremental | 5816437651ns | 5440443949ns | 6129307463ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 574454985ns | 574454985ns | -90.12% |
| llvm_non_incremental | 5816441505ns | 5816441505ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 5756395768ns | base | --- | [5563609721, 6129307463] | --- | --- | --- | --- |
| llvm_incremental | 578836040ns | -5195026738.6ns (-90.2%) | [-5522699111, -5008234689]ns | [537908022, 606608352] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 6217397461ns | -90.4% |
| 1 | 2 | 5712581067ns | -90.0% |
| 1 | 3 | 5686775493ns | -91.0% |
| 1 | 4 | 6041217464ns | -89.8% |
| 1 | 5 | 5800210470ns | -89.9% |
| 1 | 6 | 5440443949ns | -89.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.340 | moderate- |
| llvm_non_incremental | -0.144 | ok |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 62162416.6ns | 574450804.5ns | 10.8% | HIGH |
| llvm_non_incremental | 62561194.1ns | 5816437650.7ns | 1.1% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 511618110.7-606608351.8 ns)
  511618110.7 |########################################
  516367622.8 |
  521117134.8 |
  525866646.9 |
  530616158.9 |
  535365671.0 |
  540115183.0 |
  544864695.1 |
  549614207.1 |
  554363719.2 |
  559113231.2 |
  563862743.3 |########################################
  568612255.3 |########################################
  573361767.4 |
  578111279.4 |
  582860791.5 |########################################
  587610303.5 |
  592359815.6 |
  597109327.6 |########################################
  601858839.7 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 5440443948.9-6129307462.6 ns)
  5440443948.9 |####################
  5474887124.6 |
  5509330300.3 |
  5543773476.0 |
  5578216651.6 |
  5612659827.3 |
  5647103003.0 |
  5681546178.7 |########################################
  5715989354.4 |
  5750432530.1 |
  5784875705.8 |####################
  5819318881.4 |
  5853762057.1 |
  5888205232.8 |
  5922648408.5 |
  5957091584.2 |
  5991534759.9 |
  6025977935.5 |####################
  6060421111.2 |
  6094864286.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **llvm_incremental**: bridge=10.8% of algo (FFI overhead may distort results)
