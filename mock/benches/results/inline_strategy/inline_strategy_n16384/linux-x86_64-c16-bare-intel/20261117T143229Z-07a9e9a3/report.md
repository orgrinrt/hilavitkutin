# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inline_always) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inline_always has the worst median (18.27 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inline_default at 14.95 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inline_default is fastest but the noisiest (CV 59.0%)

inline_default wins on median (14.95 us) yet has the highest variance (CV 59.0%), while inline_never is the steadiest (CV 9.6%, 14.95 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (inline_default, inline_never) are a dead heat (<1%)

inline_default (14.95 us) and inline_never (14.95 us) differ by 0.02%, inside the noise, even though the wider field spreads 22.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3.32 us) is smaller than the fastest variant's own run-to-run std-dev (8.82 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader inline_default vs stability leader inline_never (+0% speed for 6.1x steadier)

inline_default is fastest (14.95 us, CV 59.0%); inline_never gives up 0.0% median for 6.1x lower variance (CV 9.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: inline_default** at 14947.3 ns median (-18.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.22x (fastest 14947.3 ns, slowest 18268.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x39f847b2c651de80 |
| Counter frequency | 2112795803 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 20242ns | 20155ns | 16598ns | 20007ns | 24591ns | base |
| inline_default | 17830ns | 16493ns | 16480ns | 16500ns | 23171ns | -11.92% |
| inline_never | 17424ns | 16496ns | 16483ns | 16821ns | 20176ns | -13.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 18342ns | 15039ns | 22271ns | base | 0.893 |
| inline_default | 16157ns | 14937ns | 20997ns | -11.91% | 1.014 |
| inline_never | 15788ns | 14938ns | 18283ns | -13.92% | 1.038 |

## Performance model

- Peak throughput: **1.097 Gops/s** (inline_default; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.897 | 81.8% |
| inline_default | 1.096 | 99.9% |
| inline_never | 1.096 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 20242ns | 20242ns | base |
| inline_default | 17830ns | 17830ns | -11.92% |
| inline_never | 17424ns | 17424ns | -13.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 18269ns | base | --- | [18269, 18270] | --- | --- | --- | --- |
| inline_default | 14947ns | -3321.7ns (-18.2%) | [-3323, -3318]ns | [14947, 14948] | YES | 0.0000 | 0.0000 | 0 |
| inline_never | 14950ns | -3252.7ns (-17.8%) | [-3318, -353]ns | [14950, 14951] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 18479ns | -19.1% | -19.1% |
| 1 | 2 | 14952ns | +0.0% | -0.1% |
| 1 | 3 | 15270ns | -2.0% | -2.1% |
| 1 | 4 | 18269ns | -18.2% | -18.2% |
| 1 | 5 | 18269ns | +0.0% | +0.0% |
| 1 | 6 | 18298ns | -18.3% | -0.1% |
| 1 | 7 | 18269ns | -18.2% | -18.2% |
| 1 | 8 | 18269ns | -18.2% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | 0.253 | moderate+ |
| inline_default | -0.163 | ok |
| inline_never | 0.292 | moderate+ |

**Consistency summary:**

- **inline_default**: won 6/8, lost 0/8
- **inline_never**: won 6/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 24.1ns | 18341.5ns | 0.1% |  |
| inline_default | 21.2ns | 16156.7ns | 0.1% |  |
| inline_never | 20.9ns | 15788.4ns | 0.1% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 15039.0-22270.7 ns)
  15039.0 |###
  15400.6 |##
  15762.2 |
  16123.8 |
  16485.3 |
  16846.9 |
  17208.5 |
  17570.1 |
  17931.7 |########################################
  18293.3 |#####
  18654.8 |#
  19016.4 |
  19378.0 |
  19739.6 |
  20101.2 |
  20462.8 |
  20824.3 |
  21185.9 |
  21547.5 |
  21909.1 |
  (22 below, 2 above range)

inline_default (n=160, range 14937.2-20997.3 ns)
  14937.2 |########################################
  15240.2 |
  15543.2 |
  15846.2 |
  16149.2 |
  16452.2 |
  16755.2 |
  17058.2 |
  17361.2 |
  17664.2 |
  17967.3 |#####
  18270.3 |
  18573.3 |
  18876.3 |
  19179.3 |
  19482.3 |
  19785.3 |
  20088.3 |
  20391.3 |
  20694.3 |
  (4 below, 2 above range)

inline_never (n=160, range 14938.4-18282.9 ns)
  14938.4 |########################################
  15105.6 |
  15272.8 |
  15440.0 |
  15607.3 |
  15774.5 |
  15941.7 |
  16108.9 |
  16276.2 |
  16443.4 |
  16610.6 |
  16777.8 |
  16945.1 |
  17112.3 |
  17279.5 |
  17446.7 |
  17614.0 |
  17781.2 |
  17948.4 |
  18115.6 |############
  (8 below, 6 above range)

```

## Diagnostics

- **inline_always**: CV=42.2% (high variance, measurements may be unstable)
- **inline_default**: CV=54.6% (high variance, measurements may be unstable)
