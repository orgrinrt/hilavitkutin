# build-static: rebuilt after an edit to one mid contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (32611.66 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 2547.55 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 1180% faster than the next best (llvm_non_incremental)

llvm_incremental (2547.55 ms) leads llvm_non_incremental (32611.66 ms) by 1180%, a clear separation rather than a photo finish. CV 6.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 92% (significant)

llvm_incremental is -30157.00 ms (92%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### llvm_incremental is fastest but the noisiest (CV 6.3%)

llvm_incremental wins on median (2547.55 ms) yet has the highest variance (CV 6.3%), while llvm_non_incremental is the steadiest (CV 4.0%, 32611.66 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 12.8x the fastest

Fastest llvm_incremental (2547.55 ms) to slowest llvm_non_incremental (32611.66 ms): 12.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 2547548705.2 ns median (-92.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 12.80x (fastest 2547548705.2 ns, slowest 32611664317.7 ns)

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
| llvm_incremental | 2590123611ns | 2547552569ns | 2378552361ns | 2537373407ns | 2775034543ns | -92.22% |
| llvm_non_incremental | 33279859585ns | 32611668048ns | 32138227684ns | 32509292398ns | 35006526316ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 2590119614ns | 2378548090ns | 2775030546ns | -92.22% |
| llvm_non_incremental | 33279855530ns | 32138223181ns | 35006522168ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 2590123611ns | 2590123611ns | -92.22% |
| llvm_non_incremental | 33279859585ns | 33279859585ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 32611664318ns | base | --- | [32221380105, 35006522168] | --- | --- | --- | --- |
| llvm_incremental | 2547548705ns | -30157002623.7ns (-92.5%) | [-32401225319, -29510979808]ns | [2447779589, 2775030546] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 34237711730ns | -92.5% |
| 1 | 2 | 32138223181ns | -92.2% |
| 1 | 3 | 32494394361ns | -92.2% |
| 1 | 4 | 32728934274ns | -92.7% |
| 1 | 5 | 35775332607ns | -92.6% |
| 1 | 6 | 32304537029ns | -91.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | 0.157 | ok |
| llvm_non_incremental | -0.346 | moderate- |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 292767023.6ns | 2590119613.6ns | 11.3% | HIGH |
| llvm_non_incremental | 296135642.8ns | 33279855530.3ns | 0.9% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 2378548090.3-2775030546.3 ns)
  2378548090.3 |########################################
  2398372213.1 |
  2418196335.9 |
  2438020458.7 |
  2457844581.5 |
  2477668704.3 |
  2497492827.1 |########################################
  2517316949.9 |########################################
  2537141072.7 |
  2556965195.5 |########################################
  2576789318.3 |
  2596613441.1 |
  2616437563.9 |
  2636261686.7 |########################################
  2656085809.5 |
  2675909932.3 |
  2695734055.1 |
  2715558177.9 |
  2735382300.7 |
  2755206423.5 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 32138223181.1-35006522168.2 ns)
  32138223181.1 |########################################
  32281638130.5 |########################################
  32425053079.8 |########################################
  32568468029.2 |
  32711882978.5 |########################################
  32855297927.9 |
  32998712877.2 |
  33142127826.6 |
  33285542775.9 |
  33428957725.3 |
  33572372674.6 |
  33715787624.0 |
  33859202573.4 |
  34002617522.7 |
  34146032472.1 |########################################
  34289447421.4 |
  34432862370.8 |
  34576277320.1 |
  34719692269.5 |
  34863107218.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **llvm_incremental**: bridge=11.8% of algo (FFI overhead may distort results)
