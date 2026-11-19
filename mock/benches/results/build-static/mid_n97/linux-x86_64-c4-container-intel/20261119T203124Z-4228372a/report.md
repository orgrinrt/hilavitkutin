# build-static: rebuilt after an edit to one mid contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (33358.81 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 2549.14 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 1209% faster than the next best (llvm_non_incremental)

llvm_incremental (2549.14 ms) leads llvm_non_incremental (33358.81 ms) by 1209%, a clear separation rather than a photo finish. CV 7.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 93% (significant)

llvm_incremental is -30865.36 ms (93%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### llvm_incremental is fastest but the noisiest (CV 7.3%)

llvm_incremental wins on median (2549.14 ms) yet has the highest variance (CV 7.3%), while llvm_non_incremental is the steadiest (CV 1.8%, 33358.81 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### llvm_incremental shows alternating (throttle bounce) (autocorr -0.59)

llvm_incremental's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 13.1x the fastest

Fastest llvm_incremental (2549.14 ms) to slowest llvm_non_incremental (33358.81 ms): 13.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 2549141687.8 ns median (-92.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 13.09x (fastest 2549141687.8 ns, slowest 33358813979.3 ns)

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
| Counter frequency | 2100474318 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 2619643759ns | 2549146447ns | 2409983763ns | 2528810964ns | 2860722951ns | -92.18% |
| llvm_non_incremental | 33509123692ns | 33358818448ns | 32834250612ns | 33210371242ns | 34294688905ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 2619639181ns | 2409979215ns | 2860718395ns | -92.18% |
| llvm_non_incremental | 33509119281ns | 32834245550ns | 34294684817ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 2619643759ns | 2619643759ns | -92.18% |
| llvm_non_incremental | 33509123692ns | 33509123692ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 33358813979ns | base | --- | [32873859047, 34294684817] | --- | --- | --- | --- |
| llvm_incremental | 2549141688ns | -30865356823.2ns (-92.5%) | [-31433966422, -30369117054]ns | [2449057461, 2860718395] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 33472506543ns | -92.3% |
| 1 | 2 | 34060251431ns | -91.9% |
| 1 | 3 | 32913472544ns | -92.3% |
| 1 | 4 | 32834245550ns | -92.4% |
| 1 | 5 | 34529118204ns | -91.4% |
| 1 | 6 | 33245121416ns | -92.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.590 | HIGH- (thermal bounce) |
| llvm_non_incremental | -0.406 | moderate- |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 307106155.4ns | 2619639181.4ns | 11.7% | HIGH |
| llvm_non_incremental | 307506032.1ns | 33509119281.2ns | 0.9% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 2409979215.4-2860718395.2 ns)
  2409979215.4 |########################################
  2432516174.4 |
  2455053133.4 |
  2477590092.4 |########################################
  2500127051.3 |########################################
  2522664010.3 |
  2545200969.3 |
  2567737928.3 |########################################
  2590274887.3 |
  2612811846.3 |
  2635348805.3 |
  2657885764.3 |
  2680422723.2 |
  2702959682.2 |
  2725496641.2 |
  2748033600.2 |########################################
  2770570559.2 |
  2793107518.2 |
  2815644477.2 |
  2838181436.2 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 32834245550.0-34294684817.4 ns)
  32834245550.0 |########################################
  32907267513.4 |########################################
  32980289476.7 |
  33053311440.1 |
  33126333403.5 |
  33199355366.8 |########################################
  33272377330.2 |
  33345399293.6 |
  33418421257.0 |########################################
  33491443220.3 |
  33564465183.7 |
  33637487147.1 |
  33710509110.4 |
  33783531073.8 |
  33856553037.2 |
  33929575000.6 |
  34002596963.9 |########################################
  34075618927.3 |
  34148640890.7 |
  34221662854.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **llvm_incremental**: bridge=11.7% of algo (FFI overhead may distort results)
