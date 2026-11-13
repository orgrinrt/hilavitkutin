# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (engine) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline engine has the worst median (1.00 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest std at 727.31 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### std dominates: 38% faster than the next best (engine)

std (727.31 us) leads engine (1.00 ms) by 38%, a clear separation rather than a photo finish. CV 7.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std beats baseline by 29% (significant)

std is -295.19 us (29%) faster than baseline engine, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std shows alternating (throttle bounce) (autocorr -0.50)

std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: std** at 727314.1 ns median (-27.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.38x (fastest 727314.1 ns, slowest 1001836.4 ns)

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
| engine | 1137340ns | 1004324ns | 916260ns | 997980ns | 1297130ns | base |
| std | 723692ns | 730282ns | 643995ns | 684039ns | 773355ns | -36.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 1134132ns | 909156ns | 1294463ns | base | 0.925 |
| std | 721061ns | 642023ns | 770584ns | -36.42% | 1.454 |

## Performance model

- Peak throughput: **1.633 Gops/s** (std; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 1.047 | 64.1% |
| std | 1.442 | 88.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 1137340ns | 1137340ns | base |
| std | 723692ns | 723692ns | -36.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 1001836ns | base | --- | [992991, 1275117] | --- | --- | --- | --- |
| std | 727314ns | -295193.6ns (-29.5%) | [-490796, -232626]ns | [650463, 784321] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | std |
|---|---|---|---|
| 1 | 1 | 996526ns | -24.6% |
| 1 | 2 | 1140251ns | -41.6% |

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
| engine | 4955176.5ns | 1134131.5ns | 436.9% | HIGH |
| std | 4973593.5ns | 721060.8ns | 689.8% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 909156.3-1294462.9 ns)
  909156.3 |##########
  928421.6 |
  947687.0 |
  966952.3 |
  986217.6 |########################################
  1005483.0 |
  1024748.3 |
  1044013.6 |##########
  1063279.0 |
  1082544.3 |
  1101809.6 |
  1121075.0 |
  1140340.3 |
  1159605.6 |
  1178871.0 |
  1198136.3 |
  1217401.6 |
  1236667.0 |
  1255932.3 |##########
  1275197.6 |
  (0 below, 1 above range)

std (n=8, range 642023.1-770583.5 ns)
  642023.1 |########################################
  648451.1 |########################################
  654879.1 |
  661307.2 |
  667735.2 |
  674163.2 |
  680591.2 |########################################
  687019.2 |
  693447.3 |
  699875.3 |
  706303.3 |########################################
  712731.3 |
  719159.4 |
  725587.4 |
  732015.4 |
  738443.4 |########################################
  744871.4 |
  751299.5 |
  757727.5 |########################################
  764155.5 |
  (0 below, 2 above range)

```

## Diagnostics

- **engine**: CV=25.3% (high variance, measurements may be unstable)
- **engine**: bridge=488.6% of algo (FFI overhead may distort results)
- **std**: bridge=682.4% of algo (FFI overhead may distort results)
