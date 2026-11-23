# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (1.70 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 862 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### branch_if dominates: 98% faster than the next best (branch_cmov)

branch_if (862 ns) leads branch_cmov (1.70 us) by 98%, a clear separation rather than a photo finish. CV 49.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### branch_if beats baseline by 49% (significant)

branch_if is -832 ns (49%) faster than baseline branch_cmov, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### branch_if is fastest but the noisiest (CV 49.9%)

branch_if wins on median (862 ns) yet has the highest variance (CV 49.9%), while branch_cmov is the steadiest (CV 27.6%, 1.70 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### branch_cmov shows alternating (throttle bounce) (autocorr -0.66)

branch_cmov's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### branch_if is inconsistent: worst-20% is 2.2x its best-20%

branch_if's best 20% of batches run at 699 ns but its worst 20% at 1.52 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: branch_if** at 862.0 ns median (-49.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.98x (fastest 862.0 ns, slowest 1704.2 ns)

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
| branch_cmov | 3570ns | 3586ns | 2937ns | 3428ns | 4626ns | base |
| branch_if | 3075ns | 2746ns | 2240ns | 2747ns | 4891ns | -13.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 1695ns | 1397ns | 2189ns | base | 0.604 |
| branch_if | 963ns | 699ns | 1524ns | -43.17% | 1.063 |

## Performance model

- Peak throughput: **1.465 Gops/s** (branch_if; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.601 | 41.0% |
| branch_if | 1.188 | 81.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 3570ns | 3570ns | base |
| branch_if | 3075ns | 3075ns | -13.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 1704ns | base | --- | [1689, 1709] | --- | --- | --- | --- |
| branch_if | 862ns | -832.1ns (-48.8%) | [-841, -804]ns | [862, 864] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 1708ns | -59.8% |
| 1 | 2 | 1713ns | -49.7% |
| 1 | 3 | 1403ns | -49.6% |
| 1 | 4 | 1696ns | -49.1% |
| 1 | 5 | 1402ns | -37.7% |
| 1 | 6 | 1712ns | -49.7% |
| 1 | 7 | 1402ns | -38.7% |
| 1 | 8 | 1712ns | -49.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.663 | HIGH- (thermal bounce) |
| branch_if | -0.168 | ok |

**Consistency summary:**

- **branch_if**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 23.7ns | 1695.3ns | 1.4% |  |
| branch_if | 26.6ns | 963.3ns | 2.8% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 1397.5-2189.3 ns)
   1397.5 |#########################
   1437.1 |
   1476.7 |
   1516.2 |
   1555.8 |
   1595.4 |
   1635.0 |##
   1674.6 |########################################
   1714.2 |##################
   1753.8 |
   1793.4 |
   1833.0 |
   1872.5 |
   1912.1 |
   1951.7 |
   1991.3 |
   2030.9 |
   2070.5 |
   2110.1 |
   2149.7 |
  (18 below, 9 above range)

branch_if (n=160, range 698.9-1524.5 ns)
    698.9 |#########
    740.2 |
    781.5 |
    822.7 |########################################
    864.0 |#######################
    905.3 |
    946.6 |
    987.9 |
   1029.1 |
   1070.4 |
   1111.7 |
   1153.0 |
   1194.2 |
   1235.5 |
   1276.8 |
   1318.1 |
   1359.4 |
   1400.6 |
   1441.9 |
   1483.2 |
  (15 below, 14 above range)

```

## Diagnostics

- **branch_cmov**: CV=27.7% (high variance, measurements may be unstable)
- **branch_if**: CV=44.6% (high variance, measurements may be unstable)
