# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (425 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 221 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### branch_if dominates: 93% faster than the next best (branch_cmov)

branch_if (221 ns) leads branch_cmov (425 ns) by 93%, a clear separation rather than a photo finish. CV 82.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### branch_if beats baseline by 47% (significant)

branch_if is -202 ns (47%) faster than baseline branch_cmov, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### branch_if is fastest but the noisiest (CV 82.1%)

branch_if wins on median (221 ns) yet has the highest variance (CV 82.1%), while branch_cmov is the steadiest (CV 70.4%, 425 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### branch_cmov shows alternating (throttle bounce) (autocorr -0.68)

branch_cmov's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### branch_if is inconsistent: worst-20% is 3.3x its best-20%

branch_if's best 20% of batches run at 182 ns but its worst 20% at 604 ns (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: branch_if** at 220.8 ns median (-48.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.93x (fastest 220.8 ns, slowest 425.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Branchful if/else vs branchless cmov/csel under data-dependent input |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x76543210fedcba98 |
| Counter frequency | 2112827530 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| branch_cmov | 2724ns | 2302ns | 1884ns | 2220ns | 5079ns | base |
| branch_if | 2759ns | 2100ns | 1730ns | 2102ns | 5761ns | +1.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 503ns | 348ns | 937ns | base | 0.509 |
| branch_if | 290ns | 182ns | 604ns | -42.33% | 0.882 |

## Performance model

- Peak throughput: **1.408 Gops/s** (branch_if; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.602 | 42.8% |
| branch_if | 1.159 | 82.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 2724ns | 2724ns | base |
| branch_if | 2759ns | 2759ns | +1.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 425ns | base | --- | [424, 426] | --- | --- | --- | --- |
| branch_if | 221ns | -201.6ns (-47.4%) | [-203, -199]ns | [221, 222] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 427ns | -58.5% |
| 1 | 2 | 349ns | -36.8% |
| 1 | 3 | 427ns | -47.9% |
| 1 | 4 | 349ns | -37.4% |
| 1 | 5 | 426ns | -47.8% |
| 1 | 6 | 349ns | -47.1% |
| 1 | 7 | 425ns | -48.1% |
| 1 | 8 | 426ns | -46.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.679 | HIGH- (thermal bounce) |
| branch_if | -0.185 | ok |

**Consistency summary:**

- **branch_if**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 28.0ns | 503.1ns | 5.6% | HIGH |
| branch_if | 31.3ns | 290.1ns | 10.8% | HIGH |

## Distribution (algo ns)

```
branch_cmov (n=160, range 347.6-937.2 ns)
    347.6 |####################
    377.1 |
    406.6 |########################################
    436.1 |###
    465.5 |
    495.0 |
    524.5 |
    554.0 |
    583.4 |
    612.9 |
    642.4 |
    671.9 |
    701.3 |
    730.8 |
    760.3 |
    789.8 |
    819.2 |
    848.7 |
    878.2 |
    907.7 |
  (11 below, 16 above range)

branch_if (n=160, range 181.8-604.2 ns)
    181.8 |#######
    202.9 |########################################
    224.0 |########
    245.2 |
    266.3 |
    287.4 |
    308.5 |
    329.6 |
    350.7 |
    371.9 |
    393.0 |
    414.1 |
    435.2 |#
    456.3 |
    477.5 |
    498.6 |
    519.7 |
    540.8 |
    561.9 |
    583.0 |
  (16 below, 20 above range)

```

## Diagnostics

- **branch_cmov**: CV=59.5% (high variance, measurements may be unstable)
- **branch_if**: CV=62.5% (high variance, measurements may be unstable)
- **branch_if**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **branch_if**: bridge=10.8% of algo (FFI overhead may distort results)
