# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (engine) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline engine has the worst median (8.59 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest std at 6.01 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### std dominates: 43% faster than the next best (engine)

std (6.01 ms) leads engine (8.59 ms) by 43%, a clear separation rather than a photo finish. CV 7.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std beats baseline by 29% (significant)

std is -2.52 ms (29%) faster than baseline engine, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std shows alternating (throttle bounce) (autocorr -0.50)

std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### engine is inconsistent: worst-20% is 1.6x its best-20%

engine's best 20% of batches run at 6.27 ms but its worst 20% at 10.22 ms (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std** at 6007373.1 ns median (-30.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.43x (fastest 6007373.1 ns, slowest 8586404.8 ns)

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
| engine | 8861062ns | 8592305ns | 6274817ns | 7909671ns | 10221167ns | base |
| std | 6075841ns | 6012835ns | 5516220ns | 5795944ns | 6425669ns | -31.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 8855191ns | 6270031ns | 10215044ns | base | 0.474 |
| std | 6070645ns | 5511256ns | 6420309ns | -31.45% | 0.691 |

## Performance model

- Peak throughput: **0.761 Gops/s** (std; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.488 | 64.2% |
| std | 0.698 | 91.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 8861062ns | 8861062ns | base |
| std | 6075841ns | 6075841ns | -31.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 8586405ns | base | --- | [7219712, 10884263] | --- | --- | --- | --- |
| std | 6007373ns | -2515370.5ns (-29.3%) | [-4241989, -1630764]ns | [5588948, 6642274] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | std |
|---|---|---|---|
| 1 | 1 | 10396773ns | -39.2% |
| 1 | 2 | 7576946ns | -23.5% |

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
| engine | 22467536.1ns | 8855190.7ns | 253.7% | HIGH |
| std | 22000331.6ns | 6070644.5ns | 362.4% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 6270031.1-10215043.7 ns)
  6270031.1 |####################
  6467281.7 |
  6664532.4 |
  6861783.0 |
  7059033.6 |####################
  7256284.2 |
  7453534.9 |
  7650785.5 |
  7848036.1 |####################
  8045286.8 |
  8242537.4 |
  8439788.0 |########################################
  8637038.6 |
  8834289.3 |
  9031539.9 |
  9228790.5 |
  9426041.2 |
  9623291.8 |
  9820542.4 |####################
  10017793.0 |
  (0 below, 2 above range)

std (n=8, range 5511255.6-6420308.8 ns)
  5511255.6 |####################
  5556708.3 |####################
  5602160.9 |
  5647613.6 |
  5693066.2 |
  5738518.9 |####################
  5783971.6 |
  5829424.2 |
  5874876.9 |
  5920329.6 |
  5965782.2 |########################################
  6011234.9 |
  6056687.5 |
  6102140.2 |
  6147592.9 |
  6193045.5 |
  6238498.2 |####################
  6283950.8 |
  6329403.5 |
  6374856.2 |
  (0 below, 2 above range)

```

## Diagnostics

- **engine**: bridge=259.2% of algo (FFI overhead may distort results)
- **std**: bridge=365.4% of algo (FFI overhead may distort results)
