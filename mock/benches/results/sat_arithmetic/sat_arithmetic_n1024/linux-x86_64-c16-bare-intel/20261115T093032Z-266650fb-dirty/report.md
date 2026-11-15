# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (sat_checked) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline sat_checked has the worst median (504 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sat_wrap at 295 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sat_wrap dominates: 71% faster than the next best (sat_saturate)

sat_wrap (295 ns) leads sat_saturate (504 ns) by 71%, a clear separation rather than a photo finish. CV 80.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 42% (significant)

sat_wrap is -210 ns (42%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_wrap is fastest but the noisiest (CV 80.3%)

sat_wrap wins on median (295 ns) yet has the highest variance (CV 80.3%), while sat_checked is the steadiest (CV 71.5%, 504 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (209 ns) is smaller than the fastest variant's own run-to-run std-dev (237 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### sat_wrap is inconsistent: worst-20% is 3.2x its best-20%

sat_wrap's best 20% of batches run at 241 ns but its worst 20% at 765 ns (3.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sat_wrap** at 295.1 ns median (-41.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.71x (fastest 295.1 ns, slowest 503.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Overflow strategy: wrapping_add vs saturating_add vs checked_add |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x52b9de7c318fa064 |
| Counter frequency | 2112797930 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sat_checked | 2946ns | 2382ns | 1951ns | 2378ns | 5644ns | base |
| sat_saturate | 2985ns | 2381ns | 1945ns | 2308ns | 6055ns | +1.32% |
| sat_wrap | 2783ns | 2172ns | 1780ns | 2161ns | 5652ns | -5.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 623ns | 414ns | 1194ns | base | 1.643 |
| sat_saturate | 631ns | 411ns | 1276ns | +1.24% | 1.623 |
| sat_wrap | 377ns | 241ns | 765ns | -39.44% | 2.713 |

## Performance model

- Peak throughput: **4.241 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 2.033 | 47.9% |
| sat_saturate | 2.034 | 47.9% |
| sat_wrap | 3.470 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 2946ns | 2946ns | base |
| sat_saturate | 2985ns | 2985ns | +1.32% |
| sat_wrap | 2783ns | 2783ns | -5.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 504ns | base | --- | [501, 506] | --- | --- | --- | --- |
| sat_saturate | 504ns | no significant difference | [-5, +2]ns | [502, 505] | no | 0.3041 | 0.3041 | 0 |
| sat_wrap | 295ns | -209.8ns (-41.6%) | [-212, -207]ns | [295, 295] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 417ns | -0.7% | -42.1% |
| 1 | 2 | 505ns | +0.0% | -41.5% |
| 1 | 3 | 501ns | -17.0% | -51.8% |
| 1 | 4 | 417ns | +21.6% | -29.2% |
| 1 | 5 | 509ns | -0.9% | -42.0% |
| 1 | 6 | 505ns | +0.1% | -41.6% |
| 1 | 7 | 509ns | -0.0% | -42.0% |
| 1 | 8 | 507ns | -18.4% | -41.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.184 | ok |
| sat_saturate | -0.278 | moderate- |
| sat_wrap | -0.209 | moderate- |

**Consistency summary:**

- **sat_saturate**: won 4/8, lost 1/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 29.3ns | 623.3ns | 4.7% |  |
| sat_saturate | 29.7ns | 631.0ns | 4.7% |  |
| sat_wrap | 30.3ns | 377.4ns | 8.0% | HIGH |

## Distribution (algo ns)

```
sat_checked (n=160, range 413.9-1193.8 ns)
    413.9 |######
    452.9 |
    491.9 |########################################
    530.9 |
    569.9 |
    608.9 |
    647.9 |
    686.9 |
    725.9 |
    764.9 |
    803.9 |
    842.9 |
    881.9 |
    920.9 |
    959.9 |
    998.9 |
   1037.9 |
   1076.9 |
   1115.9 |
   1154.9 |
  (16 below, 20 above range)

sat_saturate (n=160, range 410.9-1276.3 ns)
    410.9 |################
    454.2 |##
    497.4 |########################################
    540.7 |
    584.0 |
    627.3 |
    670.5 |
    713.8 |
    757.1 |
    800.4 |
    843.6 |
    886.9 |
    930.2 |
    973.4 |#
   1016.7 |
   1060.0 |
   1103.3 |
   1146.5 |
   1189.8 |#
   1233.1 |
  (17 below, 19 above range)

sat_wrap (n=160, range 241.4-764.6 ns)
    241.4 |######
    267.6 |
    293.8 |########################################
    319.9 |
    346.1 |
    372.2 |
    398.4 |
    424.5 |
    450.7 |
    476.8 |
    503.0 |
    529.2 |
    555.3 |
    581.5 |
    607.6 |
    633.8 |
    659.9 |
    686.1 |
    712.3 |
    738.4 |
  (18 below, 19 above range)

```

## Diagnostics

- **sat_checked**: CV=57.8% (high variance, measurements may be unstable)
- **sat_saturate**: CV=60.2% (high variance, measurements may be unstable)
- **sat_saturate**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **sat_wrap**: CV=62.8% (high variance, measurements may be unstable)
- **sat_wrap**: worst_20/best_20 = 3.2x (possible bimodal distribution)
