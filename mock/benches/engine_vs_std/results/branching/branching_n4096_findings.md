# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (engine) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline engine has the worst median (13.82 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest std at 1.95 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### std dominates: 607% faster than the next best (engine)

std (1.95 us) leads engine (13.82 us) by 607%, a clear separation rather than a photo finish. CV 0.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std beats baseline by 86% (significant)

std is -11.87 us (86%) faster than baseline engine, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### engine shows alternating (throttle bounce) (autocorr -0.50)

engine's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 7.1x the fastest

Fastest std (1.95 us) to slowest engine (13.82 us): 7.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### engine is inconsistent: worst-20% is 1.5x its best-20%

engine's best 20% of batches run at 11.10 us but its worst 20% at 16.73 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std** at 1954.2 ns median (-85.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 7.07x (fastest 1954.2 ns, slowest 13822.5 ns)

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
| engine | 14296ns | 14074ns | 11222ns | 11676ns | 17029ns | base |
| std | 2124ns | 2098ns | 2061ns | 2070ns | 2180ns | -85.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 14071ns | 11098ns | 16732ns | base | 0.291 |
| std | 1955ns | 1951ns | 1957ns | -86.11% | 2.095 |

## Performance model

- Peak throughput: **2.099 Gops/s** (std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.296 | 14.1% |
| std | 2.096 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 14296ns | 14296ns | base |
| std | 2124ns | 2124ns | -85.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 13823ns | base | --- | [11133, 17122] | --- | --- | --- | --- |
| std | 1954ns | -11866.0ns (-85.8%) | [-15170, -9178]ns | [1952, 1960] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | std |
|---|---|---|---|
| 1 | 1 | 11199ns | -82.5% |
| 1 | 2 | 16728ns | -88.3% |

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
| engine | 18340.4ns | 14071.4ns | 130.3% | HIGH |
| std | 18795.6ns | 1955.0ns | 961.4% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 11098.0-16732.4 ns)
  11098.0 |########################################
  11379.7 |
  11661.4 |
  11943.2 |#############
  12224.9 |
  12506.6 |
  12788.3 |
  13070.0 |
  13351.8 |
  13633.5 |
  13915.2 |
  14196.9 |
  14478.6 |
  14760.4 |
  15042.1 |
  15323.8 |#############
  15605.5 |
  15887.2 |
  16169.0 |#############
  16450.7 |
  (0 below, 2 above range)

std (n=8, range 1951.4-1957.3 ns)
   1951.4 |####################
   1951.7 |
   1952.0 |
   1952.3 |####################
   1952.6 |####################
   1952.9 |
   1953.2 |
   1953.5 |
   1953.8 |####################
   1954.1 |
   1954.4 |########################################
   1954.7 |
   1955.0 |
   1955.3 |
   1955.5 |
   1955.8 |
   1956.1 |
   1956.4 |
   1956.7 |
   1957.0 |
  (0 below, 2 above range)

```

## Diagnostics

- **engine**: bridge=130.8% of algo (FFI overhead may distort results)
- **std**: bridge=872.2% of algo (FFI overhead may distort results)
