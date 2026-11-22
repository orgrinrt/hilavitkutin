# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (8.04 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 331 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 109% faster than the next best (popcount_swar)

popcount_std (331 ns) leads popcount_swar (692 ns) by 109%, a clear separation rather than a photo finish. CV 77.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 96% (significant)

popcount_std is -7.71 us (96%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 24.3x slower than the field

popcount_handrolled (8.04 us) is 24.3x the fastest (331 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 77.5%)

popcount_std wins on median (331 ns) yet has the highest variance (CV 77.5%), while popcount_handrolled is the steadiest (CV 7.8%, 8.04 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 24.3x the fastest

Fastest popcount_std (331 ns) to slowest popcount_handrolled (8.04 us): 24.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_std is inconsistent: worst-20% is 3.3x its best-20%

popcount_std's best 20% of batches run at 271 ns but its worst 20% at 883 ns (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 331.1 ns median (-95.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 24.28x (fastest 331.1 ns, slowest 8040.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Popcount lowering: std count_ones vs Kernighan loop vs SWAR |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x24680123456789ab |
| Counter frequency | 2112756608 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| popcount_handrolled | 9563ns | 9943ns | 8207ns | 9817ns | 10157ns | base |
| popcount_std | 2891ns | 2216ns | 1816ns | 2217ns | 5986ns | -69.77% |
| popcount_swar | 3050ns | 2577ns | 2112ns | 2574ns | 5415ns | -68.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 7741ns | 6649ns | 8240ns | base | 0.529 |
| popcount_std | 430ns | 271ns | 883ns | -94.45% | 9.535 |
| popcount_swar | 817ns | 568ns | 1444ns | -89.44% | 5.013 |

## Performance model

- Peak throughput: **15.105 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 0.509 | 3.4% |
| popcount_std | 12.369 | 81.9% |
| popcount_swar | 5.919 | 39.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 9563ns | 9563ns | base |
| popcount_std | 2891ns | 2891ns | -69.77% |
| popcount_swar | 3050ns | 3050ns | -68.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 8040ns | base | --- | [8022, 8065] | --- | --- | --- | --- |
| popcount_std | 331ns | -7709.5ns (-95.9%) | [-7725, -7688]ns | [331, 331] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 692ns | -7337.0ns (-91.3%) | [-7370, -7305]ns | [692, 692] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 6636ns | -95.0% | -91.5% |
| 1 | 2 | 8021ns | -95.9% | -91.4% |
| 1 | 3 | 6680ns | -95.0% | -89.6% |
| 1 | 4 | 8272ns | -96.0% | -91.6% |
| 1 | 5 | 8048ns | -95.9% | -91.4% |
| 1 | 6 | 8117ns | -95.9% | -91.5% |
| 1 | 7 | 8065ns | -96.6% | -93.0% |
| 1 | 8 | 8014ns | -96.6% | -91.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.216 | moderate- |
| popcount_std | 0.459 | moderate+ |
| popcount_swar | -0.216 | moderate- |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 26.1ns | 7741.0ns | 0.3% |  |
| popcount_std | 31.2ns | 429.6ns | 7.3% | HIGH |
| popcount_swar | 28.7ns | 817.1ns | 3.5% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 6649.5-8240.0 ns)
   6649.5 |###################
   6729.0 |
   6808.5 |
   6888.1 |
   6967.6 |
   7047.1 |
   7126.7 |
   7206.2 |
   7285.7 |
   7365.2 |
   7444.8 |
   7524.3 |
   7603.8 |
   7683.3 |
   7762.9 |
   7842.4 |
   7921.9 |###############
   8001.4 |########################################
   8081.0 |#########################
   8160.5 |##########
  (17 below, 18 above range)

popcount_std (n=160, range 271.2-883.3 ns)
    271.2 |
    301.8 |########################################
    332.4 |
    363.0 |
    393.6 |
    424.2 |
    454.8 |
    485.4 |
    516.0 |
    546.6 |
    577.2 |
    607.9 |
    638.5 |
    669.1 |
    699.7 |
    730.3 |
    760.9 |
    791.5 |
    822.1 |
    852.7 |
  (30 below, 22 above range)

popcount_swar (n=160, range 567.6-1444.2 ns)
    567.6 |######
    611.5 |
    655.3 |########################################
    699.1 |####
    742.9 |
    786.8 |
    830.6 |
    874.4 |
    918.3 |
    962.1 |
   1005.9 |
   1049.7 |
   1093.6 |
   1137.4 |
   1181.2 |
   1225.0 |
   1268.9 |
   1312.7 |
   1356.5 |
   1400.4 |
  (18 below, 16 above range)

```

## Diagnostics

- **popcount_std**: CV=59.7% (high variance, measurements may be unstable)
- **popcount_std**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **popcount_swar**: CV=51.6% (high variance, measurements may be unstable)
