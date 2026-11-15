# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (4.63 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 292 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 683% faster than the next best (iter_seq)

iter_strided (292 ns) leads iter_seq (2.29 us) by 683%, a clear separation rather than a photo finish. CV 78.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 94% (significant)

iter_strided is -4.34 us (94%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 15.9x slower than the field

iter_gather (4.63 us) is 15.9x the fastest (292 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 78.0%)

iter_strided wins on median (292 ns) yet has the highest variance (CV 78.0%), while iter_seq is the steadiest (CV 2.5%, 2.29 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 15.9x the fastest

Fastest iter_strided (292 ns) to slowest iter_gather (4.63 us): 15.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_strided is inconsistent: worst-20% is 3.1x its best-20%

iter_strided's best 20% of batches run at 262 ns but its worst 20% at 808 ns (3.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 292.1 ns median (-93.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 15.86x (fastest 292.1 ns, slowest 4633.4 ns)

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
| Counter frequency | 2112774242 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iter_gather | 6375ns | 6518ns | 5774ns | 6517ns | 6548ns | base |
| iter_seq | 4192ns | 4172ns | 4162ns | 4173ns | 4279ns | -34.24% |
| iter_strided | 2934ns | 2172ns | 1946ns | 2231ns | 6032ns | -53.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 4532ns | 4106ns | 4646ns | base | 3.615 |
| iter_seq | 2298ns | 2287ns | 2336ns | -49.30% | 7.131 |
| iter_strided | 393ns | 262ns | 808ns | -91.32% | 41.671 |

## Performance model

- Peak throughput: **62.429 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.536 | 5.7% |
| iter_seq | 7.163 | 11.5% |
| iter_strided | 56.090 | 89.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 6375ns | 6375ns | base |
| iter_seq | 4192ns | 4192ns | -34.24% |
| iter_strided | 2934ns | 2934ns | -53.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 4633ns | base | --- | [4633, 4635] | --- | --- | --- | --- |
| iter_seq | 2287ns | -2344.3ns (-50.6%) | [-2345, -2344]ns | [2287, 2288] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 292ns | -4340.1ns (-93.7%) | [-4341, -4339]ns | [292, 292] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 4634ns | -50.6% | -93.7% |
| 1 | 2 | 4633ns | -50.6% | -93.7% |
| 1 | 3 | 3791ns | -39.7% | -92.3% |
| 1 | 4 | 4642ns | -50.7% | -93.7% |
| 1 | 5 | 4634ns | -50.6% | -94.8% |
| 1 | 6 | 4636ns | -50.7% | -87.2% |
| 1 | 7 | 4637ns | -50.7% | -93.7% |
| 1 | 8 | 4634ns | -50.6% | -93.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.165 | ok |
| iter_seq | -0.379 | moderate- |
| iter_strided | -0.288 | moderate- |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 23.3ns | 4531.7ns | 0.5% |  |
| iter_seq | 23.8ns | 2297.6ns | 1.0% |  |
| iter_strided | 32.3ns | 393.2ns | 8.2% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 4106.0-4645.8 ns)
   4106.0 |
   4133.0 |
   4160.0 |
   4187.0 |
   4214.0 |
   4241.0 |
   4268.0 |
   4294.9 |
   4321.9 |
   4348.9 |
   4375.9 |
   4402.9 |
   4429.9 |
   4456.9 |
   4483.9 |
   4510.8 |
   4537.8 |
   4564.8 |
   4591.8 |
   4618.8 |########################################
  (20 below, 7 above range)

iter_seq (n=160, range 2287.1-2336.2 ns)
   2287.1 |########################################
   2289.6 |
   2292.0 |
   2294.5 |
   2296.9 |#######
   2299.4 |####
   2301.8 |#
   2304.3 |
   2306.7 |
   2309.2 |
   2311.6 |
   2314.1 |
   2316.5 |
   2319.0 |
   2321.4 |
   2323.9 |
   2326.3 |
   2328.8 |
   2331.2 |
   2333.7 |
  (17 below, 3 above range)

iter_strided (n=160, range 262.4-808.4 ns)
    262.4 |
    289.7 |########################################
    317.0 |
    344.3 |
    371.6 |
    398.9 |
    426.2 |
    453.5 |
    480.8 |#
    508.1 |
    535.4 |
    562.7 |
    590.0 |
    617.3 |
    644.6 |
    671.9 |
    699.2 |###
    726.5 |
    753.8 |
    781.1 |
  (18 below, 17 above range)

```

## Diagnostics

- **iter_strided**: CV=57.9% (high variance, measurements may be unstable)
- **iter_strided**: worst_20/best_20 = 3.1x (possible bimodal distribution)
