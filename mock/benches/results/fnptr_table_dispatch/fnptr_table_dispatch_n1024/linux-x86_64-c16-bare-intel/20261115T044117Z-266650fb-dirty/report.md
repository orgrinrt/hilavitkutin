# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (fpt_match_16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline fpt_match_16 has the worst median (181 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest fpt_table_16 at 178 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### fpt_table_16 is fastest but the noisiest (CV 85.6%)

fpt_table_16 wins on median (178 ns) yet has the highest variance (CV 85.6%), while fpt_match_16 is the steadiest (CV 82.6%, 181 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fpt_match_16 shows alternating (throttle bounce) (autocorr -0.57)

fpt_match_16's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4 ns) is smaller than the fastest variant's own run-to-run std-dev (152 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader fpt_table_16 vs stability leader fpt_match_16 (+2% speed for 1.0x steadier)

fpt_table_16 is fastest (178 ns, CV 85.6%); fpt_match_16 gives up 2.1% median for 1.0x lower variance (CV 82.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### fpt_table_16 is inconsistent: worst-20% is 3.6x its best-20%

fpt_table_16's best 20% of batches run at 146 ns but its worst 20% at 524 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### fpt_table_16's edge over baseline is significant but tiny (-2 ns, 0.96%)

fpt_table_16 differs from baseline fpt_match_16 by -2 ns (0.96%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: fpt_table_16** at 177.7 ns median (-2.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.02x (fastest 177.7 ns, slowest 181.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fn-pointer table vs match dispatch for 16-variant hot loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x478ab261e0d9f352 |
| Counter frequency | 2112822230 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fpt_match_16 | 2684ns | 2063ns | 1686ns | 1949ns | 5888ns | base |
| fpt_table_16 | 2780ns | 2056ns | 1693ns | 2059ns | 6031ns | +3.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 237ns | 147ns | 518ns | base | 4.329 |
| fpt_table_16 | 242ns | 146ns | 524ns | +2.40% | 4.228 |

## Performance model

- Peak throughput: **7.018 Gops/s** (fpt_table_16; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 5.643 | 80.4% |
| fpt_table_16 | 5.763 | 82.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2684ns | 2684ns | base |
| fpt_table_16 | 2780ns | 2780ns | +3.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 181ns | base | --- | [180, 183] | --- | --- | --- | --- |
| fpt_table_16 | 178ns | -1.7ns (-1.0%) | [-3, -1]ns | [177, 178] | YES | 0.0089 | 0.0089 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 146ns | -0.2% |
| 1 | 2 | 183ns | -0.5% |
| 1 | 3 | 149ns | +18.4% |
| 1 | 4 | 180ns | -1.4% |
| 1 | 5 | 150ns | +27.7% |
| 1 | 6 | 183ns | -3.3% |
| 1 | 7 | 185ns | -4.0% |
| 1 | 8 | 149ns | -2.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.566 | HIGH- (thermal bounce) |
| fpt_table_16 | -0.031 | ok |

**Consistency summary:**

- **fpt_table_16**: won 6/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 31.7ns | 236.5ns | 13.4% | HIGH |
| fpt_table_16 | 33.2ns | 242.2ns | 13.7% | HIGH |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 146.8-518.2 ns)
    146.8 |########################################
    165.4 |######################################
    183.9 |##################
    202.5 |
    221.1 |
    239.7 |
    258.2 |
    276.8 |
    295.4 |
    313.9 |
    332.5 |
    351.1 |
    369.6 |
    388.2 |
    406.8 |
    425.3 |#
    443.9 |#
    462.5 |##
    481.0 |#
    499.6 |#
  (15 below, 17 above range)

fpt_table_16 (n=160, range 145.9-523.8 ns)
    145.9 |
    164.8 |########################################
    183.7 |#######
    202.6 |
    221.5 |
    240.4 |
    259.3 |
    278.2 |
    297.1 |
    316.0 |
    334.8 |
    353.7 |
    372.6 |
    391.5 |
    410.4 |
    429.3 |
    448.2 |
    467.1 |
    486.0 |#
    504.9 |
  (29 below, 16 above range)

```

## Diagnostics

- **fpt_match_16**: CV=63.4% (high variance, measurements may be unstable)
- **fpt_match_16**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **fpt_match_16**: bridge=13.4% of algo (FFI overhead may distort results)
- **fpt_table_16**: CV=62.8% (high variance, measurements may be unstable)
- **fpt_table_16**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **fpt_table_16**: bridge=13.7% of algo (FFI overhead may distort results)
