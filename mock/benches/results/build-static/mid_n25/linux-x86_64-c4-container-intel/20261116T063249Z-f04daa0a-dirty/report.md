# build-static: rebuilt after an edit to one mid contract crate

2 variants, 3 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (5582.15 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 693.37 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 705% faster than the next best (llvm_non_incremental)

llvm_incremental (693.37 ms) leads llvm_non_incremental (5582.15 ms) by 705%, a clear separation rather than a photo finish. CV 7.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 87% (significant)

llvm_incremental is -4874.94 ms (87%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### llvm_incremental is fastest but the noisiest (CV 7.4%)

llvm_incremental wins on median (693.37 ms) yet has the highest variance (CV 7.4%), while llvm_non_incremental is the steadiest (CV 3.3%, 5582.15 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### llvm_incremental shows alternating (throttle bounce) (autocorr -0.66)

llvm_incremental's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 8.1x the fastest

Fastest llvm_incremental (693.37 ms) to slowest llvm_non_incremental (5582.15 ms): 8.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 693372470.1 ns median (-87.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 8.05x (fastest 693372470.1 ns, slowest 5582150815.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | build-static: rebuilt after an edit to one mid contract crate |
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
| llvm_incremental | 663998155ns | 693383846ns | 591390689ns | -0ns | 663998155ns | -87.91% |
| llvm_non_incremental | 5494114090ns | 5582187883ns | 5239089009ns | -0ns | 5494114090ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 663986760ns | 591377855ns | 663986760ns | -87.91% |
| llvm_non_incremental | 5494093263ns | 5239073650ns | 5494093263ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 663998155ns | 663998155ns | -87.91% |
| llvm_non_incremental | 5494114090ns | 5494114090ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 5582150816ns | base | --- | [5239073650, 5661055324] | --- | --- | --- | --- |
| llvm_incremental | 693372470ns | -4874940859.3ns (-87.3%) | [-5069677469, -4545701180]ns | [591377855, 707209956] | YES (adj: no) | 0.2500 | 0.2500 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 5239073650ns | -86.8% |
| 1 | 2 | 5661055324ns | -89.6% |
| 1 | 3 | 5582150816ns | -87.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.659 | HIGH- (thermal bounce) |
| llvm_non_incremental | -0.277 | moderate- |

**Consistency summary:**

- **llvm_incremental**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 71905643.8ns | 663986760.3ns | 10.8% | HIGH |
| llvm_non_incremental | 61287281.6ns | 5494093263.3ns | 1.1% |  |

## Distribution (algo ns)

```
llvm_incremental (n=3, range 591377854.6-663986760.3 ns)
  591377854.6 |########################################
  595008299.9 |
  598638745.2 |
  602269190.5 |
  605899635.7 |
  609530081.0 |
  613160526.3 |
  616790971.6 |
  620421416.9 |
  624051862.2 |
  627682307.5 |
  631312752.8 |
  634943198.0 |
  638573643.3 |
  642204088.6 |
  645834533.9 |
  649464979.2 |
  653095424.5 |
  656725869.8 |
  660356315.0 |
  (0 below, 2 above range)

llvm_non_incremental (n=3, range 5239073650.5-5494093263.3 ns)
  5239073650.5 |########################################
  5251824631.1 |
  5264575611.8 |
  5277326592.4 |
  5290077573.1 |
  5302828553.7 |
  5315579534.4 |
  5328330515.0 |
  5341081495.6 |
  5353832476.3 |
  5366583456.9 |
  5379334437.6 |
  5392085418.2 |
  5404836398.8 |
  5417587379.5 |
  5430338360.1 |
  5443089340.8 |
  5455840321.4 |
  5468591302.0 |
  5481342282.7 |
  (0 below, 2 above range)

```

## Diagnostics

- **llvm_incremental**: bridge=10.7% of algo (FFI overhead may distort results)
