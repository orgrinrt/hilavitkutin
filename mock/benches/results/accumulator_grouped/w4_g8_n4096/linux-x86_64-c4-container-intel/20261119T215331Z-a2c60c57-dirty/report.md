# W = 4, G = 8

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (grouped) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline grouped has the worst median (53.77 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sequential at 26.10 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sequential dominates: 71% faster than the next best (per_core)

sequential (26.10 us) leads per_core (44.67 us) by 71%, a clear separation rather than a photo finish. CV 22.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 53% (significant)

sequential is -28.54 us (53%) faster than baseline grouped, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### grouped is an outlier: 2.1x slower than the field

grouped (53.77 us) is 2.1x the fastest (26.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### grouped shows alternating (throttle bounce) (autocorr -0.66)

grouped's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### grouped is inconsistent: worst-20% is 3.1x its best-20%

grouped's best 20% of batches run at 44.99 us but its worst 20% at 139.73 us (3.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 26102.9 ns median (-51.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.06x (fastest 26102.9 ns, slowest 53769.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 8 |
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
| grouped | 80660ns | 54286ns | 45669ns | 52431ns | 140497ns | base |
| per_core | 46930ns | 45072ns | 42318ns | 44173ns | 53372ns | -41.82% |
| sequential | 29253ns | 26787ns | 22984ns | 26516ns | 36493ns | -63.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 80001ns | 44991ns | 139730ns | base | 0.051 |
| per_core | 46408ns | 41925ns | 52740ns | -41.99% | 0.088 |
| sequential | 28688ns | 22361ns | 35880ns | -64.14% | 0.143 |

## Performance model

- Peak throughput: **0.183 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.076 | 41.6% |
| per_core | 0.092 | 50.1% |
| sequential | 0.157 | 85.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 80660ns | 80660ns | base |
| per_core | 46930ns | 46930ns | -41.82% |
| sequential | 29253ns | 29253ns | -63.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 53769ns | base | --- | [49631, 68411] | --- | --- | --- | --- |
| per_core | 44673ns | -9697.6ns (-18.0%) | [-23738, -2304]ns | [42576, 46834] | YES | 0.0309 | 0.0309 | 0 |
| sequential | 26103ns | -28544.3ns (-53.1%) | [-38236, -18129]ns | [24159, 33905] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 53769ns | -18.9% | -53.0% |
| 1 | 2 | 108651ns | -59.7% | -78.8% |
| 1 | 3 | 47682ns | -0.3% | -27.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.661 | HIGH- (thermal bounce) |
| per_core | -0.139 | ok |
| sequential | -0.279 | moderate- |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 116070.1ns | 80001.2ns | 145.1% | HIGH |
| per_core | 45444.1ns | 46408.4ns | 97.9% | HIGH |
| sequential | 28785.8ns | 28688.4ns | 100.3% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 44990.8-139730.3 ns)
  44990.8 |################################
  49727.7 |########################################
  54464.7 |################
  59201.7 |
  63938.7 |########
  68675.7 |########
  73412.6 |
  78149.6 |
  82886.6 |
  87623.6 |
  92360.5 |
  97097.5 |
  101834.5 |
  106571.5 |
  111308.5 |
  116045.4 |########
  120782.4 |
  125519.4 |
  130256.4 |
  134993.4 |
  (1 below, 3 above range)

per_core (n=18, range 41924.9-52740.5 ns)
  41924.9 |########################################
  42465.7 |####################
  43006.5 |
  43547.3 |##########
  44088.0 |##########
  44628.8 |####################
  45169.6 |##########
  45710.4 |##########
  46251.2 |
  46791.9 |
  47332.7 |####################
  47873.5 |##########
  48414.3 |
  48955.0 |
  49495.8 |
  50036.6 |##########
  50577.4 |
  51118.2 |
  51658.9 |
  52199.7 |
  (1 below, 1 above range)

sequential (n=18, range 22361.0-35879.8 ns)
  22361.0 |########################################
  23036.9 |####################
  23712.9 |########################################
  24388.8 |####################
  25064.7 |####################
  25740.7 |########################################
  26416.6 |
  27092.6 |####################
  27768.5 |
  28444.4 |
  29120.4 |
  29796.3 |
  30472.3 |
  31148.2 |
  31824.2 |
  32500.1 |####################
  33176.0 |####################
  33852.0 |####################
  34527.9 |########################################
  35203.9 |
  (1 below, 2 above range)

```

## Diagnostics

- **grouped**: CV=74.0% (high variance, measurements may be unstable)
- **grouped**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **grouped**: bridge=106.3% of algo (FFI overhead may distort results)
- **per_core**: bridge=99.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=104.2% of algo (FFI overhead may distort results)
