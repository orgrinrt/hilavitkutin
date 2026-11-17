# build-static: rebuilt after an edit to one mid contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (25917.33 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 2908.28 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 791% faster than the next best (llvm_non_incremental)

llvm_incremental (2908.28 ms) leads llvm_non_incremental (25917.33 ms) by 791%, a clear separation rather than a photo finish. CV 5.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 89% (significant)

llvm_incremental is -23081.12 ms (89%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### llvm_incremental is fastest but the noisiest (CV 5.9%)

llvm_incremental wins on median (2908.28 ms) yet has the highest variance (CV 5.9%), while llvm_non_incremental is the steadiest (CV 5.9%, 25917.33 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### llvm_incremental shows alternating (throttle bounce) (autocorr -0.72)

llvm_incremental's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 8.9x the fastest

Fastest llvm_incremental (2908.28 ms) to slowest llvm_non_incremental (25917.33 ms): 8.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 2908277916.7 ns median (-88.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 8.91x (fastest 2908277916.7 ns, slowest 25917330416.7 ns)

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
| llvm_incremental | 2930234486ns | 2908284458ns | 2698558042ns | 2874349972ns | 3129899479ns | -88.87% |
| llvm_non_incremental | 26320035535ns | 25917335292ns | 24586575375ns | 25649194931ns | 28193026521ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 2930230736ns | 2698555458ns | 3129897250ns | -88.87% |
| llvm_non_incremental | 26320028500ns | 24586569583ns | 28193018167ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 2930234486ns | 2930234486ns | -88.87% |
| llvm_non_incremental | 26320035535ns | 26320035535ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 25917330417ns | base | --- | [24849736917, 28193018167] | --- | --- | --- | --- |
| llvm_incremental | 2908277917ns | -23081120916.7ns (-89.1%) | [-25174535146, -21913737229]ns | [2752517042, 3129897250] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 25112904250ns | -87.9% |
| 1 | 2 | 26040620083ns | -89.6% |
| 1 | 3 | 27159062000ns | -88.1% |
| 1 | 4 | 24586569583ns | -88.4% |
| 1 | 5 | 25794040750ns | -88.5% |
| 1 | 6 | 29226974333ns | -90.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.717 | HIGH- (thermal bounce) |
| llvm_non_incremental | -0.141 | ok |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 372196180.6ns | 2930230736.1ns | 12.7% | HIGH |
| llvm_non_incremental | 426176548.6ns | 26320028500.0ns | 1.6% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 2698555458.3-3129897250.0 ns)
  2698555458.3 |########################################
  2720122547.9 |
  2741689637.5 |
  2763256727.1 |
  2784823816.6 |
  2806390906.2 |########################################
  2827957995.8 |########################################
  2849525085.4 |
  2871092175.0 |
  2892659264.6 |
  2914226354.2 |
  2935793443.7 |
  2957360533.3 |########################################
  2978927622.9 |
  3000494712.5 |
  3022061802.1 |########################################
  3043628891.7 |
  3065195981.2 |
  3086763070.8 |
  3108330160.4 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 24586569583.3-28193018166.7 ns)
  24586569583.3 |########################################
  24766892012.5 |
  24947214441.6 |########################################
  25127536870.8 |
  25307859300.0 |
  25488181729.1 |
  25668504158.3 |########################################
  25848826587.5 |
  26029149016.6 |########################################
  26209471445.8 |
  26389793875.0 |
  26570116304.1 |
  26750438733.3 |
  26930761162.5 |
  27111083591.6 |########################################
  27291406020.8 |
  27471728450.0 |
  27652050879.1 |
  27832373308.3 |
  28012695737.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **llvm_incremental**: bridge=12.8% of algo (FFI overhead may distort results)
