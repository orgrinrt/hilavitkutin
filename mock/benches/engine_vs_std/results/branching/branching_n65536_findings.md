# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (engine) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline engine has the worst median (63.73 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest std at 31.73 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### std dominates: 101% faster than the next best (engine)

std (31.73 us) leads engine (63.73 us) by 101%, a clear separation rather than a photo finish. CV 7.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std beats baseline by 48% (significant)

std is -30.36 us (48%) faster than baseline engine, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### engine shows alternating (throttle bounce) (autocorr -0.50)

engine's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### engine is inconsistent: worst-20% is 1.7x its best-20%

engine's best 20% of batches run at 41.31 us but its worst 20% at 72.01 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std** at 31729.1 ns median (-50.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.01x (fastest 31729.1 ns, slowest 63733.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | branching, two transforms joined by a third |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000002 |
| Counter frequency | 2300408230 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 61061ns | 64673ns | 41662ns | 51589ns | 73015ns | base |
| std | 33255ns | 32225ns | 31238ns | 31985ns | 34711ns | -45.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 60310ns | 41314ns | 72014ns | base | 1.087 |
| std | 32808ns | 30938ns | 34177ns | -45.60% | 1.998 |

## Performance model

- Peak throughput: **2.118 Gops/s** (std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 1.028 | 48.5% |
| std | 2.065 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 61061ns | 61061ns | base |
| std | 33255ns | 33255ns | -45.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 63734ns | base | --- | [44482, 71892] | --- | --- | --- | --- |
| std | 31729ns | -30363.1ns (-47.6%) | [-37885, -12813]ns | [31456, 33646] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | std |
|---|---|---|---|
| 1 | 1 | 71456ns | -54.3% |
| 1 | 2 | 47493ns | -33.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| std | -0.500 | moderate- |

**Consistency summary:**

- **std**: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 292166.2ns | 60310.4ns | 484.4% | HIGH |
| std | 291312.5ns | 32807.7ns | 887.9% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 41313.7-72014.5 ns)
  41313.7 |####################
  42848.7 |
  44383.8 |####################
  45918.8 |
  47453.9 |
  48988.9 |####################
  50523.9 |
  52059.0 |
  53594.0 |
  55129.1 |
  56664.1 |####################
  58199.1 |
  59734.2 |
  61269.2 |
  62804.3 |
  64339.3 |
  65874.3 |
  67409.4 |
  68944.4 |####################
  70479.5 |########################################
  (0 below, 1 above range)

std (n=8, range 30937.5-34176.9 ns)
  30937.5 |####################
  31099.5 |
  31261.4 |
  31423.4 |####################
  31585.4 |########################################
  31747.4 |####################
  31909.3 |
  32071.3 |
  32233.3 |
  32395.3 |
  32557.2 |
  32719.2 |
  32881.2 |####################
  33043.1 |
  33205.1 |
  33367.1 |
  33529.1 |####################
  33691.0 |
  33853.0 |
  34015.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **engine**: CV=21.0% (high variance, measurements may be unstable)
- **engine**: bridge=459.0% of algo (FFI overhead may distort results)
- **std**: bridge=907.9% of algo (FFI overhead may distort results)
