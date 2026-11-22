# build-static: rebuilt after an edit to one mid contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (19538.46 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 1564.03 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 1149% faster than the next best (llvm_non_incremental)

llvm_incremental (1564.03 ms) leads llvm_non_incremental (19538.46 ms) by 1149%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 92% (significant)

llvm_incremental is -17992.27 ms (92%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 12.5x the fastest

Fastest llvm_incremental (1564.03 ms) to slowest llvm_non_incremental (19538.46 ms): 12.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 1564027100.7 ns median (-92.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 12.49x (fastest 1564027100.7 ns, slowest 19538461785.6 ns)

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
| llvm_incremental | 1557748002ns | 1564029505ns | 1532651396ns | 1553602341ns | 1576514796ns | -91.99% |
| llvm_non_incremental | 19440656858ns | 19538463942ns | 18924138599ns | 19442707597ns | 19695839878ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 1557745877ns | 1532649707ns | 1576513009ns | -91.99% |
| llvm_non_incremental | 19440654747ns | 18924136060ns | 19695837942ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 1557748002ns | 1557748002ns | -91.99% |
| llvm_non_incremental | 19440656858ns | 19440656858ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 19538461786ns | base | --- | [19087664513, 19695837942] | --- | --- | --- | --- |
| llvm_incremental | 1564027101ns | -17992274126.7ns (-92.1%) | [-18119324932, -17537127549]ns | [1532697522, 1576513009] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 18924136060ns | -91.9% |
| 1 | 2 | 19586213612ns | -92.0% |
| 1 | 3 | 19490709960ns | -92.1% |
| 1 | 4 | 19761914107ns | -92.0% |
| 1 | 5 | 19629761776ns | -92.0% |
| 1 | 6 | 19251192966ns | -91.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.067 | ok |
| llvm_non_incremental | -0.058 | ok |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 225608248.5ns | 1557745877.5ns | 14.5% | HIGH |
| llvm_non_incremental | 228359118.6ns | 19440654746.6ns | 1.2% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 1532649706.8-1576513009.4 ns)
  1532649706.8 |########################################
  1534842871.9 |
  1537036037.1 |
  1539229202.2 |
  1541422367.3 |
  1543615532.5 |
  1545808697.6 |
  1548001862.7 |
  1550195027.8 |
  1552388193.0 |
  1554581358.1 |
  1556774523.2 |
  1558967688.4 |####################
  1561160853.5 |
  1563354018.6 |
  1565547183.8 |
  1567740348.9 |####################
  1569933514.0 |
  1572126679.1 |####################
  1574319844.3 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 18924136059.5-19695837941.5 ns)
  18924136059.5 |########################################
  18962721153.6 |
  19001306247.7 |
  19039891341.8 |
  19078476435.9 |
  19117061530.0 |
  19155646624.1 |
  19194231718.2 |
  19232816812.3 |########################################
  19271401906.4 |
  19309987000.5 |
  19348572094.6 |
  19387157188.7 |
  19425742282.8 |
  19464327376.9 |########################################
  19502912471.0 |
  19541497565.1 |
  19580082659.2 |########################################
  19618667753.3 |########################################
  19657252847.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **llvm_incremental**: bridge=14.1% of algo (FFI overhead may distort results)
