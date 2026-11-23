# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### inline_always dominates: 22% faster than the next best (inline_default)

inline_always (3.75 us) leads inline_default (4.57 us) by 22%, a clear separation rather than a photo finish. CV 10.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inline_always is fastest but the noisiest (CV 10.8%)

inline_always wins on median (3.75 us) yet has the highest variance (CV 10.8%), while inline_never is the steadiest (CV 7.7%, 4.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (inline_always)

The baseline inline_always is the fastest (3.75 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### inline_default's edge over baseline is significant but tiny (0 ns, 0.01%)

inline_default differs from baseline inline_always by 0 ns (0.01%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (inline_always) is the fastest** at 3746.6 ns median
- 2 variants significantly slower than baseline
- Spread: 1.22x (fastest 3746.6 ns, slowest 4567.6 ns)

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
| Counter frequency | 2112764099 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 5727ns | 5288ns | 5272ns | 5630ns | 6473ns | base |
| inline_default | 6021ns | 6450ns | 5276ns | 6116ns | 6478ns | +5.13% |
| inline_never | 6175ns | 6456ns | 5286ns | 6370ns | 6480ns | +7.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 4053ns | 3737ns | 4579ns | base | 1.011 |
| inline_default | 4261ns | 3737ns | 4579ns | +5.13% | 0.961 |
| inline_never | 4370ns | 3740ns | 4581ns | +7.83% | 0.937 |

## Performance model

- Peak throughput: **1.096 Gops/s** (inline_always; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 1.093 | 99.7% |
| inline_default | 0.897 | 81.8% |
| inline_never | 0.897 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 5727ns | 5727ns | base |
| inline_default | 6021ns | 6021ns | +5.13% |
| inline_never | 6175ns | 6175ns | +7.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 3747ns | base | --- | [3737, 3748] | --- | --- | --- | --- |
| inline_default | 4567ns | +0.3ns (+0.0%) | [+0, +10]ns | [4567, 4567] | YES | 0.0041 | 0.0041 | 10 |
| inline_never | 4568ns | +520.6ns (+13.9%) | [+11, +829]ns | [4568, 4568] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 4578ns | -18.2% | -0.2% |
| 1 | 2 | 4567ns | +0.0% | -18.2% |
| 1 | 3 | 3737ns | +22.2% | +22.4% |
| 1 | 4 | 3737ns | +22.2% | +22.2% |
| 1 | 5 | 3737ns | -0.0% | +22.4% |
| 1 | 6 | 3737ns | +22.2% | +22.2% |
| 1 | 7 | 3737ns | -0.0% | +0.3% |
| 1 | 8 | 4573ns | +0.1% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | 0.260 | moderate+ |
| inline_default | -0.478 | moderate- |
| inline_never | -0.375 | moderate- |

**Consistency summary:**

- **inline_default**: won 1/8, lost 4/8
- **inline_never**: won 2/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 20.9ns | 4052.6ns | 0.5% |  |
| inline_default | 22.0ns | 4260.7ns | 0.5% |  |
| inline_never | 22.8ns | 4369.7ns | 0.5% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 3736.8-4579.1 ns)
   3736.8 |########################################
   3778.9 |
   3821.0 |
   3863.1 |
   3905.3 |
   3947.4 |
   3989.5 |
   4031.6 |
   4073.7 |
   4115.8 |
   4157.9 |
   4200.1 |
   4242.2 |
   4284.3 |
   4326.4 |
   4368.5 |
   4410.6 |
   4452.8 |
   4494.9 |
   4537.0 |########################
  (11 below, 6 above range)

inline_default (n=160, range 3736.8-4579.2 ns)
   3736.8 |###################
   3778.9 |
   3821.1 |
   3863.2 |
   3905.3 |
   3947.4 |
   3989.5 |
   4031.7 |
   4073.8 |
   4115.9 |
   4158.0 |
   4200.1 |
   4242.3 |
   4284.4 |
   4326.5 |
   4368.6 |
   4410.8 |
   4452.9 |
   4495.0 |
   4537.1 |########################################
  (17 below, 11 above range)

inline_never (n=160, range 3740.0-4580.6 ns)
   3740.0 |####
   3782.0 |
   3824.0 |
   3866.1 |
   3908.1 |
   3950.1 |
   3992.2 |
   4034.2 |
   4076.2 |
   4118.2 |
   4160.3 |
   4202.3 |
   4244.3 |
   4286.4 |
   4328.4 |
   4370.4 |
   4412.5 |
   4454.5 |
   4496.5 |
   4538.6 |########################################
  (23 below, 11 above range)

```
