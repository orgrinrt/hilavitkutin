# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (atomic_acquire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline atomic_acquire has the worst median (3.58 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest atomic_seqcst at 2.16 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### atomic_seqcst beats baseline by 39% (significant)

atomic_seqcst is -1.41 us (39%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_relaxed is inconsistent: worst-20% is 1.8x its best-20%

atomic_relaxed's best 20% of batches run at 2.15 us but its worst 20% at 3.77 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: atomic_seqcst** at 2160.7 ns median (-39.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.65x (fastest 2160.7 ns, slowest 3575.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x0ddc0ffee055b011 |
| Counter frequency | 2112683200 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| atomic_acquire | 5594ns | 5118ns | 5103ns | 5302ns | 6960ns | base |
| atomic_relaxed | 4485ns | 3935ns | 3694ns | 4076ns | 6506ns | -19.81% |
| atomic_seqcst | 4054ns | 3707ns | 3693ns | 3834ns | 5073ns | -27.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 3904ns | 3565ns | 4855ns | base | 1.049 |
| atomic_relaxed | 2612ns | 2155ns | 3774ns | -33.10% | 1.568 |
| atomic_seqcst | 2362ns | 2155ns | 2947ns | -39.49% | 1.734 |

## Performance model

- Peak throughput: **1.901 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.146 | 60.3% |
| atomic_relaxed | 1.784 | 93.9% |
| atomic_seqcst | 1.896 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 5594ns | 5594ns | base |
| atomic_relaxed | 4485ns | 4485ns | -19.81% |
| atomic_seqcst | 4054ns | 4054ns | -27.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 3575ns | base | --- | [3566, 3578] | --- | --- | --- | --- |
| atomic_relaxed | 2296ns | -1405.7ns (-39.3%) | [-1409, -1386]ns | [2183, 2546] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 2161ns | -1411.8ns (-39.5%) | [-1419, -1410]ns | [2159, 2167] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 4358ns | -50.6% | -39.5% |
| 1 | 2 | 3566ns | -26.0% | -39.5% |
| 1 | 3 | 3576ns | -38.2% | -39.6% |
| 1 | 4 | 3571ns | -38.1% | -39.6% |
| 1 | 5 | 3566ns | -39.4% | -39.4% |
| 1 | 6 | 3565ns | -28.0% | -39.4% |
| 1 | 7 | 4369ns | -39.6% | -50.6% |
| 1 | 8 | 3565ns | -39.4% | -26.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.217 | moderate- |
| atomic_relaxed | -0.225 | moderate- |
| atomic_seqcst | -0.043 | ok |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 21.6ns | 3903.8ns | 0.6% |  |
| atomic_relaxed | 23.2ns | 2611.8ns | 0.9% |  |
| atomic_seqcst | 21.4ns | 2362.3ns | 0.9% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 3565.2-4854.6 ns)
   3565.2 |########################################
   3629.6 |
   3694.1 |#
   3758.6 |#
   3823.1 |
   3887.5 |#
   3952.0 |
   4016.5 |
   4081.0 |
   4145.4 |
   4209.9 |
   4274.4 |
   4338.8 |###############
   4403.3 |
   4467.8 |
   4532.3 |
   4596.7 |
   4661.2 |
   4725.7 |
   4790.2 |
  (12 below, 5 above range)

atomic_relaxed (n=160, range 2154.6-3773.6 ns)
   2154.6 |########################################
   2235.5 |#####
   2316.5 |###
   2397.4 |
   2478.4 |##
   2559.3 |##################
   2640.3 |#########
   2721.3 |##
   2802.2 |#
   2883.2 |#
   2964.1 |
   3045.1 |
   3126.0 |
   3207.0 |
   3287.9 |#
   3368.9 |#
   3449.8 |
   3530.8 |
   3611.7 |
   3692.7 |#
  (15 below, 10 above range)

atomic_seqcst (n=160, range 2154.8-2947.4 ns)
   2154.8 |########################################
   2194.4 |#
   2234.1 |
   2273.7 |
   2313.3 |
   2353.0 |
   2392.6 |
   2432.2 |
   2471.8 |
   2511.5 |
   2551.1 |
   2590.7 |
   2630.4 |################
   2670.0 |
   2709.6 |
   2749.3 |
   2788.9 |
   2828.5 |
   2868.2 |
   2907.8 |
  (12 below, 5 above range)

```

## Diagnostics

- **atomic_acquire**: CV=20.1% (high variance, measurements may be unstable)
- **atomic_relaxed**: CV=33.1% (high variance, measurements may be unstable)
- **atomic_seqcst**: CV=20.1% (high variance, measurements may be unstable)
