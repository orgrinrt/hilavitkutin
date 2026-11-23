# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (atomic_acquire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline atomic_acquire has the worst median (17.43 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest atomic_seqcst at 9.49 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### atomic_seqcst beats baseline by 40% (significant)

atomic_seqcst is -7.00 us (40%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_seqcst is fastest but the noisiest (CV 10.0%)

atomic_seqcst wins on median (9.49 us) yet has the highest variance (CV 10.0%), while atomic_acquire is the steadiest (CV 6.0%, 17.43 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: atomic_seqcst** at 9491.4 ns median (-45.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.84x (fastest 9491.4 ns, slowest 17425.7 ns)

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
| Counter frequency | 2112895079 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| atomic_acquire | 18879ns | 19308ns | 17112ns | 19308ns | 19359ns | base |
| atomic_relaxed | 11762ns | 12310ns | 10070ns | 12128ns | 12356ns | -37.70% |
| atomic_seqcst | 11202ns | 11234ns | 10064ns | 11198ns | 12354ns | -40.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 17033ns | 15439ns | 17453ns | base | 0.962 |
| atomic_relaxed | 9959ns | 8528ns | 10456ns | -41.53% | 1.645 |
| atomic_seqcst | 9485ns | 8524ns | 10457ns | -44.31% | 1.727 |

## Performance model

- Peak throughput: **1.922 Gops/s** (atomic_seqcst; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 0.940 | 48.9% |
| atomic_relaxed | 1.572 | 81.8% |
| atomic_seqcst | 1.726 | 89.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 18879ns | 18879ns | base |
| atomic_relaxed | 11762ns | 11762ns | -37.70% |
| atomic_seqcst | 11202ns | 11202ns | -40.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 17426ns | base | --- | [17425, 17426] | --- | --- | --- | --- |
| atomic_relaxed | 10424ns | -6998.5ns (-40.2%) | [-7003, -6995]ns | [10420, 10427] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 9491ns | -7004.5ns (-40.2%) | [-7011, -6998]ns | [8542, 10418] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 17426ns | -40.2% | -40.2% |
| 1 | 2 | 17426ns | -40.1% | -40.2% |
| 1 | 3 | 14254ns | -26.8% | -40.1% |
| 1 | 4 | 17427ns | -51.0% | -40.1% |
| 1 | 5 | 17424ns | -40.2% | -40.1% |
| 1 | 6 | 17426ns | -40.2% | -51.0% |
| 1 | 7 | 17426ns | -51.1% | -51.0% |
| 1 | 8 | 17427ns | -40.1% | -51.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.161 | ok |
| atomic_relaxed | -0.375 | moderate- |
| atomic_seqcst | 0.125 | ok |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 23.3ns | 17032.6ns | 0.1% |  |
| atomic_relaxed | 22.4ns | 9958.6ns | 0.2% |  |
| atomic_seqcst | 21.4ns | 9484.9ns | 0.2% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 15439.1-17452.6 ns)
  15439.1 |
  15539.8 |
  15640.5 |
  15741.1 |
  15841.8 |
  15942.5 |
  16043.2 |
  16143.8 |
  16244.5 |
  16345.2 |
  16445.8 |
  16546.5 |
  16647.2 |
  16747.9 |
  16848.5 |
  16949.2 |
  17049.9 |
  17150.6 |
  17251.2 |
  17351.9 |########################################
  (20 below, 5 above range)

atomic_relaxed (n=160, range 8528.2-10455.8 ns)
   8528.2 |#######
   8624.6 |
   8721.0 |
   8817.3 |
   8913.7 |
   9010.1 |
   9106.5 |
   9202.9 |
   9299.3 |
   9395.6 |
   9492.0 |
   9588.4 |
   9684.8 |
   9781.2 |
   9877.5 |
   9973.9 |
  10070.3 |
  10166.7 |
  10263.1 |
  10359.5 |########################################
  (17 below, 7 above range)

atomic_seqcst (n=160, range 8524.2-10457.0 ns)
   8524.2 |###################################
   8620.8 |
   8717.4 |
   8814.1 |
   8910.7 |
   9007.4 |
   9104.0 |
   9200.7 |
   9297.3 |
   9394.0 |
   9490.6 |
   9587.2 |
   9683.9 |
   9780.5 |
   9877.2 |
   9973.8 |
  10070.5 |
  10167.1 |
  10263.7 |
  10360.4 |########################################
  (13 below, 5 above range)

```
