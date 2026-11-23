# build-static: rebuilt after an edit to one mid contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (19377.74 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 1520.98 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 1174% faster than the next best (llvm_non_incremental)

llvm_incremental (1520.98 ms) leads llvm_non_incremental (19377.74 ms) by 1174%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 92% (significant)

llvm_incremental is -17851.37 ms (92%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### llvm_incremental shows alternating (throttle bounce) (autocorr -0.59)

llvm_incremental's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 12.7x the fastest

Fastest llvm_incremental (1520.98 ms) to slowest llvm_non_incremental (19377.74 ms): 12.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 1520980009.0 ns median (-92.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 12.74x (fastest 1520980009.0 ns, slowest 19377736666.8 ns)

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
| Counter frequency | 2112712847 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 1521910093ns | 1520981853ns | 1508662362ns | 1519592498ns | 1532010350ns | -92.12% |
| llvm_non_incremental | 19320777893ns | 19377738919ns | 19061199792ns | 19354006073ns | 19400724674ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 1521908080ns | 1508660590ns | 1532008118ns | -92.12% |
| llvm_non_incremental | 19320775758ns | 19061197627ns | 19400722626ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 1521910093ns | 1521910093ns | -92.12% |
| llvm_non_incremental | 19320777893ns | 19320777893ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 19377736667ns | base | --- | [19183867982, 19400722626] | --- | --- | --- | --- |
| llvm_incremental | 1520980009ns | -17851368905.2ns (-92.1%) | [-17874102261, -17671131869]ns | [1512736113, 1532008118] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 19306538337ns | -92.2% |
| 1 | 2 | 19382634944ns | -92.1% |
| 1 | 3 | 19061197627ns | -92.0% |
| 1 | 4 | 19415885749ns | -92.1% |
| 1 | 5 | 19385559504ns | -92.1% |
| 1 | 6 | 19372838389ns | -92.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.595 | HIGH- (thermal bounce) |
| llvm_non_incremental | -0.367 | moderate- |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 217496947.5ns | 1521908079.9ns | 14.3% | HIGH |
| llvm_non_incremental | 224224350.5ns | 19320775758.4ns | 1.2% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 1508660590.5-1532008117.8 ns)
  1508660590.5 |########################################
  1509827966.9 |
  1510995343.2 |
  1512162719.6 |
  1513330096.0 |
  1514497472.3 |
  1515664848.7 |########################################
  1516832225.0 |########################################
  1517999601.4 |
  1519166977.8 |
  1520334354.1 |
  1521501730.5 |
  1522669106.8 |
  1523836483.2 |########################################
  1525003859.6 |
  1526171235.9 |
  1527338612.3 |
  1528505988.7 |########################################
  1529673365.0 |
  1530840741.4 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 19061197626.7-19400722626.3 ns)
  19061197626.7 |####################
  19078173876.7 |
  19095150126.7 |
  19112126376.6 |
  19129102626.6 |
  19146078876.6 |
  19163055126.6 |
  19180031376.6 |
  19197007626.6 |
  19213983876.5 |
  19230960126.5 |
  19247936376.5 |
  19264912626.5 |
  19281888876.5 |
  19298865126.5 |####################
  19315841376.4 |
  19332817626.4 |
  19349793876.4 |
  19366770126.4 |########################################
  19383746376.4 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **llvm_incremental**: bridge=14.2% of algo (FFI overhead may distort results)
