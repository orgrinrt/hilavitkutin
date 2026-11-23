# build-static: rebuilt after an edit to one leaf implementation crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (3929.92 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 2647.57 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 48% faster than the next best (llvm_non_incremental)

llvm_incremental (2647.57 ms) leads llvm_non_incremental (3929.92 ms) by 48%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 33% (significant)

llvm_incremental is -1289.38 ms (33%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: llvm_incremental** at 2647567313.6 ns median (-32.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.48x (fastest 2647567313.6 ns, slowest 3929915966.4 ns)

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
| Counter frequency | 2112789370 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 2656646732ns | 2647569215ns | 2633107286ns | 2643760562ns | 2687745710ns | -32.52% |
| llvm_non_incremental | 3936700201ns | 3929917960ns | 3901725173ns | 3923822009ns | 3973505002ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 2656644461ns | 2633105033ns | 2687742946ns | -32.52% |
| llvm_non_incremental | 3936698125ns | 3901722906ns | 3973502976ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 2656646732ns | 2656646732ns | -32.52% |
| llvm_non_incremental | 3936700201ns | 3936700201ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 3929915966ns | base | --- | [3906675433, 3973502976] | --- | --- | --- | --- |
| llvm_incremental | 2647567314ns | -1289382068.5ns (-32.8%) | [-1309392247, -1241386678]ns | [2634623122, 2687742946] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 3993072295ns | -32.5% |
| 1 | 2 | 3953933657ns | -33.4% |
| 1 | 3 | 3920517284ns | -32.8% |
| 1 | 4 | 3911627961ns | -31.5% |
| 1 | 5 | 3939314649ns | -32.9% |
| 1 | 6 | 3901722906ns | -32.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.348 | moderate- |
| llvm_non_incremental | 0.168 | ok |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 205205234.0ns | 2656644460.7ns | 7.7% | HIGH |
| llvm_non_incremental | 199689803.1ns | 3936698125.3ns | 5.1% | HIGH |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 2633105032.6-2687742946.4 ns)
  2633105032.6 |########################################
  2635836928.3 |########################################
  2638568824.0 |
  2641300719.7 |
  2644032615.4 |########################################
  2646764511.1 |
  2649496406.8 |########################################
  2652228302.4 |
  2654960198.1 |
  2657692093.8 |
  2660423989.5 |
  2663155885.2 |
  2665887780.9 |
  2668619676.6 |
  2671351572.3 |
  2674083468.0 |
  2676815363.7 |
  2679547259.4 |########################################
  2682279155.1 |
  2685011050.8 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 3901722905.8-3973502976.3 ns)
  3901722905.8 |########################################
  3905311909.3 |
  3908900912.9 |########################################
  3912489916.4 |
  3916078919.9 |
  3919667923.4 |########################################
  3923256927.0 |
  3926845930.5 |
  3930434934.0 |
  3934023937.5 |
  3937612941.1 |########################################
  3941201944.6 |
  3944790948.1 |
  3948379951.6 |
  3951968955.2 |########################################
  3955557958.7 |
  3959146962.2 |
  3962735965.7 |
  3966324969.2 |
  3969913972.8 |
  (0 below, 1 above range)

```
