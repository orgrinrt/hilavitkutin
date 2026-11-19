# W = 2, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (grouped) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline grouped has the worst median (32.25 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sequential at 25.54 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sequential dominates: 23% faster than the next best (per_core)

sequential (25.54 us) leads per_core (31.34 us) by 23%, a clear separation rather than a photo finish. CV 5.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### per_core shows alternating (throttle bounce) (autocorr -0.66)

per_core's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: sequential** at 25537.0 ns median (-20.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.26x (fastest 25537.0 ns, slowest 32254.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 2, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100451752 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 34841ns | 33325ns | 28214ns | 32263ns | 42021ns | base |
| per_core | 32644ns | 32356ns | 30093ns | 31579ns | 35516ns | -6.31% |
| sequential | 26482ns | 26450ns | 24544ns | 25938ns | 28268ns | -23.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 34043ns | 27844ns | 41153ns | base | 0.120 |
| per_core | 32032ns | 29673ns | 34856ns | -5.91% | 0.128 |
| sequential | 25932ns | 24134ns | 27774ns | -23.83% | 0.158 |

## Performance model

- Peak throughput: **0.170 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.127 | 74.8% |
| per_core | 0.131 | 77.0% |
| sequential | 0.160 | 94.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 34841ns | 34841ns | base |
| per_core | 32644ns | 32644ns | -6.31% |
| sequential | 26482ns | 26482ns | -23.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 32255ns | base | --- | [30156, 35780] | --- | --- | --- | --- |
| per_core | 31341ns | no significant difference | [-2842, +641]ns | [30261, 33895] | no | 1.0000 | 1.0000 | 0 |
| sequential | 25537ns | -5892.7ns (-18.3%) | [-9690, -4220]ns | [24906, 26868] | YES | 0.0003 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 31893ns | -3.2% | -20.1% |
| 1 | 2 | 36959ns | -6.9% | -27.3% |
| 1 | 3 | 29255ns | +3.6% | -16.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.591 | HIGH- (thermal bounce) |
| per_core | -0.656 | HIGH- (thermal bounce) |
| sequential | -0.543 | HIGH- (thermal bounce) |

**Consistency summary:**

- **per_core**: won 2/3, lost 1/3
- **sequential**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 43126.2ns | 34043.4ns | 126.7% | HIGH |
| per_core | 33565.1ns | 32031.9ns | 104.8% | HIGH |
| sequential | 26998.3ns | 25932.4ns | 104.1% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 27844.3-41153.4 ns)
  27844.3 |
  28509.7 |
  29175.2 |########################################
  29840.6 |
  30506.1 |########################################
  31171.5 |#############
  31837.0 |
  32502.5 |##########################
  33167.9 |#############
  33833.4 |
  34498.8 |#############
  35164.3 |
  35829.7 |
  36495.2 |#############
  37160.6 |
  37826.1 |
  38491.6 |#############
  39157.0 |#############
  39822.5 |
  40487.9 |
  (2 below, 2 above range)

per_core (n=18, range 29672.9-34856.3 ns)
  29672.9 |#############
  29932.0 |########################################
  30191.2 |#############
  30450.4 |##########################
  30709.6 |
  30968.7 |
  31227.9 |##########################
  31487.1 |
  31746.3 |#############
  32005.4 |
  32264.6 |#############
  32523.8 |
  32783.0 |
  33042.1 |
  33301.3 |
  33560.5 |
  33819.7 |########################################
  34078.8 |
  34338.0 |
  34597.2 |#############
  (1 below, 2 above range)

sequential (n=18, range 24133.6-27773.9 ns)
  24133.6 |##########################
  24315.6 |#############
  24497.7 |#############
  24679.7 |
  24861.7 |
  25043.7 |
  25225.7 |##########################
  25407.7 |########################################
  25589.7 |#############
  25771.8 |
  25953.8 |##########################
  26135.8 |
  26317.8 |
  26499.8 |
  26681.8 |
  26863.8 |
  27045.8 |
  27227.9 |
  27409.9 |
  27591.9 |#############
  (1 below, 4 above range)

```

## Diagnostics

- **grouped**: bridge=105.3% of algo (FFI overhead may distort results)
- **per_core**: bridge=107.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=103.5% of algo (FFI overhead may distort results)
