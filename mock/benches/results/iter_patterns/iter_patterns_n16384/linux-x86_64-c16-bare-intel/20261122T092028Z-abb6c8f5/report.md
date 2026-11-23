# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (4.63 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 292 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 683% faster than the next best (iter_seq)

iter_strided (292 ns) leads iter_seq (2.29 us) by 683%, a clear separation rather than a photo finish. CV 74.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 94% (significant)

iter_strided is -4.34 us (94%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 15.9x slower than the field

iter_gather (4.63 us) is 15.9x the fastest (292 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 74.6%)

iter_strided wins on median (292 ns) yet has the highest variance (CV 74.6%), while iter_gather is the steadiest (CV 6.0%, 4.63 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 15.9x the fastest

Fastest iter_strided (292 ns) to slowest iter_gather (4.63 us): 15.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_strided is inconsistent: worst-20% is 2.7x its best-20%

iter_strided's best 20% of batches run at 267 ns but its worst 20% at 725 ns (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 292.1 ns median (-93.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 15.86x (fastest 292.1 ns, slowest 4633.0 ns)

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
| Counter frequency | 2112768594 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iter_gather | 6377ns | 6520ns | 5776ns | 6519ns | 6555ns | base |
| iter_seq | 4189ns | 4171ns | 3742ns | 4172ns | 4684ns | -34.32% |
| iter_strided | 2791ns | 2171ns | 1985ns | 2174ns | 5450ns | -56.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 4532ns | 4106ns | 4648ns | base | 3.615 |
| iter_seq | 2295ns | 2055ns | 2554ns | -49.37% | 7.140 |
| iter_strided | 374ns | 267ns | 725ns | -91.75% | 43.830 |

## Performance model

- Peak throughput: **61.328 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.536 | 5.8% |
| iter_seq | 7.162 | 11.7% |
| iter_strided | 56.090 | 91.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 6377ns | 6377ns | base |
| iter_seq | 4189ns | 4189ns | -34.32% |
| iter_strided | 2791ns | 2791ns | -56.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 4633ns | base | --- | [4632, 4639] | --- | --- | --- | --- |
| iter_seq | 2288ns | -2343.2ns (-50.6%) | [-2344, -2343]ns | [2287, 2288] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 292ns | -4339.3ns (-93.7%) | [-4341, -4338]ns | [292, 292] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 4632ns | -50.6% | -93.7% |
| 1 | 2 | 4640ns | -50.7% | -94.8% |
| 1 | 3 | 4634ns | -50.6% | -93.7% |
| 1 | 4 | 4635ns | -50.6% | -93.7% |
| 1 | 5 | 4632ns | -50.6% | -93.7% |
| 1 | 6 | 4639ns | -50.7% | -93.7% |
| 1 | 7 | 3791ns | -50.6% | -92.3% |
| 1 | 8 | 4641ns | -50.7% | -93.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.172 | ok |
| iter_seq | -0.160 | ok |
| iter_strided | -0.163 | ok |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 23.3ns | 4532.1ns | 0.5% |  |
| iter_seq | 24.1ns | 2294.8ns | 1.0% |  |
| iter_strided | 30.8ns | 373.8ns | 8.2% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 4106.0-4648.5 ns)
   4106.0 |
   4133.2 |
   4160.3 |
   4187.4 |
   4214.5 |
   4241.7 |
   4268.8 |
   4295.9 |
   4323.0 |
   4350.1 |
   4377.3 |
   4404.4 |
   4431.5 |
   4458.6 |
   4485.7 |
   4512.9 |
   4540.0 |
   4567.1 |
   4594.2 |
   4621.3 |########################################
  (20 below, 4 above range)

iter_seq (n=160, range 2055.4-2553.9 ns)
   2055.4 |
   2080.4 |
   2105.3 |
   2130.2 |
   2155.1 |
   2180.1 |
   2205.0 |
   2229.9 |
   2254.8 |
   2279.7 |########################################
   2304.7 |
   2329.6 |
   2354.5 |
   2379.4 |
   2404.3 |
   2429.3 |
   2454.2 |
   2479.1 |
   2504.0 |
   2529.0 |
  (18 below, 5 above range)

iter_strided (n=160, range 267.2-725.4 ns)
    267.2 |
    290.1 |########################################
    313.0 |
    335.9 |
    358.8 |
    381.7 |
    404.6 |
    427.5 |
    450.4 |
    473.4 |
    496.3 |
    519.2 |
    542.1 |
    565.0 |
    587.9 |
    610.8 |
    633.7 |
    656.7 |
    679.6 |
    702.5 |
  (15 below, 21 above range)

```

## Diagnostics

- **iter_strided**: CV=58.3% (high variance, measurements may be unstable)
