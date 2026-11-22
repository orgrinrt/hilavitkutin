# build-static: rebuilt after an edit to one leaf implementation crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (4663.18 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 3244.23 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 44% faster than the next best (llvm_non_incremental)

llvm_incremental (3244.23 ms) leads llvm_non_incremental (4663.18 ms) by 44%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 31% (significant)

llvm_incremental is -1423.87 ms (31%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### llvm_incremental shows alternating (throttle bounce) (autocorr -0.55)

llvm_incremental's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: llvm_incremental** at 3244233083.3 ns median (-30.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.44x (fastest 3244233083.3 ns, slowest 4663179083.4 ns)

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
| llvm_incremental | 3285536389ns | 3244234854ns | 3174665208ns | 3235997875ns | 3415279750ns | -29.73% |
| llvm_non_incremental | 4675900674ns | 4663180812ns | 4630012167ns | 4655748500ns | 4729073188ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 3285534125ns | 3174663083ns | 3415276562ns | -29.73% |
| llvm_non_incremental | 4675898257ns | 4630008750ns | 4729071000ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 3285536389ns | 3285536389ns | -29.73% |
| llvm_non_incremental | 4675900674ns | 4675900674ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 4663179083ns | base | --- | [4635444688, 4729071000] | --- | --- | --- | --- |
| llvm_incremental | 3244233083ns | -1423873062.5ns (-30.5%) | [-1514442458, -1232776875]ns | [3197092729, 3415276562] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 4640880625ns | -29.6% |
| 1 | 2 | 4660260042ns | -31.9% |
| 1 | 3 | 4666098125ns | -24.5% |
| 1 | 4 | 4762810333ns | -32.4% |
| 1 | 5 | 4630008750ns | -28.5% |
| 1 | 6 | 4695331667ns | -31.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.551 | HIGH- (thermal bounce) |
| llvm_non_incremental | -0.434 | moderate- |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 344578298.6ns | 3285534125.0ns | 10.5% | HIGH |
| llvm_non_incremental | 324487701.4ns | 4675898256.9ns | 6.9% | HIGH |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 3174663083.3-3415276562.5 ns)
  3174663083.3 |####################
  3186693757.3 |
  3198724431.2 |
  3210755105.2 |########################################
  3222785779.1 |
  3234816453.1 |
  3246847127.1 |
  3258877801.0 |####################
  3270908475.0 |
  3282939148.9 |
  3294969822.9 |
  3307000496.9 |####################
  3319031170.8 |
  3331061844.8 |
  3343092518.7 |
  3355123192.7 |
  3367153866.7 |
  3379184540.6 |
  3391215214.6 |
  3403245888.5 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 4630008750.0-4729071000.0 ns)
  4630008750.0 |########################################
  4634961862.5 |
  4639914975.0 |########################################
  4644868087.5 |
  4649821200.0 |
  4654774312.5 |
  4659727425.0 |########################################
  4664680537.5 |########################################
  4669633650.0 |
  4674586762.5 |
  4679539875.0 |
  4684492987.5 |
  4689446100.0 |
  4694399212.5 |########################################
  4699352325.0 |
  4704305437.5 |
  4709258550.0 |
  4714211662.5 |
  4719164775.0 |
  4724117887.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **llvm_incremental**: bridge=10.6% of algo (FFI overhead may distort results)
