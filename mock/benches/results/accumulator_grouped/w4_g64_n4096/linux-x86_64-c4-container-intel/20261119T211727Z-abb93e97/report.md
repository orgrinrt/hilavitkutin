# W = 4, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (grouped) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline grouped has the worst median (46.93 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sequential at 24.16 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sequential dominates: 85% faster than the next best (per_core)

sequential (24.16 us) leads per_core (44.78 us) by 85%, a clear separation rather than a photo finish. CV 11.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 51% (significant)

sequential is -23.72 us (51%) faster than baseline grouped, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### per_core shows alternating (throttle bounce) (autocorr -0.59)

per_core's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### grouped is inconsistent: worst-20% is 1.6x its best-20%

grouped's best 20% of batches run at 40.20 us but its worst 20% at 63.13 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 24158.8 ns median (-48.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.94x (fastest 24158.8 ns, slowest 46926.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100372148 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 51634ns | 47680ns | 40796ns | 47062ns | 63913ns | base |
| per_core | 47834ns | 45433ns | 41805ns | 44630ns | 55655ns | -7.36% |
| sequential | 25523ns | 24562ns | 22900ns | 24346ns | 28599ns | -50.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 51017ns | 40202ns | 63134ns | base | 0.080 |
| per_core | 47270ns | 40992ns | 55194ns | -7.34% | 0.087 |
| sequential | 24997ns | 22277ns | 27938ns | -51.00% | 0.164 |

## Performance model

- Peak throughput: **0.184 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.087 | 47.5% |
| per_core | 0.091 | 49.8% |
| sequential | 0.170 | 92.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 51634ns | 51634ns | base |
| per_core | 47834ns | 47834ns | -7.36% |
| sequential | 25523ns | 25523ns | -50.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 46926ns | base | --- | [43335, 54197] | --- | --- | --- | --- |
| per_core | 44777ns | no significant difference | [-8695, +2203]ns | [43126, 47860] | no | 0.4807 | 0.4807 | 0 |
| sequential | 24159ns | -23724.1ns (-50.6%) | [-30372, -19475]ns | [23661, 25628] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 43250ns | +2.2% | -44.8% |
| 1 | 2 | 45502ns | +2.0% | -48.5% |
| 1 | 3 | 56496ns | -20.4% | -56.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.085 | ok |
| per_core | -0.587 | HIGH- (thermal bounce) |
| sequential | -0.390 | moderate- |

**Consistency summary:**

- **per_core**: won 1/3, lost 2/3
- **sequential**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 48982.0ns | 51016.6ns | 96.0% | HIGH |
| per_core | 45514.3ns | 47270.4ns | 96.3% | HIGH |
| sequential | 25492.4ns | 24997.3ns | 102.0% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 40202.2-63134.3 ns)
  40202.2 |#############
  41348.8 |#############
  42495.4 |########################################
  43642.0 |
  44788.6 |#############
  45935.2 |##########################
  47081.8 |#############
  48228.4 |
  49375.0 |
  50521.6 |
  51668.2 |##########################
  52814.9 |#############
  53961.5 |#############
  55108.1 |
  56254.7 |
  57401.3 |#############
  58547.9 |#############
  59694.5 |
  60841.1 |
  61987.7 |
  (1 below, 2 above range)

per_core (n=18, range 40991.9-55194.5 ns)
  40991.9 |#############
  41702.0 |
  42412.2 |##########################
  43122.3 |##########################
  43832.4 |#############
  44542.5 |########################################
  45252.7 |#############
  45962.8 |#############
  46672.9 |
  47383.0 |
  48093.2 |
  48803.3 |
  49513.4 |#############
  50223.6 |#############
  50933.7 |
  51643.8 |
  52353.9 |#############
  53064.1 |#############
  53774.2 |
  54484.3 |
  (2 below, 1 above range)

sequential (n=18, range 22276.9-27938.0 ns)
  22276.9 |
  22560.0 |#############
  22843.0 |#############
  23126.1 |
  23409.1 |#############
  23692.2 |########################################
  23975.2 |##########################
  24258.3 |#############
  24541.3 |##########################
  24824.4 |
  25107.4 |
  25390.5 |
  25673.5 |
  25956.6 |
  26239.6 |
  26522.7 |#############
  26805.8 |#############
  27088.8 |#############
  27371.9 |#############
  27654.9 |
  (2 below, 1 above range)

```

## Diagnostics

- **grouped**: CV=21.3% (high variance, measurements may be unstable)
- **grouped**: bridge=100.7% of algo (FFI overhead may distort results)
- **per_core**: bridge=100.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=104.0% of algo (FFI overhead may distort results)
