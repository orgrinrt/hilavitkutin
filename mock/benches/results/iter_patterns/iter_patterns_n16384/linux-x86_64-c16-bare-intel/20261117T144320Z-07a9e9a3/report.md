# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (4.63 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 292 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 683% faster than the next best (iter_seq)

iter_strided (292 ns) leads iter_seq (2.29 us) by 683%, a clear separation rather than a photo finish. CV 78.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 94% (significant)

iter_strided is -4.34 us (94%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 15.8x slower than the field

iter_gather (4.63 us) is 15.8x the fastest (292 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 78.8%)

iter_strided wins on median (292 ns) yet has the highest variance (CV 78.8%), while iter_gather is the steadiest (CV 8.1%, 4.63 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 15.8x the fastest

Fastest iter_strided (292 ns) to slowest iter_gather (4.63 us): 15.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_strided is inconsistent: worst-20% is 3.0x its best-20%

iter_strided's best 20% of batches run at 268 ns but its worst 20% at 804 ns (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 292.3 ns median (-93.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 15.85x (fastest 292.3 ns, slowest 4632.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Column iteration patterns: sequential vs strided vs data-dependent gather |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x3a5c8e91b2d4f607 |
| Counter frequency | 2112828630 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iter_gather | 6241ns | 6523ns | 5336ns | 6431ns | 6579ns | base |
| iter_seq | 4008ns | 4173ns | 3418ns | 4121ns | 4261ns | -35.78% |
| iter_strided | 2902ns | 2180ns | 1991ns | 2187ns | 5957ns | -53.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 4433ns | 3791ns | 4674ns | base | 3.696 |
| iter_seq | 2194ns | 1872ns | 2326ns | -50.50% | 7.466 |
| iter_strided | 390ns | 268ns | 804ns | -91.20% | 41.992 |

## Performance model

- Peak throughput: **61.161 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.537 | 5.8% |
| iter_seq | 7.163 | 11.7% |
| iter_strided | 56.052 | 91.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 6241ns | 6241ns | base |
| iter_seq | 4008ns | 4008ns | -35.78% |
| iter_strided | 2902ns | 2902ns | -53.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 4632ns | base | --- | [4631, 4634] | --- | --- | --- | --- |
| iter_seq | 2287ns | -2345.8ns (-50.6%) | [-2355, -2344]ns | [2287, 2288] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 292ns | -4338.6ns (-93.7%) | [-4339, -4330]ns | [292, 292] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 4639ns | -59.7% | -93.7% |
| 1 | 2 | 4636ns | -50.7% | -93.7% |
| 1 | 3 | 4639ns | -59.7% | -94.8% |
| 1 | 4 | 3790ns | -39.6% | -92.3% |
| 1 | 5 | 4633ns | -50.6% | -93.7% |
| 1 | 6 | 4642ns | -50.7% | -93.4% |
| 1 | 7 | 4638ns | -50.7% | -93.7% |
| 1 | 8 | 3790ns | -39.6% | -92.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.205 | moderate- |
| iter_seq | -0.208 | moderate- |
| iter_strided | -0.063 | ok |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 22.8ns | 4433.4ns | 0.5% |  |
| iter_seq | 22.9ns | 2194.4ns | 1.0% |  |
| iter_strided | 32.0ns | 390.2ns | 8.2% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 3790.8-4673.9 ns)
   3790.8 |#####
   3834.9 |
   3879.1 |
   3923.2 |
   3967.4 |
   4011.5 |
   4055.7 |
   4099.8 |
   4144.0 |
   4188.2 |
   4232.3 |
   4276.5 |
   4320.6 |
   4364.8 |
   4408.9 |
   4453.1 |
   4497.3 |
   4541.4 |
   4585.6 |##
   4629.7 |########################################
  (25 below, 2 above range)

iter_seq (n=160, range 1872.1-2326.0 ns)
   1872.1 |###
   1894.8 |
   1917.5 |
   1940.2 |
   1962.9 |
   1985.6 |
   2008.3 |
   2031.0 |
   2053.7 |
   2076.4 |
   2099.1 |
   2121.8 |
   2144.5 |
   2167.2 |
   2189.8 |
   2212.5 |
   2235.2 |
   2257.9 |
   2280.6 |########################################
   2303.3 |
  (30 below, 2 above range)

iter_strided (n=160, range 267.9-804.2 ns)
    267.9 |########################################
    294.7 |
    321.5 |
    348.3 |
    375.2 |
    402.0 |
    428.8 |
    455.6 |
    482.4 |
    509.2 |
    536.1 |
    562.9 |
    589.7 |
    616.5 |
    643.3 |
    670.2 |
    697.0 |
    723.8 |
    750.6 |
    777.4 |
  (15 below, 21 above range)

```

## Diagnostics

- **iter_strided**: CV=59.1% (high variance, measurements may be unstable)
- **iter_strided**: worst_20/best_20 = 3.0x (possible bimodal distribution)
