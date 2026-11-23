# build-static: rebuilt after an edit to one leaf implementation crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (3659.50 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 2385.83 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 53% faster than the next best (llvm_non_incremental)

llvm_incremental (2385.83 ms) leads llvm_non_incremental (3659.50 ms) by 53%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 35% (significant)

llvm_incremental is -1269.46 ms (35%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### llvm_non_incremental shows alternating (throttle bounce) (autocorr -0.68)

llvm_non_incremental's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: llvm_incremental** at 2385830098.7 ns median (-34.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.53x (fastest 2385830098.7 ns, slowest 3659498319.8 ns)

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
| llvm_incremental | 2386508661ns | 2385832486ns | 2370504832ns | 2381624982ns | 2401836093ns | -34.78% |
| llvm_non_incremental | 3659325873ns | 3659504883ns | 3626541628ns | 3650042317ns | 3689643329ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 2386506460ns | 2370503057ns | 2401834027ns | -34.78% |
| llvm_non_incremental | 3659322244ns | 3626539249ns | 3689641108ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 2386508661ns | 2386508661ns | -34.78% |
| llvm_non_incremental | 3659325873ns | 3659325873ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 3659498320ns | base | --- | [3628827304, 3689641108] | --- | --- | --- | --- |
| llvm_incremental | 2385830099ns | -1269459098.5ns (-34.7%) | [-1317785854, -1231202399]ns | [2371855254, 2401834027] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 3626539249ns | -34.2% |
| 1 | 2 | 3709997912ns | -36.0% |
| 1 | 3 | 3631115358ns | -33.6% |
| 1 | 4 | 3658740482ns | -34.6% |
| 1 | 5 | 3660256158ns | -34.8% |
| 1 | 6 | 3669284304ns | -35.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.134 | ok |
| llvm_non_incremental | -0.675 | HIGH- (thermal bounce) |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 46306987.3ns | 2386506459.9ns | 1.9% |  |
| llvm_non_incremental | 43980959.3ns | 3659322243.8ns | 1.2% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 2370503056.6-2401834026.9 ns)
  2370503056.6 |########################################
  2372069605.1 |########################################
  2373636153.6 |
  2375202702.2 |
  2376769250.7 |
  2378335799.2 |
  2379902347.7 |
  2381468896.2 |
  2383035444.7 |
  2384601993.3 |########################################
  2386168541.8 |########################################
  2387735090.3 |
  2389301638.8 |
  2390868187.3 |
  2392434735.8 |########################################
  2394001284.4 |
  2395567832.9 |
  2397134381.4 |
  2398700929.9 |
  2400267478.4 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 3626539248.8-3689641108.2 ns)
  3626539248.8 |####################
  3629694341.8 |####################
  3632849434.7 |
  3636004527.7 |
  3639159620.7 |
  3642314713.7 |
  3645469806.6 |
  3648624899.6 |
  3651779992.6 |
  3654935085.5 |
  3658090178.5 |########################################
  3661245271.5 |
  3664400364.4 |
  3667555457.4 |####################
  3670710550.4 |
  3673865643.3 |
  3677020736.3 |
  3680175829.3 |
  3683330922.3 |
  3686486015.2 |
  (0 below, 1 above range)

```
