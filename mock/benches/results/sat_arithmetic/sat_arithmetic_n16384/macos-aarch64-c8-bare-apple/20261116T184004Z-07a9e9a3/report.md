# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (sat_checked) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline sat_checked has the worst median (10.52 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sat_wrap at 5.25 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sat_wrap dominates: 100% faster than the next best (sat_saturate)

sat_wrap (5.25 us) leads sat_saturate (10.49 us) by 100%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 50% (significant)

sat_wrap is -5.26 us (50%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_checked is an outlier: 2.0x slower than the field

sat_checked (10.52 us) is 2.0x the fastest (5.25 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sat_wrap shows warm-up / thermal drift (autocorr +0.60)

sat_wrap's per-pass series has lag-1 autocorrelation +0.60, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### sat_checked is inconsistent: worst-20% is 1.7x its best-20%

sat_checked's best 20% of batches run at 10.43 us but its worst 20% at 17.93 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### sat_saturate's edge over baseline is significant but tiny (-5 ns, 0.05%)

sat_saturate differs from baseline sat_checked by -5 ns (0.05%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: sat_wrap** at 5245.0 ns median (-50.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.01x (fastest 5245.0 ns, slowest 10516.7 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sat_checked | 14341ns | 12647ns | 12555ns | 12724ns | 20979ns | base |
| sat_saturate | 12795ns | 12620ns | 12555ns | 12697ns | 13329ns | -10.78% |
| sat_wrap | 7435ns | 7375ns | 7354ns | 7392ns | 7646ns | -48.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 12020ns | 10433ns | 17935ns | base | 1.363 |
| sat_saturate | 10625ns | 10432ns | 11052ns | -11.60% | 1.542 |
| sat_wrap | 5291ns | 5234ns | 5442ns | -55.98% | 3.097 |

## Performance model

- Peak throughput: **3.131 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 1.558 | 49.8% |
| sat_saturate | 1.562 | 49.9% |
| sat_wrap | 3.124 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 14341ns | 14341ns | base |
| sat_saturate | 12795ns | 12795ns | -10.78% |
| sat_wrap | 7435ns | 7435ns | -48.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 10517ns | base | --- | [10489, 10573] | --- | --- | --- | --- |
| sat_saturate | 10487ns | -4.8ns (-0.0%) | [-32, -1]ns | [10459, 10553] | YES | 0.0084 | 0.0084 | 2 |
| sat_wrap | 5245ns | -5256.9ns (-50.0%) | [-5297, -5211]ns | [5241, 5263] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 10441ns | +0.0% | -49.8% |
| 1 | 2 | 10454ns | -0.1% | -49.9% |
| 1 | 3 | 10452ns | -0.1% | -49.8% |
| 1 | 4 | 10443ns | -0.1% | -49.8% |
| 1 | 5 | 11025ns | -3.8% | -52.4% |
| 1 | 6 | 15032ns | -26.6% | -65.0% |
| 1 | 7 | 10726ns | +1.2% | -50.5% |
| 1 | 8 | 10449ns | +1.8% | -49.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.014 | ok |
| sat_saturate | 0.556 | HIGH+ (drift/warm-up) |
| sat_wrap | 0.598 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **sat_saturate**: won 4/8, lost 2/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 4.4ns | 12019.7ns | 0.0% |  |
| sat_saturate | 3.4ns | 10625.1ns | 0.0% |  |
| sat_wrap | 3.1ns | 5290.6ns | 0.1% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 10433.0-17934.9 ns)
  10433.0 |########################################
  10808.1 |#####
  11183.2 |
  11558.3 |
  11933.3 |
  12308.4 |
  12683.5 |#
  13058.6 |
  13433.7 |
  13808.8 |
  14183.9 |
  14559.0 |
  14934.1 |
  15309.2 |
  15684.3 |
  16059.4 |
  16434.5 |
  16809.6 |
  17184.7 |
  17559.8 |
  (15 below, 8 above range)

sat_saturate (n=160, range 10432.3-11051.7 ns)
  10432.3 |########################################
  10463.2 |########
  10494.2 |#####
  10525.2 |###
  10556.2 |##
  10587.1 |#
  10618.1 |##
  10649.1 |#####
  10680.0 |#
  10711.0 |#####
  10742.0 |##
  10773.0 |#
  10803.9 |##
  10834.9 |###
  10865.9 |
  10896.8 |##
  10927.8 |
  10958.8 |#
  10989.8 |#
  11020.7 |#####
  (15 below, 12 above range)

sat_wrap (n=160, range 5233.6-5442.0 ns)
   5233.6 |########################################
   5244.0 |#######
   5254.4 |##
   5264.8 |#####
   5275.3 |#####
   5285.7 |###
   5296.1 |#########
   5306.5 |#####
   5316.9 |#
   5327.3 |#
   5337.8 |
   5348.2 |###
   5358.6 |
   5369.0 |
   5379.4 |
   5389.9 |
   5400.3 |
   5410.7 |#
   5421.1 |
   5431.5 |
  (16 below, 8 above range)

```

## Diagnostics

- **sat_checked**: CV=63.0% (high variance, measurements may be unstable)
- **sat_saturate**: autocorrelation=0.56 (measurement drift or warm-up artifact)
- **sat_wrap**: autocorrelation=0.60 (measurement drift or warm-up artifact)
