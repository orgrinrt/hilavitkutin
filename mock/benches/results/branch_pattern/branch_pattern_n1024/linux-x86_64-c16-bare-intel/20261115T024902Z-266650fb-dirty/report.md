# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (1.71 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 861 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### branch_if dominates: 99% faster than the next best (branch_cmov)

branch_if (861 ns) leads branch_cmov (1.71 us) by 99%, a clear separation rather than a photo finish. CV 61.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### branch_if beats baseline by 50% (significant)

branch_if is -850 ns (50%) faster than baseline branch_cmov, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### branch_if is fastest but the noisiest (CV 61.1%)

branch_if wins on median (861 ns) yet has the highest variance (CV 61.1%), while branch_cmov is the steadiest (CV 27.3%, 1.71 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### branch_if shows alternating (throttle bounce) (autocorr -0.54)

branch_if's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### branch_if is inconsistent: worst-20% is 2.6x its best-20%

branch_if's best 20% of batches run at 710 ns but its worst 20% at 1.82 us (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: branch_if** at 860.8 ns median (-49.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.99x (fastest 860.8 ns, slowest 1714.0 ns)

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
| Counter frequency | 2112793168 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| branch_cmov | 3639ns | 3595ns | 2941ns | 3569ns | 4547ns | base |
| branch_if | 3207ns | 2736ns | 2246ns | 2652ns | 5833ns | -11.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 1734ns | 1403ns | 2160ns | base | 0.591 |
| branch_if | 1005ns | 710ns | 1821ns | -42.02% | 1.019 |

## Performance model

- Peak throughput: **1.442 Gops/s** (branch_if; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.597 | 41.4% |
| branch_if | 1.190 | 82.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 3639ns | 3639ns | base |
| branch_if | 3207ns | 3207ns | -11.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 1714ns | base | --- | [1711, 1716] | --- | --- | --- | --- |
| branch_if | 861ns | -850.1ns (-49.6%) | [-855, -847]ns | [859, 861] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 1717ns | -51.1% |
| 1 | 2 | 1715ns | -49.8% |
| 1 | 3 | 1722ns | -58.9% |
| 1 | 4 | 1715ns | -49.6% |
| 1 | 5 | 1403ns | -49.1% |
| 1 | 6 | 1715ns | -49.7% |
| 1 | 7 | 1717ns | -50.0% |
| 1 | 8 | 1405ns | -49.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.201 | moderate- |
| branch_if | -0.536 | HIGH- (thermal bounce) |

**Consistency summary:**

- **branch_if**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 24.1ns | 1734.0ns | 1.4% |  |
| branch_if | 27.7ns | 1005.3ns | 2.8% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 1402.9-2160.5 ns)
   1402.9 |######
   1440.8 |
   1478.7 |
   1516.5 |
   1554.4 |
   1592.3 |
   1630.2 |
   1668.1 |#
   1705.9 |########################################
   1743.8 |
   1781.7 |
   1819.6 |
   1857.5 |
   1895.3 |
   1933.2 |
   1971.1 |
   2009.0 |
   2046.9 |
   2084.7 |
   2122.6 |
  (18 below, 7 above range)

branch_if (n=160, range 709.9-1820.9 ns)
    709.9 |###############
    765.4 |
    821.0 |########################################
    876.5 |#
    932.1 |
    987.6 |
   1043.2 |
   1098.7 |
   1154.3 |
   1209.8 |
   1265.4 |
   1320.9 |
   1376.5 |
   1432.0 |
   1487.6 |
   1543.1 |
   1598.7 |
   1654.2 |
   1709.8 |
   1765.3 |
  (16 below, 18 above range)

```

## Diagnostics

- **branch_cmov**: CV=27.0% (high variance, measurements may be unstable)
- **branch_if**: CV=52.3% (high variance, measurements may be unstable)
