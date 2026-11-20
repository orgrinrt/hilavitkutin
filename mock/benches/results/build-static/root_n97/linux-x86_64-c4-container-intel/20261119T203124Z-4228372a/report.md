# build-static: rebuilt after an edit to one root contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (89677.26 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 7182.84 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 1148% faster than the next best (llvm_non_incremental)

llvm_incremental (7182.84 ms) leads llvm_non_incremental (89677.26 ms) by 1148%, a clear separation rather than a photo finish. CV 4.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 92% (significant)

llvm_incremental is -82818.00 ms (92%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 12.5x the fastest

Fastest llvm_incremental (7182.84 ms) to slowest llvm_non_incremental (89677.26 ms): 12.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 7182840693.9 ns median (-92.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 12.48x (fastest 7182840693.9 ns, slowest 89677259104.6 ns)

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
| Counter frequency | 2100474318 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 7060926019ns | 7182844938ns | 6408866982ns | 7095711734ns | 7334776966ns | -92.10% |
| llvm_non_incremental | 89367347781ns | 89677263891ns | 87008064674ns | 88887023386ns | 91267475926ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 7060921531ns | 6408862760ns | 7334772539ns | -92.10% |
| llvm_non_incremental | 89367343230ns | 87008060124ns | 91267471623ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 7060926019ns | 7060926019ns | -92.10% |
| llvm_non_incremental | 89367347781ns | 89367347781ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 89677259105ns | base | --- | [87157298964, 91267471623] | --- | --- | --- | --- |
| llvm_incremental | 7182840694ns | -82818003551.9ns (-92.4%) | [-83976529718, -80124731827]ns | [6665151361, 7334772539] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 87306537804ns | -92.1% |
| 1 | 2 | 90675962705ns | -92.0% |
| 1 | 3 | 90057184317ns | -91.9% |
| 1 | 4 | 87008060124ns | -91.8% |
| 1 | 5 | 89297333892ns | -92.8% |
| 1 | 6 | 91858980540ns | -92.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.335 | moderate- |
| llvm_non_incremental | -0.188 | ok |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 316291513.4ns | 7060921531.3ns | 4.5% |  |
| llvm_non_incremental | 316495402.6ns | 89367343230.2ns | 0.4% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 6408862760.4-7334772538.6 ns)
  6408862760.4 |########################################
  6455158249.3 |
  6501453738.2 |
  6547749227.1 |
  6594044716.0 |
  6640340204.9 |
  6686635693.9 |
  6732931182.8 |
  6779226671.7 |
  6825522160.6 |
  6871817649.5 |
  6918113138.4 |########################################
  6964408627.3 |
  7010704116.2 |
  7056999605.1 |
  7103295094.1 |########################################
  7149590583.0 |
  7195886071.9 |########################################
  7242181560.8 |
  7288477049.7 |########################################
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 87008060123.5-91267471622.5 ns)
  87008060123.5 |########################################
  87221030698.5 |########################################
  87434001273.4 |
  87646971848.4 |
  87859942423.3 |
  88072912998.3 |
  88285883573.2 |
  88498854148.2 |
  88711824723.1 |
  88924795298.1 |
  89137765873.0 |########################################
  89350736448.0 |
  89563707022.9 |
  89776677597.9 |
  89989648172.8 |########################################
  90202618747.8 |
  90415589322.7 |
  90628559897.7 |########################################
  90841530472.6 |
  91054501047.6 |
  (0 below, 1 above range)

```
