# build-static: rebuilt after an edit to one root contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (49961.76 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 3976.99 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 1156% faster than the next best (llvm_non_incremental)

llvm_incremental (3976.99 ms) leads llvm_non_incremental (49961.76 ms) by 1156%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 92% (significant)

llvm_incremental is -45905.81 ms (92%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### llvm_non_incremental shows alternating (throttle bounce) (autocorr -0.51)

llvm_non_incremental's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 12.6x the fastest

Fastest llvm_incremental (3976.99 ms) to slowest llvm_non_incremental (49961.76 ms): 12.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 3976993476.4 ns median (-92.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 12.56x (fastest 3976993476.4 ns, slowest 49961763771.3 ns)

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
| Counter frequency | 2112687708 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 3997413996ns | 3976995421ns | 3950857726ns | 3973902342ns | 4055959610ns | -91.99% |
| llvm_non_incremental | 49936357684ns | 49961765962ns | 49583333401ns | 49859603362ns | 50228001309ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 3997412099ns | 3950855856ns | 4055957808ns | -91.99% |
| llvm_non_incremental | 49936355449ns | 49583330608ns | 50227999195ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 3997413996ns | 3997413996ns | -91.99% |
| llvm_non_incremental | 49936357684ns | 49936357684ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 49961763771ns | base | --- | [49619303380, 50227999195] | --- | --- | --- | --- |
| llvm_incremental | 3976993476ns | -45905805963.4ns (-91.9%) | [-46267174704, -45643849381]ns | [3959285014, 4055957808] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 50310670339ns | -92.1% |
| 1 | 2 | 49583330608ns | -92.0% |
| 1 | 3 | 49849180741ns | -91.8% |
| 1 | 4 | 50074346802ns | -92.0% |
| 1 | 5 | 49655276152ns | -92.0% |
| 1 | 6 | 50145328052ns | -92.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.030 | ok |
| llvm_non_incremental | -0.509 | HIGH- (thermal bounce) |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 245500233.8ns | 3997412099.3ns | 6.1% | HIGH |
| llvm_non_incremental | 237222250.8ns | 49936355448.9ns | 0.5% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 3950855855.8-4055957807.9 ns)
  3950855855.8 |####################
  3956110953.4 |
  3961366051.0 |
  3966621148.6 |########################################
  3971876246.2 |
  3977131343.8 |
  3982386441.4 |####################
  3987641539.0 |
  3992896636.6 |
  3998151734.2 |
  4003406831.8 |####################
  4008661929.4 |
  4013917027.0 |
  4019172124.6 |
  4024427222.2 |
  4029682319.8 |
  4034937417.4 |
  4040192515.0 |
  4045447612.6 |
  4050702710.2 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 49583330607.5-50227999195.4 ns)
  49583330607.5 |########################################
  49615564036.9 |
  49647797466.3 |########################################
  49680030895.7 |
  49712264325.1 |
  49744497754.5 |
  49776731183.9 |
  49808964613.3 |
  49841198042.7 |########################################
  49873431472.1 |
  49905664901.5 |
  49937898330.9 |
  49970131760.3 |
  50002365189.7 |
  50034598619.1 |
  50066832048.5 |########################################
  50099065477.9 |
  50131298907.3 |########################################
  50163532336.7 |
  50195765766.1 |
  (0 below, 1 above range)

```
