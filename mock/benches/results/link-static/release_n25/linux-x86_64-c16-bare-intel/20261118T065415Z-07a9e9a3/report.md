# link-static: the fixture's release profile

2 variants, 6 samples per variant.
Baseline: **default_relink**

## Highlights

Baseline for all deltas below: **default_relink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (default_relink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline default_relink has the worst median (103.15 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest default_replay at 47.66 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### default_replay dominates: 116% faster than the next best (default_relink)

default_replay (47.66 ms) leads default_relink (103.15 ms) by 116%, a clear separation rather than a photo finish. CV 6.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### default_replay beats baseline by 54% (significant)

default_replay is -55.27 ms (54%) faster than baseline default_relink, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### default_replay is fastest but the noisiest (CV 6.4%)

default_replay wins on median (47.66 ms) yet has the highest variance (CV 6.4%), while default_relink is the steadiest (CV 1.1%, 103.15 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: default_replay** at 47661140.6 ns median (-53.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.16x (fastest 47661140.6 ns, slowest 103154546.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | link-static: the fixture's release profile |
| Passes | 6 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6c6b737461740001 |
| Counter frequency | 2112646720 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| default_relink | 103006073ns | 103156719ns | 101439942ns | 102717738ns | 104221641ns | base |
| default_replay | 47935184ns | 47663635ns | 43342931ns | 46932222ns | 51735754ns | -53.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| default_relink | 103003629ns | 101437139ns | 104219190ns | base |
| default_replay | 47932724ns | 43339528ns | 51733539ns | -53.47% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| default_relink | 103006073ns | 103006073ns | base |
| default_replay | 47935184ns | 47935184ns | -53.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| default_relink | 103154546ns | base | --- | [101637149, 104219190] | --- | --- | --- | --- |
| default_replay | 47661141ns | -55266753.4ns (-53.6%) | [-58134073, -51811887]ns | [44403493, 51733539] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | default_relink | default_replay |
|---|---|---|---|
| 1 | 1 | 103742420ns | -49.9% |
| 1 | 2 | 104695961ns | -54.1% |
| 1 | 3 | 103348431ns | -50.2% |
| 1 | 4 | 101437139ns | -53.4% |
| 1 | 5 | 102960662ns | -57.9% |
| 1 | 6 | 101837158ns | -55.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| default_relink | 0.192 | ok |
| default_replay | 0.228 | moderate+ |

**Consistency summary:**

- **default_replay**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| default_relink | 48081726.4ns | 103003628.5ns | 46.7% | HIGH |
| default_replay | 46823745.0ns | 47932724.2ns | 97.7% | HIGH |

## Distribution (algo ns)

```
default_relink (n=6, range 101437139.3-104219190.5 ns)
  101437139.3 |########################################
  101576241.9 |
  101715344.4 |########################################
  101854447.0 |
  101993549.5 |
  102132652.1 |
  102271754.7 |
  102410857.2 |
  102549959.8 |
  102689062.3 |
  102828164.9 |########################################
  102967267.5 |
  103106370.0 |
  103245472.6 |########################################
  103384575.1 |
  103523677.7 |
  103662780.3 |########################################
  103801882.8 |
  103940985.4 |
  104080087.9 |
  (0 below, 1 above range)

default_replay (n=6, range 43339527.8-51733538.9 ns)
  43339527.8 |########################################
  43759228.4 |
  44178928.9 |
  44598629.5 |
  45018330.0 |
  45438030.6 |########################################
  45857731.1 |
  46277431.7 |
  46697132.2 |
  47116832.8 |########################################
  47536533.3 |
  47956233.9 |########################################
  48375934.4 |
  48795635.0 |
  49215335.5 |
  49635036.1 |
  50054736.6 |
  50474437.2 |
  50894137.7 |
  51313838.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **default_relink**: bridge=45.7% of algo (FFI overhead may distort results)
- **default_replay**: bridge=97.7% of algo (FFI overhead may distort results)
