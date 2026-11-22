# build-static: rebuilt after an edit to one leaf implementation crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (4043.19 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 2682.51 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 51% faster than the next best (llvm_non_incremental)

llvm_incremental (2682.51 ms) leads llvm_non_incremental (4043.19 ms) by 51%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 34% (significant)

llvm_incremental is -1370.65 ms (34%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### llvm_incremental shows alternating (throttle bounce) (autocorr -0.52)

llvm_incremental's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: llvm_incremental** at 2682506437.5 ns median (-33.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.51x (fastest 2682506437.5 ns, slowest 4043185729.2 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 2664198896ns | 2682508625ns | 2594406917ns | 2663805889ns | 2699684396ns | -34.44% |
| llvm_non_incremental | 4063870715ns | 4043188208ns | 4008674625ns | 4035370333ns | 4134219333ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 2664196618ns | 2594405125ns | 2699681771ns | -34.44% |
| llvm_non_incremental | 4063868222ns | 4008672458ns | 4134216833ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 2664198896ns | 2664198896ns | -34.44% |
| llvm_non_incremental | 4063870715ns | 4063870715ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 4043185729ns | base | --- | [4014202104, 4134216833] | --- | --- | --- | --- |
| llvm_incremental | 2682506438ns | -1370651166.6ns (-33.9%) | [-1503552792, -1324810854]ns | [2610401646, 2699681771] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 4029253500ns | -33.0% |
| 1 | 2 | 4117053167ns | -37.0% |
| 1 | 3 | 4151380500ns | -35.8% |
| 1 | 4 | 4019731750ns | -32.8% |
| 1 | 5 | 4008672458ns | -34.5% |
| 1 | 6 | 4057117958ns | -33.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.520 | HIGH- (thermal bounce) |
| llvm_non_incremental | 0.105 | ok |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 75512458.3ns | 2664196618.0ns | 2.8% |  |
| llvm_non_incremental | 67959479.2ns | 4063868222.2ns | 1.7% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 2594405125.0-2699681770.8 ns)
  2594405125.0 |####################
  2599668957.3 |
  2604932789.6 |
  2610196621.9 |
  2615460454.2 |
  2620724286.4 |
  2625988118.7 |####################
  2631251951.0 |
  2636515783.3 |
  2641779615.6 |
  2647043447.9 |
  2652307280.2 |
  2657571112.5 |
  2662834944.8 |####################
  2668098777.1 |
  2673362609.4 |
  2678626441.6 |
  2683890273.9 |
  2689154106.2 |
  2694417938.5 |########################################
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 4008672458.3-4134216833.3 ns)
  4008672458.3 |########################################
  4014949677.1 |########################################
  4021226895.8 |
  4027504114.6 |########################################
  4033781333.3 |
  4040058552.1 |
  4046335770.8 |
  4052612989.6 |########################################
  4058890208.3 |
  4065167427.1 |
  4071444645.8 |
  4077721864.6 |
  4083999083.3 |
  4090276302.1 |
  4096553520.8 |
  4102830739.6 |
  4109107958.3 |
  4115385177.1 |########################################
  4121662395.8 |
  4127939614.6 |
  (0 below, 1 above range)

```
